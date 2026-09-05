//! Turns a run into something somebody can use.
//!
//! Two reports, written from one summary. The human one is markdown and leads with the
//! answer. The machine one is JSON against a versioned schema, plus the raw records as JSON
//! Lines, plus SARIF for the failures. They cannot disagree with each other because they are
//! both rendered from [`summary::Summary`] and nothing else.
//!
//! # Why write the JSON by hand
//!
//! The machine report is a contract. Other tools read it, a nightly job compares today's
//! against yesterday's, and a pull request comment is built from it. A hand written writer
//! means the key order is the order in the source, floats print to three places rather than
//! to seventeen, and a report that has not changed produces a file that has not changed. A
//! diff of two reports then shows what actually moved, which is the whole point of keeping
//! them in the repository.

#![forbid(unsafe_code)]

pub mod human;
pub mod machine;
pub mod summary;
pub mod terminal;

use corpus_run::Run;
use std::path::Path;

/// The names of the files a report is made of.
pub mod files {
    /// The human report.
    pub const HUMAN: &str = "report.md";
    /// The machine report.
    pub const MACHINE: &str = "report.json";
    /// The raw records.
    pub const RECORDS: &str = "runs.jsonl";
    /// The failures, for code hosting.
    pub const SARIF: &str = "findings.sarif";
}

/// Writes all four files into a directory.
///
/// # Errors
///
/// When the directory cannot be created or a file cannot be written.
pub fn write_all(
    dir: &Path,
    run: &Run,
    corpus_digest: &str,
    cases: usize,
) -> Result<summary::Summary, String> {
    std::fs::create_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    let summary = summary::summarise(run, corpus_digest, cases);
    write(dir, files::RECORDS, &machine::runs_jsonl(run))?;
    write(dir, files::MACHINE, &machine::report_json(run, &summary))?;
    write(dir, files::SARIF, &machine::findings_sarif(run))?;
    write(dir, files::HUMAN, &human::report_md(run, &summary))?;
    Ok(summary)
}

fn write(dir: &Path, name: &str, text: &str) -> Result<(), String> {
    let path = dir.join(name);
    std::fs::write(&path, text).map_err(|error| format!("{}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::{files, write_all};
    use corpus_model::{
        Axes, Case, Compile, Dialect, Execute, Expect, Facet, Level, RunRecord, Toolchain, Verdict,
        json,
    };
    use corpus_run::Run;
    use std::collections::BTreeMap;

    fn sample() -> Run {
        let case = Case::new(
            Facet::Simplify,
            Axes::of([("shape", "one")]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Output("1\n".to_owned()),
        );
        let mut records = Vec::new();
        for (id, text) in [("gcc-16", 100u64), ("rucc", 104)] {
            let mut record = RunRecord::skipped(&case, id, Level::O2);
            record.compile = Compile {
                ok: true,
                status: 0,
                micros: 2500,
                diagnostics: String::new(),
                bytes: text * 4,
                text_bytes: text,
            };
            record.execute =
                Execute { ok: true, status: 0, micros: 70, repeats: 5, output: "1\n".to_owned() };
            records.push(record);
        }
        let verdicts: BTreeMap<String, Verdict> =
            records.iter().map(|r| (r.key(), Verdict::Pass)).collect();
        Run {
            toolchains: vec![
                Toolchain {
                    id: "gcc-16".to_owned(),
                    program: "gcc-16".to_owned(),
                    version: "gcc (GCC) 16.1.0".to_owned(),
                    reference: true,
                },
                Toolchain {
                    id: "rucc".to_owned(),
                    program: "rucc".to_owned(),
                    version: "rucc 0.4.0".to_owned(),
                    reference: false,
                },
            ],
            records,
            verdicts,
            findings: Vec::new(),
        }
    }

    #[test]
    fn all_four_files_are_written_and_the_two_reports_agree_with_each_other() {
        let dir = std::env::temp_dir().join(format!("rucc-corpus-report-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let summary = write_all(&dir, &sample(), "abc123", 1).unwrap();

        let machine = std::fs::read_to_string(dir.join(files::MACHINE)).unwrap();
        let parsed = json::parse(&machine).unwrap();
        let human = std::fs::read_to_string(dir.join(files::HUMAN)).unwrap();

        assert_eq!(parsed.get("cases").unwrap().as_f64(), Some(1.0));
        assert!(human.contains("The corpus holds 1 programs"));

        // The size ratio is four percent, and it has to be the same four percent in both.
        let facets = parsed.get("facets").unwrap().as_array().unwrap();
        let scores = facets[0].get("scores").unwrap().as_array().unwrap();
        let rucc = scores.iter().find(|s| s.get("toolchain").unwrap().as_str() == Some("rucc"));
        assert_eq!(rucc.unwrap().get("size_ratio").unwrap().as_f64(), Some(1.04));
        assert!(human.contains("4 percent more"));

        assert!(summary.correct());
        assert!(dir.join(files::RECORDS).is_file());
        assert!(dir.join(files::SARIF).is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
