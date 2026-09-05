//! A case: one C program, what it is for, and what it is supposed to do.

use crate::Json;
use crate::facet::{Facet, Phase};
use crate::sha256;

/// Which C the program is written in.
///
/// The dialect is a property of the case rather than a flag on the run, because a program that
/// uses `constexpr` cannot be compiled as C17 at all. Mixing the two on one command line
/// produces a diagnostic and not a result, and a corpus that cannot tell those apart is a
/// corpus that reports a language version mismatch as a compiler bug.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dialect {
    /// C17, which is the floor. Every toolchain under test accepts it.
    C17,
    /// C23. The modern constructs live here and the generator gates them behind it.
    C23,
}

impl Dialect {
    /// Every dialect, oldest first.
    pub const ALL: &'static [Self] = &[Self::C17, Self::C23];

    /// The name used in a record and on the command line.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::C17 => "c17",
            Self::C23 => "c23",
        }
    }

    /// What to pass after `-std=`.
    #[must_use]
    pub const fn std_flag(self) -> &'static str {
        match self {
            Self::C17 => "-std=c17",
            Self::C23 => "-std=c23",
        }
    }

    /// The dialect with this name.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|d| d.name() == name)
    }
}

impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The point in the axis space a generated case came from.
///
/// Every field is the name of a point on one axis, kept as a string rather than an enum so
/// that the generator can add an axis without every consumer of a stored record needing a new
/// variant. The generator owns the vocabulary and the report only groups by it.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Axes {
    /// The pairs, in the order the generator names its axes.
    pub points: Vec<(String, String)>,
}

impl Axes {
    /// Axes from a list of pairs.
    #[must_use]
    pub fn of<const N: usize>(points: [(&str, &str); N]) -> Self {
        Self {
            points: points
                .into_iter()
                .map(|(axis, point)| (axis.to_owned(), point.to_owned()))
                .collect(),
        }
    }

    /// The point on one axis.
    #[must_use]
    pub fn get(&self, axis: &str) -> Option<&str> {
        self.points.iter().find(|(name, _)| name == axis).map(|(_, point)| point.as_str())
    }

    /// The points joined with dots, which is what a case id is built from.
    #[must_use]
    pub fn slug(&self) -> String {
        self.points.iter().map(|(_, point)| point.as_str()).collect::<Vec<_>>().join(".")
    }

    /// The axes as JSON, one field per axis.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::Object(
            self.points
                .iter()
                .map(|(axis, point)| (axis.clone(), Json::string(point.clone())))
                .collect(),
        )
    }

    /// Axes read back from JSON. A value that is not an object gives empty axes.
    #[must_use]
    pub fn from_json(value: &Json) -> Self {
        let Json::Object(fields) = value else {
            return Self::default();
        };
        Self {
            points: fields
                .iter()
                .filter_map(|(axis, point)| point.as_str().map(|p| (axis.clone(), p.to_owned())))
                .collect(),
        }
    }
}

/// What the case is supposed to do when it runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expect {
    /// It compiles, runs, exits zero, and prints exactly this.
    ///
    /// This is the shape almost every case has. The program computes something and prints it,
    /// so the printed text is the oracle and two compilers agreeing on it is the evidence.
    /// The expected text is filled in by whichever toolchain is named as the reference, and a
    /// case whose reference output is not yet known carries an empty string until the first
    /// run records one.
    Output(String),

    /// It must not compile, and the diagnostic must mention this text.
    ///
    /// For the cases that exist to check that an error is caught. Matching on a substring
    /// rather than the whole message means the wording can improve without the corpus
    /// turning red, while still checking that the compiler pointed at the right thing.
    Rejected(String),
}

impl Expect {
    /// The tag stored in a record.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Output(_) => "output",
            Self::Rejected(_) => "rejected",
        }
    }

    /// The text that goes with the tag.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::Output(text) | Self::Rejected(text) => text,
        }
    }
}

/// One program in the corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    /// A stable name, unique across the corpus.
    ///
    /// Built from the facet and the axis slug, with a digest of the source on the end so that
    /// changing a generator template changes the id. A record from an old run therefore never
    /// silently describes a program that has since been edited.
    pub id: String,
    /// The one thing this case is about.
    pub facet: Facet,
    /// Where in the axis space it came from. Empty for a handwritten case.
    pub axes: Axes,
    /// Which C it is written in.
    pub dialect: Dialect,
    /// The complete translation unit.
    pub source: String,
    /// What it is supposed to do.
    pub expect: Expect,
    /// Free labels, sorted, for selecting subsets that no axis captures.
    ///
    /// `handwritten`, `slow`, `large`, `ub-clean` and the like. The harness takes
    /// `--tag` and `--without-tag` so that a nightly job can run what a pull request skips.
    pub tags: Vec<String>,
}

impl Case {
    /// Builds a case and computes its id.
    #[must_use]
    pub fn new(
        facet: Facet,
        axes: Axes,
        dialect: Dialect,
        source: impl Into<String>,
        expect: Expect,
    ) -> Self {
        let source = source.into();
        let id = Self::make_id(facet, &axes, dialect, &source);
        Self { id, facet, axes, dialect, source, expect, tags: Vec::new() }
    }

    /// The same, with tags.
    #[must_use]
    pub fn tagged(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|t| (*t).to_owned()).collect();
        self.tags.sort_unstable();
        self.tags.dedup();
        self
    }

    /// The id a case with these parts would have.
    #[must_use]
    pub fn make_id(facet: Facet, axes: &Axes, dialect: Dialect, source: &str) -> String {
        let slug = axes.slug();
        let digest = sha256::short(source.as_bytes());
        if slug.is_empty() {
            format!("{}.{}.{}", facet.name(), dialect.name(), &digest[..8])
        } else {
            format!("{}.{}.{}.{}", facet.name(), slug, dialect.name(), &digest[..8])
        }
    }

    /// The phase this case is evidence for.
    #[must_use]
    pub const fn phase(&self) -> Phase {
        self.facet.phase()
    }

    /// The file name the source is written to.
    #[must_use]
    pub fn file_name(&self) -> String {
        format!("{}.c", self.id)
    }

    /// Where the source is written, under the programs directory.
    ///
    /// Grouped by phase and then by facet, so somebody who wants to see what the corpus has
    /// to say about loop unrolling can open one directory and read it, rather than filtering
    /// a flat list of a thousand files. The report and the SARIF output both point here.
    #[must_use]
    pub fn path(&self) -> String {
        program_path(self.facet, &self.id)
    }

    /// Whether the case carries a tag.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Whether the case is one that must not compile.
    #[must_use]
    pub const fn expect_is_rejection(&self) -> bool {
        matches!(self.expect, Expect::Rejected(_))
    }

    /// The digest of the source, which is what a report cites.
    #[must_use]
    pub fn digest(&self) -> String {
        sha256::hex(self.source.as_bytes())
    }

    /// The case as JSON, which is what the manifest holds.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("id", Json::string(self.id.clone())),
            ("facet", Json::string(self.facet.name())),
            ("phase", Json::string(self.phase().name())),
            ("dialect", Json::string(self.dialect.name())),
            ("axes", self.axes.to_json()),
            ("expect_kind", Json::string(self.expect.kind())),
            ("expect", Json::string(self.expect.text().to_owned())),
            ("tags", Json::array(self.tags.iter().map(|t| Json::string(t.clone())))),
            ("source_sha256", Json::string(self.digest())),
        ])
    }
}

/// Where a case with this facet and this id is written, relative to the programs directory.
///
/// A free function as well as a method, because a finding carries the facet and the case id
/// but not the case itself, and a report that pointed at a path it worked out differently
/// from the generator would point at a file that is not there.
#[must_use]
pub fn program_path(facet: Facet, id: &str) -> String {
    format!("{}/{}/{id}.c", facet.phase().name(), facet.name())
}

/// The whole corpus, as one list plus the digest that identifies it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
    /// Every case, sorted by id so that two generator runs produce the same file.
    pub cases: Vec<Case>,
}

impl Manifest {
    /// A manifest from cases, sorted and checked for duplicate ids.
    ///
    /// # Errors
    ///
    /// When two cases share an id, which means two generator templates collided and the
    /// report would silently drop one of them.
    pub fn new(mut cases: Vec<Case>) -> Result<Self, String> {
        cases.sort_by(|a, b| a.id.cmp(&b.id));
        for pair in cases.windows(2) {
            if pair[0].id == pair[1].id {
                return Err(format!("two cases share the id `{}`", pair[0].id));
            }
        }
        Ok(Self { cases })
    }

    /// The digest of the whole corpus.
    ///
    /// Over the ids and the sources, so that adding a case, editing a case or reordering the
    /// generator all change it. A report carries this, and two reports with the same corpus
    /// digest are comparable case by case without any further checking.
    #[must_use]
    pub fn digest(&self) -> String {
        let mut hasher = sha256::Sha256::new();
        for case in &self.cases {
            hasher.update(case.id.as_bytes());
            hasher.update(b"\0");
            hasher.update(case.source.as_bytes());
            hasher.update(b"\0");
        }
        let bytes = hasher.finish();
        let mut out = String::with_capacity(64);
        for byte in bytes {
            out.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
            out.push(char::from_digit(u32::from(byte & 0xf), 16).unwrap_or('0'));
        }
        out
    }

    /// How many cases carry each facet.
    #[must_use]
    pub fn by_facet(&self) -> Vec<(Facet, usize)> {
        Facet::ALL
            .iter()
            .map(|facet| (*facet, self.cases.iter().filter(|c| c.facet == *facet).count()))
            .filter(|(_, count)| *count > 0)
            .collect()
    }

    /// The manifest as JSON.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object([
            ("schema", Json::string(crate::SCHEMA_VERSION)),
            ("corpus_sha256", Json::string(self.digest())),
            ("count", Json::int(self.cases.len() as i64)),
            ("cases", Json::array(self.cases.iter().map(Case::to_json))),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::{Axes, Case, Dialect, Expect, Manifest};
    use crate::facet::Facet;

    fn case(source: &str) -> Case {
        Case::new(
            Facet::ConstantFold,
            Axes::of([("type", "int32"), ("op", "add")]),
            Dialect::C17,
            source,
            Expect::Output("7\n".to_owned()),
        )
    }

    #[test]
    fn an_id_is_built_from_the_facet_the_axes_the_dialect_and_the_source() {
        let one = case("int main(void) { return 0; }");
        assert!(one.id.starts_with("constant-fold.int32.add.c17."), "{}", one.id);
    }

    #[test]
    fn editing_the_source_changes_the_id_so_an_old_record_never_names_a_new_program() {
        let before = case("int main(void) { return 0; }");
        let after = case("int main(void) { return 1; }");
        assert_ne!(before.id, after.id);
    }

    #[test]
    fn a_case_with_no_axes_still_gets_a_readable_id() {
        let bare = Case::new(
            Facet::Baseline,
            Axes::default(),
            Dialect::C23,
            "int main(void) { return 0; }",
            Expect::Output(String::new()),
        );
        assert!(bare.id.starts_with("baseline.c23."), "{}", bare.id);
    }

    #[test]
    fn tags_come_out_sorted_and_deduplicated() {
        let tagged = case("int main(void) { return 0; }").tagged(&["slow", "ub-clean", "slow"]);
        assert_eq!(tagged.tags, ["slow", "ub-clean"]);
        assert!(tagged.has_tag("slow"));
        assert!(!tagged.has_tag("large"));
    }

    #[test]
    fn a_manifest_refuses_two_cases_with_the_same_id() {
        let same = case("int main(void) { return 0; }");
        let err = Manifest::new(vec![same.clone(), same]).unwrap_err();
        assert!(err.contains("share the id"), "{err}");
    }

    #[test]
    fn a_manifest_sorts_by_id_so_two_generator_runs_agree() {
        let a = case("int main(void) { return 1; }");
        let b = case("int main(void) { return 2; }");
        let one = Manifest::new(vec![a.clone(), b.clone()]).unwrap();
        let two = Manifest::new(vec![b, a]).unwrap();
        assert_eq!(one, two);
        assert_eq!(one.digest(), two.digest());
    }

    #[test]
    fn the_corpus_digest_moves_when_any_case_changes() {
        let base = Manifest::new(vec![case("int main(void) { return 0; }")]).unwrap();
        let grown = Manifest::new(vec![
            case("int main(void) { return 0; }"),
            case("int main(void) { return 1; }"),
        ])
        .unwrap();
        assert_ne!(base.digest(), grown.digest());
    }

    #[test]
    fn axes_round_trip_through_json_and_keep_their_order() {
        let axes = Axes::of([("type", "uint8"), ("shape", "loop"), ("storage", "static")]);
        assert_eq!(Axes::from_json(&axes.to_json()), axes);
        assert_eq!(axes.slug(), "uint8.loop.static");
        assert_eq!(axes.get("shape"), Some("loop"));
        assert_eq!(axes.get("missing"), None);
    }

    #[test]
    fn a_dialect_names_the_flag_the_compiler_actually_takes() {
        assert_eq!(Dialect::C17.std_flag(), "-std=c17");
        assert_eq!(Dialect::C23.std_flag(), "-std=c23");
        assert_eq!(Dialect::parse("c23"), Some(Dialect::C23));
        assert_eq!(Dialect::parse("c11"), None);
    }
}
