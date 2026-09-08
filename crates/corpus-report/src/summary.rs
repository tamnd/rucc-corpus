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

use corpus_model::{Facet, Level, Phase, RunRecord, Source, Verdict};
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
    /// Would not compile a valid program and said so, which is a gap and not a bug.
    pub unimplemented: usize,
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
            Verdict::Unimplemented => &mut self.unimplemented,
            Verdict::Accepted => &mut self.accepted,
            Verdict::Crashed => &mut self.crashed,
            Verdict::Skipped => &mut self.skipped,
        };
        *slot += 1;
    }

    /// How many were actually run.
    #[must_use]
    pub const fn ran(&self) -> usize {
        self.pass + self.wrong + self.rejected + self.unimplemented + self.accepted + self.crashed
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
    /// Median compiler memory against the reference, at the headline level.
    ///
    /// `None` on a platform that cannot measure memory at all, which is every platform that is
    /// not Linux, rather than nought. See `corpus_run::memory`.
    pub memory_ratio: Option<f64>,
    /// Median executable size on disk against the reference, at the headline level.
    ///
    /// Separate from `size_ratio`, which is the code alone. This one includes the runtime, the
    /// symbol table and whatever the linker padded with, so it is what a build costs on disk
    /// rather than what the optimizer decided.
    pub disk_ratio: Option<f64>,
    /// Median initialized data in the image against the reference, at the headline level.
    pub data_ratio: Option<f64>,
    /// How many cases had a measurable size ratio, which is how much the median is worth.
    pub compared: usize,
    /// How many cases had a measurable memory ratio, which is how much that median is worth.
    ///
    /// Its own count rather than sharing `compared`, because memory is the one number here
    /// that can be missing for a reason that has nothing to do with the compiler.
    pub memory_compared: usize,
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
    /// How much C those cases add up to.
    pub source: Source,
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

/// What `-Os` did to one compiler's own output.
///
/// Every other ratio in this file is one compiler against the reference at the same level. This
/// one is a compiler against itself at two levels, which is a different question and the only way
/// to ask it. `-Os` is a different cost function, not a cheaper `-O2`, so it selects different
/// rewrites, and the thing worth knowing is whether it selects any. A compiler whose `-Os` output
/// is byte for byte its `-O2` output has accepted the flag and ignored it, and the corpus ran
/// every program at `-Os` for a year without ever being able to say so.
#[derive(Debug, Clone)]
pub struct SizeModel {
    /// Which compiler.
    pub toolchain: String,
    /// Median of its own `-Os` code size over its own `-O2` code size.
    pub against_o2: Option<f64>,
    /// How many cases had a size at both levels.
    pub compared: usize,
    /// How many of those came out exactly the same size, which is the number that says whether
    /// the flag did anything at all.
    pub unmoved: usize,
    /// Where `-Os` moves the size most, largest saving first.
    pub by_facet: Vec<(Facet, f64, usize)>,
}

impl SizeModel {
    /// How many facets to name in either direction.
    const NAMED: usize = 6;

    /// The facets `-Os` saves the most on.
    #[must_use]
    pub fn best(&self) -> Vec<(Facet, f64, usize)> {
        self.by_facet.iter().take(Self::NAMED).copied().collect()
    }

    /// The facets `-Os` saves the least on, which is where it costs size for nothing when the
    /// number is above one.
    #[must_use]
    pub fn worst(&self) -> Vec<(Facet, f64, usize)> {
        let mut tail: Vec<(Facet, f64, usize)> =
            self.by_facet.iter().rev().take(Self::NAMED).copied().collect();
        tail.reverse();
        tail
    }

    /// Whether every case came out the same size.
    ///
    /// The raw fact and not the conclusion. Read `Summary::ignoring_size` for the conclusion,
    /// because this on its own says yes about a compiler that has done nothing wrong.
    #[must_use]
    pub const fn ignored(&self) -> bool {
        self.compared > 0 && self.unmoved == self.compared
    }

    /// Whether `-Os` moved anything at all here.
    #[must_use]
    pub const fn moved_something(&self) -> bool {
        self.unmoved < self.compared
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
    /// How much C the whole corpus is, counting each case once.
    ///
    /// The denominator for everything else on the page. A compile time, a code size and a peak
    /// memory figure are all answers to a question that starts with how much source there was,
    /// and until this existed the report asked the reader to supply that themselves.
    pub source: Source,
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
    /// What `-Os` did, per compiler, empty when the run did not build it.
    pub size_model: Vec<SizeModel>,
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

    /// What `-Os` did to one compiler's own output.
    #[must_use]
    pub fn size_model_of(&self, toolchain: &str) -> Option<&SizeModel> {
        self.size_model.iter().find(|model| model.toolchain == toolchain)
    }

    /// Whether a compiler is taking `-Os` and doing nothing with it.
    ///
    /// Not simply every case coming out the same size, which is what it looked like at first and
    /// is wrong. Whole facets have no size against speed tradeoff in them, and constant folding is
    /// one: `gcc-16` produces the same 112 bytes at both levels on every case in it, because there
    /// is nothing there to trade. On a run filtered down to that facet every compiler comes out
    /// unmoved and none of them has done anything. The reference is the control. When it moved
    /// cases in this run and the compiler under test moved none, the flag is being ignored, and
    /// when the reference did not move either there is no question to answer.
    #[must_use]
    pub fn ignoring_size(&self, toolchain: &str) -> bool {
        let Some(model) = self.size_model_of(toolchain) else {
            return false;
        };
        model.ignored()
            && self.size_model_of(&self.reference).is_some_and(SizeModel::moved_something)
    }

    /// Whether the run has anything to say about `-Os` at all.
    ///
    /// Per-commit CI builds two levels and nightly builds five, so both reports come through here
    /// and only one of them can answer the question. A section that prints itself with no numbers
    /// in it teaches people to skip that part of the report.
    #[must_use]
    pub fn measured_size_model(&self) -> bool {
        self.size_model.iter().any(|model| model.compared > 0)
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
    let size_model = size_models(run, &toolchains);
    let targets = check_targets(&totals, &facets, &size_model, &reference, &toolchains);

    Summary {
        corpus_digest: corpus_digest.to_owned(),
        cases,
        source: source_of(run.records.iter()),
        reference,
        toolchains,
        levels,
        totals,
        facets,
        size_model,
        targets,
    }
}

/// How much C a set of records is, counting each case once.
///
/// The dedup is the whole function. A record is one case built by one compiler at one level, so a
/// corpus of a thousand programs run two ways at five levels leaves ten thousand of them behind,
/// and a sum over records would report the corpus at ten times its size. The wrong number would
/// look perfectly plausible sitting in a table, which is what makes it worth a function of its own
/// and a test underneath it.
fn source_of<'a>(records: impl Iterator<Item = &'a RunRecord>) -> Source {
    let mut seen: BTreeMap<&str, Source> = BTreeMap::new();
    for record in records {
        seen.entry(record.case.as_str()).or_insert(record.source);
    }
    add_up(seen.values().copied())
}

/// Adds sizes together, saturating rather than wrapping.
///
/// Safe to use on facets, since a case belongs to exactly one of them, and never safe to use on
/// records. [`source_of`] is the one for records and it says why.
#[must_use]
pub fn add_up(sizes: impl Iterator<Item = Source>) -> Source {
    sizes.fold(Source::default(), Source::plus)
}

/// The level `-Os` is judged against.
///
/// The same level as the headline, and for the same reason: `-O2` is what everybody ships, so it
/// is the size a person already has in mind when they ask what `-Os` would save them.
const AGAINST: Level = Level::O2;

/// What `-Os` did to each compiler's own output.
///
/// This is the one measurement in the file that crosses levels, which `compare::comparable` will
/// not do and is right not to do. Comparing rucc at `-Os` against gcc at `-O2` would be two
/// differences at once and would tell nobody anything. Comparing a compiler at `-Os` against the
/// same compiler at `-O2` on the same program is one difference, and it is the flag.
fn size_models(run: &Run, toolchains: &[String]) -> Vec<SizeModel> {
    toolchains.iter().map(|id| size_model_of(run, id)).collect()
}

/// The `-Os` figures for one compiler.
fn size_model_of(run: &Run, toolchain: &str) -> SizeModel {
    let mut all = Vec::new();
    let mut unmoved = 0;
    let mut per_facet: BTreeMap<Facet, Vec<f64>> = BTreeMap::new();

    for record in &run.records {
        if record.toolchain != toolchain || record.level != Level::Os {
            continue;
        }
        let Some(against) = run.record(&record.case, toolchain, AGAINST) else {
            continue;
        };
        let Some(ratio) = compare::size_ratio(record, against) else {
            continue;
        };
        if record.compile.text_bytes == against.compile.text_bytes {
            unmoved += 1;
        }
        all.push(ratio);
        per_facet.entry(record.facet).or_default().push(ratio);
    }

    let mut by_facet: Vec<(Facet, f64, usize)> = per_facet
        .into_iter()
        .filter_map(|(facet, mut ratios)| {
            let count = ratios.len();
            Some((facet, median(&mut ratios)?, count))
        })
        .collect();
    by_facet.sort_by(|a, b| a.1.total_cmp(&b.1));

    SizeModel {
        toolchain: toolchain.to_owned(),
        compared: all.len(),
        unmoved,
        against_o2: median(&mut all),
        by_facet,
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
        let size = source_of(records.iter().copied());
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
            source: size,
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
    let mut memories = Vec::new();
    let mut disks = Vec::new();
    let mut datas = Vec::new();

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
        if let Some(value) = compare::memory_ratio(record, against) {
            memories.push(value);
        }
        if let Some(value) = compare::disk_ratio(record, against) {
            disks.push(value);
        }
        if let Some(value) = compare::data_ratio(record, against) {
            datas.push(value);
        }
    }

    FacetScore {
        toolchain: toolchain.to_owned(),
        tally,
        compared: sizes.len(),
        memory_compared: memories.len(),
        size_ratio: median(&mut sizes),
        speed_ratio: median(&mut speeds),
        compile_ratio: median(&mut compiles),
        memory_ratio: median(&mut memories),
        disk_ratio: median(&mut disks),
        data_ratio: median(&mut datas),
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
    size_model: &[SizeModel],
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

        // Only when the run built `-Os`. Per-commit CI builds `-O0` and `-O2`, and a target that
        // reports itself unmet because nobody asked the question is a target people learn to
        // ignore, which costs more than leaving it out costs.
        if let Some(model) = size_model.iter().find(|model| &model.toolchain == id) {
            if model.compared > 0 {
                // The reference is the control for whether the flag had anything to do here. See
                // `Summary::ignoring_size`.
                let control = size_model.iter().find(|model| model.toolchain == reference);
                let ignoring = model.ignored() && control.is_some_and(SizeModel::moved_something);
                targets.push(Target {
                    name: format!("size-model:{id}"),
                    wanted: format!(
                        "the code {id} produces at -Os is no larger than the code it produces at -O2, and where {reference} found something to trade away {id} found something too, since -Os is a different cost function and not a cheaper -O2"
                    ),
                    threshold: Some(1.0),
                    actual: model.against_o2,
                    met: !ignoring && model.against_o2.is_some_and(|value| value <= 1.0),
                });
            }
        }
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
            ..Compile::skipped()
        };
        record.execute = Execute {
            ok: true,
            status: 0,
            micros,
            repeats: 5,
            output: "1\n".to_owned(),
            ..Execute::skipped()
        };
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
    fn the_corpus_size_counts_each_case_once_however_many_records_it_left_behind() {
        let cases: Vec<Case> =
            (0..5).map(|n| case_for(Facet::ConstantFold, &n.to_string())).collect();
        let mut records = Vec::new();
        for case in &cases {
            for level in [Level::O0, Level::O2] {
                records.push(record_for(case, "gcc-16", level, 100, 100));
                records.push(record_for(case, "rucc", level, 100, 100));
            }
        }
        assert_eq!(records.len(), 20, "five cases, two compilers, two levels");
        let manifest = Manifest::new(cases).unwrap();
        let run = run_of(records);
        let summary = summarise(&run, &manifest.digest(), manifest.cases.len());
        // One line per case. A sum over records would say twenty, which is a number that would
        // sit in the table looking perfectly reasonable.
        assert_eq!(summary.source.lines, 5);
        assert_eq!(summary.facets[0].source.lines, 5);
        assert!(summary.source.bytes > 0);
    }

    #[test]
    fn a_run_whose_records_never_recorded_a_size_reports_none_rather_than_nought() {
        let case = case_for(Facet::Simplify, "one");
        let records: Vec<RunRecord> = [Level::O2]
            .iter()
            .map(|level| RunRecord {
                source: corpus_model::Source::default(),
                ..record_for(&case, "gcc-16", *level, 100, 100)
            })
            .collect();
        let manifest = Manifest::new(vec![case]).unwrap();
        let run = run_of(records);
        let summary = summarise(&run, &manifest.digest(), manifest.cases.len());
        assert!(!summary.source.measured());
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
    fn what_os_did_is_measured_against_the_same_compiler_and_not_against_the_reference() {
        let case = case_for(Facet::Simplify, "one");
        let records = vec![
            // The reference is twice the size of rucc at both levels, which would swamp the
            // answer if this were measured the way every other ratio in the file is.
            record_for(&case, "gcc-16", Level::O2, 200, 100),
            record_for(&case, "gcc-16", Level::Os, 160, 100),
            record_for(&case, "rucc", Level::O2, 100, 100),
            record_for(&case, "rucc", Level::Os, 80, 100),
        ];
        let summary = summarise(&run_of(records), "abc", 1);

        let rucc = summary.size_model_of("rucc").unwrap();
        assert_eq!(rucc.against_o2, Some(0.8), "80 over rucc's own 100, not over gcc's 200");
        assert_eq!(rucc.compared, 1);
        assert_eq!(rucc.unmoved, 0);
        assert!(!rucc.ignored());
        assert_eq!(rucc.by_facet, vec![(Facet::Simplify, 0.8, 1)]);
        // The reference gets one too, since it is the control a reader needs to know what a
        // working size cost model looks like.
        assert_eq!(summary.size_model_of("gcc-16").unwrap().against_o2, Some(0.8));

        let target = summary.targets.iter().find(|t| t.name == "size-model:rucc").unwrap();
        assert!(target.met);
        assert_eq!(target.actual, Some(0.8));
        assert!(!summary.targets.iter().any(|t| t.name == "size-model:gcc-16"));
    }

    #[test]
    fn a_compiler_that_accepts_os_and_then_ignores_it_does_not_get_to_pass_the_target() {
        let case = case_for(Facet::Simplify, "one");
        let records = vec![
            record_for(&case, "gcc-16", Level::O2, 200, 100),
            record_for(&case, "gcc-16", Level::Os, 160, 100),
            record_for(&case, "rucc", Level::O2, 100, 100),
            record_for(&case, "rucc", Level::Os, 100, 100),
        ];
        let summary = summarise(&run_of(records), "abc", 1);

        let rucc = summary.size_model_of("rucc").unwrap();
        assert_eq!(rucc.against_o2, Some(1.0));
        assert_eq!(rucc.unmoved, 1);
        assert!(rucc.ignored());
        assert!(
            summary.ignoring_size("rucc"),
            "the reference moved the same case and rucc did not, so the flag is being ignored"
        );

        let target = summary.targets.iter().find(|t| t.name == "size-model:rucc").unwrap();
        assert!(
            !target.met,
            "a ratio of one reads as no worse, and that is exactly what ignoring the flag looks like"
        );
    }

    #[test]
    fn a_run_where_os_had_nothing_to_trade_is_not_a_compiler_ignoring_the_flag() {
        // What `--facet constant-fold` looks like. A folded constant is the same bytes at both
        // levels for everybody, so nobody moved and nobody is at fault.
        let case = case_for(Facet::ConstantFold, "one");
        let records = vec![
            record_for(&case, "gcc-16", Level::O2, 100, 100),
            record_for(&case, "gcc-16", Level::Os, 100, 100),
            record_for(&case, "rucc", Level::O2, 100, 100),
            record_for(&case, "rucc", Level::Os, 100, 100),
        ];
        let summary = summarise(&run_of(records), "abc", 1);

        assert!(summary.size_model_of("rucc").unwrap().ignored(), "the raw fact is still yes");
        assert!(
            !summary.ignoring_size("rucc"),
            "the control did not move either, so there is no question to answer"
        );
        let target = summary.targets.iter().find(|t| t.name == "size-model:rucc").unwrap();
        assert!(target.met, "a facet with no tradeoff in it must not fail anybody");
    }

    #[test]
    fn a_run_that_did_not_build_os_says_nothing_about_it_rather_than_reporting_a_miss() {
        let case = case_for(Facet::Simplify, "one");
        let records = vec![
            record_for(&case, "gcc-16", Level::O2, 100, 100),
            record_for(&case, "rucc", Level::O2, 100, 100),
        ];
        let summary = summarise(&run_of(records), "abc", 1);

        assert!(!summary.measured_size_model());
        assert_eq!(summary.size_model_of("rucc").unwrap().compared, 0);
        assert!(
            !summary.targets.iter().any(|t| t.name.starts_with("size-model:")),
            "the per-commit job builds two levels, and a target nobody asked for is one people \
             learn to skip"
        );
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
