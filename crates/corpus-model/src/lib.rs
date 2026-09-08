//! The vocabulary the corpus is written in.
//!
//! Everything else in this workspace depends on this crate and this crate depends on nothing.
//! It holds the shape of a case, the shape of a run record, the facet list that ties the two
//! together, and the JSON writer that puts them on disk. The generator produces these types,
//! the harness fills them in, and the reporters read them.
//!
//! # Why the types live apart from the code that makes them
//!
//! The machine report is meant to be read by tools that are not in this repository. Keeping
//! the format in one small crate with no dependencies means the format can be depended on
//! without dragging in a process runner and a C generator, and it means a change to the
//! format is a change to one file that reviewers can see all of.
//!
//! # Compatibility
//!
//! [`SCHEMA_VERSION`] appears in every file written. Adding a field is a minor change and
//! readers are expected to ignore fields they do not know. Removing or renaming one is a
//! major change and bumps the version.

#![forbid(unsafe_code)]

pub mod case;
pub mod facet;
pub mod json;
pub mod record;
pub mod sha256;

pub use case::{Axes, Case, Dialect, Expect, Manifest, program_path};
pub use facet::{Facet, Phase};
pub use json::Json;
pub use record::{
    Compile, Execute, Finding, Insight, Level, RunRecord, Source, Toolchain, Verdict,
};

/// The version stamped into every file this workspace writes.
///
/// A reader that finds a version it does not know should say so and stop rather than guess.
pub const SCHEMA_VERSION: &str = "1";

/// The milestone this corpus is evidence for.
pub const MILESTONE: &str = "M4";

#[cfg(test)]
mod tests {
    use super::{Facet, Level, SCHEMA_VERSION};

    #[test]
    fn the_schema_version_is_a_plain_number_so_a_reader_can_compare_it() {
        assert!(SCHEMA_VERSION.parse::<u32>().is_ok());
    }

    #[test]
    fn the_re_exports_are_the_names_the_rest_of_the_workspace_uses() {
        assert_eq!(Facet::Baseline.name(), "baseline");
        assert_eq!(Level::O2.flag(), "-O2");
    }
}
