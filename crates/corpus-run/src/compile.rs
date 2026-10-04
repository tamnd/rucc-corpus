//! Building one case with one toolchain at one level, and running what came out.
//!
//! Everything happens in a directory the caller owns, one per case, so a run can be looked at
//! afterwards. That matters more than it sounds: the first question anybody asks about a
//! failing case is what the source actually was and what the compiler actually said, and a
//! harness that deletes both leaves them re-running it by hand to find out.

use crate::counter;
use crate::exec;
use crate::insight;
use crate::object;
use crate::shape;
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

/// What one of the other translation units is written to, before its own name.
///
/// Named after the unit rather than after the case, unlike the copy under `programs/`, because
/// each case has a working directory to itself and a short name is a shorter command line for
/// somebody to paste. Every case in the corpus but a handful has none of these.
pub const UNIT_PREFIX: &str = "unit-";

/// What the compiler is asked to produce.
///
/// With `.exe` on Windows, where a program is found by it.
pub const BINARY: &str = if cfg!(windows) { "case.exe" } else { "case.bin" };

/// Where the compiler is asked to write down what it thought.
///
/// A file of its own rather than the diagnostics stream, because a rejected case is matched
/// against what the compiler said, and thousands of lines about inlining decisions mixed into
/// that would make the match meaningless.
pub const OPINIONS: &str = "case.opt-info";

/// What the executable is linked with when the case loads a module, before the sources.
///
/// On ELF the executable's symbols are only in the dynamic symbol table, where a module loaded
/// later can find them, when the executable is linked with `-rdynamic`. That is what Postgres
/// links the server with. A Mach-O executable exports its globals already, and the bundle is
/// the side that names it. On Windows the executable exports everything and writes an import
/// library for it, which each module is then linked against, the way MinGW builds of Postgres
/// link `postgres.exe` and `libpostgres.a`.
const LOADER: &[&str] = if cfg!(target_os = "macos") {
    &[]
} else if cfg!(windows) {
    &["-Wl,--export-all-symbols", "-Wl,--out-implib,libcase.a"]
} else {
    &["-rdynamic"]
};

/// What the executable is linked with when the case loads a module, after the sources.
///
/// `dlopen` lives in `libdl` on a C library older than glibc 2.34, and in the C library itself
/// on anything newer and on macOS, where naming it still works and changes nothing. Windows
/// loads a module with `LoadLibraryA` from `kernel32`, which every program is linked with.
const LOADER_LIBRARIES: &[&str] =
    if cfg!(any(target_os = "macos", windows)) { &[] } else { &["-ldl"] };

/// What each module is linked with, after its source.
///
/// The import library the executable wrote, on Windows, which is where a module finds the
/// functions and data it imports from the executable. Nothing anywhere else, where a module
/// leaves those references undefined until it is loaded.
const MODULE_LIBRARIES: &[&str] = if cfg!(windows) { &["libcase.a"] } else { &[] };

/// How a module is built, over and above the level and the case's own flags.
///
/// On ELF a module is a shared object of position independent code whose references to the
/// executable are left undefined, which `-shared` allows. On Mach-O it is a bundle, and a
/// bundle has to be linked against the executable that will load it so the linker can check
/// every reference and record where each one is bound. On Windows it is a DLL, and every
/// reference to the executable is resolved against the executable's import library, which
/// [`MODULE_LIBRARIES`] names after the source.
fn module_flags() -> Vec<String> {
    if cfg!(target_os = "macos") {
        vec![
            "-fPIC".to_owned(),
            "-bundle".to_owned(),
            "-bundle_loader".to_owned(),
            BINARY.to_owned(),
        ]
    } else if cfg!(windows) {
        vec!["-shared".to_owned()]
    } else {
        vec!["-fPIC".to_owned(), "-shared".to_owned()]
    }
}

/// Everything one build produced.
#[derive(Debug, Clone)]
pub struct Built {
    /// What the compiler did.
    pub compile: Compile,
    /// What running it did, or a skipped record when it was never built.
    pub execute: Execute,
    /// What the compiler said about its own decisions.
    pub insights: Vec<Insight>,
    /// What each `switch` became, as `function: shape`, when the case has one.
    pub switches: Vec<String>,
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
    std::fs::write(&source, &case.source)
        .map_err(|error| format!("{}: {error}", source.display()))?;
    let mut units: Vec<String> = Vec::with_capacity(case.units.len());
    let mut modules: Vec<(String, String)> = Vec::new();
    for unit in &case.units {
        let name = format!("{UNIT_PREFIX}{}.c", unit.name);
        let path = dir.join(&name);
        std::fs::write(&path, &unit.source)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if unit.is_module() {
            modules.push((name, unit.module_file_name()));
        } else {
            units.push(name);
        }
    }

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
    // The flags the case asks for come before the ones the run asks for, so that a person who
    // adds a flag on the command line can still override what a case wanted.
    args.extend(case.flags.iter().cloned());
    if !modules.is_empty() {
        args.extend(LOADER.iter().map(|flag| (*flag).to_owned()));
    }
    let wants_opinions = spec.understands_opt_info();
    if wants_opinions {
        args.extend(insight::flags(OPINIONS));
    }
    args.extend(spec.extra.iter().cloned());
    // The unit with `main` in it goes first, which is the order somebody reading the command
    // would expect and the order the sources are listed in everywhere else.
    args.push(SOURCE.to_owned());
    args.extend(units);
    if !modules.is_empty() {
        args.extend(LOADER_LIBRARIES.iter().map(|flag| (*flag).to_owned()));
    }

    let mut outcome = exec::run(&spec.program, &args, Some(dir), COMPILE_TIMEOUT)
        .map_err(|error| format!("could not run {}: {error}", spec.program))?;
    let mut produced = outcome.ok && binary.is_file();
    let mut stderr = outcome.stderr.clone();
    let mut opinion_files = vec![opinions.clone()];

    // The modules come after the executable, because on Mach-O a bundle is linked against the
    // executable that will load it, and on Windows against the import library the executable's
    // link wrote, and either has to exist first. Each is a compile of its own, and the case
    // compiled when every one of them did. The time, the diagnostics and the sizes are all of
    // them together, since the program is all of them together.
    let mut artifacts = vec![binary.clone()];
    for (source_name, module_name) in &modules {
        if !produced {
            break;
        }
        let mut module_args: Vec<String> = vec![
            case.dialect.std_flag().to_owned(),
            level.flag().to_owned(),
            "-o".to_owned(),
            module_name.clone(),
        ];
        module_args.extend(case.flags.iter().cloned());
        module_args.extend(module_flags());
        if wants_opinions {
            let said = format!("{source_name}.opt-info");
            module_args.extend(insight::flags(&said));
            opinion_files.push(dir.join(said));
        }
        module_args.extend(spec.extra.iter().cloned());
        module_args.push(source_name.clone());
        module_args.extend(MODULE_LIBRARIES.iter().map(|library| (*library).to_owned()));
        let built = exec::run(&spec.program, &module_args, Some(dir), COMPILE_TIMEOUT)
            .map_err(|error| format!("could not run {}: {error}", spec.program))?;
        let path = dir.join(module_name);
        produced = built.ok && path.is_file();
        if !built.stderr.trim().is_empty() {
            stderr.push_str(&built.stderr);
        }
        outcome.micros += built.micros;
        outcome.peak_bytes = outcome.peak_bytes.max(built.peak_bytes);
        outcome.timed_out |= built.timed_out;
        if !built.ok {
            outcome.status = built.status;
        }
        artifacts.push(path);
    }

    let (bytes, sizes) = measure_all(&artifacts, produced);
    let compile = Compile {
        ok: produced,
        status: if outcome.timed_out { -1 } else { outcome.status },
        micros: outcome.micros,
        diagnostics: trim_diagnostics(&stderr),
        bytes,
        text_bytes: sizes.text,
        data_bytes: sizes.data,
        bss_bytes: sizes.bss,
        peak_bytes: outcome.peak_bytes,
    };

    let insights = if wants_opinions {
        opinion_files
            .iter()
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .flat_map(|text| insight::parse(&text))
            .collect()
    } else {
        Vec::new()
    };

    // After the timed compile and before the run, so that the binary being run is the one the
    // timed compile wrote. `-S` writes somewhere else.
    let switches = if produced && shape::wanted(case) {
        let mut asked = vec![case.dialect.std_flag().to_owned(), level.flag().to_owned()];
        asked.extend(case.flags.iter().cloned());
        asked.extend(spec.extra.iter().cloned());
        shape::shapes(spec, asked, dir)
    } else {
        Vec::new()
    };

    let should_run = produced && matches!(case.expect, Expect::Output(_));
    // With a dot and a slash on the front, because a bare name would be looked for on the path and
    // the path is not where this was just built. Windows reads a relative program from the
    // harness's own directory rather than the one the child starts in, so there it is the whole
    // path.
    let program = if cfg!(windows) {
        std::path::absolute(&binary).unwrap_or_else(|_| binary.clone()).display().to_string()
    } else {
        format!("./{BINARY}")
    };
    let execute = if should_run {
        let ran =
            exec::run_repeatedly::<String>(&program, &[], Some(dir), EXECUTE_TIMEOUT, repeats)
                .map_err(|error| format!("could not run {}: {error}", binary.display()))?;
        // Only for a program that worked. Counting the instructions of a program that crashed
        // would count the ones it managed before it died, which is a number that looks like a
        // measurement and is not one.
        let counted = if ran.ok {
            counter::count_repeatedly::<String>(&program, &[], Some(dir))
        } else {
            Vec::new()
        };
        Execute {
            ok: ran.ok,
            status: if ran.timed_out { -1 } else { ran.status },
            micros: ran.micros,
            repeats: repeats.max(1),
            samples: ran.samples,
            instructions: counted.iter().min().copied(),
            instruction_samples: counted,
            peak_bytes: ran.peak_bytes,
            // A Windows C library writes standard output in text mode, which puts a carriage
            // return before every newline. The expected answers are written once for every
            // platform, and the line ending is the C library's and not the compiler's.
            output: if cfg!(windows) { ran.stdout.replace("\r\n", "\n") } else { ran.stdout },
        }
    } else {
        Execute::skipped()
    };

    Ok(Built { compile, execute, insights, switches })
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
        source: case.size(),
        compile: built.compile,
        execute: built.execute,
        insights: built.insights,
        switches: built.switches,
        // This one was built here, just now. Only the cache sets the flag, on the way out.
        reused: false,
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
        .map(
            |c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' },
        )
        .collect();
    root.join(toolchain).join(level.name()).join(safe)
}

/// Sizes what was produced, when anything was.
///
/// The file size and the image sizes are two different measurements and both are kept. The
/// file is what a build costs on disk, padding and symbol table and all. The image is what the
/// optimizer decided, which is the number a code quality claim can rest on.
fn measure(binary: &Path, produced: bool) -> (u64, object::Sizes) {
    if !produced {
        return (0, object::Sizes::default());
    }
    let Ok(bytes) = std::fs::read(binary) else {
        return (0, object::Sizes::default());
    };
    (bytes.len() as u64, object::sizes(&bytes).unwrap_or_default())
}

/// Sizes everything a build produced together, the executable and every module it loads.
///
/// Summed rather than kept apart, because the program is all of them together and a module
/// that grew is code the case cost as surely as a function in the executable that grew.
fn measure_all(artifacts: &[PathBuf], produced: bool) -> (u64, object::Sizes) {
    artifacts.iter().fold((0, object::Sizes::default()), |(bytes, sum), path| {
        let (more, sizes) = measure(path, produced);
        (
            bytes + more,
            object::Sizes {
                text: sum.text + sizes.text,
                data: sum.data + sizes.data,
                bss: sum.bss + sizes.bss,
            },
        )
    })
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
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Level, Unit};

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
        let dir =
            std::env::temp_dir().join(format!("rucc-corpus-test-{name}-{}", std::process::id()));
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
    fn a_module_is_built_on_its_own_and_calls_back_into_the_executable_that_loads_it() {
        // Built with the system compiler, so this is the harness's link recipe for the host
        // being checked on every machine the tests run on, Linux in CI and macOS on a laptop.
        let dir = scratch("module");
        let case = Case::linked(
            Facet::Bundle,
            Axes::of([("shape", "smoke")]),
            Dialect::C17,
            "#include <dlfcn.h>\nint printf(const char *, ...);\nint base = 40;\nint add_base(int x) { return base + x; }\nint main(void) {\n    void *h = dlopen(\"./m0.so\", RTLD_NOW | RTLD_LOCAL);\n    if (!h) return 1;\n    int (*run)(void) = (int (*)(void))dlsym(h, \"run\");\n    printf(\"%d\\n\", run());\n    return 0;\n}\n",
            vec![Unit::module("m0", "int add_base(int);\nint run(void) { return add_base(2); }\n")],
            Vec::new(),
            Expect::Output("42\n".to_owned()),
        );
        let built = build_and_run(&case, &system_cc(), Level::O2, &dir, 1).unwrap();
        assert!(built.compile.ok, "compile said: {}", built.compile.diagnostics);
        assert!(dir.join("m0.so").is_file());
        assert_eq!(built.execute.output, "42\n");
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
