//! How much memory a build took.
//!
//! A compiler that gets the right answer in six gigabytes is not the same compiler as one that
//! gets it in two hundred megabytes, and nothing else the corpus records would tell them apart.
//! The cases here are small on purpose, so the absolute numbers are small, but the ratio
//! against the reference is the same kind of trend signal that code size is: cheap to collect
//! because the run is happening anyway, and worth watching between two commits of the compiler
//! rather than worth quoting on its own.
//!
//! Three things about how the number is arrived at, because it is not what the kernel would
//! hand back from `wait4` and pretending otherwise would be worse than not measuring.
//!
//! **It is the high water mark rather than a sample of the resident set.** Linux keeps `VmHWM`
//! per process and only ever raises it, so one read at any point during a process's life gets
//! the largest it had been up to then. That makes the poll interval a question of which
//! processes are seen at all rather than of catching them at their peak.
//!
//! **It is the whole tree and not just the child.** `gcc` is a driver that spawns `cc1`, `as`
//! and `ld`, and the driver itself never allocates anything interesting. So the walk starts at
//! the process that was spawned and follows `children` down. A process that is born and dies
//! entirely between two polls is missed, and the number is an underestimate that says nothing
//! about being one. That is the honest limit of measuring from the outside without an unsafe
//! call, and it is why the interval is short.
//!
//! **It is the largest single process and not the sum.** Nothing here runs a parallel build, so
//! in practice the two are nearly the same, but the question worth asking about a compiler is
//! whether one translation unit fits in a machine of a given size, and a sum answers a
//! different question.
//!
//! On anything that is not Linux there is no cheap way to ask, so the answer is `None` and the
//! report says the number is missing. Reading it out of `ps` would mean spawning a process
//! every few milliseconds, and the timing numbers this corpus publishes are worth more than the
//! memory numbers would be.

use std::time::Duration;

/// How often the tree is walked while a process runs.
///
/// Short, because the risk is not catching a process at the wrong moment, it is not catching it
/// at all. A `cc1` invocation on one of these cases lives for tens of milliseconds.
pub const INTERVAL: Duration = Duration::from_millis(10);

/// Whether this platform can answer at all.
///
/// Worth asking before a report claims a compiler used no memory on a machine where nobody
/// looked.
#[must_use]
pub const fn is_available() -> bool {
    cfg!(target_os = "linux")
}

/// The largest high water mark, in bytes, of any process in the tree rooted at `pid`.
///
/// `None` when the platform cannot say, or when every process in the tree had already exited by
/// the time the walk reached it.
#[must_use]
pub fn high_water(pid: u32) -> Option<u64> {
    platform::high_water(pid)
}

#[cfg(target_os = "linux")]
mod platform {
    /// Walks the tree breadth first and keeps the largest `VmHWM` found.
    ///
    /// Breadth first with an explicit queue rather than recursion, because the depth is
    /// whatever the compiler under test decided to spawn and a corpus harness has no business
    /// blowing its stack over it.
    pub(super) fn high_water(pid: u32) -> Option<u64> {
        let mut queue = vec![pid];
        let mut seen = 0usize;
        let mut largest = None;
        while let Some(pid) = queue.pop() {
            // A process cannot be its own ancestor, so this only trips if the kernel handed
            // back something impossible or a pid was reused mid walk. Either way, stop.
            seen += 1;
            if seen > 4096 {
                break;
            }
            if let Ok(status) = std::fs::read_to_string(format!("/proc/{pid}/status")) {
                largest = largest.max(super::kilobytes_of(&status, "VmHWM:").map(|kb| kb * 1024));
            }
            queue.extend(children_of(pid));
        }
        largest
    }

    /// Every direct child of a process, across all of its threads.
    ///
    /// `/proc/<pid>/task/<tid>/children` is a space separated list, and a process with several
    /// threads has one such file per thread. Missing files mean the process exited while the
    /// walk was in it, which is expected and is not an error.
    fn children_of(pid: u32) -> Vec<u32> {
        let Ok(tasks) = std::fs::read_dir(format!("/proc/{pid}/task")) else {
            return Vec::new();
        };
        let mut found = Vec::new();
        for task in tasks.flatten() {
            let Ok(list) = std::fs::read_to_string(task.path().join("children")) else {
                continue;
            };
            found.extend(list.split_whitespace().filter_map(|word| word.parse::<u32>().ok()));
        }
        found
    }
}

#[cfg(not(target_os = "linux"))]
mod platform {
    pub(super) const fn high_water(_pid: u32) -> Option<u64> {
        None
    }
}

/// Pulls the number off one of the size lines in `/proc/<pid>/status`.
///
/// The unit in that file is kibibytes and it is stated on every line, which is why this reads
/// it rather than `statm`. `statm` counts in pages and getting the page size means an unsafe
/// call or a guess, and a guess that is wrong by a factor of four is worse than no number.
#[cfg(any(target_os = "linux", test))]
fn kilobytes_of(status: &str, field: &str) -> Option<u64> {
    status
        .lines()
        .find_map(|line| line.strip_prefix(field))?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::{high_water, is_available, kilobytes_of};

    #[test]
    fn the_high_water_line_is_read_and_the_others_are_left_alone() {
        let status =
            "Name:\tcc1\nVmPeak:\t  914088 kB\nVmSize:\t  914088 kB\nVmHWM:\t  221056 kB\n";
        assert_eq!(kilobytes_of(status, "VmHWM:"), Some(221_056));
        assert_eq!(kilobytes_of(status, "VmRSS:"), None);
    }

    #[test]
    fn a_status_file_without_the_field_gives_nothing_rather_than_nought() {
        assert_eq!(kilobytes_of("Name:\tsh\n", "VmHWM:"), None);
        assert_eq!(kilobytes_of("", "VmHWM:"), None);
    }

    #[test]
    fn a_pid_that_is_not_running_has_no_number() {
        // Pid nought is never a process. On a platform that cannot answer at all this is the
        // same answer for a different reason, which is the point of the assertion being here.
        assert_eq!(high_water(0), None);
    }

    #[test]
    fn this_very_process_is_using_memory_on_a_platform_that_can_say_so() {
        let mine = high_water(std::process::id());
        if is_available() {
            let bytes = mine.expect("linux can read its own status file");
            assert!(bytes > 1 << 20, "a rust test binary claims to have peaked at {bytes} bytes");
        } else {
            assert_eq!(mine, None, "a platform that cannot measure must not invent a number");
        }
    }
}
