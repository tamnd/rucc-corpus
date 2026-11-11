//! The arithmetic conversions of kencc, which keep the sign of an unsigned operand.
//!
//! `kencc-conversions` is the table `tab` of `cc/sub.c` in kencc, which `2c(1)` calls the unsigned preserving rules.
//! The common type of two integer operands has the larger rank of the two, and it is unsigned when either operand is unsigned.
//! So `unsigned char` with `int` is `unsigned int`, and `(c - 1) < 0` is false for an `unsigned char c`, where C gives true.
//! An arithmetic operator then makes the type at least an `int`, and keeps the sign, so `unsigned char` with `unsigned char` is `unsigned int`.
//! A comparison and a conditional do not, so `signed char` with `unsigned char` is `unsigned char` there.
//! A compound assignment does not use the table: kencc casts the right side to the type of the left side, so `x /= b` with an `int x` and an `unsigned int b` is a signed divide.
//! Each case is one pair of operand types, and prints the result of each operator in the type of the table.
//! A conditional is stored in a variable of its type before it is printed, and `sizeof` gives the width of the type.
//! `6c` converts a conditional narrower than an `int` to `vlong` arm by arm, and an arm in a register loses its sign, so the cast of the conditional itself depends on the registers.
//! The answers were compared with `6c` on 9front amd64.
//! Only rucc has the dialect, so each case has the tag `rucc-dialect`.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, RUCC_DIALECT};

/// The value of the left operand: a negative one for a signed type.
const fn left(ty: Ty) -> i128 {
    if ty.signed() { -7 } else { 5 }
}

/// The value of the right operand, which is never zero in any type.
const fn right(ty: Ty) -> i128 {
    if ty.signed() { -2 } else { 3 }
}

/// The rank of a type in the table: `char`, `short`, `int` and `long long`.
const fn rank(ty: Ty) -> u8 {
    match ty {
        Ty::I8 | Ty::U8 => 0,
        Ty::I16 | Ty::U16 => 1,
        Ty::I32 | Ty::U32 => 2,
        Ty::I64 | Ty::U64 => 3,
    }
}

/// The type of a rank with a sign.
const fn of_rank(rank: u8, unsigned: bool) -> Ty {
    match (rank, unsigned) {
        (0, false) => Ty::I8,
        (0, true) => Ty::U8,
        (1, false) => Ty::I16,
        (1, true) => Ty::U16,
        (2, false) => Ty::I32,
        (2, true) => Ty::U32,
        (_, false) => Ty::I64,
        (_, true) => Ty::U64,
    }
}

/// The common type of `a` and `b` in kencc. `promote` is set for an arithmetic operator, and
/// then the type is at least an `int`.
#[must_use]
pub(crate) fn common(a: Ty, b: Ty, promote: bool) -> Ty {
    if a == b && (!promote || rank(a) >= 2) {
        return a;
    }
    let mut r = if rank(a) > rank(b) { rank(a) } else { rank(b) };
    if promote && r < 2 {
        r = 2;
    }
    of_rank(r, !a.signed() || !b.signed())
}

/// The case for the operand types `a` and `b`.
fn case(a: Ty, b: Ty) -> Case {
    let mut program =
        Program::new(format!("the conversions of kencc for {} with {}", a.c_name(), b.c_name()));
    let (va, vb) = (left(a), right(b));
    program.input(a, "a", va);
    program.input(b, "b", vb);
    let c = common(a, b, false);
    program.line(format!("{} x;", a.c_name()));
    program.line(format!("{} r;", c.c_name()));
    program.blank();

    let t = common(a, b, true);
    let (ta, tb) = (t.convert(va), t.convert(vb));
    program.check(t, "a + b", t.convert(ta + tb));
    program.check(t, "a - b", t.convert(ta - tb));
    program.check(t, "a * b", t.convert(ta * tb));
    program.check(t, "a / b", t.convert(ta / tb));
    program.check(t, "a % b", t.convert(ta % tb));
    let diff = t.convert(ta - tb);
    program.check(t, "(a - b) >> 1", diff >> 1);
    program.check(Ty::I32, "(a - b) < 0", i128::from(diff < 0));

    let (ca, cb) = (c.convert(va), c.convert(vb));
    program.check(Ty::I32, "a < b", i128::from(ca < cb));
    program.check(Ty::I32, "a > b", i128::from(ca > cb));
    program.check(Ty::I32, "a == b", i128::from(ca == cb));
    program.line("r = a ? a : b;");
    program.check(c, "r", ca);
    program.line("r = a ? b : a;");
    program.check(c, "r", cb);
    program.check(Ty::I32, "sizeof(a ? b : a)", i128::from(c.bits() / 8));

    let (xa, xb) = (a.convert(va), a.convert(vb));
    program.line("x = a;");
    program.line("x /= b;");
    program.check(a, "x", a.convert(xa / xb));
    program.line("x = a;");
    program.line("x %= b;");
    program.check(a, "x", a.convert(xa % xb));
    program.line("x = a;");
    program.line("x = x / b;");
    program.check(a, "x", a.convert(t.convert(ta / tb)));

    let (source, expected) = program.finish();
    Case::linked(
        Facet::KenccConversions,
        Axes::of([("left", a.name()), ("right", b.name()), ("dialect", "kencc")]),
        Dialect::C17,
        source,
        Vec::new(),
        vec!["-fkencc".to_owned()],
        Expect::Output(expected),
    )
    .tagged(&["gnu", RUCC_DIALECT])
}

pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &a in Ty::ALL {
        for &b in Ty::ALL {
            if !sink.wants(Facet::KenccConversions) {
                return;
            }
            sink.push_case(case(a, b));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::common;
    use crate::lang::Ty;

    #[test]
    fn the_table_keeps_the_sign_of_an_unsigned_operand() {
        assert_eq!(common(Ty::U8, Ty::I32, true), Ty::U32);
        assert_eq!(common(Ty::U16, Ty::I32, true), Ty::U32);
        assert_eq!(common(Ty::U8, Ty::U8, true), Ty::U32);
        assert_eq!(common(Ty::I8, Ty::U8, true), Ty::U32);
        assert_eq!(common(Ty::I8, Ty::I8, true), Ty::I32);
        assert_eq!(common(Ty::I64, Ty::U32, true), Ty::U64);
    }

    #[test]
    fn a_comparison_does_not_promote() {
        assert_eq!(common(Ty::I8, Ty::U8, false), Ty::U8);
        assert_eq!(common(Ty::I16, Ty::U8, false), Ty::U16);
        assert_eq!(common(Ty::U8, Ty::I32, false), Ty::U32);
        assert_eq!(common(Ty::I8, Ty::I8, false), Ty::I8);
    }
}
