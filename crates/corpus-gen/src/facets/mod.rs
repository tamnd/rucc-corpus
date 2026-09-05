//! One module per phase of the plan, each emitting the cases for its own facets.
//!
//! A facet generator is a loop over an axis subspace that builds a program per point. It does
//! not decide whether the case is wanted, how many to emit or where they go, because the
//! [`crate::Sink`] does all three. That keeps a generator short enough to read in one sitting,
//! which matters because a bug in a generator is a case that quietly tests nothing.

pub(crate) mod backend;
pub(crate) mod global;
pub(crate) mod interproc;
pub(crate) mod local;
pub(crate) mod loops;
pub(crate) mod special;

use crate::lang::Ty;

/// A literal of a type, cast so that the expression really has that type.
///
/// Without the cast, `255u` is an `unsigned int` no matter what type the case is about, and a
/// case that meant to test `unsigned char` arithmetic ends up testing `unsigned int`
/// arithmetic with a small operand. Casting first and letting the usual promotions happen
/// afterwards is what a real program does, and it is what the evaluator in `lang` models.
pub(crate) fn lit(ty: Ty, value: i128) -> String {
    format!("({}){}", ty.c_name(), ty.literal(value))
}

/// The value with every bit set, in a form the type can hold.
pub(crate) fn all_ones(ty: Ty) -> i128 {
    if ty.signed() { -1 } else { ty.max() }
}

/// At most `n` values from a sorted list, keeping both ends and spacing the rest evenly.
///
/// Some axis points have more interesting operands than a single program should carry. This
/// thins them without ever dropping the boundary values, which are the ones that find bugs.
/// The choice is a function of the input alone, so it is the same on every run.
pub(crate) fn spread(values: &[i128], n: usize) -> Vec<i128> {
    if values.len() <= n || n == 0 {
        return values.to_vec();
    }
    if n == 1 {
        return vec![values[0]];
    }
    let last = values.len() - 1;
    let mut out: Vec<i128> = (0..n).map(|at| values[at * last / (n - 1)]).collect();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::{all_ones, lit, spread};
    use crate::lang::Ty;

    #[test]
    fn a_literal_is_cast_to_the_type_the_case_is_about() {
        assert_eq!(lit(Ty::U8, 255), "(unsigned char)255u");
        assert_eq!(lit(Ty::I64, -1), "(long long)(-1ll)");
    }

    #[test]
    fn all_ones_is_minus_one_when_signed_and_the_maximum_when_not() {
        assert_eq!(all_ones(Ty::I32), -1);
        assert_eq!(all_ones(Ty::U8), 255);
        assert_eq!(all_ones(Ty::U64), Ty::U64.max());
    }

    #[test]
    fn spreading_keeps_both_ends_and_never_grows_the_list() {
        let values: Vec<i128> = (0..20).collect();
        let picked = spread(&values, 5);
        assert_eq!(picked.first(), Some(&0));
        assert_eq!(picked.last(), Some(&19));
        assert!(picked.len() <= 5);
    }

    #[test]
    fn spreading_a_short_list_leaves_it_alone() {
        let values = vec![1, 2, 3];
        assert_eq!(spread(&values, 5), values);
        assert_eq!(spread(&values, 3), values);
    }

    #[test]
    fn spreading_is_the_same_every_time_it_is_asked() {
        let values: Vec<i128> = (0..37).collect();
        assert_eq!(spread(&values, 6), spread(&values, 6));
    }
}
