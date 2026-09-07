//! Running a program and getting everything back, including when it will not stop.
//!
//! There is no process helper crate here for the same reason there is no serde. A run of the
//! corpus is thousands of processes, and a compiler under development hangs sometimes, so the
//! timeout is not optional and it has to be right. Writing it here means it is fifty lines
//! that can be read, rather than a dependency whose behaviour on a kill has to be taken on
//! trust.
//!
//! The output pipes are drained by their own threads. That is the part people get wrong: if
//! the parent polls for exit while the child fills a pipe buffer, the child blocks on the
//! write, the parent never sees an exit, and the timeout fires on a program that was working.
//!
//! The poll that enforces the timeout is also where the memory number comes from, since the
//! loop is running anyway and the alternative is an unsafe call this crate forbids. What that
//! number means and what it misses is in [`crate::memory`].

use crate::memory;
use std::ffi::OsStr;
use std::io::{self, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// What came out of running a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// Whether it exited zero.
    pub ok: bool,
    /// The exit status, or -1 when it was killed by a signal or by the timeout.
    pub status: i32,
    /// Whether the timeout fired.
    pub timed_out: bool,
    /// Everything it wrote to standard output.
    pub stdout: String,
    /// Everything it wrote to standard error.
    pub stderr: String,
    /// Wall time in microseconds.
    pub micros: u64,
    /// The largest high water mark reached by any process in the tree, in bytes.
    ///
    /// `None` on a platform that cannot say, which is every platform that is not Linux, and
    /// `None` when the whole tree came and went between two polls. Never nought: a program
    /// that ran used memory, and a nought here would be a measurement nobody took.
    pub peak_bytes: Option<u64>,
}

impl Outcome {
    /// An outcome for a program that could not be started at all.
    #[must_use]
    pub fn failed_to_start(error: &io::Error) -> Self {
        Self {
            ok: false,
            status: -1,
            timed_out: false,
            stdout: String::new(),
            stderr: format!("could not start the program: {error}"),
            micros: 0,
            peak_bytes: None,
        }
    }
}

/// Runs a program to completion, or kills it when the timeout runs out.
///
/// Output that is not valid UTF-8 is replaced rather than refused. A compiler that emits a
/// stray byte in a diagnostic should show up in the report as a stray byte, not as a harness
/// error that hides whatever else it said.
///
/// # Errors
///
/// When the process cannot be spawned, which usually means the program is not on the path.
pub fn run<S: AsRef<OsStr>>(
    program: &str,
    args: &[S],
    cwd: Option<&Path>,
    timeout: Duration,
) -> io::Result<Outcome> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // Colour escapes in a captured diagnostic make the report unreadable and the
        // comparison against the expected text unreliable.
        .env("NO_COLOR", "1")
        .env("TERM", "dumb");
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    let started = Instant::now();
    let mut child = command.spawn()?;
    let mut out_pipe = child.stdout.take();
    let mut err_pipe = child.stderr.take();
    let out_reader = std::thread::spawn(move || drain(out_pipe.as_mut()));
    let err_reader = std::thread::spawn(move || drain(err_pipe.as_mut()));

    let group = child.id();
    let mut timed_out = false;
    let mut status = None;
    let mut peak_bytes = None;
    let mut sample_due = Instant::now();
    // Poll rather than block, so the timeout can be enforced. The wait starts short because
    // most cases finish in well under a millisecond, and backs off so that a slow case does
    // not spin a core for its whole run.
    let mut wait = Duration::from_micros(200);
    loop {
        if let Some(exit) = child.try_wait()? {
            status = Some(exit);
            break;
        }
        let now = Instant::now();
        if now.duration_since(started) >= timeout {
            timed_out = true;
            let _ = child.kill();
            let _ = child.wait();
            break;
        }
        // Behind its own clock rather than on every poll, because the poll backs off from two
        // hundred microseconds and walking the tree that often would cost more time than the
        // compile it is measuring.
        if now >= sample_due {
            peak_bytes = peak_bytes.max(memory::high_water(group));
            sample_due = now + memory::INTERVAL;
        }
        std::thread::sleep(wait);
        wait = (wait * 2).min(Duration::from_millis(5));
    }
    let micros = u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX);

    let stdout = out_reader.join().unwrap_or_default();
    let stderr = err_reader.join().unwrap_or_default();
    let code = status.and_then(|exit| exit.code()).unwrap_or(-1);
    Ok(Outcome {
        ok: !timed_out && code == 0,
        status: code,
        timed_out,
        stdout,
        stderr,
        micros,
        peak_bytes,
    })
}

/// Runs a program several times and keeps the fastest.
///
/// The fastest is the honest number on a machine that is doing other things. Every slower
/// measurement contains somebody else's work as well as ours, and there is no way to subtract
/// it, so the floor is the closest thing available to the time the code alone would take.
///
/// The output and status come from the first repetition. If a program is not deterministic
/// across repetitions the corpus has a bigger problem than its timing.
///
/// # Errors
///
/// When the process cannot be spawned.
pub fn run_repeatedly<S: AsRef<OsStr>>(
    program: &str,
    args: &[S],
    cwd: Option<&Path>,
    timeout: Duration,
    repeats: u32,
) -> io::Result<Outcome> {
    let mut best = run(program, args, cwd, timeout)?;
    if !best.ok {
        return Ok(best);
    }
    for _ in 1..repeats.max(1) {
        let again = run(program, args, cwd, timeout)?;
        if !again.ok {
            return Ok(again);
        }
        best.micros = best.micros.min(again.micros);
        // The largest across the repetitions rather than the one that went with the fastest,
        // because the repetitions run the same program on the same input and a difference
        // between them is the sampler having missed something, not the program having changed.
        best.peak_bytes = best.peak_bytes.max(again.peak_bytes);
    }
    Ok(best)
}

/// Reads a pipe to the end, turning anything that is not text into replacement characters.
fn drain(pipe: Option<&mut impl Read>) -> String {
    let Some(pipe) = pipe else {
        return String::new();
    };
    let mut bytes = Vec::new();
    if pipe.read_to_end(&mut bytes).is_err() {
        return String::new();
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::memory;
    use std::time::Duration;

    const PATIENT: Duration = Duration::from_secs(10);

    #[test]
    fn a_program_that_works_comes_back_with_what_it_printed() {
        let out = run("/bin/sh", &["-c", "printf hello; printf oops 1>&2"], None, PATIENT).unwrap();
        assert!(out.ok);
        assert_eq!(out.status, 0);
        assert_eq!(out.stdout, "hello");
        assert_eq!(out.stderr, "oops");
        assert!(!out.timed_out);
    }

    #[test]
    fn a_program_that_fails_comes_back_with_its_status_and_not_an_error() {
        let out = run("/bin/sh", &["-c", "exit 3"], None, PATIENT).unwrap();
        assert!(!out.ok);
        assert_eq!(out.status, 3);
    }

    #[test]
    fn a_program_that_never_stops_is_killed_and_says_so() {
        let out = run("/bin/sh", &["-c", "sleep 30"], None, Duration::from_millis(200)).unwrap();
        assert!(out.timed_out);
        assert!(!out.ok);
        assert_eq!(out.status, -1);
        assert!(out.micros < 5_000_000, "the kill took {} microseconds", out.micros);
    }

    #[test]
    fn a_program_that_writes_more_than_a_pipe_holds_does_not_deadlock() {
        // A pipe buffer is sixty four kilobytes on most systems. A harness that polls for
        // exit without draining the pipe hangs here, and it hangs on exactly the compilers
        // that are the most interesting to run, the ones that print a lot.
        let out = run(
            "/bin/sh",
            &["-c", "i=0; while [ $i -lt 20000 ]; do echo aaaaaaaaaaaaaaaaaaaa; i=$((i+1)); done"],
            None,
            Duration::from_secs(30),
        )
        .unwrap();
        assert!(out.ok);
        assert_eq!(out.stdout.lines().count(), 20_000);
    }

    #[test]
    fn a_program_that_lives_long_enough_to_be_looked_at_reports_its_memory() {
        // Half a second is fifty polls at the sampling interval, so on a platform that can
        // answer at all there is no version of this that misses the process.
        let out = run("/bin/sh", &["-c", "sleep 0.5"], None, PATIENT).unwrap();
        assert!(out.ok);
        if memory::is_available() {
            let bytes = out.peak_bytes.expect("half a second is fifty chances to look");
            assert!(bytes > 0, "a shell that ran cannot have peaked at nothing");
        } else {
            assert_eq!(out.peak_bytes, None, "a platform that cannot measure must not invent");
        }
    }

    #[test]
    fn a_program_that_could_not_be_started_has_no_memory_number() {
        let failed = super::Outcome::failed_to_start(&std::io::Error::other("no"));
        assert_eq!(failed.peak_bytes, None);
    }

    #[test]
    fn a_program_that_does_not_exist_is_an_error_rather_than_a_silent_pass() {
        let error = run("/definitely/not/a/program", &[""; 0], None, PATIENT).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    }
}
