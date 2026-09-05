//! A JSON value, a writer and a parser.
//!
//! The machine report is a contract that other tools read, so the format is written here
//! rather than delegated. Two properties matter and both are cheap to get by hand and awkward
//! to get from a derive macro. Object keys come out in insertion order, which makes a report
//! diffable against yesterday's. And floats come out with a fixed number of decimal places,
//! which stops a timing that differs in the seventeenth digit from showing up as a change.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// A JSON value.
///
/// Objects keep their keys in the order they were inserted rather than sorting them, because
/// a report is read by people as well as by machines and a summary whose fields move around
/// between runs is a report nobody trusts.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    /// The null literal.
    Null,
    /// A boolean.
    Bool(bool),
    /// A number. Integers and floats are one case, and the writer decides how to spell it.
    Number(f64),
    /// A string.
    String(String),
    /// An array.
    Array(Vec<Json>),
    /// An object, in insertion order.
    Object(Vec<(String, Json)>),
}

impl Json {
    /// An object built from pairs, in the order given.
    #[must_use]
    pub fn object<const N: usize>(fields: [(&str, Json); N]) -> Self {
        Self::Object(fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
    }

    /// A string value from anything that can become one.
    #[must_use]
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// An integer value.
    ///
    /// Takes an `i64` and stores an `f64`, which is exact for every integer a corpus run
    /// produces. Sizes, counts and nanosecond timings all sit far below the point where a
    /// double stops counting by ones.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn int(value: i64) -> Self {
        Self::Number(value as f64)
    }

    /// An array built from an iterator.
    pub fn array(values: impl IntoIterator<Item = Self>) -> Self {
        Self::Array(values.into_iter().collect())
    }

    /// The value at a key, if this is an object that has one.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// This value as a string, if it is one.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// This value as a number, if it is one.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// This value as a slice, if it is an array.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }

    /// This value as its fields in insertion order, if it is an object.
    ///
    /// In insertion order rather than sorted, because the order a report was written in is the
    /// order somebody chose to have it read in, and a reader that reordered it would be
    /// showing something other than what is in the file.
    #[must_use]
    pub fn as_object(&self) -> Option<&[(String, Self)]> {
        match self {
            Self::Object(fields) => Some(fields),
            _ => None,
        }
    }

    /// The value written out on one line, which is what a JSON Lines record is.
    #[must_use]
    pub fn to_line(&self) -> String {
        let mut out = String::new();
        write_compact(self, &mut out);
        out
    }

    /// The value written out indented, which is what a report file is.
    #[must_use]
    pub fn to_pretty(&self) -> String {
        let mut out = String::new();
        write_pretty(self, 0, &mut out);
        out.push('\n');
        out
    }
}

/// Writes the value with no spaces at all.
fn write_compact(value: &Json, out: &mut String) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Bool(true) => out.push_str("true"),
        Json::Bool(false) => out.push_str("false"),
        Json::Number(n) => out.push_str(&number(*n)),
        Json::String(s) => quote(s, out),
        Json::Array(items) => {
            out.push('[');
            for (at, item) in items.iter().enumerate() {
                if at > 0 {
                    out.push(',');
                }
                write_compact(item, out);
            }
            out.push(']');
        }
        Json::Object(fields) => {
            out.push('{');
            for (at, (key, item)) in fields.iter().enumerate() {
                if at > 0 {
                    out.push(',');
                }
                quote(key, out);
                out.push(':');
                write_compact(item, out);
            }
            out.push('}');
        }
    }
}

/// Writes the value with two spaces per level.
fn write_pretty(value: &Json, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let inner = "  ".repeat(depth + 1);
    match value {
        Json::Array(items) if !items.is_empty() => {
            out.push_str("[\n");
            for (at, item) in items.iter().enumerate() {
                out.push_str(&inner);
                write_pretty(item, depth + 1, out);
                if at + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&pad);
            out.push(']');
        }
        Json::Object(fields) if !fields.is_empty() => {
            out.push_str("{\n");
            for (at, (key, item)) in fields.iter().enumerate() {
                out.push_str(&inner);
                quote(key, out);
                out.push_str(": ");
                write_pretty(item, depth + 1, out);
                if at + 1 < fields.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&pad);
            out.push('}');
        }
        other => write_compact(other, out),
    }
}

/// A number, spelled as an integer when it is one and to three decimal places when it is not.
///
/// Three places is enough for a ratio and for a millisecond timing, and few enough that two
/// runs of the same work on the same machine produce the same text most of the time. A report
/// that differs only in floating point noise is a report whose diff nobody reads.
fn number(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 9.0e15 {
        format!("{value:.0}")
    } else if value.is_finite() {
        let mut text = format!("{value:.3}");
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.push('0');
        }
        text
    } else {
        // JSON has no infinity and no NaN. A timing that came out as one of those is a bug
        // upstream of here, and writing null says so rather than writing text no parser takes.
        "null".to_owned()
    }
}

/// Writes a quoted, escaped string.
fn quote(text: &str, out: &mut String) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// What went wrong while reading JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Where in the input, counted in bytes from the start.
    pub at: usize,
    /// What was expected there.
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at byte {}: {}", self.at, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Reads a JSON value.
///
/// The parser exists so that `corpus diff` can read yesterday's report. It is not a general
/// purpose one and it does not need to be: the only JSON it ever sees is JSON this crate wrote.
///
/// # Errors
///
/// When the text is not one complete JSON value.
pub fn parse(text: &str) -> Result<Json, ParseError> {
    let bytes = text.as_bytes();
    let mut at = 0;
    let value = parse_value(bytes, &mut at)?;
    skip_space(bytes, &mut at);
    if at != bytes.len() {
        return Err(ParseError { at, message: "trailing text after the value".to_owned() });
    }
    Ok(value)
}

/// Reads a file of JSON Lines, one value per non-empty line.
///
/// # Errors
///
/// When any line is not one complete JSON value, naming the line number.
pub fn parse_lines(text: &str) -> Result<Vec<Json>, ParseError> {
    let mut values = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match parse(line) {
            Ok(value) => values.push(value),
            Err(err) => {
                return Err(ParseError {
                    at: err.at,
                    message: format!("on line {}: {}", n + 1, err.message),
                });
            }
        }
    }
    Ok(values)
}

fn skip_space(bytes: &[u8], at: &mut usize) {
    while *at < bytes.len() && bytes[*at].is_ascii_whitespace() {
        *at += 1;
    }
}

fn parse_value(bytes: &[u8], at: &mut usize) -> Result<Json, ParseError> {
    skip_space(bytes, at);
    let Some(&byte) = bytes.get(*at) else {
        return Err(ParseError { at: *at, message: "a value, and the text ended".to_owned() });
    };
    match byte {
        b'n' => literal(bytes, at, "null", Json::Null),
        b't' => literal(bytes, at, "true", Json::Bool(true)),
        b'f' => literal(bytes, at, "false", Json::Bool(false)),
        b'"' => parse_string(bytes, at).map(Json::String),
        b'[' => parse_array(bytes, at),
        b'{' => parse_object(bytes, at),
        b'-' | b'0'..=b'9' => parse_number(bytes, at),
        _ => Err(ParseError { at: *at, message: "a value".to_owned() }),
    }
}

fn literal(bytes: &[u8], at: &mut usize, word: &str, value: Json) -> Result<Json, ParseError> {
    if bytes[*at..].starts_with(word.as_bytes()) {
        *at += word.len();
        Ok(value)
    } else {
        Err(ParseError { at: *at, message: format!("the literal `{word}`") })
    }
}

fn parse_number(bytes: &[u8], at: &mut usize) -> Result<Json, ParseError> {
    let start = *at;
    if bytes.get(*at) == Some(&b'-') {
        *at += 1;
    }
    while at_digit_or_point(bytes, *at) {
        *at += 1;
    }
    let text = std::str::from_utf8(&bytes[start..*at])
        .map_err(|_| ParseError { at: start, message: "a number".to_owned() })?;
    text.parse::<f64>()
        .map(Json::Number)
        .map_err(|_| ParseError { at: start, message: format!("a number, and got `{text}`") })
}

fn at_digit_or_point(bytes: &[u8], at: usize) -> bool {
    matches!(bytes.get(at), Some(b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-'))
}

fn parse_string(bytes: &[u8], at: &mut usize) -> Result<String, ParseError> {
    *at += 1;
    let mut out = String::new();
    loop {
        let Some(&byte) = bytes.get(*at) else {
            return Err(ParseError { at: *at, message: "a closing quote".to_owned() });
        };
        *at += 1;
        match byte {
            b'"' => return Ok(out),
            b'\\' => {
                let Some(&escape) = bytes.get(*at) else {
                    return Err(ParseError { at: *at, message: "an escape".to_owned() });
                };
                *at += 1;
                match escape {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'b' => out.push('\u{8}'),
                    b'f' => out.push('\u{c}'),
                    b'u' => {
                        let hex = bytes
                            .get(*at..*at + 4)
                            .and_then(|h| std::str::from_utf8(h).ok())
                            .and_then(|h| u32::from_str_radix(h, 16).ok())
                            .ok_or(ParseError {
                                at: *at,
                                message: "four hex digits".to_owned(),
                            })?;
                        *at += 4;
                        out.push(char::from_u32(hex).unwrap_or('\u{fffd}'));
                    }
                    _ => return Err(ParseError { at: *at, message: "a known escape".to_owned() }),
                }
            }
            _ => {
                // The input is a `&str`, so the bytes are valid UTF-8 and a multi-byte
                // sequence can be copied through byte by byte only if the boundary is kept.
                // Finding the char at this position and taking its length does that.
                let rest = std::str::from_utf8(&bytes[*at - 1..]).map_err(|_| ParseError {
                    at: *at,
                    message: "valid UTF-8".to_owned(),
                })?;
                let ch = rest.chars().next().unwrap_or('\u{fffd}');
                out.push(ch);
                *at += ch.len_utf8() - 1;
            }
        }
    }
}

fn parse_array(bytes: &[u8], at: &mut usize) -> Result<Json, ParseError> {
    *at += 1;
    let mut items = Vec::new();
    skip_space(bytes, at);
    if bytes.get(*at) == Some(&b']') {
        *at += 1;
        return Ok(Json::Array(items));
    }
    loop {
        items.push(parse_value(bytes, at)?);
        skip_space(bytes, at);
        match bytes.get(*at) {
            Some(b',') => *at += 1,
            Some(b']') => {
                *at += 1;
                return Ok(Json::Array(items));
            }
            _ => return Err(ParseError { at: *at, message: "a comma or a bracket".to_owned() }),
        }
    }
}

fn parse_object(bytes: &[u8], at: &mut usize) -> Result<Json, ParseError> {
    *at += 1;
    let mut fields = Vec::new();
    skip_space(bytes, at);
    if bytes.get(*at) == Some(&b'}') {
        *at += 1;
        return Ok(Json::Object(fields));
    }
    loop {
        skip_space(bytes, at);
        if bytes.get(*at) != Some(&b'"') {
            return Err(ParseError { at: *at, message: "a key".to_owned() });
        }
        let key = parse_string(bytes, at)?;
        skip_space(bytes, at);
        if bytes.get(*at) != Some(&b':') {
            return Err(ParseError { at: *at, message: "a colon".to_owned() });
        }
        *at += 1;
        fields.push((key, parse_value(bytes, at)?));
        skip_space(bytes, at);
        match bytes.get(*at) {
            Some(b',') => *at += 1,
            Some(b'}') => {
                *at += 1;
                return Ok(Json::Object(fields));
            }
            _ => return Err(ParseError { at: *at, message: "a comma or a brace".to_owned() }),
        }
    }
}

/// Counts how many times each string appears, which several reports need.
#[must_use]
pub fn tally(items: impl IntoIterator<Item = String>) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::{Json, parse, parse_lines};

    #[test]
    fn an_object_keeps_the_order_its_fields_were_given_in() {
        let value = Json::object([
            ("zebra", Json::int(1)),
            ("apple", Json::int(2)),
            ("mango", Json::int(3)),
        ]);
        assert_eq!(value.to_line(), r#"{"zebra":1,"apple":2,"mango":3}"#);
    }

    #[test]
    fn an_integer_is_written_without_a_decimal_point_and_a_ratio_with_three() {
        assert_eq!(Json::int(42).to_line(), "42");
        assert_eq!(Json::Number(1.5).to_line(), "1.5");
        assert_eq!(Json::Number(1.0 / 3.0).to_line(), "0.333");
        assert_eq!(Json::Number(-0.0).to_line(), "-0");
    }

    #[test]
    fn a_number_that_is_not_finite_becomes_null_rather_than_text_no_parser_takes() {
        assert_eq!(Json::Number(f64::NAN).to_line(), "null");
        assert_eq!(Json::Number(f64::INFINITY).to_line(), "null");
    }

    #[test]
    fn a_string_with_control_characters_survives_a_round_trip() {
        let awkward = "a\"b\\c\nd\te\u{1}f\u{2764}g";
        let written = Json::string(awkward).to_line();
        assert_eq!(parse(&written).unwrap(), Json::string(awkward));
    }

    #[test]
    fn every_shape_survives_a_round_trip_through_both_writers() {
        let value = Json::object([
            ("null", Json::Null),
            ("yes", Json::Bool(true)),
            ("no", Json::Bool(false)),
            ("empty_array", Json::Array(Vec::new())),
            ("empty_object", Json::Object(Vec::new())),
            (
                "nested",
                Json::array([
                    Json::int(-7),
                    Json::Number(2.25),
                    Json::object([("deep", Json::array([Json::string("x")]))]),
                ]),
            ),
        ]);
        assert_eq!(parse(&value.to_line()).unwrap(), value);
        assert_eq!(parse(&value.to_pretty()).unwrap(), value);
    }

    #[test]
    fn the_pretty_writer_indents_by_two_and_the_compact_one_uses_no_spaces() {
        let value = Json::object([("a", Json::array([Json::int(1)]))]);
        assert_eq!(value.to_pretty(), "{\n  \"a\": [\n    1\n  ]\n}\n");
        assert_eq!(value.to_line(), r#"{"a":[1]}"#);
    }

    #[test]
    fn trailing_text_is_refused_rather_than_ignored() {
        assert!(parse("{} {}").is_err());
        assert!(parse("[1,]").is_err());
        assert!(parse("").is_err());
    }

    #[test]
    fn json_lines_skips_blank_lines_and_names_the_line_that_broke() {
        let good = "{\"a\":1}\n\n{\"a\":2}\n";
        assert_eq!(parse_lines(good).unwrap().len(), 2);
        let bad = "{\"a\":1}\nnot json\n";
        let err = parse_lines(bad).unwrap_err();
        assert!(err.message.contains("line 2"), "{err}");
    }
}
