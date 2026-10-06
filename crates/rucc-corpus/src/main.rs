//! The command line front end to the corpus.
//!
//! Five things it does. `gen` writes the C out, so the corpus in the repository is readable C
//! that anybody can open, compile by hand and argue with, rather than a program that promises
//! to produce some. `run` builds every program with every compiler at every level, checks the
//! answers against what the generator computed, and writes the reports. `diff` compares two
//! reports, which is how progress across the phases of the plan gets tracked. `list` says what
//! is in the corpus. `sweep` moves each of rucc's thresholds and measures what moved.
//!
//! Everything is deterministic. The same commit produces the same programs with the same
//! digest, so a report that disagrees with another report is a report about different code.

mod args;
mod diff;
mod emit;
mod sweep;

use args::Args;
use corpus_gen::Options;
use corpus_model::{Facet, Level};
use corpus_report::terminal::Watcher;
use corpus_run::{Plan, cache::Reuse, known, quiet, toolchain::Spec};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    match dispatch(raw) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("rucc-corpus: {message}");
            ExitCode::from(2)
        }
    }
}

/// Picks the subcommand and runs it.
fn dispatch(raw: Vec<String>) -> Result<ExitCode, String> {
    let args = Args::parse(raw)?;
    match args.command.as_str() {
        "gen" => generate(&args),
        "run" => run(&args),
        "diff" => diff::command(&args),
        "list" => list(&args),
        "sweep" => sweep::command(&args),
        "help" | "" => {
            print!("{USAGE}");
            Ok(ExitCode::SUCCESS)
        }
        other => Err(format!("{other} is not a command, try `rucc-corpus help`")),
    }
}

const USAGE: &str = "\
rucc-corpus, the systematic C corpus for rucc

  rucc-corpus gen [options]     write the corpus out as C
  rucc-corpus run [options]     build and run it against every compiler
  rucc-corpus diff old new      compare two report.json files
  rucc-corpus list [options]    say what is in the corpus
  rucc-corpus sweep [options]   move each of rucc's thresholds to half and double and measure it

Options for gen:
  --out DIR            where to write, default programs
  --manifest FILE      where to write the manifest, default programs/manifest.json
  --facet NAME         only this facet, may be repeated
  --limit N            at most this many cases per facet
  --clean              remove what is already there first

Options for run:
  --toolchain ID=PROG  a compiler to test, may be repeated, default gcc-16 and rucc
  --reference ID       the compiler the others are measured against, default gcc-16
  --flag ID=FLAG       a flag for one compiler on every compile, may be repeated
  --level NAME         a level to build at, may be repeated, default all five
  --facet NAME         only this facet, may be repeated
  --limit N            at most this many cases per facet
  --exclude-tag TAG    skip cases carrying this tag, may be repeated
  --out DIR            where the reports go, default reports
  --work DIR           where the builds happen, default target/corpus-work
  --jobs N             how many builds at once, default one per core
  --repeats N          how many times each program is timed, default five
  --keep               keep the working directory of cases that passed
  --quiet              say nothing while it runs
  --no-pages           do not write the linked report pages or touch the front page
  --refresh            build every case, then keep the results for next time
  --no-cache           neither read nor write the record cache
  --known FILE         what each compiler is allowed to fail, default known-failures.json
  --quiet-passes FILE  which rucc passes may fire on nothing, default quiet-passes.json
  --accept             write those two files from this run instead of checking against them

Options for sweep, which also takes --facet, --limit, --exclude-tag, --jobs and --repeats:
  --rucc PROG          the rucc to sweep, default rucc
  --reference ID=PROG  the compiler to measure against, default gcc-16
  --level NAME         the level to build at, default O2
  --row NAME           only this row of --print-params, may be repeated
  --out DIR            where the reports go, default reports/sweep
  --work DIR           where the builds happen, default target/sweep-work
  --no-cache           neither read nor write the record cache
  --quiet              say nothing while it runs
";

/// Writes the corpus out as C.
fn generate(args: &Args) -> Result<ExitCode, String> {
    args.only(&["out", "manifest", "facet", "limit", "clean"])?;
    let out = PathBuf::from(args.value_or("out", "programs"));
    let manifest_path =
        args.value("manifest").map_or_else(|| out.join("manifest.json"), PathBuf::from);

    let corpus = corpus_gen::generate(&options(args)?)?;
    if args.flag("clean") && out.exists() {
        std::fs::remove_dir_all(&out).map_err(|error| format!("{}: {error}", out.display()))?;
    }
    let written = emit::write_programs(&out, &corpus)?;
    emit::write_manifest(&manifest_path, &corpus)?;

    println!(
        "wrote {} programs in {written} files across {} facets to {}",
        corpus.cases.len(),
        corpus.by_facet().len(),
        out.display()
    );
    println!("corpus digest {}", corpus.digest());
    Ok(ExitCode::SUCCESS)
}

/// One line saying how much of this run was measured today.
///
/// Printed on every run rather than only when something came out of the cache, because the
/// interesting case is the one where somebody expected a fresh set of numbers and did not get
/// one, and a line that only appears sometimes is a line nobody learns to look for.
fn reuse_line(outcome: &corpus_run::Run, reuse: Reuse) -> String {
    let reused = outcome.records.iter().filter(|record| record.reused).count();
    let built = outcome.records.len() - reused;
    match reuse {
        Reuse::Off => format!("{} built, nothing read from the cache", cases(built)),
        Reuse::Refresh if built > 0 => {
            format!("{} built and kept for next time", cases(built))
        }
        _ if reused == 0 => format!("{} built, none of them read from the cache", cases(built)),
        _ => format!("{} read from the cache and {} built", cases(reused), cases(built)),
    }
}

/// A count of results with the word after it in the right number.
fn cases(how_many: usize) -> String {
    format!("{how_many} result{}", if how_many == 1 { "" } else { "s" })
}

/// Everything `run` will accept.
///
/// Spelled out here rather than at the call site so that a test can hold it against the usage
/// text. An option the program takes and does not document is an option nobody uses.
const RUN_OPTIONS: &[&str] = &[
    "toolchain",
    "reference",
    "flag",
    "level",
    "facet",
    "limit",
    "exclude-tag",
    "out",
    "work",
    "jobs",
    "repeats",
    "keep",
    "quiet",
    "no-pages",
    "refresh",
    "no-cache",
    "known",
    "quiet-passes",
    "accept",
];

/// Builds and runs the corpus, and writes the reports.
fn run(args: &Args) -> Result<ExitCode, String> {
    args.only(RUN_OPTIONS)?;

    let corpus = corpus_gen::generate(&options(args)?)?;
    let work = PathBuf::from(args.value_or("work", "target/corpus-work"));
    let mut plan = Plan::new(&work);
    plan.specs = specs(args)?;
    plan.levels = levels(args)?;
    plan.keep_passes = args.flag("keep");
    plan.exclude_tags = args.values("exclude-tag").to_vec();
    // Both at once is a contradiction rather than a preference, so it is an error instead of
    // one of them quietly winning.
    plan.reuse = match (args.flag("refresh"), args.flag("no-cache")) {
        (true, true) => return Err("--refresh and --no-cache ask for opposite things".to_owned()),
        (true, false) => Reuse::Refresh,
        (false, true) => Reuse::Off,
        (false, false) => Reuse::Allow,
    };
    if let Some(jobs) = args.number("jobs")? {
        plan.jobs = jobs.max(1);
    }
    if let Some(repeats) = args.number("repeats")? {
        plan.repeats = u32::try_from(repeats).map_err(|_| "--repeats is too large".to_owned())?;
    }

    let quiet = args.flag("quiet");
    if !quiet {
        eprintln!(
            "running {} cases against {} compilers at {} levels",
            corpus.cases.len(),
            plan.specs.len(),
            plan.levels.len()
        );
    }
    let watcher = Watcher::new(quiet);
    let outcome = corpus_run::execute(&corpus, &plan, &|progress, record, verdict| {
        watcher.saw(progress, record, verdict);
    })?;
    watcher.finished();

    let reports = PathBuf::from(args.value_or("out", "reports"));
    let summary =
        corpus_report::write_all(&reports, &outcome, &corpus.digest(), corpus.cases.len())?;

    // The page tree lives beside the reports directory rather than inside it, because the
    // front page it splices is at the root of the repository.
    //
    // On by default, because the pages are the version of the report anybody actually reads
    // and a flag people have to know about is a flag nobody turns on. Off for the jobs that
    // run the corpus to check something and then throw the output away, since those would
    // otherwise rewrite a tree they were never asked to touch.
    if args.flag("no-pages") {
        println!("wrote the reports to {}", reports.display());
    } else {
        let root = reports
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf);
        let pages = corpus_report::write_pages(&root, &outcome, &summary)?;
        println!(
            "wrote the reports to {} and {pages} pages under {}",
            reports.display(),
            root.display()
        );
    }
    println!("{}", reuse_line(&outcome, plan.reuse));
    for target in &summary.targets {
        println!("  {:<28} {}", target.name, if target.met { "met" } else { "not met" });
    }
    let gaps = outcome.gaps();
    if gaps > 0 {
        println!(
            "{gaps} case{} needs something a compiler has not built yet, listed under \"what is not built yet\"",
            if gaps == 1 { "" } else { "s" }
        );
    }
    if outcome.failed() {
        println!("{} results did not come out as expected", outcome.failures());
    } else {
        println!("every case produced the answer the generator computed");
    }
    let quiet = against_the_quiet(args, &outcome)?;
    let known = against_the_known(args, &corpus, &plan, &outcome)?;
    Ok(if quiet { known } else { ExitCode::FAILURE })
}

/// Compares the passes that fired on nothing against the file that says which may.
///
/// Only for a run of the whole corpus. A run narrowed to some facets or some cases leaves out
/// the programs a pass is written for, and a pass quiet there says nothing about the pass.
/// Answers whether the run agreed with the file. See tamnd/rucc#2967.
fn against_the_quiet(args: &Args, outcome: &corpus_run::Run) -> Result<bool, String> {
    let builds = corpus_run::firing::builds(&outcome.records);
    if builds.is_empty() {
        return Ok(true);
    }
    if args.value("facet").is_some() || args.value("limit").is_some() {
        println!("the passes that fired on nothing are only checked on a run of the whole corpus");
        return Ok(true);
    }
    let path = PathBuf::from(args.value_or("quiet-passes", quiet::FILE));
    let quiet = quiet::Quiet::read(&path)?;
    if args.flag("accept") {
        let next = quiet.accepting(&builds);
        std::fs::write(&path, next.to_text())
            .map_err(|error| format!("could not write {}: {error}", path.display()))?;
        println!("wrote {} lines to {}", next.entries.len(), path.display());
        return Ok(true);
    }
    let check = quiet.check(&builds);
    if check.agrees() {
        println!(
            "every pass that fired on nothing is one {} names, and every line in it is still quiet",
            path.display()
        );
        return Ok(true);
    }
    let file = path.display();
    for entry in check.unexpected.iter().take(TOLD) {
        println!("  {} fired on nothing and {file} does not name it", entry.describe());
    }
    for entry in check.fired.iter().take(TOLD) {
        println!("  {} is named in {file} and fired, so the line comes out", entry.describe());
    }
    for entry in check.gone.iter().take(TOLD) {
        println!("  {} is named in {file} and no longer runs at that level", entry.describe());
    }
    println!("\nrun the same command with --accept to write the file this run would have made");
    Ok(false)
}

/// Compares what went wrong against the file that says what is allowed to go wrong.
///
/// This is what decides the exit status, rather than the raw failure count. A compiler under
/// development fails cases it has not got to yet, and a job that goes red on those is a job that
/// is red every day and catches nothing on the day it matters. See tamnd/rucc-corpus#22.
fn against_the_known(
    args: &Args,
    corpus: &corpus_model::Manifest,
    plan: &Plan,
    outcome: &corpus_run::Run,
) -> Result<ExitCode, String> {
    let path = PathBuf::from(args.value_or("known", known::FILE));
    let known = known::Known::read(&path)?;
    // A facet whose every case was left out by a tag was not run, so the run has nothing to say
    // about it. Without this an x86-64 run that leaves out the AArch64 facets would read every
    // line those facets have in the file as a case that had started working.
    let covered = known::Coverage::new(
        plan.specs.iter().map(|spec| spec.id.clone()),
        corpus.cases.iter().filter(|case| !plan.excludes(case)).map(|case| case.facet),
    );

    if args.flag("accept") {
        let next = known.accepting(&outcome.findings, &covered);
        std::fs::write(&path, next.to_text())
            .map_err(|error| format!("could not write {}: {error}", path.display()))?;
        println!("wrote {} lines to {}", next.entries.len(), path.display());
        return Ok(ExitCode::SUCCESS);
    }

    let check = known.check(&outcome.findings, &covered);
    if check.agrees() {
        if !known.entries.is_empty() {
            println!(
                "every failure is one {} already names, and every line in it still fails",
                path.display()
            );
        }
        return Ok(ExitCode::SUCCESS);
    }

    if !check.unexpected.is_empty() {
        println!(
            "\n{} result{} went wrong that {} does not name:",
            check.unexpected.len(),
            if check.unexpected.len() == 1 { "" } else { "s" },
            path.display()
        );
        for finding in check.unexpected.iter().take(TOLD) {
            println!("  {} {} {}", finding.toolchain, finding.case, finding.verdict);
            println!("    {}", finding.summary);
        }
        if check.unexpected.len() > TOLD {
            println!("  and {} more, all of them in the report", check.unexpected.len() - TOLD);
        }
    }
    if !check.fixed.is_empty() {
        println!(
            "\n{} line{} in {} did not fail this time, so the compiler has caught up:",
            check.fixed.len(),
            if check.fixed.len() == 1 { "" } else { "s" },
            path.display()
        );
        for entry in check.fixed.iter().take(TOLD) {
            println!("  {}", entry.describe());
        }
        if check.fixed.len() > TOLD {
            println!("  and {} more", check.fixed.len() - TOLD);
        }
    }
    println!("\nrun the same command with --accept to write the file this run would have made");
    Ok(ExitCode::FAILURE)
}

/// How many lines of either kind are printed before the rest are counted instead.
///
/// Enough to see the shape of what happened without a wall of output in a CI log that nobody
/// scrolls. The report has all of them.
const TOLD: usize = 20;

/// Says what is in the corpus, without building anything.
fn list(args: &Args) -> Result<ExitCode, String> {
    args.only(&["facet", "limit", "format"])?;
    let corpus = corpus_gen::generate(&options(args)?)?;
    if args.value("format") == Some("json") {
        print!("{}", corpus.to_json().to_pretty());
        println!();
        return Ok(ExitCode::SUCCESS);
    }
    println!("{:<24} {:<16} {:>6}  what it is about", "facet", "phase", "cases");
    for (facet, count) in corpus.by_facet() {
        println!(
            "{:<24} {:<16} {count:>6}  {}",
            facet.name(),
            facet.phase().name(),
            facet.describe()
        );
    }
    println!("\n{} cases, digest {}", corpus.cases.len(), corpus.digest());
    Ok(ExitCode::SUCCESS)
}

/// The generator options that `gen`, `run` and `list` all share.
fn options(args: &Args) -> Result<Options, String> {
    let mut opts = Options::all();
    for name in args.values("facet") {
        let facet = Facet::parse(name)
            .ok_or_else(|| format!("{name} is not a facet, try `rucc-corpus list`"))?;
        opts.facets.push(facet);
    }
    if let Some(limit) = args.number("limit")? {
        opts.limit_per_facet = Some(limit);
    }
    Ok(opts)
}

/// The compilers to test, and which one is the reference.
fn specs(args: &Args) -> Result<Vec<Spec>, String> {
    let asked = args.values("toolchain");
    let mut specs: Vec<Spec> = if asked.is_empty() {
        corpus_run::toolchain::defaults()
    } else {
        asked.iter().map(|text| Spec::parse(text)).collect()
    };

    let wanted = args.value("reference").unwrap_or("gcc-16");
    if args.value("reference").is_some() || asked.is_empty() {
        for spec in &mut specs {
            spec.reference = spec.id == wanted;
        }
    }
    // A flag goes to the compiler it names and no other. rucc on Windows needs to be told its
    // target and where MSYS2 keeps the headers and libraries, and GCC there would reject both.
    for text in args.values("flag") {
        let Some((id, flag)) = text.split_once('=') else {
            return Err(format!("--flag {text} should be ID=FLAG"));
        };
        let Some(spec) = specs.iter_mut().find(|spec| spec.id == id) else {
            return Err(format!(
                "--flag {text} names {id}, which is not one of the compilers under test"
            ));
        };
        spec.extra.push(flag.to_owned());
    }
    if !specs.iter().any(|spec| spec.reference) {
        // Without a reference there is nothing to measure size or speed against, and the
        // report would silently drop every ratio. Better to say so now.
        return Err(format!(
            "{wanted} is not one of the compilers under test, so there is nothing to compare against"
        ));
    }
    Ok(specs)
}

/// The levels to build at.
fn levels(args: &Args) -> Result<Vec<Level>, String> {
    let asked = args.values("level");
    if asked.is_empty() {
        return Ok(Level::ALL.to_vec());
    }
    let mut levels = Vec::new();
    for name in asked {
        let level = Level::parse(name)
            .ok_or_else(|| format!("{name} is not a level, try one of O0 O1 O2 O3 Os"))?;
        levels.push(level);
    }
    levels.sort_unstable();
    levels.dedup();
    Ok(levels)
}

#[cfg(test)]
mod tests {
    use super::{RUN_OPTIONS, USAGE, levels, options, specs};
    use crate::args::Args;
    use corpus_model::{Facet, Level};

    fn parse(line: &str) -> Args {
        Args::parse(line.split_whitespace().map(str::to_owned)).unwrap()
    }

    #[test]
    fn every_option_the_run_command_takes_is_in_the_usage_text() {
        for option in RUN_OPTIONS {
            assert!(USAGE.contains(&format!("--{option} ")), "--{option} is not documented");
        }
    }

    #[test]
    fn every_option_the_sweep_command_takes_is_in_the_usage_text() {
        for option in crate::sweep::OPTIONS {
            assert!(USAGE.contains(&format!("--{option}")), "--{option} is not documented");
        }
    }

    #[test]
    fn the_default_run_tests_rucc_against_gcc_sixteen() {
        let found = specs(&parse("run")).unwrap();
        assert_eq!(found.len(), 2);
        let reference: Vec<&str> =
            found.iter().filter(|s| s.reference).map(|s| s.id.as_str()).collect();
        assert_eq!(reference, ["gcc-16"]);
    }

    #[test]
    fn naming_a_reference_that_is_not_under_test_is_an_error_and_not_a_silent_loss_of_ratios() {
        let error = specs(&parse("run --toolchain rucc --reference gcc-16")).unwrap_err();
        assert!(error.contains("nothing to compare against"), "{error}");
    }

    #[test]
    fn the_reference_can_be_any_of_the_compilers_under_test() {
        let found =
            specs(&parse("run --toolchain gcc-16 --toolchain rucc --reference rucc")).unwrap();
        let reference: Vec<&str> =
            found.iter().filter(|s| s.reference).map(|s| s.id.as_str()).collect();
        assert_eq!(reference, ["rucc"]);
    }

    #[test]
    fn a_flag_goes_to_the_compiler_it_names_and_no_other() {
        let found = specs(&parse(
            "run --toolchain gcc --toolchain rucc --reference gcc --flag rucc=--target=x86_64-windows-gnu",
        ))
        .unwrap();
        let extra: Vec<(&str, &[String])> =
            found.iter().map(|s| (s.id.as_str(), s.extra.as_slice())).collect();
        assert_eq!(
            extra,
            [("gcc", &[][..]), ("rucc", &["--target=x86_64-windows-gnu".to_owned()][..])]
        );
        let error = specs(&parse("run --flag clang=-O1")).unwrap_err();
        assert!(error.contains("not one of the compilers under test"), "{error}");
        let error = specs(&parse("run --flag -O1")).unwrap_err();
        assert!(error.contains("should be ID=FLAG"), "{error}");
    }

    #[test]
    fn every_level_is_built_unless_somebody_asked_for_fewer() {
        assert_eq!(levels(&parse("run")).unwrap(), Level::ALL);
        assert_eq!(
            levels(&parse("run --level O2 --level O0 --level O2")).unwrap(),
            [Level::O0, Level::O2]
        );
        let error = levels(&parse("run --level Ofast")).unwrap_err();
        assert!(error.contains("Ofast is not a level"));
    }

    #[test]
    fn a_facet_that_does_not_exist_says_so_rather_than_running_nothing() {
        let error = options(&parse("run --facet loop-unrolls")).unwrap_err();
        assert!(error.contains("is not a facet"), "{error}");
        let good = options(&parse("run --facet loop-unroll --limit 3")).unwrap();
        assert_eq!(good.facets, [Facet::LoopUnroll]);
        assert_eq!(good.limit_per_facet, Some(3));
    }
}
