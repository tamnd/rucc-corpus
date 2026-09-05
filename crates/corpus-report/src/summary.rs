//! Turning thousands of records into the handful of numbers a claim is made from.
//!
//! Both reports are written from this, so the human one and the machine one cannot disagree.
//! That is not a theoretical worry. A report where the table says one thing and the JSON says
//! another is worse than no report, because somebody will quote whichever half suits them.
//!
//! # Why the median
//!
//! Every ratio here is a median rather than a mean. The corpus contains programs whose text
//! is a few dozen bytes, and on those a single extra instruction is a twenty percent
//! regression. A mean lets a handful of tiny cases decide the headline number for the whole
//! corpus. The median says what a typical case looks like, which is the thing the ten percent
//! target in spec 16 was actually about.

use corpus_model::{Facet, Level, Phase, RunRecord, Verdict};
use corpus_run::{Run, compare};
use std::collections::BTreeMap;

/// How many cases came out each way.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tally {
    /// Did what it was supposed to.
    pub pass: usize,
    /// Compiled, ran, printed the wrong thing.
    pub wrong: usize,
    /// Would not compile a valid program.
    pub rejected: usize,
    /// Compiled a program that is not valid C.
    pub accepted: usize,
    /// Died, hung, or was killed.
    pub crashed: usize,
    /// Was not run.
    pub skipped: usize,
}

impl Tally {
    /// Counts one verdict.
    pub fn add(&mut self, verdict: Verdict) {
        let slot = match verdict {
            Verdict::Pass => &mut self.pass,
            Verdict::Wrong => &mut self.wrong,
            Verdict::Rejected => &mut self.rejected,
            Verdict::Accepted => &mut self.accepted,
            Verdict::Crashed => &mut self.crashed,
            Verdict::Skipped => &mut self.skipped,
        };
        *slot += 1;
    }

    /// How many were actually run.
    #[must_use]
    pub const fn ran(&self) -> usize {
        self.pass + self.wrong + self.rejected + self.accepted + self.crashed
    }

    /// How many went wrong in any way.
    #[must_use]
    pub const fn failures(&self) -> usize {
        self.wrong + self.rejected + self.accepted + self.crashed
    }

    /// The share that passed, of those that ran.
    ///
    /// A facet where nothing ran scores nought rather than one. An empty run is not a clean
    /// run, and a report that says otherwise is the kind of thing that gets believed.
    #[must_use]
    pub fn pass_rate(&self) -> f64 {
        if self.ran() == 0 {
            return 0.0;
        }
        self.pass as f64 / self.ran() as f64
    }
}

/// What one compiler did with one facet.
#[derive(Debug, Clone, Default)]
pub struct FacetScore {
    /// Which compiler.
    pub toolchain: String,
    /// How the cases came out, over every level.
    pub tally: Tally,
    /// Median code size against the reference, at the headline level.
    pub size_ratio: Option<f64>,
    /// Median run time against the reference, at the headline level.
    pub speed_ratio: Option<f64>,
    /// Median compile time against the reference, at the headline level.
    pub compile_ratio: Option<f64>,
    /// How many cases had a measurable size ratio, which is how much the median is worth.
    pub compared: usize,
}

/// What every compiler did with one facet.
#[derive(Debug, Clone)]
pub struct FacetSummary {
    /// The facet.
    pub facet: Facet,
    /// The phase of the plan it belongs to.
    pub phase: Phase,
    /// How many cases the corpus has for it.
    pub cases: usize,
    /// One score per compiler, in the order the compilers were given.
    pub scores: Vec<FacetScore>,
    /// What the reference compiler said it optimized here.
    pub optimized: usize,
    /// What the reference compiler said it wanted to optimize and could not.
    pub missed: usize,
}

impl FacetSummary {
    /// The score for one compiler.
    #[must_use]
    pub fn score(&self, toolchain: &str) -> Option<&FacetScore> {
        self.scores.iter().find(|score| score.toolchain == toolchain)
    }
}

/// One of the claims the corpus exists to check.
#[derive(Debug, Clone)]
pub struct Target {
    /// What it is called in the spec.
    pub name: String,
    /// What the spec asks for, in one sentence.
    pub wanted: String,
    /// The number the spec sets, when it sets one.
    pub threshold: Option<f64>,
    /// The number this run produced.
    pub actual: Option<f64>,
    /// Whether the run met it.
    pub met: bool,
}

/// The whole run, boiled down.
#[derive(Debug, Clone)]
pub struct Summary {
    /// The digest of the corpus that was run, so a report names the corpus it is about.
    pub corpus_digest: String,
    /// How many cases there were.
    pub cases: usize,
    /// The compiler the others are measured against.
    pub reference: String,
    /// The compilers under test, reference first.
    pub toolchains: Vec<String>,
    /// The levels that were built.
    pub levels: Vec<Level>,
    /// Everything, per compiler.
    pub totals: BTreeMap<String, Tally>,
    /// Everything, per facet.
    pub facets: Vec<FacetSummary>,
    /// The claims, and whether this run supports them.
    pub targets: Vec<Target>,
}

impl Summary {
    /// Whether every case passed for every compiler.
    #[must_use]
    pub fn correct(&self) -> bool {
        self.totals.values().all(|tally| tally.failures() == 0)
    }

    /// The facets where a compiler is furthest behind the reference on code size.
    ///
    /// Spec 16 rule 3 says a report shows the losses next to the wins, so this is the list the
    /// human report leads the code quality section with. A report that only shows the wins is
    /// marketing, and nobody making decisions can use it.
    #[must_use]
    pub fn worst_facets(&self, toolchain: &str, count: usize) -> Vec<(&FacetSummary, f64)> {
        let mut ranked: Vec<(&FacetSummary, f64)> = self
            .facets
            .iter()
            .filter_map(|facet| Some((facet, facet.score(toolchain)?.size_ratio?)))
            .collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
        ranked.truncate(count);
        ranked
    }

    /// The facets where a compiler is furthest ahead.
    #[must_use]
    pub fn best_facets(&self, toolchain: &str, count: usize) -> Vec<(&FacetSummary, f64)> {
        let mut ranked: Vec<(&FacetSummary, f64)> = self
            .facets
            .iter()
            .filter_map(|facet| Some((facet, facet.score(toolchain)?.size_ratio?)))
            .collect();
        ranked.sort_by(|a, b| a.1.total_cmp(&b.1));
        ranked.truncate(count);
        ranked
    }

    /// The compilers under test, which is everything except the reference.
    #[must_use]
    pub fn under_test(&self) -> Vec<&str> {
        self.toolchains.iter().map(String::as_str).filter(|id| *id != self.reference).collect()
    }
}

/// The level the headline numbers are taken at.
const HEADLINE: Level = Level::O2;

/// Boils a run down.
///
/// The corpus digest is passed in rather than recomputed, because the report has to name the
/// corpus that was actually run and the run does not carry it.
#[must_use]
pub fn summarise(run: &Run, corpus_digest: &str, cases: usize) -> Summary {
    let reference = run.reference().map_or_else(String::new, |t| t.id.clone());
    let toolchains: Vec<String> = run.toolchains.iter().map(|t| t.id.clone()).collect();
    let levels = collect_levels(run);

    let mut totals: BTreeMap<String, Tally> = BTreeMap::new();
    for record in &run.records {
        totals.entry(record.toolchain.clone()).or_default().add(run.verdict(record));
    }

    let facets = summarise_facets(run, &toolchains, &reference);
    let targets = check_targets(&totals, &facets, &reference, &toolchains);

    Summary {
        corpus_digest: corpus_digest.to_owned(),
        cases,
        reference,
        toolchains,
        levels,
        totals,
        facets,
        targets,
    }
}

/// The levels that actually appear in the run, cheapest first.
fn collect_levels(run: &Run) -> Vec<Level> {
    let mut seen: Vec<Level> = run.records.iter().map(|record| record.level).collect();
    seen.sort_unstable();
    seen.dedup();
    seen
}

/// One summary per facet that has any records.
fn summarise_facets(run: &Run, toolchains: &[String], reference: &str) -> Vec<FacetSummary> {
    let mut by_facet: BTreeMap<Facet, Vec<&RunRecord>> = BTreeMap::new();
    for record in &run.records {
        by_facet.entry(record.facet).or_default().push(record);
    }

    let mut out = Vec::new();
    for (facet, records) in by_facet {
        let mut cases: Vec<&str> = records.iter().map(|record| record.case.as_str()).collect();
        cases.sort_unstable();
        cases.dedup();

        let mut optimized = 0;
        let mut missed = 0;
        for record in &records {
            if record.toolchain != reference {
                continue;
            }
            for insight in &record.insights {
                match insight.kind.as_str() {
                    "optimized" => optimized += 1,
                    "missed" => missed += 1,
                    _ => {}
                }
            }
        }

        let scores =
            toolchains.iter().map(|id| score_facet(run, &records, id, reference)).collect();

        out.push(FacetSummary {
            facet,
            phase: facet.phase(),
            cases: cases.len(),
            scores,
            optimized,
            missed,
        });
    }
    out
}

/// What one compiler did with one facet.
fn score_facet(run: &Run, records: &[&RunRecord], toolchain: &str, reference: &str) -> FacetScore {
    let mut tally = Tally::default();
    let mut sizes = Vec::new();
    let mut speeds = Vec::new();
    let mut compiles = Vec::new();

    for record in records {
        if record.toolchain != toolchain {
            continue;
        }
        tally.add(run.verdict(record));
        if record.level != HEADLINE {
            continue;
        }
        let Some(against) = run.record(&record.case, reference, record.level) else {
            continue;
        };
        if !compare::comparable(record.level, against.level) {
            continue;
        }
        if let Some(value) = compare::size_ratio(record, against) {
            sizes.push(value);
        }
        if let Some(value) = compare::speed_ratio(record, against) {
            speeds.push(value);
        }
        if let Some(value) = compare::compile_ratio(record, against) {
            compiles.push(value);
        }
    }

    FacetScore {
        toolchain: toolchain.to_owned(),
        tally,
        compared: sizes.len(),
        size_ratio: median(&mut sizes),
        speed_ratio: median(&mut speeds),
        compile_ratio: median(&mut compiles),
    }
}

/// The middle value, or `None` when there is nothing to take a middle of.
#[must_use]
pub fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    if values.len() % 2 == 1 {
        Some(values[middle])
    } else {
        Some(f64::midpoint(values[middle - 1], values[middle]))
    }
}

/// Checks each claim against the run.
fn check_targets(
    totals: &BTreeMap<String, Tally>,
    facets: &[FacetSummary],
    reference: &str,
    toolchains: &[String],
) -> Vec<Target> {
    let mut targets = Vec::new();

    let failures: usize = totals.values().map(Tally::failures).sum();
    targets.push(Target {
        name: "correctness".to_owned(),
        wanted:
            "every case prints the answer the generator computed, on every compiler, at every level"
                .to_owned(),
        threshold: Some(0.0),
        actual: Some(failures as f64),
        met: failures == 0,
    });

    for id in toolchains {
        if id == reference {
            continue;
        }
        let mut sizes: Vec<f64> =
            facets.iter().filter_map(|facet| facet.score(id)?.size_ratio).collect();
        let size = median(&mut sizes);
        targets.push(Target {
            name: format!("code-quality:{id}"),
            wanted: format!(
                "the code {id} produces at -O2 is within ten percent of what {reference} produces at -O2"
            ),
            threshold: Some(1.10),
            actual: size,
            met: size.is_some_and(|value| value <= 1.10),
        });

        let mut compiles: Vec<f64> =
            facets.iter().filter_map(|facet| facet.score(id)?.compile_ratio).collect();
        let speed = median(&mut compiles);
        targets.push(Target {
            name: format!("compile-throughput:{id}"),
            wanted: format!(
                "{id} compiles the corpus at least as fast as {reference} does, which is the corpus proxy for the throughput target in spec 00"
            ),
            threshold: Some(1.0),
            actual: speed,
            met: speed.is_some_and(|value| value <= 1.0),
        });
    }

    targets
}

#[cfg(test)]
mod tests {
    use super::{Tally, median, summarise};
    use corpus_model::{
        Axes, Case, Compile, Dialect, Execute, Expect, Facet, Level, Manifest, RunRecord,
        Toolchain, Verdict,
    };
    use corpus_run::Run;
    use std::collections::BTreeMap;

    fn case_for(facet: Facet, name: &str) -> Case {
        Case::new(
            facet,
            Axes::of([("shape", name)]),
            Dialect::C17,
            format!("int main(void) {{ return 0; }} /* {name} */\n"),
            Expect::Output("1\n".to_owned()),
        )
    }

    fn record_for(case: &Case, toolchain: &str, level: Level, text: u64, micros: u64) -> RunRecord {
        let mut record = RunRecord::skipped(case, toolchain, level);
        record.compile = Compile {
            ok: true,
            status: 0,
            micros: 1000,
            diagnostics: String::new(),
            bytes: text * 4,
            text_bytes: text,
        };
        record.execute =
            Execute { ok: true, status: 0, micros, repeats: 5, output: "1\n".to_owned() };
        record
    }

    fn run_of(records: Vec<RunRecord>) -> Run {
        let verdicts: BTreeMap<String, Verdict> =
            records.iter().map(|record| (record.key(), Verdict::Pass)).collect();
        Run {
            toolchains: vec![
                Toolchain {
                    id: "gcc-16".to_owned(),
                    program: "gcc-16".to_owned(),
                    version: "gcc (GCC) 16.0.0".to_owned(),
                    reference: true,
                },
                Toolchain {
                    id: "rucc".to_owned(),
                    program: "rucc".to_owned(),
                    version: "rucc 0.4.0".to_owned(),
                    reference: false,
                },
            ],
            records,
            verdicts,
            findings: Vec::new(),
        }
    }

    #[test]
    fn a_tally_counts_every_verdict_and_knows_which_ones_are_bugs() {
        let mut tally = Tally::default();
        for verdict in [Verdict::Pass, Verdict::Pass, Verdict::Wrong, Verdict::Skipped] {
            tally.add(verdict);
        }
        assert_eq!(tally.pass, 2);
        assert_eq!(tally.failures(), 1);
        assert_eq!(tally.ran(), 3);
        assert!((tally.pass_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn a_facet_where_nothing_ran_scores_nought_rather_than_a_clean_sheet() {
        let mut tally = Tally::default();
        tally.add(Verdict::Skipped);
        assert_eq!(tally.ran(), 0);
        assert_eq!(tally.pass_rate(), 0.0);
    }

    #[test]
    fn the_median_of_an_even_list_is_the_middle_of_the_two_middles() {
        assert_eq!(median(&mut [1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(median(&mut [1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(median(&mut [] as &mut [f64]), None);
        assert_eq!(median(&mut [7.0]), Some(7.0));
    }

    #[test]
    fn the_headline_ratio_is_a_median_so_a_handful_of_tiny_cases_cannot_set_it() {
        let cases: Vec<Case> =
            (0..5).map(|n| case_for(Facet::ConstantFold, &n.to_string())).collect();
        let mut records = Vec::new();
        for (at, case) in cases.iter().enumerate() {
            records.push(record_for(case, "gcc-16", Level::O2, 100, 100));
            // One case is four times the size and the rest are at parity. A mean would report
            // a sixty percent regression, which is not what a typical case looks like.
            let text = if at == 0 { 400 } else { 100 };
            records.push(record_for(case, "rucc", Level::O2, text, 100));
        }
        let manifest = Manifest::new(cases).unwrap();
        let run = run_of(records);
        let summary = summarise(&run, &manifest.digest(), manifest.cases.len());
        let score = summary.facets[0].score("rucc").unwrap();
        assert_eq!(score.size_ratio, Some(1.0));
        assert_eq!(score.compared, 5);
    }

    #[test]
    fn the_targets_say_what_they_wanted_and_what_they_got() {
        let case = case_for(Facet::Simplify, "one");
        let records = vec![
            record_for(&case, "gcc-16", Level::O2, 100, 100),
            record_for(&case, "rucc", Level::O2, 150, 100),
        ];
        let manifest = Manifest::new(vec![case]).unwrap();
        let summary = summarise(&run_of(records), &manifest.digest(), 1);

        let correctness = summary.targets.iter().find(|t| t.name == "correctness").unwrap();
        assert!(correctness.met);
        assert_eq!(correctness.actual, Some(0.0));

        let quality = summary.targets.iter().find(|t| t.name == "code-quality:rucc").unwrap();
        assert_eq!(quality.actual, Some(1.5));
        assert!(!quality.met, "fifty percent bigger is not within ten percent");
        assert_eq!(quality.threshold, Some(1.10));
        // The reference is not measured against itself.
        assert!(!summary.targets.iter().any(|t| t.name.ends_with("gcc-16")));
    }

    #[test]
    fn the_losses_are_findable_because_the_report_has_to_lead_with_them() {
        let good = case_for(Facet::Simplify, "good");
        let bad = case_for(Facet::LoopUnroll, "bad");
        let records = vec![
            record_for(&good, "gcc-16", Level::O2, 100, 100),
            record_for(&good, "rucc", Level::O2, 90, 100),
            record_for(&bad, "gcc-16", Level::O2, 100, 100),
            record_for(&bad, "rucc", Level::O2, 300, 100),
        ];
        let summary = summarise(&run_of(records), "abc", 2);
        let worst = summary.worst_facets("rucc", 1);
        assert_eq!(worst[0].0.facet, Facet::LoopUnroll);
        assert_eq!(worst[0].1, 3.0);
        let best = summary.best_facets("rucc", 1);
        assert_eq!(best[0].0.facet, Facet::Simplify);
        assert!((best[0].1 - 0.9).abs() < 1e-9);
    }

    #[test]
    fn the_reference_is_named_and_is_not_one_of_the_compilers_under_test() {
        let case = case_for(Facet::Baseline, "one");
        let records = vec![
            record_for(&case, "gcc-16", Level::O2, 100, 100),
            record_for(&case, "rucc", Level::O2, 100, 100),
        ];
        let summary = summarise(&run_of(records), "abc", 1);
        assert_eq!(summary.reference, "gcc-16");
        assert_eq!(summary.under_test(), ["rucc"]);
        assert!(summary.correct());
    }
}
