//! The page that says what each rucc pass did over the corpus.
//!
//! Every case rucc builds is compiled with `-frucc-trace`, and the trace says for each optimizer
//! pass that ran what it did and how many times. [`page`] adds that up per compiler and level, and
//! puts the passes that never fired first, since those are what section 42.2 of rucc's
//! measurement spec and tamnd/rucc#2967 ask to find.

use corpus_run::Run;
use corpus_run::firing::{Build, Total, builds};
use std::fmt::Write as _;

/// How many facets a row names before it stops.
const FACETS: usize = 3;

/// The page, as `reports/firing.md`.
#[must_use]
pub fn page(run: &Run) -> String {
    let mut out = String::new();
    out.push_str("# What each rucc pass did\n\n");
    out.push_str(
        "Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.\n\n",
    );
    out.push_str(
        "A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.\n\n",
    );
    let builds = builds(&run.records);
    if builds.is_empty() {
        out.push_str(
            "No build in this run wrote a trace, so there is nothing to add up. A rucc from before tamnd/rucc#2967 writes a trace without what each pass did in it, and a compiler that is not rucc is never asked for one.\n",
        );
        return out;
    }
    for build in &builds {
        section(&mut out, build);
    }
    out
}

/// One compiler at one level.
fn section(out: &mut String, build: &Build) {
    let quiet: Vec<&Total> = build.quiet().collect();
    let _ = writeln!(out, "## `{}` at {}\n", build.toolchain, build.level.flag());
    let _ = writeln!(
        out,
        "{} passes ran on {} cases, and {} of them never fired.\n",
        build.passes.len(),
        build.cases,
        quiet.len()
    );
    if !quiet.is_empty() {
        out.push_str("### Never fired\n\n");
        out.push_str("| pass | ran on | missed | what it said |\n|---|--:|--:|---|\n");
        for total in &quiet {
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} |",
                total.pass,
                total.ran,
                total.missed,
                said(total)
            );
        }
        out.push('\n');
    }
    out.push_str("### Every pass, in the order it runs\n\n");
    out.push_str(
        "| pass | fired on | rewrites | missed | facets it fired on most | what it did most |\n|---|--:|--:|--:|---|---|\n",
    );
    for total in &build.passes {
        let _ = writeln!(
            out,
            "| `{}` | {} of {} | {} | {} | {} | {} |",
            total.pass,
            total.fired,
            total.ran,
            total.rewrites,
            total.missed,
            facets(total),
            most(total)
        );
    }
    out.push('\n');
}

/// The facets a pass fired on most, with how many cases of each.
fn facets(total: &Total) -> String {
    let mut ranked: Vec<_> = total.facets.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.name().cmp(b.0.name())));
    let named: Vec<String> = ranked
        .iter()
        .take(FACETS)
        .map(|(facet, cases)| format!("{} {cases}", facet.name()))
        .collect();
    named.join(", ")
}

/// The rewrite a pass made most, with how many times.
fn most(total: &Total) -> String {
    total
        .events
        .iter()
        .filter_map(|(what, count)| what.strip_prefix("optimized: ").map(|what| (what, count)))
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
        .map(|(what, count)| format!("{} ({count})", what.replace('|', "\\|")))
        .unwrap_or_default()
}

/// What a quiet pass said instead, which is the thing to read first when deciding why.
fn said(total: &Total) -> String {
    let lines: Vec<String> = total
        .events
        .iter()
        .take(2)
        .map(|(what, count)| format!("{} ({count})", what.replace('|', "\\|")))
        .collect();
    lines.join("; ")
}

#[cfg(test)]
mod tests {
    use super::page;
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Fired, Level, RunRecord};
    use corpus_run::Run;
    use std::collections::BTreeMap;

    fn record(facet: Facet, fired: Vec<Fired>) -> RunRecord {
        let case = Case::new(
            facet,
            Axes::of([("shape", facet.name())]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Output(String::new()),
        );
        RunRecord { fired, ..RunRecord::skipped(&case, "rucc", Level::O2) }
    }

    fn fired(pass: &str, events: &[(&str, u64)]) -> Fired {
        Fired {
            pass: pass.to_owned(),
            events: events.iter().map(|(what, n)| ((*what).to_owned(), *n)).collect(),
        }
    }

    fn run(records: Vec<RunRecord>) -> Run {
        Run { toolchains: Vec::new(), records, verdicts: BTreeMap::new(), findings: Vec::new() }
    }

    #[test]
    fn the_passes_that_never_fired_come_first_and_every_pass_is_in_the_table() {
        let text = page(&run(vec![
            record(
                Facet::Simplify,
                vec![
                    fired("fold", &[("optimized: folded", 2)]),
                    fired("licm", &[("missed: not invariant", 1)]),
                ],
            ),
            record(
                Facet::Strength,
                vec![fired("fold", &[("optimized: folded", 1)]), fired("licm", &[])],
            ),
        ]));
        assert!(text.contains("## `rucc` at -O2"), "{text}");
        assert!(text.contains("2 passes ran on 2 cases, and 1 of them never fired."), "{text}");
        let never = text.find("### Never fired").unwrap();
        let every = text.find("### Every pass").unwrap();
        assert!(never < every);
        assert!(
            text[never..every].contains("| `licm` | 2 | 1 | missed: not invariant (1) |"),
            "{text}"
        );
        assert!(
            text.contains("| `fold` | 2 of 2 | 3 | 0 | simplify 1, strength 1 | folded (3) |"),
            "{text}"
        );
    }

    #[test]
    fn a_run_with_no_trace_says_so() {
        assert!(page(&run(Vec::new())).contains("No build in this run wrote a trace"));
    }
}
