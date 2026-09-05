//! The report a person reads.
//!
//! It opens with the answer, because the first question is always whether anything broke and
//! the second is by how much. It shows the losses before the wins, which is spec 16 rule 3
//! and is also the only way a report is any use for deciding what to do next. It ends with the
//! command that reproduces it, because a number nobody can regenerate is a number nobody has
//! to believe.
//!
//! The whole thing is markdown with no pictures. It is read in a pull request, in a terminal
//! and on a phone, and a chart that only renders in one of those is worse than a table that
//! renders in all three.

use crate::summary::{FacetSummary, Summary, Tally};
use corpus_model::{Finding, Phase};
use corpus_run::Run;

/// How many facets are listed in each direction in the code quality section.
const RANKED: usize = 8;

/// How many findings are written out in full before the rest are counted.
const LISTED: usize = 25;

/// Writes the whole report.
#[must_use]
pub fn report_md(run: &Run, summary: &Summary) -> String {
    let mut out = String::new();
    heading(&mut out, summary, run);
    targets(&mut out, summary);
    totals(&mut out, summary);
    findings(&mut out, run);
    code_quality(&mut out, summary);
    by_phase(&mut out, summary);
    reference_opinion(&mut out, summary);
    reproducing(&mut out, summary);
    out
}

/// The answer, before anything else.
fn heading(out: &mut String, summary: &Summary, run: &Run) {
    out.push_str("# rucc corpus report\n\n");
    let verdict = if summary.correct() {
        "Every case in the corpus produced the answer the generator computed, on every compiler, at every level.".to_owned()
    } else {
        let failures: usize = summary.totals.values().map(Tally::failures).sum();
        format!(
            "{failures} case results did not come out as expected. They are listed below, worst first."
        )
    };
    out.push_str(&verdict);
    out.push_str("\n\n");
    out.push_str(&format!(
        "The corpus holds {} programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `{}`.\n\n",
        summary.cases,
        short(&summary.corpus_digest)
    ));

    out.push_str("| compiler | version | role |\n|---|---|---|\n");
    for toolchain in &run.toolchains {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            toolchain.id,
            toolchain.version,
            if toolchain.reference { "reference" } else { "under test" }
        ));
    }
    out.push('\n');
}

/// What was claimed, and whether this run supports it.
fn targets(out: &mut String, summary: &Summary) {
    out.push_str("## Did it meet the targets\n\n");
    out.push_str("| target | wanted | got | met |\n|---|---|---|---|\n");
    for target in &summary.targets {
        let wanted = match target.threshold {
            Some(value) if target.name == "correctness" => format!("{value:.0} failures"),
            Some(value) => as_percent_limit(value).to_string(),
            None => "not set".to_owned(),
        };
        let got = match target.actual {
            Some(value) if target.name == "correctness" => format!("{value:.0} failures"),
            Some(value) => as_change(value),
            None => "not measured".to_owned(),
        };
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            target.name,
            wanted,
            got,
            if target.met { "yes" } else { "no" }
        ));
    }
    out.push('\n');
    for target in &summary.targets {
        out.push_str(&format!("- `{}`: {}.\n", target.name, target.wanted));
    }
    out.push('\n');
}

/// How the cases came out, per compiler.
fn totals(out: &mut String, summary: &Summary) {
    out.push_str("## What happened\n\n");
    out.push_str("| compiler | ran | passed | wrong answer | wrongly rejected | wrongly accepted | crashed | skipped |\n|---|---|---|---|---|---|---|---|\n");
    for (id, tally) in &summary.totals {
        out.push_str(&format!(
            "| `{id}` | {} | {} | {} | {} | {} | {} | {} |\n",
            tally.ran(),
            tally.pass,
            tally.wrong,
            tally.rejected,
            tally.accepted,
            tally.crashed,
            tally.skipped
        ));
    }
    out.push('\n');
}

/// Everything that went wrong, in enough detail to act on.
fn findings(out: &mut String, run: &Run) {
    if run.findings.is_empty() {
        return;
    }
    out.push_str("## What went wrong\n\n");
    for finding in run.findings.iter().take(LISTED) {
        one_finding(out, finding);
    }
    if run.findings.len() > LISTED {
        out.push_str(&format!(
            "And {} more, which are all in `findings.sarif` and in `report.json`.\n\n",
            run.findings.len() - LISTED
        ));
    }
}

/// One failure, written the way somebody would write it in a bug report.
fn one_finding(out: &mut String, finding: &Finding) {
    out.push_str(&format!("### `{}`\n\n", finding.case));
    out.push_str(&format!(
        "{}. This is a case about {}, built at `{}`.\n\n",
        finding.summary.trim_end_matches('.'),
        finding.facet.describe(),
        finding.level.flag()
    ));
    if !finding.expected.is_empty() || !finding.actual.is_empty() {
        out.push_str("Expected:\n\n```\n");
        out.push_str(&clip(&finding.expected));
        out.push_str("\n```\n\nGot:\n\n```\n");
        out.push_str(&clip(&finding.actual));
        out.push_str("\n```\n\n");
    }
    out.push_str(&format!(
        "The program is `programs/{}` and the working directory of the failing build was kept under the run directory.\n\n",
        corpus_model::program_path(finding.facet, &finding.case)
    ));
}

/// The size comparison, losses first.
fn code_quality(out: &mut String, summary: &Summary) {
    let under_test = summary.under_test();
    if under_test.is_empty() {
        return;
    }
    out.push_str("## How big the code is\n\n");
    out.push_str(&format!(
        "Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `{}` at `-O2`, and every number is a median over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the headline.\n\n",
        summary.reference
    ));

    for id in under_test {
        out.push_str(&format!("### `{id}`\n\n"));

        let worst = summary.worst_facets(id, RANKED);
        if worst.is_empty() {
            out.push_str("Nothing could be compared, which means no case built on both compilers at `-O2`.\n\n");
            continue;
        }
        out.push_str("Furthest behind:\n\n");
        ratio_table(out, id, &worst);

        let best = summary.best_facets(id, RANKED);
        out.push_str("\nFurthest ahead:\n\n");
        ratio_table(out, id, &best);
        out.push('\n');
    }
}

/// A table of facets and their ratios.
fn ratio_table(out: &mut String, toolchain: &str, ranked: &[(&FacetSummary, f64)]) {
    out.push_str("| facet | phase | cases | code size | run time | compile time |\n|---|---|---|---|---|---|\n");
    for (facet, size) in ranked {
        let score = facet.score(toolchain);
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            facet.facet.name(),
            facet.phase.name(),
            facet.cases,
            as_change(*size),
            score.and_then(|s| s.speed_ratio).map_or_else(|| "not measured".to_owned(), as_change),
            score.and_then(|s| s.compile_ratio).map_or_else(|| "not measured".to_owned(), as_change),
        ));
    }
}

/// Where the work is, by phase of the plan.
fn by_phase(out: &mut String, summary: &Summary) {
    out.push_str("## By phase of the plan\n\n");
    out.push_str(
        "The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.\n\n",
    );
    out.push_str("| phase | facets | cases |");
    for id in summary.under_test() {
        out.push_str(&format!(" `{id}` passed | `{id}` code size |"));
    }
    out.push_str("\n|---|---|---|");
    for _ in summary.under_test() {
        out.push_str("---|---|");
    }
    out.push('\n');

    for phase in Phase::ALL {
        let in_phase: Vec<&FacetSummary> =
            summary.facets.iter().filter(|facet| facet.phase == *phase).collect();
        if in_phase.is_empty() {
            continue;
        }
        let cases: usize = in_phase.iter().map(|facet| facet.cases).sum();
        out.push_str(&format!("| {} | {} | {cases} |", phase.name(), in_phase.len()));
        for id in summary.under_test() {
            let mut tally = Tally::default();
            let mut sizes = Vec::new();
            for facet in &in_phase {
                if let Some(score) = facet.score(id) {
                    tally.pass += score.tally.pass;
                    tally.wrong += score.tally.wrong;
                    tally.rejected += score.tally.rejected;
                    tally.accepted += score.tally.accepted;
                    tally.crashed += score.tally.crashed;
                    tally.skipped += score.tally.skipped;
                    if let Some(ratio) = score.size_ratio {
                        sizes.push(ratio);
                    }
                }
            }
            let size = crate::summary::median(&mut sizes);
            out.push_str(&format!(
                " {} of {} | {} |",
                tally.pass,
                tally.ran(),
                size.map_or_else(|| "not measured".to_owned(), as_change)
            ));
        }
        out.push('\n');
    }
    out.push('\n');
}

/// What the reference compiler thought it could do.
fn reference_opinion(out: &mut String, summary: &Summary) {
    let total: usize = summary.facets.iter().map(|facet| facet.optimized + facet.missed).sum();
    if total == 0 {
        return;
    }
    out.push_str(&format!("## What {} said about these programs\n\n", summary.reference));
    out.push_str(&format!(
        "Asked with `-fopt-info`, {} reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.\n\n",
        summary.reference
    ));
    out.push_str("| facet | phase | it optimized | it says it missed |\n|---|---|---|---|\n");
    let mut ranked: Vec<&FacetSummary> = summary
        .facets
        .iter()
        .filter(|facet| facet.optimized + facet.missed > 0)
        .collect();
    ranked.sort_by_key(|facet| std::cmp::Reverse(facet.optimized + facet.missed));
    for facet in ranked.iter().take(20) {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            facet.facet.name(),
            facet.phase.name(),
            facet.optimized,
            facet.missed
        ));
    }
    out.push('\n');
}

/// How to get this report again.
fn reproducing(out: &mut String, summary: &Summary) {
    out.push_str("## Running this yourself\n\n");
    out.push_str("```sh\ncargo run --release -p rucc-corpus -- run \\\n");
    for id in &summary.toolchains {
        out.push_str(&format!("    --toolchain {id} \\\n"));
    }
    out.push_str(&format!("    --reference {}\n```\n\n", summary.reference));
    out.push_str(&format!(
        "The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same {} programs with the same digest, and a report that disagrees with this one is a report about a different commit.\n",
        summary.cases
    ));
}

/// A ratio written the way people talk about it.
fn as_change(ratio: f64) -> String {
    let percent = (ratio - 1.0) * 100.0;
    if percent.abs() < 0.5 {
        return "level".to_owned();
    }
    if percent > 0.0 {
        format!("{percent:.0} percent more")
    } else {
        format!("{:.0} percent less", -percent)
    }
}

/// A threshold written the same way.
fn as_percent_limit(ratio: f64) -> String {
    let percent = (ratio - 1.0) * 100.0;
    if percent.abs() < 0.5 {
        return "no worse".to_owned();
    }
    format!("within {percent:.0} percent")
}

/// Keeps a block of output from swamping the report.
fn clip(text: &str) -> String {
    const LINES: usize = 12;
    let trimmed = text.trim_end_matches('\n');
    if trimmed.is_empty() {
        return "(nothing)".to_owned();
    }
    let counted = trimmed.lines().count();
    if counted <= LINES {
        return trimmed.to_owned();
    }
    let head: Vec<&str> = trimmed.lines().take(LINES).collect();
    format!("{}\n... and {} more lines", head.join("\n"), counted - LINES)
}

/// The front of a digest, which is all anybody quotes.
fn short(digest: &str) -> String {
    digest.chars().take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::{as_change, clip, report_md};
    use crate::summary::summarise;
    use corpus_model::{
        Axes, Case, Compile, Dialect, Execute, Expect, Facet, Finding, Insight, Level, RunRecord,
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

    fn record_for(case: &Case, id: &str, text: u64) -> RunRecord {
        let mut record = RunRecord::skipped(case, id, Level::O2);
        record.compile = Compile {
            ok: true,
            status: 0,
            micros: 3000,
            diagnostics: String::new(),
            bytes: text * 4,
            text_bytes: text,
        };
        record.execute =
            Execute { ok: true, status: 0, micros: 90, repeats: 5, output: "1\n".to_owned() };
        record
    }

    fn sample(findings: Vec<Finding>) -> (Run, Vec<Case>) {
        let cases = vec![
            case_for(Facet::ConstantFold, "one"),
            case_for(Facet::LoopUnroll, "two"),
        ];
        let mut records = Vec::new();
        for case in &cases {
            let mut reference = record_for(case, "gcc-16", 100);
            reference.insights.push(Insight {
                kind: "optimized".to_owned(),
                pass: "loop-unroll".to_owned(),
                line: 7,
                message: "loop unrolled 4 times".to_owned(),
            });
            records.push(reference);
            let bigger = if case.facet == Facet::LoopUnroll { 250 } else { 95 };
            records.push(record_for(case, "rucc", bigger));
        }
        let verdicts: BTreeMap<String, Verdict> =
            records.iter().map(|r| (r.key(), Verdict::Pass)).collect();
        let run = Run {
            toolchains: vec![
                Toolchain {
                    id: "gcc-16".to_owned(),
                    program: "gcc-16".to_owned(),
                    version: "gcc (GCC) 16.1.0".to_owned(),
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
            findings,
        };
        (run, cases)
    }

    fn rendered(findings: Vec<Finding>) -> String {
        let (run, cases) = sample(findings);
        let summary = summarise(&run, "abc123def456abc123", cases.len());
        report_md(&run, &summary)
    }

    #[test]
    fn the_report_opens_with_the_answer_and_names_both_compilers() {
        let text = rendered(Vec::new());
        let opening: String = text.lines().take(4).collect::<Vec<&str>>().join("\n");
        assert!(opening.starts_with("# rucc corpus report"));
        assert!(opening.contains("Every case in the corpus produced the answer"));
        assert!(text.contains("gcc (GCC) 16.1.0"));
        assert!(text.contains("rucc 0.4.0"));
    }

    #[test]
    fn the_losses_are_shown_before_the_wins() {
        let text = rendered(Vec::new());
        let behind = text.find("Furthest behind").expect("no losses section");
        let ahead = text.find("Furthest ahead").expect("no wins section");
        assert!(behind < ahead, "the report leads with the wins, which spec 16 rule 3 forbids");
        // The regression is the loop unrolling facet, and it has to be named rather than
        // hidden inside an average.
        let losses = &text[behind..ahead];
        assert!(losses.contains("loop-unroll"), "{losses}");
    }

    #[test]
    fn a_failure_is_written_out_with_both_sides_of_the_comparison() {
        let (_, cases) = sample(Vec::new());
        let finding = Finding {
            case: cases[0].id.clone(),
            facet: Facet::ConstantFold,
            toolchain: "rucc".to_owned(),
            level: Level::O2,
            verdict: Verdict::Wrong,
            summary: "rucc printed the wrong answer for a case about constant folding".to_owned(),
            expected: "42\n".to_owned(),
            actual: "41\n".to_owned(),
        };
        let text = rendered(vec![finding]);
        assert!(text.contains("## What went wrong"));
        assert!(text.contains("Expected:"));
        assert!(text.contains("Got:"));
        assert!(text.contains("programs/"));
    }

    #[test]
    fn a_clean_run_has_no_failure_section_at_all() {
        let text = rendered(Vec::new());
        assert!(!text.contains("## What went wrong"));
    }

    #[test]
    fn the_report_says_how_to_produce_it_again() {
        let text = rendered(Vec::new());
        assert!(text.contains("## Running this yourself"));
        assert!(text.contains("cargo run --release -p rucc-corpus -- run"));
        assert!(text.contains("--reference gcc-16"));
    }

    #[test]
    fn what_the_reference_compiler_said_is_reported_as_a_lead_and_not_as_a_failure() {
        let text = rendered(Vec::new());
        assert!(text.contains("## What gcc-16 said about these programs"));
        assert!(text.contains("None of it is a pass or fail signal"));
    }

    #[test]
    fn the_prose_holds_to_the_house_style() {
        let text = rendered(Vec::new());
        assert!(!text.contains('\u{2014}'), "the report contains an em dash");
        assert!(!text.contains('\u{2013}'), "the report contains an en dash");
        for line in text.lines() {
            assert_ne!(line.trim(), "---", "the report contains a horizontal rule");
        }
    }

    #[test]
    fn a_ratio_is_written_the_way_a_person_would_say_it() {
        assert_eq!(as_change(1.0), "level");
        assert_eq!(as_change(1.30), "30 percent more");
        assert_eq!(as_change(0.90), "10 percent less");
        assert_eq!(as_change(1.001), "level");
    }

    #[test]
    fn a_wall_of_output_is_clipped_rather_than_pasted_into_the_report() {
        let long: String = (0..100).map(|n| format!("line {n}\n")).collect();
        let clipped = clip(&long);
        assert!(clipped.lines().count() <= 13);
        assert!(clipped.contains("and 88 more lines"));
        assert_eq!(clip(""), "(nothing)");
    }
}
