//! The constant sweep, which is experiment 60 of section 42.5 in rucc's optimizer measurement
//! plan and tamnd/rucc#2970.
//!
//! Every number rucc's optimizer decides by is a row of section 40.12 of its cost model, and
//! `rucc --print-params` lists them with their values. The sweep moves each row to half and to
//! double its value, one row at a time, and measures what moved. Most rows change nothing on
//! most programs, so a whole run of the corpus for every move would spend a week measuring
//! nothing. Every case is first compiled to assembly with the row moved, and only the cases
//! whose assembly changed are built and run, three ways: by rucc as it is, by rucc with the row
//! moved, and by the reference. A case whose assembly did not change is known to be the same.
//!
//! Each move writes one line to `sweep.jsonl` when it finishes, so a sweep that is stopped
//! starts again at the first move it has no line for. The lines are kept only while the
//! compiler and the corpus are the ones that wrote them.

use crate::args::Args;
use corpus_model::json::{self, Json};
use corpus_model::sha256::{self, Sha256};
use corpus_model::{Case, Level, Manifest, RunRecord, Verdict};
use corpus_run::cache::Reuse;
use corpus_run::compile::COMPILE_TIMEOUT;
use corpus_run::toolchain::Spec;
use corpus_run::{Plan, exec};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Everything `sweep` will accept.
pub(crate) const OPTIONS: &[&str] = &[
    "rucc",
    "reference",
    "level",
    "row",
    "facet",
    "limit",
    "exclude-tag",
    "out",
    "work",
    "jobs",
    "repeats",
    "no-cache",
    "quiet",
];

/// The id the compiler as it is goes under in the run.
const BASE: &str = "rucc";

/// The id the compiler with the row moved goes under.
const MOVED: &str = "rucc-moved";

/// How far an end has to move the instructions or the text, as a fraction of what the cases it
/// changed had before, to count as moving them at all.
///
/// Instructions retired are counted rather than timed and barely move from run to run, so this
/// is about what is worth a sentence in section 40.14 rather than about noise.
const FLAT: f64 = 0.001;

/// One row of `--print-params`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Row {
    /// The name `--param` takes.
    pub(crate) name: String,
    /// What it is set to.
    pub(crate) value: u64,
}

/// The rows a `--print-params` listing names, in the order it names them.
///
/// Each line is `name = value` with anything after the value left alone, which is where rucc
/// says what the row is.
///
/// # Errors
///
/// When a line is not a name and a number.
pub(crate) fn rows(listing: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for line in listing.lines().filter(|line| !line.trim().is_empty()) {
        let parsed = line.split_once(" = ").and_then(|(name, rest)| {
            let value = rest.split_whitespace().next()?.parse().ok()?;
            Some(Row { name: name.trim().to_owned(), value })
        });
        rows.push(parsed.ok_or_else(|| format!("`{line}` is not a row of --print-params"))?);
    }
    Ok(rows)
}

/// Which end of a row a move is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum End {
    /// Half the value, rounded down.
    Half,
    /// Twice the value, or one for a row at nought.
    Double,
}

impl End {
    /// The word the reports use.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Half => "half",
            Self::Double => "double",
        }
    }

    /// The value this end moves a row to.
    ///
    /// Twice nought is nought, so a row at nought is moved to one instead, which is the
    /// smallest move there is and the one that turns a row that is off on.
    pub(crate) const fn of(self, value: u64) -> u64 {
        match self {
            Self::Half => value / 2,
            Self::Double if value == 0 => 1,
            Self::Double => value.saturating_mul(2),
        }
    }
}

/// One row at one end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Move {
    /// The row.
    pub(crate) row: String,
    /// What it is set to by default.
    pub(crate) default: u64,
    /// Which end.
    pub(crate) end: End,
    /// What it is moved to.
    pub(crate) value: u64,
}

impl Move {
    /// The flag that moves it.
    pub(crate) fn flag(&self) -> String {
        format!("--param={}={}", self.row, self.value)
    }
}

/// Both ends of every row, leaving out an end that is the value the row already has, which is
/// half of nought.
pub(crate) fn moves(rows: &[Row]) -> Vec<Move> {
    let mut moves = Vec::new();
    for row in rows {
        for end in [End::Half, End::Double] {
            let value = end.of(row.value);
            if value != row.value {
                moves.push(Move { row: row.name.clone(), default: row.value, end, value });
            }
        }
    }
    moves
}

/// What one move did.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Moved {
    /// The cases whose assembly changed.
    pub(crate) changed: usize,
    /// Of those, the ones that ran right for both compilers and were counted, which are the ones
    /// the instruction totals are over.
    pub(crate) counted: usize,
    /// Instructions retired by the compiler as it is, by the moved one and by the reference.
    pub(crate) instructions: [u64; 3],
    /// The text of everything both compilers built, as it is and moved.
    pub(crate) text: [u64; 2],
    /// The compile time of everything both compilers built, as it is and moved.
    pub(crate) compile_micros: [u64; 2],
    /// The instructions the moved compiler saved or spent on each facet, moved less as it is.
    pub(crate) facets: BTreeMap<String, i64>,
    /// The cases the moved compiler got wrong and the compiler as it is did not.
    pub(crate) broke: Vec<String>,
}

impl Moved {
    /// The change in instructions, as a fraction of what the changed cases retired before.
    pub(crate) fn instruction_change(&self) -> Option<f64> {
        change(self.instructions[0], self.instructions[1])
    }

    /// The change in text, as a fraction of what the changed cases had before.
    pub(crate) fn text_change(&self) -> Option<f64> {
        change(self.text[0], self.text[1])
    }

    /// The change in compile time, as a fraction.
    pub(crate) fn compile_change(&self) -> Option<f64> {
        change(self.compile_micros[0], self.compile_micros[1])
    }

    /// The facets that moved most, most first.
    pub(crate) fn top_facets(&self, how_many: usize) -> Vec<(&str, i64)> {
        let mut facets: Vec<(&str, i64)> = self
            .facets
            .iter()
            .filter(|(_, delta)| **delta != 0)
            .map(|(f, d)| (f.as_str(), *d))
            .collect();
        facets.sort_by(|a, b| b.1.unsigned_abs().cmp(&a.1.unsigned_abs()).then(a.0.cmp(b.0)));
        facets.truncate(how_many);
        facets
    }
}

/// The change from one total to another, as a fraction of the first.
#[allow(clippy::cast_precision_loss)]
fn change(before: u64, after: u64) -> Option<f64> {
    (before > 0).then(|| (after as f64 - before as f64) / before as f64)
}

/// What the sweep makes of a row from its two ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Call {
    /// Neither end moves the instructions or the text past [`FLAT`].
    Flat,
    /// An end moves them, and every end that saves on one spends on the other.
    Tuning,
    /// An end saves on both, or saves on one and leaves the other where it was, so the value the
    /// row has is beaten by one the sweep tried.
    Wrong,
}

impl Call {
    /// The word the reports use.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Tuning => "worth tuning",
            Self::Wrong => "wrong",
        }
    }
}

/// The call on a row, from what each of its ends did.
pub(crate) fn call(ends: &[&Moved]) -> Call {
    let mut moved = false;
    for end in ends {
        let instructions = end.instruction_change().unwrap_or(0.0);
        let text = end.text_change().unwrap_or(0.0);
        let (fewer, more) = (instructions < -FLAT, instructions > FLAT);
        let (smaller, larger) = (text < -FLAT, text > FLAT);
        if (fewer && !larger) || (smaller && !more) {
            return Call::Wrong;
        }
        moved |= fewer || more || smaller || larger;
    }
    if moved { Call::Tuning } else { Call::Flat }
}

/// One line of `sweep.jsonl`.
#[derive(Debug, Clone)]
struct Line {
    the_move: Move,
    moved: Moved,
}

impl Line {
    fn to_json(&self, stamp: &Stamp) -> Json {
        let m = &self.moved;
        let numbers = |values: &[u64]| {
            Json::array(
                values.iter().map(|value| Json::int(i64::try_from(*value).unwrap_or(i64::MAX))),
            )
        };
        Json::object([
            ("row", Json::string(&self.the_move.row)),
            ("default", Json::int(i64::try_from(self.the_move.default).unwrap_or(i64::MAX))),
            ("end", Json::string(self.the_move.end.name())),
            ("value", Json::int(i64::try_from(self.the_move.value).unwrap_or(i64::MAX))),
            ("changed", Json::int(i64::try_from(m.changed).unwrap_or(i64::MAX))),
            ("counted", Json::int(i64::try_from(m.counted).unwrap_or(i64::MAX))),
            ("instructions", numbers(&m.instructions)),
            ("text", numbers(&m.text)),
            ("compile_micros", numbers(&m.compile_micros)),
            (
                "facets",
                Json::Object(
                    m.facets
                        .iter()
                        .map(|(facet, delta)| (facet.clone(), Json::int(*delta)))
                        .collect(),
                ),
            ),
            ("broke", Json::array(m.broke.iter().map(Json::string))),
            ("compiler", Json::string(&stamp.compiler)),
            ("corpus", Json::string(&stamp.corpus)),
        ])
    }

    fn from_json(value: &Json, stamp: &Stamp) -> Option<Self> {
        let text = |key: &str| value.get(key)?.as_str().map(str::to_owned);
        if text("compiler")? != stamp.compiler || text("corpus")? != stamp.corpus {
            return None;
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let number = |item: &Json| item.as_f64().map(|n| n as u64);
        let numbers = |key: &str| -> Option<Vec<u64>> {
            value.get(key)?.as_array()?.iter().map(number).collect()
        };
        let end = match text("end")?.as_str() {
            "half" => End::Half,
            "double" => End::Double,
            _ => return None,
        };
        let instructions = numbers("instructions")?;
        let text_bytes = numbers("text")?;
        let compile_micros = numbers("compile_micros")?;
        #[allow(clippy::cast_possible_truncation)]
        let facets = value
            .get("facets")?
            .as_object()?
            .iter()
            .map(|(facet, delta)| Some((facet.clone(), delta.as_f64()? as i64)))
            .collect::<Option<_>>()?;
        Some(Self {
            the_move: Move {
                row: text("row")?,
                default: number(value.get("default")?)?,
                end,
                value: number(value.get("value")?)?,
            },
            moved: Moved {
                changed: usize::try_from(number(value.get("changed")?)?).ok()?,
                counted: usize::try_from(number(value.get("counted")?)?).ok()?,
                instructions: instructions.try_into().ok()?,
                text: text_bytes.try_into().ok()?,
                compile_micros: compile_micros.try_into().ok()?,
                facets,
                broke: value
                    .get("broke")?
                    .as_array()?
                    .iter()
                    .map(|case| case.as_str().map(str::to_owned))
                    .collect::<Option<_>>()?,
            },
        })
    }
}

/// What a line was measured against, so a line from another compiler or corpus is not kept.
#[derive(Debug, Clone)]
struct Stamp {
    compiler: String,
    corpus: String,
}

/// Runs the sweep.
pub(crate) fn command(args: &Args) -> Result<ExitCode, String> {
    args.only(OPTIONS)?;
    let rucc = Spec::parse(&format!("{BASE}={}", args.value_or("rucc", "rucc")));
    let reference = Spec::parse(args.value_or("reference", "gcc-16")).as_reference();
    let level = match args.value("level") {
        None => Level::O2,
        Some(name) => Level::parse(name).ok_or_else(|| format!("{name} is not a level"))?,
    };
    let out = PathBuf::from(args.value_or("out", "reports/sweep"));
    let work = PathBuf::from(args.value_or("work", "target/sweep-work"));
    let jobs = args
        .number("jobs")?
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(4, std::num::NonZero::get));
    let repeats = args.number("repeats")?.unwrap_or(1);
    let quiet = args.flag("quiet");

    let corpus = corpus_gen::generate(&crate::options(args)?)?;
    let excluded = args.values("exclude-tag");
    let cases: Vec<&Case> = corpus
        .cases
        .iter()
        .filter(|case| !case.expect_is_rejection())
        .filter(|case| !excluded.iter().any(|tag| case.has_tag(tag)))
        .collect();

    let listing = exec::run(&rucc.program, &["--print-params"], None, COMPILE_TIMEOUT)
        .map_err(|error| format!("could not run {}: {error}", rucc.program))?;
    if !listing.ok {
        return Err(format!("{} --print-params failed: {}", rucc.program, listing.stderr.trim()));
    }
    let mut rows = rows(&listing.stdout)?;
    let asked = args.values("row");
    if !asked.is_empty() {
        for name in asked {
            if !rows.iter().any(|row| &row.name == name) {
                return Err(format!("{name} is not a row of {} --print-params", rucc.program));
            }
        }
        rows.retain(|row| asked.contains(&row.name));
    }
    let stamp = Stamp { compiler: rucc.describe()?.version, corpus: corpus.digest() };

    std::fs::create_dir_all(&out).map_err(|error| format!("{}: {error}", out.display()))?;
    let journal = out.join("sweep.jsonl");
    let mut lines = read_journal(&journal, &stamp);
    let mut file = std::fs::File::create(&journal)
        .map_err(|error| format!("{}: {error}", journal.display()))?;
    for line in &lines {
        writeln!(file, "{}", line.to_json(&stamp).to_line())
            .map_err(|error| format!("{}: {error}", journal.display()))?;
    }

    let sources = work.join("src");
    write_sources(&sources, &cases)?;
    let compile = Compile { program: &rucc.program, level, root: &sources, jobs };
    if !quiet {
        eprintln!("compiling {} cases to assembly as rucc is", cases.len());
    }
    let baseline = compile.all(&cases, None);

    let all = moves(&rows);
    for (at, the_move) in all.iter().enumerate() {
        let done = lines.iter().any(|line| line.the_move == *the_move);
        if done {
            continue;
        }
        let flag = the_move.flag();
        let after = compile.all(&cases, Some(&flag));
        let changed: Vec<Case> = cases
            .iter()
            .zip(baseline.iter().zip(&after))
            .filter(|(_, (before, after))| before != after)
            .map(|(case, _)| (*case).clone())
            .collect();
        let moved = if changed.is_empty() {
            Moved::default()
        } else {
            let mut plan = Plan::new(work.join("run"));
            let mut moved_spec = Spec::parse(&format!("{MOVED}={}", rucc.program));
            moved_spec.extra.push(flag.clone());
            plan.specs = vec![reference.clone(), rucc.clone(), moved_spec];
            plan.levels = vec![level];
            plan.jobs = jobs.max(1);
            plan.repeats =
                u32::try_from(repeats).map_err(|_| "--repeats is too large".to_owned())?;
            plan.reuse = if args.flag("no-cache") { Reuse::Off } else { Reuse::Allow };
            let manifest = Manifest::new(changed)?;
            let run = corpus_run::execute(&manifest, &plan, &|_, _, _| {})?;
            measure(&manifest, &run, &reference.id, level)
        };
        if !quiet {
            eprintln!(
                "{}/{} {} at {}: {} cases changed",
                at + 1,
                all.len(),
                the_move.row,
                the_move.value,
                moved.changed
            );
        }
        let line = Line { the_move: the_move.clone(), moved };
        writeln!(file, "{}", line.to_json(&stamp).to_line())
            .map_err(|error| format!("{}: {error}", journal.display()))?;
        lines.push(line);
    }

    let report = Report {
        lines: &lines,
        rows: &rows,
        stamp: &stamp,
        level,
        reference: &reference.id,
        cases: cases.len(),
    };
    write(&out.join("sweep.json"), &report.to_json().to_pretty())?;
    write(&out.join("index.md"), &report.to_markdown())?;
    println!("wrote the sweep of {} rows to {}", rows.len(), out.display());
    let broke = lines.iter().filter(|line| !line.moved.broke.is_empty()).count();
    if broke > 0 {
        println!("{broke} moves built a case that gave the wrong answer, listed in the report");
    }
    Ok(ExitCode::SUCCESS)
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|error| format!("{}: {error}", path.display()))
}

/// The lines a sweep that was stopped already wrote, when they are about this compiler and this
/// corpus.
fn read_journal(path: &Path, stamp: &Stamp) -> Vec<Line> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    // A line cut short by a sweep that was killed in the middle of writing it is the last one,
    // and it is dropped with the rest of what does not parse.
    text.lines()
        .filter_map(|line| json::parse(line).ok())
        .filter_map(|value| Line::from_json(&value, stamp))
        .collect()
}

/// Writes each case's sources into a directory of its own, named after the case.
fn write_sources(root: &Path, cases: &[&Case]) -> Result<(), String> {
    for case in cases {
        let dir = root.join(&case.id);
        std::fs::create_dir_all(&dir).map_err(|error| format!("{}: {error}", dir.display()))?;
        let mut files = vec![(corpus_run::compile::SOURCE.to_owned(), &case.source)];
        for unit in &case.units {
            files.push((
                format!("{}{}.c", corpus_run::compile::UNIT_PREFIX, unit.name),
                &unit.source,
            ));
        }
        for (name, source) in files {
            let path = dir.join(name);
            std::fs::write(&path, source)
                .map_err(|error| format!("{}: {error}", path.display()))?;
        }
    }
    Ok(())
}

/// Compiling every case to assembly.
struct Compile<'a> {
    program: &'a str,
    level: Level,
    root: &'a Path,
    jobs: usize,
}

impl Compile<'_> {
    /// A digest of the assembly of each case, in the order the cases are given, with the flag
    /// added when there is one.
    fn all(&self, cases: &[&Case], flag: Option<&str>) -> Vec<String> {
        let next = AtomicUsize::new(0);
        let digests: Mutex<Vec<String>> = Mutex::new(vec![String::new(); cases.len()]);
        std::thread::scope(|scope| {
            for _ in 0..self.jobs.clamp(1, cases.len().max(1)) {
                scope.spawn(|| {
                    loop {
                        let at = next.fetch_add(1, Ordering::Relaxed);
                        let Some(case) = cases.get(at) else {
                            return;
                        };
                        let digest = self.one(case, flag);
                        if let Ok(mut held) = digests.lock() {
                            held[at] = digest;
                        }
                    }
                });
            }
        });
        digests.into_inner().unwrap_or_default()
    }

    /// A digest of the assembly of one case.
    ///
    /// Each translation unit is compiled on its own, a module the same as one that is linked,
    /// since all this asks is whether the flag changed what any of them became. A compile that
    /// fails is part of the digest, so a flag that makes a case fail to build counts as a change.
    fn one(&self, case: &Case, flag: Option<&str>) -> String {
        let dir = self.root.join(&case.id);
        let mut names = vec![corpus_run::compile::SOURCE.to_owned()];
        names.extend(
            case.units
                .iter()
                .map(|unit| format!("{}{}.c", corpus_run::compile::UNIT_PREFIX, unit.name)),
        );
        let mut digest = Sha256::default();
        for name in names {
            let mut args: Vec<String> =
                vec![case.dialect.std_flag().to_owned(), self.level.flag().to_owned()];
            args.extend(case.flags.iter().cloned());
            args.extend(flag.map(str::to_owned));
            args.extend(["-S".to_owned(), "-o".to_owned(), "-".to_owned(), name]);
            match exec::run(self.program, &args, Some(&dir), COMPILE_TIMEOUT) {
                Ok(outcome) if outcome.ok => digest.update(outcome.stdout.as_bytes()),
                Ok(outcome) => digest.update(format!("failed {}", outcome.status).as_bytes()),
                Err(error) => digest.update(format!("could not run: {error}").as_bytes()),
            }
        }
        sha256::hex(&digest.finish())
    }
}

/// What the changed cases did, three ways.
fn measure(manifest: &Manifest, run: &corpus_run::Run, reference: &str, level: Level) -> Moved {
    let mut moved = Moved { changed: manifest.cases.len(), ..Moved::default() };
    for case in &manifest.cases {
        let find = |toolchain: &str| run.record(&case.id, toolchain, level);
        let (Some(base), Some(after)) = (find(BASE), find(MOVED)) else {
            continue;
        };
        let (was, is) = (run.verdict(base), run.verdict(after));
        if is.is_failure() && !was.is_failure() {
            moved.broke.push(case.id.clone());
        }
        if base.compile.ok && after.compile.ok {
            moved.text[0] += base.compile.text_bytes;
            moved.text[1] += after.compile.text_bytes;
            moved.compile_micros[0] += base.compile.micros;
            moved.compile_micros[1] += after.compile.micros;
        }
        let counted = |record: &RunRecord, verdict: Verdict| {
            if verdict == Verdict::Pass { record.execute.instructions } else { None }
        };
        let theirs = find(reference).and_then(|record| counted(record, run.verdict(record)));
        let (Some(before), Some(now), Some(theirs)) =
            (counted(base, was), counted(after, is), theirs)
        else {
            continue;
        };
        moved.counted += 1;
        moved.instructions[0] += before;
        moved.instructions[1] += now;
        moved.instructions[2] += theirs;
        let delta =
            i64::try_from(now).unwrap_or(i64::MAX) - i64::try_from(before).unwrap_or(i64::MAX);
        *moved.facets.entry(case.facet.name().to_owned()).or_default() += delta;
    }
    moved
}

/// The two reports a sweep writes.
struct Report<'a> {
    lines: &'a [Line],
    rows: &'a [Row],
    stamp: &'a Stamp,
    level: Level,
    reference: &'a str,
    cases: usize,
}

impl Report<'_> {
    /// The ends of a row that were measured.
    fn ends(&self, row: &str) -> Vec<&Line> {
        let mut ends: Vec<&Line> =
            self.lines.iter().filter(|line| line.the_move.row == row).collect();
        ends.sort_by_key(|line| line.the_move.end);
        ends
    }

    fn to_json(&self) -> Json {
        let rows = self.rows.iter().map(|row| {
            let ends = self.ends(&row.name);
            let moved: Vec<&Moved> = ends.iter().map(|line| &line.moved).collect();
            Json::object([
                ("row", Json::string(&row.name)),
                ("default", Json::int(i64::try_from(row.value).unwrap_or(i64::MAX))),
                ("call", Json::string(call(&moved).name())),
                ("ends", Json::array(ends.iter().map(|line| line.to_json(self.stamp)))),
            ])
        });
        Json::object([
            ("compiler", Json::string(&self.stamp.compiler)),
            ("corpus", Json::string(&self.stamp.corpus)),
            ("level", Json::string(self.level.name())),
            ("reference", Json::string(self.reference)),
            ("cases", Json::int(i64::try_from(self.cases).unwrap_or(i64::MAX))),
            ("rows", Json::array(rows)),
        ])
    }

    fn to_markdown(&self) -> String {
        let mut out = String::new();
        let calls: Vec<(&Row, Call)> = self
            .rows
            .iter()
            .map(|row| {
                let ends = self.ends(&row.name);
                (row, call(&ends.iter().map(|line| &line.moved).collect::<Vec<_>>()))
            })
            .collect();
        let count = |wanted: Call| calls.iter().filter(|(_, call)| *call == wanted).count();
        let _ = writeln!(out, "# The constant sweep\n");
        let _ = writeln!(
            out,
            "Every row of `rucc --print-params` at half and at double its value, one row at a time, over {} cases of the corpus at `-{}`. This is experiment 60 of section 42.5 of rucc's optimizer measurement plan, tamnd/rucc#2970. The compiler is `{}` and the corpus digest is `{}`.\n",
            self.cases,
            self.level.name(),
            self.stamp.compiler,
            self.stamp.corpus
        );
        let _ = writeln!(
            out,
            "Of {} rows the sweep calls {} flat, {} worth tuning and {} wrong. A row is flat when neither end moves the instructions retired or the text by more than {:.1}% of what the cases it changed had before. It is wrong when an end saves on one of the two and does not spend on the other, so a value the sweep tried beats the one the row has. Every other row that moves something is worth tuning.\n",
            self.rows.len(),
            count(Call::Flat),
            count(Call::Tuning),
            count(Call::Wrong),
            FLAT * 100.0
        );
        let _ = writeln!(
            out,
            "Only the cases whose assembly changed are built and run, so each change below is over those cases and not over the whole corpus. Instructions are over the cases that ran right for rucc both ways and for {}, and the last column is rucc's instructions over {}'s on those cases, as it is and then moved. Compile time is wall time on a shared machine and is the noisiest of the four.\n",
            self.reference, self.reference
        );
        let _ = writeln!(
            out,
            "| row | default | end | value | cases changed | instructions | text | compile time | against {} | instructions by facet | call |",
            self.reference
        );
        let _ = writeln!(out, "|---|---:|---|---:|---:|---:|---:|---:|---|---|---|");
        for (row, row_call) in &calls {
            for line in self.ends(&row.name) {
                let m = &line.moved;
                let facets: Vec<String> = m
                    .top_facets(3)
                    .iter()
                    .map(|(facet, delta)| format!("{facet} {delta:+}"))
                    .collect();
                let against = match ratio(m.instructions[0], m.instructions[2]) {
                    Some(before) => format!(
                        "{before:.3} to {:.3}",
                        ratio(m.instructions[1], m.instructions[2]).unwrap_or(before)
                    ),
                    None => String::new(),
                };
                let _ = writeln!(
                    out,
                    "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                    row.name,
                    row.value,
                    line.the_move.end.name(),
                    line.the_move.value,
                    m.changed,
                    percent(m.instruction_change()),
                    percent(m.text_change()),
                    percent(m.compile_change()),
                    against,
                    facets.join(", "),
                    row_call.name()
                );
            }
        }
        let broke: Vec<&Line> =
            self.lines.iter().filter(|line| !line.moved.broke.is_empty()).collect();
        if !broke.is_empty() {
            let _ = writeln!(out, "\n## Moves that broke a case\n");
            let _ = writeln!(
                out,
                "A row moved to a value it takes should never change what a program prints. Each of these is a bug in the pass that reads the row, whatever the row's call above.\n"
            );
            for line in broke {
                let _ = writeln!(
                    out,
                    "- `{}` at {}: {}",
                    line.the_move.row,
                    line.the_move.value,
                    line.moved
                        .broke
                        .iter()
                        .map(|case| format!("`{case}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        out
    }
}

/// One total over another.
#[allow(clippy::cast_precision_loss)]
fn ratio(mine: u64, theirs: u64) -> Option<f64> {
    (mine > 0 && theirs > 0).then(|| mine as f64 / theirs as f64)
}

/// A change as a signed percentage, or nothing when there was nothing to change.
fn percent(change: Option<f64>) -> String {
    change.map_or_else(String::new, |change| format!("{:+.2}%", change * 100.0))
}

#[cfg(test)]
mod tests {
    use super::{Call, End, Moved, Row, call, moves, rows};

    #[test]
    fn the_listing_is_read_as_names_and_values_and_the_rest_of_the_line_is_left() {
        let listing = "inline-insns-single = 70  (section 33.5)\nmax-unroll-times = 8\n";
        assert_eq!(
            rows(listing).unwrap(),
            [
                Row { name: "inline-insns-single".to_owned(), value: 70 },
                Row { name: "max-unroll-times".to_owned(), value: 8 },
            ]
        );
        assert!(rows("inline-insns-single = many").is_err());
        assert!(rows("not a row at all").is_err());
    }

    #[test]
    fn each_row_moves_to_half_and_double_and_an_end_that_is_where_it_was_is_left_out() {
        let found = moves(&[
            Row { name: "a".to_owned(), value: 9 },
            Row { name: "off".to_owned(), value: 0 },
            Row { name: "one".to_owned(), value: 1 },
        ]);
        let found: Vec<(&str, End, u64)> =
            found.iter().map(|m| (m.row.as_str(), m.end, m.value)).collect();
        assert_eq!(
            found,
            [
                ("a", End::Half, 4),
                ("a", End::Double, 18),
                ("off", End::Double, 1),
                ("one", End::Half, 0),
                ("one", End::Double, 2),
            ]
        );
    }

    fn end(instructions: [u64; 2], text: [u64; 2]) -> Moved {
        Moved {
            changed: 1,
            counted: 1,
            instructions: [instructions[0], instructions[1], 1000],
            text,
            ..Moved::default()
        }
    }

    #[test]
    fn a_row_is_flat_wrong_or_worth_tuning_by_what_its_ends_did() {
        let same = end([1000, 1000], [100, 100]);
        let noise = end([100_000, 100_050], [100, 100]);
        assert_eq!(call(&[&same, &noise]), Call::Flat);
        assert_eq!(call(&[]), Call::Flat);

        let trade = end([1000, 900], [100, 120]);
        let worse = end([1000, 1100], [100, 100]);
        assert_eq!(call(&[&trade, &worse]), Call::Tuning);

        let better = end([1000, 900], [100, 100]);
        assert_eq!(call(&[&worse, &better]), Call::Wrong);
        let smaller = end([1000, 1000], [100, 90]);
        assert_eq!(call(&[&smaller]), Call::Wrong);
    }

    #[test]
    fn the_facets_that_moved_most_come_first_whichever_way_they_moved() {
        let mut moved = Moved::default();
        moved.facets.insert("a".to_owned(), 5);
        moved.facets.insert("b".to_owned(), -50);
        moved.facets.insert("c".to_owned(), 0);
        moved.facets.insert("d".to_owned(), 20);
        assert_eq!(moved.top_facets(2), [("b", -50), ("d", 20)]);
    }
}
