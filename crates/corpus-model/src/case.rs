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

/// One more translation unit that a case is linked with.
///
/// Almost every case is a single file and that is the right default, because a failure in a
/// program the compiler saw all of at once is a failure about code generation rather than about
/// linking. The exception is the whole class of optimizations that only exist across a file
/// boundary, and a case about one of those has to hand the compiler the boundary to work on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    /// A short word naming the unit, which is what its file is named after.
    pub name: String,
    /// The complete translation unit. It has no `main` in it, since the case already has one.
    pub source: String,
}

impl Unit {
    /// A named unit.
    #[must_use]
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self { name: name.into(), source: source.into() }
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
    /// The translation unit with `main` in it.
    pub source: String,
    /// The other translation units, linked with it, in the order they go on the command line.
    ///
    /// Empty for almost every case in the corpus.
    pub units: Vec<Unit>,
    /// Flags this case has to be built with, over and above the optimization level.
    ///
    /// A level is something every case is built at, so it is a dimension of the run. A flag like
    /// `-flto` is not: it only means anything for a program written to be linked, and asking
    /// every case in the corpus for it would multiply the whole run to learn one facet's answer.
    /// So it belongs to the case, and the case that carries it says so in its axes.
    pub flags: Vec<String>,
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
        Self::build(facet, axes, dialect, source.into(), Vec::new(), Vec::new(), expect)
    }

    /// The same, spanning more than one translation unit and with flags of its own.
    #[must_use]
    pub fn linked(
        facet: Facet,
        axes: Axes,
        dialect: Dialect,
        source: impl Into<String>,
        units: Vec<Unit>,
        flags: Vec<String>,
        expect: Expect,
    ) -> Self {
        Self::build(facet, axes, dialect, source.into(), units, flags, expect)
    }

    /// Assembles a case and gives it its id.
    fn build(
        facet: Facet,
        axes: Axes,
        dialect: Dialect,
        source: String,
        units: Vec<Unit>,
        flags: Vec<String>,
        expect: Expect,
    ) -> Self {
        let mut case = Self {
            id: String::new(),
            facet,
            axes,
            dialect,
            source,
            units,
            flags,
            expect,
            tags: Vec::new(),
        };
        let slug = case.axes.slug();
        let digest = sha256::short(case.fingerprint().as_bytes());
        case.id = if slug.is_empty() {
            format!("{}.{}.{}", facet.name(), dialect.name(), &digest[..8])
        } else {
            format!("{}.{slug}.{}.{}", facet.name(), dialect.name(), &digest[..8])
        };
        case
    }

    /// The name of the family this case belongs to, which is its id without the digest.
    ///
    /// The id ends in eight characters of a hash of the program text, so it moves whenever the
    /// generator changes what the program says, even when the case is about exactly the same
    /// thing it was about before. Anything that has to name a case across such a change names
    /// the family instead. See [`family_of`].
    #[must_use]
    pub fn family(&self) -> String {
        family_of(&self.id).to_owned()
    }

    /// Everything about the case that decides what a compiler is asked to do.
    ///
    /// The sources, the names they are compiled under, and the flags. Not the facet and not the
    /// axes, which are how the corpus talks about the case rather than anything the compiler
    /// sees. A case with one unit and no flags of its own fingerprints as exactly its source, so
    /// adding the two fields left every id and every cache key in the corpus where it was.
    fn fingerprint(&self) -> String {
        let mut feed = String::with_capacity(self.source.len() + 64);
        feed.push_str(&self.source);
        for unit in &self.units {
            feed.push('\0');
            feed.push_str(&unit.name);
            feed.push('\0');
            feed.push_str(&unit.source);
        }
        for flag in &self.flags {
            feed.push('\0');
            feed.push_str(flag);
        }
        feed
    }

    /// The same, with tags.
    #[must_use]
    pub fn tagged(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|t| (*t).to_owned()).collect();
        self.tags.sort_unstable();
        self.tags.dedup();
        self
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

    /// The file name one of the other units is written to.
    ///
    /// Named after the case rather than after the unit alone, so that two cases in one directory
    /// that both have a unit called `helper` do not write to the same file.
    #[must_use]
    pub fn unit_file_name(&self, unit: &Unit) -> String {
        format!("{}.{}.c", self.id, unit.name)
    }

    /// How much C the whole case is, counting every unit it is linked from.
    #[must_use]
    pub fn size(&self) -> crate::Source {
        self.units.iter().fold(crate::Source::of(&self.source), |sum, unit| {
            sum.plus(crate::Source::of(&unit.source))
        })
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

    /// The digest of everything the compiler is handed, which is what a report cites.
    #[must_use]
    pub fn digest(&self) -> String {
        sha256::hex(self.fingerprint().as_bytes())
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
            ("units", Json::array(self.units.iter().map(|u| Json::string(u.name.clone())))),
            ("flags", Json::array(self.flags.iter().map(|f| Json::string(f.clone())))),
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

/// A case id with its trailing digest taken off.
///
/// The stable half of an id, which is the facet, the axis point and the dialect. An id with no
/// dot in it is returned whole, since there is nothing to take off and a caller that hands one
/// over wants a name back rather than an empty string.
#[must_use]
pub fn family_of(id: &str) -> &str {
    match id.rfind('.') {
        Some(at) => &id[..at],
        None => id,
    }
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
            hasher.update(case.fingerprint().as_bytes());
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
    use super::{Axes, Case, Dialect, Expect, Manifest, Unit};
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
    fn a_case_with_one_unit_and_no_flags_digests_to_exactly_its_source() {
        // The whole reason the fingerprint is spelled the way it is. Adding the two fields had to
        // leave every id and every cached record in the corpus where they were, and a test is the
        // only thing that keeps that true the next time somebody adds a third field.
        let plain = case("int main(void) { return 0; }");
        assert_eq!(plain.digest(), crate::sha256::hex(plain.source.as_bytes()));
    }

    #[test]
    fn a_unit_and_a_flag_each_change_what_a_case_is() {
        let plain = case("int main(void) { return 0; }");
        let linked = Case::linked(
            Facet::ConstantFold,
            Axes::of([("type", "int32"), ("op", "add")]),
            Dialect::C17,
            "int main(void) { return 0; }",
            vec![Unit::new("helper", "int helper(void) { return 1; }\n")],
            Vec::new(),
            Expect::Output("7\n".to_owned()),
        );
        let flagged = Case::linked(
            Facet::ConstantFold,
            Axes::of([("type", "int32"), ("op", "add")]),
            Dialect::C17,
            "int main(void) { return 0; }",
            Vec::new(),
            vec!["-flto".to_owned()],
            Expect::Output("7\n".to_owned()),
        );
        assert_ne!(plain.digest(), linked.digest());
        assert_ne!(plain.digest(), flagged.digest());
        assert_ne!(linked.digest(), flagged.digest());
        assert_ne!(plain.id, linked.id);
        assert_eq!(linked.unit_file_name(&linked.units[0]), format!("{}.helper.c", linked.id));
    }

    #[test]
    fn the_size_of_a_case_is_every_file_it_is_linked_from() {
        let linked = Case::linked(
            Facet::ConstantFold,
            Axes::default(),
            Dialect::C17,
            "one\ntwo\n",
            vec![Unit::new("helper", "three\n"), Unit::new("other", "four\nfive\n")],
            Vec::new(),
            Expect::Output("1\n".to_owned()),
        );
        let size = linked.size();
        assert_eq!(size.lines, 5);
        assert_eq!(size.files, 3);
        assert_eq!(size.bytes, 8 + 6 + 10);
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
