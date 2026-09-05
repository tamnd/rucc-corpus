//! What GCC says about its own work.
//!
//! GCC will tell you what it optimized, what it wanted to optimize and could not, and often
//! why, if you ask it with `-fopt-info`. Run against a corpus where every program was written
//! for one named transformation, that turns into the most useful reference data here: a
//! mature compiler's opinion, per program, about whether the transformation the program was
//! written for was available and whether it took it.
//!
//! It is reference data and not a pass or fail signal. GCC saying it missed something is not
//! a bug in GCC and GCC saying it optimized something is not a requirement on rucc. What it
//! is good for is deciding what to implement next, and for telling the difference between a
//! case rucc leaves alone because the pass is missing and a case rucc leaves alone because
//! there was nothing there to do.
//!
//! # The shape of a line
//!
//! ```text
//! case.c:14:19: optimized: loop vectorized using 16 byte vectors
//! case.c:9:3: missed: couldn't vectorize loop
//! case.c:6:12: note: ===== asm =====
//! ```
//!
//! The pass is not in the line. GCC prints the message the pass wrote and nothing about who
//! wrote it, so the pass has to be recognised from the wording. The table below is built from
//! the strings in the GCC tree and it is deliberately conservative: a message that is not
//! recognised gets an empty pass rather than a guessed one, because a wrong attribution in
//! reference data is worse than a missing one.

use corpus_model::Insight;

/// Turns whatever `-fopt-info` wrote into records.
///
/// Lines that are not opt-info lines are dropped. GCC mixes ordinary warnings into the same
/// stream when it is told to write to standard error, and a warning is not an insight.
#[must_use]
pub fn parse(text: &str) -> Vec<Insight> {
    text.lines().filter_map(parse_line).collect()
}

/// Reads one line, or decides it is not one of ours.
fn parse_line(line: &str) -> Option<Insight> {
    let line = line.trim_end();
    // Everything up to the first space is the position, and it has at least two colons in it
    // for the file and the line. Anything else is not an opt-info line.
    let (position, rest) = line.split_once(' ')?;
    let position = position.strip_suffix(':')?;
    // The position is `file:line` or `file:line:column`. Splitting from the front rather than
    // the back means the line number is always the field after the file name, so both shapes
    // are read the same way and a file name with a digit in it cannot be mistaken for one.
    let fields: Vec<&str> = position.split(':').collect();
    if fields.len() < 2 {
        return None;
    }
    let line_number = fields[1].parse::<u32>().ok()?;

    let (kind, message) = rest.split_once(':')?;
    let kind = kind.trim();
    if !matches!(kind, "optimized" | "missed" | "note") {
        return None;
    }
    let message = message.trim();
    if noise(kind, message) {
        return None;
    }
    let message = message.to_owned();
    Some(Insight {
        kind: kind.to_owned(),
        pass: classify(&message).to_owned(),
        line: line_number,
        message,
    })
}

/// Whether a line is about the harness rather than about the program.
///
/// Every case in this corpus gets its answer out through `printf`, and `printf` is declared
/// rather than defined, so GCC says two things about it in every single program. The inliner
/// says it cannot inline a function whose body it has not got, and the memory analysis says
/// the call clobbers memory. Both are true and neither has anything to do with the
/// transformation the case was written for.
///
/// Left in, they drown everything else. One small program produced 128 missed lines, 68 of
/// them about `printf`, and the inliner one carries the word `inlinable`, so it was also being
/// attributed to the inline facet in every case in the corpus. Reference data that says a
/// constant folding case missed an inlining opportunity ninety times is worse than no
/// reference data.
///
/// Only `missed` lines are dropped. If GCC ever says it optimized something about `printf`,
/// such as turning it into a `puts`, that is real work it did on the program and it stays.
fn noise(kind: &str, message: &str) -> bool {
    kind == "missed" && message.contains("printf")
}

/// Guesses which pass wrote a message, from the wording.
///
/// The names on the right are the corpus facet names, not the GCC pass names, because the
/// point of attributing a message is to line it up with the facet the case belongs to.
#[must_use]
pub fn classify(message: &str) -> &'static str {
    const TABLE: &[(&str, &str)] = &[
        ("inlin", "inline"),
        ("vectoriz", "loop-idiom"),
        ("unrolled", "loop-unroll"),
        ("unrolling", "loop-unroll"),
        ("peeled", "loop-unroll"),
        ("completely unroll", "loop-unroll"),
        ("versioning", "loop-unswitch"),
        ("unswitch", "loop-unswitch"),
        ("loop nest", "loop-restructure"),
        ("interchang", "loop-restructure"),
        ("distribut", "loop-restructure"),
        ("loop turned into non-loop", "loop-deletion"),
        ("basic block part vectorized", "loop-idiom"),
        ("converted to a builtin", "loop-idiom"),
        ("memset", "loop-idiom"),
        ("memcpy", "loop-idiom"),
        ("devirtualiz", "devirtualize"),
        ("indirect call", "devirtualize"),
        ("cloned", "constant-args"),
        ("clone", "constant-args"),
        ("propagat", "constant-propagation"),
        ("tail call", "tail-call"),
        ("tail recursion", "tail-call"),
        ("sinking", "code-motion"),
        ("hoist", "code-motion"),
        ("store motion", "code-motion"),
        ("invariant", "loop-invariant"),
        ("if-conver", "if-conversion"),
        ("switch", "switch-lowering"),
        ("alias", "alias-analysis"),
        ("scalarizing", "scalar-replacement"),
        ("sra", "scalar-replacement"),
        ("removing load", "load-forwarding"),
        ("dead store", "dead-store"),
        ("removing basic block", "unreachable-code"),
        ("prefetch", "scheduling"),
        ("register", "register-alloc"),
        ("splitting", "block-layout"),
    ];
    let lowered = message.to_ascii_lowercase();
    for &(needle, facet) in TABLE {
        if lowered.contains(needle) {
            return facet;
        }
    }
    ""
}

/// The flags that ask GCC to write its opinions to a file.
///
/// To a file rather than to standard error, because standard error is where the diagnostics
/// live and the harness compares those against the text a rejected case expects. Mixing tens
/// of notes into that stream would make the comparison useless.
#[must_use]
pub fn flags(path: &str) -> Vec<String> {
    vec![format!("-fopt-info-all={path}")]
}

#[cfg(test)]
mod tests {
    use super::{classify, flags, parse};

    #[test]
    fn the_three_kinds_of_line_are_read_and_anything_else_is_dropped() {
        let text = "\
case.c:14:19: optimized: loop vectorized using 16 byte vectors
case.c:9:3: missed: couldn't vectorize loop
case.c:6: note: ===== analyze_loop_nest =====
case.c:3:1: warning: unused variable 'x' [-Wunused-variable]
gcc: fatal error: no input files
";
        let found = parse(text);
        assert_eq!(found.len(), 3);
        assert_eq!(found[0].kind, "optimized");
        assert_eq!(found[0].line, 14);
        assert_eq!(found[0].message, "loop vectorized using 16 byte vectors");
        assert_eq!(found[1].kind, "missed");
        assert_eq!(found[1].line, 9);
        assert_eq!(found[2].kind, "note");
        assert_eq!(found[2].line, 6);
    }

    #[test]
    fn a_message_is_attributed_to_a_facet_only_when_the_wording_says_so() {
        assert_eq!(classify("Inlining helper into main"), "inline");
        assert_eq!(classify("loop vectorized using 16 byte vectors"), "loop-idiom");
        assert_eq!(classify("loop unrolled 7 times"), "loop-unroll");
        assert_eq!(classify("converted to a builtin memset"), "loop-idiom");
        assert_eq!(classify("Semantic equality hit"), "");
        assert_eq!(classify(""), "");
    }

    #[test]
    fn the_attribution_is_the_facet_name_and_not_the_gcc_pass_name() {
        // A message attributed to something the corpus does not have a facet for would join
        // nothing to nothing in the report, so every name in the table has to be one.
        for message in ["Inlining a into b", "loop unrolled 4 times", "dead store removed"] {
            let facet = classify(message);
            assert!(
                corpus_model::Facet::parse(facet).is_some(),
                "{message} was attributed to {facet}, which is not a facet"
            );
        }
    }

    #[test]
    fn the_opinions_are_asked_for_in_a_file_of_their_own() {
        let asked = flags("/tmp/case.opt");
        assert_eq!(asked, ["-fopt-info-all=/tmp/case.opt"]);
    }

    #[test]
    fn what_gcc_says_about_printf_is_dropped_because_every_program_here_calls_printf() {
        let text = "\
case.c:7:5: missed:   not inlinable: main/1 -> printf/2, function body not available
case.c:7:5: missed: statement clobbers memory: printf (\"%lld\\n\", 2);
case.c:7:5: optimized: printf turned into puts
case.c:4:3: missed: couldn't vectorize loop
";
        let found = parse(text);
        assert_eq!(found.len(), 2, "{found:?}");
        // The one GCC says it did is kept, because that is work it did on the program.
        assert_eq!(found[0].kind, "optimized");
        assert_eq!(found[1].message, "couldn't vectorize loop");
    }

    #[test]
    fn nothing_from_an_empty_run_is_read_as_an_insight() {
        assert!(parse("").is_empty());
        assert!(parse("\n\n").is_empty());
    }
}
