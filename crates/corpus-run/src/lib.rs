//! Runs the corpus.
//!
//! One job is one case, one toolchain, one level. The jobs are independent, so they are run
//! on as many threads as the machine has, and the records come back in the order the jobs
//! were listed rather than the order they finished. A run of the same corpus with the same
//! toolchains produces the same file, which is what makes two runs comparable at all.
//!
//! Nothing here decides what a good result is. It compiles, it runs, it judges each case
//! against the answer the generator computed, and it writes down what happened. Deciding
//! whether the numbers add up to a claim is the reporter's job, and keeping those two apart
//! means a claim can be re-argued from a run that is already on disk.

#![forbid(unsafe_code)]

pub mod compare;
pub mod compile;
pub mod exec;
pub mod insight;
pub mod memory;
pub mod object;
pub mod toolchain;

use corpus_model::{Case, Finding, Level, Manifest, RunRecord, Toolchain, Verdict};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use toolchain::Spec;

/// What to run, and how.
#[derive(Debug, Clone)]
pub struct Plan {
    /// The compilers under test, in report order.
    pub specs: Vec<Spec>,
    /// The levels to build at.
    pub levels: Vec<Level>,
    /// How many times each program is run to get a time.
    pub repeats: u32,
    /// How many jobs to run at once.
    pub jobs: usize,
    /// Where the working directories go.
    pub root: PathBuf,
    /// Whether to keep the working directory of a case that passed.
    ///
    /// A failure is always kept, because the source and the diagnostics are the first thing
    /// anybody wants. Keeping the passes as well is thousands of directories nobody reads.
    pub keep_passes: bool,
    /// Cases carrying any of these tags are recorded as skipped rather than run.
    pub exclude_tags: Vec<String>,
}

impl Plan {
    /// A plan with sensible defaults, running under this directory.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            specs: toolchain::defaults(),
            levels: Level::ALL.to_vec(),
            repeats: compile::REPEATS,
            jobs: std::thread::available_parallelism().map_or(4, std::num::NonZero::get),
            root: root.into(),
            keep_passes: false,
            exclude_tags: Vec::new(),
        }
    }

    /// Whether a case is excluded by a tag.
    #[must_use]
    pub fn excludes(&self, case: &Case) -> bool {
        self.exclude_tags.iter().any(|tag| case.has_tag(tag))
    }
}

/// Everything one run produced.
#[derive(Debug, Clone)]
pub struct Run {
    /// The compilers as they described themselves.
    pub toolchains: Vec<Toolchain>,
    /// One record per job, in job order.
    pub records: Vec<RunRecord>,
    /// The verdict for each record, by record key.
    pub verdicts: BTreeMap<String, Verdict>,
    /// Everything that went wrong, worst first.
    pub findings: Vec<Finding>,
}

impl Run {
    /// The record for one case, toolchain and level.
    #[must_use]
    pub fn record(&self, case: &str, toolchain: &str, level: Level) -> Option<&RunRecord> {
        let key = format!("{case}|{toolchain}|{}", level.name());
        self.records.iter().find(|record| record.key() == key)
    }

    /// The verdict for a record.
    #[must_use]
    pub fn verdict(&self, record: &RunRecord) -> Verdict {
        self.verdicts.get(&record.key()).copied().unwrap_or(Verdict::Skipped)
    }

    /// The toolchain that the others are measured against.
    #[must_use]
    pub fn reference(&self) -> Option<&Toolchain> {
        self.toolchains.iter().find(|t| t.reference)
    }

    /// Whether anything happened that should turn the build red.
    ///
    /// A declared gap does not. It is in `findings` because somebody has to act on it eventually,
    /// and it is not a failure because the compiler already told us about it, so counting it here
    /// would make the exit status disagree with the correctness target two lines above it.
    #[must_use]
    pub fn failed(&self) -> bool {
        self.findings.iter().any(|finding| finding.verdict.is_failure())
    }

    /// How many results are bugs.
    #[must_use]
    pub fn failures(&self) -> usize {
        self.findings.iter().filter(|finding| finding.verdict.is_failure()).count()
    }

    /// How many results are gaps a compiler declared.
    #[must_use]
    pub fn gaps(&self) -> usize {
        self.findings.iter().filter(|finding| finding.verdict.is_gap()).count()
    }
}

/// How far along a run is.
#[derive(Debug, Clone, Copy)]
pub struct Progress {
    /// How many jobs have finished.
    pub done: usize,
    /// How many there are in total.
    pub total: usize,
}

/// One unit of work.
#[derive(Debug, Clone, Copy)]
struct Job {
    case: usize,
    spec: usize,
    level: Level,
}

/// Runs the corpus and judges every case.
///
/// The progress callback is invoked from whichever thread finished a job, so it has to be
/// cheap and it has to be safe to call from several threads at once. Printing a line is fine.
///
/// # Errors
///
/// When a compiler cannot be found or will not say what version it is. Everything else that
/// can go wrong is a result rather than an error, and ends up in the findings.
pub fn execute(
    manifest: &Manifest,
    plan: &Plan,
    progress: &(dyn Fn(Progress, &RunRecord, Verdict) + Sync),
) -> Result<Run, String> {
    let toolchains: Vec<Toolchain> =
        plan.specs.iter().map(Spec::describe).collect::<Result<_, _>>()?;
    let jobs = schedule(manifest, plan);
    let total = jobs.len();

    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let collected: Mutex<Vec<(usize, RunRecord, Verdict)>> = Mutex::new(Vec::with_capacity(total));

    let workers = plan.jobs.clamp(1, total.max(1));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let at = next.fetch_add(1, Ordering::Relaxed);
                    let Some(job) = jobs.get(at) else {
                        return;
                    };
                    let case = &manifest.cases[job.case];
                    let spec = &plan.specs[job.spec];
                    let (record, verdict) = run_one(case, spec, job.level, plan);
                    let finished = done.fetch_add(1, Ordering::Relaxed) + 1;
                    progress(Progress { done: finished, total }, &record, verdict);
                    if let Ok(mut held) = collected.lock() {
                        held.push((at, record, verdict));
                    }
                }
            });
        }
    });

    let mut collected =
        collected.into_inner().map_err(|_| "a worker thread panicked".to_owned())?;
    collected.sort_by_key(|(at, _, _)| *at);

    let mut records = Vec::with_capacity(collected.len());
    let mut verdicts = BTreeMap::new();
    let mut findings = Vec::new();
    for (at, record, verdict) in collected {
        let case = &manifest.cases[jobs[at].case];
        verdicts.insert(record.key(), verdict);
        if let Some(found) = compare::finding(case, &record, verdict) {
            findings.push(found);
        }
        records.push(record);
    }
    findings.sort_by(|a, b| {
        a.verdict
            .cmp(&b.verdict)
            .then_with(|| a.case.cmp(&b.case))
            .then_with(|| a.toolchain.cmp(&b.toolchain))
    });

    Ok(Run { toolchains, records, verdicts, findings })
}

/// Lists every job, in a fixed order.
fn schedule(manifest: &Manifest, plan: &Plan) -> Vec<Job> {
    let mut jobs = Vec::new();
    for (case_at, case) in manifest.cases.iter().enumerate() {
        for (spec_at, _) in plan.specs.iter().enumerate() {
            for &level in &plan.levels {
                // A program that is supposed to be rejected is rejected at every level, so
                // building it five times would be four copies of the same diagnostic. It is
                // built once, at the level where the diagnostics are plainest.
                if case.expect_is_rejection() && level != Level::O0 {
                    continue;
                }
                jobs.push(Job { case: case_at, spec: spec_at, level });
            }
        }
    }
    jobs
}

fn run_one(case: &Case, spec: &Spec, level: Level, plan: &Plan) -> (RunRecord, Verdict) {
    if plan.excludes(case) {
        return (RunRecord::skipped(case, &spec.id, level), Verdict::Skipped);
    }
    let dir = compile::work_dir(&plan.root, case, &spec.id, level);
    match compile::build_and_run(case, spec, level, &dir, plan.repeats) {
        Ok(built) => {
            let record = compile::record(case, &spec.id, level, built);
            let verdict = compare::judge(case, &record);
            tidy(&dir, verdict, plan);
            (record, verdict)
        }
        Err(message) => {
            // The harness itself could not do its job. That is not the compiler's fault, so
            // it is recorded as a crash with the reason in the diagnostics rather than
            // silently dropped, which would make the totals in the report not add up.
            let mut record = RunRecord::skipped(case, &spec.id, level);
            record.compile.diagnostics = message;
            (record, Verdict::Crashed)
        }
    }
}

/// Throws away the working directory of a case nobody will want to look at.
fn tidy(dir: &Path, verdict: Verdict, plan: &Plan) {
    if plan.keep_passes || verdict.is_failure() {
        return;
    }
    let _ = std::fs::remove_dir_all(dir);
}

#[cfg(test)]
mod tests {
    use super::{Plan, execute};
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Level, Manifest, Verdict};

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("rucc-corpus-run-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn prints(value: &str) -> Case {
        Case::new(
            Facet::Baseline,
            Axes::of([("program", value)]),
            Dialect::C17,
            format!(
                "int printf(const char *, ...);\nint main(void) {{\n    printf(\"{value}\\n\");\n    return 0;\n}}\n"
            ),
            Expect::Output(format!("{value}\n")),
        )
    }

    fn plan(dir: &std::path::Path) -> Plan {
        let mut plan = Plan::new(dir);
        plan.specs = vec![super::toolchain::Spec::parse("cc")];
        plan.levels = vec![Level::O0, Level::O2];
        plan.repeats = 1;
        plan
    }

    #[test]
    fn a_small_corpus_runs_and_every_case_gets_a_record_at_every_level() {
        let dir = scratch("small");
        let manifest = Manifest::new(vec![prints("1"), prints("2"), prints("3")]).unwrap();
        let run = execute(&manifest, &plan(&dir), &|_, _, _| {}).unwrap();
        assert_eq!(run.records.len(), 6);
        assert!(!run.failed(), "unexpected findings: {:?}", run.findings);
        for record in &run.records {
            assert_eq!(run.verdict(record), Verdict::Pass);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_case_with_the_wrong_expected_answer_produces_exactly_one_finding_per_level() {
        let dir = scratch("wrong");
        let mut liar = prints("1");
        liar.expect = Expect::Output("this is not what it prints\n".to_owned());
        let manifest = Manifest::new(vec![liar]).unwrap();
        let run = execute(&manifest, &plan(&dir), &|_, _, _| {}).unwrap();
        assert!(run.failed());
        assert_eq!(run.findings.len(), 2);
        for found in &run.findings {
            assert_eq!(found.verdict, Verdict::Wrong);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_program_that_must_not_compile_is_built_once_and_not_five_times() {
        let dir = scratch("rejected");
        let case = Case::new(
            Facet::Frontend,
            Axes::of([("rejected", "undeclared")]),
            Dialect::C17,
            "int main(void) {\n    return missing_name;\n}\n",
            Expect::Rejected("undeclared".to_owned()),
        );
        let manifest = Manifest::new(vec![case]).unwrap();
        let run = execute(&manifest, &plan(&dir), &|_, _, _| {}).unwrap();
        assert_eq!(run.records.len(), 1);
        assert_eq!(run.records[0].level, Level::O0);
        assert!(!run.failed(), "unexpected findings: {:?}", run.findings);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_tag_can_take_a_case_out_of_a_run_without_taking_it_out_of_the_corpus() {
        let dir = scratch("tagged");
        let case = prints("1").tagged(&["gnu"]);
        let manifest = Manifest::new(vec![case]).unwrap();
        let mut plan = plan(&dir);
        plan.exclude_tags = vec!["gnu".to_owned()];
        let run = execute(&manifest, &plan, &|_, _, _| {}).unwrap();
        assert_eq!(run.records.len(), 2);
        for record in &run.records {
            assert_eq!(run.verdict(record), Verdict::Skipped);
        }
        assert!(!run.failed());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_records_come_back_in_job_order_however_the_threads_finished() {
        let dir = scratch("order");
        let cases: Vec<Case> = (0..12).map(|n| prints(&n.to_string())).collect();
        let manifest = Manifest::new(cases).unwrap();
        let mut plan = plan(&dir);
        plan.jobs = 8;
        let one = execute(&manifest, &plan, &|_, _, _| {}).unwrap();
        let two = execute(&manifest, &plan, &|_, _, _| {}).unwrap();
        let keys_one: Vec<String> = one.records.iter().map(|r| r.key()).collect();
        let keys_two: Vec<String> = two.records.iter().map(|r| r.key()).collect();
        assert_eq!(keys_one, keys_two);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn progress_is_reported_once_per_job_and_counts_up_to_the_total() {
        let dir = scratch("progress");
        let manifest = Manifest::new(vec![prints("1"), prints("2")]).unwrap();
        let seen = std::sync::Mutex::new(Vec::new());
        let run = execute(&manifest, &plan(&dir), &|progress, _, _| {
            seen.lock().unwrap().push((progress.done, progress.total));
        })
        .unwrap();
        let mut seen = seen.into_inner().unwrap();
        seen.sort_unstable();
        assert_eq!(seen.len(), run.records.len());
        assert_eq!(seen.first().unwrap().0, 1);
        assert_eq!(seen.last().unwrap().0, seen.len());
        assert!(seen.iter().all(|(_, total)| *total == seen.len()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_compiler_that_is_not_installed_stops_the_run_instead_of_filling_the_report() {
        let dir = scratch("missing");
        let manifest = Manifest::new(vec![prints("1")]).unwrap();
        let mut plan = plan(&dir);
        plan.specs = vec![super::toolchain::Spec::parse("definitely-not-a-compiler-9999")];
        let error = execute(&manifest, &plan, &|_, _, _| {}).unwrap_err();
        assert!(error.contains("definitely-not-a-compiler-9999"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
