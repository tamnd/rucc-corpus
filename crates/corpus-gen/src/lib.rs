//! Generates the corpus.
//!
//! The generator is systematic and not random. It walks a named axis space and emits one
//! program per point, so the corpus is a function of the generator and nothing else. Two
//! people who run it get the same programs, a program can be found again from the axis point
//! that produced it, and a gap in coverage is a missing axis rather than bad luck with a seed.
//!
//! # Why not a random program generator
//!
//! Csmith and YARPGen are excellent and rucc should be run against both. They answer a
//! different question. A random generator asks whether the compiler is wrong anywhere, and it
//! is very good at that. It cannot tell you whether loop unswitching pays for itself, because
//! it does not know which of its programs were about loop unswitching. This generator tags
//! every program with the one transformation it was written for, which is what makes a
//! per-optimization correctness and performance claim possible at all.
//!
//! # The axes
//!
//! Not every facet uses every axis, and a facet that used all of them would emit a hundred
//! thousand near duplicates. Each facet declares the subspace that is meaningful for it.
//!
//! - `type`, one of the eight integer types
//! - `op`, one of the fourteen operations
//! - `shape`, the control flow the value is computed in
//! - `storage`, where the value lives
//! - `dialect`, which C the program is written in
//!
//! # The guarantee
//!
//! Every program is free of undefined behaviour by construction, prints a value the generator
//! computed itself, and prints nothing that depends on the machine it runs on. Where the
//! generator cannot be sure of an answer it drops the case instead of guessing.

#![forbid(unsafe_code)]

pub mod emit;
pub mod lang;

mod facets;

use corpus_model::{Axes, Case, Dialect, Expect, Facet, Manifest};
use emit::Program;
use std::collections::BTreeMap;

/// What to generate.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Which facets to emit. Empty means all of them.
    pub facets: Vec<Facet>,
    /// Which dialects to emit. Empty means all of them.
    pub dialects: Vec<Dialect>,
    /// At most this many cases per facet, for a quick run.
    ///
    /// The cap is applied in generation order, and generation order is deterministic, so a
    /// capped run is a prefix of a full one rather than a sample of it. That matters because
    /// a pull request that runs a capped corpus and a nightly job that runs the full one
    /// should never disagree about a case they both ran.
    pub limit_per_facet: Option<usize>,
}

impl Options {
    /// Everything.
    #[must_use]
    pub fn all() -> Self {
        Self::default()
    }

    /// Only these facets.
    #[must_use]
    pub fn only(mut self, facets: &[Facet]) -> Self {
        self.facets = facets.to_vec();
        self
    }

    /// At most this many cases per facet.
    #[must_use]
    pub const fn capped(mut self, limit: usize) -> Self {
        self.limit_per_facet = Some(limit);
        self
    }

    /// Whether a facet is wanted.
    #[must_use]
    pub fn wants_facet(&self, facet: Facet) -> bool {
        self.facets.is_empty() || self.facets.contains(&facet)
    }

    /// Whether a dialect is wanted.
    #[must_use]
    pub fn wants_dialect(&self, dialect: Dialect) -> bool {
        self.dialects.is_empty() || self.dialects.contains(&dialect)
    }
}

/// Where a facet generator puts its cases.
///
/// The filtering and the per-facet cap live here rather than in each generator, so that a
/// generator is just a loop that emits programs and cannot get the bookkeeping wrong.
#[derive(Debug)]
pub struct Sink<'a> {
    opts: &'a Options,
    cases: Vec<Case>,
    counts: BTreeMap<Facet, usize>,
    dropped: usize,
}

impl<'a> Sink<'a> {
    /// A sink that obeys these options.
    #[must_use]
    pub fn new(opts: &'a Options) -> Self {
        Self { opts, cases: Vec::new(), counts: BTreeMap::new(), dropped: 0 }
    }

    /// Whether it is worth generating for this facet at all.
    ///
    /// A generator checks this once at the top rather than building programs that the sink
    /// will throw away, which is the difference between a capped run being fast and a capped
    /// run doing all the work and then discarding it.
    #[must_use]
    pub fn wants(&self, facet: Facet) -> bool {
        if !self.opts.wants_facet(facet) {
            return false;
        }
        match self.opts.limit_per_facet {
            Some(limit) => self.counts.get(&facet).copied().unwrap_or(0) < limit,
            None => true,
        }
    }

    /// Adds a program that is expected to compile, run and print what it printed.
    ///
    /// A program that prints nothing is dropped. Such a program has no oracle, so running it
    /// proves only that the compiler did not crash, and the corpus has a facet for that.
    pub fn push(&mut self, facet: Facet, axes: Axes, dialect: Dialect, program: Program) {
        if !self.wants(facet) || !self.opts.wants_dialect(dialect) {
            return;
        }
        if program.is_empty() {
            self.dropped += 1;
            return;
        }
        let (source, expected) = program.finish();
        let case = Case::new(facet, axes, dialect, source, Expect::Output(expected));
        *self.counts.entry(facet).or_insert(0) += 1;
        self.cases.push(case);
    }

    /// The same, with tags attached.
    ///
    /// Tags are how a case says something about itself that no axis captures. The one that
    /// earns its keep is `gnu`, for a case that uses a GCC extension: those belong in the
    /// corpus, because compatibility with GCC is the goal, but a run against a compiler that
    /// has not got to extensions yet should be able to leave them out and still be a real run.
    pub fn push_tagged(
        &mut self,
        facet: Facet,
        axes: Axes,
        dialect: Dialect,
        program: Program,
        tags: &[&str],
    ) {
        if !self.wants(facet) || !self.opts.wants_dialect(dialect) {
            return;
        }
        if program.is_empty() {
            self.dropped += 1;
            return;
        }
        let (source, expected) = program.finish();
        let case = Case::new(facet, axes, dialect, source, Expect::Output(expected)).tagged(tags);
        *self.counts.entry(facet).or_insert(0) += 1;
        self.cases.push(case);
    }

    /// Adds a program that must not compile, and the text the diagnostic has to mention.
    pub fn push_rejected(
        &mut self,
        facet: Facet,
        axes: Axes,
        dialect: Dialect,
        source: impl Into<String>,
        mentions: impl Into<String>,
    ) {
        if !self.wants(facet) || !self.opts.wants_dialect(dialect) {
            return;
        }
        let case = Case::new(facet, axes, dialect, source, Expect::Rejected(mentions.into()));
        *self.counts.entry(facet).or_insert(0) += 1;
        self.cases.push(case);
    }

    /// Adds a case that is already built, so a generator can attach tags.
    pub fn push_case(&mut self, case: Case) {
        if !self.wants(case.facet) || !self.opts.wants_dialect(case.dialect) {
            return;
        }
        *self.counts.entry(case.facet).or_insert(0) += 1;
        self.cases.push(case);
    }

    /// How many programs were built and then thrown away for having no oracle.
    ///
    /// A generator change that sends this number up has usually broken a template rather than
    /// tightened a guard, so the count is reported and the tests watch it.
    #[must_use]
    pub const fn dropped(&self) -> usize {
        self.dropped
    }

    /// Everything collected.
    #[must_use]
    pub fn into_cases(self) -> Vec<Case> {
        self.cases
    }
}

/// Generates the corpus.
///
/// # Errors
///
/// When two generated cases end up with the same id, which means two templates produced the
/// same source at the same axis point and one of them would be invisible in the report.
pub fn generate(opts: &Options) -> Result<Manifest, String> {
    let mut sink = Sink::new(opts);
    facets::special::baseline(&mut sink);
    facets::special::control_flow(&mut sink);
    facets::local::generate(&mut sink);
    facets::global::generate(&mut sink);
    facets::loops::generate(&mut sink);
    facets::interproc::generate(&mut sink);
    facets::backend::generate(&mut sink);
    facets::special::barrier(&mut sink);
    facets::special::frontend(&mut sink);
    Manifest::new(sink.into_cases())
}

#[cfg(test)]
mod tests {
    use super::{Options, generate};
    use corpus_model::{Expect, Facet, Phase};
    use std::collections::BTreeSet;

    #[test]
    fn the_generator_produces_the_same_corpus_twice() {
        let one = generate(&Options::all()).unwrap();
        let two = generate(&Options::all()).unwrap();
        assert_eq!(one.digest(), two.digest());
        assert_eq!(one.cases.len(), two.cases.len());
    }

    #[test]
    fn every_facet_produces_at_least_one_case() {
        let corpus = generate(&Options::all()).unwrap();
        let covered: BTreeSet<Facet> = corpus.cases.iter().map(|c| c.facet).collect();
        let missing: Vec<&Facet> = Facet::ALL.iter().filter(|f| !covered.contains(f)).collect();
        assert!(missing.is_empty(), "no cases for {missing:?}");
    }

    #[test]
    fn every_phase_of_the_plan_has_evidence() {
        let corpus = generate(&Options::all()).unwrap();
        let covered: BTreeSet<Phase> = corpus.cases.iter().map(|c| c.phase()).collect();
        assert_eq!(covered.len(), Phase::ALL.len());
    }

    #[test]
    fn every_case_that_should_run_prints_something_so_it_has_an_oracle() {
        let corpus = generate(&Options::all()).unwrap();
        for case in &corpus.cases {
            if let Expect::Output(text) = &case.expect {
                assert!(!text.is_empty(), "{} prints nothing", case.id);
                assert!(text.ends_with('\n'), "{} output is not newline ended", case.id);
            }
        }
    }

    #[test]
    fn every_case_is_a_whole_translation_unit_with_a_main() {
        let corpus = generate(&Options::all()).unwrap();
        for case in &corpus.cases {
            assert!(case.source.contains("main"), "{} has no main", case.id);
            assert!(case.source.ends_with('\n'), "{} has no final newline", case.id);
        }
    }

    #[test]
    fn no_case_prints_anything_that_depends_on_the_machine_it_runs_on() {
        let corpus = generate(&Options::all()).unwrap();
        for case in &corpus.cases {
            for banned in ["%p", "sizeof", "__FILE__", "__LINE__", "__TIME__", "__DATE__"] {
                assert!(
                    !case.source.contains(banned),
                    "{} uses {banned}, which makes its output depend on the build",
                    case.id
                );
            }
        }
    }

    #[test]
    fn a_cap_takes_a_prefix_so_a_short_run_never_disagrees_with_a_long_one() {
        let full = generate(&Options::all()).unwrap();
        let capped = generate(&Options::all().capped(3)).unwrap();
        let full_ids: BTreeSet<&str> = full.cases.iter().map(|c| c.id.as_str()).collect();
        for case in &capped.cases {
            assert!(full_ids.contains(case.id.as_str()), "{} is not in the full run", case.id);
        }
        for (facet, count) in capped.by_facet() {
            assert!(count <= 3, "{facet} produced {count} cases under a cap of three");
        }
    }

    #[test]
    fn asking_for_one_facet_gives_only_that_facet() {
        let corpus = generate(&Options::all().only(&[Facet::LoopUnroll])).unwrap();
        assert!(!corpus.cases.is_empty());
        for case in &corpus.cases {
            assert_eq!(case.facet, Facet::LoopUnroll);
        }
    }

    #[test]
    fn the_corpus_is_big_enough_to_be_worth_running() {
        let corpus = generate(&Options::all()).unwrap();
        assert!(
            corpus.cases.len() > 500,
            "only {} cases, which is too thin to make a claim from",
            corpus.cases.len()
        );
    }
}
