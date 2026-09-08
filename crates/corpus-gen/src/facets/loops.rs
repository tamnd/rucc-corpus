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
    loop_hoist(sink);
    induction_variable(sink);
    trip_counts(sink);
    loop_unswitch(sink);
    loop_unroll(sink);
    loop_idiom(sink);
    loop_deletion(sink);
    loop_rotate(sink);
    loop_shape(sink);
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

/// The shapes that decide whether an invariant computation is allowed out of its loop.
///
/// `loop-invariant` above asks whether the hoist happens. These ask whether it is allowed to,
/// which is a different question and the one that has the miscompilations in it. Half of these
/// cases fault rather than print the wrong answer if a compiler gets them wrong: a division
/// hoisted out of a loop that never runs divides by zero, and a load hoisted out of a loop that
/// never runs reads through a null pointer. That is deliberate. A case that can only print the
/// wrong answer is a case that a compiler can pass by accident.
///
/// The other half are the ones where the output cannot tell you anything and the remark can. A
/// volatile read has to happen every iteration and no amount of printing will say whether it
/// did, so what the case is for is the `-fopt-info` line saying the compiler declined it. Those
/// are marked as such in the purpose line of the generated file.
fn loop_hoist(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "global-load",
        "global-load-written",
        "load-that-may-not-run",
        "load-that-always-runs",
        "divide-at-the-bottom",
        "divide-that-may-not-run",
        "divide-under-a-test",
        "volatile-load",
        "a-chain-of-three",
        "out-of-a-nest",
        "under-pressure",
        "an-array-element",
        "a-call-with-nothing-in-it",
        "a-store-of-something-invariant",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::LoopHoist) {
                return;
            }
            let Some(program) = hoisted(ty, shape) else { continue };
            sink.push(
                Facet::LoopHoist,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// One shape of the hoisting question, or `None` where the accumulator cannot hold the answer.
///
/// Every operand a compiler is not allowed to know comes out of a `volatile` global, including
/// the ones whose value is zero. That matters more here than anywhere else in this file: a
/// divisor that is written as a literal zero is a program with undefined behaviour in it that
/// the compiler is entitled to do anything with, and a divisor that is read at run time and
/// happens to be zero on a path nothing takes is an ordinary program that has to work.
#[expect(clippy::too_many_lines, reason = "fourteen shapes, each one a short block")]
fn hoisted(ty: Ty, shape: &str) -> Option<Program> {
    let name = ty.c_name();
    let mut program = Program::new(match shape {
        "global-load" => format!("a {name} loop reading a global nothing in it writes"),
        "global-load-written" => format!("a {name} loop reading a global the body writes"),
        "load-that-may-not-run" => {
            format!("a {name} loop that never runs, over a pointer that is null")
        }
        "load-that-always-runs" => format!("a {name} loop that always runs, over a valid pointer"),
        "divide-at-the-bottom" => format!("an invariant divide in a {name} loop that always runs"),
        "divide-that-may-not-run" => {
            format!("an invariant divide by zero in a {name} loop that never runs")
        }
        "divide-under-a-test" => format!("an invariant divide by zero under a false test, {name}"),
        "volatile-load" => format!("a volatile read a {name} loop must do every iteration"),
        "a-chain-of-three" => format!("three invariant computations feeding each other, {name}"),
        "out-of-a-nest" => format!("an invariant in the inner loop of a {name} nest"),
        "under-pressure" => format!("a cheap invariant in a {name} loop with no registers spare"),
        "an-array-element" => format!("an invariant index into a global array, {name}"),
        "a-call-with-nothing-in-it" => format!("an invariant call to a pure function, {name}"),
        _ => format!("an invariant value a {name} loop stores every iteration"),
    });

    let total: i128 = match shape {
        "global-load" => {
            // A global rather than a local, because a local the loop does not write is
            // invariant by looking at the definition and a global is only invariant once
            // something has decided the loop cannot have written it.
            program.top("static int seed;");
            program.input(Ty::I32, "start", 3);
            program.blank();
            program.line("seed = start;");
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("total += ({name})(seed + i);"));
            program.line("}");
            (0..8i128).map(|at| 3 + at).sum()
        }
        "global-load-written" => {
            // The same read, and the answer is no. The call writes what the read reads, so
            // every iteration sees a different value, and a compiler that hoists the read
            // prints eight times the first one.
            program.top("static int seed;");
            program.top("");
            program.top("static void bump(void) {");
            program.top("    seed++;");
            program.top("}");
            program.input(Ty::I32, "start", 3);
            program.blank();
            program.line("seed = start;");
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("total += ({name})seed;"));
            program.line_at(1, "bump();");
            program.line("}");
            (0..8i128).map(|at| 3 + at).sum()
        }
        "load-that-may-not-run" => {
            // The bound and the pointer are both settled at run time and both say nothing
            // happens. A compiler that moves the read in front of the loop reads through a
            // null pointer, so this case does not print the wrong answer, it stops.
            program.input(Ty::I32, "bound", 0);
            program.blank();
            program.line("static int cell = 41;");
            program.line("int *at = bound ? &cell : (int *)0;");
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < bound; i++) {");
            program.line_at(1, format!("total += ({name})*at;"));
            program.line("}");
            0
        }
        "load-that-always-runs" => {
            // The same shape with the answer the other way. The body runs before the test, so
            // the read was going to happen whatever else is true, and moving it in front of
            // the loop cannot fault where the loop did not.
            program.input(Ty::I32, "bound", 8);
            program.blank();
            program.line("static int cell = 41;");
            program.line("int *at = bound ? &cell : (int *)0;");
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("do {");
            program.line_at(1, format!("total += ({name})(*at + i);"));
            program.line_at(1, "i++;");
            program.line("} while (i < bound);");
            (0..8i128).map(|at| 41 + at).sum()
        }
        "divide-at-the-bottom" => {
            // Section 27.1's middle answer. The divisor is not known to be non zero, so the
            // divide may not go just anywhere, and the body of a loop that tests at the
            // bottom is not just anywhere: it runs on every entry to the loop.
            program.input(ty, "top", 84);
            program.input(ty, "bottom", 4);
            program.input(Ty::I32, "bound", 8);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("do {");
            program.line_at(1, format!("total += top / bottom + ({name})i;"));
            program.line_at(1, "i++;");
            program.line("} while (i < bound);");
            (0..8i128).map(|at| 21 + at).sum()
        }
        "divide-that-may-not-run" => {
            // The same divide where the loop does not run and the divisor is zero. Nothing
            // in the program divides by zero. A compiler that hoists it makes one that does.
            program.input(ty, "top", 84);
            program.input(ty, "bottom", 0);
            program.input(Ty::I32, "bound", 0);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < bound; i++) {");
            program.line_at(1, "total += top / bottom;");
            program.line("}");
            0
        }
        "divide-under-a-test" => {
            // And once more with the loop running every iteration and the divide under a test
            // that is false every time. This is the one a pass gets wrong by treating the
            // loop body as one region instead of asking which parts of it always run.
            program.input(ty, "top", 84);
            program.input(ty, "bottom", 0);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, "if (bottom) {");
            program.line_at(2, "total += top / bottom;");
            program.line_at(1, "} else {");
            program.line_at(2, format!("total += ({name})i;"));
            program.line_at(1, "}");
            program.line("}");
            (0..8i128).sum()
        }
        "volatile-load" => {
            // Invariant by every test a pass applies and it still may not move, because the
            // reads are what the program is for. The output cannot tell you whether it moved.
            // The remark can, and that is what this case is read with.
            program.top("static volatile int watched = 5;");
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("total += ({name})(watched + i);"));
            program.line("}");
            (0..8i128).map(|at| 5 + at).sum()
        }
        "a-chain-of-three" => {
            // Each one is invariant only once the one before it has been found to be, so a
            // pass that walks the body in order gets all three in one go and a pass that
            // walks it in the wrong order gets none of them.
            program.input(ty, "a", 3);
            program.input(ty, "b", 5);
            program.input(ty, "c", 2);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("total += (a * b + c) * (a + b) + ({name})i;"));
            program.line("}");
            let invariant = (3 * 5 + 2) * (3 + 5);
            (0..8i128).map(|at| invariant + at).sum()
        }
        "out-of-a-nest" => {
            // Invariant with respect to both loops, so it belongs in front of the outer one.
            // A pass that only lifts it one level leaves three quarters of the work behind.
            program.input(ty, "a", 3);
            program.input(ty, "b", 5);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int o = 0; o < 4; o++) {");
            program.line_at(1, "for (int i = 0; i < 4; i++) {");
            program.line_at(2, format!("total += a * b + ({name})(o + i);"));
            program.line_at(1, "}");
            program.line("}");
            (0..4i128).flat_map(|o| (0..4i128).map(move |i| 15 + o + i)).sum()
        }
        "under-pressure" => {
            // Eight accumulators and eight inputs, all live across the loop, and one cheap
            // invariant on top. Hoisting the invariant here buys one add and costs a spill
            // and a reload every iteration, which is the trade section 27.2 exists to refuse.
            let inputs = ["a", "b", "c", "d", "e", "f", "g", "h"];
            for (at, input) in inputs.iter().enumerate() {
                program.input(ty, input, at as i128 + 1);
            }
            program.blank();
            program.line(format!("{name} total = 0;"));
            for at in 0..inputs.len() {
                program.line(format!("{name} v{at} = 0;"));
            }
            program.line("for (int i = 0; i < 8; i++) {");
            for (at, input) in inputs.iter().enumerate() {
                program.line_at(1, format!("v{at} += {input} + ({name})i;"));
            }
            program.line_at(1, "total += a + b;");
            program.line("}");
            program.blank();
            program.line(format!(
                "total += {};",
                (0..inputs.len()).map(|at| format!("v{at}")).collect::<Vec<_>>().join(" + ")
            ));
            let held: i128 =
                (1..=8i128).map(|start| (0..8i128).map(|at| start + at).sum::<i128>()).sum();
            held + 8 * (1 + 2)
        }
        "an-array-element" => {
            // Two invariant values and only one of them worth moving. The address of the
            // array is worked out again wherever it is wanted, so on its own it is not worth
            // a register, and it still has to travel because the read of it is.
            program.top("static int table[8] = {0, 1, 4, 9, 16, 25, 36, 49};");
            program.input(Ty::I32, "which", 3);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("total += ({name})(table[which & 7] + i);"));
            program.line("}");
            (0..8i128).map(|at| 9 + at).sum()
        }
        "a-call-with-nothing-in-it" => {
            // A call to a function that reads nothing and writes nothing, on an argument the
            // loop does not change. rucc will not move this yet, because a pass is handed a
            // function and the purity of another one is not in it. The case is here to say
            // what the gap costs and to have something to measure the day it closes.
            program.top("static int square(int of) {");
            program.top("    return of * of;");
            program.top("}");
            program.input(Ty::I32, "a", 3);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("total += ({name})(square(a) + i);"));
            program.line("}");
            (0..8i128).map(|at| 9 + at).sum()
        }
        _ => {
            // Section 27.3, store motion, which is the other half of the same question and
            // the half rucc has not built. The store is unconditional and its value does not
            // change, so it belongs after the loop rather than inside it.
            program.top("static int written;");
            program.input(ty, "a", 3);
            program.input(ty, "b", 5);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, "written = (int)(a + b);");
            program.line_at(1, format!("total += ({name})i;"));
            program.line("}");
            program.blank();
            program.check(Ty::I32, "written", 8);
            (0..8i128).sum()
        }
    };
    if !fits(ty, total) {
        return None;
    }
    program.blank();
    program.check(ty.promoted(), "total", total);
    Some(program)
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

/// The loop shapes whose trip count is the thing being tested.
///
/// Everything a compiler does to a loop is downstream of how many times it thinks the loop
/// runs. A wrong trip count is the worst bug the optimizer can have, because it turns into a
/// loop that runs the wrong number of times after unrolling and there is no later pass that
/// notices. So every case here counts its own iterations and prints the count, and the number
/// it prints was worked out by the generator running the same recurrence in Rust.
///
/// The shapes are the ones where the arithmetic is not the obvious subtraction. An exit on
/// `!=` needs the step to divide the distance. A step that does not divide the distance means
/// the last iteration is short. An unsigned counter that wraps is defined and the count is not
/// the difference of the endpoints. A counter that starts past its limit runs no times at all,
/// which is the case a compiler that computes the count as a difference gets spectacularly
/// wrong.
fn trip_counts(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "exit-on-not-equal",
        "step-does-not-divide",
        "starts-past-the-limit",
        "unsigned-wraps-round",
        "counts-down",
        "walks-a-pointer",
        "derived-counter",
        "two-exits",
        "inner-depends-on-outer",
        "unknown-limit",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::InductionVariable) {
            return;
        }
        let mut program = Program::new(format!("a loop that {shape}, counted by the generator"));
        match shape {
            "exit-on-not-equal" => {
                // The step divides the distance exactly, so the loop ends. If it did not the
                // counter would run past the limit and round the type, and a compiler that
                // computed the count without checking would say five.
                program.line("int counted = 0;");
                program.line("int total = 0;");
                program.line("for (int i = 0; i != 10; i += 2) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "total += i;");
                program.line("}");
                program.blank();
                let mut counted = 0i128;
                let mut total = 0i128;
                let mut i = 0i128;
                while i != 10 {
                    counted += 1;
                    total += i;
                    i += 2;
                }
                program.check(Ty::I32, "counted", counted);
                program.check(Ty::I32, "total", total);
            }
            "step-does-not-divide" => {
                // Ten over three is three and a bit, and the answer is four. Rounding the
                // division the other way is the single most common trip count bug there is.
                program.line("int counted = 0;");
                program.line("int last = -1;");
                program.line("for (int i = 0; i < 10; i += 3) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "last = i;");
                program.line("}");
                program.blank();
                let mut counted = 0i128;
                let mut last = -1i128;
                let mut i = 0i128;
                while i < 10 {
                    counted += 1;
                    last = i;
                    i += 3;
                }
                program.check(Ty::I32, "counted", counted);
                program.check(Ty::I32, "last", last);
            }
            "starts-past-the-limit" => {
                // Zero iterations. The body sets the answer to something wrong, so a compiler
                // that peeled an iteration it should not have prints that instead.
                program.input(Ty::I32, "start", 20);
                program.blank();
                program.line("int counted = 0;");
                program.line("int touched = 0;");
                program.line("for (int i = start; i < 10; i++) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "touched = 100;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "counted", 0);
                program.check(Ty::I32, "touched", 0);
            }
            "unsigned-wraps-round" => {
                // Unsigned arithmetic wraps and the standard says so, so this loop is defined
                // and it runs twelve times rather than the negative number the subtraction of
                // the endpoints suggests.
                program.line("int counted = 0;");
                program.line("unsigned int total = 0u;");
                program.line("for (unsigned int i = 4294967288u; i != 4u; i++) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "total += 1u;");
                program.line("}");
                program.blank();
                let mut counted = 0i128;
                let mut i = 4_294_967_288u32;
                while i != 4 {
                    counted += 1;
                    i = i.wrapping_add(1);
                }
                program.check(Ty::I32, "counted", counted);
                program.check(Ty::U32, "total", counted);
            }
            "counts-down" => {
                // The step is negative, so the distance and the direction have to be worked
                // out together. The loop ends on a comparison with zero, which is where a
                // compiler is most tempted to assume the counter is unsigned.
                program.line("int counted = 0;");
                program.line("int total = 0;");
                program.line("for (int i = 10; i > 0; i -= 3) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "total += i;");
                program.line("}");
                program.blank();
                let mut counted = 0i128;
                let mut total = 0i128;
                let mut i = 10i128;
                while i > 0 {
                    counted += 1;
                    total += i;
                    i -= 3;
                }
                program.check(Ty::I32, "counted", counted);
                program.check(Ty::I32, "total", total);
            }
            "walks-a-pointer" => {
                // The counter is a pointer, so the step is the width of what it points at and
                // the trip count is a division the compiler has to do rather than read off.
                program.line("int values[12];");
                program.line("for (int i = 0; i < 12; i++) {");
                program.line_at(1, "values[i] = i * 2;");
                program.line("}");
                program.line("int counted = 0;");
                program.line("int total = 0;");
                program.line("for (int *at = values; at != values + 12; at += 3) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "total += *at;");
                program.line("}");
                program.blank();
                let mut counted = 0i128;
                let mut total = 0i128;
                let mut at = 0i128;
                while at != 12 {
                    counted += 1;
                    total += at * 2;
                    at += 3;
                }
                program.check(Ty::I32, "counted", counted);
                program.check(Ty::I32, "total", total);
            }
            "derived-counter" => {
                // A second variable that is a linear function of the first. Rewriting the loop
                // in terms of either one is allowed, and the two have to stay in step.
                program.line("int counted = 0;");
                program.line("int derived = 1;");
                program.line("int total = 0;");
                program.line("for (int i = 0; i < 9; i++) {");
                program.line_at(1, "derived = i * 3 + 1;");
                program.line_at(1, "total += derived;");
                program.line_at(1, "counted++;");
                program.line("}");
                program.blank();
                let mut total = 0i128;
                for i in 0..9i128 {
                    total += i * 3 + 1;
                }
                program.check(Ty::I32, "counted", 9);
                program.check(Ty::I32, "derived", 8 * 3 + 1);
                program.check(Ty::I32, "total", total);
            }
            "two-exits" => {
                // Two ways out, so there is no single trip count. What a compiler may say is a
                // bound, and the case checks which exit ran as well as how many times round.
                program.input(Ty::I32, "stop", 5);
                program.blank();
                program.line("int counted = 0;");
                program.line("int reason = 0;");
                program.line("for (int i = 0; i < 20; i++) {");
                program.line_at(1, "if (i == stop) { reason = 1; break; }");
                program.line_at(1, "counted++;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "counted", 5);
                program.check(Ty::I32, "reason", 1);
            }
            "inner-depends-on-outer" => {
                // The inner trip count is the outer counter, so the total is a triangle rather
                // than a rectangle and the compiler cannot use one number for both.
                program.line("int counted = 0;");
                program.line("int total = 0;");
                program.line("for (int i = 0; i < 8; i++) {");
                program.line_at(1, "for (int j = 0; j < i; j++) {");
                program.line_at(2, "counted++;");
                program.line_at(2, "total += j;");
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                let mut counted = 0i128;
                let mut total = 0i128;
                for i in 0..8i128 {
                    for j in 0..i {
                        counted += 1;
                        total += j;
                    }
                }
                program.check(Ty::I32, "counted", counted);
                program.check(Ty::I32, "total", total);
            }
            _ => {
                // The limit comes out of a volatile, so the count is not known at compile
                // time at all. What a compiler may still say is that the counter is bounded
                // by the limit, and the case checks that the loop ran the number of times the
                // limit says once the limit is known at run time.
                program.input(Ty::I32, "limit", 7);
                program.blank();
                program.line("int counted = 0;");
                program.line("int total = 0;");
                program.line("for (int i = 0; i < limit; i++) {");
                program.line_at(1, "counted++;");
                program.line_at(1, "total += i;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "counted", 7);
                program.check(Ty::I32, "total", 21);
            }
        }
        sink.push(Facet::InductionVariable, Axes::of([("shape", shape)]), Dialect::C17, program);
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
                let total: i128 =
                    (0..trips128).map(|at| if flag == 1 { at * 2 } else { at + 1 }).sum();
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
                        program.check(ty.promoted(), &format!("target[{}]", trips - 1), trips128);
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
                    Axes::of([("type", ty.name()), ("trips", &trips.to_string()), ("kind", kind)]),
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

/// The shapes a loop can arrive in, rather than the arithmetic it does.
///
/// Every loop pass after canonicalization is allowed to assume the loop has a preheader, one
/// latch, exits that belong to it, values handed over at those exits, and a test at the
/// bottom. Nothing else in this corpus is written to notice when one of those is wrong,
/// because every other loop case prints the right number whatever shape the loop came out in.
/// That is the gap these cases fill: they still print a number the generator worked out, and
/// they are also the programs to point `-fopt-info` at, since a case here that says it wants
/// its header copied and gets told the header was declined has failed even though the number
/// is right.
///
/// The first eight are canonicalization and the last eight are header copying. Two of the
/// eight are shapes the whole corpus never produced once, a loop the ranges can prove never
/// runs and a header over either size limit, so neither limit had any evidence behind it.
///
/// The remarks these get on rucc are worth writing down, because two of them are not what
/// they look like. `live-out-from-header` is copied rather than declined, and that is right:
/// canonicalization puts the loop in loop closed form first, so the value is handed over at
/// the exit and the header no longer defines anything read outside. The shapes that really
/// do get declined for an escaping value are `two-exits-one-join` and `live-out-multi-exit`,
/// which says loop closed form covers the single exit case and not the several exit one,
/// which is what section 26.4 of the rucc spec warns about from the other side.
fn loop_shape(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "two-exits-one-join",
        "two-latches",
        "header-many-preds",
        "live-out-once",
        "live-out-twice",
        "live-out-multi-exit",
        "already-canonical",
        "irreducible",
        "while-to-do-while",
        "entry-provable",
        "entry-disprovable",
        "entry-unknown",
        "header-store",
        "header-over-size-limit",
        "header-over-speed-limit",
        "live-out-from-header",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::LoopShape) {
                return;
            }
            let Some(program) = shaped(ty, shape) else { continue };
            sink.push(
                Facet::LoopShape,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The program for one shape, or nothing when the answer will not fit the accumulator.
///
/// One function per shape would read better and would put sixteen names in a file that has
/// nine. The shapes share the same frame, a loop that accumulates and a check afterwards, and
/// what differs between them is a handful of lines, so they are written out here where a
/// reader can see the sixteen next to each other and tell what each one is doing that its
/// neighbour is not.
#[expect(clippy::too_many_lines, reason = "sixteen shapes read better side by side")]
fn shaped(ty: Ty, shape: &str) -> Option<Program> {
    let name = ty.c_name();
    let mut program = Program::new(match shape {
        "two-exits-one-join" => format!("a {name} loop with two exits that leave to one block"),
        "two-latches" => format!("a {name} loop whose `continue` gives it a second back edge"),
        "header-many-preds" => format!("a {name} loop two blocks outside it jump into"),
        "live-out-once" => format!("a {name} loop whose counter is read once afterwards"),
        "live-out-twice" => format!("a {name} loop whose counter is read twice afterwards"),
        "live-out-multi-exit" => format!("a {name} loop read afterwards that leaves two ways"),
        "already-canonical" => format!("a {name} loop that is already in all five shapes"),
        "irreducible" => format!("a {name} cycle of two blocks entered at both of them"),
        "while-to-do-while" => format!("a plain {name} `while` loop, which should end up a `do`"),
        "entry-provable" => format!("a {name} loop the ranges can prove runs at least once"),
        "entry-disprovable" => format!("a {name} loop the ranges can prove never runs"),
        "entry-unknown" => format!("a {name} loop whose entry test the ranges cannot settle"),
        "header-store" => format!("a {name} loop whose test writes to memory"),
        "header-over-size-limit" => format!("a {name} loop header over the -Os limit of five"),
        "header-over-speed-limit" => format!("a {name} loop header over the -O1 limit of twenty"),
        _ => format!("a {name} loop whose header defines a value read after the loop"),
    });

    // The bound is an input rather than a literal wherever the point of the shape is that the
    // compiler cannot see the trip count. Where the point is the opposite, that the ranges
    // can see it, the case still reads an input and then narrows it with arithmetic, because
    // a literal would be settled by constant folding long before the ranges were asked.
    let total: i128 = match shape {
        "two-exits-one-join" => {
            program.input(Ty::I32, "first", 5);
            program.input(Ty::I32, "second", 9);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 16; i++) {");
            program.line_at(1, "if (i == first) break;");
            program.line_at(1, "if (i == second) break;");
            program.line_at(1, format!("total += ({name})i;"));
            program.line("}");
            (0..5).sum()
        }
        "two-latches" => {
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 12; i++) {");
            program.line_at(1, "switch (i % 3) {");
            program.line_at(1, "case 0:");
            program.line_at(2, "total += 1;");
            program.line_at(2, "continue;");
            program.line_at(1, "case 1:");
            program.line_at(2, "total += 2;");
            program.line_at(2, "break;");
            program.line_at(1, "default:");
            program.line_at(2, "total += 3;");
            program.line_at(2, "break;");
            program.line_at(1, "}");
            program.line_at(1, "total += 10;");
            program.line("}");
            (0..12i128)
                .map(|at| match at % 3 {
                    0 => 1,
                    1 => 12,
                    _ => 13,
                })
                .sum()
        }
        "header-many-preds" => {
            program.input(Ty::I32, "which", 1);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("if (which) goto top;");
            program.line("total += 1;");
            program.line("goto top;");
            program.line("top:");
            program.line("while (i < 8) {");
            program.line_at(1, format!("total += ({name})i;"));
            program.line_at(1, "i++;");
            program.line("}");
            (0..8).sum()
        }
        "live-out-once" | "live-out-twice" => {
            program.input(Ty::I32, "bound", 10);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i;");
            program.line("for (i = 0; i < bound; i++) {");
            program.line_at(1, format!("total += ({name})i;"));
            program.line("}");
            program.blank();
            program.check(Ty::I32, "i", 10);
            if shape == "live-out-twice" {
                program.check(Ty::I32, "i * 2", 20);
            }
            (0..10).sum()
        }
        "live-out-multi-exit" => {
            program.input(Ty::I32, "bound", 10);
            program.input(Ty::I32, "stop", 6);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i;");
            program.line("for (i = 0; i < bound; i++) {");
            program.line_at(1, "if (i == stop) break;");
            program.line_at(1, format!("total += ({name})i;"));
            program.line("}");
            program.blank();
            program.check(Ty::I32, "i", 6);
            (0..6).sum()
        }
        "already-canonical" => {
            program.input(Ty::I32, "bound", 8);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("if (0 < bound) {");
            program.line_at(1, "do {");
            program.line_at(2, format!("total += ({name})i;"));
            program.line_at(2, "i++;");
            program.line_at(1, "} while (i < bound);");
            program.line("}");
            (0..8).sum()
        }
        "irreducible" => {
            program.input(Ty::I32, "start_odd", 0);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("if (start_odd) goto odd;");
            program.line("even:");
            program.line("if (i >= 8) goto done;");
            program.line(format!("total += ({name})i;"));
            program.line("i++;");
            program.line("goto odd;");
            program.line("odd:");
            program.line("if (i >= 8) goto done;");
            program.line(format!("total += ({name})(2 * i);"));
            program.line("i++;");
            program.line("goto even;");
            program.line("done:;");
            (0..8i128).map(|at| if at % 2 == 0 { at } else { 2 * at }).sum()
        }
        "while-to-do-while" => {
            program.input(Ty::I32, "bound", 10);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("while (i < bound) {");
            program.line_at(1, format!("total += ({name})i;"));
            program.line_at(1, "i++;");
            program.line("}");
            (0..10).sum()
        }
        "entry-provable" => {
            // `% 8 + 1` is one to eight, so the ranges know the loop runs whatever the input
            // was. Getting that from a literal instead would prove nothing about the ranges,
            // because folding would have settled it several passes earlier.
            program.input(Ty::U32, "seed", 13);
            program.blank();
            program.line("unsigned bound = seed % 8u + 1u;");
            program.line(format!("{name} total = 0;"));
            program.line("unsigned i = 0;");
            program.line("while (i < bound) {");
            program.line_at(1, format!("total += ({name})i;"));
            program.line_at(1, "i++;");
            program.line("}");
            (0..6).sum()
        }
        "entry-disprovable" => {
            // Nought to three against a counter that starts at eight, so the ranges know the
            // test can never hold and the whole loop goes.
            program.input(Ty::U32, "seed", 13);
            program.blank();
            program.line("unsigned bound = seed % 4u;");
            program.line(format!("{name} total = 0;"));
            program.line("unsigned i = 8;");
            program.line("while (i < bound) {");
            program.line_at(1, format!("total += ({name})i;"));
            program.line_at(1, "i++;");
            program.line("}");
            0
        }
        "entry-unknown" => {
            program.input(Ty::U32, "bound", 9);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("unsigned i = 0;");
            program.line("while (i < bound) {");
            program.line_at(1, format!("total += ({name})i;"));
            program.line_at(1, "i++;");
            program.line("}");
            (0..9).sum()
        }
        "header-store" => {
            program.top("static int counter;");
            program.top("");
            program.top("static int step(void) {");
            program.top("    counter++;");
            program.top("    return counter < 8;");
            program.top("}");
            program.line(format!("{name} total = 0;"));
            program.line("while (step()) {");
            program.line_at(1, format!("total += ({name})counter;"));
            program.line("}");
            (1..8).sum()
        }
        "header-over-size-limit" | "header-over-speed-limit" => {
            // Exclusive ors rather than a sum of the counter, because a chain a compiler can
            // reassociate into one multiply is a header that is over the limit on the way in
            // and under it by the time anybody asks.
            let terms: i128 = if shape == "header-over-size-limit" { 3 } else { 11 };
            let at = |i: i128| (1..=terms).map(|k| i ^ k).sum::<i128>();
            let bound = at(0) + 4;
            let trips = (0..).take_while(|&i| at(i) < bound).count() as i128;
            let sum = (1..=terms).map(|k| format!("(i ^ {k})")).collect::<Vec<_>>().join(" + ");
            program.input(Ty::I32, "bound", bound);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line(format!("while ({sum} < bound) {{"));
            program.line_at(1, format!("total += ({name})i;"));
            program.line_at(1, "i++;");
            program.line("}");
            (0..trips).sum()
        }
        _ => {
            program.input(Ty::I32, "bound", 20);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("int i = 0;");
            program.line("int last = 0;");
            program.line("while ((last = i * 3) < bound) {");
            program.line_at(1, format!("total += ({name})last;"));
            program.line_at(1, "i++;");
            program.line("}");
            program.blank();
            program.check(Ty::I32, "last", 21);
            (0..).map(|i: i128| i * 3).take_while(|&last| last < 20).sum()
        }
    };
    if !fits(ty, total) {
        return None;
    }
    program.blank();
    program.check(ty.promoted(), "total", total);
    Some(program)
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
                let mut program =
                    Program::new(format!("a {side} by {side} {} traversal, {shape}", ty.c_name()));
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
                    Axes::of([("type", ty.name()), ("side", &side.to_string()), ("shape", shape)]),
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
    fn every_hoisting_shape_is_generated_for_every_type() {
        let cases = cases_for(Facet::LoopHoist);
        for ty in ["i32", "u32", "i64", "u64"] {
            let shapes: Vec<&str> = cases
                .iter()
                .filter(|c| c.axes.get("type") == Some(ty))
                .filter_map(|c| c.axes.get("shape"))
                .collect();
            assert_eq!(shapes.len(), 14, "{ty}");
            assert!(shapes.contains(&"divide-under-a-test"), "{ty}");
            assert!(shapes.contains(&"volatile-load"), "{ty}");
        }
    }

    #[test]
    fn the_two_shapes_that_fault_when_hoisted_expect_nothing_to_happen() {
        // Both loops run zero times, so both print zero. That is the whole answer and it is
        // the wrong thing to look at: what these two are for is that a compiler which moves
        // the work in front of the loop divides by zero or reads through a null pointer, and
        // a program that stops has not printed anything either way.
        let cases = cases_for(Facet::LoopHoist);
        for shape in ["divide-that-may-not-run", "load-that-may-not-run"] {
            let case = cases
                .iter()
                .find(|c| c.axes.get("type") == Some("i64") && c.axes.get("shape") == Some(shape))
                .unwrap();
            assert_eq!(output(case), "0\n", "{shape}");
        }
    }

    #[test]
    fn the_same_divide_expects_the_same_answer_whichever_end_the_loop_tests_at() {
        // The bottom tested loop runs eight times and adds the counter, the guarded one runs
        // eight times and adds the counter instead of dividing. The difference between them
        // is the twenty one the divide contributes each time, and nothing else.
        let cases = cases_for(Facet::LoopHoist);
        let answer = |shape: &str| {
            let case = cases
                .iter()
                .find(|c| c.axes.get("type") == Some("i32") && c.axes.get("shape") == Some(shape))
                .unwrap();
            output(case).trim().parse::<i64>().unwrap()
        };
        assert_eq!(answer("divide-at-the-bottom") - answer("divide-under-a-test"), 8 * 21);
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
