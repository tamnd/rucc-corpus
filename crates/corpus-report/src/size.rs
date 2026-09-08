//! How big the thing being measured is, written the way a person would say it.
//!
//! Every other number in this crate is a cost, and a cost is unreadable on its own. Four hundred
//! milliseconds to compile is quick for ten thousand lines and slow for two hundred, and a reader
//! who cannot see which one they are holding has been handed a number they can do nothing with.
//! The corpus is generated, so nobody arrives with the feel for its size that they would have for
//! a project they had checked out, and these are the words that give them one.
//!
//! Two rules, and they are the reason this is a module rather than three calls to `format!`.
//!
//! **A count of lines gets separators.** `30212` and `302120` look the same at a glance and are an
//! order of magnitude apart, and the glance is all most of these numbers get.
//!
//! **Nothing here is ever divided into anything.** Lines per second is a throughput measurement,
//! and a throughput measurement taken as a side effect of a correctness run, over generated
//! programs chosen for what they exercise rather than for what they cost, would be quoted by
//! somebody eventually. It is a denominator and never a score.

use corpus_model::Source;

/// A size written as one phrase, for the middle of a sentence.
#[must_use]
pub fn line(source: Source) -> String {
    format!("{} of C, {}", lines_of(source.lines), bytes(source.bytes))
}

/// A count of lines, with separators and with the noun agreeing with it.
///
/// The agreement matters because a corpus of one program is what every test fixture and every
/// first run looks like, and a report that opens by saying 1 lines is a report a reader stops
/// trusting on the strength of a sentence that had nothing to do with the compiler.
#[must_use]
pub fn lines_of(lines: u32) -> String {
    let noun = if lines == 1 { "line" } else { "lines" };
    format!("{} {noun}", thousands(u64::from(lines)))
}

/// A line count for a table cell, or a plain statement that nothing measured it.
///
/// The two are told apart rather than both printed as nought, because a column of noughts reads
/// as a corpus that has lost its programs and that is a different thing from a run whose records
/// were written before anybody counted.
#[must_use]
pub fn cell(source: Source) -> String {
    if source.measured() { thousands(u64::from(source.lines)) } else { "not measured".to_owned() }
}

/// A count with separators in it.
#[must_use]
pub fn thousands(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (seen, digit) in digits.chars().enumerate() {
        if seen > 0 && (digits.len() - seen) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// A byte count in the unit a person would use for it.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn bytes(value: u64) -> String {
    const KIB: f64 = 1024.0;
    let value = value as f64;
    if value < KIB {
        return format!("{value:.0} B");
    }
    if value < KIB * KIB {
        return format!("{:.1} KiB", value / KIB);
    }
    format!("{:.1} MiB", value / (KIB * KIB))
}

#[cfg(test)]
mod tests {
    use super::{bytes, line, lines_of, thousands};
    use corpus_model::Source;

    #[test]
    fn a_count_a_person_has_to_read_at_a_glance_has_separators_in_it() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1_000), "1,000");
        assert_eq!(thousands(30_212), "30,212");
        assert_eq!(thousands(1_234_567), "1,234,567");
    }

    #[test]
    fn a_byte_count_is_given_in_the_unit_somebody_would_say_it_in() {
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(2_048), "2.0 KiB");
        assert_eq!(bytes(1_572_864), "1.5 MiB");
    }

    #[test]
    fn a_size_reads_as_a_phrase_rather_than_as_a_pair_of_figures() {
        let source = Source { lines: 30_212, bytes: 1_153_434 };
        assert_eq!(line(source), "30,212 lines of C, 1.1 MiB");
    }

    #[test]
    fn a_corpus_of_one_program_is_not_described_as_one_lines() {
        assert_eq!(lines_of(1), "1 line");
        assert_eq!(lines_of(0), "0 lines");
        assert_eq!(lines_of(2), "2 lines");
        assert_eq!(line(Source { lines: 1, bytes: 28 }), "1 line of C, 28 B");
    }
}
