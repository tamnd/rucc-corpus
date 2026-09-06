//! Everything below the machine independent IR.
//!
//! These cases are the ones where the report matters more than the pass or fail. Almost any
//! compiler gets the right answer for a switch on eight consecutive values. What separates
//! them is whether it emitted a jump table, a chain of comparisons or a binary search, and
//! that shows up in the size and the time rather than in the output.
//!
//! Every case here is still self checking, because a backend that gets the right shape and
//! the wrong answer is the worst outcome of all and the easiest one to miss.

use super::lit;
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// The types used here.
const TYPES: &[Ty] = Ty::WIDE;

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    selection(sink);
    register_pressure(sink);
    register_alloc(sink);
    scheduling(sink);
    block_layout(sink);
    if_conversion(sink);
    switch_lowering(sink);
    calling_convention(sink);
    machine_peephole(sink);
}

/// Expressions that a target usually has one instruction for.
///
/// Each of these is several operations in C and one instruction on at least one of the targets
/// rucc cares about. A compiler that emits the arithmetic literally is correct and slow, which
/// is exactly the distinction the report exists to draw.
fn selection(sink: &mut Sink<'_>) {
    const PATTERNS: &[(&str, &str)] = &[
        ("multiply-add", "a * b + c"),
        ("shift-add", "(a << 3) + b"),
        ("negate-add", "c - a"),
        ("min", "a < b ? a : b"),
        ("max", "a > b ? a : b"),
        ("compare-set", "(a < b) + (b < c)"),
        ("mask-low", "a & 0xff"),
        ("test-bit", "(a >> 2) & 1"),
        ("round-down", "a & ~(unsigned)7"),
        ("average", "(a >> 1) + (b >> 1)"),
    ];
    for &ty in TYPES {
        for &(name, expr) in PATTERNS {
            if !sink.wants(Facet::Selection) {
                return;
            }
            // Every pattern is evaluated here in the same integer arithmetic the program will
            // use, with operands small enough that nothing overflows for any of the four
            // types. Small operands are a real limitation of this facet and the reason the
            // constant folding facet carries the boundary values instead.
            let (a, b, c) = (20i128, 12i128, 100i128);
            let value = match name {
                "multiply-add" => a * b + c,
                "shift-add" => (a << 3) + b,
                "negate-add" => c - a,
                "min" => a.min(b),
                "max" => a.max(b),
                "compare-set" => i128::from(a < b) + i128::from(b < c),
                "mask-low" => a & 0xff,
                "test-bit" => (a >> 2) & 1,
                "round-down" => a & !7,
                "average" => (a >> 1) + (b >> 1),
                _ => continue,
            };
            if !ty.promoted().holds(value) {
                continue;
            }
            let mut program = Program::new(format!("the {name} pattern on {}", ty.c_name()));
            program.input(ty, "a", a);
            program.input(ty, "b", b);
            program.input(ty, "c", c);
            program.blank();
            program.check(ty.promoted(), expr, value);
            sink.push(
                Facet::Selection,
                Axes::of([("type", ty.name()), ("pattern", name)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// How many values are live at once, and what the compiler does about it.
///
/// The allocator facet next door asks whether the answer survives spilling. This one is about
/// the count itself, because in SSA the number of values live at a point is exactly the number
/// of registers needed there rather than an estimate of it, and four different decisions are
/// made from that number. So each shape here is one of those decisions.
///
/// `across-a-call` holds values over a call, which means they have to live in callee saved
/// registers or on the stack, and there are far fewer of the former than of the whole file.
/// `through-a-loop` holds them across a loop, where a spill is paid once per iteration rather
/// than once. `both-classes` holds integers and doubles at the same time, which is two banks
/// of registers and has to be counted as two numbers rather than one. `one-arm-only` holds
/// them across one arm of a branch, where a count taken over the whole function says the
/// pressure is high everywhere and a count taken per point says it is high in one place.
fn register_pressure(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["across-a-call", "through-a-loop", "both-classes", "one-arm-only"];
    for &ty in TYPES {
        for &live in &[6usize, 12, 24] {
            for &shape in SHAPES {
                if !sink.wants(Facet::RegisterPressure) {
                    return;
                }
                let name = ty.c_name();
                let mut program = Program::new(format!(
                    "{live} live {name} values held {shape}, which is what the pressure model counts"
                ));
                if shape == "across-a-call" {
                    program.top(format!("static {name} opaque({name} v) {{"));
                    program.top("    return v + 1;".to_owned());
                    program.top("}".to_owned());
                }
                program.input(ty, "seed", 1);
                program.input(Ty::I32, "flag", 1);
                program.blank();
                for at in 0..live {
                    program.line(format!("{name} v{at} = seed + {};", lit(ty, at as i128)));
                }
                program.blank();

                // Every value is defined above and read below, so all of them are live over
                // whatever goes in between, which is the thing each shape varies.
                let mut total: i128 = (0..live as i128).map(|at| 1 + at).sum();
                match shape {
                    "across-a-call" => {
                        program.line(format!("{name} kept = opaque(seed);"));
                        total += 2;
                    }
                    "through-a-loop" => {
                        program.line(format!("{name} kept = 0;"));
                        program.line("for (int i = 0; i < 4; i++) {");
                        program.line_at(1, "kept = kept + 1;");
                        program.line("}");
                        total += 4;
                    }
                    "both-classes" => {
                        // A double alongside the integers, because they are two register files
                        // and a model that adds them up gets both numbers wrong.
                        program.line("double d0 = 1.0, d1 = 2.0, d2 = 3.0, d3 = 4.0;");
                        program.line("double dsum = d0 + d1 + d2 + d3;");
                        program.line(format!("{name} kept = ({name})dsum;"));
                        total += 10;
                    }
                    _ => {
                        program.line(format!("{name} kept = 0;"));
                        program.line("if (flag) {");
                        program.line_at(1, format!("kept = ({name})1;"));
                        program.line("} else {");
                        program.line_at(1, format!("kept = ({name})2;"));
                        program.line("}");
                        total += 1;
                    }
                }
                program.blank();

                // Read back to front, so the value defined first is the one live longest.
                program.line(format!("{name} total = kept;"));
                for at in (0..live).rev() {
                    program.line(format!("total += v{at};"));
                }
                if !ty.promoted().holds(total) {
                    continue;
                }
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::RegisterPressure,
                    Axes::of([("type", ty.name()), ("live", &live.to_string()), ("shape", shape)]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// More values live at once than the machine has registers.
///
/// The pressure axis walks from comfortably inside any register file to well past every one
/// of them. What is being checked is that the answer survives spilling, and what is being
/// measured is how much the spill cost. The interleaved use pattern is deliberate: values are
/// touched in an order that defeats the simplest possible spill heuristic, which is to evict
/// whatever was defined longest ago.
fn register_alloc(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &live in &[4usize, 8, 16, 32, 64] {
            for &shape in &["straight", "in-a-loop"] {
                if !sink.wants(Facet::RegisterAlloc) {
                    return;
                }
                let name = ty.c_name();
                let mut program =
                    Program::new(format!("{live} live {} values at once, {shape}", ty.c_name()));
                program.input(ty, "seed", 1);
                program.blank();
                for at in 0..live {
                    program.line(format!("{name} v{at} = seed + {};", lit(ty, at as i128)));
                }
                program.blank();
                let mut total: i128 = (0..live as i128).map(|at| 1 + at).sum();
                if shape == "in-a-loop" {
                    program.line(format!("{name} total = 0;"));
                    program.line("for (int i = 0; i < 4; i++) {");
                    for at in (0..live).rev() {
                        program.line_at(1, format!("total += v{at};"));
                    }
                    program.line("}");
                    total *= 4;
                } else {
                    program.line(format!("{name} total = 0;"));
                    // Touched back to front, so the value defined first is needed last.
                    for at in (0..live).rev() {
                        program.line(format!("total += v{at};"));
                    }
                }
                if !ty.promoted().holds(total) {
                    continue;
                }
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::RegisterAlloc,
                    Axes::of([("type", ty.name()), ("live", &live.to_string()), ("shape", shape)]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// The same work as one long chain and as several short ones.
///
/// A chain of dependent multiplies can only issue one at a time. The same number of multiplies
/// split into four independent chains can issue four at a time on any machine with the
/// execution units for it. Nothing about the answer changes, and on a wide machine the time
/// changes by close to the number of chains, which makes this the clearest performance case
/// in the corpus.
fn scheduling(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &chains in &[1usize, 2, 4] {
            for &length in &[8usize, 32] {
                if !sink.wants(Facet::Scheduling) {
                    return;
                }
                let name = ty.c_name();
                let mut program = Program::new(format!(
                    "{chains} independent {} chains of {length} steps",
                    ty.c_name()
                ));
                program.input(ty, "seed", 1);
                program.blank();
                for chain in 0..chains {
                    program.line(format!("{name} c{chain} = seed + {};", lit(ty, chain as i128)));
                }
                program.line("for (int i = 0; i < 4; i++) {");
                for _ in 0..length {
                    for chain in 0..chains {
                        program.line_at(
                            1,
                            format!("c{chain} = c{chain} * {} + {};", lit(ty, 3), lit(ty, 1)),
                        );
                    }
                }
                program.line("}");
                program.blank();

                // The chains are stepped in unsigned arithmetic on purpose. Thirty two steps
                // of multiply by three overflows every one of these types many times over,
                // and wrapping is only defined for the unsigned ones.
                if ty.signed() {
                    continue;
                }
                let mut total: i128 = 0;
                for chain in 0..chains as i128 {
                    let mut value = 1 + chain;
                    for _ in 0..(4 * length) {
                        value = ty.promoted().convert(value * 3 + 1);
                    }
                    total = ty.promoted().convert(total + value);
                }
                program.line(format!("{name} total = 0;"));
                for chain in 0..chains {
                    program.line(format!("total += c{chain};"));
                }
                program.blank();
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::Scheduling,
                    Axes::of([
                        ("type", ty.name()),
                        ("chains", &chains.to_string()),
                        ("length", &length.to_string()),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// Branches that almost always go one way.
///
/// The condition is read from a volatile global so it cannot be folded, but the value is such
/// that one side runs on every iteration and the other never does. A compiler that lays the
/// blocks out so the common path falls through pays nothing. One that puts the cold block in
/// the middle pays a taken branch every time round.
fn block_layout(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["rare-then", "rare-else", "early-exit", "chain-of-tests"];
    for &shape in SHAPES {
        if !sink.wants(Facet::BlockLayout) {
            return;
        }
        let mut program = Program::new(format!("a {shape} branch inside a hot loop"));
        program.input(Ty::I32, "rare", 0);
        program.blank();
        program.line("long long total = 0;");
        program.line("for (int i = 0; i < 1000; i++) {");
        let total: i128 = match shape {
            "rare-then" => {
                program.line_at(1, "if (rare) {");
                program.line_at(2, "total += 1000;");
                program.line_at(1, "} else {");
                program.line_at(2, "total += 1;");
                program.line_at(1, "}");
                1000
            }
            "rare-else" => {
                program.line_at(1, "if (!rare) {");
                program.line_at(2, "total += 1;");
                program.line_at(1, "} else {");
                program.line_at(2, "total += 1000;");
                program.line_at(1, "}");
                1000
            }
            "early-exit" => {
                program.line_at(1, "if (rare) {");
                program.line_at(2, "break;");
                program.line_at(1, "}");
                program.line_at(1, "total += 1;");
                1000
            }
            _ => {
                program.line_at(1, "if (rare == 1) {");
                program.line_at(2, "total += 1000;");
                program.line_at(1, "} else if (rare == 2) {");
                program.line_at(2, "total += 2000;");
                program.line_at(1, "} else if (rare == 3) {");
                program.line_at(2, "total += 3000;");
                program.line_at(1, "} else {");
                program.line_at(2, "total += 1;");
                program.line_at(1, "}");
                1000
            }
        };
        program.line("}");
        program.blank();
        program.check(Ty::I64, "total", total);
        sink.push(Facet::BlockLayout, Axes::of([("shape", shape)]), Dialect::C17, program);
    }
}

/// Short branches that could be a conditional move instead.
///
/// The condition alternates every iteration, so a branch here is mispredicted about half the
/// time and branchless code wins by a lot. That is the case where if conversion is obviously
/// right. The corpus does not have a case where it is obviously wrong, because that needs a
/// predictable condition and a long branch, which is the `block-layout` facet next door.
fn if_conversion(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["select", "min", "conditional-add", "abs", "clamp"];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::IfConversion) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!(
                "a {shape} on {} with an unpredictable condition",
                ty.c_name()
            ));
            program.input(ty, "a", 20);
            program.input(ty, "b", 12);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 1000; i++) {");
            program.line_at(1, "int odd = i & 1;");
            let total: i128 = match shape {
                "select" => {
                    program.line_at(1, "total += odd ? a : b;");
                    500 * 20 + 500 * 12
                }
                "min" => {
                    program.line_at(1, format!("{name} left = odd ? a : b;"));
                    program.line_at(1, "total += left < b ? left : b;");
                    1000 * 12
                }
                "conditional-add" => {
                    program.line_at(1, "if (odd) {");
                    program.line_at(2, "total += a;");
                    program.line_at(1, "}");
                    500 * 20
                }
                "abs" => {
                    program.line_at(1, format!("{name} shifted = odd ? a : b;"));
                    program.line_at(
                        1,
                        format!(
                            "total += shifted > {} ? shifted : {} - shifted;",
                            lit(ty, 0),
                            lit(ty, 0)
                        ),
                    );
                    500 * 20 + 500 * 12
                }
                _ => {
                    program.line_at(1, format!("{name} raw = odd ? a : b;"));
                    program.line_at(
                        1,
                        format!("{name} low = raw < {} ? {} : raw;", lit(ty, 15), lit(ty, 15)),
                    );
                    program.line_at(
                        1,
                        format!("total += low > {} ? {} : low;", lit(ty, 18), lit(ty, 18)),
                    );
                    // Odd iterations clamp twenty down to eighteen, even ones clamp twelve
                    // up to fifteen, and neither bound is hit by both.
                    500 * 18 + 500 * 15
                }
            };
            program.line("}");
            program.blank();
            if !ty.promoted().holds(total) {
                continue;
            }
            program.check(ty.promoted(), "total", total);
            sink.push(
                Facet::IfConversion,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Switches of every density and size.
///
/// Density is the axis that decides the lowering. Consecutive labels become a jump table.
/// Labels spread over a wide range become a binary search or a chain. Three labels become
/// three comparisons whatever their spacing. Every case walks every label and adds up what it
/// got, so a switch that falls into the wrong arm for one input out of forty is caught.
fn switch_lowering(sink: &mut Sink<'_>) {
    const DENSITIES: &[(&str, i128)] = &[("dense", 1), ("sparse", 17), ("very-sparse", 1000)];
    for &(density, stride) in DENSITIES {
        for &labels in &[3i128, 8, 40] {
            if !sink.wants(Facet::SwitchLowering) {
                return;
            }
            let mut program = Program::new(format!("a {density} switch with {labels} labels"));
            program.top("static int classify(int value) {".to_owned());
            program.top("    switch (value) {".to_owned());
            for label in 0..labels {
                program.top(format!("    case {}: return {};", label * stride, label + 1));
            }
            program.top("    default: return 0;".to_owned());
            program.top("    }".to_owned());
            program.top("}".to_owned());
            program.input(Ty::I32, "step", stride);
            program.blank();
            program.line("long long total = 0;");
            program.line(format!("for (int label = 0; label < {labels}; label++) {{"));
            program.line_at(1, "total += classify(label * step);");
            program.line("}");
            program.line("total += classify(-1);");
            program.blank();
            let total: i128 = (1..=labels).sum();
            program.check(Ty::I64, "total", total);
            sink.push(
                Facet::SwitchLowering,
                Axes::of([("density", density), ("labels", &labels.to_string())]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Calls with more arguments than fit in registers, and aggregates passed by value.
///
/// The argument count axis walks past the point where the target runs out of argument
/// registers and starts using the stack, which is where a calling convention bug lives. The
/// aggregate shapes are the other half: a small struct goes in registers, a large one goes in
/// memory, and the boundary between them is a target decision that has to be got exactly
/// right or nothing links against a system library.
fn calling_convention(sink: &mut Sink<'_>) {
    for &count in &[1usize, 4, 6, 8, 16] {
        if !sink.wants(Facet::CallingConvention) {
            return;
        }
        let mut program = Program::new(format!("a call with {count} integer arguments"));
        let params: Vec<String> = (0..count).map(|at| format!("long long a{at}")).collect();
        program.top(format!("static long long total{count}({}) {{", params.join(", ")));
        let sum: Vec<String> = (0..count).map(|at| format!("a{at}")).collect();
        program.top(format!("    return {};", sum.join(" + ")));
        program.top("}".to_owned());
        program.input(Ty::I64, "seed", 1);
        program.blank();
        let args: Vec<String> = (0..count).map(|at| format!("seed + {at}")).collect();
        let expected: i128 = (0..count as i128).map(|at| 1 + at).sum();
        program.check(Ty::I64, &format!("total{count}({})", args.join(", ")), expected);
        sink.push(
            Facet::CallingConvention,
            Axes::of([("count", &count.to_string()), ("kind", "integer")]),
            Dialect::C17,
            program,
        );
    }

    const AGGREGATES: &[(&str, usize)] =
        &[("one-field", 1), ("two-fields", 2), ("four-fields", 4), ("sixteen-fields", 16)];
    for &(name, fields) in AGGREGATES {
        if !sink.wants(Facet::CallingConvention) {
            return;
        }
        let mut program = Program::new(format!("a struct of {fields} fields passed by value"));
        let members: Vec<String> = (0..fields).map(|at| format!("long long f{at};")).collect();
        program.top(format!("struct bag {{ {} }};", members.join(" ")));
        program.top("static long long total(struct bag value) {".to_owned());
        let sum: Vec<String> = (0..fields).map(|at| format!("value.f{at}")).collect();
        program.top(format!("    return {};", sum.join(" + ")));
        program.top("}".to_owned());
        program.top("static struct bag build(long long seed) {".to_owned());
        program.top("    struct bag out;".to_owned());
        for at in 0..fields {
            program.top(format!("    out.f{at} = seed + {at};"));
        }
        program.top("    return out;".to_owned());
        program.top("}".to_owned());
        program.input(Ty::I64, "seed", 1);
        program.blank();
        program.line("struct bag made = build(seed);");
        let expected: i128 = (0..fields as i128).map(|at| 1 + at).sum();
        program.check(Ty::I64, "total(made)", expected);
        sink.push(
            Facet::CallingConvention,
            Axes::of([("count", &fields.to_string()), ("kind", name)]),
            Dialect::C17,
            program,
        );
    }

    if sink.wants(Facet::CallingConvention) {
        let mut program = Program::new("a variable argument list");
        program.include("stdarg.h");
        program.top("static long long total(int count, ...) {".to_owned());
        program.top("    va_list rest;".to_owned());
        program.top("    long long sum = 0;".to_owned());
        program.top("    va_start(rest, count);".to_owned());
        program.top("    for (int at = 0; at < count; at++) {".to_owned());
        program.top("        sum += va_arg(rest, long long);".to_owned());
        program.top("    }".to_owned());
        program.top("    va_end(rest);".to_owned());
        program.top("    return sum;".to_owned());
        program.top("}".to_owned());
        program.input(Ty::I64, "seed", 1);
        program.blank();
        program.check(
            Ty::I64,
            "total(6, seed, seed + 1, seed + 2, seed + 3, seed + 4, seed + 5)",
            21,
        );
        sink.push_tagged(
            Facet::CallingConvention,
            Axes::of([("count", "6"), ("kind", "varargs")]),
            Dialect::C17,
            program,
            &["headers"],
        );
    }
}

/// The rewrites that only make sense once instructions have been chosen.
///
/// A comparison against zero that a subtraction already set the flags for. A move whose source
/// and destination end up the same register. A load followed by a mask that the load could
/// have done itself. None of these are visible in the IR, which is why they are a separate
/// facet from `simplify`.
fn machine_peephole(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "compare-after-subtract",
        "test-against-zero",
        "redundant-move",
        "load-then-mask",
        "double-negate",
        "shift-pair",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::MachinePeephole) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("the {shape} peephole on {}", ty.c_name()));
            program.input(ty, "a", 40);
            program.input(ty, "b", 15);
            program.blank();
            let value: i128 = match shape {
                "compare-after-subtract" => {
                    program.line(format!("{name} difference = a - b;"));
                    program.line(format!("int positive = difference > {};", lit(ty, 0)));
                    program.line(format!("{name} total = difference + ({name})positive;"));
                    26
                }
                "test-against-zero" => {
                    program.line(format!("{name} masked = a & {};", lit(ty, 7)));
                    program.line(format!("int zero = masked == {};", lit(ty, 0)));
                    program.line(format!("{name} total = masked + ({name})zero;"));
                    // Forty has no low bits set, so the mask gives nought and the test gives
                    // one.
                    1
                }
                "redundant-move" => {
                    program.line(format!("{name} copy = a;"));
                    program.line(format!("{name} again = copy;"));
                    program.line(format!("{name} total = again - b;"));
                    25
                }
                "load-then-mask" => {
                    program.line(format!("{name} table[2] = {{ a, b }};"));
                    program.line(format!("{name} total = table[0] & {};", lit(ty, 255)));
                    40
                }
                "double-negate" => {
                    if !ty.signed() {
                        continue;
                    }
                    program.line(format!("{name} total = -(-(a - b));"));
                    25
                }
                _ => {
                    program.line(format!("{name} total = (a << 2) >> 2;"));
                    40
                }
            };
            if !ty.promoted().holds(value) {
                continue;
            }
            program.blank();
            program.check(ty.promoted(), "total", value);
            sink.push(
                Facet::MachinePeephole,
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

    fn output(case: &Case) -> &str {
        match &case.expect {
            Expect::Output(text) => text,
            Expect::Rejected(_) => panic!("{} should run", case.id),
        }
    }

    #[test]
    fn the_selection_patterns_cover_arithmetic_comparison_and_bit_work() {
        let cases = cases_for(Facet::Selection);
        let patterns: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("pattern")).collect();
        for wanted in ["multiply-add", "min", "max", "mask-low", "round-down"] {
            assert!(patterns.contains(&wanted), "no case for {wanted}");
        }
        let mad = cases
            .iter()
            .find(|c| {
                c.axes.get("pattern") == Some("multiply-add") && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        assert_eq!(output(mad), "340\n");
    }

    #[test]
    fn register_pressure_goes_past_the_register_file_of_every_target_rucc_has() {
        let cases = cases_for(Facet::RegisterAlloc);
        let live: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("live")).collect();
        assert!(live.contains(&"64"));
        let case = cases
            .iter()
            .find(|c| {
                c.axes.get("live") == Some("4")
                    && c.axes.get("type") == Some("i32")
                    && c.axes.get("shape") == Some("straight")
            })
            .unwrap();
        assert_eq!(output(case), "10\n");
    }

    #[test]
    fn the_scheduling_cases_only_use_types_where_wrapping_is_defined() {
        let cases = cases_for(Facet::Scheduling);
        assert!(!cases.is_empty());
        for case in &cases {
            let ty = case.axes.get("type").unwrap();
            assert!(ty.starts_with('u'), "{} accumulates in {ty}", case.id);
        }
    }

    #[test]
    fn one_chain_and_four_chains_do_the_same_number_of_multiplies() {
        let cases = cases_for(Facet::Scheduling);
        let one = cases
            .iter()
            .find(|c| {
                c.axes.get("chains") == Some("1")
                    && c.axes.get("length") == Some("8")
                    && c.axes.get("type") == Some("u64")
            })
            .unwrap();
        let four = cases
            .iter()
            .find(|c| {
                c.axes.get("chains") == Some("4")
                    && c.axes.get("length") == Some("8")
                    && c.axes.get("type") == Some("u64")
            })
            .unwrap();
        assert_eq!(one.source.matches("* (unsigned long long)3").count(), 8);
        assert_eq!(four.source.matches("* (unsigned long long)3").count(), 32);
    }

    #[test]
    fn every_switch_case_visits_every_label_and_the_default() {
        for case in cases_for(Facet::SwitchLowering) {
            let labels: i128 = case.axes.get("labels").unwrap().parse().unwrap();
            let expected: i128 = (1..=labels).sum();
            assert_eq!(output(&case), format!("{expected}\n"), "{}", case.id);
            assert!(case.source.contains("classify(-1)"), "{}", case.id);
        }
    }

    #[test]
    fn a_sparse_switch_really_is_spread_out() {
        let cases = cases_for(Facet::SwitchLowering);
        let sparse = cases
            .iter()
            .find(|c| {
                c.axes.get("density") == Some("very-sparse") && c.axes.get("labels") == Some("8")
            })
            .unwrap();
        assert!(sparse.source.contains("case 7000:"), "{}", sparse.source);
    }

    #[test]
    fn calls_are_generated_past_the_point_where_arguments_stop_fitting_in_registers() {
        let cases = cases_for(Facet::CallingConvention);
        let counts: Vec<&str> = cases
            .iter()
            .filter(|c| c.axes.get("kind") == Some("integer"))
            .filter_map(|c| c.axes.get("count"))
            .collect();
        assert!(counts.contains(&"8"));
        assert!(counts.contains(&"16"));
        let sixteen = cases
            .iter()
            .find(|c| c.axes.get("kind") == Some("integer") && c.axes.get("count") == Some("16"))
            .unwrap();
        assert_eq!(output(sixteen), "136\n");
    }

    #[test]
    fn the_variable_argument_case_is_the_only_one_that_needs_a_header() {
        let cases = cases_for(Facet::CallingConvention);
        for case in &cases {
            assert_eq!(case.has_tag("headers"), case.source.contains("#include"), "{}", case.id);
        }
        let varargs = cases.iter().find(|c| c.axes.get("kind") == Some("varargs")).unwrap();
        assert!(varargs.source.contains("#include <stdarg.h>"));
        assert_eq!(output(varargs), "21\n");
    }

    #[test]
    fn the_double_negate_peephole_only_appears_for_signed_types() {
        let cases = cases_for(Facet::MachinePeephole);
        for case in &cases {
            if case.axes.get("shape") == Some("double-negate") {
                assert!(case.axes.get("type").unwrap().starts_with('i'), "{}", case.id);
            }
        }
    }

    #[test]
    fn the_if_conversion_cases_run_a_condition_that_flips_every_iteration() {
        for case in cases_for(Facet::IfConversion) {
            assert!(case.source.contains("int odd = i & 1;"), "{}", case.id);
        }
    }

    #[test]
    fn every_layout_case_expects_a_thousand_iterations_of_the_common_path() {
        for case in cases_for(Facet::BlockLayout) {
            assert_eq!(output(&case), "1000\n", "{}", case.id);
        }
    }
}
