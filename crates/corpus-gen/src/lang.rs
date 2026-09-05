//! The C the generator knows how to write, and what it evaluates to.
//!
//! The generator computes the answer to every program it emits. That is the whole reason the
//! corpus can be trusted: the expected output does not come from a compiler, so a bug shared
//! by every compiler under test still shows up. The cost is that the generator has to model
//! the part of C it uses, which is what this module is.
//!
//! The model is deliberately small. Integers are evaluated in `i128` and then narrowed to the
//! width and signedness of the result type, following the usual arithmetic conversions for
//! the cases the generator actually emits. Anything that would be undefined returns `None`
//! and the case is dropped rather than emitted with a guessed answer.

/// A scalar type the generator can write and evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    /// `signed char`.
    I8,
    /// `short`.
    I16,
    /// `int`.
    I32,
    /// `long long`.
    I64,
    /// `unsigned char`.
    U8,
    /// `unsigned short`.
    U16,
    /// `unsigned int`.
    U32,
    /// `unsigned long long`.
    U64,
}

impl Ty {
    /// Every type, narrowest first, signed before unsigned at each width.
    pub const ALL: &'static [Self] =
        &[Self::I8, Self::U8, Self::I16, Self::U16, Self::I32, Self::U32, Self::I64, Self::U64];

    /// The four types at least as wide as `int`, where no promotion changes the result type.
    pub const WIDE: &'static [Self] = &[Self::I32, Self::U32, Self::I64, Self::U64];

    /// The name used on an axis and in a case id.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
        }
    }

    /// How the type is spelled in C.
    ///
    /// Written out rather than using the fixed width names from `stdint.h`, because a corpus
    /// that only ever says `int32_t` never exercises the path where the compiler has to work
    /// out that `int` and `long` are different types that happen to be the same width.
    #[must_use]
    pub const fn c_name(self) -> &'static str {
        match self {
            Self::I8 => "signed char",
            Self::I16 => "short",
            Self::I32 => "int",
            Self::I64 => "long long",
            Self::U8 => "unsigned char",
            Self::U16 => "unsigned short",
            Self::U32 => "unsigned int",
            Self::U64 => "unsigned long long",
        }
    }

    /// How wide it is.
    #[must_use]
    pub const fn bits(self) -> u32 {
        match self {
            Self::I8 | Self::U8 => 8,
            Self::I16 | Self::U16 => 16,
            Self::I32 | Self::U32 => 32,
            Self::I64 | Self::U64 => 64,
        }
    }

    /// Whether it is signed.
    #[must_use]
    pub const fn signed(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I32 | Self::I64)
    }

    /// The type an operand of this type is promoted to before arithmetic.
    ///
    /// Everything narrower than `int` promotes to `int`, because `int` can hold every value
    /// of every narrower type on the targets this corpus runs on. That is the one place the
    /// model assumes something about the target, and it is an assumption both compilers under
    /// test make too.
    #[must_use]
    pub const fn promoted(self) -> Self {
        match self {
            Self::I8 | Self::I16 | Self::U8 | Self::U16 => Self::I32,
            other => other,
        }
    }

    /// The lowest value of the type.
    #[must_use]
    pub const fn min(self) -> i128 {
        if self.signed() { -(1i128 << (self.bits() - 1)) } else { 0 }
    }

    /// The highest value of the type.
    #[must_use]
    pub const fn max(self) -> i128 {
        if self.signed() { (1i128 << (self.bits() - 1)) - 1 } else { (1i128 << self.bits()) - 1 }
    }

    /// Whether a value fits without conversion.
    #[must_use]
    pub const fn holds(self, value: i128) -> bool {
        value >= self.min() && value <= self.max()
    }

    /// A value converted to this type, the way an assignment in C would convert it.
    ///
    /// For unsigned this is the wraparound the standard defines. For signed it is the two's
    /// complement result, which is implementation defined in C and is what both compilers
    /// under test do. The generator never relies on it for a value it computes, because it
    /// only emits arithmetic that stays in range, but a narrowing store is itself a thing
    /// worth testing and this is what those cases expect.
    #[must_use]
    pub const fn convert(self, value: i128) -> i128 {
        let bits = self.bits();
        let mask = if bits == 128 { -1i128 } else { (1i128 << bits) - 1 };
        let raw = value & mask;
        if self.signed() && raw > self.max() { raw - (mask + 1) } else { raw }
    }

    /// The `printf` conversion to use, with the value cast to match it.
    ///
    /// Everything is printed through `long long` or `unsigned long long` so that one format
    /// string works for every width. That keeps the output of a case independent of how wide
    /// the type under test is, which means two cases that compute the same number print the
    /// same text and a diff between them is a real difference.
    #[must_use]
    pub const fn print_spec(self) -> &'static str {
        if self.signed() { "%lld" } else { "%llu" }
    }

    /// The cast that goes with [`Self::print_spec`].
    #[must_use]
    pub const fn print_cast(self) -> &'static str {
        if self.signed() { "(long long)" } else { "(unsigned long long)" }
    }

    /// How a literal of this type is written.
    ///
    /// The suffix matters. Without it a large unsigned constant is a different type from the
    /// variable it is assigned to, and the case ends up testing the conversion rather than
    /// the thing it was written for.
    #[must_use]
    pub fn literal(self, value: i128) -> String {
        let suffix = match self {
            Self::I64 => "ll",
            Self::U64 => "ull",
            Self::U8 | Self::U16 | Self::U32 => "u",
            _ => "",
        };
        // The most negative value of a signed type has no positive counterpart, so writing it
        // directly makes a positive constant that does not fit followed by a negation. Both
        // compilers warn about it. Writing it as one less than the negated maximum does not.
        if self.signed() && value == self.min() {
            format!("(-{}{suffix} - 1)", self.max())
        } else if value < 0 {
            format!("(-{}{suffix})", -value)
        } else {
            format!("{value}{suffix}")
        }
    }
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// A binary or unary operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    /// `a + b`.
    Add,
    /// `a - b`.
    Sub,
    /// `a * b`.
    Mul,
    /// `a / b`.
    Div,
    /// `a % b`.
    Rem,
    /// `a & b`.
    And,
    /// `a | b`.
    Or,
    /// `a ^ b`.
    Xor,
    /// `a << b`.
    Shl,
    /// `a >> b`.
    Shr,
    /// `-a`.
    Neg,
    /// `~a`.
    Not,
    /// `a == b`, which yields `int`.
    Eq,
    /// `a < b`, which yields `int`.
    Lt,
}

impl Op {
    /// Every operation.
    pub const ALL: &'static [Self] = &[
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Rem,
        Self::And,
        Self::Or,
        Self::Xor,
        Self::Shl,
        Self::Shr,
        Self::Neg,
        Self::Not,
        Self::Eq,
        Self::Lt,
    ];

    /// The arithmetic ones, which is what most facets want.
    pub const ARITH: &'static [Self] = &[Self::Add, Self::Sub, Self::Mul, Self::Div, Self::Rem];

    /// The bitwise ones.
    pub const BITWISE: &'static [Self] = &[Self::And, Self::Or, Self::Xor, Self::Shl, Self::Shr];

    /// The name used on an axis and in a case id.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Rem => "rem",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Shl => "shl",
            Self::Shr => "shr",
            Self::Neg => "neg",
            Self::Not => "not",
            Self::Eq => "eq",
            Self::Lt => "lt",
        }
    }

    /// The C spelling.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Rem => "%",
            Self::And => "&",
            Self::Or => "|",
            Self::Xor => "^",
            Self::Shl => "<<",
            Self::Shr => ">>",
            Self::Neg => "-",
            Self::Not => "~",
            Self::Eq => "==",
            Self::Lt => "<",
        }
    }

    /// Whether it takes one operand.
    #[must_use]
    pub const fn is_unary(self) -> bool {
        matches!(self, Self::Neg | Self::Not)
    }

    /// Whether the right operand is a shift count rather than a value of the same type.
    #[must_use]
    pub const fn is_shift(self) -> bool {
        matches!(self, Self::Shl | Self::Shr)
    }

    /// Whether the result is an `int` regardless of the operand type.
    #[must_use]
    pub const fn is_comparison(self) -> bool {
        matches!(self, Self::Eq | Self::Lt)
    }

    /// Whether reordering the operands leaves the value alone.
    #[must_use]
    pub const fn is_commutative(self) -> bool {
        matches!(self, Self::Add | Self::Mul | Self::And | Self::Or | Self::Xor | Self::Eq)
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The type an expression of this operation on this operand type has.
#[must_use]
pub fn result_ty(op: Op, ty: Ty) -> Ty {
    if op.is_comparison() {
        Ty::I32
    } else if op.is_shift() {
        // A shift is not a usual arithmetic conversion. The result has the type of the
        // promoted left operand and the right operand has nothing to do with it, which is
        // the one rule people most often get wrong when they write this by hand.
        ty.promoted()
    } else {
        ty.promoted()
    }
}

/// Evaluates an operation, or refuses.
///
/// Returns `None` when the operation would be undefined or implementation defined in C, which
/// is the generator's guard against emitting a program whose answer it does not actually
/// know. Overflow of a signed type, division by zero, the one division that overflows, and a
/// shift count that is negative or at least the width all come back as `None`.
#[must_use]
pub fn eval(op: Op, ty: Ty, left: i128, right: i128) -> Option<i128> {
    if !ty.holds(left) || (!op.is_unary() && !op.is_shift() && !ty.holds(right)) {
        return None;
    }
    let out = result_ty(op, ty);
    let value = match op {
        Op::Add => left.checked_add(right)?,
        Op::Sub => left.checked_sub(right)?,
        Op::Mul => left.checked_mul(right)?,
        Op::Div => {
            if right == 0 {
                return None;
            }
            left / right
        }
        Op::Rem => {
            if right == 0 {
                return None;
            }
            left % right
        }
        Op::And => left & right,
        Op::Or => left | right,
        Op::Xor => left ^ right,
        Op::Shl => {
            let width = i128::from(out.bits());
            if !(0..width).contains(&right) {
                return None;
            }
            // Shifting a negative value left, or shifting a one out of the sign bit of a
            // signed type, is undefined. The generator refuses rather than guessing.
            if out.signed() && (left < 0 || left.checked_shl(right.try_into().ok()?)? > out.max()) {
                return None;
            }
            left << right
        }
        Op::Shr => {
            let width = i128::from(out.bits());
            if !(0..width).contains(&right) || left < 0 {
                return None;
            }
            left >> right
        }
        Op::Neg => {
            // Negating an unsigned value wraps, and that is defined, so it falls through to
            // the conversion below. Negating the most negative value of a signed type is not
            // defined, so it is refused here.
            if out.signed() && !out.holds(-left) {
                return None;
            }
            -left
        }
        Op::Not => !left,
        Op::Eq => i128::from(left == right),
        Op::Lt => i128::from(left < right),
    };
    // Unsigned arithmetic wraps and that is defined, so the result is converted. Signed
    // arithmetic that would need converting has already been refused above, so converting it
    // here is a no-op that keeps the two paths one line of code.
    if !out.signed() {
        return Some(out.convert(value));
    }
    if out.holds(value) { Some(value) } else { None }
}

/// A handful of operand values worth trying for a type.
///
/// Not random. These are the values that break things: the identities, the boundaries, a
/// power of two, something that is not a power of two, and the extremes of the type. A
/// generator that picked numbers at random would need thousands of programs to stumble onto
/// the interesting ones and would still miss the boundary by one.
#[must_use]
pub fn interesting(ty: Ty) -> Vec<i128> {
    let mut values = vec![0, 1, 2, 3, 7, 8, 16, 255];
    if ty.signed() {
        values.extend([-1, -2, -8, -255]);
    }
    values.push(ty.max());
    values.push(ty.min());
    if ty.bits() >= 16 {
        values.push(ty.max() / 2);
        values.push(1 << (ty.bits() / 2));
    }
    values.retain(|value| ty.holds(*value));
    values.sort_unstable();
    values.dedup();
    values
}

/// The shift counts worth trying for a type.
#[must_use]
pub fn shift_counts(ty: Ty) -> Vec<i128> {
    let width = i128::from(ty.promoted().bits());
    let mut counts = vec![0, 1, 3, width / 2, width - 1];
    counts.retain(|count| (0..width).contains(count));
    counts.sort_unstable();
    counts.dedup();
    counts
}

#[cfg(test)]
mod tests {
    use super::{Op, Ty, eval, interesting, result_ty, shift_counts};

    #[test]
    fn a_narrow_type_promotes_to_int_and_a_wide_one_stays_itself() {
        assert_eq!(Ty::U8.promoted(), Ty::I32);
        assert_eq!(Ty::I16.promoted(), Ty::I32);
        assert_eq!(Ty::U32.promoted(), Ty::U32);
        assert_eq!(Ty::I64.promoted(), Ty::I64);
    }

    #[test]
    fn the_bounds_of_each_type_are_the_ones_c_gives_it() {
        assert_eq!((Ty::I8.min(), Ty::I8.max()), (-128, 127));
        assert_eq!((Ty::U8.min(), Ty::U8.max()), (0, 255));
        assert_eq!((Ty::I32.min(), Ty::I32.max()), (-2_147_483_648, 2_147_483_647));
        assert_eq!(Ty::U64.max(), 18_446_744_073_709_551_615);
        assert_eq!(Ty::I64.min(), -9_223_372_036_854_775_808);
    }

    #[test]
    fn converting_wraps_unsigned_and_sign_extends_signed() {
        assert_eq!(Ty::U8.convert(256), 0);
        assert_eq!(Ty::U8.convert(-1), 255);
        assert_eq!(Ty::I8.convert(255), -1);
        assert_eq!(Ty::I8.convert(128), -128);
        assert_eq!(Ty::U32.convert(-1), 4_294_967_295);
    }

    #[test]
    fn signed_overflow_is_refused_and_unsigned_wraparound_is_not() {
        assert_eq!(eval(Op::Add, Ty::I32, Ty::I32.max(), 1), None);
        assert_eq!(eval(Op::Mul, Ty::I64, Ty::I64.max(), 2), None);
        assert_eq!(eval(Op::Add, Ty::U32, Ty::U32.max(), 1), Some(0));
        assert_eq!(eval(Op::Sub, Ty::U32, 0, 1), Some(4_294_967_295));
    }

    #[test]
    fn dividing_by_zero_and_the_one_division_that_overflows_are_both_refused() {
        assert_eq!(eval(Op::Div, Ty::I32, 1, 0), None);
        assert_eq!(eval(Op::Rem, Ty::U64, 1, 0), None);
        assert_eq!(eval(Op::Div, Ty::I32, Ty::I32.min(), -1), None);
        assert_eq!(eval(Op::Div, Ty::I32, 7, 2), Some(3));
        assert_eq!(eval(Op::Div, Ty::I32, -7, 2), Some(-3));
        assert_eq!(eval(Op::Rem, Ty::I32, -7, 2), Some(-1));
    }

    #[test]
    fn a_shift_count_outside_the_width_of_the_promoted_type_is_refused() {
        assert_eq!(eval(Op::Shl, Ty::I32, 1, 32), None);
        assert_eq!(eval(Op::Shl, Ty::I32, 1, -1), None);
        assert_eq!(eval(Op::Shr, Ty::U64, 1, 64), None);
        // The count is measured against the promoted type, so eight is a fine count for an
        // unsigned char even though the type itself is only eight bits wide.
        assert_eq!(eval(Op::Shl, Ty::U8, 1, 8), Some(256));
        assert_eq!(eval(Op::Shl, Ty::U8, 1, 32), None);
    }

    #[test]
    fn shifting_a_negative_value_or_out_of_the_sign_bit_is_refused() {
        assert_eq!(eval(Op::Shl, Ty::I32, -1, 1), None);
        assert_eq!(eval(Op::Shr, Ty::I32, -8, 1), None);
        assert_eq!(eval(Op::Shl, Ty::I32, 1, 31), None);
        assert_eq!(eval(Op::Shl, Ty::I32, 1, 30), Some(1_073_741_824));
    }

    #[test]
    fn a_shift_takes_the_type_of_its_left_operand_and_a_comparison_yields_int() {
        assert_eq!(result_ty(Op::Shl, Ty::U8), Ty::I32);
        assert_eq!(result_ty(Op::Shl, Ty::U64), Ty::U64);
        assert_eq!(result_ty(Op::Lt, Ty::U64), Ty::I32);
        assert_eq!(eval(Op::Lt, Ty::U64, 1, 2), Some(1));
        assert_eq!(eval(Op::Eq, Ty::I8, -1, -1), Some(1));
    }

    #[test]
    fn negating_the_most_negative_value_is_refused() {
        assert_eq!(eval(Op::Neg, Ty::I64, Ty::I64.min(), 0), None);
        assert_eq!(eval(Op::Neg, Ty::I32, 5, 0), Some(-5));
        assert_eq!(eval(Op::Neg, Ty::U32, 5, 0), Some(4_294_967_291));
    }

    #[test]
    fn the_most_negative_literal_is_written_so_that_it_never_overflows_on_the_way_in() {
        assert_eq!(Ty::I32.literal(Ty::I32.min()), "(-2147483647 - 1)");
        assert_eq!(Ty::I64.literal(Ty::I64.min()), "(-9223372036854775807ll - 1)");
        assert_eq!(Ty::U64.literal(1), "1ull");
        assert_eq!(Ty::I32.literal(-5), "(-5)");
    }

    #[test]
    fn every_interesting_value_fits_the_type_it_was_chosen_for() {
        for ty in Ty::ALL {
            let values = interesting(*ty);
            assert!(values.len() >= 6, "{ty} has too few values");
            for value in values {
                assert!(ty.holds(value), "{ty} does not hold {value}");
            }
        }
    }

    #[test]
    fn every_shift_count_is_inside_the_width_of_the_promoted_type() {
        for ty in Ty::ALL {
            for count in shift_counts(*ty) {
                assert!(count >= 0 && count < i128::from(ty.promoted().bits()), "{ty}");
            }
        }
    }

    #[test]
    fn no_two_types_or_operations_share_a_name() {
        let mut names: Vec<&str> = Ty::ALL.iter().map(|t| t.name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), Ty::ALL.len());
        let mut ops: Vec<&str> = Op::ALL.iter().map(|o| o.name()).collect();
        ops.sort_unstable();
        ops.dedup();
        assert_eq!(ops.len(), Op::ALL.len());
    }
}
