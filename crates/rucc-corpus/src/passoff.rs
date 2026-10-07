//! The pass off measurement, which is the default experiment of section 42.4 in rucc's optimizer
//! measurement plan and tamnd/rucc#2968.
//!
//! Every pass in rucc's pipeline at a level is turned off on its own with `-fdisable-NAME`, and
//! the corpus is measured without it against the whole pipeline. The machinery is the sweep's:
//! every case is compiled to assembly with the pass off, and only the cases whose assembly
//! changed are built and run, by rucc as it is, by rucc with the pass off, and by the reference.
//!
//! A pass that appears more than once in the pipeline is turned off everywhere it runs, since
//! that is what the flag does. The numbers for one pass are not the share of the whole that pass
//! is worth, because passes enable each other, and the page says so.

use crate::args::Args;
use crate::sweep::{self, FLAT, Moved, Setup};
use corpus_model::json::Json;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::process::ExitCode;

/// Everything `passoff` will accept.
pub(crate) const OPTIONS: &[&str] = &[
    "rucc",
    "reference",
    "level",
    "pass",
    "facet",
    "limit",
    "exclude-tag",
    "out",
    "work",
    "jobs",
    "repeats",
    "no-cache",
    "quiet",
];

/// The passes a `--print-pipeline` listing names, each once, in the order they first run.
///
/// Each pass is a line `N: name, what it does`. The line that says the level, and anything else
/// that is not numbered, is left alone.
pub(crate) fn passes(listing: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut passes = Vec::new();
    for line in listing.lines() {
        let Some((number, rest)) = line.split_once(": ") else {
            continue;
        };
        if number.trim().parse::<u32>().is_err() {
            continue;
        }
        let name = rest.split(',').next().unwrap_or_default().trim();
        if !name.is_empty() && seen.insert(name.to_owned()) {
            passes.push(name.to_owned());
        }
    }
    passes
}

/// What the measurement makes of a pass from what turning it off did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Call {
    /// Turning it off changes the assembly of no case.
    Idle,
    /// It changes some, and neither the instructions nor the text move past [`FLAT`].
    Flat,
    /// Off costs instructions or text, and saves on neither.
    Pays,
    /// Off saves on one and costs on the other.
    Trades,
    /// Off saves on one of the two and costs on neither, so on this corpus the pass makes the
    /// code worse.
    Costs,
}

impl Call {
    /// The word the reports use.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Flat => "flat",
            Self::Pays => "pays",
            Self::Trades => "trades",
            Self::Costs => "costs",
        }
    }
}

/// The call on a pass, from what turning it off did. The changes are off against on, so a
/// positive change is what the pass saves.
pub(crate) fn call(off: &Moved) -> Call {
    if off.changed == 0 {
        return Call::Idle;
    }
    let instructions = off.instruction_change().unwrap_or(0.0);
    let text = off.text_change().unwrap_or(0.0);
    let (fewer, more) = (instructions < -FLAT, instructions > FLAT);
    let (smaller, larger) = (text < -FLAT, text > FLAT);
    match (fewer || smaller, more || larger) {
        (false, false) => Call::Flat,
        (false, true) => Call::Pays,
        (true, true) => Call::Trades,
        (true, false) => Call::Costs,
    }
}

/// Facets with the instructions turning a pass off cost on each.
pub(crate) type Facets<'a> = Vec<(&'a str, i64)>;

/// The facets turning the pass off cost the most instructions on, which are the ones it helps
/// most, and the ones it saved the most on, which are the ones it hurts most.
pub(crate) fn best_and_worst(off: &Moved, how_many: usize) -> (Facets<'_>, Facets<'_>) {
    let mut facets: Facets<'_> =
        off.facets.iter().map(|(facet, delta)| (facet.as_str(), *delta)).collect();
    facets.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let best = facets.iter().filter(|(_, delta)| *delta > 0).take(how_many).copied().collect();
    let worst =
        facets.iter().rev().filter(|(_, delta)| *delta < 0).take(how_many).copied().collect();
    (best, worst)
}

/// One line of `passoff.jsonl`.
#[derive(Debug, Clone)]
struct Line {
    pass: String,
    off: Moved,
}

impl Line {
    fn to_json(&self, stamp: &sweep::Stamp) -> Json {
        let mut fields = vec![("pass".to_owned(), Json::string(&self.pass))];
        fields.extend(self.off.fields());
        fields.extend(stamp.fields());
        Json::Object(fields)
    }

    fn from_json(value: &Json) -> Option<Self> {
        Some(Self { pass: value.get("pass")?.as_str()?.to_owned(), off: Moved::from_json(value)? })
    }
}

/// Runs the measurement.
pub(crate) fn command(args: &Args) -> Result<ExitCode, String> {
    args.only(OPTIONS)?;
    let setup = Setup::new(args, "reports/passoff", "target/passoff-work")?;
    let level = setup.level.flag();
    let mut passes = passes(&setup.listing(&[level, "--print-pipeline"])?);
    let asked = args.values("pass");
    if !asked.is_empty() {
        for name in asked {
            if !passes.contains(name) {
                return Err(format!("{name} is not a pass rucc runs at {level}"));
            }
        }
        passes.retain(|pass| asked.contains(pass));
    }

    let (kept, mut journal) = setup.journal("passoff.jsonl")?;
    let mut lines: Vec<Line> = kept.iter().filter_map(Line::from_json).collect();
    if !setup.quiet {
        eprintln!("compiling {} cases to assembly as rucc is", setup.cases.len());
    }
    let baseline = setup.assemble(None);

    for (at, pass) in passes.iter().enumerate() {
        if lines.iter().any(|line| line.pass == *pass) {
            continue;
        }
        let off = setup.measure(&baseline, &format!("-fdisable-{pass}"))?;
        if !setup.quiet {
            eprintln!("{}/{} {pass} off: {} cases changed", at + 1, passes.len(), off.changed);
        }
        let line = Line { pass: pass.clone(), off };
        journal.add(&line.to_json(&setup.stamp))?;
        lines.push(line);
    }
    lines.retain(|line| passes.contains(&line.pass));
    lines.sort_by_key(|line| passes.iter().position(|pass| *pass == line.pass));

    let report = Report { lines: &lines, setup: &setup };
    sweep::write(&setup.out.join("passoff.json"), &report.to_json().to_pretty())?;
    sweep::write(&setup.out.join("index.md"), &report.to_markdown())?;
    println!("wrote the pass off measurement of {} passes to {}", lines.len(), setup.out.display());
    let broke = lines.iter().filter(|line| !line.off.broke.is_empty()).count();
    if broke > 0 {
        println!("{broke} passes turned off built a case that gave the wrong answer");
    }
    Ok(ExitCode::SUCCESS)
}

/// The two reports the measurement writes.
struct Report<'a> {
    lines: &'a [Line],
    setup: &'a Setup,
}

impl Report<'_> {
    fn to_json(&self) -> Json {
        let stamp = &self.setup.stamp;
        let passes = self.lines.iter().map(|line| {
            let mut fields = vec![
                ("pass".to_owned(), Json::string(&line.pass)),
                ("call".to_owned(), Json::string(call(&line.off).name())),
            ];
            fields.extend(line.off.fields());
            Json::Object(fields)
        });
        Json::object([
            ("compiler", Json::string(&stamp.compiler)),
            ("corpus", Json::string(&stamp.corpus)),
            ("level", Json::string(self.setup.level.name())),
            ("reference", Json::string(&self.setup.reference.id)),
            ("cases", Json::int(i64::try_from(self.setup.cases.len()).unwrap_or(i64::MAX))),
            ("passes", Json::array(passes)),
        ])
    }

    fn to_markdown(&self) -> String {
        let setup = self.setup;
        let reference = &setup.reference.id;
        let mut out = String::new();
        let count =
            |wanted: Call| self.lines.iter().filter(|line| call(&line.off) == wanted).count();
        let _ = writeln!(out, "# Each pass off\n");
        let _ = writeln!(
            out,
            "Every pass rucc runs at `-{}`, turned off on its own with `-fdisable-NAME`, over {} cases of the corpus. This is the pass off experiment of section 42.4 of rucc's optimizer measurement plan, tamnd/rucc#2968. The compiler is `{}` and the corpus digest is `{}`.\n",
            setup.level.name(),
            setup.cases.len(),
            setup.stamp.compiler,
            setup.stamp.corpus
        );
        let _ = writeln!(
            out,
            "**These numbers do not add up.** Passes enable each other, so what one pass is worth with the rest of the pipeline there is not its share of what the pipeline is worth, and a pass that looks idle here may be what makes another one fire. This is the warning of section 42.8.\n"
        );
        let _ = writeln!(
            out,
            "Of {} passes the measurement calls {} pays, {} trades, {} costs, {} flat and {} idle. A pass pays when turning it off costs instructions retired or text by more than {:.1}% of what the cases it changed had, and saves on neither. It trades when off saves on one and costs on the other, and it costs when off saves on one and costs on neither, which is a pass that makes the code worse on this corpus. It is flat when it changes the assembly and moves neither, and idle when it changes no case at all.\n",
            self.lines.len(),
            count(Call::Pays),
            count(Call::Trades),
            count(Call::Costs),
            count(Call::Flat),
            count(Call::Idle),
            FLAT * 100.0
        );
        let _ = writeln!(
            out,
            "Only the cases whose assembly changed are built and run, so each change is over those cases and not over the whole corpus, and is off against on: a positive change is what the pass saves. Instructions are over the cases that ran right for rucc both ways and for {reference}, and the `against {reference}` column is rucc's instructions over {reference}'s on those cases, with the pass and then without. Compile time is wall time on a shared machine and is the noisiest of the four.\n"
        );
        let _ = writeln!(
            out,
            "| pass | cases changed | instructions | text | compile time | against {reference} | helps most | hurts most | call |"
        );
        let _ = writeln!(out, "|---|---:|---:|---:|---:|---|---|---|---|");
        let show = |facets: &[(&str, i64)]| {
            facets
                .iter()
                .map(|(facet, delta)| format!("{facet} {delta:+}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        for line in self.lines {
            let off = &line.off;
            let (best, worst) = best_and_worst(off, 3);
            let against = match sweep::ratio(off.instructions[0], off.instructions[2]) {
                Some(on) => format!(
                    "{on:.3} to {:.3}",
                    sweep::ratio(off.instructions[1], off.instructions[2]).unwrap_or(on)
                ),
                None => String::new(),
            };
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} |",
                line.pass,
                off.changed,
                sweep::percent(off.instruction_change()),
                sweep::percent(off.text_change()),
                sweep::percent(off.compile_change()),
                against,
                show(&best),
                show(&worst),
                call(off).name()
            );
        }
        let broke: Vec<&Line> =
            self.lines.iter().filter(|line| !line.off.broke.is_empty()).collect();
        if !broke.is_empty() {
            let _ = writeln!(out, "\n## Passes whose absence broke a case\n");
            let _ = writeln!(
                out,
                "Turning a pass off should never change what a program prints or whether it builds. A case that goes wrong without one is a bug in a pass that runs later and counted on it, and each gets an issue of its own.\n"
            );
            for line in broke {
                let cases: Vec<String> =
                    line.off.broke.iter().map(|case| format!("`{case}`")).collect();
                let _ = writeln!(out, "- `{}` off: {}", line.pass, cases.join(", "));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{Call, best_and_worst, call, passes};
    use crate::sweep::Moved;

    #[test]
    fn each_pass_is_named_once_in_the_order_it_first_runs() {
        let listing = "level: -O2\n1: fold, constants\n2: sroa, a local, one value\n3: fold, again\n\
                       4: simplify-cfg, blocks\n";
        assert_eq!(passes(listing), ["fold", "sroa", "simplify-cfg"]);
        assert!(passes("level: -O0\n").is_empty());
    }

    fn off(instructions: [u64; 2], text: [u64; 2]) -> Moved {
        Moved {
            changed: 1,
            counted: 1,
            instructions: [instructions[0], instructions[1], 1000],
            text,
            ..Moved::default()
        }
    }

    #[test]
    fn a_pass_pays_trades_costs_or_is_flat_or_idle_by_what_off_did() {
        assert_eq!(call(&Moved::default()), Call::Idle);
        assert_eq!(call(&off([100_000, 100_050], [100, 100])), Call::Flat);
        assert_eq!(call(&off([1000, 1100], [100, 100])), Call::Pays);
        assert_eq!(call(&off([1000, 1100], [100, 90])), Call::Trades);
        assert_eq!(call(&off([1000, 900], [100, 100])), Call::Costs);
        assert_eq!(call(&off([1000, 1000], [100, 90])), Call::Costs);
    }

    #[test]
    fn the_facets_a_pass_helps_and_hurts_most_are_apart() {
        let mut moved = Moved::default();
        for (facet, delta) in [("a", 5), ("b", -50), ("c", 0), ("d", 20), ("e", -1)] {
            moved.facets.insert(facet.to_owned(), delta);
        }
        let (best, worst) = best_and_worst(&moved, 3);
        assert_eq!(best, [("d", 20), ("a", 5)]);
        assert_eq!(worst, [("b", -50), ("e", -1)]);
    }
}
