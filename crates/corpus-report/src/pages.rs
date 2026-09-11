//! The report as a tree of pages rather than as one long file.
//!
//! `report.md` is the right shape for the artifact of a single run. You open it, you scroll,
//! you close it. It is the wrong shape for what somebody arriving at the repository wants,
//! which is to learn in one screen how far the compiler has got and then click through to the
//! part they care about. Nine hundred lines of markdown in one file is a file people stop
//! opening, and a report nobody opens is a report that is not doing anything.
//!
//! So the same summary is rendered a second way, as a small tree:
//!
//! ```text
//! README.md                      the front page, with one generated block in it
//! reports/README.md              the hub: the verdict, the compilers, links to everything
//! reports/cost.md                what each facet cost, six ways, against the reference
//! reports/failures.md            what went wrong, and what the compiler admits is missing
//! reports/phases/README.md       one row per phase of the plan
//! reports/phases/<phase>.md      one phase, its facets, and links to the programs
//! ```
//!
//! Three properties hold and all three are load bearing.
//!
//! **The front page is not generated.** It is mostly prose that a person wrote and should keep
//! writing, so [`splice`] replaces what sits between two HTML comment markers and leaves the
//! rest alone. A front page with no markers in it comes back unchanged rather than appended to.
//! A generator that appends to a file it does not understand eventually eats somebody's prose.
//!
//! **Every page is a pure function of the run.** No clock, no filesystem, no environment. That
//! is what makes it possible to regenerate the tree and compare it against what is committed,
//! and it is why two runs of the same records produce the same bytes.
//!
//! **Only markdown is committed.** The records and the JSON summary these pages are rendered
//! from are workflow artifacts. They change on every run whether or not the compiler moved, so
//! keeping them in the history would make every diff unreadable.

use crate::size;
use crate::summary::{FacetScore, FacetSummary, Summary, Tally};
use corpus_model::{Facet, Finding, Level, Phase, Verdict};
use corpus_run::Run;
use std::fmt::Write as _;

/// Where the generated block on the front page starts.
pub const BEGIN: &str = "<!-- corpus:begin -->";

/// Where it ends.
pub const END: &str = "<!-- corpus:end -->";

/// How many findings a page writes out in full before it starts counting them instead.
const FINDINGS: usize = 40;

/// One page, and where it goes relative to the root of the repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    /// The path, always with forward slashes, always relative to the repository root.
    pub path: String,
    /// The whole file.
    pub text: String,
}

/// Renders the whole tree.
///
/// The order is stable and the content is a function of the arguments, so running this twice on
/// the same run produces the same pages in the same order.
#[must_use]
pub fn generate(run: &Run, summary: &Summary) -> Vec<Page> {
    let mut pages = vec![
        Page { path: "reports/README.md".to_owned(), text: hub(run, summary) },
        Page { path: "reports/cost.md".to_owned(), text: cost_page(run, summary) },
        Page { path: "reports/failures.md".to_owned(), text: failures_page(run) },
        Page { path: "reports/phases/README.md".to_owned(), text: phase_index(summary) },
    ];
    for phase in Phase::ALL {
        if summary.facets.iter().any(|facet| facet.phase == *phase) {
            pages.push(Page {
                path: format!("reports/phases/{}.md", phase.name()),
                text: phase_page(*phase, summary),
            });
        }
    }
    pages
}

/// The block that goes on the front page, between the two markers.
///
/// Short on purpose. The front page should say where things stand and then get out of the way,
/// and everything it leaves out is one click down.
#[must_use]
pub fn headline(run: &Run, summary: &Summary) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}\n", against(run, summary));
    let _ = writeln!(out, "| compiler | passed | wrong | rejected | not built yet | crashed |");
    let _ = writeln!(out, "|---|---|---|---|---|---|");
    for id in &summary.toolchains {
        let Some(tally) = summary.totals.get(id) else {
            continue;
        };
        let _ = writeln!(
            out,
            "| `{id}` | {} of {} | {} | {} | {} | {} |",
            tally.pass,
            tally.ran(),
            tally.wrong,
            tally.rejected,
            tally.unimplemented,
            tally.crashed
        );
    }
    let _ = writeln!(
        out,
        "\nThe full report is in [reports/README.md](reports/README.md). What each facet cost is in [reports/cost.md](reports/cost.md), what went wrong is in [reports/failures.md](reports/failures.md), and the breakdown by phase of the plan is in [reports/phases/README.md](reports/phases/README.md)."
    );
    out
}

/// Puts a generated block into a file that has the markers, and leaves any other file alone.
///
/// Returns the file unchanged when either marker is missing or when they are the wrong way
/// round, because both of those mean the file is not what this thinks it is, and writing to it
/// anyway would destroy something a person wrote.
#[must_use]
pub fn splice(existing: &str, block: &str) -> String {
    let (Some(begin), Some(end)) = (existing.find(BEGIN), existing.find(END)) else {
        return existing.to_owned();
    };
    if end < begin {
        return existing.to_owned();
    }
    format!("{}{BEGIN}\n\n{}\n\n{}", &existing[..begin], block.trim_end(), &existing[end..])
}

/// The hub, which is the page to open when the front page was not enough.
fn hub(run: &Run, summary: &Summary) -> String {
    let mut out = String::new();
    out.push_str("# The corpus report\n\n");
    let _ = writeln!(out, "{}\n", verdict(summary));
    let _ = writeln!(out, "{}\n", against(run, summary));

    out.push_str("| compiler | version | role |\n|---|---|---|\n");
    for toolchain in &run.toolchains {
        let _ = writeln!(
            out,
            "| `{}` | {} | {} |",
            toolchain.id,
            toolchain.version,
            if toolchain.reference { "reference" } else { "under test" }
        );
    }
    out.push('\n');

    out.push_str("## How the cases came out\n\n");
    out.push_str(
        "Every count is per case per level, so a corpus of a thousand programs built at five levels has five thousand results in it. The seven verdicts are always all seven and never collapsed into a pass rate, because `unimplemented` is not a failure and `skipped` is not a pass, and a single percentage hides which of those it counted.\n\n",
    );
    out.push_str(
        "| compiler | pass | wrong | rejected | unimplemented | accepted | crashed | skipped |\n",
    );
    out.push_str("|---|---|---|---|---|---|---|---|\n");
    for id in &summary.toolchains {
        let Some(tally) = summary.totals.get(id) else {
            continue;
        };
        let _ = writeln!(
            out,
            "| `{id}` | {} | {} | {} | {} | {} | {} | {} |",
            tally.pass,
            tally.wrong,
            tally.rejected,
            tally.unimplemented,
            tally.accepted,
            tally.crashed,
            tally.skipped
        );
    }
    out.push('\n');
    out.push_str(&meanings());

    out.push_str("## The rest of the report\n\n");
    out.push_str("| page | what is on it |\n|---|---|\n");
    out.push_str(
        "| [What it cost](cost.md) | Code size, size on disk, initialized data, compile time, run time and compiler memory, per facet, each against the reference build of the same program |\n",
    );
    let _ = writeln!(
        out,
        "| [What went wrong](failures.md) | {} |",
        went_wrong(run.failures(), run.gaps())
    );
    out.push_str(
        "| [By phase of the plan](phases/README.md) | One page per phase, its facets, how they came out, and links to the programs themselves |\n\n",
    );

    if !summary.targets.is_empty() {
        out.push_str("## The claims this run checks\n\n");
        out.push_str("| target | wanted | this run | met |\n|---|---|---|---|\n");
        for target in &summary.targets {
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} |",
                target.name,
                target.wanted,
                target
                    .actual
                    .map_or_else(|| "not measured".to_owned(), |value| format!("{value:.3}")),
                if target.met { "yes" } else { "no" }
            );
        }
        out.push('\n');
    }

    out.push_str(&reused_note(run));
    out.push_str(&reproducing(summary));
    out
}

/// Says how many of the numbers on the pages below were measured in this sitting.
///
/// An outcome keeps and a timing does not. A run assembled partly from this morning and partly
/// from a fortnight ago is a different claim from one gathered in one go, and a reader chasing a
/// timing regression has to be able to tell which one they are holding. Nothing is said when
/// nothing was reused, since the note would be noise on every page it appeared on.
fn reused_note(run: &Run) -> String {
    let reused = run.records.iter().filter(|record| record.reused).count();
    if reused == 0 {
        return String::new();
    }
    format!(
        "## How much of this was measured today\n\n{reused} of the {} results on these pages were read out of the record cache rather than built in this run. Their verdicts are as good as any other, since a program that printed the wrong answer prints it again. Their timings and their memory figures were measured on an earlier run of the same machine, so a comparison of seconds that spans them is a comparison across sittings. Run with `--refresh` for a set of numbers that were all taken at once.\n\n",
        run.records.len()
    )
}

/// What the failures page has on it, in the words the counts call for.
fn went_wrong(failures: usize, gaps: usize) -> String {
    match (failures, gaps) {
        (0, 0) => "Nothing this time. Both lists on that page are empty.".to_owned(),
        (0, gaps) => format!(
            "No failures, and {gaps} cases a compiler said it has not built yet, grouped so that twenty cases blocked on one missing thing read as one missing thing"
        ),
        (failures, 0) => format!("{failures} failures in full, and nothing a compiler admitted to"),
        (failures, gaps) => format!(
            "{failures} failures in full, and {gaps} cases a compiler said it has not built yet, grouped so that twenty cases blocked on one missing thing read as one missing thing"
        ),
    }
}

/// One sentence saying whether anything is wrong.
fn verdict(summary: &Summary) -> String {
    if summary.correct() {
        return "Every case in the corpus produced the answer the generator computed, on every compiler, at every level.".to_owned();
    }
    let failures: usize = summary.totals.values().map(Tally::failures).sum();
    format!(
        "{failures} case results did not come out as expected. They are on the failures page, worst first."
    )
}

/// What was run, against what, and how much of it there was.
fn against(run: &Run, summary: &Summary) -> String {
    let reference = run
        .reference()
        .map_or_else(|| summary.reference.clone(), |toolchain| toolchain.version.clone());
    format!(
        "{} programs{}, built at {}, against {}. Corpus digest `{}`.",
        summary.cases,
        how_big(summary),
        listed(&summary.levels),
        reference,
        short(&summary.corpus_digest)
    )
}

/// The size of the corpus, as a clause hung off the count of programs.
///
/// Empty when nothing measured it, which is what a run assembled from records written before the
/// size was recorded looks like. Nought lines is not a corpus anybody generated, so saying nothing
/// is both the honest answer and a better one than a sentence reading as a corpus that has lost
/// its programs.
fn how_big(summary: &Summary) -> String {
    if !summary.source.measured() {
        return String::new();
    }
    let lines = size::lines_of(summary.source.lines);
    if size::one_file_each(summary.source, summary.cases) {
        return format!(", one translation unit each and {lines} of C in all");
    }
    format!(", {} between them and {lines} of C in all", size::files_of(summary.source.files))
}

/// The levels written out the way a person would say them.
///
/// Per commit CI builds two of the five and nightly builds all of them, so both readings of
/// this line happen and a bare count would leave a reader guessing which two.
fn listed(levels: &[Level]) -> String {
    let names: Vec<String> = levels.iter().map(|level| format!("`{}`", level.name())).collect();
    match names.split_last() {
        None => "no levels at all".to_owned(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// What each verdict means, so the table above is readable without the source.
fn meanings() -> String {
    let mut out = String::from("| verdict | what it means |\n|---|---|\n");
    for verdict in [
        Verdict::Pass,
        Verdict::Wrong,
        Verdict::Rejected,
        Verdict::Unimplemented,
        Verdict::Accepted,
        Verdict::Crashed,
        Verdict::Skipped,
    ] {
        let _ = writeln!(out, "| `{}` | {} |", verdict.name(), meaning(verdict));
    }
    out.push('\n');
    out
}

/// One sentence per verdict.
const fn meaning(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pass => "It did what the generator worked out that it should do.",
        Verdict::Wrong => {
            "It compiled, it ran, and it printed something else. This is the one verdict that is unambiguously a bug in the compiler under test."
        }
        Verdict::Rejected => "It is valid C and the compiler would not compile it.",
        Verdict::Unimplemented => {
            "The compiler read it, recognised it, and said in its own words that it has not been taught to lower it. A line on a to-do list rather than a bug."
        }
        Verdict::Accepted => "It is not valid C and the compiler compiled it anyway.",
        Verdict::Crashed => {
            "The compiler failed in a way that was not a diagnostic, which is a crash or a timeout."
        }
        Verdict::Skipped => {
            "It was never run, because a tag excluded it or an earlier step failed."
        }
    }
}

/// Everything the run cost, per facet, against the reference.
fn cost_page(run: &Run, summary: &Summary) -> String {
    let mut out = String::new();
    out.push_str("# What it cost\n\n");
    out.push_str(
        "Six numbers per facet, each a median over the cases in that facet at the headline level, and each a ratio against the reference compiler building the same program on the same machine in the same run. A ratio below one is the compiler under test doing better.\n\n",
    );
    out.push_str("| number | what it is | why it is separate |\n|---|---|---|\n");
    out.push_str("| code | Allocated executable sections in the image | This is the code quality number. It is what the optimizer decided and nothing else. |\n");
    out.push_str("| on disk | The whole executable file | What a build costs somebody. Includes the runtime, the symbol table and whatever the linker padded with, none of which the optimizer chose. |\n");
    out.push_str("| data | Allocated initialized sections in the image | A compiler that unrolls by materializing a table and one that folds the loop away move the code column the same way and this column the opposite way. |\n");
    out.push_str("| compile | Wall clock for the build | Noisy and machine dependent. Worth watching between two commits of the compiler, not worth quoting on its own. |\n");
    out.push_str("| run | Wall clock for the program, fastest of the repetitions | The fastest rather than the mean, because every slower measurement has somebody else's work in it and there is no way to subtract that. |\n");
    out.push_str("| memory | Largest high water mark in the compiler's process tree | Sampled from outside the process, so it is a floor rather than an exact peak, and it is missing entirely on any platform that is not Linux. |\n\n");
    out.push_str(
        "None of these are averaged into a single figure for the corpus. A mean over fifty facets of wildly different shapes is a number with no referent.\n\n",
    );
    if summary.under_test().is_empty() {
        let _ = writeln!(
            out,
            "This run had no compiler under test in it. Every column above is a ratio of one compiler against the reference, and the only compiler here was the reference, so there is nothing to put in them. That is what a run on a machine rucc has no back end for looks like, and it is still worth doing, because it checks {} expected answers against a compiler that has been wrong about very few things since 1987.",
            summary.cases
        );
        return out;
    }

    out.push_str(
        "The `lines` column is the odd one out, because it is not a ratio and not a cost. It is how much C the facet is, counted once per case however many compilers and levels the case was built with, and it is here because none of the six columns beside it can be read without it. Four hundred milliseconds is quick for ten thousand lines and slow for two hundred. It is a denominator and never a score, so there is deliberately no lines per second anywhere on this page.\n\n",
    );

    for id in summary.under_test() {
        let _ = writeln!(out, "## `{id}` against `{}`\n", summary.reference);
        out.push_str(
            "| facet | cases | lines | code | on disk | data | compile | run | memory |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|---|---|\n");
        for facet in &summary.facets {
            let Some(score) = facet.score(id) else {
                continue;
            };
            let _ = writeln!(
                out,
                "| [`{}`]({}) | {} | {} | {} | {} | {} | {} | {} | {} |",
                facet.facet.name(),
                facet_link(facet.facet, "../"),
                facet.cases,
                size::cell(facet.source, facet.cases),
                ratio(score.size_ratio),
                ratio(score.disk_ratio),
                ratio(score.data_ratio),
                ratio(score.compile_ratio),
                speed(score),
                ratio(score.memory_ratio)
            );
        }
        out.push('\n');
        out.push_str(&measured_note(summary, id));
    }
    // Three of the six columns on this page are times or memory, so this is the page where a
    // reused record matters most. A ratio is no safer than a raw number here, since the two
    // compilers are cached separately and one half of it can be a fortnight older than the other.
    out.push_str(&reused_note(run));
    out
}

/// Says how much of the table above was actually measured.
///
/// Without this a page full of "not measured" reads as a broken generator, when usually it
/// means the run was on a machine that cannot answer or at a level the comparison refuses.
fn measured_note(summary: &Summary, id: &str) -> String {
    let scores: Vec<&FacetScore> = summary.facets.iter().filter_map(|f| f.score(id)).collect();
    let sized: usize = scores.iter().map(|score| score.compared).sum();
    let memoried: usize = scores.iter().map(|score| score.memory_compared).sum();
    if memoried == 0 && sized > 0 {
        return format!(
            "{sized} cases had a size to compare. None had a memory figure, which means this run was on a machine where the harness cannot read a process tree. Everything else on this page still holds.\n\n"
        );
    }
    format!("{sized} cases had a size to compare and {memoried} had a memory figure.\n\n")
}

/// Everything that went wrong, and everything the compiler admits it cannot do yet.
fn failures_page(run: &Run) -> String {
    let mut out = String::new();
    out.push_str("# What went wrong\n\n");
    if run.findings.is_empty() {
        out.push_str("Nothing. Every case produced the answer the generator computed.\n\n");
    } else {
        let _ = writeln!(
            out,
            "{} findings, worst first. A finding is one case, one compiler, one level.\n",
            run.findings.len()
        );
        for finding in run.findings.iter().take(FINDINGS) {
            out.push_str(&one_finding(finding));
        }
        if run.findings.len() > FINDINGS {
            let _ = writeln!(
                out,
                "And {} more. The whole list is in `findings.sarif` and `report.json`, both of which are uploaded as artifacts by the run that produced this page.\n",
                run.findings.len() - FINDINGS
            );
        }
    }

    out.push_str("## What the compiler says it has not built yet\n\n");
    out.push_str(
        "Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.\n\n",
    );
    let gaps = gap_clusters(run);
    if gaps.is_empty() {
        out.push_str("Nothing. No compiler in this run refused a case on those terms.\n");
        return out;
    }
    out.push_str("| what the compiler said | cases | compiler |\n|---|---|---|\n");
    for (said, count, id) in gaps {
        let _ = writeln!(out, "| {said} | {count} | `{id}` |", said = escape(&said));
    }
    out.push('\n');
    out
}

/// One failure, written the way somebody would write it in a bug report.
fn one_finding(finding: &Finding) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "### `{}` at `{}` on `{}`\n",
        finding.case,
        finding.level.name(),
        finding.toolchain
    );
    let _ = writeln!(out, "{}\n", finding.summary);
    let _ = writeln!(
        out,
        "Facet [`{}`]({}).\n",
        finding.facet.name(),
        facet_link(finding.facet, "../")
    );
    if !finding.expected.is_empty() || !finding.actual.is_empty() {
        let _ = writeln!(
            out,
            "```\nexpected: {}\nactual:   {}\n```\n",
            one_line(&finding.expected),
            one_line(&finding.actual)
        );
    }
    out
}

/// Groups the cases a compiler refused while saying it has not built the construct yet.
fn gap_clusters(run: &Run) -> Vec<(String, usize, String)> {
    let mut counted: std::collections::BTreeMap<(String, String), usize> =
        std::collections::BTreeMap::new();
    for record in &run.records {
        if run.verdict(record) != Verdict::Unimplemented {
            continue;
        }
        let said = first_line(&record.compile.diagnostics);
        *counted.entry((said, record.toolchain.clone())).or_default() += 1;
    }
    let mut out: Vec<(String, usize, String)> =
        counted.into_iter().map(|((said, id), count)| (said, count, id)).collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}

/// One row per phase of the plan, which is the table to read when deciding what to do next.
fn phase_index(summary: &Summary) -> String {
    let mut out = String::new();
    out.push_str("# By phase of the plan\n\n");
    out.push_str(
        "The phases are the ones in the M4 plan, in the order the plan does them, because each one is built on the one above it. This is the table to read when deciding what to implement next.\n\n",
    );
    out.push_str("| phase | facets | cases | lines |");
    for id in summary.under_test() {
        let _ = write!(out, " `{id}` passed | `{id}` code |");
    }
    out.push_str("\n|---|---|---|---|");
    for _ in summary.under_test() {
        out.push_str("---|---|");
    }
    out.push('\n');

    for phase in Phase::ALL {
        let facets = in_phase(summary, *phase);
        if facets.is_empty() {
            continue;
        }
        let cases: usize = facets.iter().map(|facet| facet.cases).sum();
        let size = crate::summary::add_up(facets.iter().map(|facet| facet.source));
        let _ = write!(
            out,
            "| [{0}]({0}.md) | {1} | {cases} | {2} |",
            phase.name(),
            facets.len(),
            size::cell(size, cases)
        );
        for id in summary.under_test() {
            let (tally, size) = rolled_up(&facets, id);
            let _ = write!(out, " {} of {} | {} |", tally.pass, tally.ran(), ratio(size));
        }
        out.push('\n');
    }
    out.push_str("\nEach phase has a page of its own with its facets on it, and each facet links to the programs it was generated for.\n");
    out
}

/// One phase, its facets, and the way into the programs.
fn phase_page(phase: Phase, summary: &Summary) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# {}\n", phase.name());
    let _ = writeln!(out, "{}\n", phase.describe());

    let facets = in_phase(summary, phase);
    if summary.under_test().is_empty() {
        out.push_str(
            "This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.\n\n",
        );
    }
    out.push_str("| facet | cases | lines |");
    for id in summary.under_test() {
        let _ = write!(out, " `{id}` passed | `{id}` code | `{id}` compile | `{id}` memory |");
    }
    out.push_str("\n|---|---|---|");
    for _ in summary.under_test() {
        out.push_str("---|---|---|---|");
    }
    out.push('\n');

    for facet in &facets {
        let _ = write!(
            out,
            "| [`{}`]({}) | {} | {} |",
            facet.facet.name(),
            facet_link(facet.facet, "../../"),
            facet.cases,
            size::cell(facet.source, facet.cases)
        );
        for id in summary.under_test() {
            let Some(score) = facet.score(id) else {
                out.push_str(" not run | not run | not run | not run |");
                continue;
            };
            let _ = write!(
                out,
                " {} of {} | {} | {} | {} |",
                score.tally.pass,
                score.tally.ran(),
                ratio(score.size_ratio),
                ratio(score.compile_ratio),
                ratio(score.memory_ratio)
            );
        }
        out.push('\n');
    }

    let optimized: usize = facets.iter().map(|facet| facet.optimized).sum();
    let missed: usize = facets.iter().map(|facet| facet.missed).sum();
    if optimized + missed > 0 {
        let _ = write!(
            out,
            "\nThe reference compiler said it took {optimized} transformations in this phase and wanted {missed} more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.\n"
        );
    }
    let _ = write!(
        out,
        "\nBack to [the hub](../README.md), or across to [what it cost](../cost.md).\n"
    );
    out
}

/// The facets of one phase, in the order the summary has them.
fn in_phase(summary: &Summary, phase: Phase) -> Vec<&FacetSummary> {
    summary.facets.iter().filter(|facet| facet.phase == phase).collect()
}

/// Adds up the tallies of several facets and takes the middle of their size ratios.
fn rolled_up(facets: &[&FacetSummary], id: &str) -> (Tally, Option<f64>) {
    let mut tally = Tally::default();
    let mut sizes = Vec::new();
    for facet in facets {
        let Some(score) = facet.score(id) else {
            continue;
        };
        tally.pass += score.tally.pass;
        tally.wrong += score.tally.wrong;
        tally.rejected += score.tally.rejected;
        tally.unimplemented += score.tally.unimplemented;
        tally.accepted += score.tally.accepted;
        tally.crashed += score.tally.crashed;
        tally.skipped += score.tally.skipped;
        if let Some(value) = score.size_ratio {
            sizes.push(value);
        }
    }
    (tally, crate::summary::median(&mut sizes))
}

/// The path from a page to the listing of the programs a facet was generated for.
fn facet_link(facet: Facet, up: &str) -> String {
    format!("{up}programs/{}/{}/README.md", facet.phase().name(), facet.name())
}

/// How to get this report again.
fn reproducing(summary: &Summary) -> String {
    let mut out = String::from(
        "## Running this yourself\n\n```sh\ncargo run --release -p rucc-corpus -- run \\\n",
    );
    for id in &summary.toolchains {
        let _ = writeln!(out, "    --toolchain {id} \\");
    }
    let _ = writeln!(out, "    --reference {}\n```\n", summary.reference);
    let _ = writeln!(
        out,
        "The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same {} programs with the same digest, and a report that disagrees with this one is a report about a different commit.",
        summary.cases
    );
    out
}

/// A ratio written the way people say it out loud.
fn ratio(value: Option<f64>) -> String {
    let Some(value) = value else {
        return "not measured".to_owned();
    };
    let percent = (value - 1.0) * 100.0;
    if percent.abs() < 0.5 {
        return "level".to_owned();
    }
    if percent > 0.0 { format!("{percent:.0}% more") } else { format!("{:.0}% less", -percent) }
}

/// A run time, or the reason there is no run time worth printing.
///
/// Same rule as the human report's, and it has to be the same rule, because two pages of the
/// same run that disagree about whether a facet got faster are worse than either page alone.
fn speed(score: &FacetScore) -> String {
    let cell = ratio(score.speed_ratio);
    if cell == "level" || score.speed_spread.is_none() || score.speed_is_real() {
        return cell;
    }
    "inside the noise".to_owned()
}

/// The first line of a diagnostic with the file, line and column taken off the front.
fn first_line(diagnostics: &str) -> String {
    let line = diagnostics.lines().next().unwrap_or_default().trim();
    let mut fields = line.splitn(4, ':');
    match (fields.next(), fields.next(), fields.next(), fields.next()) {
        (Some(_), Some(a), Some(b), Some(rest))
            if a.trim().parse::<u32>().is_ok() && b.trim().parse::<u32>().is_ok() =>
        {
            rest.trim().to_owned()
        }
        _ => line.to_owned(),
    }
}

/// Squashes something multi line down to one line so it fits in a table cell.
fn one_line(text: &str) -> String {
    let flat: String = text.trim().replace('\n', "\\n");
    if flat.chars().count() <= 120 {
        return flat;
    }
    let head: String = flat.chars().take(117).collect();
    format!("{head}...")
}

/// Makes text safe to put in a markdown table cell.
fn escape(text: &str) -> String {
    text.replace('|', "\\|")
}

/// The front of a digest, which is all anybody quotes.
fn short(digest: &str) -> String {
    digest.chars().take(16).collect()
}

/// The level the ratios on these pages are taken at, restated here so the pages can say so.
#[must_use]
pub const fn headline_level() -> Level {
    Level::O2
}

#[cfg(test)]
mod tests {
    use super::{BEGIN, END, generate, headline, ratio, splice};
    use crate::summary::summarise;
    use corpus_model::{Case, Compile, Execute, Expect, Facet, Level, RunRecord, Toolchain};
    use corpus_run::Run;
    use std::collections::BTreeMap;

    fn a_run() -> (Run, crate::summary::Summary) {
        let case = Case::new(
            Facet::LoopUnroll,
            corpus_model::case::Axes::default(),
            corpus_model::case::Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output("7\n".to_owned()),
        );
        let mut records = Vec::new();
        let mut verdicts = BTreeMap::new();
        for (id, text, peak) in [("gcc-16", 1000u64, 100u64 << 20), ("rucc", 1400, 180u64 << 20)] {
            let mut record = RunRecord::skipped(&case, id, Level::O2);
            record.compile = Compile {
                ok: true,
                status: 0,
                micros: if id == "rucc" { 2000 } else { 1000 },
                bytes: text * 8,
                text_bytes: text,
                data_bytes: text / 4,
                peak_bytes: Some(peak),
                ..Compile::skipped()
            };
            record.execute = Execute {
                ok: true,
                status: 0,
                micros: 500,
                repeats: 5,
                output: "7\n".to_owned(),
                ..Execute::skipped()
            };
            verdicts.insert(record.key(), corpus_model::Verdict::Pass);
            records.push(record);
        }
        let run = Run {
            toolchains: vec![
                Toolchain {
                    id: "gcc-16".to_owned(),
                    program: "gcc-16".to_owned(),
                    version: "gcc-16 (GCC) 16.2.0".to_owned(),
                    reference: true,
                },
                Toolchain {
                    id: "rucc".to_owned(),
                    program: "rucc".to_owned(),
                    version: "rucc 0.7.8".to_owned(),
                    reference: false,
                },
            ],
            records,
            verdicts,
            findings: Vec::new(),
        };
        let summary = summarise(&run, "d".repeat(64).as_str(), 1);
        (run, summary)
    }

    #[test]
    fn the_same_run_produces_the_same_pages_every_time() {
        let (run, summary) = a_run();
        assert_eq!(generate(&run, &summary), generate(&run, &summary));
    }

    #[test]
    fn every_page_has_a_heading_and_a_path_under_reports() {
        let (run, summary) = a_run();
        for page in generate(&run, &summary) {
            assert!(page.path.starts_with("reports/"), "{}", page.path);
            assert!(page.text.starts_with("# "), "{} does not open with a heading", page.path);
            assert!(!page.text.contains("\n\n\n"), "{} has a gap in it", page.path);
        }
    }

    #[test]
    fn every_relative_link_points_at_something_that_could_exist() {
        // The check that a link resolves against the filesystem belongs in CI, where the
        // programs are actually on disk. What is checked here is the arithmetic: a page two
        // directories down has to climb two directories to reach the programs.
        let (run, summary) = a_run();
        for page in generate(&run, &summary) {
            let depth = page.path.matches('/').count();
            for link in page.text.split("](").skip(1).filter_map(|rest| rest.split(')').next()) {
                if link.starts_with("http") || link.starts_with('#') {
                    continue;
                }
                let climbs = link.matches("../").count();
                assert!(climbs <= depth, "{} climbs past the root with {link}", page.path);
            }
        }
    }

    #[test]
    fn the_headline_names_the_reference_compiler_and_both_tallies() {
        let (run, summary) = a_run();
        let block = headline(&run, &summary);
        assert!(block.contains("16.2.0"), "{block}");
        assert!(block.contains("`rucc`"), "{block}");
        assert!(block.contains("reports/cost.md"), "{block}");
    }

    #[test]
    fn a_front_page_without_the_markers_comes_back_untouched() {
        let prose = "# rucc corpus\n\nSomebody wrote this.\n";
        assert_eq!(splice(prose, "generated"), prose);
        let backwards = format!("{END}\nmiddle\n{BEGIN}\n");
        assert_eq!(splice(&backwards, "generated"), backwards);
    }

    #[test]
    fn splicing_twice_produces_the_same_front_page_as_splicing_once() {
        let prose = format!("# Title\n\nprose\n\n{BEGIN}\n\nold\n\n{END}\n\nmore prose\n");
        let once = splice(&prose, "new block");
        let twice = splice(&once, "new block");
        assert_eq!(once, twice);
        assert!(once.contains("new block"));
        assert!(!once.contains("old"));
        assert!(once.contains("more prose"), "the prose after the block was eaten");
    }

    #[test]
    fn a_ratio_that_nobody_measured_says_so_rather_than_saying_parity() {
        assert_eq!(ratio(None), "not measured");
        assert_eq!(ratio(Some(1.0)), "level");
        assert_eq!(ratio(Some(1.4)), "40% more");
        assert_eq!(ratio(Some(0.8)), "20% less");
    }

    #[test]
    fn the_cost_page_carries_every_one_of_the_six_numbers() {
        let (run, summary) = a_run();
        let pages = generate(&run, &summary);
        let cost = pages.iter().find(|page| page.path == "reports/cost.md").unwrap();
        for column in ["code", "on disk", "data", "compile", "run", "memory"] {
            assert!(cost.text.contains(column), "the cost page has no {column} column");
        }
        // 1400 against 1000 is forty percent more code, and 180 against 100 megabytes is
        // eighty percent more memory. If either of those is missing the wiring is broken
        // somewhere between the record and the page.
        assert!(cost.text.contains("40% more"), "{}", cost.text);
        assert!(cost.text.contains("80% more"), "{}", cost.text);
    }

    #[test]
    fn every_page_that_quotes_a_cost_says_how_much_source_it_was_against() {
        let (run, summary) = a_run();
        let block = headline(&run, &summary);
        assert!(block.contains("1 line of C in all"), "{block}");
        let pages = generate(&run, &summary);
        let hub = pages.iter().find(|page| page.path == "reports/README.md").unwrap();
        assert!(hub.text.contains("1 line of C in all"), "{}", hub.text);
        let cost = pages.iter().find(|page| page.path == "reports/cost.md").unwrap();
        assert!(cost.text.contains("| lines |"), "the cost page has no lines column");
        let phases = pages.iter().find(|page| page.path == "reports/phases/README.md").unwrap();
        assert!(phases.text.contains("| lines |"), "the phase index has no lines column");
        let loops = pages.iter().find(|page| page.path == "reports/phases/loops.md").unwrap();
        assert!(loops.text.contains("| lines |"), "the phase page has no lines column");
    }

    #[test]
    fn a_run_from_before_the_size_was_kept_leaves_the_clause_out_rather_than_saying_nought() {
        let (mut run, _) = a_run();
        for record in &mut run.records {
            record.source = corpus_model::Source::default();
        }
        let summary = summarise(&run, "d".repeat(64).as_str(), 1);
        let block = headline(&run, &summary);
        assert!(!block.contains("of C in all"), "{block}");
        assert!(block.contains("1 programs"), "the rest of the sentence is still there");
        for page in generate(&run, &summary) {
            assert!(!page.text.contains("0 lines of C"), "{} reports an empty corpus", page.path);
        }
    }

    #[test]
    fn a_run_that_measured_everything_itself_says_nothing_about_a_cache() {
        let (run, summary) = a_run();
        for page in generate(&run, &summary) {
            assert!(!page.text.contains("record cache"), "{} talks about a cache", page.path);
        }
    }

    #[test]
    fn a_page_that_quotes_a_cached_timing_says_where_it_came_from() {
        let (mut run, summary) = a_run();
        run.records[0].reused = true;
        let pages = generate(&run, &summary);
        for path in ["reports/README.md", "reports/cost.md"] {
            let page = pages.iter().find(|page| page.path == path).unwrap();
            assert!(page.text.contains("1 of the 2 results"), "{path}: {}", page.text);
            assert!(page.text.contains("--refresh"), "{path} does not say how to get fresh ones");
        }
    }
}
