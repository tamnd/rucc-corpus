//! The record cache.
//!
//! A full run is fifteen hundred programs built at five levels by two compilers, and on a
//! normal day almost none of that work is new. The generator produced the same sources it
//! produced yesterday, the reference compiler has not moved in months, and the only thing that
//! changed is the compiler under test. Building the reference column again to learn what it
//! already told us is most of the run and none of the answer.
//!
//! So a job whose every input hashes to what it hashed before is not run again. Its record is
//! read out of a file instead. Three rules make that safe to do.
//!
//! **A reused record says so.** Every record carries a `reused` flag and the report says how
//! many of its numbers were not measured today. An outcome keeps. A timing does not.
//!
//! **The key is deliberately over specified.** It covers the source, the expectation, the
//! dialect, the level, the compiler as bytes and as a version string, the flags, the repeat
//! count and the harness version. A key that is missing an ingredient is worse than no cache at
//! all, because it hands back a stale answer confidently and does so exactly when somebody has
//! changed the thing the key forgot. The cost of an ingredient that turns out not to matter is
//! one wasted rebuild, which is the cheaper mistake by a wide margin.
//!
//! **A cache that cannot be read is a miss.** A truncated file, a line from a newer harness, a
//! directory nobody has permission to write: none of them are errors, because a harness that
//! refuses to run when its cache is damaged is worse than one with no cache.
//!
//! The whole cache is one JSON Lines file rather than a file per record. Fifteen thousand
//! records is fifteen thousand files of a few hundred bytes each, which is a directory that
//! costs more to walk than the answers in it are worth. One file is read once at the start of a
//! run and written once at the end.

use corpus_model::{Level, RunRecord, sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// The layout of a cache file.
///
/// A file written by a harness that spelled its keys differently is not a file this one can
/// read, and a stale hit is the one failure mode a cache must not have. So the number is part
/// of every key, and raising it retires every entry at once.
pub const FORMAT: u32 = 1;

/// How many entries a cache file is allowed to keep.
///
/// A run of the whole corpus at every level with two compilers is about fifteen thousand
/// records, so this holds a few runs worth of history and then starts dropping the oldest. The
/// point is that a cache nobody prunes is a file that grows until somebody notices it, and
/// noticing it is not a thing anybody should have to do.
pub const LIMIT: usize = 60_000;

/// Whether a run may read the cache, write it, both, or neither.
///
/// Three settings rather than a flag, because the nightly and a person waiting on an answer
/// want opposite halves of the same cache. The nightly wants every number measured in one
/// sitting and still wants to leave the cache warm for whoever runs next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Reuse {
    /// Read and write. What a person running by hand gets.
    #[default]
    Allow,
    /// Build everything, then keep the results. What the nightly runs.
    Refresh,
    /// Neither read nor write.
    Off,
}

impl Reuse {
    /// Whether a hit may be handed back instead of a build.
    #[must_use]
    pub const fn reads(self) -> bool {
        matches!(self, Self::Allow)
    }

    /// Whether a finished record is kept for next time.
    #[must_use]
    pub const fn writes(self) -> bool {
        matches!(self, Self::Allow | Self::Refresh)
    }
}

/// A compiler as the cache identifies it.
///
/// Both halves are needed. The version string alone would not notice a rebuilt rucc, since a
/// compiler under development prints the same version all week. The bytes alone would make
/// every entry miss on a machine where the same compiler lives at a different path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compiler {
    /// What the compiler said when asked for its version.
    pub version: String,
    /// The hash of the program on disk, or a stand-in when it could not be read.
    pub digest: String,
}

impl Compiler {
    /// Identifies the program at this path.
    ///
    /// A program that cannot be read gets a stand-in built from its name rather than an error.
    /// That covers a compiler behind a wrapper script and a compiler named by a bare word this
    /// process cannot find on the path, and in both cases the version string is still doing its
    /// half of the work.
    #[must_use]
    pub fn of(program: &str, version: &str) -> Self {
        let digest = found(program)
            .as_deref()
            .and_then(digest_of)
            .unwrap_or_else(|| format!("unreadable:{program}"));
        Self { version: version.to_owned(), digest }
    }
}

/// Everything about a job that could change what it produces.
///
/// Every field is here because leaving it out would let a stale record through. The struct is
/// spelled out rather than being a bag of strings so that adding an input to the harness is a
/// compile error here rather than a silently wrong answer six months later.
#[derive(Debug, Clone)]
pub struct Ingredients<'a> {
    /// The case id.
    pub case: &'a str,
    /// The hash of the source that was compiled.
    pub source: &'a str,
    /// What the case was expected to do, as a kind and a text.
    pub expect: &'a str,
    /// Which C the case is written in.
    pub dialect: &'a str,
    /// The level it was built at.
    pub level: Level,
    /// Which compiler, by name.
    pub toolchain: &'a str,
    /// That compiler, by version and by bytes.
    pub compiler: &'a Compiler,
    /// The flags added to every compile of this toolchain.
    pub flags: &'a [String],
    /// How many times the program was run for a timing.
    pub repeats: u32,
    /// Whether the compiler was asked what it optimized.
    pub opinions: bool,
    /// The version of the harness that produced the record.
    pub harness: &'a str,
    /// The operating system and architecture the record was measured on.
    pub host: &'a str,
}

impl Ingredients<'_> {
    /// The key these ingredients hash to.
    ///
    /// Each field is fed in with its own label and a separator, so that two fields cannot run
    /// together into a third. Without that, a case named `a` on toolchain `bc` and a case named
    /// `ab` on toolchain `c` would key the same, and one of them would get the other's answer.
    #[must_use]
    pub fn key(&self) -> String {
        let mut feed_into = String::new();
        let mut feed = |label: &str, value: &str| {
            feed_into.push_str(label);
            feed_into.push('\0');
            feed_into.push_str(value);
            feed_into.push('\n');
        };
        feed("format", &FORMAT.to_string());
        feed("case", self.case);
        feed("source", self.source);
        feed("expect", self.expect);
        feed("dialect", self.dialect);
        feed("level", self.level.name());
        feed("toolchain", self.toolchain);
        feed("version", &self.compiler.version);
        feed("digest", &self.compiler.digest);
        for flag in self.flags {
            feed("flag", flag);
        }
        feed("repeats", &self.repeats.to_string());
        feed("opinions", if self.opinions { "yes" } else { "no" });
        feed("harness", self.harness);
        feed("host", self.host);
        sha256::hex(feed_into.as_bytes())
    }
}

/// One entry as it sits in the file.
struct Entry {
    /// When it was written, in seconds since the epoch.
    stamp: u64,
    /// What was measured.
    record: RunRecord,
}

/// A set of records that were measured earlier and can stand in for measuring them again.
#[derive(Debug)]
pub struct Cache {
    /// Where the file lives, or `None` when the cache is switched off.
    path: Option<PathBuf>,
    /// The entries, by key.
    entries: Mutex<BTreeMap<String, Entry>>,
    /// How many hits were handed out, and how many misses were built.
    counts: Mutex<(usize, usize)>,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry").field("stamp", &self.stamp).finish_non_exhaustive()
    }
}

impl Cache {
    /// Opens the cache file at this path, reading whatever is already in it.
    #[must_use]
    pub fn at(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let entries = read(&path);
        Self { path: Some(path), entries: Mutex::new(entries), counts: Mutex::new((0, 0)) }
    }

    /// A cache that holds nothing and keeps nothing.
    #[must_use]
    pub fn off() -> Self {
        Self { path: None, entries: Mutex::new(BTreeMap::new()), counts: Mutex::new((0, 0)) }
    }

    /// The cache where this machine keeps its records.
    ///
    /// `CORPUS_CACHE` names the directory when it is set, so that CI can put it somewhere it
    /// knows how to save and restore. Otherwise it goes under the usual cache directory, which
    /// is a place a person can delete without losing anything they cannot measure again.
    #[must_use]
    pub fn from_env() -> Self {
        let root = std::env::var_os("CORPUS_CACHE").map(PathBuf::from).or_else(|| {
            std::env::var_os("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
                .map(|cache| cache.join("rucc-corpus"))
        });
        root.map_or_else(Self::off, |root| Self::at(root.join("records.jsonl")))
    }

    /// Whether this cache does anything at all.
    #[must_use]
    pub const fn is_on(&self) -> bool {
        self.path.is_some()
    }

    /// Where the file is.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// How many records were already in it when it was opened.
    #[must_use]
    pub fn held(&self) -> usize {
        self.entries.lock().map_or(0, |held| held.len())
    }

    /// The record for this key, if there is one.
    ///
    /// The record comes back with its `reused` flag set, so that nothing downstream has to
    /// remember to set it and no report can quote a cached timing without saying it did.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<RunRecord> {
        let found = self.entries.lock().ok()?.get(key).map(|entry| entry.record.clone());
        if let Ok(mut counts) = self.counts.lock() {
            if found.is_some() {
                counts.0 += 1;
            } else {
                counts.1 += 1;
            }
        }
        found.map(|record| RunRecord { reused: true, ..record })
    }

    /// Keeps this record for next time.
    ///
    /// A record that came out of the cache is put back as it was found, without today's stamp,
    /// so that an entry nobody has rebuilt in months eventually falls off the end rather than
    /// living forever on the strength of being read.
    pub fn put(&self, key: &str, record: &RunRecord) {
        if !self.is_on() || record.reused {
            return;
        }
        let Ok(mut held) = self.entries.lock() else {
            return;
        };
        held.insert(key.to_owned(), Entry { stamp: now(), record: record.clone() });
    }

    /// How many jobs were answered from the file and how many had to be built.
    #[must_use]
    pub fn tally(&self) -> (usize, usize) {
        self.counts.lock().map_or((0, 0), |counts| *counts)
    }

    /// Writes the cache back out.
    ///
    /// Written to a neighbouring file and renamed into place, so that a run interrupted halfway
    /// through the write leaves the old cache intact rather than a truncated one. Every failure
    /// here is silent on purpose: a corpus run that already produced its report should not fail
    /// because a cache directory was read only.
    pub fn save(&self) {
        let (Some(path), Ok(held)) = (self.path.as_ref(), self.entries.lock()) else {
            return;
        };
        if let Some(parent) = path.parent() {
            if std::fs::create_dir_all(parent).is_err() {
                return;
            }
        }
        let mut kept: Vec<(&String, &Entry)> = held.iter().collect();
        if kept.len() > LIMIT {
            kept.sort_by(|a, b| b.1.stamp.cmp(&a.1.stamp).then_with(|| a.0.cmp(b.0)));
            kept.truncate(LIMIT);
            kept.sort_by(|a, b| a.0.cmp(b.0));
        }
        let mut out = String::new();
        for (key, entry) in kept {
            let line = corpus_model::Json::object([
                ("format", corpus_model::Json::int(i64::from(FORMAT))),
                ("key", corpus_model::Json::string(key.clone())),
                ("stamp", corpus_model::Json::int(entry.stamp as i64)),
                ("record", entry.record.to_json()),
            ]);
            out.push_str(&line.to_line());
            out.push('\n');
        }
        let part = path.with_extension("jsonl.part");
        if std::fs::write(&part, out).is_ok() {
            let _ = std::fs::rename(&part, path);
        }
    }
}

/// Reads whatever is in the file, ignoring anything that does not make sense.
fn read(path: &Path) -> BTreeMap<String, Entry> {
    let mut entries = BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return entries;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(value) = corpus_model::json::parse(line) else {
            continue;
        };
        let format = value.get("format").and_then(corpus_model::Json::as_f64).unwrap_or(0.0);
        if format != f64::from(FORMAT) {
            continue;
        }
        let Some(key) = value.get("key").and_then(corpus_model::Json::as_str) else {
            continue;
        };
        let Some(record) = value.get("record").and_then(RunRecord::from_json) else {
            continue;
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let stamp = value
            .get("stamp")
            .and_then(corpus_model::Json::as_f64)
            .map_or(0, |n| if n < 0.0 { 0 } else { n as u64 });
        entries.insert(key.to_owned(), Entry { stamp, record });
    }
    entries
}

/// The hash of a file, or `None` when it cannot be read.
#[must_use]
pub fn digest_of(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(sha256::hex(&bytes))
}

/// Where a program named on the command line actually is.
///
/// A path is taken as given. A bare word is looked for on the path, the same way the shell that
/// spelled it would look, because the point of hashing a compiler is to notice when it is
/// rebuilt and a name that never resolves would never notice anything.
fn found(program: &str) -> Option<PathBuf> {
    if program.contains('/') {
        let path = PathBuf::from(program);
        return path.is_file().then_some(path);
    }
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths).map(|dir| dir.join(program)).find(|candidate| candidate.is_file())
}

/// The seconds since the epoch, or nought on a machine whose clock is before it.
fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.as_secs())
}

/// The operating system and architecture, which is the granularity a cache is shared at.
///
/// Not the machine name. Two runners of the same image are the same host for this purpose, and
/// that is the whole reason a CI cache is worth restoring. Timings do differ between two such
/// machines, which is what the `reused` flag on every record is for.
#[must_use]
pub fn host() -> String {
    format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
}

#[cfg(test)]
mod tests {
    use super::{Cache, Compiler, Ingredients, Reuse};
    use corpus_model::case::{Axes, Case, Dialect, Expect};
    use corpus_model::facet::Facet;
    use corpus_model::{Level, RunRecord};

    fn compiler() -> Compiler {
        Compiler { version: "gcc-16 (GCC) 16.2.0".to_owned(), digest: "abc123".to_owned() }
    }

    fn ingredients<'a>(compiler: &'a Compiler, flags: &'a [String]) -> Ingredients<'a> {
        Ingredients {
            case: "loop-unroll-0001",
            source: "deadbeef",
            expect: "output:6\n",
            dialect: "c17",
            level: Level::O2,
            toolchain: "gcc-16",
            compiler,
            flags,
            repeats: 5,
            opinions: true,
            harness: "0.1.0",
            host: "linux-x86_64",
        }
    }

    fn record() -> RunRecord {
        let case = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C17,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        RunRecord::skipped(&case, "gcc-16", Level::O2)
    }

    #[test]
    fn changing_any_one_ingredient_changes_the_key() {
        let compiler = compiler();
        let flags: Vec<String> = Vec::new();
        let base = ingredients(&compiler, &flags).key();

        assert_ne!(base, Ingredients { case: "other", ..ingredients(&compiler, &flags) }.key());
        assert_ne!(base, Ingredients { source: "0000", ..ingredients(&compiler, &flags) }.key());
        assert_ne!(
            base,
            Ingredients { expect: "output:7\n", ..ingredients(&compiler, &flags) }.key()
        );
        assert_ne!(base, Ingredients { dialect: "c23", ..ingredients(&compiler, &flags) }.key());
        assert_ne!(base, Ingredients { level: Level::O3, ..ingredients(&compiler, &flags) }.key());
        assert_ne!(base, Ingredients { toolchain: "rucc", ..ingredients(&compiler, &flags) }.key());
        assert_ne!(base, Ingredients { repeats: 3, ..ingredients(&compiler, &flags) }.key());
        assert_ne!(base, Ingredients { opinions: false, ..ingredients(&compiler, &flags) }.key());
        assert_ne!(base, Ingredients { harness: "0.2.0", ..ingredients(&compiler, &flags) }.key());
        assert_ne!(
            base,
            Ingredients { host: "macos-aarch64", ..ingredients(&compiler, &flags) }.key()
        );

        let rebuilt = Compiler { digest: "999".to_owned(), ..compiler.clone() };
        assert_ne!(base, ingredients(&rebuilt, &flags).key());
        let renamed = Compiler { version: "gcc-16 (GCC) 16.3.0".to_owned(), ..compiler.clone() };
        assert_ne!(base, ingredients(&renamed, &flags).key());
        let extra = vec!["-I/usr/local/include".to_owned()];
        assert_ne!(base, ingredients(&compiler, &extra).key());
    }

    #[test]
    fn the_same_ingredients_key_the_same_every_time() {
        let compiler = compiler();
        let flags: Vec<String> = Vec::new();
        assert_eq!(ingredients(&compiler, &flags).key(), ingredients(&compiler, &flags).key());
    }

    #[test]
    fn two_fields_cannot_run_together_into_a_third() {
        // Without a separator between fields, a case named `ab` on toolchain `c` and a case
        // named `a` on toolchain `bc` would hash the same, and one of them would be handed the
        // other's answer.
        let compiler = compiler();
        let flags: Vec<String> = Vec::new();
        let left = Ingredients { case: "ab", toolchain: "c", ..ingredients(&compiler, &flags) };
        let right = Ingredients { case: "a", toolchain: "bc", ..ingredients(&compiler, &flags) };
        assert_ne!(left.key(), right.key());
    }

    #[test]
    fn a_record_that_goes_in_comes_back_out_saying_it_was_reused() {
        let dir = std::env::temp_dir().join(format!("corpus-cache-reused-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cache = Cache::at(dir.join("records.jsonl"));
        cache.put("k", &record());
        cache.save();

        let read = Cache::at(dir.join("records.jsonl"));
        let found = read.get("k").expect("the record that was just written");
        assert!(found.reused, "a record out of the cache has to say so");
        assert_eq!(found.case, record().case);
        assert_eq!(read.get("not-in-there"), None);
        assert_eq!(read.tally(), (1, 1));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_damaged_file_is_a_miss_rather_than_an_error() {
        let dir = std::env::temp_dir().join(format!("corpus-cache-damaged-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("records.jsonl");
        std::fs::write(&path, "{\"format\":1,\"key\":\"k\",\"record\":{\"case\"").unwrap();
        let cache = Cache::at(&path);
        assert_eq!(cache.held(), 0);
        assert_eq!(cache.get("k"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_file_from_another_format_is_ignored_whole() {
        let dir = std::env::temp_dir().join(format!("corpus-cache-format-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("records.jsonl");
        let line = corpus_model::Json::object([
            ("format", corpus_model::Json::int(99)),
            ("key", corpus_model::Json::string("k")),
            ("stamp", corpus_model::Json::int(0)),
            ("record", record().to_json()),
        ]);
        std::fs::write(&path, format!("{}\n", line.to_line())).unwrap();
        assert_eq!(Cache::at(&path).held(), 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_reused_record_is_not_written_back_with_todays_stamp() {
        // Otherwise an entry that nobody has rebuilt in months would stay at the front of the
        // file forever on the strength of being read, and the pruning would never reach it.
        let dir = std::env::temp_dir().join(format!("corpus-cache-stamp-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cache = Cache::at(dir.join("records.jsonl"));
        cache.put("k", &RunRecord { reused: true, ..record() });
        assert_eq!(cache.held(), 0);
        cache.put("k", &record());
        assert_eq!(cache.held(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_cache_that_is_off_holds_nothing_and_writes_nothing() {
        let cache = Cache::off();
        assert!(!cache.is_on());
        assert_eq!(cache.path(), None);
        cache.put("k", &record());
        cache.save();
        assert_eq!(cache.get("k"), None);
    }

    #[test]
    fn the_two_halves_of_the_cache_can_be_switched_on_separately() {
        assert!(Reuse::Allow.reads() && Reuse::Allow.writes());
        assert!(!Reuse::Refresh.reads() && Reuse::Refresh.writes());
        assert!(!Reuse::Off.reads() && !Reuse::Off.writes());
    }
}
