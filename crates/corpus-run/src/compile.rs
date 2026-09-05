//! Building one case with one toolchain at one level, and running what came out.
//!
//! Everything happens in a directory the caller owns, one per case, so a run can be looked at
//! afterwards. That matters more than it sounds: the first question anybody asks about a
//! failing case is what the source actually was and what the compiler actually said, and a
//! harness that deletes both leaves them re-running it by hand to find out.

use crate::exec;
use crate::insight;
use crate::object;
use crate::toolchain::Spec;
use corpus_model::{Case, Compile, Execute, Expect, Insight, Level, RunRecord};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// How long a compiler gets on one case.
///
/// Generous, because a case with twenty thousand call sites at `-O3` is genuinely slow, and a
/// timeout that fires on real work turns the report into noise. A compiler that is stuck is
/// stuck for much longer than this.
pub const COMPILE_TIMEOUT: Duration = Duration::from_secs(120);

/// How long a compiled case gets to run.
///
/// Tight, because every case in this corpus finishes in milliseconds when it is correct. A
/// case that takes ten seconds has been miscompiled into an endless loop, and that is a
/// finding rather than something to wait out.
pub const EXECUTE_TIMEOUT: Duration = Duration::from_secs(10);

/// How many times a case is run to get a time.
pub const REPEATS: u32 = 5;

/// What the case is written to inside its own directory.
pub const SOURCE: &str = "case.c";

/// What the compiler is asked to produce.
pub const BINARY: &str = "case.bin";

/// Where the compiler is asked to write down what it thought.
///
/// A file of its own rather than the diagnostics stream, because a rejected case is matched
/// against what the compiler said, and thousands of lines about inlining decisions mixed into
/// that would make the match meaningless.
pub const OPINIONS: &str = "case.opt-info";

/// Everything one build produced.
#[derive(Debug, Clone)]
pub struct Built {
    /// What the compiler did.
    pub compile: Compile,
    /// What running it did, or a skipped record when it was never built.
    pub execute: Execute,
    /// What the compiler said about its own decisions.
    pub insights: Vec<Insight>,
}

/// Builds a case and runs it, unless it was not supposed to build.
///
/// A case that is expected to be rejected is compiled and then not run, whatever happened.
/// Running an executable that should never have existed proves nothing and, if the compiler
/// under test is the one that was wrong, runs a program nobody reasoned about.
///
/// # Errors
///
/// When the working directory cannot be written to, or the compiler cannot be spawned. A
/// compiler that runs and fails is not an error here, it is a result.
pub fn build_and_run(
    case: &Case,
    spec: &Spec,
    level: Level,
    dir: &Path,
    repeats: u32,
) -> Result<Built, String> {
    std::fs::create_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    let source = dir.join(SOURCE);
    let binary = dir.join(BINARY);
    let opinions = dir.join(OPINIONS);
    std::fs::write(&source, &case.source).map_err(|error| format!("{}: {error}", source.display()))?;

    // Everything on the command line is named relative to the directory the compiler is run
    // in, so the command in the report is one somebody can paste after a cd into that
    // directory and get the same result. It also means a relative work root, which is what
    // the default is, does not get pasted on twice.
    let mut args: Vec<String> = vec![
        case.dialect.std_flag().to_owned(),
        level.flag().to_owned(),
        "-o".to_owned(),
        BINARY.to_owned(),
    ];
    let wants_opinions = spec.understands_opt_info();
    if wants_opinions {
        args.extend(insight::flags(OPINIONS));
    }
    args.extend(spec.extra.iter().cloned());
    args.push(SOURCE.to_owned());

    let outcome = exec::run(&spec.program, &args, Some(dir), COMPILE_TIMEOUT)
        .map_err(|error| format!("could not run {}: {error}", spec.program))?;
    let produced = outcome.ok && binary.is_file();
    let (bytes, text_bytes) = measure(&binary, produced);
    let compile = Compile {
        ok: produced,
        status: if outcome.timed_out { -1 } else { outcome.status },
        micros: outcome.micros,
        diagnostics: trim_diagnostics(&outcome.stderr),
        bytes,
        text_bytes,
    };

    let insights = if wants_opinions {
        std::fs::read_to_string(&opinions).map(|text| insight::parse(&text)).unwrap_or_default()
    } else {
        Vec::new()
    };

    let should_run = produced && matches!(case.expect, Expect::Output(_));
    let execute = if should_run {
        // With a dot and a slash on the front, because a bare name would be looked for on the
        // path and the path is not where this was just built.
        let ran =
            exec::run_repeatedly::<String>(&format!("./{BINARY}"), &[], Some(dir), EXECUTE_TIMEOUT, repeats)
        .map_err(|error| format!("could not run {}: {error}", binary.display()))?;
        Execute {
            ok: ran.ok,
            status: if ran.timed_out { -1 } else { ran.status },
            micros: ran.micros,
            repeats: repeats.max(1),
            output: ran.stdout,
        }
    } else {
        Execute::skipped()
    };

    Ok(Built { compile, execute, insights })
}

/// Turns a build into the record that goes in the JSON Lines file.
#[must_use]
pub fn record(case: &Case, toolchain: &str, level: Level, built: Built) -> RunRecord {
    RunRecord {
        case: case.id.clone(),
        facet: case.facet,
        phase: case.phase(),
        dialect: case.dialect,
        toolchain: toolchain.to_owned(),
        level,
        compile: built.compile,
        execute: built.execute,
        insights: built.insights,
    }
}

/// The directory a case gets to itself.
///
/// The id is already unique and already safe for a path, because it is made of facet names,
/// axis values and hex, but a generator change could put a slash in an axis value one day and
/// the result would be files written outside the run directory.
#[must_use]
pub fn work_dir(root: &Path, case: &Case, toolchain: &str, level: Level) -> PathBuf {
    let safe: String = case
        .id
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    root.join(toolchain).join(level.name()).join(safe)
}

/// Sizes what was produced, when anything was.
fn measure(binary: &Path, produced: bool) -> (u64, u64) {
    if !produced {
        return (0, 0);
    }
    let Ok(bytes) = std::fs::read(binary) else {
        return (0, 0);
    };
    let total = bytes.len() as u64;
    (total, object::text_size(&bytes).unwrap_or(0))
}

/// Keeps diagnostics down to something a report can hold.
///
/// A compiler that goes wrong can print a megabyte about one case, and a JSON Lines file with
/// a megabyte on a line is one no editor will open. The head is kept rather than the tail
/// because the first error is the one that caused the rest.
fn trim_diagnostics(text: &str) -> String {
    const LIMIT: usize = 4000;
    let trimmed = text.trim();
    if trimmed.len() <= LIMIT {
        return trimmed.to_owned();
    }
    let mut cut = LIMIT;
    while cut > 0 && !trimmed.is_char_boundary(cut) {
        cut -= 1;
    }
    format!("{}\n... {} more bytes", &trimmed[..cut], trimmed.len() - cut)
}

#[cfg(test)]
mod tests {
    use super::{build_and_run, record, trim_diagnostics, work_dir};
    use crate::toolchain::Spec;
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Level};

    fn hello() -> Case {
        Case::new(
            Facet::Baseline,
            Axes::of([("program", "hello")]),
            Dialect::C17,
            "int printf(const char *, ...);\nint main(void) {\n    printf(\"42\\n\");\n    return 0;\n}\n",
            Expect::Output("42\n".to_owned()),
        )
    }

    fn broken() -> Case {
        Case::new(
            Facet::Frontend,
            Axes::of([("rejected", "undeclared")]),
            Dialect::C17,
            "int main(void) {\n    return missing_name;\n}\n",
            Expect::Rejected("undeclared".to_owned()),
        )
    }

    /// The system compiler, whatever it is. Every machine that can build this crate has one.
    fn system_cc() -> Spec {
        Spec::parse("cc")
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rucc-corpus-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn a_case_that_works_compiles_runs_and_prints_what_it_was_supposed_to() {
        let dir = scratch("works");
        let case = hello();
        let built = build_and_run(&case, &system_cc(), Level::O2, &dir, 2).unwrap();
        assert!(built.compile.ok, "compile said: {}", built.compile.diagnostics);
        assert!(built.compile.bytes > 0);
        assert!(built.compile.text_bytes > 0, "no code section was found in the executable");
        assert!(built.execute.ok);
        assert_eq!(built.execute.output, "42\n");
        assert_eq!(built.execute.repeats, 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_case_that_should_be_rejected_is_compiled_and_then_not_run() {
        let dir = scratch("rejected");
        let case = broken();
        let built = build_and_run(&case, &system_cc(), Level::O0, &dir, 1).unwrap();
        assert!(!built.compile.ok);
        assert!(!built.compile.diagnostics.is_empty());
        assert_eq!(built.execute.repeats, 0);
        assert!(built.execute.output.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_source_and_the_diagnostics_are_left_behind_for_somebody_to_look_at() {
        let dir = scratch("kept");
        let case = hello();
        build_and_run(&case, &system_cc(), Level::O1, &dir, 1).unwrap();
        let written = std::fs::read_to_string(dir.join("case.c")).unwrap();
        assert_eq!(written, case.source);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_relative_work_directory_builds_the_same_as_an_absolute_one() {
        // The default work root is relative, and the compiler runs with the case directory as
        // its working directory. Handing it a path relative to here rather than to there
        // pasted the root on twice and failed every single case with a missing file.
        let dir = std::path::Path::new("target/corpus-relative-test");
        let _ = std::fs::remove_dir_all(dir);
        let case = hello();
        let built = build_and_run(&case, &system_cc(), Level::O1, dir, 1).unwrap();
        assert!(built.compile.ok, "compile said: {}", built.compile.diagnostics);
        assert_eq!(built.execute.output, "42\n");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_record_carries_the_facet_so_it_can_stand_on_its_own() {
        let dir = scratch("record");
        let case = hello();
        let built = build_and_run(&case, &system_cc(), Level::O2, &dir, 1).unwrap();
        let made = record(&case, "cc", Level::O2, built);
        assert_eq!(made.facet, Facet::Baseline);
        assert_eq!(made.toolchain, "cc");
        assert_eq!(made.level, Level::O2);
        assert_eq!(made.key(), format!("{}|cc|O2", case.id));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn every_case_gets_a_directory_of_its_own_under_the_toolchain_and_the_level() {
        let case = hello();
        let path = work_dir(std::path::Path::new("/run"), &case, "gcc-16", Level::Os);
        assert!(path.starts_with("/run/gcc-16/Os"));
        assert!(path.display().to_string().contains("baseline"));
    }

    #[test]
    fn a_compiler_that_will_not_stop_talking_is_cut_off_rather_than_written_out_in_full() {
        let short = trim_diagnostics("  error: something\n");
        assert_eq!(short, "error: something");
        let long = trim_diagnostics(&"x".repeat(10_000));
        assert!(long.len() < 4200, "the trimmed text is still {} bytes", long.len());
        assert!(long.contains("more bytes"));
    }
}
