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
    narrowing(sink);
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

/// Arithmetic C widened to `int` and then threw the width away again.
///
/// C says that two `unsigned char` operands are added as `int`. If the result is stored back
/// into an `unsigned char`, the top twenty four bits of that addition were never going to be
/// read, and a compiler is allowed to do the work at the narrow width instead. That is the
/// whole of the `narrow` pass in rucc, and it is worth evidence because it is a rewrite that
/// changes the width of an operation, which is exactly the kind of rewrite that is wrong on
/// the operands nobody tried.
///
/// The shapes are the cases the pass has to tell apart. Two of them, `wide-use` and
/// `divide`, are the ones where narrowing is not allowed, and they are here so that a pass
/// which narrows everything fails rather than scores.
fn narrowing(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["stored-back", "wide-use", "mixed-sign", "round-trip", "divide", "shift", "bit-field"];
    for &ty in Ty::ALL {
        if ty.bits() >= 32 {
            continue;
        }
        for &shape in SHAPES {
            if !sink.wants(Facet::Narrowing) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("{shape} arithmetic on {name}"));
            let operands = spread(&interesting(ty), 5);
            match shape {
                "bit-field" => {
                    // A bit field is the one place where the width is not a power of two, so
                    // it is the one place where narrowing has to work out the width rather
                    // than read it off the type.
                    program.top(
                        "struct packed { unsigned three : 3; unsigned five : 5; unsigned thirteen : 13; };"
                            .to_owned(),
                    );
                    program.input(ty, "seed", 7);
                    program.blank();
                    program.line("struct packed p;");
                    program.line("p.three = (unsigned)seed & 7u;");
                    program.line("p.five = (unsigned)seed & 31u;");
                    program.line("p.thirteen = (unsigned)seed & 8191u;");
                    program.blank();
                    program.check(Ty::U32, "p.three", 7);
                    program.check(Ty::U32, "p.five", 7);
                    program.check(Ty::U32, "p.thirteen", 7);
                    program.check(Ty::U32, "p.three + p.five + p.thirteen", 21);
                }
                "mixed-sign" => {
                    // The operands promote to `int` by different routes, and the result is
                    // stored at a width neither of them has. A pass that narrows on the
                    // width alone and forgets which promotion happened gets this wrong.
                    let other = if ty.signed() { unsigned_twin(ty) } else { signed_twin(ty) };
                    for (at, &left) in operands.iter().enumerate() {
                        let right = 3;
                        if !other.holds(right) {
                            continue;
                        }
                        let a = format!("a{at}");
                        let b = format!("b{at}");
                        program.input(ty, &a, left);
                        program.input(other, &b, right);
                        let Some(sum) = eval(Op::Add, Ty::I32, left, right) else {
                            continue;
                        };
                        program.line(format!("{name} sum{at} = ({name})({a} + {b});"));
                        program.check(ty.promoted(), &format!("sum{at}"), ty.convert(sum));
                    }
                }
                "divide" => {
                    // Division does not narrow the way addition does. The quotient of two
                    // `int` values is not the quotient of their low bytes, so a pass that
                    // treats it like the others prints a different number here.
                    for (at, &left) in operands.iter().enumerate() {
                        let right = 3;
                        let Some(quotient) = eval(Op::Div, ty, left, right) else {
                            continue;
                        };
                        let Some(remainder) = eval(Op::Rem, ty, left, right) else {
                            continue;
                        };
                        let a = format!("a{at}");
                        program.input(ty, &a, left);
                        program.line(format!("{name} q{at} = ({name})({a} / {});", lit(ty, right)));
                        program.line(format!("{name} r{at} = ({name})({a} % {});", lit(ty, right)));
                        program.check(ty.promoted(), &format!("q{at}"), ty.convert(quotient));
                        program.check(ty.promoted(), &format!("r{at}"), ty.convert(remainder));
                    }
                }
                "shift" => {
                    // The value narrows and the count does not. A count of three is inside
                    // the narrow width, and a count of nine is not, which is the case where
                    // doing the shift at the narrow width would lose every bit.
                    for (at, &left) in operands.iter().enumerate() {
                        let a = format!("a{at}");
                        program.input(ty, &a, left);
                        for count in [3i128, 9] {
                            let Some(shifted) = eval(Op::Shl, ty, left, count) else {
                                continue;
                            };
                            let slot = format!("s{at}_{count}");
                            program.line(format!("{name} {slot} = ({name})({a} << {count});"));
                            program.check(ty.promoted(), &slot, ty.convert(shifted));
                        }
                    }
                }
                "round-trip" => {
                    // Narrowed, widened again and compared. The comparison is at `int`, so
                    // the sign of the widening is what decides the answer, and a pass that
                    // narrowed the wrong way round prints zero where the case says one.
                    for (at, &left) in operands.iter().enumerate() {
                        let a = format!("a{at}");
                        program.input(ty, &a, left);
                        let Some(sum) = eval(Op::Add, ty, left, 1) else {
                            continue;
                        };
                        let narrowed = ty.convert(sum);
                        program.line(format!("{name} n{at} = ({name})({a} + 1);"));
                        program.line(format!("int w{at} = (int)n{at};"));
                        program.check(Ty::I32, &format!("w{at} == {narrowed}"), 1);
                        program.check(ty.promoted(), &format!("n{at}"), narrowed);
                    }
                }
                "wide-use" => {
                    // The sum is read at `int` width, so it may not be narrowed at all. The
                    // operands are the extremes of the type, which is where the wide answer
                    // and the narrow one differ.
                    for (at, &left) in operands.iter().enumerate() {
                        let a = format!("a{at}");
                        program.input(ty, &a, left);
                        let Some(sum) = eval(Op::Add, ty, left, ty.max()) else {
                            continue;
                        };
                        program.line(format!("int w{at} = {a} + {};", lit(ty, ty.max())));
                        program.check(Ty::I32, &format!("w{at}"), sum);
                    }
                }
                _ => {
                    // The plain case. Every operation, stored back at the width it came from.
                    for (at, &left) in operands.iter().enumerate() {
                        let a = format!("a{at}");
                        program.input(ty, &a, left);
                        for &op in &[Op::Add, Op::Sub, Op::Mul, Op::And, Op::Or, Op::Xor] {
                            let right = 5;
                            if !ty.holds(right) {
                                continue;
                            }
                            let Some(value) = eval(op, ty, left, right) else {
                                continue;
                            };
                            let slot = format!("v{at}_{}", op.name());
                            program.line(format!(
                                "{name} {slot} = ({name})({a} {} {});",
                                op.spelling(),
                                lit(ty, right)
                            ));
                            program.check(ty.promoted(), &slot, ty.convert(value));
                        }
                        program.blank();
                    }
                }
            }
            sink.push(
                Facet::Narrowing,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The unsigned type of the same width, for the cases about mixed signedness.
fn unsigned_twin(ty: Ty) -> Ty {
    match ty {
        Ty::I8 => Ty::U8,
        Ty::I16 => Ty::U16,
        Ty::I32 => Ty::U32,
        other => other,
    }
}

/// The signed type of the same width.
fn signed_twin(ty: Ty) -> Ty {
    match ty {
        Ty::U8 => Ty::I8,
        Ty::U16 => Ty::I16,
        Ty::U32 => Ty::I32,
        other => other,
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
///
/// The last three shapes go further and put a call to a function nothing defines in the dead
/// arm. A compiler that keeps the arm emits a relocation against a name the linker cannot
/// resolve, so the case does not build at all rather than printing the wrong number. That is
/// the strongest oracle in the corpus, it needs nothing from the harness beyond a link that
/// already happens, and it is what `gcc.c-torture/execute/medce-1.c` is for. It is also the
/// reason a dead arm has to go at every optimization level including `-O0`: a program that
/// links at `-O2` and not at `-O0` is broken at `-O0`, not unoptimized there.
fn unreachable_code(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "if-false",
        "if-true",
        "while-false",
        "switch-constant",
        "early-return",
        "dead-call",
        "dead-call-compare",
        "dead-call-label",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::UnreachableCode) {
            return;
        }
        let mut program = Program::new(format!("a {shape} whose dead arm must not run"));
        program.input(Ty::I32, "seed", 10);
        if shape.starts_with("dead-call") {
            // Declared and never defined, on purpose. The name is the assertion.
            program.top("void corpus_link_error(void);");
        }
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
            "early-return" => {
                program.top("static int pick(int seed) {");
                program.top("    return seed + 1;");
                program.top("    return seed * 100;");
                program.top("}");
                program.line("answer = pick(seed);");
            }
            "dead-call" => {
                program.line("if (0) {");
                program.line_at(1, "corpus_link_error();");
                program.line_at(1, "answer = seed * 100;");
                program.line("} else {");
                program.line_at(1, "answer = seed + 1;");
                program.line("}");
            }
            "dead-call-compare" => {
                // The condition is a comparison rather than a literal, so a compiler that
                // only looks for the constant zero keeps the call. Both operands are
                // constants, which is as far as this case asks anything to go.
                program.line("answer = seed + 1;");
                program.line("if (3 > 5) {");
                program.line_at(1, "corpus_link_error();");
                program.line_at(1, "answer = seed * 100;");
                program.line("}");
            }
            _ => {
                // `medce-1.c`. The label is inside the body of the dead `if`, so control does
                // reach the assignment and never reaches the call. A compiler that deletes
                // the whole compound statement gets this as wrong as one that keeps all of
                // it: the first prints zero, the second does not link.
                program.line("switch (seed - 9) {");
                program.line_at(1, "case 0:");
                program.line_at(2, "if (0) { corpus_link_error(); case 1: answer = seed + 1; }");
                program.line("}");
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
        assert_eq!(cases.len(), 8);
        for case in cases {
            assert_eq!(case.expect, Expect::Output("11\n".to_owned()), "{}", case.id);
        }
    }

    #[test]
    fn a_dead_call_case_names_a_function_it_never_defines() {
        // Which is the whole oracle. If the declaration ever gains a body, the case still
        // prints eleven and stops proving anything, and nothing else would notice.
        let cases = cases_for(Facet::UnreachableCode);
        let dead: Vec<_> = cases
            .iter()
            .filter(|c| c.axes.get("shape").is_some_and(|s| s.starts_with("dead-call")))
            .collect();
        assert_eq!(dead.len(), 3);
        for case in dead {
            assert!(case.source.contains("void corpus_link_error(void);"), "{}", case.id);
            assert!(case.source.contains("corpus_link_error();"), "{}", case.id);
            assert!(!case.source.contains("corpus_link_error(void) {"), "{}", case.id);
        }
    }
}
