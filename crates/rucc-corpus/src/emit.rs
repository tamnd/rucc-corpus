//! Writing the corpus out as C that a person can read.
//!
//! The generator could stay a program that produces programs, and the harness would work
//! exactly the same. It is written to disk anyway, and the files are kept in the repository,
//! for reasons that have nothing to do with the harness.
//!
//! A reviewer can open the file and disagree with it. A bug report can name a file rather
//! than a seed and a version of the generator. A diff of two commits shows exactly which
//! programs a generator change altered, which is the only practical way to review a change
//! that touches a thousand cases. And somebody who has never seen this repository before can
//! read one directory and understand what the corpus claims about loop unrolling.
//!
//! Each facet directory gets an index that lists what its programs expect, because the
//! expected answer is the interesting half and it is not in the C file. It is not in the C
//! file on purpose: the file that gets compiled is the file that is in the repository, byte
//! for byte, so a comment carrying the answer would be a second copy that could drift.

use corpus_model::{Case, Expect, Facet, Manifest, Phase};
use std::collections::BTreeMap;
use std::path::Path;

/// Writes every case, grouped by phase and then by facet.
///
/// Returns how many files were written.
///
/// # Errors
///
/// When a directory cannot be created or a file cannot be written.
pub(crate) fn write_programs(root: &Path, corpus: &Manifest) -> Result<usize, String> {
    std::fs::create_dir_all(root).map_err(|error| format!("{}: {error}", root.display()))?;

    let mut by_facet: BTreeMap<Facet, Vec<&Case>> = BTreeMap::new();
    for case in &corpus.cases {
        by_facet.entry(case.facet).or_default().push(case);
    }

    let mut written = 0;
    for (facet, cases) in &by_facet {
        let dir = root.join(facet.phase().name()).join(facet.name());
        std::fs::create_dir_all(&dir).map_err(|error| format!("{}: {error}", dir.display()))?;
        for case in cases {
            let path = dir.join(case.file_name());
            std::fs::write(&path, &case.source)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            written += 1;
        }
        let index = dir.join("index.md");
        std::fs::write(&index, facet_index(*facet, cases))
            .map_err(|error| format!("{}: {error}", index.display()))?;
    }

    let index = root.join("index.md");
    std::fs::write(&index, top_index(corpus))
        .map_err(|error| format!("{}: {error}", index.display()))?;
    Ok(written)
}

/// Writes the manifest, which is the machine readable copy of the same thing.
///
/// # Errors
///
/// When the file cannot be written.
pub(crate) fn write_manifest(path: &Path, corpus: &Manifest) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    let mut text = corpus.to_json().to_pretty();
    text.push('\n');
    std::fs::write(path, text).map_err(|error| format!("{}: {error}", path.display()))
}

/// The page at the top of the programs directory.
fn top_index(corpus: &Manifest) -> String {
    let mut out = String::new();
    out.push_str("# The corpus\n\n");
    out.push_str(&format!(
        "{} programs. Every one of them was written for exactly one named transformation, prints an answer this repository computed in Rust before any C was compiled, and prints nothing that depends on the machine it runs on. That last part is what lets one expected output be right everywhere.\n\n",
        corpus.cases.len()
    ));
    out.push_str(
        "These files are generated. Editing one here changes nothing, because the next run of `rucc-corpus gen` writes it back. The thing to edit is the generator in `crates/corpus-gen`, and the reason the output is kept in the repository anyway is so that a change to the generator shows up as a diff of the programs it produces.\n\n",
    );
    out.push_str(&format!("Corpus digest `{}`.\n\n", corpus.digest()));

    for phase in Phase::ALL {
        let facets: Vec<(Facet, usize)> = corpus
            .by_facet()
            .into_iter()
            .filter(|(facet, _)| facet.phase() == *phase)
            .collect();
        if facets.is_empty() {
            continue;
        }
        let total: usize = facets.iter().map(|(_, count)| count).sum();
        out.push_str(&format!("## {} ({total} programs)\n\n", phase.name()));
        out.push_str(&format!("{}\n\n", phase.describe()));
        out.push_str("| facet | programs | what it is about |\n|---|---|---|\n");
        for (facet, count) in facets {
            out.push_str(&format!(
                "| [`{0}`]({1}/{0}/index.md) | {count} | {2} |\n",
                facet.name(),
                phase.name(),
                facet.describe()
            ));
        }
        out.push('\n');
    }
    out
}

/// The page inside one facet directory.
fn facet_index(facet: Facet, cases: &[&Case]) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", facet.name()));
    out.push_str(&format!("{}. Part of the {} phase of the M4 plan.\n\n", facet.describe(), facet.phase().name()));
    out.push_str(&format!(
        "{} programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.\n\n",
        cases.len()
    ));
    out.push_str("| program | axes | dialect | must print |\n|---|---|---|---|\n");
    for case in cases {
        let axes = case
            .axes
            .points
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<String>>()
            .join(", ");
        out.push_str(&format!(
            "| [`{}`]({}) | {} | {} | {} |\n",
            case.id,
            case.file_name(),
            if axes.is_empty() { "none".to_owned() } else { axes },
            case.dialect.name(),
            expectation(case)
        ));
    }
    out.push('\n');
    out
}

/// What a case must produce, written to fit in a table cell.
fn expectation(case: &Case) -> String {
    match &case.expect {
        Expect::Output(text) => {
            let lines: Vec<&str> = text.lines().collect();
            if lines.len() <= 4 {
                return format!("`{}`", lines.join(" "));
            }
            format!("`{} ...` and {} more lines", lines[..3].join(" "), lines.len() - 3)
        }
        Expect::Rejected(mentions) => {
            format!("nothing, it must be rejected with a diagnostic mentioning `{mentions}`")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{expectation, facet_index, top_index, write_manifest, write_programs};
    use corpus_model::{Axes, Case, Dialect, Expect, Facet, Manifest};

    fn case_for(facet: Facet, name: &str, output: &str) -> Case {
        Case::new(
            facet,
            Axes::of([("shape", name)]),
            Dialect::C17,
            format!("int main(void) {{ return 0; }} /* {name} */\n"),
            Expect::Output(output.to_owned()),
        )
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("rucc-corpus-emit-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn every_program_lands_under_its_phase_and_its_facet() {
        let dir = scratch("layout");
        let corpus = Manifest::new(vec![
            case_for(Facet::ConstantFold, "one", "1\n"),
            case_for(Facet::LoopUnroll, "two", "2\n"),
        ])
        .unwrap();
        let written = write_programs(&dir, &corpus).unwrap();
        assert_eq!(written, 2);
        for case in &corpus.cases {
            let path = dir.join(case.path());
            assert!(path.is_file(), "{} was not written", path.display());
            assert_eq!(std::fs::read_to_string(&path).unwrap(), case.source);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_file_on_disk_is_byte_for_byte_the_file_that_gets_compiled() {
        let dir = scratch("bytes");
        let case = case_for(Facet::Simplify, "one", "1\n");
        let corpus = Manifest::new(vec![case.clone()]).unwrap();
        write_programs(&dir, &corpus).unwrap();
        let written = std::fs::read_to_string(dir.join(case.path())).unwrap();
        // No header comment carrying the expected answer, because that would be a second copy
        // of the answer that could drift from the one the harness checks.
        assert_eq!(written, case.source);
        assert!(!written.contains("must print"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn every_directory_gets_an_index_that_says_what_its_programs_expect() {
        let dir = scratch("index");
        let corpus = Manifest::new(vec![case_for(Facet::ConstantFold, "one", "42\n")]).unwrap();
        write_programs(&dir, &corpus).unwrap();

        let top = std::fs::read_to_string(dir.join("index.md")).unwrap();
        assert!(top.contains("# The corpus"));
        assert!(top.contains("constant-fold"));
        assert!(top.contains(&corpus.digest()));

        let inner =
            std::fs::read_to_string(dir.join("local/constant-fold/index.md")).unwrap();
        assert!(inner.contains("`42`"), "{inner}");
        assert!(inner.contains("must print"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_case_that_must_be_rejected_says_so_in_the_index_instead_of_showing_an_output() {
        let case = Case::new(
            Facet::Frontend,
            Axes::of([("rejected", "duplicate-case")]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Rejected("duplicate case".to_owned()),
        );
        let text = expectation(&case);
        assert!(text.contains("must be rejected"));
        assert!(text.contains("duplicate case"));
    }

    #[test]
    fn a_long_expected_output_is_summarised_rather_than_pasted_into_a_table_cell() {
        let long: String = (0..40).map(|n| format!("{n}\n")).collect();
        let case = case_for(Facet::LoopIdiom, "long", &long);
        let text = expectation(&case);
        assert!(text.contains("and 37 more lines"), "{text}");
        assert!(!text.contains('\n'));
    }

    #[test]
    fn the_indexes_hold_to_the_house_style() {
        let corpus = Manifest::new(vec![
            case_for(Facet::ConstantFold, "one", "1\n"),
            case_for(Facet::LoopUnroll, "two", "2\n"),
        ])
        .unwrap();
        let cases: Vec<&Case> = corpus.cases.iter().collect();
        for text in [top_index(&corpus), facet_index(Facet::ConstantFold, &cases)] {
            assert!(!text.contains('\u{2014}'), "an em dash got into an index");
            assert!(!text.contains('\u{2013}'), "an en dash got into an index");
            for line in text.lines() {
                assert_ne!(line.trim(), "---", "a horizontal rule got into an index");
            }
        }
    }

    #[test]
    fn the_manifest_is_valid_json_and_names_the_corpus_it_describes() {
        let dir = scratch("manifest");
        std::fs::create_dir_all(&dir).unwrap();
        let corpus = Manifest::new(vec![case_for(Facet::Baseline, "one", "1\n")]).unwrap();
        let path = dir.join("manifest.json");
        write_manifest(&path, &corpus).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        let parsed = corpus_model::json::parse(&text).unwrap();
        assert_eq!(parsed.get("count").unwrap().as_f64(), Some(1.0));
        assert_eq!(
            parsed.get("corpus_sha256").unwrap().as_str(),
            Some(corpus.digest().as_str())
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
