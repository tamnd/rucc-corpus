//! Which compilers are under test, and what they can be asked to do.
//!
//! A toolchain is named on the command line as `id=program`, for example `gcc-16=gcc-16` or
//! `rucc=../rucc/target/release/rucc`. The id is what appears in every record and every
//! column of the report, and the program is what actually gets run. Keeping them separate
//! means a report from a laptop and a report from CI line up even though the binary lived in
//! a different place.
//!
//! One toolchain is the reference. Its job is not to be right, because the generator already
//! knows the right answer. Its job is to be the number the others are measured against, and
//! to be asked what it thought about each program, which is what `-fopt-info` is for.

use crate::exec;
use corpus_model::Toolchain;
use std::time::Duration;

/// How long a compiler gets to say what version it is.
const VERSION_TIMEOUT: Duration = Duration::from_secs(20);

/// A toolchain as it was asked for, before it has been found and questioned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spec {
    /// The name it goes by in the report.
    pub id: String,
    /// The program to run.
    pub program: String,
    /// Whether the others are compared against it.
    pub reference: bool,
    /// Flags to add to every compile, such as an include path.
    pub extra: Vec<String>,
}

impl Spec {
    /// Reads a spec written as `id=program`, or as just a program.
    ///
    /// When there is no id the file name is used, so `gcc-16` on the path becomes the id
    /// `gcc-16` and `/usr/bin/cc` becomes `cc`.
    #[must_use]
    pub fn parse(text: &str) -> Self {
        let (id, program) = match text.split_once('=') {
            Some((id, program)) => (id.to_owned(), program.to_owned()),
            None => {
                let name = text.rsplit('/').next().unwrap_or(text);
                (name.to_owned(), text.to_owned())
            }
        };
        Self { id, program: resolve(&program), reference: false, extra: Vec::new() }
    }

    /// Marks this toolchain as the one the others are compared against.
    #[must_use]
    pub fn as_reference(mut self) -> Self {
        self.reference = true;
        self
    }

    /// Whether this toolchain can be asked what it optimized.
    ///
    /// Only GCC understands `-fopt-info`, and asking a compiler for a flag it does not know
    /// turns every compile into a failure, so the question is only put to a toolchain whose
    /// id says it is a GCC. That is a deliberately blunt rule. A compiler that grows the flag
    /// later can be given an id that says so.
    #[must_use]
    pub fn understands_opt_info(&self) -> bool {
        let id = self.id.to_ascii_lowercase();
        id == "gcc" || id.starts_with("gcc-") || id.starts_with("g++")
    }

    /// Runs the program with `--version` and records what it said.
    ///
    /// # Errors
    ///
    /// When the program cannot be run, which is the one failure worth stopping the whole run
    /// for. Every other failure in the harness belongs in the report as a finding, but a
    /// compiler that is not installed would produce thousands of identical findings and hide
    /// everything else.
    pub fn describe(&self) -> Result<Toolchain, String> {
        let outcome = exec::run(&self.program, &["--version"], None, VERSION_TIMEOUT)
            .map_err(|error| format!("could not run {}: {error}", self.program))?;
        if !outcome.ok {
            return Err(format!(
                "{} did not answer --version, status {}",
                self.program, outcome.status
            ));
        }
        let version = outcome
            .stdout
            .lines()
            .chain(outcome.stderr.lines())
            .map(str::trim)
            .find(|line| !line.is_empty())
            .unwrap_or("unknown")
            .to_owned();
        Ok(Toolchain {
            id: self.id.clone(),
            program: self.program.clone(),
            version,
            reference: self.reference,
        })
    }
}

/// Turns a relative program name into an absolute one.
///
/// Every case is compiled with its own directory as the working directory, so a compiler
/// asked for as `../rucc/target/release/rucc` would be looked for one level up from the case
/// rather than one level up from here. A bare name is left alone, because that is a name to be
/// found on the path and the path does not move. A relative name that does not resolve is also
/// left alone, so the error a person gets back names the thing they actually typed.
fn resolve(program: &str) -> String {
    if !program.contains('/') || program.starts_with('/') {
        return program.to_owned();
    }
    std::fs::canonicalize(program)
        .map_or_else(|_| program.to_owned(), |path| path.display().to_string())
}

/// The toolchains to use when nobody said.
///
/// GCC 16 is the reference because compatibility with it is the whole point of rucc, and it
/// is the compiler whose opinion about a program is worth recording. Homebrew installs it as
/// `gcc-16` and the Ubuntu toolchain archive installs it under the same name, so one spelling
/// covers both places the corpus is run.
#[must_use]
pub fn defaults() -> Vec<Spec> {
    vec![Spec::parse("gcc-16").as_reference(), Spec::parse("rucc")]
}

#[cfg(test)]
mod tests {
    use super::{Spec, defaults};

    #[test]
    fn a_spec_with_an_id_keeps_the_id_and_one_without_takes_the_file_name() {
        let named = Spec::parse("reference=/opt/homebrew/bin/gcc-16");
        assert_eq!(named.id, "reference");
        assert_eq!(named.program, "/opt/homebrew/bin/gcc-16");

        let bare = Spec::parse("/usr/bin/cc");
        assert_eq!(bare.id, "cc");
        assert_eq!(bare.program, "/usr/bin/cc");

        let short = Spec::parse("gcc-16");
        assert_eq!(short.id, "gcc-16");
        assert_eq!(short.program, "gcc-16");
    }

    #[test]
    fn a_compiler_named_by_a_relative_path_is_resolved_before_the_working_directory_moves() {
        // Every case is built in a directory of its own, so a relative name that was right
        // here would be wrong there. A bare name and an absolute name are both left alone.
        let relative = Spec::parse("sh=./src/toolchain.rs");
        assert!(relative.program.starts_with('/'), "{}", relative.program);
        assert!(relative.program.ends_with("src/toolchain.rs"));
        assert_eq!(Spec::parse("gcc-16").program, "gcc-16");
        assert_eq!(Spec::parse("/usr/bin/cc").program, "/usr/bin/cc");
        // Something relative that is not there keeps the spelling the person typed, so the
        // error they get back is about the name they used.
        assert_eq!(Spec::parse("./nowhere/rucc").program, "./nowhere/rucc");
    }

    #[test]
    fn only_a_gcc_is_asked_what_it_optimized() {
        assert!(Spec::parse("gcc-16").understands_opt_info());
        assert!(Spec::parse("gcc").understands_opt_info());
        assert!(Spec::parse("gcc-17=/opt/gcc/bin/cc").understands_opt_info());
        assert!(!Spec::parse("rucc").understands_opt_info());
        assert!(!Spec::parse("clang").understands_opt_info());
        // The id decides, not the program, because the id is what a person chose.
        assert!(!Spec::parse("cc=/opt/homebrew/bin/gcc-16").understands_opt_info());
    }

    #[test]
    fn exactly_one_of_the_default_toolchains_is_the_reference() {
        let specs = defaults();
        let references: Vec<&Spec> = specs.iter().filter(|s| s.reference).collect();
        assert_eq!(references.len(), 1);
        assert_eq!(references[0].id, "gcc-16");
    }

    #[test]
    fn a_compiler_that_is_not_installed_is_an_error_and_not_a_silent_pass() {
        let missing = Spec::parse("definitely-not-a-compiler-9999");
        assert!(missing.describe().is_err());
    }

    #[test]
    fn asking_a_real_program_for_its_version_gets_the_first_line_back() {
        let shell = Spec::parse("sh=/bin/sh");
        // Not every shell answers --version, so this only checks the shape of the failure
        // path when it does not, and the shape of the success path when it does.
        match shell.describe() {
            Ok(found) => {
                assert_eq!(found.id, "sh");
                assert!(!found.version.is_empty());
                assert!(!found.version.contains('\n'));
            }
            Err(message) => assert!(message.contains("--version")),
        }
    }
}
