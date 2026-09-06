//! The three facets that are not about one transformation.
//!
//! `baseline` is the floor. If these fail, nothing else in the report means anything, because
//! the compiler is wrong before any optimization has run.
//!
//! `branch-probability` is about the odds the compiler puts on an edge before it has run
//! anything. Nothing in a program tells you those odds directly, so each case here is written
//! so that one arm is the one the static predictors are supposed to pick and the other is not,
//! and the axis says which one the program actually takes.
//!
//! `barrier` is the opposite of every other facet. Every case here is a place where the
//! compiler must not act, and the evidence that it did not is that the answer is right and
//! the access count in the object file did not go down.
//!
//! `frontend` is about the shape of the language rather than about code generation. It is
//! where the C23 constructs live, and it is the only facet that contains programs which are
//! supposed to be rejected.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// Programs with no optimization target, which every level has to get right.
///
/// These are ordinary small programs of the kind anybody would write. They exist so that a
/// run against a compiler that is broken in some basic way produces a short readable list of
/// failures at the top of the report instead of nine hundred failures spread across every
/// facet, which tells you nothing about where to look.
pub(crate) fn baseline(sink: &mut Sink<'_>) {
    const PROGRAMS: &[(&str, &str)] = &[
        ("arithmetic", "the four operations on ordinary numbers"),
        ("locals", "locals, assignment and sequencing"),
        ("branches", "if, else and the comparison operators"),
        ("loops", "for, while and do while"),
        ("arrays", "indexing, and walking an array with a pointer"),
        ("structs", "members, nesting and assignment of a whole struct"),
        ("strings", "a character array and a walk to its terminator"),
        ("recursion", "a function that calls itself"),
        ("pointers", "taking an address, dereferencing and pointer arithmetic"),
        ("sort", "a bubble sort, which uses most of the above at once"),
    ];
    for &(name, purpose) in PROGRAMS {
        if !sink.wants(Facet::Baseline) {
            return;
        }
        let mut program = Program::new(purpose);
        match name {
            "arithmetic" => {
                program.input(Ty::I32, "a", 37);
                program.input(Ty::I32, "b", 5);
                program.blank();
                program.check(Ty::I32, "a + b", 42);
                program.check(Ty::I32, "a - b", 32);
                program.check(Ty::I32, "a * b", 185);
                program.check(Ty::I32, "a / b", 7);
                program.check(Ty::I32, "a % b", 2);
            }
            "locals" => {
                program.input(Ty::I32, "seed", 3);
                program.blank();
                program.line("int one = seed;");
                program.line("int two = one + one;");
                program.line("int three = two * two;");
                program.line("one = three - two;");
                program.blank();
                program.check(Ty::I32, "one", 30);
                program.check(Ty::I32, "two", 6);
                program.check(Ty::I32, "three", 36);
            }
            "branches" => {
                program.input(Ty::I32, "value", 7);
                program.blank();
                program.line("int picked;");
                program.line("if (value < 5) {");
                program.line_at(1, "picked = 1;");
                program.line("} else if (value < 10) {");
                program.line_at(1, "picked = 2;");
                program.line("} else {");
                program.line_at(1, "picked = 3;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "picked", 2);
                program.check(Ty::I32, "value == 7", 1);
                program.check(Ty::I32, "value != 7", 0);
                program.check(Ty::I32, "value >= 7 && value <= 7", 1);
                program.check(Ty::I32, "value < 0 || value > 0", 1);
            }
            "loops" => {
                program.line("int total = 0;");
                program.line("for (int i = 1; i <= 10; i++) {");
                program.line_at(1, "total += i;");
                program.line("}");
                program.line("int down = 10;");
                program.line("while (down > 0) {");
                program.line_at(1, "down--;");
                program.line("}");
                program.line("int once = 0;");
                program.line("do {");
                program.line_at(1, "once++;");
                program.line("} while (0);");
                program.blank();
                program.check(Ty::I32, "total", 55);
                program.check(Ty::I32, "down", 0);
                program.check(Ty::I32, "once", 1);
            }
            "arrays" => {
                program.line("int values[8];");
                program.line("for (int i = 0; i < 8; i++) {");
                program.line_at(1, "values[i] = i * i;");
                program.line("}");
                program.line("int total = 0;");
                program.line("for (int *at = values; at != values + 8; at++) {");
                program.line_at(1, "total += *at;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "values[0]", 0);
                program.check(Ty::I32, "values[7]", 49);
                program.check(Ty::I32, "total", 140);
            }
            "structs" => {
                program.top("struct point { int x; int y; };");
                program.top("struct line { struct point from; struct point to; };");
                program.input(Ty::I32, "seed", 2);
                program.blank();
                program.line("struct line edge;");
                program.line("edge.from.x = seed;");
                program.line("edge.from.y = seed * 2;");
                program.line("edge.to.x = seed * 3;");
                program.line("edge.to.y = seed * 4;");
                program.line("struct point copied = edge.to;");
                program.blank();
                program.check(Ty::I32, "edge.from.x + edge.from.y", 6);
                program.check(Ty::I32, "copied.x + copied.y", 14);
            }
            "strings" => {
                program.line("char text[6];");
                program.line("text[0] = 'r';");
                program.line("text[1] = 'u';");
                program.line("text[2] = 'c';");
                program.line("text[3] = 'c';");
                program.line("text[4] = '!';");
                program.line("text[5] = 0;");
                program.line("int length = 0;");
                program.line("while (text[length] != 0) {");
                program.line_at(1, "length++;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "length", 5);
                program.check(Ty::I32, "text[0]", 114);
                program.check(Ty::I32, "text[4]", 33);
            }
            "recursion" => {
                program.top("static int factorial(int value) {");
                program.top("    if (value <= 1) {");
                program.top("        return 1;");
                program.top("    }");
                program.top("    return value * factorial(value - 1);");
                program.top("}");
                program.top("static int fibonacci(int value) {");
                program.top("    if (value < 2) {");
                program.top("        return value;");
                program.top("    }");
                program.top("    return fibonacci(value - 1) + fibonacci(value - 2);");
                program.top("}");
                program.input(Ty::I32, "seed", 10);
                program.blank();
                program.check(Ty::I32, "factorial(seed)", 3_628_800);
                program.check(Ty::I32, "fibonacci(seed)", 55);
            }
            "pointers" => {
                program.input(Ty::I32, "seed", 9);
                program.blank();
                program.line("int values[4] = { 0, 0, 0, 0 };");
                program.line("int *at = values;");
                program.line("*at = seed;");
                program.line("at += 2;");
                program.line("*at = seed * 2;");
                program.line("int **indirect = &at;");
                program.line("**indirect = seed * 3;");
                program.blank();
                program.check(Ty::I32, "values[0]", 9);
                program.check(Ty::I32, "values[2]", 27);
                program.check(Ty::I64, "(long long)(at - values)", 2);
            }
            _ => {
                program.top("static void sort(int *values, int count) {");
                program.top("    for (int pass = 0; pass < count; pass++) {");
                program.top("        for (int at = 0; at + 1 < count; at++) {");
                program.top("            if (values[at] > values[at + 1]) {");
                program.top("                int held = values[at];");
                program.top("                values[at] = values[at + 1];");
                program.top("                values[at + 1] = held;");
                program.top("            }");
                program.top("        }");
                program.top("    }");
                program.top("}");
                program.input(Ty::I32, "seed", 7);
                program.blank();
                program.line("int values[8];");
                program.line("for (int i = 0; i < 8; i++) {");
                program.line_at(1, "values[i] = (i * seed) % 8;");
                program.line("}");
                program.line("sort(values, 8);");
                program.blank();
                // Seven and eight share no factor, so the multiples of seven modulo eight are
                // the numbers nought to seven in some order, and sorting puts them back.
                for at in 0..8 {
                    program.check(Ty::I32, &format!("values[{at}]"), at);
                }
            }
        }
        sink.push(Facet::Baseline, Axes::of([("program", name)]), Dialect::C17, program);
    }
}

/// The control flow shapes that break the analyses sitting under every pass.
///
/// The graph, the two dominance relations and the loop forest are computed before any
/// transformation runs and are consulted by all of them. When one of them is wrong the
/// failure surfaces somewhere else entirely, in whichever pass happened to ask, so the shapes
/// that break them deserve programs of their own rather than being reached by accident from a
/// facet about something else.
///
/// None of these is about an optimization, and the numbers they print are small. What they
/// check is that the compiler produced a correct program at all for a shape that a
/// hand written analysis gets wrong: a cycle with two entries, a loop the analysis cannot see
/// the end of, a switch where two arms are one block, and a jump whose target is a value.
pub(crate) fn control_flow(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "irreducible",
        "nested-five-deep",
        "jumped-over-block",
        "loop-with-no-exit",
        "switch-shared-arms",
        "computed-goto",
        "many-returns",
        "wide-if-chain",
        "two-exit-loop",
        "continue-and-break",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::ControlFlow) {
            return;
        }
        let mut program = Program::new(format!("a {shape} shape, which the analyses have to fit"));
        let mut gnu = false;
        match shape {
            "irreducible" => {
                // Two ways into the same cycle, which is what makes it irreducible. A natural
                // loop finder has to refuse this rather than pick one of the two entries and
                // call it a header, and rucc records it as a cycle that is not a loop.
                program.input(Ty::I32, "enter_second", 1);
                program.blank();
                program.line("int total = 0;");
                program.line("if (enter_second) goto second;");
                program.line("first:");
                program.line_at(1, "total += 1;");
                program.line_at(1, "if (total > 20) goto out;");
                program.line_at(1, "goto second;");
                program.line("second:");
                program.line_at(1, "total += 2;");
                program.line_at(1, "if (total > 20) goto out;");
                program.line_at(1, "goto first;");
                program.line("out:");
                program.line_at(1, ";");
                program.blank();
                let mut total = 0i128;
                let mut at_second = true;
                loop {
                    total += if at_second { 2 } else { 1 };
                    if total > 20 {
                        break;
                    }
                    at_second = !at_second;
                }
                program.check(Ty::I32, "total", total);
            }
            "nested-five-deep" => {
                // The loop forest has to be five deep and the parent of each level has to be
                // the level above it. A flat list of loops passes every other facet.
                program.line("int total = 0;");
                for depth in 0..5 {
                    program.line_at(
                        depth,
                        format!("for (int i{depth} = 0; i{depth} < 3; i{depth}++) {{"),
                    );
                }
                program.line_at(5, "total += i0 + i1 + i2 + i3 + i4;");
                for depth in (0..5).rev() {
                    program.line_at(depth, "}");
                }
                program.blank();
                // Each of the five counters runs nought, one, two over three to the fourth
                // iterations of the others, so each contributes three times eighty one.
                let total = 5 * 3 * 81;
                program.check(Ty::I32, "total", i128::from(total));
            }
            "jumped-over-block" => {
                // A block with no predecessor. Everything downstream of the analysis has to
                // agree it cannot run, and the wrong answer is in it so a compiler that keeps
                // it and somehow reaches it fails loudly.
                program.input(Ty::I32, "seed", 4);
                program.blank();
                program.line("int total = seed;");
                program.line("goto past;");
                program.line("total = total * 100;");
                program.line("past:");
                program.line_at(1, "total += 1;");
                program.blank();
                program.check(Ty::I32, "total", 5);
            }
            "loop-with-no-exit" => {
                // A loop the compiler cannot see the end of, in a function that is compiled
                // and never called. This is the shape that makes post-dominance need an edge
                // to a fake exit, because without one there is no path from the loop to the
                // end of the function and the relation is not defined.
                program.top("static void spin(void) {");
                program.top("    for (;;) {");
                program.top("    }");
                program.top("}");
                program.input(Ty::I32, "never", 0);
                program.blank();
                program.line("int total = 1;");
                program.line("if (never) {");
                program.line_at(1, "spin();");
                program.line_at(1, "total = 100;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total", 1);
            }
            "switch-shared-arms" => {
                // Two arms that are one block, and a default that is also an arm target. A
                // graph builder that assumes one edge per case ends up with a block that has
                // fewer predecessors than it really has, and every dominance answer below it
                // is then wrong.
                program.input(Ty::I32, "pick", 3);
                program.blank();
                program.line("int total = 0;");
                program.line("switch (pick) {");
                program.line_at(1, "case 0:");
                program.line_at(1, "case 1:");
                program.line_at(2, "total = 10;");
                program.line_at(2, "break;");
                program.line_at(1, "case 2:");
                program.line_at(1, "case 3:");
                program.line_at(1, "default:");
                program.line_at(2, "total = 20;");
                program.line_at(2, "break;");
                program.line("}");
                program.line("switch (pick) {");
                program.line_at(1, "case 3:");
                program.line_at(2, "total += 1;");
                program.line_at(2, "/* falls through */");
                program.line_at(1, "case 4:");
                program.line_at(2, "total += 2;");
                program.line_at(2, "break;");
                program.line_at(1, "default:");
                program.line_at(2, "total += 100;");
                program.line_at(2, "break;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total", 23);
            }
            "computed-goto" => {
                // The jump target is a value out of an array, so the successors of the block
                // are whatever the array holds. This is a GCC extension and it is in the
                // corpus because compatibility with GCC is the goal, but it is tagged so a
                // run against a compiler that has not got to it yet can leave it out.
                //
                // The array is filled at run time rather than in a static initializer. Both
                // forms are GCC extensions and both belong here, but a static initializer
                // holding label addresses is a second thing to implement on top of the jump
                // itself, and mixing the two into one case means a compiler that has neither
                // cannot say which one it is missing. rucc reports the jump plainly as not
                // lowered yet, which is a gap, and reports the static form as an initializer
                // that is not constant, which is a different bug with its own issue.
                gnu = true;
                program.input(Ty::I32, "pick", 2);
                program.blank();
                program.line("void *targets[4];");
                program.line("targets[0] = &&zero;");
                program.line("targets[1] = &&one;");
                program.line("targets[2] = &&two;");
                program.line("targets[3] = &&three;");
                program.line("int total = 0;");
                program.line("goto *targets[pick & 3];");
                program.line("zero:");
                program.line_at(1, "total += 1;");
                program.line_at(1, "goto done;");
                program.line("one:");
                program.line_at(1, "total += 2;");
                program.line_at(1, "goto done;");
                program.line("two:");
                program.line_at(1, "total += 4;");
                program.line_at(1, "goto done;");
                program.line("three:");
                program.line_at(1, "total += 8;");
                program.line("done:");
                program.line_at(1, "total += 100;");
                program.blank();
                program.check(Ty::I32, "total", 104);
            }
            "many-returns" => {
                // Five exits from one function. The reverse graph has five roots before the
                // fake exit is added, and a post-dominator tree built without noticing that
                // has the wrong root.
                program.top("static int classify(int value) {");
                program.top("    if (value < 0) return 1;");
                program.top("    if (value == 0) return 2;");
                program.top("    if (value < 10) return 3;");
                program.top("    if (value < 100) return 4;");
                program.top("    return 5;");
                program.top("}");
                program.input(Ty::I32, "seed", 7);
                program.blank();
                program.check(Ty::I32, "classify(-seed)", 1);
                program.check(Ty::I32, "classify(0)", 2);
                program.check(Ty::I32, "classify(seed)", 3);
                program.check(Ty::I32, "classify(seed * 7)", 4);
                program.check(Ty::I32, "classify(seed * 100)", 5);
            }
            "wide-if-chain" => {
                // Sixteen branches in a row, none of them nested. The dominator tree is one
                // long spine and the graph is wide rather than deep, which is the shape where
                // a quadratic dominance algorithm stops being fast enough to keep.
                let seed = 5i128;
                program.input(Ty::I32, "seed", seed);
                program.blank();
                program.line("int total = 0;");
                let mut total = 0i128;
                for at in 0..16i128 {
                    program.line(format!("if (seed > {at}) total += {};", at + 1));
                    if seed > at {
                        total += at + 1;
                    }
                }
                program.blank();
                program.check(Ty::I32, "total", total);
            }
            "two-exit-loop" => {
                // A loop that can end two ways. The loop has two exit edges and the block
                // after it has two predecessors, and which one ran decides the answer.
                program.input(Ty::I32, "limit", 6);
                program.blank();
                program.line("int total = 0;");
                program.line("int hit = 0;");
                program.line("for (int i = 0; i < 10; i++) {");
                program.line_at(1, "if (i == limit) { hit = 1; break; }");
                program.line_at(1, "total += i;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total", 15);
                program.check(Ty::I32, "hit", 1);
            }
            _ => {
                // `continue` and `break` in nested loops, which is where the back edge of the
                // inner loop and the exit edge of the outer one are easiest to mix up.
                program.input(Ty::I32, "seed", 2);
                program.blank();
                program.line("int total = 0;");
                program.line("for (int i = 0; i < 6; i++) {");
                program.line_at(1, "if (i % 2) continue;");
                program.line_at(1, "for (int j = 0; j < 6; j++) {");
                program.line_at(2, "if (j > i) break;");
                program.line_at(2, "total += seed;");
                program.line_at(1, "}");
                program.line_at(1, "if (total > 100) break;");
                program.line("}");
                program.blank();
                let mut total = 0i128;
                for i in 0..6i128 {
                    if i % 2 != 0 {
                        continue;
                    }
                    for j in 0..6i128 {
                        if j > i {
                            break;
                        }
                        total += 2;
                    }
                    if total > 100 {
                        break;
                    }
                }
                program.check(Ty::I32, "total", total);
            }
        }
        let axes = Axes::of([("shape", shape)]);
        if gnu {
            sink.push_tagged(Facet::ControlFlow, axes, Dialect::C17, program, &["gnu"]);
        } else {
            sink.push(Facet::ControlFlow, axes, Dialect::C17, program);
        }
    }
}

/// The branches the static predictors are supposed to have an opinion about.
///
/// Section 11.2 of the plan lists the heuristics a compiler uses to guess which way a branch
/// goes before it has any measurement. Each shape here is one of those heuristics, written so
/// the guess is unambiguous: a null pointer test, a comparison against zero, an arm that calls
/// something, an arm that jumps out, a loop back edge, a loop exit.
///
/// Two things are being checked and they are worth keeping apart. The answer must be right
/// whichever way the branch goes, and the `direction` axis exists so that both are run. What
/// the report measures on top of that is the layout: the case with `direction=predicted` is
/// the one where a compiler that believed its own heuristic should have laid the hot arm out
/// to fall through, and the case with `direction=against` is the one where it should not have
/// made things worse for guessing wrong.
pub(crate) fn branch_probability(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "not-negative",
        "not-equal",
        "not-null",
        "arm-with-a-call",
        "arm-that-jumps-out",
        "loop-back-edge",
        "loop-early-exit",
        "never-returns",
        "builtin-expect",
    ];
    for &ty in &[Ty::I32, Ty::I64] {
        for &shape in SHAPES {
            for &direction in &["predicted", "against"] {
                if !sink.wants(Facet::BranchProbability) {
                    return;
                }
                // Nothing to run in the direction that never comes back.
                if shape == "never-returns" && direction == "against" {
                    continue;
                }
                let name = ty.c_name();
                let predicted = direction == "predicted";
                let mut program = Program::new(format!(
                    "a {name} branch the static predictors call {shape}, taken as {direction}"
                ));
                let expected = match shape {
                    "not-negative" => {
                        // A test for a negative value is guessed false.
                        program.input(ty, "x", if predicted { 7 } else { -7 });
                        program.blank();
                        program.line(format!("{name} total;"));
                        program.line("if (x < 0) {");
                        program.line_at(1, "total = -x;");
                        program.line("} else {");
                        program.line_at(1, "total = x;");
                        program.line("}");
                        7
                    }
                    "not-equal" => {
                        // A test for equality against a constant is guessed false.
                        program.input(ty, "x", if predicted { 7 } else { 0 });
                        program.blank();
                        program.line(format!("{name} total;"));
                        program.line("if (x == 0) {");
                        program.line_at(1, "total = 7;");
                        program.line("} else {");
                        program.line_at(1, "total = x;");
                        program.line("}");
                        7
                    }
                    "not-null" => {
                        // A pointer is guessed not to be null.
                        program.input(Ty::I32, "pick", i128::from(predicted));
                        program.blank();
                        program.line(format!("{name} slot = 7;"));
                        program.line(format!("{name} *at = pick ? &slot : 0;"));
                        program.line(format!("{name} total;"));
                        program.line("if (at == 0) {");
                        program.line_at(1, "total = 7;");
                        program.line("} else {");
                        program.line_at(1, "total = *at;");
                        program.line("}");
                        7
                    }
                    "arm-with-a-call" => {
                        // The arm that calls something is the cold one.
                        program.top(format!("static {name} slow({name} v) {{"));
                        program.top("    return v;".to_owned());
                        program.top("}".to_owned());
                        program.input(Ty::I32, "flag", i128::from(!predicted));
                        program.blank();
                        program.line(format!("{name} total;"));
                        program.line("if (flag) {");
                        program.line_at(1, "total = slow(7);");
                        program.line("} else {");
                        program.line_at(1, "total = 7;");
                        program.line("}");
                        7
                    }
                    "arm-that-jumps-out" => {
                        // The arm that leaves the region is the cold one.
                        program.input(Ty::I32, "flag", i128::from(!predicted));
                        program.blank();
                        program.line(format!("{name} total = 0;"));
                        program.line("if (flag) {");
                        program.line_at(1, "total = 7;");
                        program.line_at(1, "goto done;");
                        program.line("}");
                        program.line("total = 7;");
                        program.line("done:;");
                        7
                    }
                    "loop-back-edge" => {
                        // The edge that goes round again is taken far more often than the one
                        // that leaves, which is the one heuristic that is nearly always right.
                        let rounds: i128 = if predicted { 16 } else { 1 };
                        program.input(Ty::I32, "rounds", rounds);
                        program.blank();
                        program.line(format!("{name} total = 0;"));
                        program.line("for (int i = 0; i < rounds; i++) {");
                        program.line_at(1, "total = total + 1;");
                        program.line("}");
                        rounds
                    }
                    "loop-early-exit" => {
                        // An exit out of the middle of a loop is guessed not to be taken.
                        program.input(Ty::I32, "stop", if predicted { 99 } else { 3 });
                        program.blank();
                        program.line(format!("{name} total = 0;"));
                        program.line("for (int i = 0; i < 16; i++) {");
                        program.line_at(1, "if (i == stop) {");
                        program.line_at(2, "break;");
                        program.line_at(1, "}");
                        program.line_at(1, "total = total + 1;");
                        program.line("}");
                        if predicted { 16 } else { 3 }
                    }
                    "never-returns" => {
                        // An arm that cannot come back is never taken, and this one is not.
                        program.top("_Noreturn static void nowhere(void) {".to_owned());
                        program.top("    for (;;) {".to_owned());
                        program.top("    }".to_owned());
                        program.top("}".to_owned());
                        program.input(Ty::I32, "flag", 0);
                        program.blank();
                        program.line(format!("{name} total = 7;"));
                        program.line("if (flag) {");
                        program.line_at(1, "nowhere();");
                        program.line("}");
                        7
                    }
                    _ => {
                        // The one shape where the program says what it expects out loud.
                        program.input(Ty::I32, "flag", i128::from(!predicted));
                        program.blank();
                        program.line(format!("{name} total;"));
                        program.line("if (__builtin_expect(flag != 0, 0)) {");
                        program.line_at(1, "total = 7;");
                        program.line("} else {");
                        program.line_at(1, "total = 7;");
                        program.line("}");
                        7
                    }
                };
                program.blank();
                program.check(ty.promoted(), "total", expected);
                let axes =
                    Axes::of([("type", ty.name()), ("shape", shape), ("direction", direction)]);
                if shape == "builtin-expect" {
                    sink.push_tagged(
                        Facet::BranchProbability,
                        axes,
                        Dialect::C17,
                        program,
                        &["gnu"],
                    );
                } else {
                    sink.push(Facet::BranchProbability, axes, Dialect::C17, program);
                }
            }
        }
    }
}

/// Places where the compiler is required to leave the program alone.
///
/// A `volatile` access has to happen, exactly as many times as it is written, in the order it
/// is written. Type punning through a union is allowed and the compiler has to see the store.
/// A `char` pointer may alias anything, so a store through one has to be assumed to hit
/// whatever the next load reads. Each case checks the value, and the report checks that the
/// accesses are still in the object file.
pub(crate) fn barrier(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "volatile-scalar",
        "volatile-in-loop",
        "volatile-array",
        "volatile-order",
        "char-aliasing",
        "union-punning",
        "pointer-escapes",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::Barrier) {
            return;
        }
        let mut program = Program::new(format!("a {shape} the compiler must not optimize away"));
        match shape {
            "volatile-scalar" => {
                program.top("static volatile int gate = 5;");
                program.blank();
                program.line("int first = gate;");
                program.line("int second = gate;");
                program.line("gate = first + second;");
                program.blank();
                program.check(Ty::I32, "gate", 10);
                program.check(Ty::I32, "first + second", 10);
            }
            "volatile-in-loop" => {
                program.top("static volatile int counter = 0;");
                program.blank();
                program.line("for (int i = 0; i < 100; i++) {");
                program.line_at(1, "counter = counter + 1;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "counter", 100);
            }
            "volatile-array" => {
                program.top("static volatile int slots[4] = { 0, 0, 0, 0 };");
                program.blank();
                program.line("slots[0] = 1;");
                program.line("slots[1] = slots[0] + 1;");
                program.line("slots[2] = slots[1] + 1;");
                program.line("slots[3] = slots[2] + 1;");
                program.blank();
                program.check(Ty::I32, "slots[3]", 4);
            }
            "volatile-order" => {
                program.top("static volatile int port = 0;");
                program.top("static int trace[4];");
                program.blank();
                program.line("port = 1;");
                program.line("trace[0] = port;");
                program.line("port = 2;");
                program.line("trace[1] = port;");
                program.line("port = 3;");
                program.line("trace[2] = port;");
                program.blank();
                program.check(Ty::I32, "trace[0] + trace[1] * 10 + trace[2] * 100", 321);
            }
            "char-aliasing" => {
                program.input(Ty::I32, "seed", 0);
                program.blank();
                program.line("int value = seed;");
                program.line("unsigned char *bytes = (unsigned char *)&value;");
                program.line("bytes[0] = 1;");
                program.blank();
                // Only the lowest addressed byte is written, and which end of the number that
                // is depends on the machine. Masking to that one byte keeps the answer the
                // same on a big endian target and a little endian one.
                program.check(Ty::I32, "bytes[0]", 1);
                program.check(Ty::I32, "value != 0", 1);
            }
            "union-punning" => {
                program.top("union both { unsigned int whole; unsigned char parts[4]; };");
                program.blank();
                program.line("union both held;");
                program.line("held.whole = 0;");
                program.line("held.parts[0] = 0xff;");
                program.blank();
                program.check(Ty::I32, "held.parts[0]", 255);
                program.check(Ty::I32, "held.whole != 0", 1);
            }
            _ => {
                program.top("static int *kept;");
                program.top("static void keep(int *at) { kept = at; }");
                program.top("static void bump(void) { *kept += 1; }");
                program.blank();
                program.line("static int slot = 5;");
                program.line("keep(&slot);");
                program.line("slot = 7;");
                program.line("bump();");
                program.blank();
                program.check(Ty::I32, "slot", 8);
            }
        }
        sink.push(Facet::Barrier, Axes::of([("shape", shape)]), Dialect::C17, program);
    }
}

/// The shape of the language, including everything C23 added.
///
/// Split by dialect, because a program that says `constexpr` cannot be compiled as C17 at all.
/// The C17 half is the awkward corners that have been there for decades and still catch
/// compilers out. The C23 half is what makes this a corpus for a modern compiler rather than
/// one for 1999.
pub(crate) fn frontend(sink: &mut Sink<'_>) {
    accepted_c17(sink);
    accepted_c23(sink);
    rejected(sink);
}

/// The parts of C17 that are easy to get wrong.
fn accepted_c17(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "designated-initializers",
        "compound-literal",
        "generic-selection",
        "anonymous-members",
        "bit-fields",
        "flexible-array",
        "enum-values",
        "comma-and-conditional",
        "nested-declarators",
        "static-assert",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::Frontend) {
            return;
        }
        let mut program = Program::new(format!("the {shape} corner of C17"));
        match shape {
            "designated-initializers" => {
                program.top("struct config { int width; int height; int depth; };");
                program.blank();
                program.line("struct config made = { .depth = 3, .width = 1 };");
                program.line("int table[6] = { [4] = 40, [1] = 10 };");
                program.blank();
                program.check(Ty::I32, "made.width", 1);
                program.check(Ty::I32, "made.height", 0);
                program.check(Ty::I32, "made.depth", 3);
                program.check(Ty::I32, "table[1] + table[4]", 50);
                program.check(Ty::I32, "table[0] + table[2] + table[3] + table[5]", 0);
            }
            "compound-literal" => {
                program.top("struct pair { int first; int second; };");
                program.top(
                    "static int total(struct pair value) { return value.first + value.second; }",
                );
                program.blank();
                program.line("int *slice = (int[]){ 4, 5, 6 };");
                program.blank();
                program.check(Ty::I32, "total((struct pair){ 20, 22 })", 42);
                program.check(Ty::I32, "slice[0] + slice[1] + slice[2]", 15);
            }
            "generic-selection" => {
                program.top("#define name(x) _Generic((x), int: 1, long long: 2, unsigned int: 3, default: 0)");
                program.blank();
                program.line("int as_int = 0;");
                program.line("long long as_long = 0;");
                program.line("unsigned int as_unsigned = 0;");
                program.line("char as_char = 0;");
                program.blank();
                program.check(Ty::I32, "name(as_int)", 1);
                program.check(Ty::I32, "name(as_long)", 2);
                program.check(Ty::I32, "name(as_unsigned)", 3);
                program.check(Ty::I32, "name(as_char)", 0);
            }
            "anonymous-members" => {
                program
                    .top("struct outer { int tag; union { int as_int; unsigned int as_bits; }; };");
                program.blank();
                program.line("struct outer held;");
                program.line("held.tag = 1;");
                program.line("held.as_int = 12;");
                program.blank();
                program.check(Ty::I32, "held.tag", 1);
                program.check(Ty::I32, "(int)held.as_bits", 12);
            }
            "bit-fields" => {
                program.top("struct packed { unsigned int low : 3; unsigned int mid : 5; signed int high : 4; };");
                program.blank();
                program.line("struct packed held;");
                program.line("held.low = 7;");
                program.line("held.mid = 31;");
                program.line("held.high = -8;");
                program.blank();
                program.check(Ty::I32, "(int)held.low", 7);
                program.check(Ty::I32, "(int)held.mid", 31);
                program.check(Ty::I32, "(int)held.high", -8);
            }
            "flexible-array" => {
                program.top("struct message { int count; int values[]; };");
                program.top("static struct message *storage(void) {");
                program.top(
                    "    static union { struct message head; unsigned char space[64]; } backing;",
                );
                program.top("    return &backing.head;");
                program.top("}");
                program.blank();
                program.line("struct message *held = storage();");
                program.line("held->count = 3;");
                program.line("for (int i = 0; i < 3; i++) {");
                program.line_at(1, "held->values[i] = i + 1;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "held->count", 3);
                program.check(Ty::I32, "held->values[0] + held->values[1] + held->values[2]", 6);
            }
            "enum-values" => {
                program.top("enum colour { red, green = 10, blue, violet = -1 };");
                program.blank();
                program.check(Ty::I32, "(int)red", 0);
                program.check(Ty::I32, "(int)green", 10);
                program.check(Ty::I32, "(int)blue", 11);
                program.check(Ty::I32, "(int)violet", -1);
            }
            "comma-and-conditional" => {
                program.input(Ty::I32, "seed", 4);
                program.blank();
                program.line("int stepped = 0;");
                program.line("int value = (stepped = seed, stepped + 1);");
                program.blank();
                program.check(Ty::I32, "value", 5);
                program.check(Ty::I32, "stepped", 4);
                program.check(Ty::I32, "seed > 0 ? seed < 10 ? 1 : 2 : 3", 1);
            }
            "nested-declarators" => {
                program.top("static int add(int a, int b) { return a + b; }");
                program.top(
                    "static int (*chooser(int which))(int, int) { return which ? add : add; }",
                );
                program.blank();
                program.line("int (*table[2])(int, int) = { add, add };");
                program.line("int (*picked)(int, int) = chooser(1);");
                program.blank();
                program.check(Ty::I32, "table[0](2, 3)", 5);
                program.check(Ty::I32, "picked(20, 22)", 42);
            }
            _ => {
                // Nothing here asks about a width, because a case that did would be asking
                // about the target rather than about the compiler, and the corpus is meant to
                // give the same answer wherever it is run.
                program.top("_Static_assert(1 + 1 == 2, \"arithmetic is broken\");");
                program.top("_Static_assert(-1 < 0, \"signed is not signed\");");
                program.top("enum widths { small = 1, large = 1000 };");
                program.top("_Static_assert(large - small == 999, \"enum arithmetic is broken\");");
                program.blank();
                program.check(Ty::I32, "(int)large", 1000);
            }
        }
        sink.push(Facet::Frontend, Axes::of([("dialect-feature", shape)]), Dialect::C17, program);
    }
}

/// What C23 added, which is what makes this a corpus for a modern compiler.
fn accepted_c23(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "constexpr",
        "nullptr",
        "typeof",
        "auto",
        "bool-keyword",
        "binary-literals",
        "digit-separators",
        "attributes",
        "bit-precise",
        "empty-initializer",
        "enum-fixed-underlying",
        "static-assert-keyword",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::Frontend) {
            return;
        }
        let mut program = Program::new(format!("the C23 {shape} construct"));
        match shape {
            "constexpr" => {
                program.top("constexpr int width = 6;");
                program.top("constexpr int height = 7;");
                program.blank();
                program.line("int values[width * height];");
                program.line("for (int i = 0; i < width * height; i++) {");
                program.line_at(1, "values[i] = i;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "width * height", 42);
                program.check(Ty::I32, "values[41]", 41);
            }
            "nullptr" => {
                program.top("static int held = 12;");
                program.blank();
                program.line("int *at = nullptr;");
                program.line("int found = at == nullptr;");
                program.line("at = &held;");
                program.blank();
                program.check(Ty::I32, "found", 1);
                program.check(Ty::I32, "at != nullptr", 1);
                program.check(Ty::I32, "*at", 12);
            }
            "typeof" => {
                program.input(Ty::I32, "seed", 20);
                program.blank();
                program.line("typeof(seed) same = seed;");
                program.line("typeof_unqual(seed) also = same + 2;");
                program.blank();
                program.check(Ty::I32, "same + also", 42);
            }
            "auto" => {
                program.input(Ty::I32, "seed", 21);
                program.blank();
                program.line("auto doubled = seed * 2;");
                program.blank();
                program.check(Ty::I32, "doubled", 42);
            }
            "bool-keyword" => {
                program.input(Ty::I32, "seed", 1);
                program.blank();
                program.line("bool yes = true;");
                program.line("bool no = false;");
                program.line("bool from_int = seed;");
                program.blank();
                program.check(Ty::I32, "(int)yes", 1);
                program.check(Ty::I32, "(int)no", 0);
                program.check(Ty::I32, "(int)from_int", 1);
                program.check(Ty::I32, "(int)(bool)2", 1);
            }
            "binary-literals" => {
                program.blank();
                program.check(Ty::I32, "0b101010", 42);
                program.check(Ty::I32, "0b1111 & 0b0110", 6);
                program.check(Ty::U32, "0b1u << 4", 16);
            }
            "digit-separators" => {
                program.blank();
                program.check(Ty::I32, "1'000'000 / 1'000", 1000);
                program.check(Ty::I32, "0xff'ff", 65535);
                program.check(Ty::I32, "0b1010'1010", 170);
            }
            "attributes" => {
                program.top("[[nodiscard]] static int compute(int value) { return value * 2; }");
                program.top(
                    "static int ignore([[maybe_unused]] int unused, int used) { return used; }",
                );
                program.blank();
                program.check(Ty::I32, "compute(21)", 42);
                program.check(Ty::I32, "ignore(1, 12)", 12);
            }
            "bit-precise" => {
                program.blank();
                program.line("_BitInt(12) narrow = 2000;");
                program.line("unsigned _BitInt(9) small = 300;");
                program.blank();
                program.check(Ty::I32, "(int)narrow", 2000);
                program.check(Ty::I32, "(int)small", 300);
                program.check(Ty::I32, "(int)(narrow + 47)", 2047);
            }
            "empty-initializer" => {
                program.top("struct bag { int first; int second; int values[4]; };");
                program.blank();
                program.line("struct bag empty = {};");
                program.line("int numbers[4] = {};");
                program.blank();
                program.check(Ty::I32, "empty.first + empty.second", 0);
                program.check(Ty::I32, "empty.values[3]", 0);
                program.check(Ty::I32, "numbers[0] + numbers[1] + numbers[2] + numbers[3]", 0);
            }
            "enum-fixed-underlying" => {
                program.top("enum small : unsigned char { first = 1, last = 200 };");
                program.blank();
                program.check(Ty::I32, "(int)first", 1);
                program.check(Ty::I32, "(int)last", 200);
            }
            _ => {
                // C23 let the message go, and spelt the keyword without the underscore.
                program.top("static_assert(0b1010 == 10);");
                program.top("static_assert(1 + 1 == 2, \"arithmetic is broken\");");
                program.blank();
                program.check(Ty::I32, "0b1010", 10);
            }
        }
        sink.push_tagged(
            Facet::Frontend,
            Axes::of([("dialect-feature", shape)]),
            Dialect::C23,
            program,
            &["modern-c"],
        );
    }
}

/// Programs that must not compile.
///
/// The expected text is a short lowercase fragment rather than a whole diagnostic, and the
/// harness matches it without regard to case. That is on purpose. Requiring the exact wording
/// would mean the corpus goes red every time somebody improves a message, and the thing worth
/// checking is that the compiler caught the right mistake, not how eloquently it said so.
fn rejected(sink: &mut Sink<'_>) {
    const CASES: &[(&str, &str, &str)] = &[
        (
            "duplicate-case",
            "int main(void) {\n    int value = 1;\n    switch (value) {\n    case 1: return 1;\n    case 1: return 2;\n    }\n    return 0;\n}\n",
            "duplicate case",
        ),
        ("undeclared-identifier", "int main(void) {\n    return missing_name;\n}\n", "undeclared"),
        (
            "too-few-arguments",
            "static int two(int a, int b) { return a + b; }\nint main(void) {\n    return two(1);\n}\n",
            "too few arguments",
        ),
        (
            "redefinition",
            "static int value = 1;\nstatic int value = 2;\nint main(void) {\n    return value;\n}\n",
            "redefinition",
        ),
        (
            "failed-static-assert",
            "_Static_assert(1 == 2, \"one is not two\");\nint main(void) {\n    return 0;\n}\n",
            "static assert",
        ),
        (
            "assign-to-const",
            "int main(void) {\n    const int fixed = 1;\n    fixed = 2;\n    return fixed;\n}\n",
            "read-only",
        ),
        (
            "incompatible-return",
            "static int give(void) {\n    struct empty { int value; } made = { 1 };\n    return made;\n}\nint main(void) {\n    return give();\n}\n",
            "incompatible",
        ),
        ("break-outside-loop", "int main(void) {\n    break;\n    return 0;\n}\n", "break"),
    ];
    for &(name, source, mentions) in CASES {
        if !sink.wants(Facet::Frontend) {
            return;
        }
        sink.push_rejected(
            Facet::Frontend,
            Axes::of([("rejected", name)]),
            Dialect::C17,
            source,
            mentions,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, Sink};
    use corpus_model::{Case, Dialect, Expect, Facet};

    fn cases_for(facet: Facet) -> Vec<Case> {
        let opts = Options::all().only(&[facet]);
        let mut sink = Sink::new(&opts);
        super::baseline(&mut sink);
        super::barrier(&mut sink);
        super::frontend(&mut sink);
        sink.into_cases()
    }

    fn output(case: &Case) -> &str {
        match &case.expect {
            Expect::Output(text) => text,
            Expect::Rejected(_) => panic!("{} should run", case.id),
        }
    }

    #[test]
    fn the_baseline_covers_the_ten_things_every_c_compiler_has_to_do() {
        let cases = cases_for(Facet::Baseline);
        assert_eq!(cases.len(), 10);
        let programs: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("program")).collect();
        for wanted in ["arithmetic", "loops", "structs", "recursion", "sort"] {
            assert!(programs.contains(&wanted), "no baseline for {wanted}");
        }
    }

    #[test]
    fn the_sort_baseline_expects_the_numbers_nought_to_seven_in_order() {
        let cases = cases_for(Facet::Baseline);
        let sort = cases.iter().find(|c| c.axes.get("program") == Some("sort")).unwrap();
        assert_eq!(output(sort), "0\n1\n2\n3\n4\n5\n6\n7\n");
    }

    #[test]
    fn every_barrier_case_actually_contains_the_thing_it_is_a_barrier_about() {
        let cases = cases_for(Facet::Barrier);
        assert_eq!(cases.len(), 7);
        for case in &cases {
            let shape = case.axes.get("shape").unwrap();
            if shape.starts_with("volatile") {
                assert!(case.source.contains("volatile"), "{}", case.id);
            }
        }
    }

    #[test]
    fn no_barrier_case_prints_anything_that_depends_on_byte_order() {
        let cases = cases_for(Facet::Barrier);
        let punning = cases.iter().find(|c| c.axes.get("shape") == Some("union-punning")).unwrap();
        // The whole word is only checked for being non zero, never for its value.
        assert!(punning.source.contains("held.whole != 0"));
        assert!(!punning.source.contains("printf(\"%llu\\n\", (unsigned long long)(held.whole))"));
    }

    #[test]
    fn the_c23_cases_are_marked_c23_and_the_c17_ones_are_not() {
        for case in cases_for(Facet::Frontend) {
            let modern = case.has_tag("modern-c");
            assert_eq!(case.dialect == Dialect::C23, modern, "{}", case.id);
        }
    }

    #[test]
    fn the_modern_constructs_that_matter_are_all_covered() {
        let cases = cases_for(Facet::Frontend);
        let features: Vec<&str> = cases
            .iter()
            .filter(|c| c.dialect == Dialect::C23)
            .filter_map(|c| c.axes.get("dialect-feature"))
            .collect();
        for wanted in [
            "constexpr",
            "nullptr",
            "typeof",
            "auto",
            "bool-keyword",
            "binary-literals",
            "digit-separators",
            "attributes",
            "bit-precise",
            "empty-initializer",
        ] {
            assert!(features.contains(&wanted), "no C23 case for {wanted}");
        }
    }

    #[test]
    fn every_rejected_case_names_a_short_fragment_of_the_diagnostic() {
        let cases = cases_for(Facet::Frontend);
        let rejected: Vec<&Case> =
            cases.iter().filter(|c| matches!(c.expect, Expect::Rejected(_))).collect();
        assert!(rejected.len() >= 8);
        for case in rejected {
            let Expect::Rejected(text) = &case.expect else { unreachable!() };
            assert!(!text.is_empty(), "{}", case.id);
            assert_eq!(*text, text.to_lowercase(), "{} expects mixed case", case.id);
            assert!(text.len() < 24, "{} expects too much of the wording", case.id);
        }
    }

    #[test]
    fn the_rejected_cases_are_the_only_ones_that_are_not_supposed_to_compile() {
        for case in cases_for(Facet::Frontend) {
            let rejected = matches!(case.expect, Expect::Rejected(_));
            assert_eq!(case.axes.get("rejected").is_some(), rejected, "{}", case.id);
        }
    }
}
