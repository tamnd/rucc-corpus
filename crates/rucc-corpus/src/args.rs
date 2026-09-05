//! Reading the command line.
//!
//! Written out by hand, like everything else here, because the whole repository has no
//! external dependencies and a command line parser is the easiest thing in the world to
//! write badly and then never think about again. Four subcommands and twenty flags is not
//! enough to justify pulling in a crate that has to be audited, pinned and upgraded.
//!
//! The rules are the boring ones. A flag is `--name value` or `--name=value`. A flag that
//! takes a list may be repeated. An unknown flag is an error rather than something ignored,
//! because a typo that is silently ignored is a run that quietly did the wrong thing.

use std::collections::BTreeMap;

/// A parsed command line.
#[derive(Debug, Clone, Default)]
pub(crate) struct Args {
    /// The subcommand, or empty when there was none.
    pub(crate) command: String,
    /// Every flag, in the order it first appeared.
    values: BTreeMap<String, Vec<String>>,
    /// Everything that was not a flag.
    pub(crate) rest: Vec<String>,
}

impl Args {
    /// Reads the arguments the program was started with.
    ///
    /// # Errors
    ///
    /// When a flag is given without the value it needs.
    pub(crate) fn parse(raw: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut args = Self::default();
        let mut items = raw.into_iter().peekable();
        if let Some(first) = items.peek() {
            if !first.starts_with('-') {
                args.command = items.next().unwrap_or_default();
            }
        }
        while let Some(item) = items.next() {
            let Some(name) = item.strip_prefix("--") else {
                args.rest.push(item);
                continue;
            };
            if let Some((name, value)) = name.split_once('=') {
                args.values.entry(name.to_owned()).or_default().push(value.to_owned());
                continue;
            }
            // A flag with no value is a switch. A flag whose next argument is another flag is
            // also a switch, so `--keep --jobs 4` does what it looks like it does.
            let is_switch = items.peek().is_none_or(|next| next.starts_with("--"));
            if is_switch {
                args.values.entry(name.to_owned()).or_default().push("true".to_owned());
            } else {
                let value = items.next().unwrap_or_default();
                args.values.entry(name.to_owned()).or_default().push(value);
            }
        }
        Ok(args)
    }

    /// Whether a switch was given.
    #[must_use]
    pub(crate) fn flag(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// The last value given for a flag.
    #[must_use]
    pub(crate) fn value(&self, name: &str) -> Option<&str> {
        self.values.get(name)?.last().map(String::as_str)
    }

    /// The last value given for a flag, or a default.
    #[must_use]
    pub(crate) fn value_or<'a>(&'a self, name: &str, fallback: &'a str) -> &'a str {
        self.value(name).unwrap_or(fallback)
    }

    /// Every value given for a flag, in order.
    #[must_use]
    pub(crate) fn values(&self, name: &str) -> &[String] {
        self.values.get(name).map_or(&[], Vec::as_slice)
    }

    /// A flag read as a number.
    ///
    /// # Errors
    ///
    /// When the value is not a number.
    pub(crate) fn number(&self, name: &str) -> Result<Option<usize>, String> {
        match self.value(name) {
            None => Ok(None),
            Some(text) => text
                .parse()
                .map(Some)
                .map_err(|_| format!("--{name} wants a number, and {text} is not one")),
        }
    }

    /// Rejects any flag that is not in the list.
    ///
    /// # Errors
    ///
    /// When an unknown flag was given, with the nearest known one suggested.
    pub(crate) fn only(&self, known: &[&str]) -> Result<(), String> {
        for name in self.values.keys() {
            if known.contains(&name.as_str()) {
                continue;
            }
            let suggestion = nearest(name, known)
                .map(|near| format!(", did you mean --{near}"))
                .unwrap_or_default();
            return Err(format!("--{name} is not a flag of this command{suggestion}"));
        }
        Ok(())
    }
}

/// The known flag that is closest to what was typed, when one is close enough.
fn nearest<'a>(typed: &str, known: &[&'a str]) -> Option<&'a str> {
    let mut best: Option<(&str, usize)> = None;
    for candidate in known {
        let distance = edit_distance(typed, candidate);
        if distance > typed.len().div_ceil(2) {
            continue;
        }
        if best.is_none_or(|(_, seen)| distance < seen) {
            best = Some((candidate, distance));
        }
    }
    best.map(|(name, _)| name)
}

/// The ordinary edit distance, on characters.
fn edit_distance(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let mut row: Vec<usize> = (0..=right.len()).collect();
    for (down, from) in left.iter().enumerate() {
        let mut previous = row[0];
        row[0] = down + 1;
        for (across, to) in right.iter().enumerate() {
            let held = row[across + 1];
            row[across + 1] = if from == to {
                previous
            } else {
                previous.min(row[across]).min(row[across + 1]) + 1
            };
            previous = held;
        }
    }
    row[right.len()]
}

#[cfg(test)]
mod tests {
    use super::Args;

    fn parse(line: &str) -> Args {
        Args::parse(line.split_whitespace().map(str::to_owned)).unwrap()
    }

    #[test]
    fn the_first_word_is_the_command_and_the_flags_follow_it() {
        let args = parse("run --toolchain gcc-16 --jobs 8");
        assert_eq!(args.command, "run");
        assert_eq!(args.value("toolchain"), Some("gcc-16"));
        assert_eq!(args.number("jobs").unwrap(), Some(8));
    }

    #[test]
    fn a_flag_can_be_written_either_way_round_the_equals_sign() {
        let args = parse("run --jobs=8 --out=reports");
        assert_eq!(args.number("jobs").unwrap(), Some(8));
        assert_eq!(args.value("out"), Some("reports"));
    }

    #[test]
    fn a_flag_that_can_be_given_twice_keeps_both() {
        let args = parse("run --toolchain gcc-16 --toolchain rucc=./rucc");
        assert_eq!(args.values("toolchain"), ["gcc-16", "rucc=./rucc"]);
        // The single value reader gives the last one, which is what a person overriding a
        // default on the command line expects.
        assert_eq!(args.value("toolchain"), Some("rucc=./rucc"));
    }

    #[test]
    fn a_switch_before_another_flag_is_still_a_switch() {
        let args = parse("run --keep --jobs 4");
        assert!(args.flag("keep"));
        assert_eq!(args.number("jobs").unwrap(), Some(4));
        assert!(!args.flag("quiet"));
    }

    #[test]
    fn a_switch_at_the_end_is_a_switch() {
        let args = parse("run --keep");
        assert!(args.flag("keep"));
    }

    #[test]
    fn a_flag_that_wants_a_number_and_got_a_word_says_so() {
        let args = parse("run --jobs lots");
        let error = args.number("jobs").unwrap_err();
        assert!(error.contains("--jobs wants a number"));
    }

    #[test]
    fn a_typo_in_a_flag_is_an_error_with_the_flag_that_was_probably_meant() {
        let args = parse("run --toolchian gcc-16");
        let error = args.only(&["toolchain", "jobs"]).unwrap_err();
        assert!(error.contains("--toolchian is not a flag"), "{error}");
        assert!(error.contains("did you mean --toolchain"), "{error}");
    }

    #[test]
    fn a_flag_that_looks_like_nothing_known_is_still_an_error_but_without_a_guess() {
        let args = parse("run --zzzzzzzz 1");
        let error = args.only(&["toolchain", "jobs"]).unwrap_err();
        assert!(error.contains("is not a flag"));
        assert!(!error.contains("did you mean"), "{error}");
    }

    #[test]
    fn no_arguments_at_all_is_no_command_rather_than_an_error() {
        let args = Args::parse(Vec::new()).unwrap();
        assert_eq!(args.command, "");
        assert!(args.rest.is_empty());
    }
}
