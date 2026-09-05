//! What the person watching the run sees while it is happening.
//!
//! A run of the full corpus against two compilers at five levels is thousands of processes
//! and takes minutes. A tool that prints nothing for four minutes looks hung, and a tool that
//! prints a line per case scrolls the failures off the screen. So: one line that rewrites
//! itself, and a permanent line for anything that went wrong.
//!
//! The rewriting only happens when standard error is a terminal. In CI it is not, and a
//! progress bar written to a log file is a hundred thousand carriage returns nobody can read.

use corpus_model::{RunRecord, Verdict};
use corpus_run::Progress;
use std::io::{IsTerminal, Write};
use std::sync::Mutex;
use std::time::Instant;

/// Prints progress as a run goes.
#[derive(Debug)]
pub struct Watcher {
    interactive: bool,
    quiet: bool,
    started: Instant,
    state: Mutex<State>,
}

#[derive(Debug, Default)]
struct State {
    failures: usize,
    width: usize,
}

impl Watcher {
    /// A watcher that decides for itself whether it is talking to a person.
    #[must_use]
    pub fn new(quiet: bool) -> Self {
        Self {
            interactive: std::io::stderr().is_terminal(),
            quiet,
            started: Instant::now(),
            state: Mutex::new(State::default()),
        }
    }

    /// Called once per finished job, from whichever thread finished it.
    pub fn saw(&self, progress: Progress, record: &RunRecord, verdict: Verdict) {
        if self.quiet {
            return;
        }
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        if verdict.is_failure() {
            state.failures += 1;
            self.clear(&mut state);
            let _ = writeln!(
                std::io::stderr(),
                "{:>9}  {}  {} at {}",
                verdict.name(),
                record.case,
                record.toolchain,
                record.level.flag()
            );
        }
        if !self.interactive {
            // In a log file, say something occasionally so a stuck run is visible, and say
            // nothing the rest of the time.
            if progress.done % 500 == 0 || progress.done == progress.total {
                let _ = writeln!(std::io::stderr(), "{}", self.line(progress, state.failures));
            }
            return;
        }
        let line = self.line(progress, state.failures);
        state.width = line.chars().count();
        let _ = write!(std::io::stderr(), "\r{line}");
        let _ = std::io::stderr().flush();
    }

    /// Called once when the run is over, to leave the terminal tidy.
    pub fn finished(&self) {
        if self.quiet || !self.interactive {
            return;
        }
        if let Ok(mut state) = self.state.lock() {
            self.clear(&mut state);
        }
    }

    /// Wipes the progress line so a permanent line can be written over it.
    fn clear(&self, state: &mut State) {
        if !self.interactive || state.width == 0 {
            return;
        }
        let _ = write!(std::io::stderr(), "\r{}\r", " ".repeat(state.width));
        state.width = 0;
    }

    /// The one line that keeps rewriting itself.
    fn line(&self, progress: Progress, failures: usize) -> String {
        let elapsed = self.started.elapsed().as_secs();
        let per_second = if elapsed > 0 { progress.done as u64 / elapsed.max(1) } else { 0 };
        let left = if per_second > 0 {
            let remaining = (progress.total - progress.done) as u64 / per_second.max(1);
            format!(", about {} left", duration(remaining))
        } else {
            String::new()
        };
        format!(
            "{} of {} builds, {} failures, {} elapsed{}",
            progress.done,
            progress.total,
            failures,
            duration(elapsed),
            left
        )
    }
}

/// A number of seconds, written the short way.
#[must_use]
pub fn duration(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{seconds}s");
    }
    if seconds < 3600 {
        return format!("{}m{:02}s", seconds / 60, seconds % 60);
    }
    format!("{}h{:02}m", seconds / 3600, (seconds % 3600) / 60)
}

#[cfg(test)]
mod tests {
    use super::{Watcher, duration};
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Level, RunRecord, Verdict};
    use corpus_run::Progress;

    #[test]
    fn a_length_of_time_is_written_the_way_a_person_reads_one() {
        assert_eq!(duration(0), "0s");
        assert_eq!(duration(45), "45s");
        assert_eq!(duration(60), "1m00s");
        assert_eq!(duration(3599), "59m59s");
        assert_eq!(duration(3600), "1h00m");
        assert_eq!(duration(7900), "2h11m");
    }

    #[test]
    fn a_quiet_watcher_says_nothing_and_still_counts() {
        let case = Case::new(
            Facet::Baseline,
            Axes::of([("program", "one")]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Output("1\n".to_owned()),
        );
        let record = RunRecord::skipped(&case, "rucc", Level::O2);
        let watcher = Watcher::new(true);
        watcher.saw(Progress { done: 1, total: 2 }, &record, Verdict::Wrong);
        watcher.saw(Progress { done: 2, total: 2 }, &record, Verdict::Pass);
        watcher.finished();
    }

    #[test]
    fn the_progress_line_says_how_far_along_it_is_and_how_many_broke() {
        let watcher = Watcher::new(false);
        let line = watcher.line(Progress { done: 40, total: 100 }, 3);
        assert!(line.contains("40 of 100 builds"));
        assert!(line.contains("3 failures"));
    }
}
