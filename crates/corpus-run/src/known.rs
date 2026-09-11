//! The list of what a compiler is already known not to do, and what to do about the rest.
//!
//! A corpus that fails on everything a compiler under development has not got to yet is a
//! corpus nobody runs. A corpus that fails on nothing is a corpus that catches nothing. This is
//! the middle: a file in the repository that names every case the compiler is currently known to
//! get wrong, with the issue behind it, and a run that fails on anything the file does not name.
//!
//! That gives the property the job was missing. A new failure fails, because it is not in the
//! file. A failure that has been fixed also fails, because it is in the file and did not happen,
//! and the message says which line to take out. So the file cannot rot in either direction, and
//! the day a case starts working something says so rather than it moving quietly from one
//! tolerated bucket into another. See tamnd/rucc-corpus#22.
//!
//! # Why the family and not the case
//!
//! A case id ends in a digest of the program text, so it moves whenever the generator changes
//! what the program says. A file keyed on ids would churn on every generator edit and would
//! quietly stop covering the case it was written for. The family is the id without the digest,
//! which is the facet, the axis point and the dialect, and that is the thing a person means when
//! they say a case is not built yet.
//!
//! # Why the level is not in the key
//!
//! A construct a compiler cannot lower cannot be lowered at any level, so listing the same
//! family five times would be five lines that always move together. What the level does change
//! is whether a case that works at `-O0` breaks at `-O2`, and that is a miscompilation rather
//! than a gap: it shows up as a `wrong` verdict on a family that either is not in the file at
//! all, or is in it with a verdict that does not match. Both fail.

use corpus_model::{Facet, Finding, Json, Verdict, family_of};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The name the file has when nobody says otherwise.
pub const FILE: &str = "known-failures.json";

/// One family a compiler is known not to handle, and why.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    /// Which compiler this is about. A gap in one is not a gap in another.
    pub toolchain: String,
    /// The family, which is a case id without its digest.
    pub family: String,
    /// How it comes out. A gap and a miscompilation are not interchangeable.
    pub verdict: Verdict,
    /// The issue that will close it, as `owner/repo#number`, or empty when there is not one yet.
    pub issue: String,
    /// One line saying what the compiler cannot do, in the words of whoever wrote the line.
    pub why: String,
}

impl Entry {
    /// The entry as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("toolchain", Json::string(self.toolchain.clone())),
            ("family", Json::string(self.family.clone())),
            ("verdict", Json::string(self.verdict.name().to_owned())),
            ("issue", Json::string(self.issue.clone())),
            ("why", Json::string(self.why.clone())),
        ])
    }

    /// An entry read back, or `None` when the object is not one.
    ///
    /// A line missing its toolchain, its family or its verdict is dropped rather than defaulted.
    /// Defaulting would silently widen what the file tolerates, which is the one thing this
    /// file must never do on its own.
    #[must_use]
    pub fn from_json(value: &Json) -> Option<Self> {
        let text = |key: &str| value.get(key).and_then(Json::as_str).unwrap_or_default().to_owned();
        let toolchain = text("toolchain");
        let family = text("family");
        let verdict = Verdict::parse(&text("verdict"))?;
        if toolchain.is_empty() || family.is_empty() {
            return None;
        }
        Some(Self { toolchain, family, verdict, issue: text("issue"), why: text("why") })
    }

    /// What to call this entry in a message.
    #[must_use]
    pub fn describe(&self) -> String {
        let issue = if self.issue.is_empty() { "no issue".to_owned() } else { self.issue.clone() };
        format!("{} {} {} ({issue})", self.toolchain, self.family, self.verdict.name())
    }
}

/// What a run exercised, which is the only part of the file it is allowed to have an opinion on.
///
/// Both halves are needed and for the same reason. A run narrowed with `--facet` did not build
/// the rest, and a run whose command line named one compiler says nothing about another. Without
/// the second half the reference job, which runs GCC 16 on its own, would read every rucc line in
/// the file as a case that had started working, and accepting that run would delete them all.
#[derive(Debug, Clone, Default)]
pub struct Coverage {
    /// The compilers the run had.
    pub toolchains: BTreeSet<String>,
    /// The facets the run built.
    pub facets: BTreeSet<Facet>,
}

impl Coverage {
    /// What a run over these compilers and these facets covered.
    #[must_use]
    pub fn new(
        toolchains: impl IntoIterator<Item = String>,
        facets: impl IntoIterator<Item = Facet>,
    ) -> Self {
        Self { toolchains: toolchains.into_iter().collect(), facets: facets.into_iter().collect() }
    }

    /// Whether this run is in a position to say anything about this entry.
    ///
    /// A family whose facet is not one this build knows about is covered, because an entry left
    /// behind by a renamed facet has to be reported rather than ignored forever.
    #[must_use]
    pub fn covers(&self, entry: &Entry) -> bool {
        self.toolchains.contains(&entry.toolchain)
            && facet_of(&entry.family).is_none_or(|facet| self.facets.contains(&facet))
    }
}

/// Everything a compiler is known not to do, as read from the file.
#[derive(Debug, Clone, Default)]
pub struct Known {
    /// The entries, in the order they are written.
    pub entries: Vec<Entry>,
}

impl Known {
    /// Reads the file, or answers an empty list when there is not one.
    ///
    /// A missing file is not an error, because a repository that has never had a failure does
    /// not need one and a run that is narrowed to a facet nobody has trouble with should not
    /// have to make one. A file that is there and cannot be parsed is an error, because the
    /// alternative is a run that tolerates everything and says nothing.
    ///
    /// # Errors
    ///
    /// When the file exists and is not readable, or is not the JSON this writes.
    pub fn read(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        Self::parse(&text).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// Reads the file out of text.
    ///
    /// # Errors
    ///
    /// When the text is not JSON, or is JSON without a `known` array in it.
    pub fn parse(text: &str) -> Result<Self, String> {
        let value = corpus_model::json::parse(text).map_err(|error| error.to_string())?;
        let items = value
            .get("known")
            .and_then(Json::as_array)
            .ok_or_else(|| "there is no `known` array in it".to_owned())?;
        Ok(Self { entries: items.iter().filter_map(Entry::from_json).collect() })
    }

    /// The file as it is written out, sorted so that a diff of it reads.
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut sorted = self.entries.clone();
        sorted.sort();
        sorted.dedup();
        let value = Json::object([
            (
                "note",
                Json::string(
                    "Every family a compiler under test is known not to handle, with the issue that will close it. A run fails on any failure this file does not name, and on any line in this file whose case has started working. Regenerate with `rucc-corpus run --accept`, and read the whole story in the README under \"What the compiler is known not to do\"."
                        .to_owned(),
                ),
            ),
            ("known", Json::array(sorted.iter().map(Entry::to_json))),
        ]);
        format!("{}\n", value.to_pretty())
    }

    /// Whether this family is listed for this compiler with this verdict.
    #[must_use]
    pub fn names(&self, toolchain: &str, family: &str, verdict: Verdict) -> bool {
        self.entries.iter().any(|entry| {
            entry.toolchain == toolchain && entry.family == family && entry.verdict == verdict
        })
    }

    /// What this run says about the file.
    ///
    /// `covered` is what the run was in a position to have an opinion about. Everything outside
    /// it is left alone rather than being reported as fixed, which is the difference between a
    /// narrow run being useful and a narrow run rewriting the file down to the part of it that
    /// was exercised.
    #[must_use]
    pub fn check(&self, findings: &[Finding], covered: &Coverage) -> Check {
        let mut seen: BTreeSet<(String, String, Verdict)> = BTreeSet::new();
        let mut unexpected = Vec::new();
        for finding in findings {
            let family = family_of(&finding.case).to_owned();
            seen.insert((finding.toolchain.clone(), family.clone(), finding.verdict));
            if !self.names(&finding.toolchain, &family, finding.verdict) {
                unexpected.push(finding.clone());
            }
        }
        let fixed = self
            .entries
            .iter()
            .filter(|entry| covered.covers(entry))
            .filter(|entry| {
                !seen.contains(&(entry.toolchain.clone(), entry.family.clone(), entry.verdict))
            })
            .cloned()
            .collect();
        Check { unexpected, fixed }
    }

    /// The file this run would have written, keeping the issue and the reason off the old one.
    ///
    /// The prose on an entry is the part a person wrote and the part a run cannot work out, so
    /// a family that is still failing keeps what it had. A family that is new gets an empty
    /// issue and a reason taken from the compiler's own words, which is a starting point for
    /// somebody to write over rather than an answer.
    ///
    /// Lines the run could not speak for are carried over untouched, for the same reason they
    /// are not reported as fixed. Accepting a narrow run must not be a way to empty the file.
    #[must_use]
    pub fn accepting(&self, findings: &[Finding], covered: &Coverage) -> Self {
        let mut prose: BTreeMap<(&str, &str, Verdict), (&str, &str)> = BTreeMap::new();
        for entry in &self.entries {
            prose.insert(
                (&entry.toolchain, &entry.family, entry.verdict),
                (&entry.issue, &entry.why),
            );
        }
        let mut entries: Vec<Entry> =
            self.entries.iter().filter(|entry| !covered.covers(entry)).cloned().collect();
        for finding in findings {
            let family = family_of(&finding.case);
            let (issue, why) = prose
                .get(&(finding.toolchain.as_str(), family, finding.verdict))
                .copied()
                .unwrap_or(("", finding.summary.as_str()));
            entries.push(Entry {
                toolchain: finding.toolchain.clone(),
                family: family.to_owned(),
                verdict: finding.verdict,
                issue: issue.to_owned(),
                why: why.to_owned(),
            });
        }
        entries.sort();
        entries.dedup();
        Self { entries }
    }
}

/// What a run had to say about the file.
#[derive(Debug, Clone, Default)]
pub struct Check {
    /// Failures the file does not name, which is what a run is for.
    pub unexpected: Vec<Finding>,
    /// Entries whose case did not fail this time, which is a line to take out.
    pub fixed: Vec<Entry>,
}

impl Check {
    /// Whether the run agreed with the file exactly.
    #[must_use]
    pub fn agrees(&self) -> bool {
        self.unexpected.is_empty() && self.fixed.is_empty()
    }
}

/// The facet a family belongs to, which is the part of it before the first dot.
fn facet_of(family: &str) -> Option<Facet> {
    Facet::parse(family.split('.').next().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::{Coverage, Entry, Known, facet_of};
    use corpus_model::{Facet, Finding, Level, Verdict};

    fn finding(case: &str, toolchain: &str, verdict: Verdict) -> Finding {
        Finding {
            case: case.to_owned(),
            facet: Facet::LoopUnroll,
            toolchain: toolchain.to_owned(),
            level: Level::O2,
            verdict,
            summary: "it did not work".to_owned(),
            expected: String::new(),
            actual: String::new(),
        }
    }

    fn entry(family: &str, verdict: Verdict) -> Entry {
        Entry {
            toolchain: "rucc".to_owned(),
            family: family.to_owned(),
            verdict,
            issue: "tamnd/rucc#446".to_owned(),
            why: "block_addr is not lowered yet".to_owned(),
        }
    }

    fn everywhere() -> Coverage {
        Coverage::new(["rucc".to_owned()], Facet::ALL.iter().copied())
    }

    #[test]
    fn a_failure_the_file_names_is_expected_and_one_it_does_not_is_not() {
        let known = Known { entries: vec![entry("loop-unroll.i32.7.known.c17", Verdict::Wrong)] };
        let findings = vec![
            finding("loop-unroll.i32.7.known.c17.abcd1234", "rucc", Verdict::Wrong),
            finding("loop-unroll.i32.8.known.c17.beef5678", "rucc", Verdict::Wrong),
        ];
        let check = known.check(&findings, &everywhere());
        assert_eq!(check.unexpected.len(), 1);
        assert_eq!(check.unexpected[0].case, "loop-unroll.i32.8.known.c17.beef5678");
        assert!(check.fixed.is_empty());
        assert!(!check.agrees());
    }

    #[test]
    fn the_digest_on_the_end_of_a_case_does_not_have_to_match() {
        // Which is the whole reason the file is keyed on the family. The generator rewrote the
        // program, the digest moved, and the line in the file still covers the case it was
        // written for.
        let known = Known { entries: vec![entry("loop-unroll.i32.7.known.c17", Verdict::Wrong)] };
        let findings =
            vec![finding("loop-unroll.i32.7.known.c17.99999999", "rucc", Verdict::Wrong)];
        assert!(known.check(&findings, &everywhere()).agrees());
    }

    #[test]
    fn a_line_whose_case_has_started_working_is_a_line_to_take_out() {
        let known = Known {
            entries: vec![
                entry("loop-unroll.i32.7.known.c17", Verdict::Wrong),
                entry("loop-unroll.i32.8.known.c17", Verdict::Unimplemented),
            ],
        };
        let findings =
            vec![finding("loop-unroll.i32.7.known.c17.abcd1234", "rucc", Verdict::Wrong)];
        let check = known.check(&findings, &everywhere());
        assert!(check.unexpected.is_empty());
        assert_eq!(check.fixed.len(), 1);
        assert_eq!(check.fixed[0].family, "loop-unroll.i32.8.known.c17");
        assert!(!check.agrees(), "a case that started working has to be said out loud");
    }

    #[test]
    fn the_same_family_failing_a_different_way_is_not_the_failure_that_was_signed_off() {
        // A family that was a gap and is now a wrong answer is the compiler having started to
        // lower a construct and got it wrong, which is the most interesting thing this corpus
        // can find. Matching on the family alone would report it as expected.
        let known =
            Known { entries: vec![entry("computed-goto.basic.c17", Verdict::Unimplemented)] };
        let findings = vec![finding("computed-goto.basic.c17.abcd1234", "rucc", Verdict::Wrong)];
        let check = known.check(&findings, &everywhere());
        assert_eq!(check.unexpected.len(), 1);
        assert_eq!(check.fixed.len(), 1);
    }

    #[test]
    fn a_gap_in_one_compiler_is_not_a_gap_in_another() {
        let known = Known { entries: vec![entry("loop-unroll.i32.7.known.c17", Verdict::Wrong)] };
        let findings =
            vec![finding("loop-unroll.i32.7.known.c17.abcd1234", "clang", Verdict::Wrong)];
        let both =
            Coverage::new(["rucc".to_owned(), "clang".to_owned()], Facet::ALL.iter().copied());
        let check = known.check(&findings, &both);
        assert_eq!(check.unexpected.len(), 1);
        assert_eq!(check.fixed.len(), 1);
    }

    #[test]
    fn a_run_that_did_not_build_a_facet_says_nothing_about_it() {
        // A narrow run has to stay useful. Reporting every family the run did not touch as
        // fixed would make `--facet` unusable and would invite somebody to accept the result.
        let known = Known {
            entries: vec![
                entry("loop-unroll.i32.7.known.c17", Verdict::Wrong),
                entry("computed-goto.basic.c17", Verdict::Unimplemented),
            ],
        };
        let findings =
            vec![finding("loop-unroll.i32.7.known.c17.abcd1234", "rucc", Verdict::Wrong)];
        let narrow = Coverage::new(["rucc".to_owned()], [Facet::LoopUnroll]);
        assert!(known.check(&findings, &narrow).agrees());
    }

    #[test]
    fn a_run_without_a_compiler_on_its_command_line_says_nothing_about_that_compiler() {
        // The reference job runs gcc 16 on its own. Every rucc line in the file would look like a
        // case that had started working, and accepting that run would empty the file.
        let known =
            Known { entries: vec![entry("computed-goto.basic.c17", Verdict::Unimplemented)] };
        let reference = Coverage::new(["gcc-16".to_owned()], Facet::ALL.iter().copied());
        assert!(known.check(&[], &reference).agrees());
        assert_eq!(known.accepting(&[], &reference).entries, known.entries);
    }

    #[test]
    fn accepting_a_run_keeps_the_prose_on_the_lines_that_are_still_there() {
        let known =
            Known { entries: vec![entry("computed-goto.basic.c17", Verdict::Unimplemented)] };
        let findings = vec![
            finding("computed-goto.basic.c17.abcd1234", "rucc", Verdict::Unimplemented),
            finding("loop-unroll.i32.8.known.c17.beef5678", "rucc", Verdict::Wrong),
        ];
        let next = known.accepting(&findings, &everywhere());
        assert_eq!(next.entries.len(), 2);
        let kept = next.entries.iter().find(|e| e.family == "computed-goto.basic.c17").unwrap();
        assert_eq!(kept.issue, "tamnd/rucc#446");
        assert_eq!(kept.why, "block_addr is not lowered yet");
        let fresh =
            next.entries.iter().find(|e| e.family == "loop-unroll.i32.8.known.c17").unwrap();
        assert_eq!(fresh.issue, "");
        assert_eq!(fresh.why, "it did not work");
    }

    #[test]
    fn many_cases_in_one_family_are_one_line_and_not_sixty_four() {
        let findings: Vec<Finding> = (0..64)
            .map(|n| {
                finding(&format!("loop-unroll.i32.7.known.c17.{n:08x}"), "rucc", Verdict::Wrong)
            })
            .collect();
        assert_eq!(Known::default().accepting(&findings, &everywhere()).entries.len(), 1);
    }

    #[test]
    fn the_file_reads_back_as_what_was_written() {
        let known = Known {
            entries: vec![
                entry("computed-goto.basic.c17", Verdict::Unimplemented),
                entry("loop-unroll.i32.7.known.c17", Verdict::Wrong),
            ],
        };
        let read = Known::parse(&known.to_text()).unwrap();
        assert_eq!(read.entries, known.entries);
    }

    #[test]
    fn a_file_that_is_not_the_file_is_an_error_and_not_an_empty_list() {
        // An empty list here would turn a typo into a run that tolerates everything, and it
        // would do it on exactly the run where somebody had just edited the file.
        assert!(Known::parse("{}").is_err());
        assert!(Known::parse("not json at all").is_err());
        assert!(Known::parse(r#"{"known": []}"#).is_ok());
    }

    #[test]
    fn a_line_missing_the_part_that_says_what_it_covers_is_dropped() {
        let text = r#"{"known": [
            {"toolchain": "rucc", "family": "a.b.c17", "verdict": "wrong"},
            {"toolchain": "rucc", "verdict": "wrong"},
            {"family": "a.b.c17", "verdict": "wrong"},
            {"toolchain": "rucc", "family": "a.b.c17", "verdict": "nonsense"}
        ]}"#;
        assert_eq!(Known::parse(text).unwrap().entries.len(), 1);
    }

    #[test]
    fn a_family_names_the_facet_it_came_from() {
        assert_eq!(facet_of("loop-unroll.i32.7.known.c17"), Some(Facet::LoopUnroll));
        assert_eq!(facet_of("not-a-facet.whatever"), None);
        assert_eq!(facet_of(""), None);
    }
}
