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

/// A number that was measured, or null when nobody was able to measure it.
///
/// Nought and null are different answers and the schema keeps them apart. A build that used no
/// memory did not happen, so writing nought for a platform that cannot look would turn a gap in
/// the instrument into a claim about the compiler.
fn maybe(value: Option<u64>) -> Json {
    value.map_or(Json::Null, |bytes| Json::int(bytes as i64))
}

/// A whole number read back out of a record.
///
/// Anything that is not a number reads as nought rather than failing the whole record. A field
/// that a newer harness added is missing from an older line, and refusing to read that line at
/// all would throw away evidence over a field nobody was asking about.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn number(value: &Json, key: &str) -> u64 {
    value.get(key).and_then(Json::as_f64).map_or(0, |n| if n < 0.0 { 0 } else { n as u64 })
}

/// A list of whole numbers read back out of a record.
///
/// Missing reads as empty, on the same terms as a missing number reads as nought. A report
/// written before the samples were kept has no list on the line, and the run it describes is
/// still worth reading.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn numbers(value: &Json, key: &str) -> Vec<u64> {
    let Some(items) = value.get(key).and_then(Json::as_array) else {
        return Vec::new();
    };
    items.iter().filter_map(Json::as_f64).map(|n| if n < 0.0 { 0 } else { n as u64 }).collect()
}

/// A signed status read back out of a record.
#[allow(clippy::cast_possible_truncation)]
fn status(value: &Json, key: &str) -> i32 {
    value.get(key).and_then(Json::as_f64).map_or(-1, |n| n as i32)
}

/// A measurement read back out of a record, keeping null and nought apart.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn measured(value: &Json, key: &str) -> Option<u64> {
    value.get(key).and_then(Json::as_f64).map(|n| if n < 0.0 { 0 } else { n as u64 })
}

/// A string read back out of a record.
fn text(value: &Json, key: &str) -> String {
    value.get(key).and_then(Json::as_str).unwrap_or_default().to_owned()
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
    /// The size of the initialized data in the running image, read only and writable together.
    ///
    /// Kept apart from the code because it answers a different question. A compiler that
    /// unrolls a loop by materializing a lookup table and one that folds the loop away have
    /// the same effect on the text column and the opposite effect here.
    pub data_bytes: u64,
    /// The size of the zero filled data in the running image.
    ///
    /// It costs nothing in the file and costs pages when the program runs, which is why it is
    /// neither of the other two.
    pub bss_bytes: u64,
    /// The largest high water mark of any process in the compiler's tree, in bytes.
    ///
    /// `None` rather than nought when nobody could look, which is every platform that is not
    /// Linux. What the number covers and what it can miss is in `corpus_run::memory`.
    pub peak_bytes: Option<u64>,
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
            data_bytes: 0,
            bss_bytes: 0,
            peak_bytes: None,
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
            ("data_bytes", Json::int(self.data_bytes as i64)),
            ("bss_bytes", Json::int(self.bss_bytes as i64)),
            ("peak_bytes", maybe(self.peak_bytes)),
            ("diagnostics", Json::string(self.diagnostics.clone())),
        ])
    }

    /// The compile read back from JSON.
    #[must_use]
    pub fn from_json(value: &Json) -> Self {
        Self {
            ok: value.get("ok").and_then(Json::as_bool).unwrap_or(false),
            status: status(value, "status"),
            micros: number(value, "micros"),
            diagnostics: text(value, "diagnostics"),
            bytes: number(value, "bytes"),
            text_bytes: number(value, "text_bytes"),
            data_bytes: number(value, "data_bytes"),
            bss_bytes: number(value, "bss_bytes"),
            peak_bytes: measured(value, "peak_bytes"),
        }
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
    /// Every repetition's wall time, in microseconds, in the order the repetitions were taken.
    ///
    /// Kept rather than thrown away once the minimum has been taken, because a minimum on its
    /// own cannot be argued with. Two runs of this corpus over byte for byte identical programs
    /// moved the total wall time by nearly nine percent, and a report that prints a one percent
    /// speedup out of samples that spread by nine is not reporting a speedup. The samples are
    /// what lets a reader see that, and they are what a number in the human report can be
    /// traced back to. tamnd/rucc-corpus#8.
    ///
    /// Empty for a run that never happened, and for a run that was read back from a report
    /// written before the samples were kept.
    pub samples: Vec<u64>,
    /// How many instructions the program retired, on a machine that would say.
    ///
    /// The fewest of the measurements, on the same argument as the fastest of the timings, and
    /// in practice they are all the same number. This is the honest version of the question
    /// the wall clock is asked: it is what the program did rather than how long the machine
    /// took to let it do it, and on one machine it reproduces to within a hundredth of a
    /// percent where the clock moves by a factor of four.
    ///
    /// `None` on a machine with no counters, which is every platform that is not Linux and
    /// plenty of Linux ones. See `corpus_run::counter`.
    pub instructions: Option<u64>,
    /// Every instruction count, in the order the measurements were taken.
    ///
    /// Kept for the same reason the timings are. A count that does not reproduce is worth
    /// knowing about, and a number with nothing behind it is worth arguing with.
    pub instruction_samples: Vec<u64>,
    /// The largest high water mark the program reached, in bytes, across the repetitions.
    ///
    /// `None` on the same terms as the compile's, and for the same reasons.
    pub peak_bytes: Option<u64>,
    /// Everything it printed on standard output.
    pub output: String,
}

impl Execute {
    /// A run that never happened, because the compile failed.
    #[must_use]
    pub fn skipped() -> Self {
        Self {
            ok: false,
            status: -1,
            micros: 0,
            repeats: 0,
            samples: Vec::new(),
            instructions: None,
            instruction_samples: Vec::new(),
            peak_bytes: None,
            output: String::new(),
        }
    }

    /// The middle of the repetitions, in microseconds.
    ///
    /// `None` when the samples were not kept, which is every record from a report written
    /// before they were, rather than nought. Nought is a time somebody measured.
    #[must_use]
    pub fn median_micros(&self) -> Option<u64> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_unstable();
        let middle = sorted.len() / 2;
        if sorted.len() % 2 == 1 {
            Some(sorted[middle])
        } else {
            Some(sorted[middle - 1].midpoint(sorted[middle]))
        }
    }

    /// How far the repetitions spread, as a fraction of the fastest one.
    ///
    /// Nought means every repetition took the same time and one means the slowest took twice as
    /// long as the fastest. This is the number a claim about run time has to beat: a difference
    /// between two compilers that is smaller than the spread within one of them is a difference
    /// the machine made rather than one the compiler made.
    ///
    /// `None` when there is fewer than one sample to spread, or when the fastest was nought,
    /// which means the program finished inside the clock's resolution and there is no ratio to
    /// take.
    #[must_use]
    pub fn spread(&self) -> Option<f64> {
        let low = *self.samples.iter().min()?;
        let high = *self.samples.iter().max()?;
        if low == 0 {
            return None;
        }
        Some((high - low) as f64 / low as f64)
    }

    /// The run as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("ok", Json::Bool(self.ok)),
            ("status", Json::int(i64::from(self.status))),
            ("micros", Json::int(self.micros as i64)),
            ("repeats", Json::int(i64::from(self.repeats))),
            ("samples", Json::array(self.samples.iter().map(|&one| Json::int(one as i64)))),
            ("instructions", maybe(self.instructions)),
            (
                "instruction_samples",
                Json::array(self.instruction_samples.iter().map(|&one| Json::int(one as i64))),
            ),
            ("peak_bytes", maybe(self.peak_bytes)),
            ("output", Json::string(self.output.clone())),
        ])
    }

    /// The run read back from JSON.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_json(value: &Json) -> Self {
        Self {
            ok: value.get("ok").and_then(Json::as_bool).unwrap_or(false),
            status: status(value, "status"),
            micros: number(value, "micros"),
            repeats: number(value, "repeats") as u32,
            samples: numbers(value, "samples"),
            instructions: measured(value, "instructions"),
            instruction_samples: numbers(value, "instruction_samples"),
            peak_bytes: measured(value, "peak_bytes"),
            output: text(value, "output"),
        }
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

    /// The insight read back from JSON.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_json(value: &Json) -> Self {
        Self {
            kind: text(value, "kind"),
            pass: text(value, "pass"),
            line: number(value, "line") as u32,
            message: text(value, "message"),
        }
    }
}

/// How much C a case is.
///
/// Every other number on a record is a cost, and a cost with nothing next to it cannot be read.
/// Forty milliseconds is quick for a thousand lines and slow for ten, and a reader who does not
/// know which one they are looking at has been given a number and no way to use it. The corpus is
/// generated, so nobody has the feel for its size that they would have for a project they had
/// checked out, and this is the field that gives them one.
///
/// It is a property of the case and not of the run, so it is the same at every level and for
/// every compiler, which is what lets a report take it off whichever record it meets first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Source {
    /// How many lines of C there are, over every translation unit.
    pub lines: u32,
    /// How many bytes they are.
    pub bytes: u64,
    /// How many translation units there are.
    ///
    /// One for almost every case, and more for the few that exist to be linked together. It is
    /// worth counting rather than assuming, because a report that says how many lines a facet is
    /// without saying how many files they arrived in cannot be read by somebody looking at the
    /// facet whose whole subject is the file boundary.
    pub files: u32,
}

impl Source {
    /// Measures a translation unit.
    ///
    /// A last line with no newline after it is still a line, which is the one place a naive
    /// count of newline characters gets a different answer from every editor a person uses.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn of(text: &str) -> Self {
        if text.is_empty() {
            return Self::default();
        }
        let newlines = text.bytes().filter(|byte| *byte == b'\n').count();
        let lines = if text.ends_with('\n') { newlines } else { newlines + 1 };
        Self { lines: lines as u32, bytes: text.len() as u64, files: 1 }
    }

    /// Two sizes added together, saturating rather than wrapping.
    #[must_use]
    pub const fn plus(self, other: Self) -> Self {
        Self {
            lines: self.lines.saturating_add(other.lines),
            bytes: self.bytes.saturating_add(other.bytes),
            files: self.files.saturating_add(other.files),
        }
    }

    /// Whether there is a measurement here at all.
    ///
    /// A record written before this field existed reads back as nought, and nought lines is not
    /// a program any generator produced, so the two cases are one case. A report treats both as
    /// nothing to say rather than as a corpus of empty programs.
    #[must_use]
    pub const fn measured(self) -> bool {
        self.lines > 0
    }

    /// The size as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("lines", Json::int(i64::from(self.lines))),
            ("bytes", Json::int(self.bytes as i64)),
            ("files", Json::int(i64::from(self.files))),
        ])
    }

    /// The size read back from JSON.
    ///
    /// A record written before anybody counted files has lines and no file count, and the only
    /// number of files a case with lines in it can have is at least one. So a missing count reads
    /// as one rather than as nought, which keeps an old record out of a total that would otherwise
    /// claim the corpus arrived in no files at all.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_json(value: &Json) -> Self {
        let lines = number(value, "lines") as u32;
        let files = number(value, "files") as u32;
        let files = if files == 0 && lines > 0 { 1 } else { files };
        Self { lines, bytes: number(value, "bytes"), files }
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
    /// How much C the case is, copied so a record stands alone.
    pub source: Source,
    /// The compile.
    pub compile: Compile,
    /// The run.
    pub execute: Execute,
    /// What the compiler said about its own optimization decisions, when it was asked.
    pub insights: Vec<Insight>,
    /// Whether this record was read out of the cache rather than measured today.
    ///
    /// An outcome keeps. A timing does not. A record that says the program printed the wrong
    /// answer means the same thing a fortnight later, but the microseconds next to it were
    /// measured on a machine that was doing something else at the time, so a report that quotes
    /// seconds has to say how many of them came out of a file.
    pub reused: bool,
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
            source: case.size(),
            compile: Compile::skipped(),
            execute: Execute::skipped(),
            insights: Vec::new(),
            reused: false,
        }
    }

    /// The key that identifies this record, used to line two runs up against each other.
    #[must_use]
    pub fn key(&self) -> String {
        format!("{}|{}|{}", self.case, self.toolchain, self.level.name())
    }

    /// The record as JSON, which is one line of the JSON Lines file.
    ///
    /// The `reused` field is written only when it is true. A run where nothing came out of the
    /// cache produces the same line it produced before the cache existed, which keeps the
    /// diff between two runs about the compiler rather than about the harness.
    #[must_use]
    pub fn to_json(&self) -> Json {
        let mut fields = vec![
            ("case".to_owned(), Json::string(self.case.clone())),
            ("facet".to_owned(), Json::string(self.facet.name())),
            ("phase".to_owned(), Json::string(self.phase.name())),
            ("dialect".to_owned(), Json::string(self.dialect.name())),
            ("toolchain".to_owned(), Json::string(self.toolchain.clone())),
            ("level".to_owned(), Json::string(self.level.name())),
            ("source".to_owned(), self.source.to_json()),
            ("compile".to_owned(), self.compile.to_json()),
            ("execute".to_owned(), self.execute.to_json()),
            ("insights".to_owned(), Json::array(self.insights.iter().map(Insight::to_json))),
        ];
        if self.reused {
            fields.push(("reused".to_owned(), Json::Bool(true)));
        }
        Json::Object(fields)
    }

    /// The record read back from one line of a JSON Lines file.
    ///
    /// `None` when a name in it is one this build does not know, which is what a record written
    /// by a newer harness looks like. That is a record to ignore rather than to guess at, since
    /// a facet nobody here has heard of cannot be reported on.
    #[must_use]
    pub fn from_json(value: &Json) -> Option<Self> {
        Some(Self {
            case: text(value, "case"),
            facet: Facet::parse(value.get("facet")?.as_str()?)?,
            phase: Phase::parse(value.get("phase")?.as_str()?)?,
            dialect: Dialect::parse(value.get("dialect")?.as_str()?)?,
            toolchain: text(value, "toolchain"),
            level: Level::parse(value.get("level")?.as_str()?)?,
            // Absent rather than missing on a line written before this field existed. That is a
            // record to read and report as unmeasured, not a record to throw away, because the
            // outcome in it is still the outcome and it is the only copy anybody has.
            source: value.get("source").map_or_else(Source::default, Source::from_json),
            compile: Compile::from_json(value.get("compile")?),
            execute: Execute::from_json(value.get("execute")?),
            insights: value
                .get("insights")
                .and_then(Json::as_array)
                .unwrap_or_default()
                .iter()
                .map(Insight::from_json)
                .collect(),
            reused: value.get("reused").and_then(Json::as_bool).unwrap_or(false),
        })
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
    /// It did not compile, and the compiler said plainly that the construct is not built yet.
    ///
    /// Split off from [`Verdict::Rejected`] because the two want different responses. A rejection
    /// is a bug: the compiler read valid C and got it wrong, and somebody has to find out why.
    /// This is a gap: the compiler read valid C, recognised it, and said it has not been taught to
    /// lower it, which is a line on a to-do list that already exists.
    ///
    /// It does not turn the build red, and that is the only concession it gets. It is counted, it
    /// is listed in the human report with the compiler's own words, and the count going up between
    /// two runs is a regression the diff will show. A corpus that hid these would be a corpus that
    /// stopped measuring the thing it was built to measure, and one that failed on them would be a
    /// corpus nobody could run until the compiler was finished.
    Unimplemented,
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
            Self::Unimplemented => "unimplemented",
            Self::Accepted => "accepted",
            Self::Crashed => "crashed",
            Self::Skipped => "skipped",
        }
    }

    /// The verdict with this name, as it is spelled in a record.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        [
            Self::Pass,
            Self::Wrong,
            Self::Rejected,
            Self::Unimplemented,
            Self::Accepted,
            Self::Crashed,
            Self::Skipped,
        ]
        .into_iter()
        .find(|verdict| verdict.name() == name)
    }

    /// Whether this verdict should turn the build red.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Wrong | Self::Rejected | Self::Accepted | Self::Crashed)
    }

    /// Whether this is a gap the compiler admitted to rather than a bug it does not know it has.
    ///
    /// Reported separately and counted, per [`Verdict::Unimplemented`].
    #[must_use]
    pub const fn is_gap(self) -> bool {
        matches!(self, Self::Unimplemented)
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
    use super::{Compile, Execute, Insight, Level, RunRecord, Source, Toolchain, Verdict};
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
            ..Compile::skipped()
        };
        record.execute = Execute {
            ok: true,
            status: 0,
            micros: 900,
            repeats: 5,
            output: "0\n".to_owned(),
            ..Execute::skipped()
        };
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
        assert_eq!(RunRecord::from_json(&json), Some(record));
    }

    #[test]
    fn a_record_read_back_out_of_a_file_is_the_record_that_went_in() {
        let case = Case::new(
            Facet::LoopUnroll,
            Axes::of([("count", "8")]),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output("done\n".to_owned()),
        );
        let mut record = RunRecord::skipped(&case, "rucc", Level::Os);
        record.compile = Compile {
            ok: true,
            status: 0,
            micros: 44_100,
            diagnostics: "warning: something\n".to_owned(),
            bytes: 33_112,
            text_bytes: 2_048,
            data_bytes: 512,
            bss_bytes: 64,
            peak_bytes: Some(91_000_000),
        };
        record.execute = Execute {
            ok: true,
            status: 0,
            micros: 1_200,
            repeats: 3,
            samples: vec![1_400, 1_200, 1_300],
            instructions: Some(112_183),
            instruction_samples: vec![112_183, 112_184],
            peak_bytes: Some(1_800_000),
            output: "done\n".to_owned(),
        };
        record.insights = vec![Insight {
            kind: "missed".to_owned(),
            pass: "vect".to_owned(),
            line: 9,
            message: "not vectorized".to_owned(),
        }];
        let read = RunRecord::from_json(&crate::json::parse(&record.to_json().to_line()).unwrap());
        assert_eq!(read, Some(record));
    }

    #[test]
    fn the_middle_of_the_repetitions_is_the_middle_and_not_the_average() {
        // Four hundred is the mean of these and the middle is three hundred, which is the
        // number that survives one repetition having been interrupted by something else.
        let five = Execute { samples: vec![300, 290, 310, 1_100, 300], ..Execute::skipped() };
        assert_eq!(five.median_micros(), Some(300));
        let four = Execute { samples: vec![300, 290, 310, 1_100], ..Execute::skipped() };
        assert_eq!(four.median_micros(), Some(305));
        assert_eq!(Execute::skipped().median_micros(), None);
    }

    #[test]
    fn the_spread_is_the_distance_from_the_fastest_to_the_slowest() {
        let steady = Execute { samples: vec![200, 200, 200], ..Execute::skipped() };
        assert_eq!(steady.spread(), Some(0.0));
        let moved = Execute { samples: vec![200, 300, 240], ..Execute::skipped() };
        assert_eq!(moved.spread(), Some(0.5));
        // A record from a report written before the samples were kept, which cannot say.
        assert_eq!(Execute::skipped().spread(), None);
        // And a program that finished inside the clock, where there is no ratio to take.
        let instant = Execute { samples: vec![0, 0], ..Execute::skipped() };
        assert_eq!(instant.spread(), None);
    }

    #[test]
    fn a_run_read_back_from_a_report_without_samples_has_none_rather_than_a_nought() {
        let json = crate::json::parse("{\"ok\":true,\"micros\":900,\"repeats\":5}").unwrap();
        let read = Execute::from_json(&json);
        assert_eq!(read.micros, 900);
        assert!(read.samples.is_empty());
        assert_eq!(read.spread(), None);
    }

    #[test]
    fn a_measurement_nobody_could_take_stays_missing_when_it_is_read_back() {
        // Null and nought are different answers and the round trip has to keep them apart,
        // because nought bytes of peak memory would be a claim about the compiler and null is
        // an admission that this platform cannot look.
        let json = Compile { peak_bytes: None, ..Compile::skipped() }.to_json();
        assert_eq!(Compile::from_json(&json).peak_bytes, None);
        let json = Compile { peak_bytes: Some(0), ..Compile::skipped() }.to_json();
        assert_eq!(Compile::from_json(&json).peak_bytes, Some(0));
    }

    #[test]
    fn a_record_only_says_it_was_reused_when_it_was() {
        let case = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        let fresh = RunRecord::skipped(&case, "rucc", Level::O2);
        assert!(fresh.to_json().get("reused").is_none());
        let reused = RunRecord { reused: true, ..fresh };
        assert_eq!(reused.to_json().get("reused"), Some(&crate::Json::Bool(true)));
        assert_eq!(RunRecord::from_json(&reused.to_json()), Some(reused));
    }

    #[test]
    fn a_record_naming_something_this_build_never_heard_of_is_ignored_rather_than_guessed_at() {
        let case = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        let good = RunRecord::skipped(&case, "rucc", Level::O2).to_json();
        assert!(RunRecord::from_json(&good).is_some());
        let mut fields = good.as_object().unwrap().to_vec();
        for field in &mut fields {
            if field.0 == "facet" {
                field.1 = crate::Json::string("a-facet-from-the-future");
            }
        }
        assert_eq!(RunRecord::from_json(&crate::Json::Object(fields)), None);
    }

    #[test]
    fn a_file_with_no_trailing_newline_still_has_its_last_line_counted() {
        assert_eq!(Source::of(""), Source::default());
        assert_eq!(Source::of("int main(void) { return 0; }").lines, 1);
        assert_eq!(Source::of("one\ntwo\n").lines, 2);
        assert_eq!(Source::of("one\ntwo").lines, 2);
        assert_eq!(Source::of("one\n\nthree\n").lines, 3);
        assert_eq!(Source::of("abc\n").bytes, 4);
    }

    #[test]
    fn a_record_carries_the_size_of_the_program_it_is_about() {
        let case = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C17,
            "int main(void) {\n    return 0;\n}\n",
            Expect::Output(String::new()),
        );
        let record = RunRecord::skipped(&case, "rucc", Level::O2);
        assert_eq!(record.source.lines, 3);
        assert!(record.source.measured());
        assert_eq!(RunRecord::from_json(&record.to_json()), Some(record));
    }

    #[test]
    fn a_record_from_before_the_size_was_kept_reads_back_as_unmeasured_rather_than_empty() {
        let case = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        let written = RunRecord::skipped(&case, "rucc", Level::O2).to_json();
        let older: Vec<_> = written
            .as_object()
            .unwrap()
            .iter()
            .filter(|field| field.0 != "source")
            .cloned()
            .collect();
        let read = RunRecord::from_json(&crate::Json::Object(older)).unwrap();
        assert_eq!(read.source, Source::default());
        assert!(!read.source.measured());
        assert_eq!(read.toolchain, "rucc");
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
