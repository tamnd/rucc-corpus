//! The list of rucc passes allowed to fire on nothing, and the check that holds the rest to it.
//!
//! Section 43.4 of rucc's `spec/optimizer/42-measurement.md` makes it an exit criterion of M4: a
//! pass that fires zero times over a corpus written for the transformations it makes is deleted
//! or explained. The explanation goes here, one line per pass and level with the reason and the
//! issue, and a run of the whole corpus fails on a quiet pass the file does not name. It also
//! fails on a line whose pass fired after all, or no longer runs, so the file says what is true
//! today rather than what was true the day somebody wrote it. That is the same bargain
//! `known-failures.json` makes, and tamnd/rucc#2967 is where it was asked for.
//!
//! Only a run of the whole corpus is checked. A pass that is quiet on one facet is quiet because
//! the facet has nothing for it, and that says nothing about the pass.

use crate::firing::Build;
use corpus_model::{Json, Level};
use std::path::Path;

/// The name the file has when nobody says otherwise.
pub const FILE: &str = "quiet-passes.json";

/// One pass a compiler runs at one level and is allowed to fire on nothing there.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    /// Which compiler.
    pub toolchain: String,
    /// Which level. A pass with nothing to do at `-O1` may have plenty at `-O3`.
    pub level: Level,
    /// The pass, by the name its `-f` flag spells.
    pub pass: String,
    /// The issue about it, as `owner/repo#number`, or empty when there is not one yet.
    pub issue: String,
    /// Why it fires on nothing, in the words of whoever wrote the line.
    pub why: String,
}

impl Entry {
    /// The entry as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("toolchain", Json::string(self.toolchain.clone())),
            ("level", Json::string(self.level.name())),
            ("pass", Json::string(self.pass.clone())),
            ("issue", Json::string(self.issue.clone())),
            ("why", Json::string(self.why.clone())),
        ])
    }

    /// The entry read back, or nothing when its level is not one this build knows.
    #[must_use]
    pub fn from_json(value: &Json) -> Option<Self> {
        let text = |key: &str| value.get(key).and_then(Json::as_str).unwrap_or_default().to_owned();
        Some(Self {
            toolchain: text("toolchain"),
            level: Level::parse(value.get("level")?.as_str()?)?,
            pass: text("pass"),
            issue: text("issue"),
            why: text("why"),
        })
    }

    /// One line saying which pass this is.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{} {} {}", self.toolchain, self.level.name(), self.pass)
    }

    fn is(&self, toolchain: &str, level: Level, pass: &str) -> bool {
        self.toolchain == toolchain && self.level == level && self.pass == pass
    }
}

/// The passes allowed to be quiet, as read from the file.
#[derive(Debug, Clone, Default)]
pub struct Quiet {
    /// The entries, in the order they are written.
    pub entries: Vec<Entry>,
}

impl Quiet {
    /// Reads the file, or answers an empty list when there is not one.
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
    /// When the text is not JSON, or is JSON without a `quiet` array in it.
    pub fn parse(text: &str) -> Result<Self, String> {
        let value = corpus_model::json::parse(text).map_err(|error| error.to_string())?;
        let items = value
            .get("quiet")
            .and_then(Json::as_array)
            .ok_or_else(|| "there is no `quiet` array in it".to_owned())?;
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
                    "Every rucc pass that runs at a level and rewrites nothing in the whole corpus there, with the reason. A run of the whole corpus fails on a quiet pass this file does not name, and on a line whose pass fired or no longer runs. Regenerate with `rucc-corpus run --accept` and write the reason on every new line.",
                ),
            ),
            ("quiet", Json::array(sorted.iter().map(Entry::to_json))),
        ]);
        format!("{}\n", value.to_pretty())
    }

    /// What these builds say about the file.
    ///
    /// Only the compilers and levels in `builds` are looked at, so a run at `-O2` alone says
    /// nothing about a line for `-O3`.
    #[must_use]
    pub fn check(&self, builds: &[Build]) -> Check {
        let mut check = Check::default();
        for build in builds {
            for total in build.quiet() {
                if !self
                    .entries
                    .iter()
                    .any(|entry| entry.is(&build.toolchain, build.level, &total.pass))
                {
                    check.unexpected.push(Entry {
                        toolchain: build.toolchain.clone(),
                        level: build.level,
                        pass: total.pass.clone(),
                        issue: String::new(),
                        why: String::new(),
                    });
                }
            }
        }
        for entry in &self.entries {
            let Some(build) = builds
                .iter()
                .find(|build| build.toolchain == entry.toolchain && build.level == entry.level)
            else {
                continue;
            };
            match build.passes.iter().find(|total| total.pass == entry.pass) {
                Some(total) if total.fired > 0 => check.fired.push(entry.clone()),
                Some(_) => {}
                None => check.gone.push(entry.clone()),
            }
        }
        check
    }

    /// The file these builds would have written, keeping the reason and the issue off the old one.
    ///
    /// Lines for a compiler and level the builds do not cover are carried over untouched, so
    /// accepting a run at one level does not empty the file of the others.
    #[must_use]
    pub fn accepting(&self, builds: &[Build]) -> Self {
        let covered = |entry: &Entry| {
            builds
                .iter()
                .any(|build| build.toolchain == entry.toolchain && build.level == entry.level)
        };
        let mut entries: Vec<Entry> =
            self.entries.iter().filter(|entry| !covered(entry)).cloned().collect();
        for build in builds {
            for total in build.quiet() {
                let old = self
                    .entries
                    .iter()
                    .find(|entry| entry.is(&build.toolchain, build.level, &total.pass));
                entries.push(Entry {
                    toolchain: build.toolchain.clone(),
                    level: build.level,
                    pass: total.pass.clone(),
                    issue: old.map(|entry| entry.issue.clone()).unwrap_or_default(),
                    why: old.map(|entry| entry.why.clone()).unwrap_or_default(),
                });
            }
        }
        entries.sort();
        entries.dedup();
        Self { entries }
    }
}

/// What a run had to say about the file.
#[derive(Debug, Clone, Default)]
pub struct Check {
    /// Quiet passes the file does not name.
    pub unexpected: Vec<Entry>,
    /// Lines whose pass fired this time, which is a line to take out.
    pub fired: Vec<Entry>,
    /// Lines whose pass did not run at that level at all, which is also a line to take out.
    pub gone: Vec<Entry>,
}

impl Check {
    /// Whether the run agreed with the file exactly.
    #[must_use]
    pub fn agrees(&self) -> bool {
        self.unexpected.is_empty() && self.fired.is_empty() && self.gone.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{Entry, Quiet};
    use crate::firing::{Build, Total};
    use corpus_model::Level;

    fn build(passes: &[(&str, usize)]) -> Build {
        Build {
            toolchain: "rucc".to_owned(),
            level: Level::O2,
            cases: 10,
            passes: passes
                .iter()
                .map(|(pass, fired)| Total {
                    pass: (*pass).to_owned(),
                    ran: 10,
                    fired: *fired,
                    ..Total::default()
                })
                .collect(),
        }
    }

    fn entry(level: Level, pass: &str) -> Entry {
        Entry {
            toolchain: "rucc".to_owned(),
            level,
            pass: pass.to_owned(),
            issue: "tamnd/rucc#2967".to_owned(),
            why: "nothing in the corpus has the shape yet".to_owned(),
        }
    }

    #[test]
    fn a_quiet_pass_the_file_names_is_expected_and_one_it_does_not_is_not() {
        let quiet = Quiet { entries: vec![entry(Level::O2, "licm")] };
        let check = quiet.check(&[build(&[("fold", 3), ("licm", 0), ("gcm", 0)])]);
        assert_eq!(check.unexpected.len(), 1);
        assert_eq!(check.unexpected[0].pass, "gcm");
        assert!(check.fired.is_empty() && check.gone.is_empty());
        assert!(!check.agrees());
    }

    #[test]
    fn a_line_whose_pass_fired_or_no_longer_runs_is_a_line_to_take_out() {
        let quiet = Quiet {
            entries: vec![
                entry(Level::O2, "fold"),
                entry(Level::O2, "gone"),
                entry(Level::O3, "fold"),
            ],
        };
        let check = quiet.check(&[build(&[("fold", 3)])]);
        assert_eq!(check.fired.iter().map(Entry::describe).collect::<Vec<_>>(), ["rucc O2 fold"]);
        assert_eq!(check.gone.iter().map(Entry::describe).collect::<Vec<_>>(), ["rucc O2 gone"]);
        assert!(check.unexpected.is_empty());
    }

    #[test]
    fn accepting_keeps_the_reasons_and_the_lines_the_run_did_not_cover() {
        let quiet = Quiet { entries: vec![entry(Level::O2, "licm"), entry(Level::O3, "unroll")] };
        let next = quiet.accepting(&[build(&[("licm", 0), ("gcm", 0), ("fold", 1)])]);
        let passes: Vec<String> = next.entries.iter().map(Entry::describe).collect();
        assert_eq!(passes, ["rucc O2 gcm", "rucc O2 licm", "rucc O3 unroll"]);
        assert_eq!(next.entries[1].why, "nothing in the corpus has the shape yet");
        assert!(next.entries[0].why.is_empty());
        let read = Quiet::parse(&next.to_text()).unwrap();
        assert_eq!(read.entries, next.entries);
    }
}
