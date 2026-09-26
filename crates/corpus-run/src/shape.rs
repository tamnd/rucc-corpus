//! What each `switch` in a case was lowered as.
//!
//! Section 24.7 of the rucc spec asks for the lowering of every switch next to the one gcc chose,
//! so that a switch rucc turns into compares where gcc built a table shows up in a report rather
//! than in somebody's afternoon with two assembly listings. The two compilers are asked in
//! different ways. rucc says what it did under `-fopt-info`, one `[switch-lowering]` line per
//! switch, naming the function and the shape, and a `[switch-conv]` line for a switch it turned
//! into a lookup or into arithmetic before lowering ever saw it. gcc says nothing about switches there, so its
//! assembly is read instead, for the three things a switch can leave behind: a jump table, a bit
//! test and a lookup table of answers.
//!
//! It is a second compile, with `-S`, apart from the one that is timed, so that asking does not
//! move the compile time. Only a case with a `switch` in it gets one, and only a case of one
//! translation unit, because `-S` writes a file per unit and `-o` can name one.
//!
//! # What the assembly can and cannot say
//!
//! A function with none of the three in it lowered its switches as compares, or had none. That
//! cannot be told apart from the assembly, so a function with none of them is listed as compares
//! and the comparison is made per case, over the shapes other than compares. A `bt` that tests a
//! bit of a mask the program wrote by hand reads as a bit test too. The corpus writes its switches
//! as switches, so that is rare, but it is why a disagreement here is a lead to follow and not a
//! verdict.

use crate::compile::{COMPILE_TIMEOUT, SOURCE};
use crate::exec;
use crate::toolchain::Spec;
use corpus_model::Case;
use std::path::Path;

/// What the second compile writes its assembly to.
pub const ASSEMBLY: &str = "case.s";

/// Where rucc is asked to write what it said about its switches.
pub const REMARKS: &str = "case.switches";

/// A switch that became a jump table.
pub const TABLE: &str = "table";

/// A switch whose cases were tested as bits of a mask.
pub const BIT_TEST: &str = "bit-test";

/// A switch whose arms were all constants, read out of a table of answers rather than jumped to.
pub const LOOKUP: &str = "lookup";

/// A switch that became compares, as a tree or as a walk.
pub const COMPARES: &str = "compares";

/// A switch whose answers were a fixed multiple of the label, worked out rather than looked up.
///
/// gcc does this too and leaves nothing in its assembly to say so, so the report counts it with
/// compares, and the record keeps the name for whoever reads it.
pub const ARITHMETIC: &str = "arithmetic";

/// Whether a case gets the second compile.
#[must_use]
pub fn wanted(case: &Case) -> bool {
    case.units.is_empty() && case.source.contains("switch")
}

/// Whether a toolchain is rucc, which is the one that says what it did rather than being read.
#[must_use]
pub fn says_what_it_did(spec: &Spec) -> bool {
    spec.id.to_ascii_lowercase().contains("rucc") || spec.program.contains("rucc")
}

/// What each switch in the case became, as `function: shape`, or nothing when the compiler is
/// neither of the two this knows how to ask.
///
/// `args` is everything the timed compile was given apart from the output and the sources.
#[must_use]
pub fn shapes(spec: &Spec, mut args: Vec<String>, dir: &Path) -> Vec<String> {
    let rucc = says_what_it_did(spec);
    if !rucc && !spec.understands_opt_info() {
        return Vec::new();
    }
    // Gone first, so that a compiler that fails to write either is not read as having said
    // what the last run of this case said.
    let _ = std::fs::remove_file(dir.join(ASSEMBLY));
    let _ = std::fs::remove_file(dir.join(REMARKS));
    args.extend(["-S".to_owned(), "-o".to_owned(), ASSEMBLY.to_owned()]);
    if rucc {
        args.push(format!("-fopt-info-optimized={REMARKS}"));
    }
    args.push(SOURCE.to_owned());
    let ran = exec::run(&spec.program, &args, Some(dir), COMPILE_TIMEOUT);
    if !ran.is_ok_and(|outcome| outcome.ok) {
        return Vec::new();
    }
    let read = |name: &str| std::fs::read_to_string(dir.join(name)).unwrap_or_default();
    if rucc { from_remarks(&read(REMARKS)) } else { from_assembly(&read(ASSEMBLY)) }
}

/// The switches rucc said it converted or lowered, one per `[switch-conv]` or `[switch-lowering]`
/// line.
///
/// A tree and a walk are both kept by name here. They are both compares to the report, but the
/// difference is what section 24.7 is measuring, so the record has it. A switch converted into a
/// lookup is still lowered afterwards, as the range check in front of the load, and that line is
/// kept too, since it is what rucc said.
#[must_use]
pub fn from_remarks(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.splitn(3, ": ");
            let function = fields.nth(1)?;
            let shape = if line.ends_with("[switch-lowering]") {
                let (_, after) = line.split_once("lowered as a ")?;
                after.split([';', ' ']).next()?
            } else if line.ends_with("[switch-conv]") && line.contains(": switch replaced by ") {
                if line.contains("a table of its answers") { LOOKUP } else { ARITHMETIC }
            } else {
                return None;
            };
            Some(format!("{function}: {shape}"))
        })
        .collect()
}

/// What gcc's assembly shows each function did with its switches.
///
/// Every function is listed, as compares when none of the three shapes turned up in it, so that
/// a record with nothing in it means the compiler was not asked rather than that it built no
/// tables.
#[must_use]
pub fn from_assembly(text: &str) -> Vec<String> {
    let functions: Vec<&str> = text
        .lines()
        .filter_map(|line| {
            let rest = line.trim_start().strip_prefix(".type")?;
            let (name, kind) = rest.trim().split_once(',')?;
            (kind.trim() == "@function").then_some(name.trim())
        })
        .collect();
    let mut found: Vec<(String, Vec<&str>)> = Vec::new();
    for line in text.lines() {
        if let Some(name) = line.strip_suffix(':').filter(|name| functions.contains(name)) {
            found.push((name.to_owned(), Vec::new()));
            continue;
        }
        let Some((_, shapes)) = found.last_mut() else { continue };
        let Some(shape) = shape_of(line) else { continue };
        if !shapes.contains(&shape) {
            shapes.push(shape);
        }
    }
    let mut listed = Vec::new();
    for (function, mut shapes) in found {
        if shapes.is_empty() {
            shapes.push(COMPARES);
        }
        shapes.sort_unstable();
        listed.extend(shapes.into_iter().map(|shape| format!("{function}: {shape}")));
    }
    listed
}

/// The shape one line of assembly is a sign of, if it is one.
///
/// A jump table is its entries, which gcc writes as `.long .L5-.L4` in a position independent
/// executable and as `.quad .L5` otherwise. The jump itself is no help, since a call through a
/// pointer in tail position is a `jmp *` too.
fn shape_of(line: &str) -> Option<&'static str> {
    let line = line.trim_start();
    let mut words = line.split_whitespace();
    let first = words.next()?;
    let second = words.next().unwrap_or("");
    match first {
        ".long" | ".quad" if second.starts_with(".L") => Some(TABLE),
        "bt" | "btw" | "btl" | "btq" => Some(BIT_TEST),
        _ if !first.starts_with('.') && !line.ends_with(':') && line.contains("CSWTCH") => {
            Some(LOOKUP)
        }
        _ => None,
    }
}

/// The shapes a case used, other than compares, or compares alone when it used nothing else.
///
/// What the report compares, since a function the assembly shows no table in may have had no
/// switch at all, and the two compilers do not have to inline the same functions.
#[must_use]
pub fn used(switches: &[String]) -> String {
    let mut shapes: Vec<&str> = switches
        .iter()
        .filter_map(|entry| entry.rsplit_once(": ").map(|(_, shape)| shape))
        .map(|shape| if matches!(shape, "tree" | "walk" | ARITHMETIC) { COMPARES } else { shape })
        .filter(|&shape| shape != COMPARES)
        .collect();
    shapes.sort_unstable();
    shapes.dedup();
    if shapes.is_empty() { COMPARES.to_owned() } else { shapes.join(", ") }
}

#[cfg(test)]
mod tests {
    use super::{from_assembly, from_remarks, used};

    #[test]
    fn a_remark_names_the_function_and_the_shape_and_nothing_else_is_read() {
        let text = "\
case.c: step: optimized: switch of 8 cases lowered as a walk; clusters 8, tables 0, bit tests 0 (1) [switch-lowering]
case.c: main: optimized: switch of 40 cases lowered as a table; clusters 1, tables 1, bit tests 0, hot case first (1) [switch-lowering]
case.c: main: optimized: inlined step (1) [inline]
case.c: pick: optimized: switch replaced by a range check and a load from a table of its answers (1) [switch-conv]
case.c: pick: optimized: switch of 40 cases lowered as a walk; clusters 1, tables 0, bit tests 0 (1) [switch-lowering]
case.c: line: optimized: switch replaced by a range check and the arithmetic its arms were doing (1) [switch-conv]
";
        assert_eq!(
            from_remarks(text),
            ["step: walk", "main: table", "pick: lookup", "pick: walk", "line: arithmetic"]
        );
    }

    #[test]
    fn the_assembly_shows_a_table_a_bit_test_and_a_lookup_and_compares_where_it_shows_none() {
        let text = "\
\t.type\tmain, @function
main:
\tnotrack jmp\t*%rdx
\t.section\t.rodata
.L4:
\t.long\t.L5-.L4
\t.long\t.L6-.L4
\t.text
\t.size\tmain, .-main
\t.type\tpick, @function
pick:
\tbtq\t%rdi, %rax
\tleaq\tCSWTCH.3(%rip), %rdx
\t.type\tflat, @function
flat:
\tcmpl\t$4, %edi
\t.section\t.rodata
CSWTCH.3:
\t.long\t7
";
        assert_eq!(
            from_assembly(text),
            ["main: table", "pick: bit-test", "pick: lookup", "flat: compares"]
        );
    }

    #[test]
    fn a_jump_through_a_pointer_is_not_a_table() {
        let text = "\t.type\tcall, @function\ncall:\n\tjmp\t*%rax\n";
        assert_eq!(from_assembly(text), ["call: compares"]);
    }

    #[test]
    fn a_tree_and_a_walk_are_both_compares_and_compares_goes_when_there_is_more() {
        assert_eq!(used(&["a: tree".to_owned(), "b: walk".to_owned()]), "compares");
        assert_eq!(used(&["a: arithmetic".to_owned(), "a: walk".to_owned()]), "compares");
        assert_eq!(used(&["a: lookup".to_owned(), "a: walk".to_owned()]), "lookup");
        assert_eq!(used(&["a: tree".to_owned(), "b: table".to_owned()]), "table");
        assert_eq!(
            used(&["main: table".to_owned(), "pick: lookup".to_owned(), "x: compares".to_owned()]),
            "lookup, table"
        );
    }
}
