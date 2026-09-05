//! The command line front end to the corpus.
//!
//! Four things it does. `gen` writes the C out, so the corpus in the repository is readable C
//! that anybody can open, compile by hand and argue with, rather than a program that promises
//! to produce some. `run` builds every program with every compiler at every level, checks the
//! answers against what the generator computed, and writes the reports. `diff` compares two
//! reports, which is how progress across the phases of the plan gets tracked. `list` says what
//! is in the corpus.
//!
//! Everything is deterministic. The same commit produces the same programs with the same
//! digest, so a report that disagrees with another report is a report about different code.

mod args;
mod diff;
mod emit;

use args::Args;
use corpus_gen::Options;
use corpus_model::{Facet, Level};
use corpus_report::terminal::Watcher;
use corpus_run::{Plan, toolchain::Spec};
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

Options for gen:
  --out DIR            where to write, default programs
  --manifest FILE      where to write the manifest, default programs/manifest.json
  --facet NAME         only this facet, may be repeated
  --limit N            at most this many cases per facet
  --clean              remove what is already there first

Options for run:
  --toolchain ID=PROG  a compiler to test, may be repeated, default gcc-16 and rucc
  --reference ID       the compiler the others are measured against, default gcc-16
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
        "wrote {written} programs across {} facets to {}",
        corpus.by_facet().len(),
        out.display()
    );
    println!("corpus digest {}", corpus.digest());
    Ok(ExitCode::SUCCESS)
}

/// Builds and runs the corpus, and writes the reports.
fn run(args: &Args) -> Result<ExitCode, String> {
    args.only(&[
        "toolchain",
        "reference",
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
    ])?;

    let corpus = corpus_gen::generate(&options(args)?)?;
    let work = PathBuf::from(args.value_or("work", "target/corpus-work"));
    let mut plan = Plan::new(&work);
    plan.specs = specs(args)?;
    plan.levels = levels(args)?;
    plan.keep_passes = args.flag("keep");
    plan.exclude_tags = args.values("exclude-tag").to_vec();
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

    println!("wrote the reports to {}", reports.display());
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
        return Ok(ExitCode::FAILURE);
    }
    println!("every case produced the answer the generator computed");
    Ok(ExitCode::SUCCESS)
}

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
    use super::{levels, options, specs};
    use crate::args::Args;
    use corpus_model::{Facet, Level};

    fn parse(line: &str) -> Args {
        Args::parse(line.split_whitespace().map(str::to_owned)).unwrap()
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
