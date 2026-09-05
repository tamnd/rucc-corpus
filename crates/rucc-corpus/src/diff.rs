//! Comparing two reports.
//!
//! This is the command that makes the corpus worth keeping around rather than worth running
//! once. A phase of the M4 plan lands, the corpus is run again, and the question is not
//! whether the new report looks good but whether it is better than the old one and where it
//! got worse. A pass that improves eleven facets and quietly costs thirty percent on a
//! twelfth is a pass that needs another look, and nothing except a comparison finds that.
//!
//! It reads `report.json` rather than the raw records, so a comparison is cheap and can be
//! run against a report that came from somebody else's machine or from a nightly job.

use crate::args::Args;
use corpus_model::{Json, json};
use std::process::ExitCode;

/// How much a ratio has to move before it is worth printing.
///
/// Two percent. Below that it is the machine, not the compiler. Code size does not actually
/// wobble between runs, but compile time and run time do, and one threshold for all three is
/// easier to explain than three thresholds.
const NOTICEABLE: f64 = 0.02;

/// Compares two reports.
///
/// # Errors
///
/// When either file is missing, is not JSON, or is not a report.
pub(crate) fn command(args: &Args) -> Result<ExitCode, String> {
    args.only(&["format"])?;
    let [old_path, new_path] = match args.rest.as_slice() {
        [old, new] => [old.clone(), new.clone()],
        _ => return Err("diff wants two report.json files, an old one and a new one".to_owned()),
    };

    let old = load(&old_path)?;
    let new = load(&new_path)?;
    let report = compare(&old, &new);

    if args.value("format") == Some("json") {
        print!("{}", report.to_json().to_pretty());
        println!();
    } else {
        print!("{}", report.to_text(&old_path, &new_path));
    }
    Ok(if report.regressed() { ExitCode::FAILURE } else { ExitCode::SUCCESS })
}

/// Reads a report and checks that it is one.
fn load(path: &str) -> Result<Json, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let parsed = json::parse(&text).map_err(|error| format!("{path}: {error}"))?;
    if parsed.get("facets").is_none() {
        return Err(format!("{path} has no facets in it, so it is not a corpus report"));
    }
    Ok(parsed)
}

/// One facet that moved.
#[derive(Debug, Clone)]
pub(crate) struct Moved {
    /// Which facet.
    pub(crate) facet: String,
    /// Which compiler.
    pub(crate) toolchain: String,
    /// The old ratio against the reference.
    pub(crate) before: f64,
    /// The new one.
    pub(crate) after: f64,
}

impl Moved {
    /// How much it moved, as a share of where it was.
    #[must_use]
    pub(crate) fn change(&self) -> f64 {
        self.after / self.before - 1.0
    }
}

/// What changed between two reports.
#[derive(Debug, Clone, Default)]
pub(crate) struct Comparison {
    /// Whether the two reports are about the same corpus.
    pub(crate) same_corpus: bool,
    /// Facets that got bigger.
    pub(crate) worse: Vec<Moved>,
    /// Facets that got smaller.
    pub(crate) better: Vec<Moved>,
    /// Targets that were met before and are not met now.
    pub(crate) lost: Vec<String>,
    /// Targets that were not met before and are met now.
    pub(crate) gained: Vec<String>,
    /// How many results failed before.
    pub(crate) failures_before: usize,
    /// How many fail now.
    pub(crate) failures_after: usize,
}

impl Comparison {
    /// Whether anything got worse.
    #[must_use]
    pub(crate) fn regressed(&self) -> bool {
        self.failures_after > self.failures_before || !self.lost.is_empty()
    }

    /// The comparison as JSON.
    #[must_use]
    pub(crate) fn to_json(&self) -> Json {
        Json::object([
            ("same_corpus", Json::Bool(self.same_corpus)),
            ("failures_before", Json::int(self.failures_before as i64)),
            ("failures_after", Json::int(self.failures_after as i64)),
            ("targets_lost", Json::array(self.lost.iter().map(Json::string))),
            ("targets_gained", Json::array(self.gained.iter().map(Json::string))),
            ("worse", Json::array(self.worse.iter().map(moved_json))),
            ("better", Json::array(self.better.iter().map(moved_json))),
            ("regressed", Json::Bool(self.regressed())),
        ])
    }

    /// The comparison as something to read.
    #[must_use]
    pub(crate) fn to_text(&self, old_path: &str, new_path: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("{old_path} against {new_path}\n\n"));
        if !self.same_corpus {
            out.push_str(
                "These two reports are about different corpora, so a facet that moved may have moved because its programs changed rather than because the compiler did. Compare the numbers with that in mind.\n\n",
            );
        }
        out.push_str(&format!(
            "failures: {} before, {} now\n",
            self.failures_before, self.failures_after
        ));
        for name in &self.lost {
            out.push_str(&format!("lost target: {name}\n"));
        }
        for name in &self.gained {
            out.push_str(&format!("met target: {name}\n"));
        }
        out.push('\n');
        section(&mut out, "got worse", &self.worse);
        section(&mut out, "got better", &self.better);
        if self.worse.is_empty() && self.better.is_empty() {
            out.push_str("no facet moved by more than two percent\n");
        }
        out
    }
}

fn section(out: &mut String, title: &str, moved: &[Moved]) {
    if moved.is_empty() {
        return;
    }
    out.push_str(&format!("{title}:\n"));
    for item in moved {
        out.push_str(&format!(
            "  {:<24} {:<10} {:.3} to {:.3}, {:+.1} percent\n",
            item.facet,
            item.toolchain,
            item.before,
            item.after,
            item.change() * 100.0
        ));
    }
    out.push('\n');
}

fn moved_json(moved: &Moved) -> Json {
    Json::object([
        ("facet", Json::string(moved.facet.clone())),
        ("toolchain", Json::string(moved.toolchain.clone())),
        ("before", Json::Number(moved.before)),
        ("after", Json::Number(moved.after)),
        ("change", Json::Number(moved.change())),
    ])
}

/// Works out what changed.
#[must_use]
pub(crate) fn compare(old: &Json, new: &Json) -> Comparison {
    let mut out = Comparison {
        same_corpus: old.get("corpus_digest").and_then(Json::as_str)
            == new.get("corpus_digest").and_then(Json::as_str),
        failures_before: failures(old),
        failures_after: failures(new),
        ..Comparison::default()
    };

    let before = size_ratios(old);
    let after = size_ratios(new);
    for (key, was) in &before {
        let Some(now) = after.get(key) else {
            continue;
        };
        let change = now / was - 1.0;
        if change.abs() < NOTICEABLE {
            continue;
        }
        let (facet, toolchain) = key;
        let moved = Moved {
            facet: facet.clone(),
            toolchain: toolchain.clone(),
            before: *was,
            after: *now,
        };
        if change > 0.0 { out.worse.push(moved) } else { out.better.push(moved) }
    }
    out.worse.sort_by(|a, b| b.change().total_cmp(&a.change()));
    out.better.sort_by(|a, b| a.change().total_cmp(&b.change()));

    let met_before = met_targets(old);
    let met_after = met_targets(new);
    for name in &met_before {
        if !met_after.contains(name) {
            out.lost.push(name.clone());
        }
    }
    for name in &met_after {
        if !met_before.contains(name) {
            out.gained.push(name.clone());
        }
    }
    out
}

/// The code size ratio of every facet and compiler in a report.
fn size_ratios(report: &Json) -> std::collections::BTreeMap<(String, String), f64> {
    let mut out = std::collections::BTreeMap::new();
    let Some(facets) = report.get("facets").and_then(Json::as_array) else {
        return out;
    };
    for facet in facets {
        let Some(name) = facet.get("facet").and_then(Json::as_str) else {
            continue;
        };
        let Some(scores) = facet.get("scores").and_then(Json::as_array) else {
            continue;
        };
        for score in scores {
            let Some(toolchain) = score.get("toolchain").and_then(Json::as_str) else {
                continue;
            };
            let Some(ratio) = score.get("size_ratio").and_then(Json::as_f64) else {
                continue;
            };
            if ratio <= 0.0 {
                continue;
            }
            out.insert((name.to_owned(), toolchain.to_owned()), ratio);
        }
    }
    out
}

/// How many results in a report were not passes.
fn failures(report: &Json) -> usize {
    report
        .get("totals")
        .and_then(Json::as_array)
        .map(|totals| {
            totals
                .iter()
                .filter_map(|entry| entry.get("tally")?.get("failures")?.as_f64())
                .map(|count| count as usize)
                .sum()
        })
        .unwrap_or_default()
}

/// The names of the targets a report says were met.
fn met_targets(report: &Json) -> Vec<String> {
    report
        .get("targets")
        .and_then(Json::as_array)
        .map(|targets| {
            targets
                .iter()
                .filter(|target| matches!(target.get("met"), Some(Json::Bool(true))))
                .filter_map(|target| Some(target.get("name")?.as_str()?.to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::compare;
    use corpus_model::json;

    fn report(digest: &str, ratio: f64, failures: usize, met: bool) -> corpus_model::Json {
        let text = format!(
            r#"{{
              "corpus_digest": "{digest}",
              "totals": [{{"toolchain": "rucc", "tally": {{"failures": {failures}}}}}],
              "targets": [{{"name": "code-quality:rucc", "met": {met}}}],
              "facets": [
                {{"facet": "loop-unroll", "scores": [{{"toolchain": "rucc", "size_ratio": {ratio}}}]}},
                {{"facet": "simplify", "scores": [{{"toolchain": "rucc", "size_ratio": 1.0}}]}}
              ]
            }}"#
        );
        json::parse(&text).unwrap()
    }

    #[test]
    fn a_facet_that_got_bigger_is_listed_as_worse_and_one_that_shrank_as_better() {
        let old = report("abc", 1.0, 0, true);
        let new = report("abc", 1.5, 0, true);
        let found = compare(&old, &new);
        assert_eq!(found.worse.len(), 1);
        assert_eq!(found.worse[0].facet, "loop-unroll");
        assert!((found.worse[0].change() - 0.5).abs() < 1e-9);
        assert!(found.better.is_empty());

        let back = compare(&new, &old);
        assert_eq!(back.better.len(), 1);
        assert!(back.worse.is_empty());
    }

    #[test]
    fn a_movement_smaller_than_the_noise_is_not_reported() {
        let old = report("abc", 1.00, 0, true);
        let new = report("abc", 1.01, 0, true);
        let found = compare(&old, &new);
        assert!(found.worse.is_empty(), "{:?}", found.worse);
        assert!(found.better.is_empty());
        assert!(!found.regressed());
    }

    #[test]
    fn losing_a_target_or_gaining_a_failure_is_a_regression_and_a_bigger_facet_alone_is_not() {
        let clean = report("abc", 1.0, 0, true);

        let lost = compare(&clean, &report("abc", 1.0, 0, false));
        assert_eq!(lost.lost, ["code-quality:rucc"]);
        assert!(lost.regressed());

        let broke = compare(&clean, &report("abc", 1.0, 3, true));
        assert_eq!(broke.failures_after, 3);
        assert!(broke.regressed());

        // A facet that got bigger without losing a target is worth reading about and is not
        // worth failing a build over, because the target is the thing that was promised.
        let bigger = compare(&clean, &report("abc", 1.4, 0, true));
        assert!(!bigger.worse.is_empty());
        assert!(!bigger.regressed());

        let fixed = compare(&report("abc", 1.0, 0, false), &clean);
        assert_eq!(fixed.gained, ["code-quality:rucc"]);
        assert!(!fixed.regressed());
    }

    #[test]
    fn comparing_two_different_corpora_says_so_rather_than_pretending_the_numbers_line_up() {
        let found = compare(&report("abc", 1.0, 0, true), &report("def", 1.3, 0, true));
        assert!(!found.same_corpus);
        let text = found.to_text("old.json", "new.json");
        assert!(text.contains("different corpora"), "{text}");
    }

    #[test]
    fn a_comparison_with_nothing_in_it_says_nothing_moved() {
        let found = compare(&report("abc", 1.0, 0, true), &report("abc", 1.0, 0, true));
        let text = found.to_text("old.json", "new.json");
        assert!(text.contains("no facet moved"), "{text}");
        assert!(found.same_corpus);
    }
}
