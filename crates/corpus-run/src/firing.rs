//! What rucc says each of its passes did, from `-frucc-trace`.
//!
//! rucc writes one line of JSON per file it compiles to the file `-frucc-trace` names, and the
//! line has a `fired` object in it: every optimizer pass that ran, in the order it first ran,
//! with each thing it said as `kind: what` and how many times. A pass that ran and said nothing
//! is there as an empty object. That last part is what makes the count useful. A corpus written
//! for the transformations a compiler implements should make every pass fire somewhere, and a
//! pass that never does is either dead code or a bug, which section 42.2 of rucc's
//! `spec/optimizer/42-measurement.md` and tamnd/rucc#2967 ask to find.
//!
//! A case with modules in it is more than one compile, each appending its own line, so the lines
//! are added up into one record for the case.

use corpus_model::{Facet, Fired, Level, RunRecord, json};
use std::collections::BTreeMap;

/// Where rucc is asked to write its trace, inside the case's directory.
pub const TRACE: &str = "case.trace";

/// The flag that asks for it.
#[must_use]
pub fn flag(path: &str) -> String {
    format!("-frucc-trace={path}")
}

/// Every pass the trace names, with what it said added up over every line.
///
/// A line that is not JSON, or has no `fired` in it, adds nothing. A rucc from before the field
/// existed writes the second kind, and its record then says nothing about passes rather than
/// that every pass was quiet.
#[must_use]
pub fn parse(text: &str) -> Vec<Fired> {
    let mut passes: Vec<Fired> = Vec::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let Ok(value) = json::parse(line) else { continue };
        let Some(fired) = value.get("fired").and_then(corpus_model::Json::as_object) else {
            continue;
        };
        for (pass, events) in fired {
            let index = match passes.iter().position(|one| one.pass == *pass) {
                Some(index) => index,
                None => {
                    passes.push(Fired { pass: pass.clone(), events: Vec::new() });
                    passes.len() - 1
                }
            };
            for (what, count) in events.as_object().unwrap_or_default() {
                passes[index].add(what, whole(count));
            }
        }
    }
    passes
}

/// Whether a case's own flags build it at another level than the one the run asked for.
///
/// `constant-p-after-inline`, `objtool-shapes` and `frame-size` pass `-O2` themselves, since
/// what they check only holds once the optimizer has run, and the last `-O` on the command line is
/// the one the compiler takes. Their trace at `-O0` is a trace of the `-O2` pipeline, and counted
/// under `-O0` it would say that level runs passes it does not. Such a case keeps no trace.
#[must_use]
pub fn elsewhere(flags: &[String], level: Level) -> bool {
    flags.iter().any(|flag| flag.starts_with("-O") && flag != level.flag())
}

/// What one pass did over every case one compiler built at one level.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Total {
    /// The pass, by the name its `-f` flag spells.
    pub pass: String,
    /// How many cases it ran on.
    pub ran: usize,
    /// How many of those it rewrote something in.
    pub fired: usize,
    /// How many rewrites it made over all of them.
    pub rewrites: u64,
    /// How many times it said it missed something.
    pub missed: u64,
    /// How many cases of each facet it fired on.
    pub facets: BTreeMap<Facet, usize>,
    /// Everything it said, added up, in the order it first said it.
    pub events: Vec<(String, u64)>,
}

/// Every pass one compiler ran at one level, over the cases it built there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Build {
    /// The compiler.
    pub toolchain: String,
    /// The level.
    pub level: Level,
    /// How many cases have a trace, which is every case the compiler got as far as optimizing.
    pub cases: usize,
    /// Each pass, in the order the passes ran.
    pub passes: Vec<Total>,
}

impl Build {
    /// The passes that ran and never rewrote anything.
    pub fn quiet(&self) -> impl Iterator<Item = &Total> {
        self.passes.iter().filter(|total| total.fired == 0)
    }
}

/// What every pass did, one [`Build`] per compiler and level that wrote a trace, compilers in the
/// order their records first appear and levels cheapest first.
///
/// A pass is put in the order the first case to name it ran it, and one a later case names that
/// the earlier ones did not goes after it. Every case at one level runs the same pipeline, so the
/// order is the pipeline's.
#[must_use]
pub fn builds(records: &[RunRecord]) -> Vec<Build> {
    let mut builds: Vec<Build> = Vec::new();
    for record in records.iter().filter(|record| !record.fired.is_empty()) {
        let index = match builds
            .iter()
            .position(|one| one.toolchain == record.toolchain && one.level == record.level)
        {
            Some(index) => index,
            None => {
                builds.push(Build {
                    toolchain: record.toolchain.clone(),
                    level: record.level,
                    cases: 0,
                    passes: Vec::new(),
                });
                builds.len() - 1
            }
        };
        let build = &mut builds[index];
        build.cases += 1;
        for one in &record.fired {
            let at = match build.passes.iter().position(|total| total.pass == one.pass) {
                Some(at) => at,
                None => {
                    build.passes.push(Total { pass: one.pass.clone(), ..Total::default() });
                    build.passes.len() - 1
                }
            };
            let total = &mut build.passes[at];
            total.ran += 1;
            let rewrites = one.rewrites();
            if rewrites > 0 {
                total.fired += 1;
                *total.facets.entry(record.facet).or_default() += 1;
            }
            total.rewrites += rewrites;
            total.missed += one.missed();
            for (what, count) in &one.events {
                match total.events.iter_mut().find(|(it, _)| it == what) {
                    Some((_, sum)) => *sum += count,
                    None => total.events.push((what.clone(), *count)),
                }
            }
        }
    }
    let order = |id: &str| builds.iter().position(|one| one.toolchain == id).unwrap_or(usize::MAX);
    let mut keyed: Vec<(usize, Build)> =
        builds.iter().map(|one| (order(&one.toolchain), one.clone())).collect();
    keyed.sort_by_key(|(at, one)| (*at, one.level));
    keyed.into_iter().map(|(_, one)| one).collect()
}

/// A count as a whole number, nought for anything that is not a positive number.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn whole(value: &corpus_model::Json) -> u64 {
    value.as_f64().filter(|count| *count > 0.0).map_or(0, |count| count as u64)
}

#[cfg(test)]
mod tests {
    use super::{builds, elsewhere, flag, parse};
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Fired, Level, RunRecord};

    fn record(
        facet: Facet,
        toolchain: &str,
        level: Level,
        fired: &[(&str, &[(&str, u64)])],
    ) -> RunRecord {
        let case = Case::new(
            facet,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        let fired = fired
            .iter()
            .map(|(pass, events)| Fired {
                pass: (*pass).to_owned(),
                events: events.iter().map(|(what, n)| ((*what).to_owned(), *n)).collect(),
            })
            .collect();
        RunRecord { fired, ..RunRecord::skipped(&case, toolchain, level) }
    }

    #[test]
    fn every_pass_is_kept_in_the_order_it_ran_and_a_quiet_one_is_kept_too() {
        let line = r#"{"rucc":"0.23.0","passes":{"fold":0.1,"licm":0.2},"fired":{"fold":{"optimized: folded":2,"missed: kept":1},"licm":{}}}"#;
        let fired = parse(line);
        assert_eq!(fired.len(), 2);
        assert_eq!(fired[0].pass, "fold");
        assert_eq!(
            fired[0].events,
            [("optimized: folded".to_owned(), 2), ("missed: kept".to_owned(), 1)]
        );
        assert_eq!(fired[1].pass, "licm");
        assert!(fired[1].events.is_empty());
    }

    #[test]
    fn the_lines_of_a_case_with_modules_are_added_up() {
        let text = "{\"fired\":{\"fold\":{\"optimized: folded\":2}}}\n\
                    {\"fired\":{\"dce\":{},\"fold\":{\"optimized: folded\":3}}}\n";
        let fired = parse(text);
        assert_eq!(fired.len(), 2);
        assert_eq!(fired[0].events, [("optimized: folded".to_owned(), 5)]);
        assert_eq!(fired[1].pass, "dce");
    }

    #[test]
    fn a_case_that_names_its_own_level_is_only_traced_at_that_level() {
        let pinned = ["-mno-red-zone".to_owned(), "-O2".to_owned()];
        assert!(elsewhere(&pinned, Level::O0));
        assert!(elsewhere(&pinned, Level::Os));
        assert!(!elsewhere(&pinned, Level::O2));
        assert!(!elsewhere(&["-mno-red-zone".to_owned()], Level::O0));
    }

    #[test]
    fn a_trace_from_before_the_field_says_nothing_about_passes() {
        assert!(parse("{\"rucc\":\"0.22.0\",\"passes\":{\"fold\":0.1}}\nnot json\n").is_empty());
        assert_eq!(flag("case.trace"), "-frucc-trace=case.trace");
    }

    #[test]
    fn a_build_counts_the_cases_each_pass_ran_on_and_fired_on() {
        let records = vec![
            record(
                Facet::Simplify,
                "rucc",
                Level::O2,
                &[("fold", &[("optimized: folded", 2)]), ("licm", &[])],
            ),
            record(
                Facet::Strength,
                "rucc",
                Level::O2,
                &[("fold", &[("missed: kept", 1)]), ("licm", &[])],
            ),
            record(Facet::Simplify, "gcc-16", Level::O2, &[]),
            record(Facet::Simplify, "rucc", Level::O0, &[("inline", &[])]),
        ];
        let builds = builds(&records);
        assert_eq!(builds.len(), 2);
        assert_eq!((builds[0].level, builds[1].level), (Level::O0, Level::O2));
        let o2 = &builds[1];
        assert_eq!(o2.cases, 2);
        let fold = &o2.passes[0];
        assert_eq!(
            (fold.pass.as_str(), fold.ran, fold.fired, fold.rewrites, fold.missed),
            ("fold", 2, 1, 2, 1)
        );
        assert_eq!(fold.facets.get(&Facet::Simplify), Some(&1));
        let quiet: Vec<&str> = o2.quiet().map(|total| total.pass.as_str()).collect();
        assert_eq!(quiet, ["licm"]);
    }
}
