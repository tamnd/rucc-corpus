//! The transformations that happen inside one basic block.
//!
//! Everything here is decided by looking at a short window of straight line code. That makes
//! the cases easy to write and easy to reason about, and it makes them the first thing to run
//! when a compiler is misbehaving, because a compiler that gets constant folding wrong will
//! get everything downstream of it wrong too.

use super::{all_ones, lit, spread};
use crate::Sink;
use crate::emit::Program;
use crate::lang::{Op, Ty, eval, interesting, result_ty, shift_counts};
use corpus_model::{Axes, Dialect, Facet};

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    constant_fold(sink);
    strength(sink);
    simplify(sink);
    reassociate(sink);
    dead_code(sink);
    dead_store(sink);
    unreachable_code(sink);
}

/// Every operation on every pair of literal operands.
///
/// One program per type and operation, holding a grid of operand pairs. Packing many checks
/// into one program rather than emitting one program per pair is what keeps this facet from
/// being nine thousand tiny files, and it costs nothing: a wrong answer still names the case,
/// the operation and the operands, because the operands are in the source next to the line.
fn constant_fold(sink: &mut Sink<'_>) {
    for &ty in Ty::ALL {
        for &op in Op::ALL {
            if !sink.wants(Facet::ConstantFold) {
                return;
            }
            let mut program =
                Program::new(format!("fold {} on {} operands", op.name(), ty.c_name()));
            let lefts = spread(&interesting(ty), 8);
            let rights = if op.is_shift() { shift_counts(ty) } else { spread(&interesting(ty), 8) };
            let out = result_ty(op, ty);
            for &left in &lefts {
                for &right in &rights {
                    if op.is_unary() && right != lefts[0] {
                        continue;
                    }
                    let Some(value) = eval(op, ty, left, right) else {
                        continue;
                    };
                    let expr = if op.is_unary() {
                        format!("{}({})", op.spelling(), lit(ty, left))
                    } else if op.is_shift() {
                        format!("({}) {} {right}", lit(ty, left), op.spelling())
                    } else {
                        format!("({}) {} ({})", lit(ty, left), op.spelling(), lit(ty, right))
                    };
                    program.check(out, &expr, value);
                }
            }
            sink.push(
                Facet::ConstantFold,
                Axes::of([("type", ty.name()), ("op", op.name())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The multipliers and divisors a compiler is expected to turn into something cheaper.
///
/// The operand is unknown, so nothing here folds. What is being tested is that replacing a
/// division by seven with a multiply and a shift gives the same answer as the division, for
/// every operand including the negative ones and the extremes where the usual derivation of
/// the magic number is easiest to get wrong.
fn strength(sink: &mut Sink<'_>) {
    const CONSTANTS: &[i128] = &[1, 2, 3, 4, 5, 7, 8, 10, 16, 64, 100, 255, 1000, 1024];
    for &ty in Ty::ALL {
        for &op in &[Op::Mul, Op::Div, Op::Rem] {
            if !sink.wants(Facet::Strength) {
                return;
            }
            let mut program =
                Program::new(format!("{} of an unknown {} by a constant", op.name(), ty.c_name()));
            let out = result_ty(op, ty);
            let operands = spread(&interesting(ty), 6);
            for (at, &operand) in operands.iter().enumerate() {
                let name = format!("x{at}");
                let mut emitted = false;
                for &constant in CONSTANTS {
                    if !ty.holds(constant) {
                        continue;
                    }
                    let Some(value) = eval(op, ty, operand, constant) else {
                        continue;
                    };
                    if !emitted {
                        program.input(ty, &name, operand);
                        emitted = true;
                    }
                    program.check(
                        out,
                        &format!("{name} {} ({})", op.spelling(), lit(ty, constant)),
                        value,
                    );
                }
                if emitted {
                    program.blank();
                }
            }
            sink.push(
                Facet::Strength,
                Axes::of([("type", ty.name()), ("op", op.name())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The algebraic identities, applied to an operand the compiler does not know.
///
/// Each of these should collapse to the operand itself, to a constant, or to a single
/// instruction. None of them may change the answer, and the answer is what is checked. A
/// compiler that folds `x - x` to zero and a compiler that leaves the subtraction alone both
/// pass here, which is correct: this facet proves the rewrite is sound, and the size numbers
/// in the report say whether it happened.
fn simplify(sink: &mut Sink<'_>) {
    for &ty in Ty::ALL {
        if !sink.wants(Facet::Simplify) {
            return;
        }
        let ones = all_ones(ty);
        let mut program = Program::new(format!("algebraic identities on {}", ty.c_name()));
        for (at, &operand) in spread(&interesting(ty), 6).iter().enumerate() {
            let x = format!("x{at}");
            program.input(ty, &x, operand);
            let wide = ty.promoted();
            let candidates: Vec<(String, Option<i128>, Ty)> = vec![
                (format!("{x} + {}", lit(ty, 0)), eval(Op::Add, ty, operand, 0), wide),
                (format!("{x} - {}", lit(ty, 0)), eval(Op::Sub, ty, operand, 0), wide),
                (format!("{x} * {}", lit(ty, 1)), eval(Op::Mul, ty, operand, 1), wide),
                (format!("{x} * {}", lit(ty, 0)), eval(Op::Mul, ty, operand, 0), wide),
                (format!("{x} / {}", lit(ty, 1)), eval(Op::Div, ty, operand, 1), wide),
                (format!("{x} % {}", lit(ty, 1)), eval(Op::Rem, ty, operand, 1), wide),
                (format!("{x} & {x}"), eval(Op::And, ty, operand, operand), wide),
                (format!("{x} | {x}"), eval(Op::Or, ty, operand, operand), wide),
                (format!("{x} ^ {x}"), eval(Op::Xor, ty, operand, operand), wide),
                (format!("{x} - {x}"), eval(Op::Sub, ty, operand, operand), wide),
                (format!("{x} & {}", lit(ty, 0)), eval(Op::And, ty, operand, 0), wide),
                (format!("{x} | {}", lit(ty, ones)), eval(Op::Or, ty, operand, ones), wide),
                (format!("{x} & {}", lit(ty, ones)), eval(Op::And, ty, operand, ones), wide),
                (format!("{x} ^ {}", lit(ty, 0)), eval(Op::Xor, ty, operand, 0), wide),
                (format!("{x} << 0"), eval(Op::Shl, ty, operand, 0), wide),
                (format!("{x} >> 0"), eval(Op::Shr, ty, operand, 0), wide),
                (
                    format!("~~{x}"),
                    eval(Op::Not, ty, operand, 0)
                        .and_then(|value| eval(Op::Not, result_ty(Op::Not, ty), value, 0)),
                    wide,
                ),
                (format!("{x} == {x}"), Some(1), Ty::I32),
                (format!("{x} < {x}"), Some(0), Ty::I32),
            ];
            for (expr, value, out) in candidates {
                let Some(value) = value else {
                    continue;
                };
                program.check(out, &expr, value);
            }
            program.blank();
        }
        sink.push(Facet::Simplify, Axes::of([("type", ty.name())]), Dialect::C17, program);
    }
}

/// Chains of associative operations, written every way round.
///
/// A left leaning chain, a right leaning one and a balanced tree all compute the same value,
/// and a compiler that reassociates is turning one into another. The three shapes are emitted
/// side by side in one program so the report can say whether they compiled to the same size,
/// which is the only way to tell whether reassociation actually ran.
fn reassociate(sink: &mut Sink<'_>) {
    for &ty in Ty::WIDE {
        for &op in &[Op::Add, Op::Mul, Op::And, Op::Or, Op::Xor] {
            if !sink.wants(Facet::Reassociate) {
                return;
            }
            let mut program = Program::new(format!(
                "left, right and balanced {} chains on {}",
                op.name(),
                ty.c_name()
            ));
            let operands: Vec<i128> = match op {
                Op::Add | Op::Mul => vec![1, 2, 3, 5],
                _ => vec![0x0f, 0x33, 0x55, 0xaa],
            };
            let names: Vec<String> = (0..operands.len()).map(|at| format!("a{at}")).collect();
            for (name, &value) in names.iter().zip(&operands) {
                program.input(ty, name, value);
            }
            let mut folded = operands[0];
            let mut ok = true;
            for &operand in &operands[1..] {
                match eval(op, ty, folded, operand) {
                    Some(value) => folded = value,
                    None => ok = false,
                }
            }
            if !ok {
                continue;
            }
            let sp = op.spelling();
            let [a, b, c, d] = [&names[0], &names[1], &names[2], &names[3]];
            let out = result_ty(op, ty);
            program.blank();
            program.check(out, &format!("((({a} {sp} {b}) {sp} {c}) {sp} {d})"), folded);
            program.check(out, &format!("({a} {sp} ({b} {sp} ({c} {sp} {d})))"), folded);
            program.check(out, &format!("(({a} {sp} {b}) {sp} ({c} {sp} {d}))"), folded);
            program.check(out, &format!("(({d} {sp} {c}) {sp} ({b} {sp} {a}))"), folded);
            sink.push(
                Facet::Reassociate,
                Axes::of([("type", ty.name()), ("op", op.name())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Computations nobody reads, next to one that somebody does.
///
/// The dead work is written so that removing it is legal and leaving it is harmless, and the
/// live work is written so that it cannot be removed. Correctness here is that the answer is
/// untouched. The interesting number is the size, because a compiler that emits the dead
/// chain is a compiler whose dead code elimination did not run.
fn dead_code(sink: &mut Sink<'_>) {
    for &ty in Ty::WIDE {
        for depth in [1usize, 4, 16] {
            if !sink.wants(Facet::DeadCode) {
                return;
            }
            let mut program = Program::new(format!(
                "a chain of {depth} dead {} computations beside one live one",
                ty.c_name()
            ));
            program.input(ty, "seed", 3);
            program.blank();
            program.line(format!("{} dead = seed;", ty.c_name()));
            for step in 0..depth {
                program.line(format!("dead = dead * {} + {};", lit(ty, 3), lit(ty, step as i128)));
            }
            program.blank();
            let Some(live) = eval(Op::Add, ty, 3, 1) else {
                continue;
            };
            program.check(ty.promoted(), &format!("seed + {}", lit(ty, 1)), live);
            sink.push(
                Facet::DeadCode,
                Axes::of([("type", ty.name()), ("depth", &depth.to_string())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Stores that a later store makes invisible.
///
/// Three shapes, because they need three different pieces of reasoning to remove. A plain
/// local needs nothing. An array element needs the two indices to be known equal. A local
/// whose address was taken needs the compiler to know the pointer did not escape.
fn dead_store(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["local", "array", "address-taken"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::DeadStore) {
                return;
            }
            let mut program =
                Program::new(format!("overwritten {shape} stores of {}", ty.c_name()));
            program.input(ty, "seed", 5);
            program.blank();
            let name = ty.c_name();
            match shape {
                "local" => {
                    program.line(format!("{name} slot = seed;"));
                    program.line(format!("slot = seed + {};", lit(ty, 1)));
                    program.line(format!("slot = seed + {};", lit(ty, 2)));
                    program.line(format!("slot = seed + {};", lit(ty, 3)));
                }
                "array" => {
                    program.line(format!("{name} buffer[4];"));
                    program.line("buffer[0] = seed;".to_owned());
                    program.line(format!("buffer[0] = seed + {};", lit(ty, 1)));
                    program.line(format!("buffer[0] = seed + {};", lit(ty, 3)));
                    program.line(format!("{name} slot = buffer[0];"));
                }
                _ => {
                    program.line(format!("{name} slot = seed;"));
                    program.line(format!("{name} *at = &slot;"));
                    program.line(format!("*at = seed + {};", lit(ty, 1)));
                    program.line(format!("*at = seed + {};", lit(ty, 3)));
                }
            }
            program.blank();
            let Some(value) = eval(Op::Add, ty, 5, 3) else {
                continue;
            };
            program.check(ty.promoted(), "slot", value);
            sink.push(
                Facet::DeadStore,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Branches whose condition is a constant, and the arms that can never run.
///
/// The arm that cannot run is filled with something that would give the wrong answer if it
/// ever did, so a compiler that picks the wrong side fails loudly rather than by a size
/// difference nobody notices.
fn unreachable_code(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["if-false", "if-true", "while-false", "switch-constant", "early-return"];
    for &shape in SHAPES {
        if !sink.wants(Facet::UnreachableCode) {
            return;
        }
        let mut program = Program::new(format!("a {shape} whose dead arm must not run"));
        program.input(Ty::I32, "seed", 10);
        program.blank();
        program.line("int answer = 0;");
        match shape {
            "if-false" => {
                program.line("if (0) {");
                program.line_at(1, "answer = seed * 100;");
                program.line("} else {");
                program.line_at(1, "answer = seed + 1;");
                program.line("}");
            }
            "if-true" => {
                program.line("if (1) {");
                program.line_at(1, "answer = seed + 1;");
                program.line("} else {");
                program.line_at(1, "answer = seed * 100;");
                program.line("}");
            }
            "while-false" => {
                program.line("answer = seed + 1;");
                program.line("while (0) {");
                program.line_at(1, "answer = seed * 100;");
                program.line("}");
            }
            "switch-constant" => {
                program.line("switch (2) {");
                program.line_at(1, "case 1: answer = seed * 100; break;");
                program.line_at(1, "case 2: answer = seed + 1; break;");
                program.line_at(1, "default: answer = seed * 200; break;");
                program.line("}");
            }
            _ => {
                program.top("static int pick(int seed) {");
                program.top("    return seed + 1;");
                program.top("    return seed * 100;");
                program.top("}");
                program.line("answer = pick(seed);");
            }
        }
        program.blank();
        program.check(Ty::I32, "answer", 11);
        sink.push(Facet::UnreachableCode, Axes::of([("shape", shape)]), Dialect::C17, program);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, Sink};
    use corpus_model::{Expect, Facet};

    fn cases_for(facet: Facet) -> Vec<corpus_model::Case> {
        let opts = Options::all().only(&[facet]);
        let mut sink = Sink::new(&opts);
        super::generate(&mut sink);
        sink.into_cases()
    }

    #[test]
    fn constant_folding_covers_every_type_and_operation_pair_that_has_a_defined_answer() {
        let cases = cases_for(Facet::ConstantFold);
        assert!(cases.len() >= 90, "only {} programs", cases.len());
        for case in &cases {
            assert!(case.axes.get("type").is_some());
            assert!(case.axes.get("op").is_some());
        }
    }

    #[test]
    fn a_folding_case_carries_its_operands_in_the_source_so_a_failure_names_itself() {
        let cases = cases_for(Facet::ConstantFold);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("u8") && c.axes.get("op") == Some("add"))
            .expect("unsigned char addition");
        assert!(case.source.contains("(unsigned char)"), "{}", case.source);
    }

    #[test]
    fn strength_reduction_cases_use_an_operand_the_compiler_cannot_see() {
        for case in cases_for(Facet::Strength) {
            assert!(case.source.contains("static volatile"), "{}", case.id);
        }
    }

    #[test]
    fn every_identity_case_prints_at_least_one_value_per_operand_it_declares() {
        for case in cases_for(Facet::Simplify) {
            let inputs = case.source.matches("static volatile").count();
            let Expect::Output(text) = &case.expect else { panic!("{} should run", case.id) };
            assert!(text.lines().count() >= inputs, "{}", case.id);
        }
    }

    #[test]
    fn a_reassociation_case_writes_the_same_chain_four_ways_and_expects_one_answer() {
        for case in cases_for(Facet::Reassociate) {
            let Expect::Output(text) = &case.expect else { panic!("{} should run", case.id) };
            let lines: Vec<&str> = text.lines().collect();
            assert_eq!(lines.len(), 4, "{}", case.id);
            assert!(lines.windows(2).all(|w| w[0] == w[1]), "{}", case.id);
        }
    }

    #[test]
    fn a_dead_code_case_has_more_source_at_a_greater_depth_and_the_same_answer() {
        let cases = cases_for(Facet::DeadCode);
        let shallow = cases.iter().find(|c| c.axes.get("depth") == Some("1")).unwrap();
        let deep = cases
            .iter()
            .find(|c| {
                c.axes.get("depth") == Some("16") && c.axes.get("type") == shallow.axes.get("type")
            })
            .unwrap();
        assert!(deep.source.len() > shallow.source.len());
        assert_eq!(shallow.expect, deep.expect);
    }

    #[test]
    fn every_dead_store_shape_reads_the_last_value_written() {
        let cases = cases_for(Facet::DeadStore);
        for shape in ["local", "array", "address-taken"] {
            let case = cases
                .iter()
                .find(|c| c.axes.get("shape") == Some(shape))
                .unwrap_or_else(|| panic!("no case for {shape}"));
            assert_eq!(case.expect, Expect::Output("8\n".to_owned()), "{}", case.id);
        }
    }

    #[test]
    fn every_unreachable_case_expects_the_live_arm_and_not_the_dead_one() {
        let cases = cases_for(Facet::UnreachableCode);
        assert_eq!(cases.len(), 5);
        for case in cases {
            assert_eq!(case.expect, Expect::Output("11\n".to_owned()), "{}", case.id);
        }
    }
}
