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
    cast_chain(sink);
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
///
/// The negative constants are here for the rewrites that are not about magic numbers at all.
/// Multiplying or dividing by minus one is a subtraction from nothing and the remainder of minus
/// one is nothing, and the pair that makes those worth a case is the most negative value of the
/// type over minus one. The quotient there is not representable, so C leaves it undefined and the
/// generator refuses the case rather than asserting an answer, which is the whole reason the
/// refusal is written where the quotient is computed rather than where the result is checked.
fn strength(sink: &mut Sink<'_>) {
    const CONSTANTS: &[i128] = &[-2, -1, 1, 2, 3, 4, 5, 7, 8, 10, 16, 64, 100, 255, 1000, 1024];
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

/// Two casts in a row, which is the whole of the conversion algebra.
///
/// A cast chain is the one shape where a rewrite is about two instructions rather than one, and
/// the pair is always either a single conversion or no conversion at all. C writes these without
/// being asked, because the integer promotions widen an operand and the store behind it narrows
/// the answer again, so a compiler meets far more of them than a programmer ever writes. Writing
/// them on purpose is what gives a rule about a pair a case that reaches it, and the facet exists
/// because measuring the rewrites against promoted arithmetic alone only ever reached the two
/// chains that promotion happens to produce.
///
/// The six shapes are named for what the pair does. `round-trip` widens and comes straight back,
/// so both conversions go. `stop-early` comes back to a width still above the source, which is
/// the same widening stopping sooner. `below-source` comes back past the source, where the
/// widening never mattered. `narrow-twice` and `widen-twice` are two of a kind in a row. `mixed`
/// is a widening of a widening where the two disagree about sign, which is the one chain whose
/// answer depends on which of them happened first.
///
/// The middle and final types carry the source's signedness, except in `mixed` where an unsigned
/// source goes through signed types. That is not an exotic case. Promotion to `int` is signed
/// whatever the operand was, so it is the chain real C reaches most often.
///
/// Narrowing a value that does not fit is implementation defined rather than undefined, and both
/// compilers under test wrap, which is what [`Ty::convert`] models and what these cases expect.
fn cast_chain(sink: &mut Sink<'_>) {
    // Every chain worth writing, as the shape it belongs to and the three widths in order. The
    // widths are here rather than types because each one is emitted at both signednesses, and a
    // table of types would say the same thing twice.
    const CHAINS: &[(&str, u32, u32, u32)] = &[
        ("round-trip", 8, 16, 8),
        ("round-trip", 8, 32, 8),
        ("round-trip", 8, 64, 8),
        ("round-trip", 16, 32, 16),
        ("round-trip", 16, 64, 16),
        ("round-trip", 32, 64, 32),
        ("stop-early", 8, 32, 16),
        ("stop-early", 8, 64, 16),
        ("stop-early", 8, 64, 32),
        ("stop-early", 16, 64, 32),
        ("below-source", 16, 32, 8),
        ("below-source", 16, 64, 8),
        ("below-source", 32, 64, 8),
        ("below-source", 32, 64, 16),
        ("narrow-twice", 64, 32, 16),
        ("narrow-twice", 64, 32, 8),
        ("narrow-twice", 64, 16, 8),
        ("narrow-twice", 32, 16, 8),
        ("widen-twice", 8, 16, 32),
        ("widen-twice", 8, 16, 64),
        ("widen-twice", 8, 32, 64),
        ("widen-twice", 16, 32, 64),
    ];
    const SHAPES: &[&str] =
        &["round-trip", "stop-early", "below-source", "narrow-twice", "widen-twice", "mixed"];
    for &shape in SHAPES {
        // `mixed` is the widening chains again with the signedness pulled apart, so it reads the
        // same rows and needs an unsigned source to have anything to disagree about.
        let mixed = shape == "mixed";
        let wanted = if mixed { "widen-twice" } else { shape };
        for &ty in Ty::ALL {
            if !sink.wants(Facet::Narrowing) {
                return;
            }
            if mixed && ty.signed() {
                continue;
            }
            let rows: Vec<&(&str, u32, u32, u32)> = CHAINS
                .iter()
                .filter(|&&(kind, from, _, _)| kind == wanted && from == ty.bits())
                .collect();
            if rows.is_empty() {
                continue;
            }
            let signed = if mixed { true } else { ty.signed() };
            let mut program = Program::new(format!("{shape} cast chains from {}", ty.c_name()));
            for (at, &value) in spread(&interesting(ty), 5).iter().enumerate() {
                let a = format!("a{at}");
                program.input(ty, &a, value);
                for &&(_, _, through, to) in &rows {
                    let mid = at_width(through, signed);
                    let dst = at_width(to, signed);
                    let slot = format!("c{at}_{through}_{to}");
                    program.line(format!(
                        "{} {slot} = ({})({}){a};",
                        dst.c_name(),
                        dst.c_name(),
                        mid.c_name()
                    ));
                    program.check(dst.promoted(), &slot, dst.convert(mid.convert(value)));
                }
                program.blank();
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

/// The integer type of this width and signedness.
fn at_width(bits: u32, signed: bool) -> Ty {
    match (bits, signed) {
        (8, true) => Ty::I8,
        (8, false) => Ty::U8,
        (16, true) => Ty::I16,
        (16, false) => Ty::U16,
        (32, true) => Ty::I32,
        (32, false) => Ty::U32,
        (64, true) => Ty::I64,
        _ => Ty::U64,
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
///
/// One program per group rather than one per type, because a type on its own is too coarse to
/// report against. The tier one rule set in `crates/rucc-opt/rules/simplify.rules` is a hundred
/// and twenty five separate rules, and a single program per type that exercises all of them
/// gives one size number for the lot, so a rule that stopped firing is invisible next to the
/// twenty that still do. The groups below are the families that file is written in, and a size
/// number per group is a number that moves when one family lands and does not when it does not.
fn simplify(sink: &mut Sink<'_>) {
    for &ty in Ty::ALL {
        for &group in GROUPS {
            if !sink.wants(Facet::Simplify) {
                return;
            }
            // The last group is about arithmetic C widened and the program narrowed again, so
            // there is nothing for it to say about a type that was never widened.
            if group == "narrowed" && ty.bits() >= 32 {
                continue;
            }
            let mut program = Program::new(format!("{group} identities on {}", ty.c_name()));
            for (at, &operand) in spread(&interesting(ty), 6).iter().enumerate() {
                let x = format!("x{at}");
                let mut emitted = false;
                for (expr, value, out) in identities(group, ty, &x, operand) {
                    if !emitted {
                        program.input(ty, &x, operand);
                        emitted = true;
                    }
                    program.check(out, &expr, value);
                }
                if emitted {
                    program.blank();
                }
            }
            sink.push(
                Facet::Simplify,
                Axes::of([("type", ty.name()), ("group", group)]),
                Dialect::C17,
                program,
            );
        }
    }
    one_bit_identities(sink);
}

/// The families the identities fall into, one program each.
const GROUPS: &[&str] = &[
    "additive",
    "multiplicative",
    "bitwise-self",
    "bitwise-constant",
    "shift",
    "involution",
    "narrowed",
    "canonical",
];

/// Every identity in a group that has a defined answer for this operand.
///
/// The constant is written on both sides wherever the operation is commutative. That is not
/// padding. A rewrite table matches a shape, so a rule about the constant on the right is a
/// different rule from one about the constant on the left, and a compiler can easily have one
/// and not the other. Writing only `x + 0` would leave half the table with no evidence.
fn identities(group: &str, ty: Ty, x: &str, operand: i128) -> Vec<(String, i128, Ty)> {
    // Where the arithmetic actually happens, which is not always where the operand lives. Two
    // `unsigned char` values are added as `int`, so the identity a narrow case exercises is the
    // one at `int` width, and the constant has to be written at that width to say so. A mask of
    // `255u` against an `unsigned char` leaves the value alone as well, but it is a masking rule
    // rather than an all ones rule, and the two are not the same rewrite.
    let wide = ty.promoted();
    let zero = lit(wide, 0);
    let one = lit(wide, 1);
    let ones = lit(wide, all_ones(wide));
    let set = all_ones(wide);
    let candidates: Vec<(String, Option<i128>, Ty)> = match group {
        "additive" => vec![
            (format!("{x} + {zero}"), eval(Op::Add, wide, operand, 0), wide),
            (format!("{zero} + {x}"), eval(Op::Add, wide, 0, operand), wide),
            (format!("{x} - {zero}"), eval(Op::Sub, wide, operand, 0), wide),
            (format!("{x} - {x}"), eval(Op::Sub, ty, operand, operand), wide),
        ],
        "multiplicative" => vec![
            (format!("{x} * {one}"), eval(Op::Mul, wide, operand, 1), wide),
            (format!("{one} * {x}"), eval(Op::Mul, wide, 1, operand), wide),
            (format!("{x} * {zero}"), eval(Op::Mul, wide, operand, 0), wide),
            (format!("{zero} * {x}"), eval(Op::Mul, wide, 0, operand), wide),
            (format!("{x} / {one}"), eval(Op::Div, wide, operand, 1), wide),
            (format!("{x} % {one}"), eval(Op::Rem, wide, operand, 1), wide),
        ],
        "bitwise-self" => vec![
            (format!("{x} & {x}"), eval(Op::And, ty, operand, operand), wide),
            (format!("{x} | {x}"), eval(Op::Or, ty, operand, operand), wide),
            (format!("{x} ^ {x}"), eval(Op::Xor, ty, operand, operand), wide),
        ],
        "bitwise-constant" => vec![
            (format!("{x} & {zero}"), eval(Op::And, wide, operand, 0), wide),
            (format!("{zero} & {x}"), eval(Op::And, wide, 0, operand), wide),
            (format!("{x} & {ones}"), eval(Op::And, wide, operand, set), wide),
            (format!("{ones} & {x}"), eval(Op::And, wide, set, operand), wide),
            (format!("{x} | {zero}"), eval(Op::Or, wide, operand, 0), wide),
            (format!("{zero} | {x}"), eval(Op::Or, wide, 0, operand), wide),
            (format!("{x} | {ones}"), eval(Op::Or, wide, operand, set), wide),
            (format!("{ones} | {x}"), eval(Op::Or, wide, set, operand), wide),
            (format!("{x} ^ {zero}"), eval(Op::Xor, wide, operand, 0), wide),
            (format!("{zero} ^ {x}"), eval(Op::Xor, wide, 0, operand), wide),
        ],
        // A shift by nothing, at all three of the shifts the IR has. Which one a case reaches is
        // decided by the type: a left shift is one instruction whatever the sign, and a right
        // shift on a signed operand keeps the sign bit where an unsigned one does not, so the
        // signed and the unsigned program of this group are about different rules.
        "shift" => vec![
            (format!("{x} << 0"), eval(Op::Shl, ty, operand, 0), wide),
            (format!("{x} >> 0"), eval(Op::Shr, ty, operand, 0), wide),
        ],
        // An identity that is not one until the width comes back off. C widens an `unsigned
        // char` to `int` before it does anything, so `x & 255` at `int` width really does mask
        // and is nobody's identity there. Narrow it back to the eight bits the program stores it
        // in and the constant is every bit set, which is the identity. Every case in this group
        // is written that way round: a constant chosen so that the operation does something at
        // `int` width and nothing at the width the answer is kept at.
        //
        // This is the group that says whether the narrow widths of a rule table are reachable at
        // all. They are not reachable from anything C writes directly, because the promotion
        // happens first, so a table with a rule per width has most of its rules waiting on a
        // pass that puts the width back. What the size number here says is whether the compiler
        // looks again after it has done that.
        "narrowed" => {
            let step = 1i128 << ty.bits();
            let mask = step - 1;
            let name = ty.c_name();
            let narrowed = |op: Op, left: i128, right: i128| {
                eval(op, wide, left, right).map(|value| ty.convert(value))
            };
            let mut out = Vec::new();
            // Both orders wherever the operator commutes, the same as every other group, and
            // here it matters more rather than less. The narrow half of a rule table is the half
            // nothing could reach until the compiler learned to look again after it puts the
            // width back, so it is the half most likely to have been written on one side only
            // and never noticed.
            for (op, sign, right, commutes) in [
                (Op::Add, "+", step, true),
                (Op::Sub, "-", step, false),
                (Op::Mul, "*", step + 1, true),
                (Op::Mul, "*", step, true),
                (Op::And, "&", mask, true),
                (Op::Or, "|", mask, true),
                (Op::And, "&", step, true),
                (Op::Or, "|", step, true),
                (Op::Xor, "^", step, true),
            ] {
                let k = lit(wide, right);
                out.push((format!("({name})({x} {sign} {k})"), narrowed(op, operand, right), wide));
                if commutes {
                    out.push((
                        format!("({name})({k} {sign} {x})"),
                        narrowed(op, right, operand),
                        wide,
                    ));
                }
            }
            // The identities that need no constant, narrowed the same way. A value against itself
            // and a shift by nothing are rules at every width like the rest, and the promotion
            // hides them at the narrow ones just as thoroughly, so they belong in this group
            // rather than being left to the groups that are about the promoted width.
            for (op, sign) in [(Op::And, "&"), (Op::Or, "|"), (Op::Xor, "^"), (Op::Sub, "-")] {
                out.push((
                    format!("({name})({x} {sign} {x})"),
                    narrowed(op, operand, operand),
                    wide,
                ));
            }
            // Nothing here shifts, and nothing here divides, and both are worth saying because
            // they look like omissions and are not. A rule at a narrow width is reachable from C
            // only when the same expression is not already a rewrite at `int`. The peephole runs
            // before the narrowing as well as after it, so an expression that is an identity at
            // both widths is taken at the promoted one and there is nothing left to narrow: `x <<
            // 0` is written the same way at every width, so the compiler can never be shown a
            // one byte shift by nothing. The masks and the mirrors above work precisely because
            // they are not identities at `int`, which is what leaves them standing long enough to
            // have their width put back. Division is out for a different reason: narrowing it is
            // unsound without a range, since `-128 / -1` is defined at `int` and traps at one
            // byte, so no pass produces a narrow divide at all. Between them that is fourteen
            // rules that are proved and unreachable, which is a fact about the language and the
            // pass order rather than a gap here.
            out
        }
        // A constant on the wrong side of a commutative operation, where the constant is not an
        // identity and the operation therefore survives.
        //
        // This is tier three of the rule set, the canonicalisations, and it is the one group here
        // that is not about an operation going away. `3 + x` and `x + 3` compute the same thing
        // and neither is cheaper than the other in the IR, so nothing above measures whether a
        // compiler puts them into one shape. What it costs to skip is paid twice over: every rule
        // about a constant has to be written on both sides, and a machine whose add takes an
        // immediate on the right only has to load the constant into a register first.
        //
        // The constants are chosen so that no earlier tier fires. Zero, one and every bit set are
        // identities and are covered by the groups above, and a rewrite that takes the operation
        // away leaves nothing for this one to be about. Three, five, twelve, nine and six are
        // none of those, at every width they are written at.
        //
        // Both sides again, and here the second side is the control rather than a second rule.
        // The already canonical form is what the swapped form is supposed to turn into, so a
        // compiler that gets one of them wrong and the other right is a compiler whose
        // canonicalisation changed the answer.
        "canonical" => {
            let mut out = Vec::new();
            for (op, sign, right) in [
                (Op::Add, "+", 3),
                (Op::Mul, "*", 5),
                (Op::And, "&", 12),
                (Op::Or, "|", 9),
                (Op::Xor, "^", 6),
            ] {
                if !wide.holds(right) {
                    continue;
                }
                let k = lit(wide, right);
                out.push((format!("{k} {sign} {x}"), eval(op, wide, right, operand), wide));
                out.push((format!("{x} {sign} {k}"), eval(op, wide, operand, right), wide));
            }
            out
        }
        // Two of a thing that undoes itself, and a comparison of a value with itself. Neither
        // needs a constant, and both need the compiler to look through one instruction to the
        // one that produced its operand, which is a strictly harder match than the groups above.
        _ => vec![
            (
                format!("~~{x}"),
                eval(Op::Not, ty, operand, 0)
                    .and_then(|value| eval(Op::Not, result_ty(Op::Not, ty), value, 0)),
                wide,
            ),
            (
                format!("-(-{x})"),
                eval(Op::Neg, ty, operand, 0)
                    .and_then(|value| eval(Op::Neg, result_ty(Op::Neg, ty), value, 0)),
                wide,
            ),
            (format!("{x} == {x}"), Some(1), Ty::I32),
            (format!("{x} != {x}"), Some(0), Ty::I32),
            (format!("{x} < {x}"), Some(0), Ty::I32),
            (format!("{x} > {x}"), Some(0), Ty::I32),
            (format!("{x} <= {x}"), Some(1), Ty::I32),
            (format!("{x} >= {x}"), Some(1), Ty::I32),
        ],
    };
    candidates
        .into_iter()
        .filter_map(|(expr, value, out)| value.map(|value| (expr, value, out)))
        .collect()
}

/// The same identities at one bit, which is the width a truth value has.
///
/// Worth a program of its own because one bit is the width where the constants change meaning.
/// Every bit set is `1` rather than `-1`, so `x | 1` is the rule that says every bit is set and
/// `x & 1` is the rule that changes nothing, which is the other way round from every width
/// above. A rule set written by copying the wider cases and leaving the constants alone would
/// pass every program in this facet except this one.
///
/// The operands are a `_Bool` and a comparison, because those are the two things in C that carry
/// one bit. Both go through the usual promotions before an operator sees them, so what a
/// compiler does with the width is its own business, and what is checked here is that the answer
/// survived whatever it did.
fn one_bit_identities(sink: &mut Sink<'_>) {
    if !sink.wants(Facet::Simplify) {
        return;
    }
    let mut program = Program::new("algebraic identities on one bit values");
    program.top("static volatile int truth_in = 1;");
    program.top("static volatile int falsity_in = 0;");
    program.top("static volatile int counter_in = 7;");
    program.line("_Bool p = truth_in != 0;");
    program.line("_Bool q = falsity_in != 0;");
    program.line("int counter = counter_in;");
    program.line("_Bool r = counter > 3;");
    program.blank();
    for (name, value) in [("p", 1i128), ("q", 0), ("r", 1)] {
        program.check(Ty::I32, &format!("{name} & {name}"), value);
        program.check(Ty::I32, &format!("{name} | {name}"), value);
        program.check(Ty::I32, &format!("{name} ^ {name}"), 0);
        program.check(Ty::I32, &format!("{name} & 0"), 0);
        program.check(Ty::I32, &format!("0 & {name}"), 0);
        program.check(Ty::I32, &format!("{name} & 1"), value);
        program.check(Ty::I32, &format!("1 & {name}"), value);
        program.check(Ty::I32, &format!("{name} | 0"), value);
        program.check(Ty::I32, &format!("0 | {name}"), value);
        program.check(Ty::I32, &format!("{name} | 1"), 1);
        program.check(Ty::I32, &format!("1 | {name}"), 1);
        program.check(Ty::I32, &format!("{name} ^ 0"), value);
        program.check(Ty::I32, &format!("0 ^ {name}"), value);
        program.blank();
    }
    sink.push(
        Facet::Simplify,
        Axes::of([("type", "bool"), ("group", "one-bit")]),
        Dialect::C17,
        program,
    );
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
    fn every_identity_group_is_generated_at_every_type() {
        let cases = cases_for(Facet::Simplify);
        for ty in ["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"] {
            for group in [
                "additive",
                "multiplicative",
                "bitwise-self",
                "bitwise-constant",
                "shift",
                "canonical",
            ] {
                assert!(
                    cases.iter().any(|c| {
                        c.axes.get("type") == Some(ty) && c.axes.get("group") == Some(group)
                    }),
                    "no {group} program for {ty}"
                );
            }
        }
    }

    #[test]
    fn a_commutative_identity_is_written_with_the_constant_on_both_sides() {
        // Which is the point of splitting this facet up. A table matches a shape, so the rule
        // for a constant on the left is a different rule from the one for a constant on the
        // right, and a case that only ever writes one of them can only ever prove one of them.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("i32") && c.axes.get("group") == Some("additive"))
            .expect("the signed int additive program");
        assert!(case.source.contains("x0 + (int)0"), "{}", case.source);
        assert!(case.source.contains("(int)0 + x0"), "{}", case.source);
    }

    #[test]
    fn a_canonicalisation_case_writes_a_constant_no_earlier_rule_takes_away() {
        // The group is about the constant being on the wrong side, so the operation has to still
        // be there when the compiler gets to it. Zero, one and every bit set are identities, and
        // a rewrite that removes the operation leaves nothing for this group to be about.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("i32") && c.axes.get("group") == Some("canonical"))
            .expect("the signed int canonical program");
        for both in ["(int)3 + x0", "x0 + (int)3", "(int)5 * x1", "x1 * (int)5"] {
            assert!(case.source.contains(both), "{both} is missing from {}", case.source);
        }
        for identity in ["(int)0 +", "(int)1 *", "(int)0 |", "(int)(-1) &"] {
            assert!(!case.source.contains(identity), "{identity} is an identity, {}", case.source);
        }
    }

    #[test]
    fn a_canonicalisation_case_reaches_the_width_a_long_operation_happens_at() {
        // The narrow types promote to `int` before anything happens to them, so an `i8` program
        // says nothing about an `i8` rule. A `long` does not promote, which makes this the one
        // type in the facet that can put a constant on the left of a sixty four bit operation.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("i64") && c.axes.get("group") == Some("canonical"))
            .expect("the signed long canonical program");
        assert!(case.source.contains("(long long)3ll + x0"), "{}", case.source);
        assert!(case.source.contains("(long long)6ll ^ x0"), "{}", case.source);
    }

    #[test]
    fn the_strength_cases_include_the_constants_that_are_not_magic_numbers() {
        // Minus one is the one multiplier and the one divisor whose cheaper form is a
        // subtraction rather than a multiply and a shift, and the remainder of it is nothing at
        // all. Without a case there is no evidence that the rewrite is right on any operand.
        let cases = cases_for(Facet::Strength);
        for op in ["mul", "div", "rem"] {
            let case = cases
                .iter()
                .find(|c| c.axes.get("type") == Some("i32") && c.axes.get("op") == Some(op))
                .unwrap_or_else(|| panic!("the signed int {op} program"));
            assert!(case.source.contains("((int)(-1))"), "{}", case.source);
        }
    }

    #[test]
    fn the_pair_with_no_answer_is_left_out_of_the_strength_cases() {
        // The most negative value over minus one. The quotient is not representable, so C17
        // 6.5.5p6 leaves both the quotient and the remainder undefined, and a case that asserted
        // either would report a compiler that emits the divide as broken when it raises.
        let cases = cases_for(Facet::Strength);
        for (op, sign) in [("div", '/'), ("rem", '%')] {
            let case = cases
                .iter()
                .find(|c| c.axes.get("type") == Some("i32") && c.axes.get("op") == Some(op))
                .unwrap_or_else(|| panic!("the signed int {op} program"));
            // `x0` is the most negative value of the type, which the program writes as
            // `(-2147483647 - 1)` because the literal on its own is not one.
            assert!(case.source.contains("(-2147483647 - 1)"), "{}", case.source);
            let refused = format!("x0 {sign} ((int)(-1))");
            assert!(!case.source.contains(&refused), "{refused} has no answer, {}", case.source);
            // And the operand next to it does have one, so the omission above is this pair and
            // not the constant being dropped everywhere.
            let kept = format!("x0 {sign} ((int)(-2))");
            assert!(case.source.contains(&kept), "{kept} is missing, {}", case.source);
            let also = format!("x1 {sign} ((int)(-1))");
            assert!(case.source.contains(&also), "{also} is missing, {}", case.source);
        }
    }

    #[test]
    fn the_all_ones_constant_is_written_at_the_width_the_operation_happens_at() {
        // An `unsigned char` operand is promoted to `int` before the `and`, so the constant that
        // makes the operation an identity is minus one at `int` width. Writing `255u` would
        // leave the value alone as well and would be a mask rather than the rule under test.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| {
                c.axes.get("type") == Some("u8") && c.axes.get("group") == Some("bitwise-constant")
            })
            .expect("the unsigned char bitwise program");
        assert!(case.source.contains("(int)(-1)"), "{}", case.source);
        assert!(!case.source.contains("& (unsigned char)"), "{}", case.source);
    }

    #[test]
    fn a_narrowed_identity_exists_only_where_there_was_a_promotion_to_undo() {
        let cases = cases_for(Facet::Simplify);
        for ty in ["i8", "u8", "i16", "u16"] {
            assert!(
                cases.iter().any(|c| {
                    c.axes.get("type") == Some(ty) && c.axes.get("group") == Some("narrowed")
                }),
                "no narrowed program for {ty}"
            );
        }
        for ty in ["i32", "u32", "i64", "u64"] {
            assert!(
                !cases.iter().any(|c| {
                    c.axes.get("type") == Some(ty) && c.axes.get("group") == Some("narrowed")
                }),
                "{ty} was never widened, so it has nothing to narrow back"
            );
        }
    }

    #[test]
    fn a_narrowed_case_uses_a_constant_that_does_something_at_int_width() {
        // Which is the whole point of the group. `x & 255` really does mask when the operand has
        // been promoted to `int`, and it is the identity at the eight bits the answer is kept
        // at, so a compiler only gets to remove it if it looks again after taking the width off.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("u8") && c.axes.get("group") == Some("narrowed"))
            .expect("the unsigned char narrowed program");
        assert!(case.source.contains("(unsigned char)(x0 & (int)255)"), "{}", case.source);
        assert!(case.source.contains("(unsigned char)(x0 * (int)257)"), "{}", case.source);
    }

    #[test]
    fn a_narrowed_identity_is_written_with_the_constant_on_both_sides_too() {
        // The rules at the narrow widths went unreachable for as long as nothing looked again
        // after the width came back off, so they are the ones most likely to exist on one side
        // only. Subtracting is the exception and is written once, because it does not commute.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("u8") && c.axes.get("group") == Some("narrowed"))
            .expect("the unsigned char narrowed program");
        for both in ["& (int)255", "| (int)255", "* (int)257", "+ (int)256", "^ (int)256"] {
            let (sign, k) = both.split_once(' ').expect("an operator and a constant");
            assert!(
                case.source.contains(&format!("(x0 {sign} {k})")),
                "no `x0 {sign} {k}`\n{}",
                case.source
            );
            assert!(
                case.source.contains(&format!("({k} {sign} x0)")),
                "no `{k} {sign} x0`\n{}",
                case.source
            );
        }
        assert!(case.source.contains("(x0 - (int)256)"), "{}", case.source);
        assert!(!case.source.contains("((int)256 - x0)"), "{}", case.source);
    }

    #[test]
    fn the_narrowed_group_covers_the_identities_that_need_no_constant() {
        // A value against itself and a shift by nothing are rules at the narrow widths like any
        // other, and the promotion hides them there exactly as well, so they are generated here
        // rather than being left to the groups that only ever reach `int`.
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("u8") && c.axes.get("group") == Some("narrowed"))
            .expect("the unsigned char narrowed program");
        for shape in ["(x0 & x0)", "(x0 | x0)", "(x0 ^ x0)", "(x0 - x0)"] {
            assert!(case.source.contains(shape), "no `{shape}`\n{}", case.source);
        }
        // Shifting and dividing are absent on purpose, and a program that added them back would
        // be adding source that cannot reach anything. A shift by nothing is an identity at
        // `int` too, so the peephole takes it before the narrowing runs, and narrowing a divide
        // is unsound without a range because `-128 / -1` is defined at `int` and traps at one
        // byte.
        for absent in ["(x0 << ", "(x0 >> ", "(x0 / ", "(x0 % "] {
            assert!(!case.source.contains(absent), "`{absent}` cannot fire\n{}", case.source);
        }
    }

    #[test]
    fn the_one_bit_program_writes_every_bit_set_as_one_rather_than_minus_one() {
        let cases = cases_for(Facet::Simplify);
        let case = cases
            .iter()
            .find(|c| c.axes.get("group") == Some("one-bit"))
            .expect("the one bit program");
        assert!(case.source.contains("_Bool p = truth_in != 0;"), "{}", case.source);
        assert!(case.source.contains("p | 1"), "{}", case.source);
        assert!(!case.source.contains("-1"), "{}", case.source);
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
