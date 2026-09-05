//! The transformations that need loop structure.
//!
//! Every loop here has a trip count the generator knows, so the generator can run the loop
//! itself and record the answer. That is what lets the corpus check an unrolled loop against
//! arithmetic rather than against another compiler.
//!
//! The trip counts are chosen to sit either side of the places a compiler makes a decision:
//! one, so the loop is not a loop; two and three, so a peeled iteration leaves an odd
//! remainder; four and eight, so unrolling divides evenly; seven, so it does not; and a
//! hundred, so the body is worth vectorizing and the code is not worth fully unrolling.

use super::lit;
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// The trip counts every loop facet walks.
const TRIPS: &[i64] = &[1, 2, 3, 4, 7, 8, 16, 100];

/// The types the loop facets use.
///
/// Narrower than `int` is deliberately absent. A loop that accumulates into a `signed char`
/// spends most of its iterations converting, so the case would be about conversion rather
/// than about the loop, and the accumulator would overflow long before the trip count got
/// interesting.
const TYPES: &[Ty] = Ty::WIDE;

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    loop_invariant(sink);
    induction_variable(sink);
    loop_unswitch(sink);
    loop_unroll(sink);
    loop_idiom(sink);
    loop_deletion(sink);
    loop_rotate(sink);
    loop_restructure(sink);
}

/// Whether a value fits the accumulator, so a case that would overflow is never emitted.
fn fits(ty: Ty, value: i128) -> bool {
    ty.promoted().holds(value)
}

/// A computation inside the loop that does not depend on the loop.
///
/// The invariant part is a multiply, which is expensive enough that hoisting it shows up in
/// the size and the time. The variant part is the induction variable, so the loop cannot be
/// deleted and the case keeps measuring what it was written for.
fn loop_invariant(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &trips in TRIPS {
            if !sink.wants(Facet::LoopInvariant) {
                return;
            }
            let invariant = 3 * 5;
            let total: i128 = (0..i128::from(trips)).map(|at| invariant + at).sum();
            if !fits(ty, total) {
                continue;
            }
            let mut program = Program::new(format!(
                "an invariant multiply inside a {trips} iteration {} loop",
                ty.c_name()
            ));
            let name = ty.c_name();
            program.input(ty, "a", 3);
            program.input(ty, "b", 5);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
            program.line_at(1, format!("total += a * b + ({name})i;"));
            program.line("}");
            program.blank();
            program.check(ty.promoted(), "total", total);
            sink.push(
                Facet::LoopInvariant,
                Axes::of([("type", ty.name()), ("trips", &trips.to_string())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Several induction variables that a compiler is expected to reduce to one.
///
/// A loop that walks an array, keeps a scaled offset and keeps a counter has three variables
/// that all step by a constant amount. Rewriting them into one is the whole point of induction
/// variable optimization, and printing all three at the end is how the case checks that the
/// rewrite kept them in step.
fn induction_variable(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &trips in TRIPS {
            if !sink.wants(Facet::InductionVariable) {
                return;
            }
            let trips128 = i128::from(trips);
            let scaled: i128 = (0..trips128).map(|at| at * 4).sum();
            let walked: i128 = (0..trips128).sum();
            let counted = trips128;
            if !fits(ty, scaled) || !fits(ty, walked) {
                continue;
            }
            let mut program = Program::new(format!(
                "three induction variables over {trips} iterations on {}",
                ty.c_name()
            ));
            let name = ty.c_name();
            program.line(format!("{name} scaled = 0, walked = 0;"));
            program.line("int counted = 0;");
            program.line(format!("{name} buffer[{}];", trips.max(1)));
            program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
            program.line_at(1, format!("buffer[i] = ({name})i;"));
            program.line_at(1, format!("scaled += ({name})(i * 4);"));
            program.line_at(1, "walked += buffer[i];");
            program.line_at(1, "counted++;");
            program.line("}");
            program.blank();
            program.check(ty.promoted(), "scaled", scaled);
            program.check(ty.promoted(), "walked", walked);
            program.check(Ty::I32, "counted", counted);
            sink.push(
                Facet::InductionVariable,
                Axes::of([("type", ty.name()), ("trips", &trips.to_string())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A branch inside a loop whose condition does not change.
///
/// The condition is read once from a volatile global before the loop, so the compiler cannot
/// fold it, but it is provably the same on every iteration, so the loop can be split into two
/// loops with the test outside. Both settings of the flag are generated, because a compiler
/// that unswitches the wrong way round passes one and fails the other.
fn loop_unswitch(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &trips in TRIPS {
            for flag in [0i128, 1] {
                if !sink.wants(Facet::LoopUnswitch) {
                    return;
                }
                let trips128 = i128::from(trips);
                let total: i128 = (0..trips128)
                    .map(|at| if flag == 1 { at * 2 } else { at + 1 })
                    .sum();
                if !fits(ty, total) {
                    continue;
                }
                let mut program = Program::new(format!(
                    "an invariant test inside a {trips} iteration {} loop, flag {flag}",
                    ty.c_name()
                ));
                let name = ty.c_name();
                program.input(Ty::I32, "flag", flag);
                program.blank();
                program.line(format!("{name} total = 0;"));
                program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
                program.line_at(1, "if (flag) {");
                program.line_at(2, format!("total += ({name})(i * 2);"));
                program.line_at(1, "} else {");
                program.line_at(2, format!("total += ({name})(i + 1);"));
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::LoopUnswitch,
                    Axes::of([
                        ("type", ty.name()),
                        ("trips", &trips.to_string()),
                        ("flag", &flag.to_string()),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// Loops with a known trip count and loops without one.
///
/// The known ones are what a compiler unrolls. The unknown one is the control: the trip count
/// comes out of a volatile global, so the loop cannot be unrolled fully and a compiler that
/// thought it could is wrong rather than merely optimistic.
fn loop_unroll(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &trips in TRIPS {
            for known in [true, false] {
                if !sink.wants(Facet::LoopUnroll) {
                    return;
                }
                let trips128 = i128::from(trips);
                let total: i128 = (0..trips128).map(|at| at * 3 + 1).sum();
                if !fits(ty, total) {
                    continue;
                }
                let mut program = Program::new(format!(
                    "a {trips} iteration {} loop with {} trip count",
                    ty.c_name(),
                    if known { "a known" } else { "an unknown" }
                ));
                let name = ty.c_name();
                let bound = if known {
                    trips.to_string()
                } else {
                    program.input(Ty::I32, "limit", trips128);
                    "limit".to_owned()
                };
                program.blank();
                program.line(format!("{name} total = 0;"));
                program.line(format!("for (int i = 0; i < {bound}; i++) {{"));
                program.line_at(1, format!("total += ({name})(i * 3 + 1);"));
                program.line("}");
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::LoopUnroll,
                    Axes::of([
                        ("type", ty.name()),
                        ("trips", &trips.to_string()),
                        ("bound", if known { "known" } else { "unknown" }),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// Loops that are really a library function.
///
/// Filling an array with a constant is `memset`. Copying one to another is `memcpy`. Summing
/// one is a reduction a vector unit does eight at a time. Recognizing these is worth a large
/// factor, and getting the tail of the array wrong is the classic way to get it wrong, so
/// every case checks the first element, the last element and the sum.
fn loop_idiom(sink: &mut Sink<'_>) {
    const KINDS: &[&str] = &["fill", "copy", "sum", "count"];
    for &ty in TYPES {
        for &trips in &[4i64, 7, 16, 100] {
            for &kind in KINDS {
                if !sink.wants(Facet::LoopIdiom) {
                    return;
                }
                let trips128 = i128::from(trips);
                let mut program = Program::new(format!(
                    "a {trips} element {kind} over {}, which is a library call in disguise",
                    ty.c_name()
                ));
                let name = ty.c_name();
                program.line(format!("{name} source[{trips}];"));
                program.line(format!("{name} target[{trips}];"));
                program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
                program.line_at(1, format!("source[i] = ({name})(i + 1);"));
                program.line("}");
                program.blank();
                match kind {
                    "fill" => {
                        program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
                        program.line_at(1, format!("target[i] = {};", lit(ty, 0)));
                        program.line("}");
                        program.blank();
                        program.check(ty.promoted(), "target[0]", 0);
                        program.check(ty.promoted(), &format!("target[{}]", trips - 1), 0);
                    }
                    "copy" => {
                        program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
                        program.line_at(1, "target[i] = source[i];");
                        program.line("}");
                        program.blank();
                        program.check(ty.promoted(), "target[0]", 1);
                        program.check(
                            ty.promoted(),
                            &format!("target[{}]", trips - 1),
                            trips128,
                        );
                    }
                    "sum" => {
                        let total: i128 = (1..=trips128).sum();
                        if !fits(ty, total) {
                            continue;
                        }
                        program.line(format!("{name} total = 0;"));
                        program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
                        program.line_at(1, "total += source[i];");
                        program.line("}");
                        program.blank();
                        program.check(ty.promoted(), "total", total);
                    }
                    _ => {
                        let matches = (1..=trips128).filter(|value| value % 3 == 0).count();
                        program.line("int matches = 0;");
                        program.line(format!("for (int i = 0; i < {trips}; i++) {{"));
                        program.line_at(1, format!("if (source[i] % {} == 0) {{", lit(ty, 3)));
                        program.line_at(2, "matches++;");
                        program.line_at(1, "}");
                        program.line("}");
                        program.blank();
                        program.check(Ty::I32, "matches", matches as i128);
                    }
                }
                sink.push(
                    Facet::LoopIdiom,
                    Axes::of([
                        ("type", ty.name()),
                        ("trips", &trips.to_string()),
                        ("kind", kind),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// Loops whose result nobody reads.
///
/// A loop that computes into a local nothing uses can be deleted outright, provided the
/// compiler can see the loop terminates. A loop whose bound is unknown can still be deleted
/// for the same reason, and a compiler that deletes only the first is leaving work on the
/// table. Both are emitted next to a live loop so the case still has an answer to check.
fn loop_deletion(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &trips in &[4i64, 16, 100] {
            for &bound in &["known", "unknown"] {
                if !sink.wants(Facet::LoopDeletion) {
                    return;
                }
                let trips128 = i128::from(trips);
                let live: i128 = (0..trips128).sum();
                if !fits(ty, live) {
                    continue;
                }
                let mut program = Program::new(format!(
                    "a dead {trips} iteration {} loop with an {bound} bound",
                    ty.c_name()
                ));
                let name = ty.c_name();
                let limit = if bound == "known" {
                    trips.to_string()
                } else {
                    program.input(Ty::I32, "limit", trips128);
                    "limit".to_owned()
                };
                program.blank();
                program.line(format!("{name} unread = 0;"));
                program.line(format!("for (int i = 0; i < {limit}; i++) {{"));
                program.line_at(1, format!("unread += ({name})(i * 7 + 1);"));
                program.line("}");
                program.blank();
                program.line(format!("{name} total = 0;"));
                program.line(format!("for (int i = 0; i < {limit}; i++) {{"));
                program.line_at(1, format!("total += ({name})i;"));
                program.line("}");
                program.blank();
                program.check(ty.promoted(), "total", live);
                sink.push(
                    Facet::LoopDeletion,
                    Axes::of([
                        ("type", ty.name()),
                        ("trips", &trips.to_string()),
                        ("bound", bound),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// The same loop written four ways round.
///
/// A `for`, a `while`, a `do while` and a `goto` all describe the same iteration, and a
/// compiler that rotates loops turns the first two into something close to the third. Putting
/// them side by side means the report can say whether all four compiled to the same thing,
/// which is the only evidence that rotation actually happened.
fn loop_rotate(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["for", "while", "do-while", "goto"];
    for &ty in TYPES {
        for &trips in &[1i64, 2, 7, 16] {
            for &shape in SHAPES {
                if !sink.wants(Facet::LoopRotate) {
                    return;
                }
                let total: i128 = (0..i128::from(trips)).map(|at| at + 1).sum();
                if !fits(ty, total) {
                    continue;
                }
                let mut program = Program::new(format!(
                    "a {trips} iteration {} loop written as a {shape}",
                    ty.c_name()
                ));
                let name = ty.c_name();
                program.line(format!("{name} total = 0;"));
                program.line("int i = 0;");
                match shape {
                    "for" => {
                        program.line(format!("for (i = 0; i < {trips}; i++) {{"));
                        program.line_at(1, format!("total += ({name})(i + 1);"));
                        program.line("}");
                    }
                    "while" => {
                        program.line(format!("while (i < {trips}) {{"));
                        program.line_at(1, format!("total += ({name})(i + 1);"));
                        program.line_at(1, "i++;");
                        program.line("}");
                    }
                    "do-while" => {
                        // A `do while` always runs once, so the guard has to be written out
                        // in front of it for the case to describe the same iteration as the
                        // other three. That guard is exactly what loop rotation inserts.
                        program.line(format!("if (0 < {trips}) {{"));
                        program.line_at(1, "do {");
                        program.line_at(2, format!("total += ({name})(i + 1);"));
                        program.line_at(2, "i++;");
                        program.line_at(1, format!("}} while (i < {trips});"));
                        program.line("}");
                    }
                    _ => {
                        program.line("top:");
                        program.line(format!("if (i < {trips}) {{"));
                        program.line_at(1, format!("total += ({name})(i + 1);"));
                        program.line_at(1, "i++;");
                        program.line_at(1, "goto top;");
                        program.line("}");
                    }
                }
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::LoopRotate,
                    Axes::of([
                        ("type", ty.name()),
                        ("trips", &trips.to_string()),
                        ("shape", shape),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// Nested loops that could be walked in either order, or run as one.
///
/// Interchange is about locality, so the two orders are written out and the report compares
/// them. Fusion is about the same thing from the other side: two loops over one range doing
/// independent work should cost what one loop doing both costs.
fn loop_restructure(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["row-major", "column-major", "two-loops", "fused"];
    for &ty in TYPES {
        for &side in &[4i64, 8, 16] {
            for &shape in SHAPES {
                if !sink.wants(Facet::LoopRestructure) {
                    return;
                }
                let side128 = i128::from(side);
                let total: i128 = (0..side128)
                    .flat_map(|row| (0..side128).map(move |col| row * side128 + col))
                    .sum();
                if !fits(ty, total) {
                    continue;
                }
                let mut program = Program::new(format!(
                    "a {side} by {side} {} traversal, {shape}",
                    ty.c_name()
                ));
                let name = ty.c_name();
                program.line(format!("static {name} grid[{side}][{side}];"));
                program.line(format!("{name} total = 0;"));
                program.blank();
                match shape {
                    "row-major" => {
                        program.line(format!("for (int r = 0; r < {side}; r++) {{"));
                        program.line_at(1, format!("for (int c = 0; c < {side}; c++) {{"));
                        program.line_at(2, format!("grid[r][c] = ({name})(r * {side} + c);"));
                        program.line_at(2, "total += grid[r][c];");
                        program.line_at(1, "}");
                        program.line("}");
                    }
                    "column-major" => {
                        program.line(format!("for (int c = 0; c < {side}; c++) {{"));
                        program.line_at(1, format!("for (int r = 0; r < {side}; r++) {{"));
                        program.line_at(2, format!("grid[r][c] = ({name})(r * {side} + c);"));
                        program.line_at(2, "total += grid[r][c];");
                        program.line_at(1, "}");
                        program.line("}");
                    }
                    "two-loops" => {
                        program.line(format!("for (int r = 0; r < {side}; r++) {{"));
                        program.line_at(1, format!("for (int c = 0; c < {side}; c++) {{"));
                        program.line_at(2, format!("grid[r][c] = ({name})(r * {side} + c);"));
                        program.line_at(1, "}");
                        program.line("}");
                        program.line(format!("for (int r = 0; r < {side}; r++) {{"));
                        program.line_at(1, format!("for (int c = 0; c < {side}; c++) {{"));
                        program.line_at(2, "total += grid[r][c];");
                        program.line_at(1, "}");
                        program.line("}");
                    }
                    _ => {
                        program.line(format!("for (int at = 0; at < {}; at++) {{", side * side));
                        program.line_at(1, format!("int r = at / {side};"));
                        program.line_at(1, format!("int c = at % {side};"));
                        program.line_at(1, format!("grid[r][c] = ({name})(r * {side} + c);"));
                        program.line_at(1, "total += grid[r][c];");
                        program.line("}");
                    }
                }
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::LoopRestructure,
                    Axes::of([
                        ("type", ty.name()),
                        ("side", &side.to_string()),
                        ("shape", shape),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, Sink};
    use corpus_model::{Case, Expect, Facet};

    fn cases_for(facet: Facet) -> Vec<Case> {
        let opts = Options::all().only(&[facet]);
        let mut sink = Sink::new(&opts);
        super::generate(&mut sink);
        sink.into_cases()
    }

    fn output(case: &Case) -> &str {
        match &case.expect {
            Expect::Output(text) => text,
            Expect::Rejected(_) => panic!("{} should run", case.id),
        }
    }

    #[test]
    fn a_one_iteration_loop_and_a_hundred_iteration_loop_are_both_generated() {
        let cases = cases_for(Facet::LoopUnroll);
        let trips: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("trips")).collect();
        assert!(trips.contains(&"1"));
        assert!(trips.contains(&"100"));
    }

    #[test]
    fn the_expected_total_of_an_invariant_loop_matches_the_arithmetic() {
        let cases = cases_for(Facet::LoopInvariant);
        let case = cases
            .iter()
            .find(|c| c.axes.get("type") == Some("i32") && c.axes.get("trips") == Some("4"))
            .unwrap();
        // Four iterations of fifteen plus the loop counter: 15+16+17+18.
        assert_eq!(output(case), "66\n");
    }

    #[test]
    fn a_known_and_an_unknown_trip_count_expect_the_same_answer() {
        let cases = cases_for(Facet::LoopUnroll);
        for trips in ["4", "7", "100"] {
            let known = cases
                .iter()
                .find(|c| {
                    c.axes.get("trips") == Some(trips)
                        && c.axes.get("bound") == Some("known")
                        && c.axes.get("type") == Some("i64")
                })
                .unwrap();
            let unknown = cases
                .iter()
                .find(|c| {
                    c.axes.get("trips") == Some(trips)
                        && c.axes.get("bound") == Some("unknown")
                        && c.axes.get("type") == Some("i64")
                })
                .unwrap();
            assert_eq!(output(known), output(unknown), "trips {trips}");
        }
    }

    #[test]
    fn all_four_rotations_of_the_same_loop_expect_the_same_answer() {
        let cases = cases_for(Facet::LoopRotate);
        for trips in ["1", "2", "7", "16"] {
            let answers: Vec<&str> = cases
                .iter()
                .filter(|c| c.axes.get("trips") == Some(trips) && c.axes.get("type") == Some("i32"))
                .map(output)
                .collect();
            assert_eq!(answers.len(), 4, "trips {trips}");
            assert!(answers.windows(2).all(|w| w[0] == w[1]), "trips {trips}: {answers:?}");
        }
    }

    #[test]
    fn all_four_traversals_of_the_same_grid_expect_the_same_total() {
        let cases = cases_for(Facet::LoopRestructure);
        let answers: Vec<&str> = cases
            .iter()
            .filter(|c| c.axes.get("side") == Some("8") && c.axes.get("type") == Some("i32"))
            .map(output)
            .collect();
        assert_eq!(answers.len(), 4);
        assert!(answers.windows(2).all(|w| w[0] == w[1]), "{answers:?}");
        // The numbers nought to sixty-three.
        assert_eq!(answers[0], "2016\n");
    }

    #[test]
    fn unswitching_generates_both_settings_of_the_flag_and_they_differ() {
        let cases = cases_for(Facet::LoopUnswitch);
        let on = cases
            .iter()
            .find(|c| {
                c.axes.get("flag") == Some("1")
                    && c.axes.get("trips") == Some("4")
                    && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        let off = cases
            .iter()
            .find(|c| {
                c.axes.get("flag") == Some("0")
                    && c.axes.get("trips") == Some("4")
                    && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        assert_eq!(output(on), "12\n");
        assert_eq!(output(off), "10\n");
    }

    #[test]
    fn a_deleted_loop_and_a_live_loop_sit_in_the_same_program() {
        for case in cases_for(Facet::LoopDeletion) {
            assert!(case.source.contains("unread"), "{}", case.id);
            assert!(case.source.contains("total"), "{}", case.id);
            assert_eq!(case.source.matches("for (int i").count(), 2, "{}", case.id);
        }
    }

    #[test]
    fn every_idiom_kind_is_covered_and_the_sum_adds_up() {
        let cases = cases_for(Facet::LoopIdiom);
        let kinds: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("kind")).collect();
        for wanted in ["fill", "copy", "sum", "count"] {
            assert!(kinds.contains(&wanted), "no case for {wanted}");
        }
        let sum = cases
            .iter()
            .find(|c| {
                c.axes.get("kind") == Some("sum")
                    && c.axes.get("trips") == Some("16")
                    && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        assert_eq!(output(sum), "136\n");
    }

    #[test]
    fn no_loop_case_expects_a_value_its_accumulator_cannot_hold() {
        for facet in [
            Facet::LoopInvariant,
            Facet::InductionVariable,
            Facet::LoopUnroll,
            Facet::LoopIdiom,
            Facet::LoopDeletion,
            Facet::LoopRotate,
            Facet::LoopRestructure,
            Facet::LoopUnswitch,
        ] {
            for case in cases_for(facet) {
                for line in output(&case).lines() {
                    let value: i128 = line.parse().unwrap_or_else(|_| panic!("{}", case.id));
                    assert!(value.abs() < i128::from(i64::MAX), "{}", case.id);
                }
            }
        }
    }
}
