//! The transformations that need the whole function.
//!
//! These are the cases where the answer depends on a path through the control flow graph
//! rather than on a window of straight line code. They are also where a compiler starts being
//! able to get things wrong in ways that only show up on one path in four, so almost every
//! case here computes its answer along several paths and prints all of them.

use super::{SAMPLES, lit, sample, sample_data};
use crate::Sink;
use crate::emit::Program;
use crate::lang::{Op, Ty, eval};
use corpus_model::{Axes, Dialect, Facet};

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    common_subexpr(sink);
    load_forwarding(sink);
    load_forwarding_walk(sink);
    code_motion(sink);
    copy_propagation(sink);
    constant_propagation(sink);
    value_range(sink);
    value_range_places(sink);
    prune(sink);
    alias_analysis(sink);
    alias_layers(sink);
    memory_ssa(sink);
    scalar_replacement(sink);
}

/// The same expression computed more than once.
///
/// Four shapes, in increasing order of how much analysis it takes to see the redundancy. In a
/// straight line anybody can see it. Across an `if` it needs the expression to be available on
/// both paths. On one path only it is partially redundant and removing it means inserting a
/// copy on the other path. Through a call it needs the callee not to write what the expression
/// reads.
fn common_subexpr(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["straight", "across-if", "partial", "across-call"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::CommonSubexpr) {
                return;
            }
            let mut program =
                Program::new(format!("a repeated {} expression, {shape}", ty.c_name()));
            let name = ty.c_name();
            program.input(ty, "a", 6);
            program.input(ty, "b", 7);
            program.input(Ty::I32, "flag", 1);
            program.blank();
            let Some(product) = eval(Op::Mul, ty, 6, 7) else {
                continue;
            };
            let Some(twice) = eval(Op::Add, ty, product, product) else {
                continue;
            };
            match shape {
                "straight" => {
                    program.line(format!("{name} first = a * b;"));
                    program.line(format!("{name} second = a * b;"));
                    program.line(format!("{name} total = first + second;"));
                }
                "across-if" => {
                    program.line(format!("{name} total = 0;"));
                    program.line("if (flag) {");
                    program.line_at(1, "total = a * b;");
                    program.line("} else {");
                    program.line_at(1, "total = a * b;");
                    program.line("}");
                    program.line("total = total + a * b;");
                }
                "partial" => {
                    program.line(format!("{name} total = a * b;"));
                    program.line("if (flag) {");
                    program.line_at(1, "total = total + a * b;");
                    program.line("} else {");
                    program.line_at(1, format!("total = total + {};", lit(ty, 0)));
                    program.line("}");
                }
                _ => {
                    program.top(format!("static {name} identity({name} value) {{"));
                    program.top("    return value;".to_owned());
                    program.top("}".to_owned());
                    program.line(format!("{name} first = a * b;"));
                    program.line(format!("{name} kept = identity(first);"));
                    program.line(format!("{name} total = kept + a * b;"));
                }
            }
            program.blank();
            program.check(ty.promoted(), "total", twice);
            sink.push(
                Facet::CommonSubexpr,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A load that a store already answered.
///
/// The store and the load are separated by something the compiler has to look through: a
/// second store somewhere else, a branch, or a loop that runs once. What is being checked is
/// that the value read is the value written, whether or not the load survived.
fn load_forwarding(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["same-slot", "other-slot-between", "across-branch", "through-pointer"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::LoadForwarding) {
                return;
            }
            let mut program =
                Program::new(format!("a {} load a store already answered, {shape}", ty.c_name()));
            let name = ty.c_name();
            program.input(ty, "value", 41);
            program.input(Ty::I32, "flag", 1);
            program.blank();
            program.line(format!("{name} slots[4];"));
            match shape {
                "same-slot" => {
                    program.line("slots[0] = value;");
                    program.line(format!("{name} read = slots[0];"));
                }
                "other-slot-between" => {
                    program.line("slots[0] = value;");
                    program.line(format!("slots[2] = value + {};", lit(ty, 100)));
                    program.line(format!("{name} read = slots[0];"));
                }
                "across-branch" => {
                    program.line("slots[0] = value;");
                    program.line("if (flag) {");
                    program.line_at(1, format!("slots[1] = value + {};", lit(ty, 1)));
                    program.line("}");
                    program.line(format!("{name} read = slots[0];"));
                }
                _ => {
                    program.line(format!("{name} *at = &slots[0];"));
                    program.line("*at = value;");
                    program.line(format!("{name} read = *at;"));
                }
            }
            program.blank();
            program.check(ty.promoted(), "read", 41);
            sink.push(
                Facet::LoadForwarding,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The cases that decide how far back a load is allowed to look.
///
/// Forwarding a load means walking backwards over memory until the store that wrote the place
/// being read turns up. The walk is where the reasoning is, and each shape here is one of the
/// things it has to do on the way: translate an address through a join, decide whether a call
/// wrote the place, stop at something it may not cross, and give up when the walk gets too
/// long without giving the wrong answer as it goes.
///
/// Every case prints forty one, the same as the plain shapes, so the whole facet has one
/// answer and a wrong one is obvious in the report rather than needing a table to interpret.
fn load_forwarding_walk(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "phi-translation",
        "across-opaque-call",
        "across-reading-call",
        "volatile-store",
        "long-chain",
        "copied-over",
    ];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::LoadForwarding) {
                return;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("a {} load and a walk that has to {shape}", name));
            program.input(ty, "value", 41);
            program.input(Ty::I32, "flag", 1);
            program.blank();
            match shape {
                "phi-translation" => {
                    // The address of the load is a different expression on each edge, so the
                    // walk has to carry the address back through the join before it can ask
                    // which store wrote it.
                    program.line(format!("{name} slots[4];"));
                    program.line("slots[0] = value;");
                    program.line(format!("slots[1] = value + {};", lit(ty, 1)));
                    program.line(format!("{name} *at;"));
                    program.line("if (flag) {");
                    program.line_at(1, "at = &slots[0];");
                    program.line("} else {");
                    program.line_at(1, "at = &slots[1];");
                    program.line("}");
                    program.line(format!("{name} read = *at;"));
                    program.blank();
                    program.check(ty.promoted(), "read", 41);
                }
                "across-opaque-call" => {
                    // The callee holds a pointer to the slot in a global, so it can write the
                    // place the load reads and the walk has to stop at the call. It adds one,
                    // and the case takes the one back off, so the facet keeps one answer.
                    program.top(format!("static {name} *slot;"));
                    program.top("static void bump(void) {".to_owned());
                    program.top("    *slot += 1;".to_owned());
                    program.top("}".to_owned());
                    program.line(format!("{name} slots[4];"));
                    program.line("slots[0] = value;");
                    program.line("slot = &slots[0];");
                    program.line("bump();");
                    program.line(format!("{name} read = slots[0];"));
                    program.blank();
                    program.check(ty.promoted(), &format!("read - {}", lit(ty, 1)), 41);
                }
                "across-reading-call" => {
                    // The callee touches nothing, so the walk may go straight past it. The
                    // answer is the same either way and the size is what says whether it did.
                    program.top(format!("static {name} doubled({name} of) {{"));
                    program.top("    return of + of;".to_owned());
                    program.top("}".to_owned());
                    program.line(format!("{name} slots[4];"));
                    program.line("slots[0] = value;");
                    program.line(format!("{name} elsewhere = doubled(value);"));
                    program.line("slots[2] = elsewhere;");
                    program.line(format!("{name} read = slots[0];"));
                    program.blank();
                    program.check(ty.promoted(), "read", 41);
                }
                "volatile-store" => {
                    // A volatile store has to happen and a volatile load has to happen, so
                    // nothing here may be forwarded at all. The value is still the value.
                    program.line(format!("volatile {name} slot;"));
                    program.line("slot = value;");
                    program.line(format!("{name} read = slot;"));
                    program.blank();
                    program.check(ty.promoted(), "read", 41);
                }
                "long-chain" => {
                    // Thirty two stores to other places between the store and the load. A
                    // walk with a budget gives up somewhere in here, and giving up has to
                    // mean leaving the load alone rather than answering from the wrong store.
                    program.line(format!("{name} slots[40];"));
                    program.line("slots[0] = value;");
                    for at in 1..33 {
                        program.line(format!("slots[{at}] = value + {};", lit(ty, at)));
                    }
                    program.line(format!("{name} read = slots[0];"));
                    program.blank();
                    program.check(ty.promoted(), "read", 41);
                }
                _ => {
                    // A loop copies one array over another, which is a store the walk cannot
                    // see the address of without knowing what the loop counter did.
                    program.line(format!("{name} source[4];"));
                    program.line(format!("{name} slots[4];"));
                    program.line("for (int i = 0; i < 4; i++) {");
                    program.line_at(1, format!("source[i] = value + ({name})i;"));
                    program.line_at(1, "slots[i] = 0;");
                    program.line("}");
                    program.line("for (int i = 0; i < 4; i++) {");
                    program.line_at(1, "slots[i] = source[i];");
                    program.line("}");
                    program.line(format!("{name} read = slots[0];"));
                    program.blank();
                    program.check(ty.promoted(), "read", 41);
                }
            }
            sink.push(
                Facet::LoadForwarding,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A computation that can move to where it runs less often, or not at all.
///
/// The two directions are hoisting and sinking. Hoisting takes a computation both arms of a
/// branch perform and does it once before the branch. Sinking takes a computation only one
/// arm uses and pushes it into that arm. Both leave the answer alone and both change the
/// instruction count on at least one path, which is what the report measures.
fn code_motion(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["hoist-from-both-arms", "sink-into-one-arm", "hoist-past-store"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::CodeMotion) {
                return;
            }
            let mut program = Program::new(format!("{shape} for {}", ty.c_name()));
            let name = ty.c_name();
            program.input(ty, "a", 4);
            program.input(ty, "b", 9);
            program.input(Ty::I32, "flag", 0);
            program.blank();
            let Some(product) = eval(Op::Mul, ty, 4, 9) else {
                continue;
            };
            let Some(expected) = eval(Op::Add, ty, product, 1) else {
                continue;
            };
            match shape {
                "hoist-from-both-arms" => {
                    program.line(format!("{name} total;"));
                    program.line("if (flag) {");
                    program.line_at(1, format!("total = a * b + {};", lit(ty, 1)));
                    program.line("} else {");
                    program.line_at(1, format!("total = a * b + {};", lit(ty, 1)));
                    program.line("}");
                }
                "sink-into-one-arm" => {
                    program.line(format!("{name} product = a * b;"));
                    program.line(format!("{name} total;"));
                    program.line("if (flag) {");
                    program.line_at(1, format!("total = product + {};", lit(ty, 2)));
                    program.line("} else {");
                    program.line_at(1, format!("total = product + {};", lit(ty, 1)));
                    program.line("}");
                }
                _ => {
                    program.line(format!("{name} scratch[2];"));
                    program.line("scratch[0] = a;");
                    program.line(format!("{name} product = a * b;"));
                    program.line("scratch[1] = b;");
                    program.line(format!("{name} total = product + {};", lit(ty, 1)));
                }
            }
            program.blank();
            program.check(ty.promoted(), "total", expected);
            sink.push(
                Facet::CodeMotion,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A chain of copies that should collapse to nothing.
fn copy_propagation(sink: &mut Sink<'_>) {
    for &ty in Ty::WIDE {
        for depth in [2usize, 8, 32] {
            if !sink.wants(Facet::CopyPropagation) {
                return;
            }
            let mut program = Program::new(format!("a chain of {depth} {} copies", ty.c_name()));
            let name = ty.c_name();
            program.input(ty, "start", 12);
            program.blank();
            program.line(format!("{name} c0 = start;"));
            for step in 1..depth {
                program.line(format!("{name} c{step} = c{};", step - 1));
            }
            program.blank();
            program.check(ty.promoted(), &format!("c{}", depth - 1), 12);
            sink.push(
                Facet::CopyPropagation,
                Axes::of([("type", ty.name()), ("depth", &depth.to_string())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A value that is the same constant however the program got there.
///
/// The last shape is the one that separates a real constant propagation from a peephole. Both
/// arms of the branch assign the same constant, so the value after the join is that constant,
/// but seeing it means reasoning about the join and not about either arm.
fn constant_propagation(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["direct", "through-copy", "same-on-both-arms", "through-array"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::ConstantPropagation) {
                return;
            }
            let mut program =
                Program::new(format!("a known {} value reaching a use, {shape}", ty.c_name()));
            let name = ty.c_name();
            program.input(Ty::I32, "flag", 1);
            program.blank();
            match shape {
                "direct" => {
                    program.line(format!("{name} value = {};", lit(ty, 21)));
                }
                "through-copy" => {
                    program.line(format!("{name} first = {};", lit(ty, 21)));
                    program.line(format!("{name} value = first;"));
                }
                "same-on-both-arms" => {
                    program.line(format!("{name} value;"));
                    program.line("if (flag) {");
                    program.line_at(1, format!("value = {};", lit(ty, 21)));
                    program.line("} else {");
                    program.line_at(1, format!("value = {};", lit(ty, 21)));
                    program.line("}");
                }
                _ => {
                    program.line(format!(
                        "{name} table[3] = {{ {}, {}, {} }};",
                        lit(ty, 21),
                        lit(ty, 22),
                        lit(ty, 23)
                    ));
                    program.line(format!("{name} value = table[0];"));
                }
            }
            program.blank();
            let Some(doubled) = eval(Op::Add, ty, 21, 21) else {
                continue;
            };
            program.check(ty.promoted(), "value + value", doubled);
            sink.push(
                Facet::ConstantPropagation,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Comparisons whose answer a range already decided.
///
/// Each program guards a value into a range and then asks a question the guard has already
/// answered. The answer is printed, so a compiler that gets the range wrong prints the wrong
/// number rather than merely emitting a branch that is never taken.
fn value_range(sink: &mut Sink<'_>) {
    for &ty in Ty::WIDE {
        if !sink.wants(Facet::ValueRange) {
            return;
        }
        let mut program =
            Program::new(format!("questions a guard on a {} already answered", ty.c_name()));
        let name = ty.c_name();
        program.input(ty, "raw", 40);
        program.blank();
        program.line(format!("{name} bounded = raw % {};", lit(ty, 100)));
        program.line(format!("{name} masked = raw & {};", lit(ty, 63)));
        program.blank();
        program.check(Ty::I32, &format!("bounded < {}", lit(ty, 100)), 1);
        program.check(Ty::I32, &format!("masked < {}", lit(ty, 64)), 1);
        program.check(Ty::I32, &format!("masked >= {}", lit(ty, 0)), 1);
        program.blank();
        // The answer is collected into a variable and printed after the branch rather than
        // printed inside each arm. Only one arm runs, so a `printf` in each would mean the
        // program prints one line while the generator recorded two.
        program.line("int guarded;");
        program.line(format!("if (raw > {}) {{", lit(ty, 10)));
        program.line_at(1, format!("guarded = raw > {};", lit(ty, 5)));
        program.line("} else {");
        program.line_at(1, format!("guarded = raw <= {};", lit(ty, 10)));
        program.line("}");
        program.check(Ty::I32, "guarded", 1);
        if ty.signed() {
            program.line("int halved;");
            program.line(format!("if (raw >= {}) {{", lit(ty, 0)));
            program.line_at(1, format!("halved = (raw / {}) >= {};", lit(ty, 2), lit(ty, 0)));
            program.line("} else {");
            program.line_at(1, format!("halved = (raw / {}) <= {};", lit(ty, 2), lit(ty, 0)));
            program.line("}");
            program.check(Ty::I32, "halved", 1);
        }
        sink.push(Facet::ValueRange, Axes::of([("type", ty.name())]), Dialect::C17, program);
    }
}

/// The places a range comes from, and the places it must not be used.
///
/// A range analysis worth having answers about a value at a program point rather than about
/// where the value was defined, and the difference is the whole reason it is a separate
/// analysis rather than a field on the definition. These are the ways a fact arrives at a
/// point: an edge out of a branch, an arm of a switch, a condition that has to be run
/// backwards through the operation that built it, and a relation between two unknowns that no
/// interval can hold.
///
/// The last three shapes are the other half, and they matter more. Each one has a guard that
/// is doing real work, and a compiler that decided the guarded value could not take the value
/// it takes here removes the guard and the program does something undefined. Every case prints
/// one, so the whole facet reads as a column of yes.
fn value_range_places(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "both-edges",
        "switch-arms",
        "inverted-condition",
        "relation-between-two",
        "through-a-cast",
        "carried-round-a-loop",
        "guard-keeps-the-divide",
        "guard-keeps-the-shift",
        "guard-keeps-the-index",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::ValueRange) {
            return;
        }
        let mut program = Program::new(format!("a range that comes from {shape}"));
        match shape {
            "both-edges" => {
                // The fact on the false edge is as much a fact as the one on the true edge,
                // and an analysis that only reads the taken side gets the second answer wrong.
                program.input(Ty::I32, "x", 20);
                program.blank();
                program.line("int settled;");
                program.line("if (x > 10) {");
                program.line_at(1, "settled = (x > 5) && (x >= 11);");
                program.line("} else {");
                program.line_at(1, "settled = (x <= 10) && (x < 11);");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "settled", 1);
            }
            "switch-arms" => {
                // Each arm knows its own value, and the default knows the one thing no arm
                // said, which is the part an analysis built on branches alone does not have.
                program.input(Ty::I32, "pick", 9);
                program.blank();
                program.line("int settled = 0;");
                program.line("switch (pick) {");
                program.line_at(1, "case 1: settled = (pick == 1) && (pick < 2); break;");
                program.line_at(1, "case 2: settled = (pick == 2) && (pick > 1); break;");
                program.line_at(1, "case 3: settled = (pick == 3) && (pick > 2); break;");
                program.line_at(1, "default: settled = (pick != 1) && (pick != 2) && (pick != 3);");
                program.line_at(2, "break;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "settled", 1);
            }
            "inverted-condition" => {
                // The condition is built out of two comparisons and then tested. Getting a
                // range for x out of it means running the `and` backwards, which is the
                // operation rucc bounds with `ranger-logical-depth`.
                program.input(Ty::I32, "x", 20);
                program.input(Ty::I32, "y", 2);
                program.blank();
                program.line("int both = (x > 10) & (y < 5);");
                program.line("int deeper = both & (x < 100);");
                program.line("int settled = 1;");
                program.line("if (deeper) {");
                program.line_at(1, "settled = (x > 5) && (y < 10) && (x < 1000);");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "settled", 1);
            }
            "relation-between-two" => {
                // Neither value is known, so no interval says anything, and the answer is
                // still decided. This is what the relational oracle is for, and it is the one
                // question a range on its own cannot answer at any precision.
                program.input(Ty::I32, "a", 3);
                program.input(Ty::I32, "b", 8);
                program.input(Ty::I32, "c", 11);
                program.blank();
                program.line("int settled = 1;");
                program.line("if (a < b) {");
                program.line_at(1, "if (b < c) {");
                program.line_at(2, "settled = (a < c) && (a != c) && (c > a);");
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "settled", 1);
            }
            "through-a-cast" => {
                // The width changes and the range has to change with it. Truncating to eight
                // bits bounds the value whatever it was, and widening it again is exact.
                program.input(Ty::I32, "x", 1000);
                program.blank();
                program.line("unsigned char narrow = (unsigned char)x;");
                program.line("int wide = (int)narrow;");
                program.line("signed char signed_narrow = (signed char)(x & 63);");
                program.line("int signed_wide = (int)signed_narrow;");
                program.blank();
                program.check(Ty::I32, "wide >= 0 && wide <= 255", 1);
                program.check(Ty::I32, "signed_wide >= 0 && signed_wide <= 63", 1);
            }
            "carried-round-a-loop" => {
                // The counter is bounded by the loop, and the bound holds inside the body and
                // after the exit. rucc has no widening, so what it says about the counter is
                // loose, and this case is here to record that loose is not wrong.
                program.input(Ty::I32, "limit", 10);
                program.blank();
                program.line("int inside = 1;");
                program.line("int i = 0;");
                program.line("for (i = 0; i < limit; i++) {");
                program.line_at(1, "inside = inside && (i >= 0) && (i < limit);");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "inside", 1);
                program.check(Ty::I32, "i >= limit", 1);
            }
            "guard-keeps-the-divide" => {
                // The divisor is zero and the guard is the only thing standing between the
                // program and a machine trap. A compiler that narrowed the divisor to
                // something that excludes zero, or that hoisted the division out of the
                // guard, does not print anything at all here.
                program.input(Ty::I32, "divisor", 0);
                program.input(Ty::I32, "numerator", 84);
                program.blank();
                program.line("int quotient;");
                program.line("if (divisor != 0) {");
                program.line_at(1, "quotient = numerator / divisor;");
                program.line("} else {");
                program.line_at(1, "quotient = 7;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "quotient == 7", 1);
            }
            "guard-keeps-the-shift" => {
                // The count is exactly the width, which is undefined to shift by, so the
                // guard has to survive. Thirty two is not a number an analysis is likely to
                // rule out by accident, which is why the case uses it rather than something
                // further out.
                program.input(Ty::I32, "count", 32);
                program.input(Ty::I32, "x", 1);
                program.blank();
                program.line("int shifted;");
                program.line("if (count < 32) {");
                program.line_at(1, "shifted = x << count;");
                program.line("} else {");
                program.line_at(1, "shifted = 0;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "shifted == 0", 1);
            }
            _ => {
                // The index is one past the end. The guard is what keeps the read inside the
                // array, and an analysis that decided the index was in range removes it.
                program.input(Ty::I32, "index", 4);
                program.blank();
                program.line("int table[4] = { 10, 20, 30, 40 };");
                program.line("int picked;");
                program.line("if (index >= 0 && index < 4) {");
                program.line_at(1, "picked = table[index];");
                program.line("} else {");
                program.line_at(1, "picked = 9;");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "picked == 9", 1);
            }
        }
        sink.push(Facet::ValueRange, Axes::of([("shape", shape)]), Dialect::C17, program);
    }
}

/// One settled test, and everything needed to build a program around it.
struct Pruned {
    /// The axis value, which is also the name of the shape.
    name: &'static str,
    /// One line saying what the compiler has to work out.
    purpose: &'static str,
    /// Declarations that go before `main`.
    top: &'static [&'static str],
    /// The body of the walk, each line with the depth to write it at.
    body: &'static [(usize, &'static str)],
    /// What one sample adds to the outer count and to the inner count.
    counts: fn(i128) -> (i128, i128),
}

/// Every shape the prune facet asks about.
const PRUNED: &[Pruned] = &[
    Pruned {
        name: "greater-then-greater",
        purpose: "a lower bound that settles a weaker lower bound",
        top: &[],
        body: &[
            (1, "if (x > 100) {"),
            (2, "outer++;"),
            (2, "if (x > 50) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x > 100 { (1, i128::from(x > 50)) } else { (0, 0) },
    },
    Pruned {
        name: "greater-then-not-greater",
        purpose: "a lower bound that rules out an upper bound",
        top: &[],
        body: &[
            (1, "if (x > 100) {"),
            (2, "outer++;"),
            (2, "if (x <= 50) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x > 100 { (1, i128::from(x <= 50)) } else { (0, 0) },
    },
    Pruned {
        name: "less-then-less",
        purpose: "an upper bound that settles a weaker upper bound",
        top: &[],
        body: &[
            (1, "if (x < 100) {"),
            (2, "outer++;"),
            (2, "if (x < 200) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x < 100 { (1, i128::from(x < 200)) } else { (0, 0) },
    },
    Pruned {
        name: "less-then-not-less",
        purpose: "an upper bound that rules out a lower bound",
        top: &[],
        body: &[
            (1, "if (x < 100) {"),
            (2, "outer++;"),
            (2, "if (x >= 200) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x < 100 { (1, i128::from(x >= 200)) } else { (0, 0) },
    },
    Pruned {
        name: "at-or-above-then-greater",
        purpose: "a bound that keeps its endpoint settling one that does not",
        top: &[],
        body: &[
            (1, "if (x >= 100) {"),
            (2, "outer++;"),
            (2, "if (x > 50) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x >= 100 { (1, i128::from(x > 50)) } else { (0, 0) },
    },
    Pruned {
        name: "at-or-below-then-at-or-below",
        purpose: "an upper bound at its endpoint settling a weaker one",
        top: &[],
        body: &[
            (1, "if (x <= 100) {"),
            (2, "outer++;"),
            (2, "if (x <= 200) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x <= 100 { (1, i128::from(x <= 200)) } else { (0, 0) },
    },
    Pruned {
        name: "equal-then-unequal",
        purpose: "an equality that settles a later inequality",
        top: &[],
        body: &[
            (1, "if (x == 50) {"),
            (2, "outer++;"),
            (2, "if (x != 49) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x == 50 { (1, i128::from(x != 49)) } else { (0, 0) },
    },
    Pruned {
        name: "equal-then-nonzero",
        purpose: "an equality that settles a later test for zero",
        top: &[],
        body: &[
            (1, "if (x == 50) {"),
            (2, "outer++;"),
            (2, "if (x) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x == 50 { (1, i128::from(x != 0)) } else { (0, 0) },
    },
    Pruned {
        name: "through-a-call",
        purpose: "a settled test on the far side of a call that has to be inlined first",
        top: &["static int above(int v) {", "    return v > 50;", "}"],
        body: &[
            (1, "if (x > 100) {"),
            (2, "outer++;"),
            (2, "if (above(x)) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| if x > 100 { (1, i128::from(x > 50)) } else { (0, 0) },
    },
    Pruned {
        name: "two-values",
        purpose: "a relation between two unknowns settling the same relation written backwards",
        top: &[],
        body: &[
            (1, "int b = (x * 3 + 1) & 255;"),
            (1, "if (x < b) {"),
            (2, "outer++;"),
            (2, "if (b > x) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| {
            let other = (x * 3 + 1) & 255;
            if x < other { (1, i128::from(other > x)) } else { (0, 0) }
        },
    },
    Pruned {
        name: "masked-bound",
        purpose: "a mask that settles a bound on the value it produced",
        top: &[],
        body: &[
            (1, "unsigned int u = (unsigned int)x & 63u;"),
            (1, "outer++;"),
            (1, "if (u < 64u) {"),
            (2, "inner++;"),
            (1, "}"),
        ],
        counts: |x| {
            let masked = x & 63;
            (1, i128::from(masked < 64))
        },
    },
    Pruned {
        name: "switch-unreachable-case",
        purpose: "a switch with a case the mask on its value puts out of reach",
        top: &[],
        body: &[
            (1, "switch (x & 3) {"),
            (2, "case 0: outer++; break;"),
            (2, "case 2: outer++; break;"),
            (2, "case 7: inner++; break;"),
            (2, "default: break;"),
            (1, "}"),
        ],
        counts: |x| (i128::from(matches!(x & 3, 0 | 2)), i128::from(matches!(x & 3, 7))),
    },
    Pruned {
        name: "switch-every-case-unreachable",
        purpose: "a switch the mask on its value reduces to its default",
        top: &[],
        body: &[
            (1, "switch (x & 3) {"),
            (2, "case 4: inner++; break;"),
            (2, "case 5: inner++; break;"),
            (2, "case 9: inner++; break;"),
            (2, "default: outer++; break;"),
            (1, "}"),
        ],
        counts: |x| (1, i128::from(matches!(x & 3, 4 | 5 | 9))),
    },
    Pruned {
        name: "switch-after-a-range-check",
        purpose: "a switch whose value a range check before it has bounded",
        top: &[],
        body: &[
            (1, "int n = x & 15;"),
            (1, "if (n < 4) {"),
            (2, "outer++;"),
            (2, "switch (n) {"),
            (3, "case 0: case 1: case 2: case 3: inner++; break;"),
            (3, "case 9: inner += 100; break;"),
            (3, "default: break;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| {
            let n = x & 15;
            if n >= 4 {
                return (0, 0);
            }
            (1, if n == 9 { 100 } else { 1 })
        },
    },
    Pruned {
        name: "inside-a-loop",
        purpose: "a test inside a loop that the loop bound settles",
        top: &[],
        body: &[
            (1, "int n = x & 15;"),
            (1, "for (int j = 0; j < n; j++) {"),
            (2, "outer++;"),
            (2, "if (j < 16) {"),
            (3, "inner++;"),
            (2, "}"),
            (1, "}"),
        ],
        counts: |x| (x & 15, x & 15),
    },
];

/// A branch or a switch case that a condition above it has already settled.
///
/// Section 24.2's first job. A test whose answer a dominating test already fixed is a branch
/// the compiler can delete, and the arm that cannot run goes with it. What makes this a
/// separate facet rather than a corner of the value range one is that the fact and the
/// question sit in different blocks, so the answer needs a walk up the dominator tree and not
/// just a look at the operands.
///
/// Every case counts two things. The outer count says how many samples reached the guarded
/// region, and the inner count says how many of them ran the arm the guard settled. Both are
/// printed, so a compiler that pruned the wrong side prints the wrong number rather than
/// merely emitting code of a different size. Eight of the fifteen shapes are the plain nested
/// comparison in each of the orders a range can settle, two of them settle to false so the
/// whole inner arm is dead, and the rest are the places the same reasoning has to reach: a
/// call, a pair of unknowns, a mask, three switches and a loop.
fn prune(sink: &mut Sink<'_>) {
    for shape in PRUNED {
        if !sink.wants(Facet::Prune) {
            return;
        }
        let mut program = Program::new(shape.purpose);
        for line in shape.top {
            program.top(*line);
        }
        sample_data(&mut program);
        program.blank();
        program.line("int outer = 0;");
        program.line("int inner = 0;");
        program.line(format!("for (int i = 0; i < {SAMPLES}; i++) {{"));
        program.line_at(1, "int x = data[i];");
        for &(depth, text) in shape.body {
            program.line_at(depth, text);
        }
        program.line("}");
        program.blank();
        let (outer, inner) = tally(shape.counts);
        program.check(Ty::I32, "outer", outer);
        program.check(Ty::I32, "inner", inner);
        sink.push(Facet::Prune, Axes::of([("shape", shape.name)]), Dialect::C17, program);
    }
}

/// Adds up what a shape counts over a full pass of the samples.
fn tally(counts: fn(i128) -> (i128, i128)) -> (i128, i128) {
    (0..SAMPLES).fold((0, 0), |(outer, inner), index| {
        let (reached, settled) = counts(sample(index));
        (outer + reached, inner + settled)
    })
}

/// Two references that either can or cannot name the same object.
///
/// The `restrict` and distinct type shapes are the ones where a wrong answer is a real
/// miscompile rather than a missed optimization, so they get the most operand values. The
/// `may-alias` shape is the control: the two pointers do point at the same object, and a
/// compiler that assumed otherwise prints the wrong number.
fn alias_analysis(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["distinct-locals", "distinct-indices", "restrict", "may-alias"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::AliasAnalysis) {
                return;
            }
            let mut program = Program::new(format!("two {} references, {shape}", ty.c_name()));
            let name = ty.c_name();
            program.input(ty, "seed", 5);
            program.blank();
            match shape {
                "distinct-locals" => {
                    program.line(format!("{name} left = seed;"));
                    program.line(format!("{name} right = seed;"));
                    program.line(format!("{name} *pl = &left, *pr = &right;"));
                    program.line(format!("*pl = seed + {};", lit(ty, 1)));
                    program.line(format!("*pr = seed + {};", lit(ty, 2)));
                    program.line(format!("{name} total = *pl + *pr;"));
                }
                "distinct-indices" => {
                    program.line(format!("{name} buffer[4] = {{ 0 }};"));
                    program.line(format!("buffer[0] = seed + {};", lit(ty, 1)));
                    program.line(format!("buffer[3] = seed + {};", lit(ty, 2)));
                    program.line(format!("{name} total = buffer[0] + buffer[3];"));
                }
                "restrict" => {
                    program.top(format!(
                        "static {name} combine({name} *restrict left, {name} *restrict right) {{"
                    ));
                    program.top("    *left += 1;".to_owned());
                    program.top("    *right += 2;".to_owned());
                    program.top("    return *left + *right;".to_owned());
                    program.top("}".to_owned());
                    program.line(format!("{name} left = seed, right = seed;"));
                    program.line(format!("{name} total = combine(&left, &right);"));
                }
                _ => {
                    program.line(format!("{name} shared = seed;"));
                    program.line(format!("{name} *pl = &shared, *pr = &shared;"));
                    program.line(format!("*pl = seed + {};", lit(ty, 1)));
                    program.line(format!("*pr = seed + {};", lit(ty, 2)));
                    program.line(format!("{name} total = *pl + *pr;"));
                }
            }
            program.blank();
            let expected = if shape == "may-alias" {
                // Both pointers name the same object, so the second store wins and the sum is
                // twice the second value. A compiler that decided the two stores were
                // independent prints the other number and is wrong.
                eval(Op::Add, ty, 7, 7)
            } else {
                eval(Op::Add, ty, 6, 7)
            };
            let Some(expected) = expected else {
                continue;
            };
            program.check(ty.promoted(), "total", expected);
            sink.push(
                Facet::AliasAnalysis,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The layers the answer is allowed to come from, one program each.
///
/// rucc answers the aliasing question in layers, and a layer that is wrong is a
/// miscompilation rather than a missed optimization, so each one gets a program that fails
/// rather than merely getting bigger. Two of these are the promises the language makes to the
/// compiler, which are the type rules and `restrict`. Two are promises the compiler makes back,
/// which are that a `char` pointer may touch anything and that a local whose address left the
/// function is no longer private. The last is a control where the two references really do name
/// the same byte and the only right answer is the conservative one.
fn alias_layers(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["distinct-types", "char-pointer", "union-pun", "computed-same-index", "escaped-local"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::AliasAnalysis) {
                return;
            }
            let name = ty.c_name();
            let bytes = ty.bits() / 8;
            let mut program = Program::new(format!("{shape} references to {name}"));
            program.input(ty, "seed", 5);
            program.input(Ty::I32, "index", 1);
            program.blank();
            let expected = match shape {
                "distinct-types" => {
                    // Two objects of types that no program can make into one another, which
                    // is what the type based layer is allowed to use. The store through the
                    // short cannot reach the other object and the compiler may act on that.
                    program.line(format!("{name} left = seed;"));
                    program.line("short right = 1;");
                    program.line(format!("{name} *pl = &left;"));
                    program.line("short *pr = &right;");
                    program.line(format!("*pl = seed + {};", lit(ty, 1)));
                    program.line("*pr = 2;");
                    program.line(format!("{name} total = *pl + ({name})*pr;"));
                    eval(Op::Add, ty, 6, 2)
                }
                "char-pointer" => {
                    // A pointer to character type may be used to touch any object, so the
                    // byte writes below are visible to the load that follows. Every byte is
                    // written with the same value, so the answer does not depend on which end
                    // of the object the machine puts first.
                    program.line(format!("{name} object = seed;"));
                    program.line(format!("{name} *at = &object;"));
                    program.line("unsigned char *bytes = (unsigned char *)&object;");
                    program.line(format!("*at = seed + {};", lit(ty, 1)));
                    program.line(format!("for (int i = 0; i < {bytes}; i++) {{"));
                    program.line_at(1, "bytes[i] = 0;");
                    program.line("}");
                    program.line(format!("{name} total = *at + {};", lit(ty, 7)));
                    Some(7)
                }
                "union-pun" => {
                    // Storing one member and reading another is type punning, which GCC
                    // defines and which the type based layer has to stop short of. Reading
                    // the whole after zeroing the bytes has to see the zeros.
                    program.top(format!(
                        "union both {{ {name} whole; unsigned char parts[{bytes}]; }};"
                    ));
                    program.line("union both holder;");
                    program.line(format!("holder.whole = seed + {};", lit(ty, 1)));
                    program.line(format!("for (int i = 0; i < {bytes}; i++) {{"));
                    program.line_at(1, "holder.parts[i] = 0;");
                    program.line("}");
                    program.line(format!("{name} total = holder.whole + {};", lit(ty, 7)));
                    Some(7)
                }
                "computed-same-index" => {
                    // The two indices are equal and neither is a constant, so the offsets
                    // cannot be told apart and the second store has to be assumed to land on
                    // the first. A compiler that guessed otherwise prints the other number.
                    program.line(format!("{name} buffer[4] = {{ 0 }};"));
                    program.line("int here = index;");
                    program.line("int there = index * 3 - 2;");
                    program.line(format!("buffer[here] = seed + {};", lit(ty, 1)));
                    program.line(format!("buffer[there] = seed + {};", lit(ty, 2)));
                    program.line(format!("{name} total = buffer[here] + buffer[there];"));
                    eval(Op::Add, ty, 7, 7)
                }
                _ => {
                    // The address of the local reaches a global, so anything that runs after
                    // that may write it and the compiler may not keep the local in a register
                    // across the call.
                    program.top(format!("static {name} *kept;"));
                    program.top(format!("static void keep({name} *of) {{"));
                    program.top("    kept = of;".to_owned());
                    program.top("}".to_owned());
                    program.top("static void overwrite(void) {".to_owned());
                    program.top("    *kept = *kept + 2;".to_owned());
                    program.top("}".to_owned());
                    program.line(format!("{name} local = seed;"));
                    program.line("keep(&local);");
                    program.line(format!("local = seed + {};", lit(ty, 1)));
                    program.line("overwrite();");
                    program.line(format!("{name} total = local;"));
                    eval(Op::Add, ty, 6, 2)
                }
            };
            let Some(expected) = expected else {
                continue;
            };
            program.blank();
            program.check(ty.promoted(), "total", expected);
            sink.push(
                Facet::AliasAnalysis,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Aggregates and address taken locals that never escape.
///
/// Each of these can be split into plain values, because nothing outside the function can see
/// the memory. The answer is what is checked, and the size of the frame is what the report
/// notices, since a compiler that failed to split still has to allocate the whole struct.
fn scalar_replacement(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["struct", "nested-struct", "array-constant-index", "address-taken", "union"];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::ScalarReplacement) {
                return;
            }
            let mut program =
                Program::new(format!("a non-escaping {} aggregate, {shape}", ty.c_name()));
            let name = ty.c_name();
            program.input(ty, "seed", 8);
            program.blank();
            match shape {
                "struct" => {
                    program.top(format!("struct pair {{ {name} first; {name} second; }};"));
                    program.line("struct pair p;");
                    program.line("p.first = seed;");
                    program.line(format!("p.second = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} total = p.first + p.second;"));
                }
                "nested-struct" => {
                    program.top(format!("struct inner {{ {name} value; }};"));
                    program.top("struct outer { struct inner one; struct inner two; };".to_owned());
                    program.line("struct outer o;");
                    program.line("o.one.value = seed;");
                    program.line(format!("o.two.value = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} total = o.one.value + o.two.value;"));
                }
                "array-constant-index" => {
                    program.line(format!("{name} slots[8];"));
                    program.line("slots[2] = seed;");
                    program.line(format!("slots[5] = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} total = slots[2] + slots[5];"));
                }
                "address-taken" => {
                    program.line(format!("{name} one = seed;"));
                    program.line(format!("{name} two = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} *at = &one;"));
                    program.line("*at = one;");
                    program.line(format!("{name} total = *at + two;"));
                }
                _ => {
                    program.top(format!("union holder {{ {name} value; {name} again; }};"));
                    program.line("union holder u;");
                    program.line("u.value = seed;");
                    program.line(format!("{name} total = u.again + seed + {};", lit(ty, 1)));
                }
            }
            program.blank();
            let Some(expected) = eval(Op::Add, ty, 8, 9) else {
                continue;
            };
            program.check(ty.promoted(), "total", expected);
            sink.push(
                Facet::ScalarReplacement,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A load, and everything between it and the store that answers it.
///
/// Alias analysis answers a question about two references. Memory SSA is the thing that knows
/// which question to ask, because it walks back from a load through the writes before it and
/// stops at the first one that could matter. The two halves fail differently. A walk that
/// stops too early gives up on a load it could have answered, which costs speed. A walk that
/// walks past a write that did matter reads a stale value, which is a miscompilation, and
/// every shape below has an answer that changes if that happens.
fn memory_ssa(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "skip-other-object",
        "skip-other-index",
        "stopped-by-call",
        "stopped-by-may-alias",
        "merge-at-join",
        "over-a-loop",
        "clobbered-in-a-loop",
    ];
    for &ty in Ty::WIDE {
        for &shape in SHAPES {
            if !sink.wants(Facet::MemorySsa) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("the walk back from a {name} load, {shape}"));
            program.input(ty, "value", 41);
            program.input(Ty::I32, "flag", 1);
            program.blank();
            let Some(next) = eval(Op::Add, ty, 41, 1) else {
                continue;
            };
            let expected = match shape {
                "skip-other-object" => {
                    // Two locals, so no walk that knows what a local is should stop here.
                    program.line(format!("{name} one[2];"));
                    program.line(format!("{name} two[2];"));
                    program.line("one[0] = value;");
                    program.line(format!("two[0] = value + {};", lit(ty, 1)));
                    program.line(format!("{name} read = one[0];"));
                    41
                }
                "skip-other-index" => {
                    program.line(format!("{name} one[2];"));
                    program.line("one[0] = value;");
                    program.line(format!("one[1] = value + {};", lit(ty, 1)));
                    program.line(format!("{name} read = one[0];"));
                    41
                }
                "stopped-by-call" => {
                    // The callee is given the address, so the walk has to stop at the call
                    // whatever it knows about the object.
                    program.top(format!("static void put({name} *at, {name} v) {{"));
                    program.top("    *at = v;".to_owned());
                    program.top("}".to_owned());
                    program.line(format!("{name} one[2];"));
                    program.line("one[0] = value;");
                    program.line(format!("put(&one[0], value + {});", lit(ty, 1)));
                    program.line(format!("{name} read = one[0];"));
                    next
                }
                "stopped-by-may-alias" => {
                    program.line(format!("{name} one[2];"));
                    program.line(format!("{name} two[2];"));
                    program.line("one[0] = value;");
                    program.line("two[0] = value;");
                    program.line(format!("{name} *at = flag ? &one[0] : &two[0];"));
                    program.line(format!("*at = value + {};", lit(ty, 1)));
                    program.line(format!("{name} read = one[0];"));
                    next
                }
                "merge-at-join" => {
                    // The load is answered by two stores at once, which is the case a walk
                    // that only ever follows one predecessor gets wrong.
                    program.line(format!("{name} one[2];"));
                    program.line("if (flag) {");
                    program.line_at(1, format!("one[0] = value + {};", lit(ty, 1)));
                    program.line("} else {");
                    program.line_at(1, "one[0] = value;");
                    program.line("}");
                    program.line(format!("{name} read = one[0];"));
                    next
                }
                "over-a-loop" => {
                    program.line(format!("{name} one[2];"));
                    program.line(format!("{name} two[2];"));
                    program.line("one[0] = value;");
                    program.line("for (int i = 0; i < 4; i++) {");
                    program.line_at(1, format!("two[i & 1] = ({name})i;"));
                    program.line("}");
                    program.line(format!("{name} read = one[0];"));
                    41
                }
                _ => {
                    program.line(format!("{name} one[2];"));
                    program.line("one[0] = value;");
                    program.line("for (int i = 0; i < 1; i++) {");
                    program.line_at(1, format!("one[0] = one[0] + {};", lit(ty, 1)));
                    program.line("}");
                    program.line(format!("{name} read = one[0];"));
                    next
                }
            };
            program.blank();
            program.check(ty.promoted(), "read", expected);
            sink.push(
                Facet::MemorySsa,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
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

    #[test]
    fn the_walk_back_from_a_load_reads_what_the_last_store_wrote() {
        let cases = cases_for(Facet::MemorySsa);
        assert!(!cases.is_empty());
        for case in &cases {
            let Expect::Output(text) = &case.expect else { panic!("{} should run", case.id) };
            assert!(
                text == "41\n" || text == "42\n",
                "{} expects {text:?}, which is neither the old value nor the new one",
                case.id
            );
        }
    }

    #[test]
    fn the_memory_ssa_shapes_that_must_stop_the_walk_are_all_present() {
        let cases = cases_for(Facet::MemorySsa);
        let shapes: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("shape")).collect();
        for wanted in ["stopped-by-call", "stopped-by-may-alias", "merge-at-join"] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    #[test]
    fn every_common_subexpression_shape_computes_the_same_total() {
        let cases = cases_for(Facet::CommonSubexpr);
        assert!(!cases.is_empty());
        for case in &cases {
            assert_eq!(case.expect, Expect::Output("84\n".to_owned()), "{}", case.id);
        }
    }

    #[test]
    fn every_load_forwarding_shape_reads_back_the_value_that_was_written() {
        for case in cases_for(Facet::LoadForwarding) {
            assert_eq!(case.expect, Expect::Output("41\n".to_owned()), "{}", case.id);
        }
    }

    #[test]
    fn a_copy_chain_of_any_depth_ends_where_it_started() {
        let cases = cases_for(Facet::CopyPropagation);
        for case in &cases {
            assert_eq!(case.expect, Expect::Output("12\n".to_owned()), "{}", case.id);
        }
        let deep = cases.iter().find(|c| c.axes.get("depth") == Some("32")).unwrap();
        assert!(deep.source.contains("c31 = c30;"));
    }

    #[test]
    fn the_may_alias_case_expects_the_answer_you_get_when_the_pointers_do_alias() {
        let cases = cases_for(Facet::AliasAnalysis);
        let aliasing = cases.iter().find(|c| c.axes.get("shape") == Some("may-alias")).unwrap();
        let distinct =
            cases.iter().find(|c| c.axes.get("shape") == Some("distinct-locals")).unwrap();
        assert_eq!(aliasing.expect, Expect::Output("14\n".to_owned()));
        assert_eq!(distinct.expect, Expect::Output("13\n".to_owned()));
    }

    #[test]
    fn the_restrict_case_actually_says_restrict() {
        let cases = cases_for(Facet::AliasAnalysis);
        let case = cases.iter().find(|c| c.axes.get("shape") == Some("restrict")).unwrap();
        assert!(case.source.contains("restrict"), "{}", case.source);
    }

    #[test]
    fn every_scalar_replacement_shape_adds_up_to_seventeen() {
        let cases = cases_for(Facet::ScalarReplacement);
        assert!(cases.len() >= 15);
        for case in &cases {
            assert_eq!(case.expect, Expect::Output("17\n".to_owned()), "{}", case.id);
        }
    }

    #[test]
    fn every_range_question_has_the_answer_yes() {
        for case in cases_for(Facet::ValueRange) {
            let Expect::Output(text) = &case.expect else { panic!("{} should run", case.id) };
            assert!(text.lines().all(|line| line == "1"), "{} expects {text:?}", case.id);
        }
    }

    #[test]
    fn constant_propagation_reaches_the_same_answer_by_four_routes() {
        let cases = cases_for(Facet::ConstantPropagation);
        for case in &cases {
            assert_eq!(case.expect, Expect::Output("42\n".to_owned()), "{}", case.id);
        }
        let shapes: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("shape")).collect();
        for wanted in ["direct", "through-copy", "same-on-both-arms", "through-array"] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    /// The outer and inner counts one prune shape prints, as the numbers they are.
    fn prune_counts(cases: &[Case], shape: &str) -> (i64, i64) {
        let case = cases
            .iter()
            .find(|case| case.axes.get("shape") == Some(shape))
            .unwrap_or_else(|| panic!("no prune case for {shape}"));
        let Expect::Output(text) = &case.expect else { panic!("{} should run", case.id) };
        let counts: Vec<i64> = text.lines().map(|line| line.parse().expect("a count")).collect();
        assert_eq!(counts.len(), 2, "{} prints {text:?}", case.id);
        (counts[0], counts[1])
    }

    #[test]
    fn every_prune_shape_the_pass_has_to_decide_is_generated() {
        let cases = cases_for(Facet::Prune);
        let shapes: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("shape")).collect();
        for wanted in [
            "greater-then-greater",
            "greater-then-not-greater",
            "less-then-less",
            "less-then-not-less",
            "at-or-above-then-greater",
            "at-or-below-then-at-or-below",
            "equal-then-unequal",
            "equal-then-nonzero",
            "through-a-call",
            "two-values",
            "masked-bound",
            "switch-unreachable-case",
            "switch-every-case-unreachable",
            "switch-after-a-range-check",
            "inside-a-loop",
        ] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
        assert_eq!(shapes.len(), cases.len());
    }

    #[test]
    fn a_shape_the_guard_settles_runs_its_inner_arm_every_time_the_guard_lets_a_sample_through() {
        let cases = cases_for(Facet::Prune);
        for shape in [
            "greater-then-greater",
            "less-then-less",
            "at-or-above-then-greater",
            "at-or-below-then-at-or-below",
            "equal-then-unequal",
            "equal-then-nonzero",
            "through-a-call",
            "two-values",
            "masked-bound",
            "switch-after-a-range-check",
            "inside-a-loop",
        ] {
            let (outer, inner) = prune_counts(&cases, shape);
            assert_eq!(outer, inner, "{shape} counts");
            assert!(outer > 0, "{shape} never reaches its guarded region");
        }
    }

    #[test]
    fn a_shape_the_guard_rules_out_never_runs_its_inner_arm() {
        let cases = cases_for(Facet::Prune);
        for shape in [
            "greater-then-not-greater",
            "less-then-not-less",
            "switch-unreachable-case",
            "switch-every-case-unreachable",
        ] {
            let (outer, inner) = prune_counts(&cases, shape);
            assert_eq!(inner, 0, "{shape} runs an arm it cannot reach");
            assert!(outer > 0, "{shape} never reaches its guarded region");
        }
    }

    #[test]
    fn the_counts_are_the_ones_a_full_pass_over_the_byte_range_produces() {
        let cases = cases_for(Facet::Prune);
        assert_eq!(prune_counts(&cases, "greater-then-greater"), (155, 155));
        assert_eq!(prune_counts(&cases, "less-then-less"), (100, 100));
        assert_eq!(prune_counts(&cases, "at-or-above-then-greater"), (156, 156));
        assert_eq!(prune_counts(&cases, "at-or-below-then-at-or-below"), (101, 101));
        assert_eq!(prune_counts(&cases, "equal-then-unequal"), (1, 1));
        assert_eq!(prune_counts(&cases, "two-values"), (128, 128));
        assert_eq!(prune_counts(&cases, "masked-bound"), (256, 256));
        assert_eq!(prune_counts(&cases, "switch-unreachable-case"), (128, 0));
        assert_eq!(prune_counts(&cases, "switch-every-case-unreachable"), (256, 0));
        assert_eq!(prune_counts(&cases, "switch-after-a-range-check"), (64, 64));
        assert_eq!(prune_counts(&cases, "inside-a-loop"), (1920, 1920));
    }

    #[test]
    fn no_prune_case_gets_the_value_it_tests_from_a_literal() {
        for case in cases_for(Facet::Prune) {
            assert!(case.source.contains("static volatile int seed_in"), "{}", case.id);
            assert!(case.source.contains("data[i] = (i * 37 + seed) & 255;"), "{}", case.id);
        }
    }

    #[test]
    fn the_shape_that_needs_inlining_first_is_the_only_one_with_a_helper() {
        let cases = cases_for(Facet::Prune);
        let with_helper: Vec<&str> = cases
            .iter()
            .filter(|case| case.source.contains("static int above("))
            .filter_map(|case| case.axes.get("shape"))
            .collect();
        assert_eq!(with_helper, ["through-a-call"]);
    }
}
