//! Counting the instructions a program retires, on a machine that will say.
//!
//! The corpus times every program it runs and the times are worth very little. The programs
//! finish in about a millisecond, most of which is the kernel starting a process, and the
//! repetitions of one binary on a quiet six core machine spread by a factor of four. That is
//! not a thing more repetitions fix, because the spread is not noise around a true time, it is
//! the machine doing other work in the middle of the measurement.
//!
//! An instruction count has none of that in it. It is what the program did, not how long the
//! machine took to let it do it, and on the same machine it reproduces to within one
//! instruction in a hundred thousand. So where the kernel will hand one over, the corpus takes
//! it, and the reports compare that instead of asking anybody to believe a wall clock.
//!
//! This is `perf stat` and a small parser rather than a crate or a syscall. The syscall is
//! `perf_event_open`, which needs `unsafe` and a page of ring buffer handling, and this crate
//! forbids `unsafe` for good reasons. Shelling out costs a process per measurement, which is
//! nothing next to the compile that produced the program in the first place.
//!
//! Everything here answers `None` rather than failing when the machine will not say. A laptop
//! without `perf`, a container without the permission, and a virtual machine with no counters
//! are all ordinary places to run this corpus, and none of them should turn into an error
//! about a measurement nobody asked for. See tamnd/rucc-corpus#8.

use crate::exec;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;

/// What the counter is asked for.
///
/// The user half of the count and not the kernel's, because the kernel's half is the same
/// system calls on both sides of a comparison and its cost depends on what else the machine is
/// doing. The point of counting instead of timing is to leave that out.
const EVENT: &str = "instructions:u";

/// How many times a program is measured.
///
/// Five, and the smallest is kept, for the same reason the timing keeps five and takes the
/// fastest. The counts are not a bell curve around a true value. Twenty five readings of one
/// unchanged program on a quiet machine came back 112164 nineteen times and 112183 or above
/// six times, so there is a floor the program cannot go below and a tail above it of work the
/// program did not ask for. The floor is the answer. Two readings land on it about ninety four
/// times in a hundred and five land on it about everywhere, and the difference showed up as a
/// couple of hundred instructions of drift in a facet total between two runs of the same
/// binaries.
pub const MEASUREMENTS: u32 = 5;

/// Whether this machine will count instructions at all.
///
/// Asked once and remembered, because the answer cannot change while a run is going and asking
/// it per case would put two thousand processes on a machine that has already said no.
#[must_use]
pub fn available() -> bool {
    static ANSWER: OnceLock<bool> = OnceLock::new();
    *ANSWER.get_or_init(|| count::<&str>("/bin/true", &[], None).is_some())
}

/// How many instructions a program retired, or `None` when the machine would not say.
///
/// The program's own output is thrown away here. The answer the case is judged on comes from
/// the ordinary run, which happens whether or not any of this works, so a counter that goes
/// wrong costs a number in a table and never a verdict.
#[must_use]
pub fn count<S: AsRef<OsStr>>(program: &str, args: &[S], cwd: Option<&Path>) -> Option<u64> {
    // Long enough for a program that is being measured rather than one that has hung. The
    // ordinary run has already decided whether this program terminates.
    const TIMEOUT: Duration = Duration::from_secs(30);
    let mut full: Vec<String> = vec![
        "stat".to_owned(),
        "-x,".to_owned(),
        "-e".to_owned(),
        EVENT.to_owned(),
        "--".to_owned(),
        program.to_owned(),
    ];
    full.extend(args.iter().map(|arg| arg.as_ref().to_string_lossy().into_owned()));
    let outcome = exec::run("perf", &full, cwd, TIMEOUT).ok()?;
    if !outcome.ok {
        return None;
    }
    parse(&outcome.stderr)
}

/// Every measurement of one program, fastest first, or empty when nobody could count.
#[must_use]
pub fn count_repeatedly<S: AsRef<OsStr>>(
    program: &str,
    args: &[S],
    cwd: Option<&Path>,
) -> Vec<u64> {
    if !available() {
        return Vec::new();
    }
    let mut counts = Vec::new();
    for _ in 0..MEASUREMENTS {
        let Some(one) = count(program, args, cwd) else {
            // One that fails after one that worked is a machine that has stopped counting
            // partway through, and half a measurement is worse than none.
            return Vec::new();
        };
        counts.push(one);
    }
    counts
}

/// The count out of what `perf stat -x,` wrote, when it wrote one.
///
/// The line is the count, an empty field for the unit, the event name, and then the time the
/// counter was enabled for. A count the kernel could not take is written as `<not counted>` or
/// `<not supported>` in the first field, which is why this parses a number rather than trusting
/// the line to be there.
fn parse(text: &str) -> Option<u64> {
    for line in text.lines() {
        let mut fields = line.split(',');
        let value = fields.next()?;
        let _unit = fields.next();
        let event = fields.next().unwrap_or_default();
        if !event.starts_with("instructions") {
            continue;
        }
        return value.trim().parse::<u64>().ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{available, count_repeatedly, parse};

    #[test]
    fn a_count_is_read_out_of_the_line_perf_writes() {
        assert_eq!(parse("112183,,instructions:u,1105459,100.00,,\n"), Some(112_183));
        // The event goes on the line whatever else is around it.
        assert_eq!(parse("noise\n\n4096,,instructions:u,900,100.00,,"), Some(4096));
    }

    #[test]
    fn a_count_nobody_could_take_reads_as_missing_rather_than_as_nought() {
        assert_eq!(parse("<not counted>,,instructions:u,0,0.00,,"), None);
        assert_eq!(parse("<not supported>,,instructions:u,0,0.00,,"), None);
        // A machine with no perf at all writes nothing anybody can use.
        assert_eq!(parse(""), None);
        assert_eq!(parse("perf: command not found"), None);
        // And an event that is not the one that was asked for is not the answer to it.
        assert_eq!(parse("500,,cycles:u,900,100.00,,"), None);
    }

    #[test]
    fn a_machine_that_will_not_count_says_so_instead_of_failing() {
        // Whichever this machine is, asking has to be safe and the two answers have to agree.
        // This is the one test here that runs on the machine rather than on a fixture, and it
        // is deliberately not an assertion about whether that machine has perf.
        let counts = count_repeatedly::<&str>("/bin/true", &[], None);
        if available() {
            assert_eq!(counts.len(), super::MEASUREMENTS as usize);
            assert!(counts.iter().all(|&one| one > 0), "{counts:?}");
        } else {
            assert!(counts.is_empty());
        }
    }
}
