//! The report a program reads.
//!
//! Three files, because three different things read them.
//!
//! `runs.jsonl` is one record per line, one line per case per compiler per level. It is the
//! raw evidence, it is append only, and a run that is killed halfway still leaves a usable
//! file behind. It is also the format that is cheap to grep, which matters when it has fifty
//! thousand lines in it.
//!
//! `report.json` is the summary. It is what a dashboard reads, what a pull request comment is
//! built from, and what the next run is compared against.
//!
//! `findings.sarif` is the same failures again in the format code hosting understands, so a
//! wrong answer shows up as an annotation on the pull request that caused it rather than in a
//! log nobody opens.
//!
//! Every one of them is written with keys in a fixed order and floats to three places, so two
//! runs that found the same thing produce byte identical files and a diff shows what changed
//! rather than where the whitespace moved.

use crate::summary::{FacetSummary, Summary, Tally, Target};
use corpus_model::{Finding, Json, RunRecord, SCHEMA_VERSION};
use corpus_run::Run;

/// The version of this crate, written into every report so a reader knows what made it.
const TOOL: &str = concat!("rucc-corpus ", env!("CARGO_PKG_VERSION"));

/// The raw evidence, one record per line.
#[must_use]
pub fn runs_jsonl(run: &Run) -> String {
    let mut out = String::new();
    for record in &run.records {
        out.push_str(&record.to_json().to_line());
        out.push('\n');
    }
    out
}

/// One record as a line, for a harness that streams them as it goes.
#[must_use]
pub fn run_line(record: &RunRecord) -> String {
    record.to_json().to_line()
}

/// The summary.
#[must_use]
pub fn report_json(run: &Run, summary: &Summary) -> String {
    let document = Json::object([
        ("schema", Json::string(SCHEMA_VERSION)),
        ("milestone", Json::string(corpus_model::MILESTONE)),
        ("tool", Json::string(TOOL)),
        ("corpus_digest", Json::string(summary.corpus_digest.clone())),
        ("cases", Json::int(summary.cases as i64)),
        ("reference", Json::string(summary.reference.clone())),
        ("levels", Json::array(summary.levels.iter().map(|l| Json::string(l.name())))),
        (
            "toolchains",
            Json::array(run.toolchains.iter().map(corpus_model::Toolchain::to_json)),
        ),
        (
            "totals",
            Json::array(
                summary.totals.iter().map(|(id, tally)| {
                    Json::object([("toolchain", Json::string(id.clone())), ("tally", tally_json(tally))])
                }),
            ),
        ),
        ("targets", Json::array(summary.targets.iter().map(target_json))),
        ("facets", Json::array(summary.facets.iter().map(facet_json))),
        ("findings", Json::array(run.findings.iter().map(Finding::to_json))),
    ]);
    let mut text = document.to_pretty();
    text.push('\n');
    text
}

fn tally_json(tally: &Tally) -> Json {
    Json::object([
        ("pass", Json::int(tally.pass as i64)),
        ("wrong", Json::int(tally.wrong as i64)),
        ("rejected", Json::int(tally.rejected as i64)),
        ("accepted", Json::int(tally.accepted as i64)),
        ("crashed", Json::int(tally.crashed as i64)),
        ("skipped", Json::int(tally.skipped as i64)),
        ("ran", Json::int(tally.ran() as i64)),
        ("failures", Json::int(tally.failures() as i64)),
        ("pass_rate", Json::Number(tally.pass_rate())),
    ])
}

fn target_json(target: &Target) -> Json {
    Json::object([
        ("name", Json::string(target.name.clone())),
        ("wanted", Json::string(target.wanted.clone())),
        ("threshold", target.threshold.map_or(Json::Null, Json::Number)),
        ("actual", target.actual.map_or(Json::Null, Json::Number)),
        ("met", Json::Bool(target.met)),
    ])
}

fn facet_json(facet: &FacetSummary) -> Json {
    Json::object([
        ("facet", Json::string(facet.facet.name())),
        ("phase", Json::string(facet.phase.name())),
        ("describes", Json::string(facet.facet.describe())),
        ("cases", Json::int(facet.cases as i64)),
        (
            "reference_said",
            Json::object([
                ("optimized", Json::int(facet.optimized as i64)),
                ("missed", Json::int(facet.missed as i64)),
            ]),
        ),
        (
            "scores",
            Json::array(facet.scores.iter().map(|score| {
                Json::object([
                    ("toolchain", Json::string(score.toolchain.clone())),
                    ("tally", tally_json(&score.tally)),
                    ("compared", Json::int(score.compared as i64)),
                    ("size_ratio", score.size_ratio.map_or(Json::Null, Json::Number)),
                    ("speed_ratio", score.speed_ratio.map_or(Json::Null, Json::Number)),
                    ("compile_ratio", score.compile_ratio.map_or(Json::Null, Json::Number)),
                ])
            })),
        ),
    ])
}

/// The failures, in the format code hosting understands.
///
/// The file a finding points at is the generated case under `programs/`, because that is the
/// file a person can open. Pointing at the generator would be pointing at the machine that
/// made the mistake rather than at the mistake.
#[must_use]
pub fn findings_sarif(run: &Run) -> String {
    let rules: Vec<Json> = rule_ids(run)
        .into_iter()
        .map(|id| {
            Json::object([
                ("id", Json::string(id.clone())),
                ("name", Json::string(id.replace('-', " "))),
                (
                    "shortDescription",
                    Json::object([("text", Json::string(describe_rule(&id)))]),
                ),
                (
                    "defaultConfiguration",
                    Json::object([("level", Json::string("error"))]),
                ),
            ])
        })
        .collect();

    let results: Vec<Json> = run.findings.iter().map(result_json).collect();

    let document = Json::object([
        ("version", Json::string("2.1.0")),
        (
            "$schema",
            Json::string("https://json.schemastore.org/sarif-2.1.0.json"),
        ),
        (
            "runs",
            Json::array([Json::object([
                (
                    "tool",
                    Json::object([(
                        "driver",
                        Json::object([
                            ("name", Json::string("rucc-corpus")),
                            ("version", Json::string(env!("CARGO_PKG_VERSION"))),
                            (
                                "informationUri",
                                Json::string("https://github.com/tamnd/rucc-corpus"),
                            ),
                            ("rules", Json::array(rules)),
                        ]),
                    )]),
                ),
                ("results", Json::array(results)),
            ])]),
        ),
    ]);
    let mut text = document.to_pretty();
    text.push('\n');
    text
}

fn result_json(finding: &Finding) -> Json {
    Json::object([
        ("ruleId", Json::string(finding.verdict.name())),
        ("level", Json::string("error")),
        (
            "message",
            Json::object([("text", Json::string(message_for(finding)))]),
        ),
        (
            "locations",
            Json::array([Json::object([(
                "physicalLocation",
                Json::object([(
                    "artifactLocation",
                    Json::object([
                        (
                            "uri",
                            Json::string(format!(
                                "programs/{}",
                                corpus_model::program_path(finding.facet, &finding.case)
                            )),
                        ),
                        ("uriBaseId", Json::string("%SRCROOT%")),
                    ]),
                )]),
            )])]),
        ),
        (
            "properties",
            Json::object([
                ("facet", Json::string(finding.facet.name())),
                ("toolchain", Json::string(finding.toolchain.clone())),
                ("level", Json::string(finding.level.name())),
            ]),
        ),
    ])
}

fn message_for(finding: &Finding) -> String {
    if finding.expected.is_empty() && finding.actual.is_empty() {
        return finding.summary.clone();
    }
    format!(
        "{}. Expected {}, got {}.",
        finding.summary.trim_end_matches('.'),
        one_line(&finding.expected),
        one_line(&finding.actual)
    )
}

/// Squashes a value onto one line so a SARIF message stays readable.
fn one_line(text: &str) -> String {
    let flattened = text.trim().replace('\n', " / ");
    if flattened.is_empty() {
        return "nothing".to_owned();
    }
    if flattened.chars().count() <= 200 {
        return flattened;
    }
    let cut: String = flattened.chars().take(200).collect();
    format!("{cut}...")
}

fn rule_ids(run: &Run) -> Vec<String> {
    let mut ids: Vec<String> =
        run.findings.iter().map(|finding| finding.verdict.name().to_owned()).collect();
    ids.sort();
    ids.dedup();
    ids
}

fn describe_rule(id: &str) -> String {
    match id {
        "wrong" => "The compiled program printed something other than the answer the generator computed.",
        "rejected" => "The compiler refused a program that is valid C.",
        "accepted" => "The compiler accepted a program that is not valid C.",
        "crashed" => "The compiler or the program it produced did not finish.",
        _ => "The case did not come out as expected.",
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{findings_sarif, report_json, runs_jsonl};
    use crate::summary::summarise;
    use corpus_model::{
        Axes, Case, Compile, Dialect, Execute, Expect, Facet, Finding, Json, Level, RunRecord,
        Toolchain, Verdict, json,
    };
    use corpus_run::Run;
    use std::collections::BTreeMap;

    /// Every key the schema names for one object, against every key the report actually wrote.
    ///
    /// A published schema that has drifted from what the tool emits is worse than no schema,
    /// because somebody wrote code against it. This is not a validator and does not try to be
    /// one. It checks the two lists of names are the same list, which is the failure that
    /// actually happens: a field gets added to the report and nobody opens the schema.
    fn keys_agree(schema: &Json, pointer: &[&str], written: &Json, at: &str) {
        let mut node = schema;
        for step in pointer {
            node = node.get(step).unwrap_or_else(|| panic!("the schema has no {step} in {at}"));
        }
        let named: std::collections::BTreeSet<&str> = node
            .get("properties")
            .and_then(Json::as_object)
            .unwrap_or_else(|| panic!("the schema names no properties at {at}"))
            .iter()
            .map(|(key, _)| key.as_str())
            .collect();
        let emitted: std::collections::BTreeSet<&str> = written
            .as_object()
            .unwrap_or_else(|| panic!("{at} is not an object in the report"))
            .iter()
            .map(|(key, _)| key.as_str())
            .collect();
        assert_eq!(
            named, emitted,
            "schema/report-v1.json and the {at} the reporter writes have drifted apart"
        );
    }

    #[test]
    fn the_published_schema_names_exactly_the_fields_the_report_writes() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../schema/report-v1.json");
        let schema = json::parse(&std::fs::read_to_string(path).unwrap()).unwrap();

        let run = sample_run(true);
        let summary = summarise(&run, "digest", 1);
        let report = json::parse(&report_json(&run, &summary)).unwrap();

        keys_agree(&schema, &[], &report, "report");
        keys_agree(&schema, &["$defs", "toolchain"], &report.get("toolchains").unwrap().as_array().unwrap()[0], "toolchain");
        keys_agree(&schema, &["$defs", "target"], &report.get("targets").unwrap().as_array().unwrap()[0], "target");
        keys_agree(&schema, &["$defs", "facet"], &report.get("facets").unwrap().as_array().unwrap()[0], "facet");
        keys_agree(&schema, &["$defs", "finding"], &report.get("findings").unwrap().as_array().unwrap()[0], "finding");

        let facet = &report.get("facets").unwrap().as_array().unwrap()[0];
        keys_agree(&schema, &["$defs", "score_line"], &facet.get("scores").unwrap().as_array().unwrap()[0], "score line");
        let score = &facet.get("scores").unwrap().as_array().unwrap()[0];
        keys_agree(&schema, &["$defs", "tally"], score.get("tally").unwrap(), "tally");

        assert_eq!(
            schema.get("properties").unwrap().get("schema").unwrap().get("const").unwrap().as_str(),
            Some(corpus_model::SCHEMA_VERSION),
            "the schema file and the version the reporter stamps have drifted apart"
        );
    }

    fn sample_run(with_finding: bool) -> Run {
        let case = Case::new(
            Facet::ConstantFold,
            Axes::of([("type", "i32")]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Output("42\n".to_owned()),
        );
        let mut records = Vec::new();
        for (id, text) in [("gcc-16", 100u64), ("rucc", 130)] {
            let mut record = RunRecord::skipped(&case, id, Level::O2);
            record.compile = Compile {
                ok: true,
                status: 0,
                micros: 2000,
                diagnostics: String::new(),
                bytes: text * 4,
                text_bytes: text,
            };
            record.execute = Execute {
                ok: true,
                status: 0,
                micros: 40,
                repeats: 5,
                output: "42\n".to_owned(),
            };
            records.push(record);
        }
        let verdicts: BTreeMap<String, Verdict> =
            records.iter().map(|r| (r.key(), Verdict::Pass)).collect();
        let findings = if with_finding {
            vec![Finding {
                case: case.id.clone(),
                facet: Facet::ConstantFold,
                toolchain: "rucc".to_owned(),
                level: Level::O2,
                verdict: Verdict::Wrong,
                summary: "rucc printed the wrong answer".to_owned(),
                expected: "42\n".to_owned(),
                actual: "41\n".to_owned(),
            }]
        } else {
            Vec::new()
        };
        Run {
            toolchains: vec![
                Toolchain {
                    id: "gcc-16".to_owned(),
                    program: "gcc-16".to_owned(),
                    version: "gcc (Homebrew GCC 16.1.0) 16.1.0".to_owned(),
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
            findings,
        }
    }

    #[test]
    fn the_evidence_file_is_one_record_per_line_and_every_line_is_valid_json() {
        let run = sample_run(false);
        let text = runs_jsonl(&run);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), run.records.len());
        let parsed = json::parse_lines(&text).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].get("toolchain").unwrap().as_str(), Some("gcc-16"));
        assert_eq!(parsed[0].get("level").unwrap().as_str(), Some("O2"));
    }

    #[test]
    fn the_summary_names_the_corpus_the_tool_and_the_reference() {
        let run = sample_run(false);
        let summary = summarise(&run, "deadbeef", 1);
        let text = report_json(&run, &summary);
        let parsed = json::parse(&text).unwrap();
        assert_eq!(parsed.get("schema").unwrap().as_str(), Some("1"));
        assert_eq!(parsed.get("corpus_digest").unwrap().as_str(), Some("deadbeef"));
        assert_eq!(parsed.get("reference").unwrap().as_str(), Some("gcc-16"));
        assert!(parsed.get("tool").unwrap().as_str().unwrap().starts_with("rucc-corpus "));
        assert_eq!(parsed.get("cases").unwrap().as_f64(), Some(1.0));
    }

    #[test]
    fn the_summary_carries_the_size_ratio_that_the_claim_rests_on() {
        let run = sample_run(false);
        let summary = summarise(&run, "deadbeef", 1);
        let parsed = json::parse(&report_json(&run, &summary)).unwrap();
        let facets = parsed.get("facets").unwrap().as_array().unwrap();
        let scores = facets[0].get("scores").unwrap().as_array().unwrap();
        let rucc = scores.iter().find(|s| s.get("toolchain").unwrap().as_str() == Some("rucc"));
        assert_eq!(rucc.unwrap().get("size_ratio").unwrap().as_f64(), Some(1.3));
        let quality = parsed
            .get("targets")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t.get("name").unwrap().as_str() == Some("code-quality:rucc"))
            .unwrap();
        assert_eq!(quality.get("met"), Some(&Json::Bool(false)));
    }

    #[test]
    fn the_same_run_written_twice_gives_the_same_bytes_so_a_diff_shows_a_real_change() {
        let run = sample_run(true);
        let summary = summarise(&run, "deadbeef", 1);
        assert_eq!(report_json(&run, &summary), report_json(&run, &summary));
        assert_eq!(runs_jsonl(&run), runs_jsonl(&run));
        assert_eq!(findings_sarif(&run), findings_sarif(&run));
    }

    #[test]
    fn a_failure_becomes_a_sarif_result_pointing_at_the_program_a_person_can_open() {
        let run = sample_run(true);
        let text = findings_sarif(&run);
        let parsed = json::parse(&text).unwrap();
        assert_eq!(parsed.get("version").unwrap().as_str(), Some("2.1.0"));
        let runs = parsed.get("runs").unwrap().as_array().unwrap();
        let results = runs[0].get("results").unwrap().as_array().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get("ruleId").unwrap().as_str(), Some("wrong"));
        let message = results[0].get("message").unwrap().get("text").unwrap().as_str().unwrap();
        assert!(message.contains("Expected 42"), "{message}");
        let uri = results[0]
            .get("locations")
            .unwrap()
            .as_array()
            .unwrap()[0]
            .get("physicalLocation")
            .unwrap()
            .get("artifactLocation")
            .unwrap()
            .get("uri")
            .unwrap()
            .as_str()
            .unwrap();
        assert!(uri.starts_with("programs/"), "{uri}");
        assert!(uri.ends_with(".c"), "{uri}");
    }

    #[test]
    fn a_clean_run_produces_a_sarif_file_with_no_results_rather_than_no_file() {
        let run = sample_run(false);
        let parsed = json::parse(&findings_sarif(&run)).unwrap();
        let runs = parsed.get("runs").unwrap().as_array().unwrap();
        assert!(runs[0].get("results").unwrap().as_array().unwrap().is_empty());
    }
}
