//! Deciding whether a record is a pass, and saying why when it is not.
//!
//! The oracle is the generator, not the reference compiler. That is the thing which makes
//! this corpus worth having: every program prints a value the generator worked out in Rust
//! before any C was written, so a bug that GCC and rucc both have still shows up here. A
//! differential harness cannot do that, and rucc-compat, which is a differential harness over
//! third party code, is the other half of the picture rather than the same half again.
//!
//! The reference compiler is still run on every case, for two reasons. It is the number the
//! code quality and speed claims are measured against, and it is the thing that can be asked
//! what it thought the program allowed.

use corpus_model::{Case, Expect, Finding, Level, RunRecord, Verdict};

/// Decides how a record came out.
///
/// The case has to be the one the record is about. Passing the wrong pair in would compare a
/// program against another program's expected output, so it is checked rather than assumed.
#[must_use]
pub fn judge(case: &Case, record: &RunRecord) -> Verdict {
    debug_assert_eq!(case.id, record.case, "a record was judged against the wrong case");
    match &case.expect {
        Expect::Output(wanted) => judge_running_case(record, wanted),
        Expect::Rejected(mentions) => judge_rejected_case(record, mentions),
    }
}

/// A case that should compile, run and print a known answer.
fn judge_running_case(record: &RunRecord, wanted: &str) -> Verdict {
    if !record.compile.ok {
        // A compiler that died without saying anything crashed. A compiler that printed a
        // diagnostic and gave up rejected a program it should have accepted. They are both
        // bugs and they are found in completely different places, so they are told apart.
        if record.compile.diagnostics.is_empty() || record.compile.status < 0 {
            return Verdict::Crashed;
        }
        return if admits_a_gap(&record.compile.diagnostics) {
            Verdict::Unimplemented
        } else {
            Verdict::Rejected
        };
    }
    if !record.execute.ok {
        return Verdict::Crashed;
    }
    if record.execute.output == wanted { Verdict::Pass } else { Verdict::Wrong }
}

/// A case that should not compile at all.
fn judge_rejected_case(record: &RunRecord, mentions: &str) -> Verdict {
    if record.compile.ok {
        return Verdict::Accepted;
    }
    if record.compile.status < 0 {
        return Verdict::Crashed;
    }
    if mentions_it(&record.compile.diagnostics, mentions) { Verdict::Pass } else { Verdict::Wrong }
}

/// The wordings a compiler uses to say it has not built the thing yet.
///
/// Every entry is a phrase a compiler prints about itself, and that is the whole basis for
/// treating this differently from an ordinary rejection. Guessing from the shape of the program
/// which constructs a compiler probably does not have would put the corpus in the business of
/// tracking somebody else's roadmap, and would go quietly stale the day the feature landed. A
/// compiler that says the words is a compiler that has told us, and one that stops saying them is
/// one whose case starts passing on its own.
///
/// GCC's is `sorry, unimplemented:`, which it has printed since the nineties. rucc's is the note
/// it attaches to the E0653 family, `this construct is not lowered yet`. Clang's is
/// `unsupported ... in this compiler`, which is close enough to the same sentence.
const ADMISSIONS: &[&str] = &[
    "sorry, unimplemented",
    "not lowered yet",
    "not implemented yet",
    "is not yet supported",
    "unsupported by this compiler",
];

/// Whether the compiler said, in its own words, that it has not built this yet.
#[must_use]
pub fn admits_a_gap(diagnostics: &str) -> bool {
    let lowered = diagnostics.to_ascii_lowercase();
    ADMISSIONS.iter().any(|phrase| lowered.contains(phrase))
}

/// Whether a diagnostic said the thing the case expected it to say.
///
/// Case insensitive, and a fragment rather than the whole message. Requiring the exact wording
/// would mean the corpus goes red every time somebody improves a diagnostic, and what is
/// worth checking is that the compiler caught the right mistake, not how well it worded it.
#[must_use]
pub fn mentions_it(diagnostics: &str, wanted: &str) -> bool {
    if wanted.is_empty() {
        return true;
    }
    diagnostics.to_ascii_lowercase().contains(&wanted.to_ascii_lowercase())
}

/// Turns a verdict that is not a pass into something a person can act on.
///
/// Returns `None` for a pass or a skip, so the caller can collect findings by filtering. A gap
/// gets a finding like a failure does, because it is a thing somebody has to act on eventually,
/// and the verdict on the finding is what keeps the two apart in the report.
#[must_use]
pub fn finding(case: &Case, record: &RunRecord, verdict: Verdict) -> Option<Finding> {
    if !verdict.is_failure() && !verdict.is_gap() {
        return None;
    }
    let (summary, expected, actual) = describe(case, record, verdict);
    Some(Finding {
        case: case.id.clone(),
        facet: case.facet,
        toolchain: record.toolchain.clone(),
        level: record.level,
        verdict,
        summary,
        expected,
        actual,
    })
}

/// The sentence, and the two sides of the comparison that produced it.
fn describe(case: &Case, record: &RunRecord, verdict: Verdict) -> (String, String, String) {
    let what = case.facet.describe();
    match verdict {
        Verdict::Wrong => match &case.expect {
            Expect::Output(wanted) => (
                format!("{} printed the wrong answer for a case about {what}", record.toolchain),
                wanted.clone(),
                record.execute.output.clone(),
            ),
            Expect::Rejected(mentions) => (
                format!(
                    "{} rejected the case, which is right, but for a different reason than the one it was written for",
                    record.toolchain
                ),
                format!("a diagnostic mentioning {mentions}"),
                record.compile.diagnostics.clone(),
            ),
        },
        Verdict::Rejected => (
            format!("{} would not compile a valid program about {what}", record.toolchain),
            "the program compiles".to_owned(),
            record.compile.diagnostics.clone(),
        ),
        Verdict::Unimplemented => (
            format!("{} has not built the part of {what} this case needs yet", record.toolchain),
            "the program compiles".to_owned(),
            record.compile.diagnostics.clone(),
        ),
        Verdict::Accepted => (
            format!("{} accepted a program that is not valid C", record.toolchain),
            match &case.expect {
                Expect::Rejected(mentions) => format!("a diagnostic mentioning {mentions}"),
                Expect::Output(_) => "a diagnostic".to_owned(),
            },
            "it compiled without complaint".to_owned(),
        ),
        Verdict::Crashed => (
            format!(
                "{} did not finish on a case about {what}, exit status {}",
                record.toolchain,
                if record.compile.ok { record.execute.status } else { record.compile.status }
            ),
            "the program compiles and runs".to_owned(),
            crash_detail(record),
        ),
        Verdict::Pass | Verdict::Skipped => (String::new(), String::new(), String::new()),
    }
}

/// The most useful thing that is known about a crash.
fn crash_detail(record: &RunRecord) -> String {
    if !record.compile.ok {
        if record.compile.status < 0 {
            return "the compiler was killed, either by a signal or by the timeout".to_owned();
        }
        return record.compile.diagnostics.clone();
    }
    if record.execute.status < 0 {
        return "the program was killed, either by a signal or by the timeout".to_owned();
    }
    format!("the program exited with status {}", record.execute.status)
}

/// How much bigger the code is than the reference, as a ratio.
///
/// Returns `None` when either side has no measurement, which happens when the compile failed
/// or when the object format was not one the size reader understands. A missing ratio is left
/// missing rather than filled in with one, because one means parity and that is a claim.
#[must_use]
pub fn size_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    ratio(mine.compile.text_bytes, reference.compile.text_bytes)
}

/// How much slower the program is than the same program from the reference.
#[must_use]
pub fn speed_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    if !mine.execute.ok || !reference.execute.ok {
        return None;
    }
    ratio(mine.execute.micros, reference.execute.micros)
}

/// How many more instructions the program ran than the same program from the reference.
///
/// This is the run time question asked in a way the machine can answer twice and get the same
/// number. `None` on a machine that would not count, on a program that did not finish, and on
/// a report written before the corpus counted anything.
#[must_use]
pub fn instruction_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    if !mine.execute.ok || !reference.execute.ok {
        return None;
    }
    ratio(mine.execute.instructions?, reference.execute.instructions?)
}

/// How much longer the compiler took than the reference took.
#[must_use]
pub fn compile_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    ratio(mine.compile.micros, reference.compile.micros)
}

/// How much more memory the compiler needed than the reference needed.
///
/// `None` when either side was not measured, which is the whole run on any platform that
/// cannot look, per `crate::memory`.
#[must_use]
pub fn memory_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    ratio(mine.compile.peak_bytes?, reference.compile.peak_bytes?)
}

/// How much larger the executable on disk is than the reference's.
///
/// The whole file, padding and symbol table and all, which is deliberately not the number
/// `size_ratio` reports. Disk is what a build costs somebody, and code is what the optimizer
/// decided, and a report that publishes one of those as the other has answered the wrong
/// question.
#[must_use]
pub fn disk_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    ratio(mine.compile.bytes, reference.compile.bytes)
}

/// How much more initialized data the compiler put in the image than the reference did.
#[must_use]
pub fn data_ratio(mine: &RunRecord, reference: &RunRecord) -> Option<f64> {
    ratio(mine.compile.data_bytes, reference.compile.data_bytes)
}

fn ratio(mine: u64, reference: u64) -> Option<f64> {
    if mine == 0 || reference == 0 {
        return None;
    }
    Some(mine as f64 / reference as f64)
}

/// Whether a level is one the record is allowed to be compared at.
///
/// Code quality is judged at `-O2` against `-O2`, per spec 16. Comparing a size at `-Os`
/// against a size at `-O3` would be comparing two different questions.
#[must_use]
pub const fn comparable(mine: Level, reference: Level) -> bool {
    matches!(
        (mine, reference),
        (Level::O0, Level::O0)
            | (Level::O1, Level::O1)
            | (Level::O2, Level::O2)
            | (Level::O3, Level::O3)
            | (Level::Os, Level::Os)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        comparable, finding, instruction_ratio, judge, mentions_it, size_ratio, speed_ratio,
    };
    use corpus_model::{
        Axes, Case, Compile, Dialect, Execute, Expect, Facet, Level, RunRecord, Verdict,
    };

    fn running_case() -> Case {
        Case::new(
            Facet::ConstantFold,
            Axes::of([("type", "i32")]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Output("42\n".to_owned()),
        )
    }

    fn rejected_case() -> Case {
        Case::new(
            Facet::Frontend,
            Axes::of([("rejected", "duplicate-case")]),
            Dialect::C17,
            "int main(void) { return 0; }\n",
            Expect::Rejected("duplicate case".to_owned()),
        )
    }

    fn record_for(case: &Case, compile: Compile, execute: Execute) -> RunRecord {
        let mut record = RunRecord::skipped(case, "rucc", Level::O2);
        record.compile = compile;
        record.execute = execute;
        record
    }

    fn built(text_bytes: u64) -> Compile {
        Compile {
            ok: true,
            status: 0,
            micros: 1000,
            diagnostics: String::new(),
            bytes: text_bytes * 4,
            text_bytes,
            ..Compile::skipped()
        }
    }

    fn ran(output: &str, micros: u64) -> Execute {
        Execute {
            ok: true,
            status: 0,
            micros,
            repeats: 5,
            output: output.to_owned(),
            ..Execute::skipped()
        }
    }

    #[test]
    fn the_right_answer_is_a_pass_and_the_wrong_answer_is_the_verdict_that_turns_it_red() {
        let case = running_case();
        let good = record_for(&case, built(100), ran("42\n", 50));
        assert_eq!(judge(&case, &good), Verdict::Pass);
        assert!(finding(&case, &good, Verdict::Pass).is_none());

        let bad = record_for(&case, built(100), ran("41\n", 50));
        assert_eq!(judge(&case, &bad), Verdict::Wrong);
        let found = finding(&case, &bad, Verdict::Wrong).unwrap();
        assert_eq!(found.expected, "42\n");
        assert_eq!(found.actual, "41\n");
        assert_eq!(found.facet, Facet::ConstantFold);
    }

    #[test]
    fn a_diagnostic_and_a_crash_are_told_apart_because_they_are_found_in_different_places() {
        let case = running_case();
        let diagnosed = Compile {
            ok: false,
            status: 1,
            micros: 900,
            diagnostics: "error: something".to_owned(),
            bytes: 0,
            text_bytes: 0,
            ..Compile::skipped()
        };
        assert_eq!(
            judge(&case, &record_for(&case, diagnosed, Execute::skipped())),
            Verdict::Rejected
        );

        let killed = Compile {
            ok: false,
            status: -1,
            micros: 120_000_000,
            diagnostics: String::new(),
            bytes: 0,
            text_bytes: 0,
            ..Compile::skipped()
        };
        assert_eq!(judge(&case, &record_for(&case, killed, Execute::skipped())), Verdict::Crashed);
    }

    #[test]
    fn a_compiler_that_says_it_has_not_built_the_thing_yet_is_a_gap_and_not_a_bug() {
        let case = running_case();
        // rucc's wording, from the E0653 family, taken from a real run on x86-64 Linux.
        let admitted = Compile {
            ok: false,
            status: 1,
            micros: 900,
            diagnostics: "f.c:8:26: error: cannot generate code for 'main': no rule lowers a `trunc` producing a `i12` [E0653]\nf.c:8:26: note: this construct is not lowered yet".to_owned(),
            bytes: 0,
            text_bytes: 0,
            ..Compile::skipped()
        };
        let record = record_for(&case, admitted, Execute::skipped());
        let verdict = judge(&case, &record);
        assert_eq!(verdict, Verdict::Unimplemented);
        assert!(!verdict.is_failure(), "a declared gap does not turn the build red");
        assert!(verdict.is_gap());
        // It still gets a finding, because it is still something somebody has to do.
        let found = finding(&case, &record, verdict).expect("a gap is reported");
        assert_eq!(found.verdict, Verdict::Unimplemented);
        assert!(found.actual.contains("not lowered yet"));
    }

    #[test]
    fn an_ordinary_rejection_is_still_a_bug() {
        // The same shape without the admission. This is the compiler getting valid C wrong, and
        // nothing about the phrasing should let it into the quiet pile.
        let case = running_case();
        let refused = Compile {
            ok: false,
            status: 1,
            micros: 900,
            diagnostics: "f.c:3:1: error: expected a declaration".to_owned(),
            bytes: 0,
            text_bytes: 0,
            ..Compile::skipped()
        };
        assert_eq!(
            judge(&case, &record_for(&case, refused, Execute::skipped())),
            Verdict::Rejected
        );
    }

    #[test]
    fn the_admissions_are_the_words_compilers_actually_print() {
        assert!(super::admits_a_gap("note: this construct is not lowered yet"));
        assert!(super::admits_a_gap("sorry, unimplemented: non-trivial designated initializers"));
        assert!(super::admits_a_gap("SORRY, UNIMPLEMENTED: shouting"));
        assert!(!super::admits_a_gap("error: expected a declaration"));
        assert!(!super::admits_a_gap(""));
        // Not a guess about which features a compiler has. Only what it said about itself.
        assert!(!super::admits_a_gap("error: _BitInt is not a type"));
    }

    #[test]
    fn a_program_that_compiles_and_then_dies_is_a_crash_and_not_a_wrong_answer() {
        let case = running_case();
        let died = Execute {
            ok: false,
            status: -1,
            micros: 0,
            repeats: 5,
            output: String::new(),
            ..Execute::skipped()
        };
        let record = record_for(&case, built(100), died);
        assert_eq!(judge(&case, &record), Verdict::Crashed);
        let found = finding(&case, &record, Verdict::Crashed).unwrap();
        assert!(found.actual.contains("killed"));
    }

    #[test]
    fn a_case_that_must_not_compile_passes_only_when_the_right_mistake_was_caught() {
        let case = rejected_case();

        let caught = Compile {
            ok: false,
            status: 1,
            micros: 800,
            diagnostics: "case.c:5:5: error: duplicate case value".to_owned(),
            bytes: 0,
            text_bytes: 0,
            ..Compile::skipped()
        };
        assert_eq!(judge(&case, &record_for(&case, caught, Execute::skipped())), Verdict::Pass);

        let wrong_reason = Compile {
            ok: false,
            status: 1,
            micros: 800,
            diagnostics: "case.c:1:1: error: internal compiler error".to_owned(),
            bytes: 0,
            text_bytes: 0,
            ..Compile::skipped()
        };
        assert_eq!(
            judge(&case, &record_for(&case, wrong_reason, Execute::skipped())),
            Verdict::Wrong
        );

        let missed = record_for(&case, built(100), Execute::skipped());
        assert_eq!(judge(&case, &missed), Verdict::Accepted);
        let found = finding(&case, &missed, Verdict::Accepted).unwrap();
        assert!(found.summary.contains("not valid C"));
    }

    #[test]
    fn an_instruction_ratio_needs_a_count_from_both_sides_and_not_just_from_one() {
        let case = running_case();
        let mut mine = record_for(&case, built(120), ran("42\n", 200));
        let mut reference = record_for(&case, built(100), ran("42\n", 100));
        // Neither side counted, which is every machine without perf.
        assert_eq!(instruction_ratio(&mine, &reference), None);

        mine.execute.instructions = Some(90_000);
        // One side counted, which should still be nothing rather than a ratio against nought.
        assert_eq!(instruction_ratio(&mine, &reference), None);

        reference.execute.instructions = Some(100_000);
        assert_eq!(instruction_ratio(&mine, &reference), Some(0.9));

        // A program that did not finish did not retire the instructions of one that did.
        let died = record_for(&case, built(120), Execute::skipped());
        assert_eq!(instruction_ratio(&died, &reference), None);
    }

    #[test]
    fn the_expected_diagnostic_is_matched_without_regard_to_case_or_wording_around_it() {
        assert!(mentions_it("error: Duplicate Case value", "duplicate case"));
        assert!(mentions_it("error: 'x' undeclared (first use in this function)", "undeclared"));
        assert!(!mentions_it("error: expected ';'", "undeclared"));
        assert!(mentions_it("anything at all", ""));
    }

    #[test]
    fn a_ratio_is_left_missing_rather_than_filled_in_with_parity() {
        let case = running_case();
        let mine = record_for(&case, built(120), ran("42\n", 200));
        let reference = record_for(&case, built(100), ran("42\n", 100));
        assert_eq!(size_ratio(&mine, &reference), Some(1.2));
        assert_eq!(speed_ratio(&mine, &reference), Some(2.0));

        let failed = record_for(&case, Compile::skipped(), Execute::skipped());
        assert_eq!(size_ratio(&failed, &reference), None);
        assert_eq!(speed_ratio(&failed, &reference), None);
    }

    #[test]
    fn a_level_is_only_ever_compared_against_the_same_level() {
        assert!(comparable(Level::O2, Level::O2));
        assert!(!comparable(Level::Os, Level::O3));
        assert!(!comparable(Level::O0, Level::O2));
    }
}
