//! What a run produced.
//!
//! One record per case per toolchain per optimization level. That is the grain the whole
//! system works at, and everything in the report is an aggregate over records of that shape.
//! Records are appended to a JSON Lines file as they are produced, so a run that is killed
//! halfway still leaves usable evidence behind.

use crate::Json;
use crate::case::{Case, Dialect};
use crate::facet::{Facet, Phase};

/// An optimization level.
///
/// The five that both compilers under test understand. `-Ofast` is deliberately absent: it
/// changes the meaning of floating point arithmetic, so a case compiled with it is not the
/// same program and comparing its output against the reference proves nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// No optimization. The reference for what the program means.
    O0,
    /// The cheap transformations.
    O1,
    /// The default target for code quality.
    O2,
    /// Everything, including the transformations that trade size for speed.
    O3,
    /// Optimize for size.
    Os,
}

impl Level {
    /// Every level, cheapest first.
    pub const ALL: &'static [Self] = &[Self::O0, Self::O1, Self::O2, Self::O3, Self::Os];

    /// The flag to pass.
    #[must_use]
    pub const fn flag(self) -> &'static str {
        match self {
            Self::O0 => "-O0",
            Self::O1 => "-O1",
            Self::O2 => "-O2",
            Self::O3 => "-O3",
            Self::Os => "-Os",
        }
    }

    /// The name used in a record, which is the flag without the dash.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::O0 => "O0",
            Self::O1 => "O1",
            Self::O2 => "O2",
            Self::O3 => "O3",
            Self::Os => "Os",
        }
    }

    /// The level with this name, spelled either way.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        let name = name.strip_prefix('-').unwrap_or(name);
        Self::ALL.iter().copied().find(|level| level.name() == name)
    }

    /// Whether this level is the one code quality is judged at.
    ///
    /// Spec 16 sets the code quality target against `gcc -O2`, so `-O2` is the level whose
    /// numbers go in the headline table and the others are context.
    #[must_use]
    pub const fn is_headline(self) -> bool {
        matches!(self, Self::O2)
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Which compiler produced a record.
///
/// The version string is captured from the compiler itself rather than assumed, because a
/// record that says gcc without saying which gcc is a record nobody can reproduce.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Toolchain {
    /// The short name used as a key, such as `rucc` or `gcc-16`.
    pub id: String,
    /// The program that was run.
    pub program: String,
    /// The first line of what it printed for `--version`.
    pub version: String,
    /// Whether this toolchain is the reference the others are compared against.
    pub reference: bool,
}

impl Toolchain {
    /// The toolchain as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("id", Json::string(self.id.clone())),
            ("program", Json::string(self.program.clone())),
            ("version", Json::string(self.version.clone())),
            ("reference", Json::Bool(self.reference)),
        ])
    }
}

/// What happened when the compiler was asked to build a case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compile {
    /// Whether it produced an executable.
    pub ok: bool,
    /// The exit status, or -1 when the process was killed by a timeout or a signal.
    pub status: i32,
    /// How long it took, in microseconds.
    pub micros: u64,
    /// What it printed on standard error, trimmed.
    pub diagnostics: String,
    /// The size of the executable in bytes, when there was one.
    pub bytes: u64,
    /// The size of the text section in bytes, when it could be read.
    ///
    /// This is the code quality number. Total file size includes the runtime, the symbol
    /// table and the padding the linker chose, none of which the optimizer decided, so a
    /// comparison on file size measures the linker as much as the compiler.
    pub text_bytes: u64,
}

impl Compile {
    /// A compile that never started, because something before it failed.
    #[must_use]
    pub fn skipped() -> Self {
        Self {
            ok: false,
            status: -1,
            micros: 0,
            diagnostics: String::new(),
            bytes: 0,
            text_bytes: 0,
        }
    }

    /// The compile as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("ok", Json::Bool(self.ok)),
            ("status", Json::int(i64::from(self.status))),
            ("micros", Json::int(self.micros as i64)),
            ("bytes", Json::int(self.bytes as i64)),
            ("text_bytes", Json::int(self.text_bytes as i64)),
            ("diagnostics", Json::string(self.diagnostics.clone())),
        ])
    }
}

/// What happened when the executable was run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execute {
    /// Whether it ran to completion and exited zero.
    pub ok: bool,
    /// The exit status, or -1 when it was killed.
    pub status: i32,
    /// The fastest of the repetitions, in microseconds.
    ///
    /// The fastest rather than the mean. On a shared machine the slow measurements are the
    /// ones that got interrupted, and the interruptions are noise from somebody else's work.
    /// The floor is the closest thing to the number the code alone would produce.
    pub micros: u64,
    /// How many times it was run to get that number.
    pub repeats: u32,
    /// Everything it printed on standard output.
    pub output: String,
}

impl Execute {
    /// A run that never happened, because the compile failed.
    #[must_use]
    pub fn skipped() -> Self {
        Self { ok: false, status: -1, micros: 0, repeats: 0, output: String::new() }
    }

    /// The run as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("ok", Json::Bool(self.ok)),
            ("status", Json::int(i64::from(self.status))),
            ("micros", Json::int(self.micros as i64)),
            ("repeats", Json::int(i64::from(self.repeats))),
            ("output", Json::string(self.output.clone())),
        ])
    }
}

/// One thing the reference compiler said about a case.
///
/// GCC will report what it optimized, what it wanted to optimize and could not, and why, if
/// you ask it with `-fopt-info`. That is a description of the transformations a mature
/// compiler thought were available in a program we wrote, which is the most useful reference
/// data the corpus collects. It is not a pass or fail signal. It is a list of leads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Insight {
    /// `optimized`, `missed` or `note`, as GCC classified it.
    pub kind: String,
    /// The pass that said it, when the line named one.
    pub pass: String,
    /// The source line it pointed at, or zero.
    pub line: u32,
    /// What it said.
    pub message: String,
}

impl Insight {
    /// The insight as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("kind", Json::string(self.kind.clone())),
            ("pass", Json::string(self.pass.clone())),
            ("line", Json::int(i64::from(self.line))),
            ("message", Json::string(self.message.clone())),
        ])
    }
}

/// One case, one toolchain, one level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRecord {
    /// The case.
    pub case: String,
    /// What the case is about, copied so a record stands alone.
    pub facet: Facet,
    /// Which phase that facet belongs to, copied for the same reason.
    pub phase: Phase,
    /// Which C the case is written in.
    pub dialect: Dialect,
    /// Which compiler.
    pub toolchain: String,
    /// Which level.
    pub level: Level,
    /// The compile.
    pub compile: Compile,
    /// The run.
    pub execute: Execute,
    /// What the compiler said about its own optimization decisions, when it was asked.
    pub insights: Vec<Insight>,
}

impl RunRecord {
    /// A record for a case that was not run at all.
    #[must_use]
    pub fn skipped(case: &Case, toolchain: &str, level: Level) -> Self {
        Self {
            case: case.id.clone(),
            facet: case.facet,
            phase: case.phase(),
            dialect: case.dialect,
            toolchain: toolchain.to_owned(),
            level,
            compile: Compile::skipped(),
            execute: Execute::skipped(),
            insights: Vec::new(),
        }
    }

    /// The key that identifies this record, used to line two runs up against each other.
    #[must_use]
    pub fn key(&self) -> String {
        format!("{}|{}|{}", self.case, self.toolchain, self.level.name())
    }

    /// The record as JSON, which is one line of the JSON Lines file.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("case", Json::string(self.case.clone())),
            ("facet", Json::string(self.facet.name())),
            ("phase", Json::string(self.phase.name())),
            ("dialect", Json::string(self.dialect.name())),
            ("toolchain", Json::string(self.toolchain.clone())),
            ("level", Json::string(self.level.name())),
            ("compile", self.compile.to_json()),
            ("execute", self.execute.to_json()),
            ("insights", Json::array(self.insights.iter().map(Insight::to_json))),
        ])
    }
}

/// How a case came out against the reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verdict {
    /// It did what it was supposed to do.
    Pass,
    /// It compiled and ran but produced the wrong answer.
    ///
    /// This is the only verdict that is unambiguously a bug in the compiler under test, and
    /// it is the one the exit status of the harness is decided by.
    Wrong,
    /// It did not compile, and it should have.
    Rejected,
    /// It compiled and should not have.
    Accepted,
    /// The compiler failed in a way that is not a diagnostic, such as a crash or a timeout.
    Crashed,
    /// It was not run, because a tag excluded it or an earlier step failed.
    Skipped,
}

impl Verdict {
    /// The name stored in a record.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Wrong => "wrong",
            Self::Rejected => "rejected",
            Self::Accepted => "accepted",
            Self::Crashed => "crashed",
            Self::Skipped => "skipped",
        }
    }

    /// Whether this verdict should turn the build red.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Wrong | Self::Rejected | Self::Accepted | Self::Crashed)
    }
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Something worth a person's attention.
///
/// A finding is what the report is for. It has a verdict, enough context to reproduce it, and
/// a single sentence saying what went wrong. Findings are what the SARIF output carries and
/// what the human report leads with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The case it is about.
    pub case: String,
    /// What that case is about.
    pub facet: Facet,
    /// The compiler that produced it.
    pub toolchain: String,
    /// The level it showed up at.
    pub level: Level,
    /// How it came out.
    pub verdict: Verdict,
    /// One sentence saying what went wrong.
    pub summary: String,
    /// What the reference produced.
    pub expected: String,
    /// What this toolchain produced.
    pub actual: String,
}

impl Finding {
    /// The finding as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("case", Json::string(self.case.clone())),
            ("facet", Json::string(self.facet.name())),
            ("toolchain", Json::string(self.toolchain.clone())),
            ("level", Json::string(self.level.name())),
            ("verdict", Json::string(self.verdict.name())),
            ("summary", Json::string(self.summary.clone())),
            ("expected", Json::string(self.expected.clone())),
            ("actual", Json::string(self.actual.clone())),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::{Compile, Execute, Insight, Level, RunRecord, Toolchain, Verdict};
    use crate::case::{Axes, Case, Dialect, Expect};
    use crate::facet::{Facet, Phase};

    #[test]
    fn a_level_parses_with_or_without_the_dash() {
        assert_eq!(Level::parse("-O2"), Some(Level::O2));
        assert_eq!(Level::parse("O2"), Some(Level::O2));
        assert_eq!(Level::parse("Os"), Some(Level::Os));
        assert_eq!(Level::parse("Ofast"), None);
        assert_eq!(Level::parse("O4"), None);
    }

    #[test]
    fn exactly_one_level_is_the_one_code_quality_is_judged_at() {
        let headline: Vec<_> = Level::ALL.iter().filter(|l| l.is_headline()).collect();
        assert_eq!(headline, [&Level::O2]);
    }

    #[test]
    fn a_skipped_record_carries_the_facet_and_phase_of_its_case() {
        let case = Case::new(
            Facet::LoopUnroll,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        let record = RunRecord::skipped(&case, "rucc", Level::O2);
        assert_eq!(record.facet, Facet::LoopUnroll);
        assert_eq!(record.phase, Phase::Loops);
        assert!(!record.compile.ok);
        assert!(!record.execute.ok);
    }

    #[test]
    fn a_record_key_tells_two_toolchains_and_two_levels_apart() {
        let case = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        let a = RunRecord::skipped(&case, "rucc", Level::O2).key();
        let b = RunRecord::skipped(&case, "gcc-16", Level::O2).key();
        let c = RunRecord::skipped(&case, "rucc", Level::O0).key();
        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn only_the_verdicts_that_mean_a_bug_turn_the_build_red() {
        assert!(!Verdict::Pass.is_failure());
        assert!(!Verdict::Skipped.is_failure());
        for bad in [Verdict::Wrong, Verdict::Rejected, Verdict::Accepted, Verdict::Crashed] {
            assert!(bad.is_failure(), "{bad}");
        }
    }

    #[test]
    fn every_part_of_a_record_survives_being_written_as_json() {
        let case = Case::new(
            Facet::Inline,
            Axes::of([("shape", "call")]),
            Dialect::C23,
            "int main(void) { return 0; }",
            Expect::Output("0\n".to_owned()),
        );
        let mut record = RunRecord::skipped(&case, "gcc-16", Level::O3);
        record.compile = Compile {
            ok: true,
            status: 0,
            micros: 12_345,
            diagnostics: String::new(),
            bytes: 16_384,
            text_bytes: 1_234,
        };
        record.execute =
            Execute { ok: true, status: 0, micros: 900, repeats: 5, output: "0\n".to_owned() };
        record.insights = vec![Insight {
            kind: "optimized".to_owned(),
            pass: "inline".to_owned(),
            line: 4,
            message: "inlined helper into main".to_owned(),
        }];
        let json = record.to_json();
        let line = json.to_line();
        assert_eq!(crate::json::parse(&line).unwrap(), json);
        assert!(!line.contains('\n'), "a JSON Lines record must be one line");
    }

    #[test]
    fn a_toolchain_says_which_one_it_was() {
        let gcc = Toolchain {
            id: "gcc-16".to_owned(),
            program: "/opt/homebrew/bin/gcc-16".to_owned(),
            version: "gcc-16 (Homebrew GCC 16.1.0) 16.1.0".to_owned(),
            reference: true,
        };
        assert_eq!(gcc.to_json().get("reference"), Some(&crate::Json::Bool(true)));
    }
}
