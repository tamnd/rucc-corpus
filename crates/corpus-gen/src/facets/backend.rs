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
    scheduling_memory(sink);
    block_layout(sink);
    expect_layout(sink);
    if_conversion(sink);
    if_conversion_rate(sink);
    if_conversion_chain(sink);
    switch_lowering(sink);
    switch_hot_case(sink);
    switch_runs(sink);
    switch_dispatch(sink);
    division(sink);
    exact_division(sink);
    calling_convention(sink);
    machine_peephole(sink);
    bit_liveness(sink);
    compare_elim(sink);
    address_fold(sink);
    load_fold(sink);
    store_fold(sink);
    store_fold_constant(sink);
    compare_fold(sink);
    frame_address(sink);
    stack_slots(sink);
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
///
/// `call-off-the-path` is the same question asked the other way round. The values are read in
/// the second arm of a branch and the call is in the first, so a compiler that measures where
/// a value is live by where it sits in the function as written has the call in the middle of
/// every one of them, and a compiler that measures it on the control flow graph has the call
/// on a path none of them are on. The first answer costs every caller saved register at once,
/// which on x86-64 is seven of them, for a call the values never reach. tamnd/rucc#982.
fn register_pressure(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["across-a-call", "through-a-loop", "both-classes", "one-arm-only", "call-off-the-path"];
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
                if matches!(shape, "across-a-call" | "call-off-the-path") {
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
                    "call-off-the-path" => {
                        // The call is written first and the reads are written second, so every
                        // value's first definition is above the call and its only use is below
                        // it. The branch is on a value read out of `volatile` storage, so no
                        // compiler is entitled to decide which arm runs.
                        program.line(format!("{name} kept = 0;"));
                        program.line("if (flag == 0) {");
                        program.line_at(1, "kept = opaque(seed);");
                        program.line("} else {");
                        for at in (0..live).rev() {
                            program.line_at(1, format!("kept += v{at};"));
                        }
                        program.line("}");
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

                // Read back to front, so the value defined first is the one live longest. The
                // off the path shape has read them already, inside the arm that is the whole
                // point of it, and reading them again here would put them back on every path.
                program.line(format!("{name} total = kept;"));
                if shape != "call-off-the-path" {
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

/// Work with a memory access in the middle of it, which nothing may be reordered around.
///
/// A machine scheduler leaves the accesses of a block in the order they arrived in. It does not
/// ask whether two of them can be the same place, because by the time it runs there is nothing
/// left to ask with: machine instructions do not carry `volatile` and they do not carry the types
/// the source had. That is conservative and it is not free, and these cases are here to say what
/// it buys. Each one puts a store and a load next to each other at an index worked out from the
/// loop counter and a value read out of a volatile global, so no compiler can decide the two are
/// different places, and each one prints a number that comes out different if the two are swapped.
///
/// The read-only shape is the control. It has the same shape of address arithmetic and the same
/// number of accesses and no store at all, so the difference between it and the other three is the
/// cost of the ordering rather than the cost of touching memory.
fn scheduling_memory(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["read-after-write", "write-after-read", "two-writes", "read-only"];
    const SIZE: i128 = 16;
    const STEPS: i128 = 64;
    const MASK: i128 = SIZE - 1;
    for &ty in TYPES {
        // Unsigned only, for the same reason the chains above are unsigned. Sixty four rounds of
        // multiply by five overflows every one of these types many times over, and wrapping is
        // only defined for the unsigned ones.
        if ty.signed() {
            continue;
        }
        for &shape in SHAPES {
            if !sink.wants(Facet::Scheduling) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!(
                "a {shape} memory access in a loop the compiler cannot unpick"
            ));
            program.input(ty, "seed", 1);
            program.input(Ty::I32, "start", 0);
            if shape == "two-writes" {
                // Zero, so the two stores go to the same element and the second one decides what
                // is there. Nothing in the program says that, so a compiler has to keep the two
                // in the order they were written to find out.
                program.input(Ty::I32, "gap", 0);
            }
            program.blank();
            program.line(format!("{name} buffer[{SIZE}];"));
            program.line(format!("for (int i = 0; i < {SIZE}; i++) {{"));
            program.line_at(1, format!("buffer[i] = seed + ({name})i;"));
            program.line("}");
            program.line(format!("{name} total = 0;"));
            program.line(format!("for (int i = 0; i < {STEPS}; i++) {{"));
            // Seven and sixteen have no factor in common, so this walks every element of the
            // buffer before it comes back to one, and it is not the counter so nothing folds it.
            program.line_at(1, format!("int at = (i * 7 + start) & {MASK};"));

            let mut buffer: Vec<i128> = (0..SIZE).map(|i| ty.convert(1 + i)).collect();
            let mut total: i128 = 0;
            match shape {
                // The load has to wait for the store, because it is reading what the store just
                // put there. This is the one shape where a swap is a wrong answer rather than a
                // different one.
                "read-after-write" => {
                    program.line_at(
                        1,
                        format!("buffer[at] = buffer[at] * {} + {};", lit(ty, 3), lit(ty, 1)),
                    );
                    program.line_at(1, format!("total = total * {} + buffer[at];", lit(ty, 5)));
                    for i in 0..STEPS {
                        let at = ((i * 7) & MASK) as usize;
                        buffer[at] = ty.promoted().convert(buffer[at] * 3 + 1);
                        total = ty.promoted().convert(total * 5 + buffer[at]);
                    }
                }
                // The store has to wait for the load, because it is writing over what the load
                // reads. No value passes between the two, which is the case a scheduler that
                // only tracks values would get wrong.
                "write-after-read" => {
                    program.line_at(1, format!("{name} seen = buffer[at];"));
                    program.line_at(
                        1,
                        format!("buffer[at] = seen * {} + {};", lit(ty, 3), lit(ty, 1)),
                    );
                    program.line_at(1, format!("total = total * {} + seen;", lit(ty, 5)));
                    for i in 0..STEPS {
                        let at = ((i * 7) & MASK) as usize;
                        let seen = buffer[at];
                        buffer[at] = ty.promoted().convert(seen * 3 + 1);
                        total = ty.promoted().convert(total * 5 + seen);
                    }
                }
                // Two stores to what turns out to be one element, and then a load of it. The
                // element ends up holding whichever store ran second.
                "two-writes" => {
                    program.line_at(1, format!("buffer[at] = total + {};", lit(ty, 1)));
                    program.line_at(
                        1,
                        format!("buffer[(at + gap) & {MASK}] = total + {};", lit(ty, 2)),
                    );
                    program.line_at(1, format!("total = total * {} + buffer[at];", lit(ty, 5)));
                    for i in 0..STEPS {
                        let at = ((i * 7) & MASK) as usize;
                        buffer[at] = ty.promoted().convert(total + 1);
                        buffer[at] = ty.promoted().convert(total + 2);
                        total = ty.promoted().convert(total * 5 + buffer[at]);
                    }
                }
                // No store, so the two loads are held in order by nothing but the rule. The
                // answer is the same whichever order they run in and the time should be too.
                "read-only" => {
                    program.line_at(
                        1,
                        format!(
                            "total = total * {} + buffer[at] + buffer[(at + 3) & {MASK}];",
                            lit(ty, 5)
                        ),
                    );
                    for i in 0..STEPS {
                        let at = ((i * 7) & MASK) as usize;
                        let other = ((at as i128 + 3) & MASK) as usize;
                        total = ty.promoted().convert(total * 5 + buffer[at] + buffer[other]);
                    }
                }
                other => unreachable!("no such shape: {other}"),
            }
            program.line("}");
            program.blank();
            program.check(ty.promoted(), "total", total);
            sink.push(
                Facet::Scheduling,
                Axes::of([("type", ty.name()), ("access", shape)]),
                Dialect::C17,
                program,
            );
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

/// One branch shape, written three times: with no hint, and with each of the two that GCC has.
///
/// The condition is the same text in all three, and so is the answer, so the only thing that
/// differs between a triple is what the program said about which way the branch goes. That is
/// what makes the triple evidence. A compiler that lays all three out the same way is a
/// compiler that read the hint and threw it away.
///
/// The size will often be the same in all three, and that is not the triple failing to say
/// anything. A block that moved is the same instructions in a different order, so a layout that
/// changed and a layout that did not can weigh the same, and on a branch this predictable the
/// time is the same too. What the triple gives is three objects built from one program that
/// differ only in the hint, which is the thing to put side by side.
struct Hinted {
    /// The axis value, which is also the name of the shape.
    name: &'static str,
    /// The condition, before a hint is wrapped around it.
    condition: &'static str,
    /// Whether the condition is the one that nearly always holds.
    likely: bool,
}

/// The branch shapes a hint is worth putting on.
const HINTED: &[Hinted] = &[
    Hinted { name: "rare-call", condition: "rare != 0", likely: false },
    Hinted { name: "hot-then", condition: "rare == 0", likely: true },
    Hinted { name: "widened-condition", condition: "!!(rare == 1)", likely: false },
    Hinted { name: "rare-chain", condition: "rare == 1", likely: false },
];

/// The three ways of writing the same condition.
const HINTS: &[&str] = &["none", "builtin", "probability"];

/// A condition with a hint wrapped around it, or the condition on its own.
fn hinted(hint: &str, condition: &str, likely: bool) -> String {
    match hint {
        "builtin" => format!("__builtin_expect({condition}, {})", i32::from(likely)),
        "probability" => {
            let odds = if likely { "0.99" } else { "0.01" };
            format!("__builtin_expect_with_probability({condition}, 1, {odds})")
        }
        _ => condition.to_owned(),
    }
}

/// Branches the program itself says which way they go.
///
/// The facet next door has branches that are predictable and says nothing about them, so the
/// compiler has to guess from the shape. These say it out loud, which is the case where a
/// guess is not involved and getting the layout wrong has no excuse.
///
/// The cold arm is a call through a `volatile` function pointer. A direct call to a `static`
/// function is not enough, because GCC inlines it and the cold arm stops being a call at all,
/// which was the first version of these cases and was reported back by `-fopt-info` in so many
/// words. Reading a `volatile` pointer is a read the compiler has to make, so the call stays a
/// call in every compiler and the arm stays the heavy one it was written to be. It is plain
/// C17, which matters, because the third of each triple without a hint has to be a case a run
/// that excludes extensions still gets to see.
///
/// Both spellings are here. `__builtin_expect` says which way and nothing more, and
/// `__builtin_expect_with_probability` says how strongly, and a compiler that supports the
/// first and ignores the second is a compiler that will lay the second out by guesswork.
fn expect_layout(sink: &mut Sink<'_>) {
    for shape in HINTED {
        for &hint in HINTS {
            if !sink.wants(Facet::BlockLayout) {
                return;
            }
            let mut program = Program::new(format!(
                "a {} branch in a hot loop, with the {hint} hint on it",
                shape.name
            ));
            // The cold arm calls this. It touches a volatile global so that no amount of
            // inlining can turn the call into nothing, which is what would happen to an empty
            // function and would take the case with it. The pointer is what the call goes
            // through, and it is volatile so that the call cannot be turned back into a direct
            // one and inlined.
            program.top("static volatile int oops_count;");
            program.top("static void oops(void) {");
            program.top("    oops_count = oops_count + 1;");
            program.top("}");
            program.top("static void (*volatile cold)(void) = oops;");
            program.input(Ty::I32, "rare", 0);
            program.blank();
            program.line("long long total = 0;");
            program.line("for (int i = 0; i < 1000; i++) {");
            let test = hinted(hint, shape.condition, shape.likely);
            match shape.name {
                "hot-then" => {
                    program.line_at(1, format!("if ({test}) {{"));
                    program.line_at(2, "total += 1;");
                    program.line_at(1, "} else {");
                    program.line_at(2, "cold();");
                    program.line_at(1, "}");
                }
                "rare-chain" => {
                    // Three tests in a row, each one said to be the unlikely way, and the arm
                    // that runs is the one at the bottom. A layout that walks the tests in the
                    // order they were written puts three not taken branches on the hot path
                    // and nothing else, which is the best this shape can do.
                    for (at, condition) in
                        ["rare == 1", "rare == 2", "rare == 3"].iter().enumerate()
                    {
                        let test = hinted(hint, condition, false);
                        let lead = if at == 0 { "if" } else { "} else if" };
                        program.line_at(1, format!("{lead} ({test}) {{"));
                        program.line_at(2, "cold();");
                    }
                    program.line_at(1, "} else {");
                    program.line_at(2, "total += 1;");
                    program.line_at(1, "}");
                }
                _ => {
                    program.line_at(1, format!("if ({test}) {{"));
                    program.line_at(2, "cold();");
                    program.line_at(1, "}");
                    program.line_at(1, "total += 1;");
                }
            }
            program.line("}");
            program.blank();
            program.check(Ty::I64, "total", 1000);
            let axes = Axes::of([("shape", shape.name), ("hint", hint)]);
            if hint == "none" {
                sink.push(Facet::BlockLayout, axes, Dialect::C17, program);
            } else {
                sink.push_tagged(Facet::BlockLayout, axes, Dialect::C17, program, &["gnu"]);
            }
        }
    }
}

/// How often the diamond in the rate shapes goes the first way, in percent.
const RATES: &[u32] = &[50, 75, 90, 95, 99];

/// How many times the rate shapes go round, which is enough for the time to be the measurement.
const ROLLS: u32 = 20_000_000;

/// What one of the rate shapes adds up, worked out the way the C works it out.
fn rolled(seed: u32, rate: u32) -> u64 {
    let mut state = seed;
    let mut total: u64 = 0;
    for _ in 0..ROLLS {
        state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let x = state >> 24;
        let v = if (state >> 16) % 100 < rate { x * 3 } else { x + 7 };
        total = total.wrapping_add(u64::from(v));
    }
    total
}

/// A diamond with work in both arms, on a condition that holds a known share of the time.
///
/// Section 22.2 of the rucc plan converts a diamond whose arms do work only when the branch is
/// close enough to even that the machine will miss it, and a branch is only known to be one sided
/// when the program says so with `__builtin_expect`. These are the cases that decide how close to
/// even is close enough. The condition comes from a linear congruential generator, so the only
/// thing a predictor can learn about it is how often it holds, and that is the `rate` axis. Each
/// rate is written with no hint, with the hint that agrees with the rate, and with the hint that
/// does not. Timing the three against each other at each rate shows where a branch starts beating
/// a conditional move, which is the number the margin in the cost rule is meant to be.
fn if_conversion_rate(sink: &mut Sink<'_>) {
    for &rate in RATES {
        for &hint in &["none", "right", "wrong"] {
            if !sink.wants(Facet::IfConversion) {
                return;
            }
            let mut program = Program::new(format!(
                "a diamond with work in both arms, taken {rate} times in a hundred, with {hint} hint"
            ));
            program.input(Ty::U32, "seed", 7);
            program.blank();
            program.line("unsigned state = seed;");
            program.line("unsigned long long total = 0;");
            program.line(format!("for (int i = 0; i < {ROLLS}; i++) {{"));
            program.line_at(1, "state = state * 1103515245u + 12345u;");
            program.line_at(1, "unsigned x = state >> 24;");
            program.line_at(1, format!("int taken = (state >> 16) % 100u < {rate}u;"));
            program.line_at(1, "unsigned v;");
            let test = match hint {
                "right" => "__builtin_expect(taken, 1)",
                "wrong" => "__builtin_expect(taken, 0)",
                _ => "taken",
            };
            program.line_at(1, format!("if ({test}) {{"));
            program.line_at(2, "v = x * 3u;");
            program.line_at(1, "} else {");
            program.line_at(2, "v = x + 7u;");
            program.line_at(1, "}");
            program.line_at(1, "total += v;");
            program.line("}");
            program.blank();
            program.check(Ty::U64, "total", i128::from(rolled(7, rate)));
            let rate = rate.to_string();
            let axes = Axes::of([("shape", "at-a-rate"), ("rate", rate.as_str()), ("hint", hint)]);
            if hint == "none" {
                sink.push(Facet::IfConversion, axes, Dialect::C17, program);
            } else {
                sink.push_tagged(Facet::IfConversion, axes, Dialect::C17, program, &["gnu"]);
            }
        }
    }
}

/// A diamond whose two arms end in the same operations, one of the chain shapes.
struct Chained {
    /// The axis value, which is also the name of the shape.
    name: &'static str,
    /// One line saying what the compiler has to see.
    purpose: &'static str,
    /// What the arm taken when the condition holds adds to the total, as C over `x` and `mask`.
    then: &'static str,
    /// What the other arm adds.
    other: &'static str,
    /// Both of those in Rust, given whether the condition held, `x` and `mask`.
    step: fn(bool, i64, i64) -> i64,
}

/// The value `mask` has in the chain shapes.
const CHAIN_MASK: i64 = 85;

/// Arms that share more than the operation the join reads.
///
/// Each arm adds a widened `int` to a `long long`, so both end in an add of a widening and only
/// what is widened differs. Factoring only the add leaves a widening in each arm, and one level
/// more leaves a mask as well, which is past what an arm may keep and still become a select.
const CHAINED: &[Chained] = &[
    Chained {
        name: "shared-widening",
        purpose: "arms that both add a widened int, differing only in what is widened",
        then: "(long long)(x * 3)",
        other: "(long long)(x + 7)",
        step: |taken, x, _| if taken { x * 3 } else { x + 7 },
    },
    Chained {
        name: "shared-mask",
        purpose: "arms that both add a widened masked int, differing only in what is masked",
        then: "(long long)((x * 3) ^ mask)",
        other: "(long long)((x + 7) ^ mask)",
        step: |taken, x, mask| if taken { (x * 3) ^ mask } else { (x + 7) ^ mask },
    },
    Chained {
        name: "different-widening",
        purpose: "arms that both add, one widening with a sign and one without",
        then: "(long long)(x * 3)",
        other: "(long long)(unsigned)(x + 7)",
        step: |taken, x, _| if taken { x * 3 } else { x + 7 },
    },
];

/// What one of the chain shapes adds up, worked out the way the C works it out.
fn chained(seed: u32, step: fn(bool, i64, i64) -> i64) -> i64 {
    let mut state = seed;
    let mut total: i64 = 0;
    for _ in 0..ROLLS {
        state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let x = i64::from(state >> 24);
        let taken = (state >> 16) % 100 < 50;
        total = total.wrapping_add(step(taken, x, CHAIN_MASK));
    }
    total
}

/// Diamonds whose arms share a chain of operations and differ only at the bottom of it.
///
/// Section 22.2 of the rucc plan factors an operation both arms do out from under the branch, and
/// gcc does it to a fixed point, so a chain both arms share is written once and the select is
/// between the two things at the bottom. The condition is the rate shapes' generator at an even
/// rate, so the branch is one no predictor can learn and turning it into a select is the right
/// answer whenever what is left in the arms is small. The last shape is the control, where the two
/// widenings differ and only the add can be shared.
fn if_conversion_chain(sink: &mut Sink<'_>) {
    for shape in CHAINED {
        if !sink.wants(Facet::IfConversion) {
            return;
        }
        let mut program = Program::new(shape.purpose);
        program.input(Ty::U32, "seed", 7);
        program.input(Ty::I32, "mask", CHAIN_MASK.into());
        program.blank();
        program.line("unsigned state = seed;");
        program.line("long long total = 0;");
        program.line(format!("for (int i = 0; i < {ROLLS}; i++) {{"));
        program.line_at(1, "state = state * 1103515245u + 12345u;");
        program.line_at(1, "int x = (int)(state >> 24);");
        program.line_at(1, "if ((state >> 16) % 100u < 50u) {");
        program.line_at(2, format!("total += {};", shape.then));
        program.line_at(1, "} else {");
        program.line_at(2, format!("total += {};", shape.other));
        program.line_at(1, "}");
        program.line("}");
        program.blank();
        program.check(Ty::I64, "total", i128::from(chained(7, shape.step)));
        sink.push(Facet::IfConversion, Axes::of([("shape", shape.name)]), Dialect::C17, program);
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
///
/// The sizes sit either side of thirty two, which is where rucc stops walking the clusters and
/// starts searching them. A facet whose nearest sizes to that line are eight and forty can tell
/// you the search works but not that the two halves agree at the point they meet.
fn switch_lowering(sink: &mut Sink<'_>) {
    const DENSITIES: &[(&str, i128)] = &[("dense", 1), ("sparse", 17), ("very-sparse", 1000)];
    for &(density, stride) in DENSITIES {
        for &labels in &[3i128, 8, 31, 33, 40] {
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

/// How many labels the hot case shapes have, past the point where rucc searches the clusters.
const HOT_LABELS: i128 = 40;

/// Which label the hot case shapes take most of the time, one in the middle so that a balanced
/// search reaches it last rather than first.
const HOT_LABEL: i128 = 23;

/// Which label the wrong hint in the hot case shapes names.
const COLD_LABEL: i128 = 5;

/// What the arm for this label returns, which is not a line through the labels so that nothing can
/// replace the switch with arithmetic.
fn hot_arm(label: i128) -> i128 {
    label * 7919 % 1000 + 1
}

/// Where the draw in the hot case shapes has to fall under for the hot label, out of 128, for a
/// rate in percent. Out of 128 and not 100 so that the draw is a mask rather than a division.
fn hot_bar(rate: u32) -> u32 {
    (rate * 128 + 50) / 100
}

/// What one of the hot case shapes adds up, worked out the way the C works it out.
fn hot_rolled(seed: u32, rate: u32, stride: i128) -> u64 {
    let mut state = seed;
    let mut total: u64 = 0;
    for _ in 0..ROLLS {
        state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let label = if (state >> 16) & 127 < hot_bar(rate) {
            HOT_LABEL
        } else {
            i128::from(((state >> 24) * 40) >> 8)
        };
        let value = label * stride;
        let found = (0..HOT_LABELS).find(|&at| at * stride == value).map_or(0, hot_arm);
        total = total.wrapping_add(u64::try_from(found).expect("a positive arm"));
    }
    total
}

/// A switch on a value that is one case most of the time, with and without a hint saying which.
///
/// Section 24.5 of the rucc plan tests a case a hint makes hot on its own, ahead of the search or
/// the table, so the common value is one compare and a taken branch. These are the cases that show
/// whether that pays. The value comes from a linear congruential generator and is the hot label
/// `rate` times in a hundred and any label the rest of the time. Each is written with no hint, with
/// `__builtin_expect` naming the hot label, and with it naming a label that is rarely there, and
/// for a dense switch and a sparse one, since a dense one is a table either way.
///
/// The labels are drawn with a mask and a multiply rather than with `%`, because a division in the
/// loop costs more than the whole switch and a compiler that does not turn it into a multiply
/// would be timed on that instead.
fn switch_hot_case(sink: &mut Sink<'_>) {
    const DENSITIES: &[(&str, i128)] = &[("dense", 1), ("sparse", 17)];
    for &(density, stride) in DENSITIES {
        for &rate in &[75u32, 95] {
            for &hint in &["none", "right", "wrong"] {
                if !sink.wants(Facet::SwitchLowering) {
                    return;
                }
                let mut program = Program::new(format!(
                    "a {density} switch on a value that is one case {rate} times in a hundred, with {hint} hint"
                ));
                let operand = match hint {
                    "right" => format!("__builtin_expect(value, {})", HOT_LABEL * stride),
                    "wrong" => format!("__builtin_expect(value, {})", COLD_LABEL * stride),
                    _ => "value".to_owned(),
                };
                program.top("static int classify(int value) {".to_owned());
                program.top(format!("    switch ({operand}) {{"));
                for label in 0..HOT_LABELS {
                    program.top(format!("    case {}: return {};", label * stride, hot_arm(label)));
                }
                program.top("    default: return 0;".to_owned());
                program.top("    }".to_owned());
                program.top("}".to_owned());
                program.input(Ty::U32, "seed", 7);
                program.input(Ty::I32, "step", stride);
                program.blank();
                program.line("unsigned state = seed;");
                program.line("unsigned long long total = 0;");
                program.line(format!("for (int i = 0; i < {ROLLS}; i++) {{"));
                program.line_at(1, "state = state * 1103515245u + 12345u;");
                let bar = hot_bar(rate);
                program.line_at(1, format!(
                    "int label = ((state >> 16) & 127u) < {bar}u ? {HOT_LABEL} : (int)(((state >> 24) * 40u) >> 8);"
                ));
                program.line_at(1, "total += (unsigned long long)classify(label * step);");
                program.line("}");
                program.blank();
                program.check(Ty::U64, "total", i128::from(hot_rolled(7, rate, stride)));
                let rate = rate.to_string();
                let axes = Axes::of([
                    ("density", density),
                    ("shape", "hot-case"),
                    ("rate", rate.as_str()),
                    ("hint", hint),
                ]);
                if hint == "none" {
                    sink.push(Facet::SwitchLowering, axes, Dialect::C17, program);
                } else {
                    sink.push_tagged(Facet::SwitchLowering, axes, Dialect::C17, program, &["gnu"]);
                }
            }
        }
    }
}

/// Where a run of labels starts, in the shapes that do not need a particular value.
const RUN_BASE: i128 = 10;

/// How far either side of a run the walk goes, so the label next to the run is asked about too.
const RUN_PAD: i128 = 3;

/// How long each run is in the shapes that have more than one of them.
const RUN_LENGTH: i128 = 8;

/// Stretches of consecutive labels that all go to one arm.
///
/// This is the shape every hand written classifier has and the one [`switch_lowering`] does not
/// cover, because there every label goes somewhere different and so nothing can become a range
/// test. A stretch that shares an arm is one subtraction and one unsigned comparison however long
/// it is, and a compiler that walks the labels one at a time instead is emitting a test per label
/// for nothing. Without cases like these the corpus cannot tell the two apart, and a change to
/// the cluster code can regress with every program still printing the right answer.
///
/// The value under test always comes out of a `volatile`, and so does the number of times the
/// walk runs. A switch on a value the optimizer knows is not a switch by the time it reaches the
/// back end, and a walk with a literal bound is a walk that gets unrolled into a list of
/// constants, so either one left alone turns the case into a question about folding.
fn switch_runs(sink: &mut Sink<'_>) {
    one_run(sink);
    several_runs(sink);
    runs_and_singles(sink);
    runs_at_the_edge(sink);
    classifiers(sink);
    look_alike_arms(sink);
}

/// Emits the walk that drives one of these switches, and the check on what it added up to.
fn walk(program: &mut Program, call: &str, first: i128, count: i128, total: i128) {
    program.input(Ty::I32, "first", first);
    program.input(Ty::I32, "count", count);
    program.blank();
    program.line("long long total = 0;");
    program.line("for (int step = 0; step < count; step++) {");
    program.line_at(1, format!("total += {call}(first + step);"));
    program.line("}");
    program.blank();
    program.check(Ty::I64, "total", total);
}

/// Writes the `default` arm and closes the switch and the function around it.
fn close(program: &mut Program) {
    program.top("    default:");
    program.top("        return 0;");
    program.top("    }");
    program.top("}");
}

/// One run, at three lengths.
///
/// Four labels is short enough that a chain of comparisons is a reasonable answer and a range
/// test is only slightly better. A hundred is long enough that the difference between the two is
/// the difference between one comparison and a hundred, which is the point.
fn one_run(sink: &mut Sink<'_>) {
    for &length in &[4i128, 26, 100] {
        if !sink.wants(Facet::SwitchRuns) {
            return;
        }
        let mut program = Program::new(format!("one run of {length} labels sharing an arm"));
        program.top("static int in_run(int value) {");
        program.top("    switch (value) {");
        for label in 0..length {
            program.top(format!("    case {}:", RUN_BASE + label));
        }
        program.top("        return 1;");
        close(&mut program);
        walk(&mut program, "in_run", RUN_BASE - RUN_PAD, length + 2 * RUN_PAD, length);
        sink.push(
            Facet::SwitchRuns,
            Axes::of([("shape", "one-run"), ("variant", &length.to_string())]),
            Dialect::C17,
            program,
        );
    }
}

/// Adjacent runs, each to its own arm, at counts either side of the cluster limit.
///
/// This is the partition rather than the single range test: the switch is one run after another
/// with nothing between them, so the answer is a subtraction and a lookup or a search over the
/// runs, and never a test per label. Thirty one and thirty three sit either side of the point
/// where rucc stops walking the clusters and starts searching them.
fn several_runs(sink: &mut Sink<'_>) {
    for &arms in &[2i128, 4, 31, 33] {
        if !sink.wants(Facet::SwitchRuns) {
            return;
        }
        let mut program =
            Program::new(format!("{arms} runs of {RUN_LENGTH} labels, each to its own arm"));
        program.top("static int which_run(int value) {");
        program.top("    switch (value) {");
        for arm in 0..arms {
            for label in 0..RUN_LENGTH {
                program.top(format!("    case {}:", RUN_BASE + arm * RUN_LENGTH + label));
            }
            program.top(format!("        return {};", arm + 1));
        }
        close(&mut program);
        let total = RUN_LENGTH * (1..=arms).sum::<i128>();
        let count = arms * RUN_LENGTH + 2 * RUN_PAD;
        walk(&mut program, "which_run", RUN_BASE - RUN_PAD, count, total);
        sink.push(
            Facet::SwitchRuns,
            Axes::of([("shape", "several-runs"), ("variant", &arms.to_string())]),
            Dialect::C17,
            program,
        );
    }
}

/// Runs and scattered single labels in one statement.
///
/// Section 24.2 argues for clusters rather than a single decision about the whole switch by
/// pointing at exactly this: part of it is dense enough to be a range test and part of it is so
/// spread out that nothing but a comparison will do. A compiler that picks one lowering for the
/// whole statement gets one half of this wrong whichever it picks.
///
/// The probe list is three values around each single and six around each run, so the label at
/// each end of a run and the one just outside it are all asked about. It is driven through an
/// opaque bump so the calls are not a list of constants.
fn runs_and_singles(sink: &mut Sink<'_>) {
    const RUNS: &[(i128, i128, i128)] = &[(10, 15, 1), (40, 45, 2), (300, 311, 3)];
    const SINGLES: &[(i128, i128)] = &[(100, 4), (250, 5), (600, 6), (1500, 7), (4000, 8)];
    if !sink.wants(Facet::SwitchRuns) {
        return;
    }
    let mut program = Program::new("runs and scattered single labels in one switch");
    program.top("static int mixed(int value) {");
    program.top("    switch (value) {");
    for &(low, high, arm) in RUNS {
        for label in low..=high {
            program.top(format!("    case {label}:"));
        }
        program.top(format!("        return {arm};"));
    }
    for &(value, arm) in SINGLES {
        program.top(format!("    case {value}: return {arm};"));
    }
    close(&mut program);

    let answer = |value: i128| -> i128 {
        for &(low, high, arm) in RUNS {
            if value >= low && value <= high {
                return arm;
            }
        }
        for &(single, arm) in SINGLES {
            if value == single {
                return arm;
            }
        }
        0
    };
    let mut probes: Vec<i128> = Vec::new();
    for &(low, high, _) in RUNS {
        probes.extend([low - 1, low, low + 1, high - 1, high, high + 1]);
    }
    for &(single, _) in SINGLES {
        probes.extend([single - 1, single, single + 1]);
    }
    let total: i128 = probes.iter().copied().map(answer).sum();
    let written: Vec<String> = probes.iter().map(i128::to_string).collect();
    program.top(format!("static const int probes[] = {{ {} }};", written.join(", ")));
    program.input(Ty::I32, "bump", 0);
    program.input(Ty::I32, "count", probes.len() as i128);
    program.blank();
    program.line("long long total = 0;");
    program.line("for (int at = 0; at < count; at++) {");
    program.line_at(1, "total += mixed(probes[at] + bump);");
    program.line("}");
    program.blank();
    program.check(Ty::I64, "total", total);
    sink.push(
        Facet::SwitchRuns,
        Axes::of([("shape", "runs-and-singles"), ("variant", "mixed")]),
        Dialect::C17,
        program,
    );
}

/// A run that reaches the last value of its type, at both ends and in both signednesses.
///
/// Section 24.6 names this one. A range test is the low label subtracted from the value compared
/// against the length of the run, and working the length out is one past the high label minus the
/// low one. When the run reaches the end of the type that addition is the overflow the section is
/// about, and a compiler that computes it in the type of the switch rather than in something
/// wider produces a test that is off by the whole range.
fn runs_at_the_edge(sink: &mut Sink<'_>) {
    const WINDOW: i128 = 6;
    const LENGTH: i128 = 3;
    for &ty in &[Ty::I32, Ty::U32] {
        for &top in &[false, true] {
            if !sink.wants(Facet::SwitchRuns) {
                return;
            }
            let edge = if top { "top" } else { "bottom" };
            let low = if top { ty.max() - LENGTH + 1 } else { ty.min() };
            let first = if top { ty.max() - WINDOW + 1 } else { ty.min() };
            let mut program = Program::new(format!(
                "a run of {LENGTH} labels reaching the {edge} of {}",
                ty.c_name()
            ));
            program.top(format!("static int at_edge({} value) {{", ty.c_name()));
            program.top("    switch (value) {");
            for label in low..low + LENGTH {
                program.top(format!("    case {}:", ty.literal(label)));
            }
            program.top("        return 1;");
            close(&mut program);
            program.input(ty, "first", first);
            program.input(Ty::I32, "count", WINDOW);
            program.blank();
            program.line("long long total = 0;");
            program.line("for (int step = 0; step < count; step++) {");
            program.line_at(1, format!("total += at_edge(first + ({})step);", ty.c_name()));
            program.line("}");
            program.blank();
            program.check(Ty::I64, "total", LENGTH);
            sink.push(
                Facet::SwitchRuns,
                Axes::of([("shape", "edge-run"), ("variant", &format!("{}-{edge}", ty.name()))]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A character classifier, written the way people write them.
///
/// Twenty six letters to one arm, twenty six more to another, ten digits to a third and three
/// spaces to a fourth. That is sixty five labels and four answers, and it is what the front of
/// every lexer looks like. The text is all ASCII, so whether a plain `char` is signed does not
/// change the answer, which is what makes it fair to run the same expected output against both
/// spellings of the parameter.
fn classifiers(sink: &mut Sink<'_>) {
    const CLASSES: &[(char, char, i128)] = &[('a', 'z', 1), ('A', 'Z', 2), ('0', '9', 3)];
    const SPACES: &[(&str, char)] = &[("' '", ' '), ("'\\t'", '\t'), ("'\\n'", '\n')];
    const TEXT: &str = "The quick brown fox jumps over 13 lazy dogs.\tTwice, at 07:45.\n";
    for &name in &["char", "unsigned char"] {
        if !sink.wants(Facet::SwitchRuns) {
            return;
        }
        let mut program = Program::new(format!("a character classifier written over {name}"));
        program.top(format!("static int kind({name} c) {{"));
        program.top("    switch (c) {");
        for &(low, high, arm) in CLASSES {
            for letter in low..=high {
                program.top(format!("    case '{letter}':"));
            }
            program.top(format!("        return {arm};"));
        }
        for &(spelling, _) in SPACES {
            program.top(format!("    case {spelling}:"));
        }
        program.top("        return 4;");
        close(&mut program);

        let answer = |letter: char| -> i128 {
            for &(low, high, arm) in CLASSES {
                if letter >= low && letter <= high {
                    return arm;
                }
            }
            if SPACES.iter().any(|&(_, space)| space == letter) { 4 } else { 0 }
        };
        let total: i128 = TEXT.chars().map(answer).sum();
        let written: String = TEXT
            .chars()
            .map(|letter| match letter {
                '\t' => "\\t".to_owned(),
                '\n' => "\\n".to_owned(),
                other => other.to_string(),
            })
            .collect();
        program.top(format!("static volatile {name} text[] = \"{written}\";"));
        program.input(Ty::I32, "count", TEXT.len() as i128);
        program.blank();
        program.line("long long total = 0;");
        program.line("for (int at = 0; at < count; at++) {");
        program.line_at(1, "total += kind(text[at]);");
        program.line("}");
        program.blank();
        program.check(Ty::I64, "total", total);
        let variant = if name == "char" { "char" } else { "unsigned-char" };
        sink.push(
            Facet::SwitchRuns,
            Axes::of([("shape", "classifier"), ("variant", variant)]),
            Dialect::C17,
            program,
        );
    }
}

/// Consecutive labels whose arms are written the same way and are not the same arm.
///
/// The negative control. Every arm here is one assignment of the switch value plus a constant, so
/// they look alike to anything comparing the shape of the code, and each one adds a different
/// constant so folding them into a single range test gives four wrong answers out of five. A run
/// is only a run when the labels agree about where they go, not about how they get there.
fn look_alike_arms(sink: &mut Sink<'_>) {
    const LENGTH: i128 = 5;
    if !sink.wants(Facet::SwitchRuns) {
        return;
    }
    let mut program = Program::new("consecutive labels whose arms look alike and are not");
    program.top("static int pick(int value) {");
    program.top("    int out;");
    program.top("    switch (value) {");
    for step in 0..LENGTH {
        program.top(format!("    case {}:", RUN_BASE + step));
        program.top(format!("        out = value + {};", step + 1));
        program.top("        break;");
    }
    program.top("    default:");
    program.top("        out = 0;");
    program.top("        break;");
    program.top("    }");
    program.top("    return out;");
    program.top("}");
    let total: i128 = (0..LENGTH).map(|step| RUN_BASE + step + step + 1).sum();
    walk(&mut program, "pick", RUN_BASE - RUN_PAD, LENGTH + 2 * RUN_PAD, total);
    sink.push(
        Facet::SwitchRuns,
        Axes::of([("shape", "look-alike-arms"), ("variant", "five")]),
        Dialect::C17,
        program,
    );
}

/// How many values a dispatch stream holds.
///
/// A power of two, so the walk over it is an `and` rather than a division, and the index
/// arithmetic is not what the case ends up measuring. Sixteen kilobytes of it, which stays in
/// the first level cache on anything this corpus runs on, so the case measures the switch
/// rather than the memory system.
///
/// The length matters more than it looks. A modern predictor keeps enough history to memorise
/// a repeating sequence of a few hundred branches, and a shorter stream than this one is
/// learned outright. Measured on a six core Xeon, dispatching the `scattered` shape ten million
/// times on the unpredictable stream: 512 values gives 25,645 mispredicts, 1024 gives 122,626,
/// 2048 gives 1,310,981 and 4096 gives 5,004,629. So a 512 value stream is not unpredictable at
/// all, it is a pattern the hardware has already learned by the time the timer starts, and the
/// case built on it would have been measuring branch throughput while claiming to measure
/// mispredicts. At 4096 the rate is about one mispredict every two dispatches, which is what a
/// chain of equality tests on a value it cannot guess is supposed to cost.
const STREAM: i128 = 4096;

/// How many times a dispatch case goes round its loop.
///
/// Ten million dispatches is a few tens of milliseconds of the shape a compiler emits when it
/// gets the switch right and a few hundred when it walks the labels one at a time. Both are far
/// enough above process startup that the wall clock in a record means something, and the whole
/// facet still fits in a minute at five levels against two compilers.
///
/// It is a whole number of passes over the stream, which is what lets the expected answer be a
/// multiple of one pass rather than a simulation of ten million steps.
const DISPATCHES: i128 = 10_240_000;

/// The multiplier and the addend of the generator that fills an unpredictable stream.
const LCG_MUL: u32 = 1_103_515_245;

/// The addend, per [`LCG_MUL`].
const LCG_ADD: u32 = 12_345;

/// The answers of the shape that is deliberately not a line.
///
/// The negative control for switch conversion. Half of these are one more than their position
/// and the rest are not, so no single multiple and offset gives all sixteen, and a pass looking
/// for a line has to check every one of them before it may refuse.
const SCATTER: &[i128] = &[7, 3, 91, 4, 55, 6, 2, 80, 9, 41, 11, 12, 63, 14, 15, 16];

/// The label range of one switch this facet dispatches.
#[derive(Clone, Copy)]
struct Shape {
    /// The axis point, which is also the name of the emitted function once its dash is gone.
    name: &'static str,
    /// The lowest label.
    base: i128,
    /// How many labels there are.
    labels: i128,
    /// How many values the stream draws from, counting up from `base`.
    ///
    /// Wider than the label count on purpose. A fifth of the dispatches miss every label and go
    /// to the default, so the range check a compiler writes in front of the arithmetic is a
    /// branch that is really taken sometimes rather than one that is never taken and therefore
    /// free. It is also what makes the default arm reachable evidence rather than dead code.
    span: i128,
}

/// The shapes, one program each.
const SHAPES: &[Shape] = &[
    Shape { name: "affine", base: 0, labels: 16, span: 20 },
    Shape { name: "scattered", base: 0, labels: 16, span: 20 },
    Shape { name: "constant-arms", base: 9, labels: 8, span: 10 },
    Shape { name: "below-zero", base: -8, labels: 16, span: 20 },
    Shape { name: "near-the-edge", base: 2_147_483_627, labels: 16, span: 20 },
    Shape { name: "shared-default", base: 0, labels: 16, span: 20 },
    Shape { name: "five-labels", base: 0, labels: 5, span: 7 },
    Shape { name: "six-labels", base: 0, labels: 6, span: 8 },
    Shape { name: "seven-labels", base: 0, labels: 7, span: 9 },
    Shape { name: "eight-labels", base: 0, labels: 8, span: 10 },
    Shape { name: "holes", base: 0, labels: 16, span: 36 },
    Shape { name: "wide-answers", base: 0, labels: 16, span: 20 },
    Shape { name: "negative-answers", base: 0, labels: 16, span: 20 },
    Shape { name: "masked", base: 0, labels: 16, span: 20 },
    Shape { name: "masked-four", base: 0, labels: 4, span: 20 },
];

/// The shapes that ask whether a switch becomes a table of its answers, and how wide the cells are.
///
/// Their answers are no line, so the only conversion left is a load from a table. `holes` has a
/// label at every other value, so a table has a cell nothing reads between each pair and a value
/// in a hole still has to reach the default. `wide-answers` needs four bytes a cell and
/// `negative-answers` fits a signed byte, which is what gcc 16 narrows a cell to at `-Os` and not
/// at `-O2`, so the pair says whether a compiler narrowed a cell it could and left alone one it
/// could not.
const TABLED: &[&str] = &["holes", "wide-answers", "negative-answers"];

/// The shapes whose switch is on the value masked to its labels, so the default is dead code.
///
/// Every value the mask can give is a label, and a compiler that can see that has no range check
/// to write and no default to keep. gcc 16 leaves both out. A compiler that cannot see it keeps a
/// compare in front of a table that every value is inside of, which is what tamnd/rucc#400 calls
/// removing an unreachable default. The stream still draws from a span wider than the labels, so
/// the values past the last label wrap round to the first ones rather than going anywhere new.
const MASKED: &[&str] = &["masked", "masked-four"];

/// The shapes that ask where a jump table starts to pay.
///
/// Dense, with answers no line fits, so the only question left is whether the switch is a table
/// or a set of comparisons. gcc 16 builds a table from five labels on x86-64 and rucc from eight,
/// and neither number was measured on the machine it runs on. Four sizes either side of both, on
/// a stream nothing predicts, is the measurement tamnd/rucc#1759 asks for.
const SMALL: &[&str] = &["five-labels", "six-labels", "seven-labels", "eight-labels"];

/// The shapes that are also dispatched on a stream a branch predictor can guess.
const PAIRED: &[&str] = &["affine", "scattered"];

/// A switch called often enough that how it was lowered shows up in the clock.
///
/// Every other switch facet is about size and about getting the right answer. This one is about
/// time, and it is the only place the corpus can see the thing document 24.3 says is the whole
/// reason to care about switch shape. A chain of equality tests on a value the hardware cannot
/// guess mispredicts about once every two dispatches, and that costs about half as much again
/// as the instructions themselves. A range check is one branch instead of eight and it is taken
/// the same way almost every time, so a compiler that reduces the switch to one comparison wins
/// more clock than it wins instructions, and no case that dispatches a switch a few dozen times
/// can tell you that.
///
/// The size of that second effect is worth being honest about. Going from the chain to the
/// range check cuts the instructions by half and the cycles by rather more than half, and only
/// part of the difference is mispredicts. That is why the two paired shapes exist: the same
/// switch on a stream the predictor learns and on one it does not, so the mispredict share can
/// be read off rather than assumed.
///
/// The dispatch value always comes out of an array that was filled from a `volatile` seed, and
/// the number of dispatches comes out of a `volatile` too. Either one left as a constant turns
/// the case into a question about folding.
fn switch_dispatch(sink: &mut Sink<'_>) {
    for shape in SHAPES {
        for &ordered in &[false, true] {
            // The predictable stream is only worth emitting for the two shapes that are paired
            // with each other. `affine` says what the conversion is worth, and `scattered` is
            // the same switch that no conversion can touch, so it keeps saying what a chain of
            // comparisons costs long after `affine` has stopped being one. Either pair on its
            // own is a claim that something got faster. The two together are the argument that
            // it got faster because the branches were being mispredicted, which is what the
            // facet is for, and a third shape does not make that argument any better.
            if ordered && !PAIRED.contains(&shape.name) {
                continue;
            }
            if !sink.wants(Facet::SwitchDispatch) {
                return;
            }
            let stream = if ordered { "in-order" } else { "unpredictable" };
            let mut program =
                Program::new(format!("the {} shape dispatched on {stream} values", shape.name));
            dispatch_switch(&mut program, shape);
            dispatch_loop(&mut program, shape, ordered);
            sink.push(
                Facet::SwitchDispatch,
                Axes::of([("shape", shape.name), ("stream", stream)]),
                Dialect::C17,
                program,
            );
        }
    }
    interpreter(sink);
    addresses(sink);
}

/// The words the `names` shapes return, one per label, and the one the default returns.
///
/// Each is at least four bytes long, so reading any of the first four is inside the string.
const WORDS: &[&str] = &[
    "hydrogen",
    "helium",
    "lithium",
    "beryllium",
    "boron",
    "carbon",
    "nitrogen",
    "oxygen",
    "fluorine",
    "neon",
    "sodium",
    "magnesium",
    "aluminium",
    "silicon",
    "phosphorus",
    "sulfur",
];

/// What the default of a `names` shape returns.
const NO_WORD: &str = "none";

/// The characters the `into-letters` shape returns pointers into.
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// How far into [`LETTERS`] the `into-letters` arm for a step points, with the default last.
///
/// Scattered, so no multiple and offset gives them all and the only lowering left is a table.
/// Every one is at least four short of the end, so the four bytes a dispatch reads are inside.
fn letter_offset(step: i128) -> i128 {
    if step < 0 { 58 } else { SCATTER[step as usize] * 5 % 56 }
}

/// The shapes whose arms return addresses rather than numbers.
///
/// Their spans, as in [`SHAPES`], are wider than their labels so the default is reached.
/// `names-with-holes` has a label at every other value, the way `holes` does.
const ADDRESSED: &[Shape] = &[
    Shape { name: "names", base: 0, labels: 16, span: 20 },
    Shape { name: "names-with-holes", base: 0, labels: 16, span: 36 },
    Shape { name: "into-letters", base: 0, labels: 16, span: 20 },
];

/// Which arm a value reaches in one of the [`ADDRESSED`] shapes, or `None` for the default.
fn addressed_step(shape: &Shape, value: i128) -> Option<i128> {
    let mut step = value - shape.base;
    if shape.name == "names-with-holes" {
        if step % 2 != 0 {
            return None;
        }
        step /= 2;
    }
    (0..shape.labels).contains(&step).then_some(step)
}

/// The byte `at` into the string one of the [`ADDRESSED`] shapes returns for a value.
fn addressed_byte(shape: &Shape, value: i128, at: usize) -> i128 {
    let step = addressed_step(shape, value);
    let text = if shape.name == "into-letters" {
        let from = letter_offset(step.unwrap_or(-1)) as usize;
        &LETTERS[from..]
    } else {
        step.map_or(NO_WORD, |step| WORDS[step as usize])
    };
    i128::from(text.as_bytes()[at])
}

/// Switches whose arms return the address of read only data, dispatched on a stream nothing
/// predicts.
///
/// A switch like this is how C names things: an error code to its message, an opcode to its
/// mnemonic, a state to its label. Its answers are no line and they are not numbers, so the only
/// table that can replace it is a table of addresses. gcc 16 builds one only with `-fno-pic`,
/// because anywhere else each cell is a relocation the loader has to fill in, and so in the
/// position independent executables every distribution builds by default it keeps the compares.
/// The other answer, which clang takes and tamnd/rucc#1775 asks for, is a table of how far each
/// answer is from the table, which is four bytes a cell, needs no relocation at load time, and
/// can stay in `.rodata`.
///
/// Each dispatch reads one of the first four bytes of what came back, picked by where the loop
/// is, so the address is really used. A dispatch that only ever read the first byte would let a
/// compiler turn the table of addresses into a table of characters, and then the case would be
/// about something else.
fn addresses(sink: &mut Sink<'_>) {
    for shape in ADDRESSED {
        if !sink.wants(Facet::SwitchDispatch) {
            return;
        }
        let mut program =
            Program::new(format!("the {} shape dispatched on unpredictable values", shape.name));
        let name = dispatch_name(shape);
        if shape.name == "into-letters" {
            program.top(format!("static const char letters[] = \"{LETTERS}\";"));
        }
        program.top(format!("static const char *{name}(int value) {{"));
        program.top("    switch (value) {");
        for step in 0..shape.labels {
            let label = if shape.name == "names-with-holes" { 2 * step } else { step };
            let answer = if shape.name == "into-letters" {
                format!("letters + {}", letter_offset(step))
            } else {
                format!("\"{}\"", WORDS[step as usize])
            };
            program.top(format!("    case {}: return {answer};", shape.base + label));
        }
        if shape.name == "into-letters" {
            program.top(format!("    default: return letters + {};", letter_offset(-1)));
        } else {
            program.top(format!("    default: return \"{NO_WORD}\";"));
        }
        program.top("    }");
        program.top("}");

        let values = dispatch_stream(shape, false);
        let per_pass: i128 = values
            .iter()
            .enumerate()
            .map(|(at, &value)| addressed_byte(shape, value, at & 3))
            .sum();
        program.input(Ty::I32, "seed", super::SEED);
        program.input(Ty::I32, "count", DISPATCHES);
        program.blank();
        program.line(format!("int stream[{STREAM}];"));
        program.line("unsigned int state = (unsigned int)seed;");
        program.line(format!("for (int at = 0; at < {STREAM}; at++) {{"));
        program.line_at(1, format!("state = state * {LCG_MUL}u + {LCG_ADD}u;"));
        program.line_at(1, format!("stream[at] = (int)((state >> 16) % {}u);", shape.span));
        program.line("}");
        program.blank();
        program.line("long long total = 0;");
        program.line("for (int at = 0; at < count; at++) {");
        program.line_at(1, format!("total += {name}(stream[at & {}])[at & 3];", STREAM - 1));
        program.line("}");
        program.blank();
        program.check(Ty::I64, "total", per_pass * (DISPATCHES / STREAM));
        sink.push(
            Facet::SwitchDispatch,
            Axes::of([("shape", shape.name), ("stream", "unpredictable")]),
            Dialect::C17,
            program,
        );
    }
}

/// The C name of a shape's function.
fn dispatch_name(shape: &Shape) -> String {
    shape.name.replace('-', "_")
}

/// The label a shape's arm at `step` answers for.
///
/// One apart for every shape but `holes`, whose labels are two apart so that every other value
/// in the range is a hole.
fn dispatch_label(shape: &Shape, step: i128) -> i128 {
    if shape.name == "holes" { shape.base + 2 * step } else { shape.base + step }
}

/// The answer one of these switches gives for a value, with zero for the default.
fn dispatch_answer(shape: &Shape, value: i128) -> i128 {
    if MASKED.contains(&shape.name) {
        return SCATTER[((value - shape.base) & (shape.labels - 1)) as usize];
    }
    let mut step = value - shape.base;
    if shape.name == "holes" {
        if step % 2 != 0 {
            return 0;
        }
        step /= 2;
    }
    if step < 0 || step >= shape.labels {
        return 0;
    }
    match shape.name {
        "affine" | "below-zero" => step + 1,
        // Three times the label rather than one times it, because with labels this close to the
        // top of `int` a multiple of three leaves the offset outside the type. A compiler that
        // works the offset out in wider arithmetic and then forgets to bring it back to the
        // width of the label gets this one wrong and gets the other shapes right.
        "near-the-edge" => 3 * step + 1,
        "constant-arms" => 1,
        "scattered" => SCATTER[step as usize],
        name if SMALL.contains(&name) => SCATTER[step as usize],
        name if TABLED.contains(&name) => match name {
            // Every answer over a million, so no cell narrower than four bytes holds them.
            "wide-answers" => SCATTER[step as usize] * 1_000_003,
            // Every answer below zero and above minus a hundred and twenty eight, so a byte holds
            // each one only if it is widened back with its sign.
            "negative-answers" => -SCATTER[step as usize],
            _ => SCATTER[step as usize],
        },
        // The last label has no arm of its own, so it goes wherever the default goes.
        "shared-default" => {
            if step == shape.labels - 1 {
                0
            } else {
                step + 1
            }
        }
        other => panic!("no answer is written for the {other} shape"),
    }
}

/// Writes the switch a dispatch case calls, one arm per label.
fn dispatch_switch(program: &mut Program, shape: &Shape) {
    program.top(format!("static int {}(int value) {{", dispatch_name(shape)));
    if MASKED.contains(&shape.name) {
        program.top(format!("    switch (value & {}) {{", shape.labels - 1));
    } else {
        program.top("    switch (value) {");
    }
    for step in 0..shape.labels {
        let label = dispatch_label(shape, step);
        if shape.name == "shared-default" && step == shape.labels - 1 {
            // No body and no `break`, so this label falls into the default and the block the
            // default runs is also the block this label goes to. That is the shape that makes a
            // predecessor count of one a lie: the arm looks singly reached, and a pass that
            // deletes it on that basis takes the default with it.
            program.top(format!("    case {label}:"));
            continue;
        }
        program.top(format!("    case {label}: return {};", dispatch_answer(shape, label)));
    }
    program.top("    default: return 0;");
    program.top("    }");
    program.top("}");
}

/// The values a stream holds, in the order the fill writes them.
///
/// The unpredictable fill takes bits sixteen and up of a linear congruential generator rather
/// than the low bits, because the low bits of one of these have short periods and a short period
/// is exactly what makes a branch predictable. This is the same arithmetic the emitted C does,
/// written twice on purpose, because the expected answer has to come from the generator rather
/// than from a reference compiler.
fn dispatch_stream(shape: &Shape, ordered: bool) -> Vec<i128> {
    let span = u32::try_from(shape.span).expect("a span is small and positive");
    let mut state = u32::try_from(super::SEED).expect("the seed is small and positive");
    (0..STREAM)
        .map(|at| {
            let step = if ordered {
                (at + super::SEED) % shape.span
            } else {
                state = state.wrapping_mul(LCG_MUL).wrapping_add(LCG_ADD);
                i128::from((state >> 16) % span)
            };
            shape.base + step
        })
        .collect()
}

/// Writes the fill, the dispatch loop and the check on what the loop added up to.
fn dispatch_loop(program: &mut Program, shape: &Shape, ordered: bool) {
    let values = dispatch_stream(shape, ordered);
    let per_pass: i128 = values.iter().map(|&value| dispatch_answer(shape, value)).sum();
    program.input(Ty::I32, "seed", super::SEED);
    program.input(Ty::I32, "count", DISPATCHES);
    program.blank();
    // Nothing is added when the labels start at zero, so the emitted C reads the way somebody
    // would have written it rather than the way the generator happened to build it.
    let from = if shape.base == 0 { String::new() } else { format!("{} + ", shape.base) };
    program.line(format!("int stream[{STREAM}];"));
    if ordered {
        program.line(format!("for (int at = 0; at < {STREAM}; at++) {{"));
        program.line_at(1, format!("stream[at] = {from}(at + seed) % {};", shape.span));
    } else {
        program.line("unsigned int state = (unsigned int)seed;");
        program.line(format!("for (int at = 0; at < {STREAM}; at++) {{"));
        program.line_at(1, format!("state = state * {LCG_MUL}u + {LCG_ADD}u;"));
        program.line_at(1, format!("stream[at] = {from}(int)((state >> 16) % {}u);", shape.span));
    }
    program.line("}");
    program.blank();
    program.line("long long total = 0;");
    program.line("for (int at = 0; at < count; at++) {");
    program.line_at(1, format!("total += {}(stream[at & {}]);", dispatch_name(shape), STREAM - 1));
    program.line("}");
    program.blank();
    program.check(Ty::I64, "total", per_pass * (DISPATCHES / STREAM));
}

/// How many opcodes the interpreter has, counting the one that falls to the default.
const OPCODES: i128 = 9;

/// What each of the interpreter's arms does to the accumulator.
///
/// Every arm is real work rather than a constant, so switch conversion cannot apply and this
/// case is the one that stays a switch however good the middle end gets. It is the benchmark
/// document 24.7 names, and it is where the jump table and the bit test will have to prove
/// themselves once they are written.
///
/// Every arm is invertible in the accumulator, and that is the rule rather than a nicety.
///
/// If an arm can lose a bit then the answer can stop depending on what came before it, and a run
/// that dispatched a million steps wrongly would still print the right number. An `acc & operand`
/// with an operand under two hundred and fifty six is the obvious way to get that wrong, but it
/// is not the only one: an earlier draft of this list had `(acc << 3) ^ operand` and
/// `(acc >> 5) | operand`, neither of which masks anything, and the two of them together still
/// walked a sixty four bit accumulator down to fifteen bits over ten million steps. So the shifts
/// here are rotates, which keep every bit they are given, and the multiply is by an odd number,
/// which is invertible on a wrapping word. A composition of invertible steps is invertible, so
/// two runs that started differently can never meet.
const STEPS: &[&str] = &[
    "acc + operand",
    "acc - operand",
    "acc ^ operand",
    "((acc << 3) | (acc >> 61)) ^ operand",
    "((acc >> 5) | (acc << 59)) + operand",
    "acc + (operand << 1)",
    "acc - (operand >> 1)",
    "acc * 3 + 1",
];

/// A bytecode interpreter, which is the shape a hot switch really has.
///
/// The accumulator carries from one step to the next, so the loop is bound by the latency of
/// the dispatch rather than by how many of them the machine can have in flight. That is what an
/// interpreter is actually like and it is what makes a mispredicted indirect branch cost what
/// document 24.3 says it costs. Everything is unsigned, so every arm is defined for every
/// accumulator value including the ones that have wrapped.
fn interpreter(sink: &mut Sink<'_>) {
    if !sink.wants(Facet::SwitchDispatch) {
        return;
    }
    let mut program = Program::new("a bytecode interpreter dispatching on an opcode stream");
    program.top("static unsigned long long step(int op, unsigned long long acc,");
    program.top("                               unsigned int operand) {");
    program.top("    switch (op) {");
    for (op, work) in STEPS.iter().enumerate() {
        program.top(format!("    case {op}: return {work};"));
    }
    program.top("    default: return acc;");
    program.top("    }");
    program.top("}");

    program.input(Ty::I32, "seed", super::SEED);
    program.input(Ty::I32, "count", DISPATCHES);
    program.blank();
    program.line(format!("unsigned char ops[{STREAM}];"));
    program.line(format!("unsigned int args[{STREAM}];"));
    program.line("unsigned int state = (unsigned int)seed;");
    program.line(format!("for (int at = 0; at < {STREAM}; at++) {{"));
    program.line_at(1, format!("state = state * {LCG_MUL}u + {LCG_ADD}u;"));
    program.line_at(1, format!("ops[at] = (unsigned char)((state >> 16) % {OPCODES}u);"));
    program.line_at(1, "args[at] = (state >> 8) & 255u;");
    program.line("}");
    program.blank();
    program.line("unsigned long long acc = 1;");
    program.line("for (int at = 0; at < count; at++) {");
    program.line_at(
        1,
        format!("acc = step(ops[at & {}], acc, args[at & {}]);", STREAM - 1, STREAM - 1),
    );
    program.line("}");
    program.blank();
    program.check(Ty::U64, "acc", interpreter_answer());

    sink.push(
        Facet::SwitchDispatch,
        Axes::of([("shape", "interpreter"), ("stream", "unpredictable")]),
        Dialect::C17,
        program,
    );
}

/// What the interpreter's accumulator holds when the loop stops.
///
/// Ten million steps of `u64` arithmetic, run here so the case ships with its answer. The
/// accumulator carries, so unlike every other shape in this facet there is no shortcut from one
/// pass over the stream to the whole run.
fn interpreter_answer() -> i128 {
    i128::from(interpreter_run(1))
}

/// The interpreter's loop, from a starting accumulator, which is what the emitted C does.
///
/// Taking the start as an argument is what lets a test ask whether the answer still depends on
/// it after ten million steps. If it does not, some arm is throwing the accumulator away and
/// the case has stopped being an oracle, whatever number it prints.
fn interpreter_run(start: u64) -> u64 {
    let opcodes = u32::try_from(OPCODES).expect("the opcode count is small and positive");
    let mut state = u32::try_from(super::SEED).expect("the seed is small and positive");
    let mut ops = Vec::new();
    let mut args = Vec::new();
    for _ in 0..STREAM {
        state = state.wrapping_mul(LCG_MUL).wrapping_add(LCG_ADD);
        ops.push(((state >> 16) % opcodes) as usize);
        args.push(u64::from((state >> 8) & 255));
    }
    let mask = (STREAM - 1) as usize;
    let mut acc: u64 = start;
    for at in 0..DISPATCHES as usize {
        let operand = args[at & mask];
        acc = match ops[at & mask] {
            0 => acc.wrapping_add(operand),
            1 => acc.wrapping_sub(operand),
            2 => acc ^ operand,
            3 => acc.rotate_left(3) ^ operand,
            4 => acc.rotate_right(5).wrapping_add(operand),
            5 => acc.wrapping_add(operand << 1),
            6 => acc.wrapping_sub(operand >> 1),
            7 => acc.wrapping_mul(3).wrapping_add(1),
            _ => acc,
        };
    }
    acc
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

/// A widening whose upper bits nothing ever reads.
///
/// C promotes every narrow operand to `int` before doing anything with it, so a program
/// written in `char` is a program full of moves between widths. Most of those sit next to
/// each other and a peephole rule takes the pair. What is left is the ones with a block
/// boundary in the middle, where whether the bits the widening worked out matter is a
/// question about every reader of the result rather than about the instruction on the next
/// line. Answering it takes a count of how much of a register anything reads, which is bit
/// group liveness. tamnd/rucc#1131.
///
/// Five of the seven shapes put the widening and its reader in different blocks, which is
/// where a peephole cannot see the pair. `in-another-block` has one narrow reader on the far
/// side of an `if`, `in-a-loop` puts it behind a back edge, `both-arms` widens in two arms and
/// reads the low bits at the join, so the answer has to travel along an edge rather than down
/// a block, `chain-of-widths` goes up twice and comes back down in one step, and
/// `stored-narrow` has a store for its only reader, which is the reader a pass has to be most
/// careful about because a store is where a value stops being the compiler's business.
///
/// `narrowed-from-a-double` is the other way a conversion gets out of reach, and it does not
/// need a branch at all. A conversion out of a double lands in a register at the machine's
/// width, so the narrowing to the type the program asked for is written by the selector and
/// exists in the machine IR and nowhere above it.
///
/// `upper-bits-read` is the one that catches a count that is wrong rather than one that is
/// missing. Something on the far side of the branch reads the whole widened value, and the
/// operand has the top bit of its own type set, so a compiler that decided nothing read the
/// upper bits prints a different number rather than a smaller program.
fn bit_liveness(sink: &mut Sink<'_>) {
    const NARROW: &[Ty] = &[Ty::I8, Ty::U8, Ty::I16, Ty::U16];
    const SHAPES: &[&str] = &[
        "in-another-block",
        "in-a-loop",
        "both-arms",
        "chain-of-widths",
        "stored-narrow",
        "narrowed-from-a-double",
        "upper-bits-read",
    ];
    for &ty in NARROW {
        for &shape in SHAPES {
            if !sink.wants(Facet::BitLiveness) {
                return;
            }
            let name = ty.c_name();
            let phrase = match shape {
                "in-another-block" => "whose only reader is in another block",
                "in-a-loop" => "whose only reader is in a loop",
                "both-arms" => "widened in both arms and read narrow at the join",
                "chain-of-widths" => "widened twice and read back at its own width",
                "stored-narrow" => "whose only reader is a store at its own width",
                "narrowed-from-a-double" => "narrowed out of a double by the selector",
                _ => "read whole as well as narrow",
            };
            let mut program = Program::new(format!("a widened {name} {phrase}"));
            let value: i128 = match shape {
                // One narrow reader, on the far side of a branch. The plainest case there is
                // and the one no rule can reach, since the two instructions are in two blocks.
                "in-another-block" => {
                    program.input(ty, "a", 40);
                    program.input(Ty::I32, "flag", 1);
                    program.blank();
                    program.line("int wide = a;");
                    program.line(format!("{name} low = {};", lit(ty, 0)));
                    program.line("if (flag) {");
                    program.line_at(1, format!("low = ({name})wide;"));
                    program.line("}");
                    program.line(format!("{name} total = low;"));
                    40
                }
                // The same question with a back edge in it. The widening is outside the loop
                // and the only thing that reads it is inside, so the count has to come back
                // round the loop before it settles.
                "in-a-loop" => {
                    program.input(ty, "a", 5);
                    program.input(Ty::I32, "count", 3);
                    program.blank();
                    program.line("int wide = a;");
                    program.line(format!("{name} total = {};", lit(ty, 0)));
                    program.line("for (int i = 0; i < count; i = i + 1) {");
                    program.line_at(1, format!("total = ({name})(total + ({name})wide);"));
                    program.line("}");
                    15
                }
                // Two widenings and one reader. Neither arm has the reader in it, so what
                // carries the answer is the edge into the block they both go to.
                "both-arms" => {
                    program.input(ty, "a", 40);
                    program.input(ty, "b", 15);
                    program.input(Ty::I32, "flag", 1);
                    program.blank();
                    program.line("int wide;");
                    program.line("if (flag) {");
                    program.line_at(1, "wide = a;");
                    program.line("} else {");
                    program.line_at(1, "wide = b;");
                    program.line("}");
                    program.line(format!("{name} total = ({name})wide;"));
                    40
                }
                // Up to `int`, up to `long long`, and back down to where it started in one
                // step. Nothing goes unless the count is followed through both widenings.
                "chain-of-widths" => {
                    program.input(ty, "a", 40);
                    program.input(Ty::I32, "flag", 1);
                    program.blank();
                    program.line("int wide = a;");
                    program.line("long long wider = wide;");
                    program.line(format!("{name} low = {};", lit(ty, 0)));
                    program.line("if (flag) {");
                    program.line_at(1, format!("low = ({name})wider;"));
                    program.line("}");
                    program.line(format!("{name} total = low;"));
                    40
                }
                // A store for the only reader. How much of a register a store reads is the one
                // thing a pass here has to get from the target rather than work out, since a
                // store that writes more than it was asked to writes over somebody else.
                "stored-narrow" => {
                    program.input(ty, "a", 40);
                    program.input(Ty::I32, "flag", 1);
                    program.blank();
                    program.line(format!("{name} buffer[4] = {{ 0 }};"));
                    program.line("int wide = a;");
                    program.line("if (flag) {");
                    program.line_at(1, format!("buffer[2] = ({name})wide;"));
                    program.line("}");
                    program.line(format!("{name} total = buffer[2];"));
                    40
                }
                // A narrowing the selector wrote rather than one anybody asked for. A
                // conversion out of a double lands in a register at the machine's width and
                // the value wanted is narrower, so the narrowing exists in the machine IR and
                // nowhere above it, which puts it out of reach of every rewrite rule by
                // construction rather than by luck.
                "narrowed-from-a-double" => {
                    program.top("static volatile double source_in = 40.75;");
                    program.input(Ty::I32, "flag", 1);
                    program.blank();
                    program.line("double source = source_in;");
                    program.line(format!("{name} total = ({name})source;"));
                    program.line("if (flag) {");
                    program.line_at(1, format!("total = ({name})(source + 1.0);"));
                    program.line("}");
                    41
                }
                // The one that catches a count that is wrong rather than one that is missing.
                // The comparison reads the widened value whole,
                // and the operand has the top bit of its own type set, so it answers one way
                // with the widening still there and the other way without it. The test is
                // against zero on a signed type and against the largest value a signed byte
                // holds on an unsigned one, because a value read out of an unsigned type is
                // never below zero and a comparison saying so would be folded away before any
                // of this.
                _ => {
                    let start: i128 = if ty.signed() { -56 } else { 200 };
                    program.input(ty, "a", start);
                    program.input(Ty::I32, "flag", 1);
                    program.blank();
                    program.line("int wide = a;");
                    program.line(format!("{name} low = {};", lit(ty, 0)));
                    program.line("int beyond = 0;");
                    program.line("if (flag) {");
                    program.line_at(1, format!("low = ({name})wide;"));
                    let test = if ty.signed() { "wide < 0" } else { "wide > 127" };
                    program.line_at(1, format!("beyond = {test};"));
                    program.line("}");
                    program.line(format!("{name} total = ({name})(low + ({name})beyond);"));
                    if ty.signed() { -55 } else { 201 }
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", value);
            sink.push(
                Facet::BitLiveness,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A comparison the machine has already made.
///
/// A comparison computes nothing. It sets a few bits nobody named and the instruction behind
/// it reads them, so a comparison that sets the bits already sitting there is one nothing
/// could tell had run. Two shapes put them there. The same comparison made twice, which is
/// what an expression that asks a question and then asks its negation comes out as, and a
/// comparison against zero of a value arithmetic has just worked out, which is what
/// `if (a & MASK)` comes out as, because the `and` set those bits on its way past.
/// tamnd/rucc#1132.
///
/// A facet of its own rather than a shape of `machine-peephole`, because half of what decides
/// it is which conditions read what the arithmetic left, and that is a fact about the machine
/// rather than about the pair of instructions. On x86 an `and` clears the carry and the
/// overflow and sets the zero and the sign from what it wrote, which is everything a
/// comparison against zero would have set and the same values, so every condition may read
/// it. A `sub` agrees about the zero bit alone, because a subtraction that overflowed says so
/// where the comparison would have said it did not.
///
/// `after-sub-ordered` is the one that has to stay, and it is emitted for the signed types
/// only. That is not a gap. An unsigned value is never below zero and is always at or above
/// it, so the only comparison against zero an unsigned type has is an equality, which is
/// exactly the condition a subtraction does answer. There is no unsigned program to write.
///
/// `written-in-between` is the case the issue asks for by name. The `and` leaves the bits of
/// a value that is not zero, then a plain move puts a zero in the same variable, and a move
/// touches no condition bits at all, so a compiler that read the `and`'s answer for the
/// comparison behind the move prints a different number rather than a smaller program.
fn compare_elim(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "same-comparison",
        "byte-and-branch",
        "after-and",
        "after-or",
        "after-sub",
        "after-sub-ordered",
        "written-in-between",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::CompareElim) {
                return;
            }
            // An unsigned value is never below zero, so the shape that is about an ordered
            // comparison against zero has nothing to say about an unsigned type.
            if shape == "after-sub-ordered" && !ty.signed() {
                continue;
            }
            let name = ty.c_name();
            let phrase = match shape {
                "same-comparison" => "compared with the same pair twice",
                "byte-and-branch" => "compared once for a value and once for a branch",
                "after-and" => "masked and then asked whether it is zero",
                "after-or" => "joined and then asked whether it is zero",
                "after-sub" => "subtracted and then asked whether the answer is zero",
                "after-sub-ordered" => "subtracted and then asked which side of zero it is on",
                _ => "masked, written over, and then asked whether it is zero",
            };
            let mut program = Program::new(format!("a {name} {phrase}"));
            let value: i128 = match shape {
                // The shape the issue is named after. Two comparisons of the same pair with
                // nothing between them but the byte each one kept, which is what a program
                // that asks a question and then asks its negation comes out as.
                "same-comparison" => {
                    program.input(ty, "a", 40);
                    program.input(ty, "b", 15);
                    program.blank();
                    program.line("int same = (a == b);");
                    program.line("int differ = (a != b);");
                    program.line("int total = same * 4 + differ;");
                    1
                }
                // The same pair again, with one of the two comparisons feeding a branch
                // instead of a value. After the block layout has folded the branch into it
                // that one keeps no byte at all, so the two comparisons are the two shapes a
                // comparison comes in and the pass has to see them as one thing.
                "byte-and-branch" => {
                    program.input(ty, "a", 40);
                    program.input(ty, "b", 15);
                    program.blank();
                    program.line("int total = (a == b);");
                    program.line("if (a != b) {");
                    program.line_at(1, "total = total + 7;");
                    program.line("}");
                    7
                }
                // The common one by a long way, and the reason this is worth a pass. The `and`
                // already said whether its answer was zero, so the comparison behind it is
                // asking a question that has been answered.
                "after-and" => {
                    program.input(ty, "a", 40);
                    program.input(ty, "mask", 24);
                    program.blank();
                    program.line(format!("{name} masked = ({name})(a & mask);"));
                    program.line("int total = 0;");
                    program.line("if (masked) {");
                    program.line_at(1, "total = total + 3;");
                    program.line("}");
                    program.line("total = total + (int)masked * 2;");
                    19
                }
                // The same claim about the other two bitwise operations, since what makes them
                // safe is a fact about all three and a table that named only one of them would
                // be a table somebody had stopped filling in.
                "after-or" => {
                    program.input(ty, "a", 40);
                    program.input(ty, "b", 3);
                    program.blank();
                    program.line(format!("{name} joined = ({name})(a | b);"));
                    program.line("int total = (joined != 0) ? 5 : 9;");
                    program.line("total = total + (int)(joined & 7);");
                    8
                }
                // A subtraction and the one question a subtraction answers. Whether the answer
                // was zero is what the zero bit says either way, so this goes.
                "after-sub" => {
                    program.input(ty, "a", 40);
                    program.input(ty, "b", 40);
                    program.blank();
                    program.line(format!("{name} d = ({name})(a - b);"));
                    program.line("int total = (d == 0) ? 6 : 9;");
                    program.line("total = total + (int)d;");
                    6
                }
                // A subtraction and a question it does not answer. A comparison against zero
                // would have said nothing overflowed and the subtraction says whether it did,
                // so a signed `<` after one reads a sign and an overflow that no longer belong
                // together, and the comparison has to stay.
                "after-sub-ordered" => {
                    program.input(ty, "a", 15);
                    program.input(ty, "b", 40);
                    program.blank();
                    program.line(format!("{name} d = ({name})(a - b);"));
                    program.line("int total = (d < 0) ? 4 : 8;");
                    program.line("total = total + (int)(d + 30);");
                    9
                }
                // The one the issue asks for by name. The bits are still the bits the `and`
                // left and they are about what the variable held then, not what it holds now,
                // and the move in between is an instruction that touches no condition bits, so
                // nothing else stands between a compiler and the wrong answer.
                _ => {
                    program.input(ty, "a", 40);
                    program.input(ty, "mask", 24);
                    program.input(ty, "spare", 0);
                    program.blank();
                    program.line(format!("{name} value = ({name})(a & mask);"));
                    program.line("int first = (value != 0);");
                    program.line("value = spare;");
                    program.line("int second = (value == 0);");
                    program.line("int total = first * 10 + second;");
                    11
                }
            };
            program.blank();
            program.check(Ty::I32, "total", value);
            sink.push(
                Facet::CompareElim,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// One address and the instructions that read it.
///
/// A machine that can put a base, an index and a displacement in a memory operand can either
/// work an address out into a register once and have every reader name that register, or leave
/// the address where it is and have every reader carry the whole of it. Which is cheaper is not
/// a question about one reader. Folding into some of a set and not the rest leaves the address
/// computation there for the rest, so the folds bought nothing and the registers the address
/// reads are live across instructions that no longer read them.
///
/// So the shapes here vary the thing that decides it: how many readers there are, whether they
/// are all in one block, whether the base survives from the address to the last of them, and
/// whether the address is a register or a symbol, which is the case where every reader that
/// takes it writes a whole address word rather than a register number.
fn address_fold(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "several-offsets",
        "store-then-load",
        "computed-index",
        "base-written-between",
        "reader-in-another-block",
        "filled-then-read",
        "global-many-readers",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::AddressFold) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("one {name} address, {shape}"));
            program.input(ty, "seed", 5);
            if matches!(shape, "store-then-load" | "computed-index" | "reader-in-another-block") {
                program.input(Ty::I32, "at", 3);
            }
            if shape == "reader-in-another-block" {
                program.input(Ty::I32, "flag", 1);
            }
            program.blank();
            let value: i128 = match shape {
                // Three readers of one address at three displacements, which is the shape the
                // pass exists for and the one every reader can take.
                "several-offsets" => {
                    program.line(format!("{name} buffer[8] = {{ 0 }};"));
                    program.line(format!("{name} *q = &buffer[0];"));
                    program.line("q[0] = seed;");
                    program.line(format!("q[3] = seed + {};", lit(ty, 1)));
                    program.line(format!("q[7] = seed + {};", lit(ty, 2)));
                    program.line(format!("{name} total = q[0] + q[3] + q[7];"));
                    18
                }
                // Two readers of one address, and they are not the same kind of instruction.
                // Value numbering is what gives the store and the load one address rather than
                // two, so this is the smallest case that depends on both passes.
                "store-then-load" => {
                    program.line(format!("{name} buffer[8] = {{ 0 }};"));
                    program.line(format!("buffer[at] = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} total = buffer[at];"));
                    6
                }
                // The same, with the address made of a base and a scaled index rather than a
                // base and a number, which is the other thing a memory operand has room for.
                "computed-index" => {
                    program.line(format!("{name} buffer[8] = {{ 0 }};"));
                    program.line("buffer[at] = seed;");
                    program.line(format!("buffer[at + 1] = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} total = buffer[at] + buffer[at + 1];"));
                    11
                }
                // The base is written between the two readers, so the address the second one
                // would carry is not the address the first one read. Nothing may move.
                "base-written-between" => {
                    program.line(format!("{name} buffer[8] = {{ 0 }};"));
                    program.line(format!("{name} *q = &buffer[2];"));
                    program.line("q[0] = seed;");
                    program.line("q = &buffer[5];");
                    program.line(format!("q[0] = seed + {};", lit(ty, 1)));
                    program.line(format!("{name} total = buffer[2] + buffer[5];"));
                    11
                }
                // One of the readers is behind a branch, so the set is not all in one block and
                // a decision taken a block at a time cannot see the whole of it.
                "reader-in-another-block" => {
                    program.line(format!("{name} buffer[8] = {{ 0 }};"));
                    program.line(format!("{name} *q = &buffer[at];"));
                    program.line("q[0] = seed;");
                    program.line("if (flag) {");
                    program.line_at(1, format!("q[1] = seed + {};", lit(ty, 1)));
                    program.line("}");
                    program.line(format!("{name} total = buffer[3] + buffer[4];"));
                    11
                }
                // An array filled in one loop and read back in the next. Both loops unroll, and
                // every element then has one address with a store reading it and a load reading
                // it, which is eight sets of two rather than one set of eight.
                "filled-then-read" => {
                    program.line(format!("{name} grid[8];"));
                    program.line("for (int i = 0; i < 8; i++) {");
                    program.line_at(1, format!("grid[i] = seed + ({name})i;"));
                    program.line("}");
                    program.line(format!("{name} total = 0;"));
                    program.line("for (int i = 0; i < 8; i++) {");
                    program.line_at(1, "total += grid[i];");
                    program.line("}");
                    68
                }
                // A global rather than a local, read four times. The address of a global is not
                // a register, so a reader that takes it writes the symbol itself, and four
                // readers write it four times to save one instruction that wrote it once.
                _ => {
                    program.top(format!("static {name} shared[4];"));
                    program.line("shared[0] = seed;");
                    program.line(format!("shared[1] = seed + {};", lit(ty, 1)));
                    program.line(format!("shared[2] = seed + {};", lit(ty, 2)));
                    program.line(format!("shared[3] = seed + {};", lit(ty, 3)));
                    program.line(format!(
                        "{name} total = shared[0] + shared[1] + shared[2] + shared[3];"
                    ));
                    26
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", value);
            sink.push(
                Facet::AddressFold,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A load one arithmetic instruction reads, and everything that stops it moving.
///
/// Most machines can read one source of an addition out of memory, so a load whose value a single
/// addition reads is two instructions where the machine has one. Doing it means the load stops
/// being where it was and becomes part of an instruction further down, and that is what these
/// shapes are about: the conditions are not facts about the pair, they are facts about everything
/// between the two and about everybody else who wanted the value.
///
/// So the axis is the reason rather than the operator. One reader against two, because a second
/// reader means the value has to exist in a register anyway and the load is not saved. A store, a
/// call and another load in the way. A register the address reads written in between. A reader in
/// another block. The left of a subtraction, which is the operand that cannot come out of memory
/// on a machine whose subtraction writes its first source.
///
/// The load in the way is the shape worth having and the one a back end gets wrong. Moving a read
/// past a read reorders two accesses, and by the time a machine level pass can see them it cannot
/// tell a `volatile` read from an ordinary one, because nothing in the instruction says which the
/// program insisted on. So the case is two `volatile` reads and a subtraction of one from the
/// other, which is a program whose answer is the same either way and whose observable behaviour
/// is not.
fn load_fold(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "one-reader",
        "two-readers",
        "store-between",
        "call-between",
        "load-between",
        "index-written-between",
        "subtract-left",
        "subtract-right",
        "reader-in-another-block",
        "narrower-load",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::LoadFold) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("one {name} load, {shape}"));
            if shape == "load-between" {
                // No array and no index. The whole of this shape is two reads whose order the
                // program fixed and one subtraction that reads the earlier of them.
                program.input(ty, "first", 4);
                program.input(ty, "second", 9);
            } else {
                program.input(ty, "seed", 5);
                program.input(Ty::I32, "at", 3);
            }
            if matches!(shape, "store-between" | "index-written-between") {
                program.input(Ty::I32, "other", 6);
            }
            if shape == "reader-in-another-block" {
                program.input(Ty::I32, "flag", 1);
            }
            if shape == "call-between" {
                // A call the compiler cannot see through and cannot delete. The body touches a
                // volatile global so an empty function is not what gets inlined, and the pointer
                // is volatile so the call cannot be turned back into a direct one.
                program.top("static volatile int noise_count;");
                program.top("static void noise(void) {");
                program.top("    noise_count = noise_count + 1;");
                program.top("}");
                program.top("static void (*volatile jump)(void) = noise;");
            }
            program.blank();
            if !matches!(shape, "load-between" | "narrower-load") {
                // Filled in a loop and then read at an index nothing knows, which is what makes
                // the read a load. An array written at one index and read back at the same one
                // is a store the optimizer hands straight to the reader, and then there is no
                // load for this pass to have an opinion about. Every buffer shape here reads
                // `buffer[at]`, and every one of the eight stores in front of it is a store the
                // compiler cannot rule out as the source, so the load stays.
                program.line(format!("{name} buffer[8];"));
                program.line("for (int i = 0; i < 8; i++) {");
                program.line_at(1, format!("buffer[i] = seed + ({name})i;"));
                program.line("}");
            }
            let value: i128 = match shape {
                // One load, one reader, and the reader is an addition. This is the shape the pass
                // exists for and the only one here it is meant to change.
                "one-reader" => {
                    program.line(format!("{name} total = seed + buffer[at];"));
                    13
                }
                // Two readers of the loaded value. The value has to be in a register for the
                // second of them whatever happens, so putting the load inside the first buys an
                // addressing mode and keeps the load, which is worse than leaving it alone.
                "two-readers" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line(format!(
                        "{name} total = (seed + value) + (value & {});",
                        lit(ty, 12)
                    ));
                    21
                }
                // A store between the load and its reader, to an index the compiler has no way to
                // tell apart from the one that was read. Whether the two are the same place is a
                // question about two addresses, so the answer is to leave the load where it is.
                "store-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("buffer[other] = seed;");
                    program.line(format!("{name} total = seed + value;"));
                    // Nothing else reads the array, so without this the store is dead and a
                    // compiler that removes dead stores removes the whole of the shape. The read
                    // is after the reader rather than before it, so it is not itself something
                    // the load had to pass.
                    program.sink("buffer[0]");
                    13
                }
                // A call between the two. What a call does to memory is not written in the call,
                // so it is asked separately from whether an instruction has a memory operand.
                "call-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("jump();");
                    program.line(format!("{name} total = seed + value;"));
                    13
                }
                // Two volatile reads and a subtraction reading the earlier one. Folding the read
                // of `first` into the subtraction puts it after the read of `second`, which is an
                // order this program fixed. Nothing in the instructions says so, which is the
                // point: the rule has to be that no read moves past another access at all.
                "load-between" => {
                    program.line(format!("{name} total = second - first;"));
                    5
                }
                // The index the address reads is written between the load and the reader, so the
                // address the folded instruction would carry is not the address that was read.
                "index-written-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("at = other;");
                    program.line(format!("{name} total = value + buffer[at];"));
                    19
                }
                // The load feeds the left of a subtraction. On a machine whose subtraction writes
                // its first source, that operand is the destination and cannot come out of memory,
                // so this is the arithmetic the pass has to check the side of rather than fold.
                "subtract-left" => {
                    program.line(format!("{name} total = buffer[at] - seed;"));
                    3
                }
                // The same subtraction the other way round, where the load is the operand that
                // can come out of memory. The pair is here so the two answers sit next to each
                // other and a report showing the same instruction count for both is a finding.
                "subtract-right" => {
                    program.line(format!("{name} total = (seed + {}) - buffer[at];", lit(ty, 9)));
                    6
                }
                // The reader is behind a branch, so the load and the instruction that would take
                // it in are not in the same block. A pass that works a block at a time stops here
                // whether or not the branch is taken.
                "reader-in-another-block" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line(format!("{name} total = seed;"));
                    program.line("if (flag) {");
                    program.line_at(1, "total = seed + value;");
                    program.line("}");
                    13
                }
                // A byte load feeding arithmetic at the width of the case. The load and the
                // addition are the same width only when the case is about `unsigned char`, and
                // every other point on this axis is a pair whose widths disagree, which is a fold
                // that would change the answer rather than one that costs nothing.
                _ => {
                    program.line("unsigned char bytes[8];");
                    program.line("for (int i = 0; i < 8; i++) {");
                    program.line_at(1, "bytes[i] = (unsigned char)(seed + i);");
                    program.line("}");
                    program.line(format!("{name} total = seed + bytes[at];"));
                    13
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", value);
            sink.push(
                Facet::LoadFold,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A load, arithmetic on what came back, and a store of the answer to the same place.
///
/// Three instructions rather than two, and one instruction on a machine that can write memory
/// with its arithmetic. What makes this a facet of its own rather than the shapes above with the
/// operands the other way about is the subtraction. `buffer[at] -= seed` computes the memory
/// minus the register, which is what the one instruction computes, and `(seed + 20) - buffer[at]`
/// computes the register minus the memory, which it does not. Fold the load into the subtraction
/// first and the two read alike, so the pair is here to say that the three have to be read as
/// instruction selection wrote them.
///
/// The rest of the shapes are the reasons three instructions are not one: the loaded word read
/// twice, the answer read by somebody besides the store, a store in the way, a call in the way, a
/// store to another place, a store at another displacement off the same address, and an index
/// written in between. Each of them is a program whose answer a back end that folds anyway gets
/// wrong.
///
/// The two frame slots are the shape worth having. Two locals the layout has not placed yet are
/// written down as the same distance from the stack pointer, so a back end comparing the
/// addresses it can see calls them one place. They are two, and the program says which.
fn store_fold(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "add",
        "subtract",
        "subtract-reversed",
        "bitwise-or",
        "bitwise-and",
        "exclusive-or",
        "two-readers",
        "answer-read",
        "store-between",
        "call-between",
        "other-place",
        "other-offset",
        "index-written-between",
        "frame-slots",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::StoreFold) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("one {name} read and write, {shape}"));
            program.input(ty, "seed", 5);
            if shape != "frame-slots" {
                program.input(Ty::I32, "at", 3);
                // The index the answer is read back at. It holds what `at` holds, and nothing
                // says so, which is what keeps the store alive: an array written at an index and
                // read at the same index is a store the optimizer hands straight to the reader,
                // and then the case has no store in it at all.
                program.input(Ty::I32, "back", 3);
            }
            if matches!(shape, "store-between" | "other-place" | "index-written-between") {
                program.input(Ty::I32, "other", 6);
            }
            if shape == "bitwise-and" {
                program.input(ty, "mask", 12);
            }
            if shape == "call-between" {
                // A call the compiler cannot see through and cannot delete, the same one the
                // load fold cases use and for the same reason.
                program.top("static volatile int noise_count;");
                program.top("static void noise(void) {");
                program.top("    noise_count = noise_count + 1;");
                program.top("}");
                program.top("static void (*volatile jump)(void) = noise;");
            }
            if shape == "frame-slots" {
                // Where the addresses go. It is volatile so both stores to it happen, which is
                // what stops either local being promoted into a register and is the whole of
                // what puts the pair in the frame.
                program.top(format!("static {name} *volatile hold;"));
            }
            program.blank();
            if shape != "frame-slots" {
                // Filled in a loop and worked on at an index nothing knows. `buffer[3]` is 8
                // afterwards, which is the number every shape below counts from.
                program.line(format!("{name} buffer[8];"));
                program.line("for (int i = 0; i < 8; i++) {");
                program.line_at(1, format!("buffer[i] = seed + ({name})i;"));
                program.line("}");
            }
            let value: i128 = match shape {
                // The shape the pass exists for. 8 and 5 make 13, in one instruction on a
                // machine that has one.
                "add" => {
                    program.line("buffer[at] += seed;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // The memory minus the register, which is the arrangement the one instruction
                // computes. 8 take away 5 is 3.
                "subtract" => {
                    program.line("buffer[at] -= seed;");
                    program.line(format!("{name} total = buffer[back];"));
                    3
                }
                // The register minus the memory, which the one instruction does not compute.
                // 25 take away 8 is 17, and a back end that folded this anyway prints 17 less
                // twice over, which is to say it prints something else.
                "subtract-reversed" => {
                    program.line(format!("buffer[at] = (seed + {}) - buffer[at];", lit(ty, 20)));
                    program.line(format!("{name} total = buffer[back];"));
                    17
                }
                // 8 or 5 is 13.
                "bitwise-or" => {
                    program.line("buffer[at] |= seed;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // 8 and 12 is 8. The mask is an input rather than a constant because arithmetic
                // against a constant is a different instruction, one that carries an address and
                // an immediate at once, and no target here describes it yet.
                "bitwise-and" => {
                    program.line("buffer[at] &= mask;");
                    program.line(format!("{name} total = buffer[back];"));
                    8
                }
                // 8 exclusive or 5 is 13.
                "exclusive-or" => {
                    program.line("buffer[at] ^= seed;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // The word that came back is read by the arithmetic and by the sum at the end,
                // so it has to be in a register whatever happens and the load cannot go away.
                // 13 and 8 make 21.
                "two-readers" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("buffer[at] = value + seed;");
                    program.line(format!("{name} total = buffer[back] + value;"));
                    21
                }
                // The answer is read by the store and by the sum at the end. One instruction
                // that writes memory leaves nothing in a register, so this one cannot be it.
                // 13 and 13 make 26.
                "answer-read" => {
                    program.line(format!("{name} sum = buffer[at] + seed;"));
                    program.line("buffer[at] = sum;");
                    program.line(format!("{name} total = buffer[back] + sum;"));
                    26
                }
                // A store between the load and the store, at an index nothing can tell apart
                // from the one being worked on. 13 and 5 make 18.
                "store-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("buffer[other] = seed;");
                    program.line("buffer[at] = value + seed;");
                    program.line(format!("{name} total = buffer[back] + buffer[other];"));
                    18
                }
                // A call between the two. What a call does to memory is not written in the call.
                "call-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("jump();");
                    program.line("buffer[at] = value + seed;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // Read at one index and written at another. The two addresses are the same
                // shape and are not the same place. 8 and 13 make 21.
                "other-place" => {
                    program.line("buffer[other] = buffer[at] + seed;");
                    program.line(format!("{name} total = buffer[back] + buffer[other];"));
                    21
                }
                // The same address and the same index, one element apart. This is the shape a
                // back end gets wrong by comparing the base and the index and stopping there.
                // 8 and 13 make 21.
                "other-offset" => {
                    program.line("buffer[at + 1] = buffer[at] + seed;");
                    program.line(format!("{name} total = buffer[back] + buffer[back + 1];"));
                    21
                }
                // The index the address reads is written between the load and the store, so the
                // address the one instruction would carry is not the address that was read.
                // 8 and 13 make 21.
                "index-written-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("at = other;");
                    program.line("buffer[at] = value + seed;");
                    program.line(format!("{name} total = buffer[back] + buffer[other];"));
                    21
                }
                // Two locals whose addresses got out, so both are in the frame, and neither has
                // been given a place in it by the time a back end pass sees the three
                // instructions. 5 and 10 make 15.
                _ => {
                    program.line(format!("{name} one = seed;"));
                    program.line(format!("{name} two = seed + {};", lit(ty, 1)));
                    program.line("hold = &one;");
                    program.line("hold = &two;");
                    program.line("two = one + seed;");
                    program.line(format!("{name} total = one + two;"));
                    15
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", value);
            sink.push(
                Facet::StoreFold,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A load, arithmetic against a constant, and a store of the answer to the same place.
///
/// The shape above with the second operand written down instead of held in a register, which is
/// the commoner half of it: `*p += 1` and `s->count -= 1` are what a program says far more often
/// than `*p += x`. On the machine it is a different instruction rather than the same one with a
/// constant in it, since it carries an addressing mode and an immediate at once and no register
/// of its own, so a back end that has the register form has not got this one for free and these
/// programs are the ones that say whether it does.
///
/// Every type rather than the four at least as wide as `int`, because the widths are the whole
/// question here. C promotes a narrow operand to `int` before the arithmetic and narrows the
/// answer on the way back into memory, so reaching the byte form at all means a compiler worked
/// out that the wide arithmetic in between was not asked for. One that never works that out is
/// correct in all of these and longer in all of them, which the size column says and the output
/// does not.
///
/// The shapes are the five operations that have a memory destination, the subtraction both ways
/// round, a constant too big for the short form of the encoding, and then the same reasons the
/// three are not one that the register form has. There is no multiply, for the same reason as
/// above, and there is no reversed constant that a compiler could fold either, because `20 - *p`
/// is the constant minus the memory and the instruction computes the memory minus the constant.
fn store_fold_constant(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "add",
        "subtract",
        "subtract-reversed",
        "bitwise-or",
        "bitwise-and",
        "exclusive-or",
        "big-constant",
        "two-readers",
        "answer-read",
        "store-between",
        "call-between",
        "other-place",
        "other-offset",
        "index-written-between",
        "frame-slots",
    ];
    for &ty in Ty::ALL {
        for &shape in SHAPES {
            if !sink.wants(Facet::StoreFoldConstant) {
                return;
            }
            // A constant that needs the long form of the encoding does not fit a byte at all,
            // so there is nothing to ask at the narrowest width and no program for it.
            if shape == "big-constant" && ty.bits() == 8 {
                continue;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("one {name} read and written against a constant, {shape}"));
            program.input(ty, "seed", 5);
            if shape != "frame-slots" {
                program.input(Ty::I32, "at", 3);
                // The index the answer is read back at, holding what `at` holds without saying
                // so. The same trick the register form uses, and for the same reason: an array
                // written and read at one index is a store handed straight to the reader, and
                // then the program has no store left in it to fold.
                program.input(Ty::I32, "back", 3);
            }
            if matches!(shape, "store-between" | "other-place" | "index-written-between") {
                program.input(Ty::I32, "other", 6);
            }
            if shape == "call-between" {
                // A call the compiler cannot see through and cannot delete, the same one the
                // register form uses.
                program.top("static volatile int noise_count;");
                program.top("static void noise(void) {");
                program.top("    noise_count = noise_count + 1;");
                program.top("}");
                program.top("static void (*volatile jump)(void) = noise;");
            }
            if shape == "frame-slots" {
                // Where the addresses go, volatile so both stores happen and neither local can
                // be promoted out of the frame.
                program.top(format!("static {name} *volatile hold;"));
            }
            program.blank();
            if shape != "frame-slots" {
                // The same array filled the same way as the register form, so `buffer[3]` is 8
                // here too and the numbers below can be read against each other.
                program.line(format!("{name} buffer[8];"));
                program.line("for (int i = 0; i < 8; i++) {");
                program.line_at(1, format!("buffer[i] = seed + ({name})i;"));
                program.line("}");
            }
            let value: i128 = match shape {
                // The shape the walk exists for. 8 and 5 make 13, in one instruction carrying
                // an address and an immediate on a machine that has one.
                "add" => {
                    program.line("buffer[at] += 5;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // The memory minus the constant, which is what the instruction computes.
                "subtract" => {
                    program.line("buffer[at] -= 5;");
                    program.line(format!("{name} total = buffer[back];"));
                    3
                }
                // The constant minus the memory, which it does not. A back end that took this
                // for the shape above prints 3 where the answer is 12.
                "subtract-reversed" => {
                    program.line("buffer[at] = 20 - buffer[at];");
                    program.line(format!("{name} total = buffer[back];"));
                    12
                }
                // 8 or 5 is 13. At a byte and at a word this is one of the two operations no
                // rule in rucc selects yet, so it is the shape that says when that changes.
                "bitwise-or" => {
                    program.line("buffer[at] |= 5;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // 8 and 12 is 8, with the mask written down rather than handed in, which is the
                // difference between this facet and the one above.
                "bitwise-and" => {
                    program.line("buffer[at] &= 12;");
                    program.line(format!("{name} total = buffer[back];"));
                    8
                }
                // 8 exclusive or 5 is 13, and the other of the two the narrow widths cannot
                // reach yet.
                "exclusive-or" => {
                    program.line("buffer[at] ^= 5;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // A constant that will not fit the sign extended byte the short encoding holds,
                // so the instruction is the same one and four bytes longer. 8 and 4660 make
                // 4668.
                "big-constant" => {
                    program.line("buffer[at] += 4660;");
                    program.line(format!("{name} total = buffer[back];"));
                    4668
                }
                // The word that came back is read by the arithmetic and by the sum at the end,
                // so it has to be in a register and the load cannot go away. 13 and 8 make 21.
                "two-readers" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("buffer[at] = value + 5;");
                    program.line(format!("{name} total = buffer[back] + value;"));
                    21
                }
                // The answer is read by the store and by the sum at the end, and the one
                // instruction leaves nothing in a register. 13 and 13 make 26.
                "answer-read" => {
                    program.line(format!("{name} sum = buffer[at] + 5;"));
                    program.line("buffer[at] = sum;");
                    program.line(format!("{name} total = buffer[back] + sum;"));
                    26
                }
                // A store in between, at an index nothing can tell apart from the one being
                // worked on. 13 and 5 make 18.
                "store-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("buffer[other] = seed;");
                    program.line("buffer[at] = value + 5;");
                    program.line(format!("{name} total = buffer[back] + buffer[other];"));
                    18
                }
                // A call in between. What a call does to memory is not written in the call.
                "call-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("jump();");
                    program.line("buffer[at] = value + 5;");
                    program.line(format!("{name} total = buffer[back];"));
                    13
                }
                // Read at one index and written at another, two addresses of the same shape
                // that are not the same place. 8 and 13 make 21.
                "other-place" => {
                    program.line("buffer[other] = buffer[at] + 5;");
                    program.line(format!("{name} total = buffer[back] + buffer[other];"));
                    21
                }
                // The same address and the same index, one element along, which is the shape a
                // back end gets wrong by comparing the base and the index and stopping.
                "other-offset" => {
                    program.line("buffer[at + 1] = buffer[at] + 5;");
                    program.line(format!("{name} total = buffer[back] + buffer[back + 1];"));
                    21
                }
                // The index is written between the load and the store, so the address the one
                // instruction would carry is not the address that was read.
                "index-written-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("at = other;");
                    program.line("buffer[at] = value + 5;");
                    program.line(format!("{name} total = buffer[back] + buffer[other];"));
                    21
                }
                // Two locals whose addresses got out, neither placed in the frame yet by the
                // time the back end sees the three instructions. 5 and 10 make 15.
                _ => {
                    program.line(format!("{name} one = seed;"));
                    program.line(format!("{name} two = seed + {};", lit(ty, 1)));
                    program.line("hold = &one;");
                    program.line("hold = &two;");
                    program.line("two = one + 5;");
                    program.line(format!("{name} total = one + two;"));
                    15
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", value);
            sink.push(
                Facet::StoreFoldConstant,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A load and the comparison that reads it, on either side and against a constant.
///
/// A comparison is not arithmetic with a different name, which is why this is not a shape of
/// `load-fold`. It writes no value, only the answer to a question, and which side of it the
/// memory is on decides the question rather than the operand order. A machine that reads its
/// right hand side out of memory folds `x < *p` and keeps the condition, and folds `*p < x`
/// into `x > *p`, so the two arrangements are two different rows in whatever table a back end
/// keeps. Both arrangements of the equality are here as well, because equality turns over into
/// itself: a back end whose table reached for the opposite answer rather than the opposite
/// ordering writes inequality there, and these are the programs that say so.
///
/// The comparison against a constant is a different instruction again. It carries an addressing
/// mode and an immediate at once with no register on either side, and there is nothing to
/// arrange either way round because the constant has nowhere else to be. The constant written on
/// the left is here too, since `10 < *p` is the same one instruction with the condition turned
/// over.
///
/// Every type rather than the four at least as wide as `int`, because the two halves do not
/// behave alike at the narrow widths. C promotes a narrow operand before comparing it. Against a
/// constant the constant is narrowed along with it, so the comparison can be asked at the width
/// the memory has; against a register both sides arrive through an extension and the comparison
/// is asked at `int`, which is a fold that is not there to be made. A report where the narrow
/// rows of the two halves differ is that difference and not a fault.
///
/// The rest of the shapes are the two ways the answer is used, which are a byte and a branch and
/// are two different instructions once the block layout has been through, and then the reasons
/// the two are not one: the loaded word read twice, a store in the way, a call in the way, an
/// index written in between, and the comparison in another block.
fn compare_fold(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "equal-right",
        "equal-left",
        "less-right",
        "less-left",
        "constant-equal",
        "constant-ordered",
        "constant-reversed",
        "big-constant",
        "branch",
        "byte-and-branch",
        "two-readers",
        "store-between",
        "call-between",
        "index-written-between",
        "comparison-in-another-block",
    ];
    for &ty in Ty::ALL {
        for &shape in SHAPES {
            if !sink.wants(Facet::CompareFold) {
                return;
            }
            // A constant that needs the long form of the encoding does not fit a byte at all,
            // so there is nothing to ask at the narrowest width and no program for it.
            if shape == "big-constant" && ty.bits() == 8 {
                continue;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("one {name} load compared, {shape}"));
            program.input(ty, "seed", 5);
            program.input(Ty::I32, "at", 3);
            if matches!(shape, "store-between" | "index-written-between") {
                program.input(Ty::I32, "other", 6);
            }
            if shape == "comparison-in-another-block" {
                program.input(Ty::I32, "flag", 1);
            }
            if shape == "call-between" {
                // A call the compiler cannot see through and cannot delete, the same one the
                // folds above use.
                program.top("static volatile int noise_count;");
                program.top("static void noise(void) {");
                program.top("    noise_count = noise_count + 1;");
                program.top("}");
                program.top("static void (*volatile jump)(void) = noise;");
            }
            program.blank();
            // The same array filled the same way as the folds above, so `buffer[3]` is 8 here
            // too and the numbers can be read against each other.
            program.line(format!("{name} buffer[8];"));
            program.line("for (int i = 0; i < 8; i++) {");
            program.line_at(1, format!("buffer[i] = seed + ({name})i;"));
            program.line("}");
            let value: i128 = match shape {
                // The shape the fold exists for, with the load on the side the machine reads out
                // of memory. 5 is not 8, so this is the second number.
                "equal-right" => {
                    program.line("int total = (seed == buffer[at]) ? 4 : 7;");
                    7
                }
                // The same question with the sides the other way round, which is the same
                // question. A table that turned the condition into its opposite rather than into
                // its reverse asks whether they differ here and prints 4.
                "equal-left" => {
                    program.line("int total = (buffer[at] == seed) ? 4 : 7;");
                    7
                }
                // An ordering with the load on the right, which folds and keeps the condition it
                // was written with. 5 is below 8.
                "less-right" => {
                    program.line("int total = (seed < buffer[at]) ? 4 : 7;");
                    4
                }
                // The same ordering with the load on the left, which is the one that has to come
                // out as the question backwards. 8 is not below 5, and a back end that folded
                // this without turning the condition over prints 4.
                "less-left" => {
                    program.line("int total = (buffer[at] < seed) ? 4 : 7;");
                    7
                }
                // The comparison against a constant, which is what a program writes far more
                // often than either of the two above. 8 is 8.
                "constant-equal" => {
                    program.line("int total = (buffer[at] == 8) ? 4 : 7;");
                    4
                }
                // An ordering against a constant, where the constant is the right hand side the
                // instruction already reads. 8 is below 10.
                "constant-ordered" => {
                    program.line("int total = (buffer[at] < 10) ? 4 : 7;");
                    4
                }
                // The constant written first, which is the same one instruction with the
                // condition turned over. 10 is not below 8.
                "constant-reversed" => {
                    program.line("int total = (10 < buffer[at]) ? 4 : 7;");
                    7
                }
                // A constant that will not fit the sign extended byte the short encoding holds,
                // so the instruction is the same one and four bytes longer.
                "big-constant" => {
                    program.line("int total = (buffer[at] < 4660) ? 4 : 7;");
                    4
                }
                // The answer branched on rather than kept, which on a machine whose comparison
                // sets a byte is a second instruction the block layout takes away again. What is
                // left reads memory and writes nothing, and that is a form of its own.
                "branch" => {
                    program.line("int total = 0;");
                    program.line("if (buffer[at] > seed) {");
                    program.line_at(1, "total = total + 9;");
                    program.line("}");
                    program.line("total = total + 3;");
                    12
                }
                // The answer read as a value and branched on, so the byte has to survive. The
                // fold is still there to be made and the instruction it makes still writes a
                // byte, which is the pair this shape holds apart from the one above.
                "byte-and-branch" => {
                    program.line("int answer = (buffer[at] > seed);");
                    program.line("int total = answer;");
                    program.line("if (answer) {");
                    program.line_at(1, "total = total + 9;");
                    program.line("}");
                    10
                }
                // The loaded word read by the comparison and by the sum at the end, so it has to
                // be in a register whatever happens and folding the load buys an addressing mode
                // and keeps the load. 4 and 8 make 12.
                "two-readers" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("int total = (seed < value) ? 4 : 7;");
                    program.line("total = total + (int)value;");
                    12
                }
                // A store between the load and the comparison, at an index nothing can tell
                // apart from the one that was read. 4 and 5 make 9.
                "store-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("buffer[other] = seed;");
                    program.line("int total = (seed < value) ? 4 : 7;");
                    program.line("total = total + (int)buffer[other];");
                    9
                }
                // A call between the two. What a call does to memory is not written in the call,
                // so it is asked separately from whether an instruction has a memory operand.
                "call-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("jump();");
                    program.line("int total = (seed < value) ? 4 : 7;");
                    4
                }
                // The index the address reads is written between the load and the comparison, so
                // the address a folded comparison would carry is not the address that was read.
                // A back end that folded anyway compares 11 against 10 and prints 7. 4 and 11
                // make 15.
                "index-written-between" => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("at = other;");
                    program.line("int total = (value < 10) ? 4 : 7;");
                    program.line("total = total + (int)buffer[at];");
                    15
                }
                // The comparison is behind a branch, so the load and the instruction that would
                // take it in are not in the same block. A pass that works a block at a time stops
                // here whether or not the branch is taken.
                _ => {
                    program.line(format!("{name} value = buffer[at];"));
                    program.line("int total = 3;");
                    program.line("if (flag) {");
                    program.line_at(1, "total = (seed < value) ? 4 : 7;");
                    program.line("}");
                    4
                }
            };
            program.blank();
            program.check(Ty::I32, "total", value);
            sink.push(
                Facet::CompareFold,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// One local, used at a counted number of offsets, one program per count.
///
/// The question here is a number rather than a yes or a no. An address into the frame is a
/// distance from the stack pointer, and on x86-64 a memory operand based on the stack pointer
/// needs an index byte whether anything is indexed or not, so an instruction that takes one grows
/// by more than an instruction that takes an address in an ordinary register. Working the address
/// out once into a register costs an instruction and saves every user those bytes, so there is a
/// count past which handing it to all of them is the worse of the two, and the count belongs to
/// the target rather than to the compiler. tamnd/rucc#784 picked three for x86-64 by building one
/// compiler per candidate value and reading `.text` over the whole corpus, which says what the
/// number is without ever showing the shape it is about. These programs are that shape and
/// nothing else.
///
/// Every offset is written and then read, so a program on this axis hands its one address rather
/// more users than the axis point says. That is the honest way round: the axis is a count of
/// offsets, which is a fact about the C, and how many instructions end up wanting the address is
/// a fact about the compiler, which is what the run is measuring. Nothing here has an index in
/// it, because an address that is folded into a reader takes the base slot and an index would
/// mean the two do not compose.
fn frame_address(sink: &mut Sink<'_>) {
    const COUNTS: &[(i128, &str)] =
        &[(1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"), (6, "six")];
    const SEED: i128 = 5;
    for &(offsets, spelled) in COUNTS {
        if !sink.wants(Facet::FrameAddress) {
            return;
        }
        let mut program = Program::new(format!("one frame address, used at {spelled} offsets"));
        program.input(Ty::I64, "seed", SEED);
        program.blank();
        // The widest integer, so the offsets are eight bytes apart and no two of them share a
        // displacement, and exactly as many elements as the program uses, so the frame is as
        // small as the count allows and the displacements stay in one byte.
        program.line(format!("long long room[{offsets}];"));
        for at in 0..offsets {
            let more = if at == 0 { String::new() } else { format!(" + {}", lit(Ty::I64, at)) };
            program.line(format!("room[{at}] = seed{more};"));
        }
        let reads: Vec<String> = (0..offsets).map(|at| format!("room[{at}]")).collect();
        program.line(format!("long long total = {};", reads.join(" + ")));
        program.blank();
        // Every offset is in the sum, so a compiler that loses one prints a different number.
        program.check(Ty::I64, "total", offsets * SEED + offsets * (offsets - 1) / 2);
        sink.push(Facet::FrameAddress, Axes::of([("offsets", spelled)]), Dialect::C17, program);
    }
}

/// Two runs of bytes in the frame, in the shapes where they may be one run and where they may not.
///
/// A local and a value the allocator wrote out are the same kind of thing once the back end has
/// got this far, which is a run of bytes at a distance from the stack pointer. A back end that
/// lays every one of them out end to end takes a frame as large as all of them added together,
/// however few of them are wanted at any one point. One that shares takes a frame as large as the
/// most it needs at once, so these programs are written in pairs: a shape where two things are
/// never both wanted, and the same amount of C where they are.
///
/// The shapes where the answer is no are the ones worth having. A frame that came out larger than
/// it needed to be is a program that runs. A local given away while something can still reach it
/// through a pointer is a program that prints the wrong number, and the case that catches that is
/// `address-escapes`, where the last mention of the name is the call that hands its address out
/// and every read after that goes through the global the call stored it in. Every array here is
/// in scope at the point it is read, so there is nothing undefined for a compiler to take
/// advantage of: what separates a right answer from a wrong one is only whether the frame was
/// handed to something else too early.
///
/// The sizes are what make the difference visible in a report. Eight `long long` is sixty four
/// bytes, so two of them sharing is a frame sixty four bytes smaller, which is a number that
/// shows up next to a frame rather than inside the rounding.
fn stack_slots(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "two-scopes",
        "both-at-once",
        "address-escapes",
        "wide-alignment",
        "spilled-around-a-call",
        "scope-in-a-loop",
    ];
    const SEED: i128 = 5;
    const ROOM: i128 = 8;
    // Filling `room[i]` with `seed + i` and adding all of it up, which every shape does at least
    // once and which no compiler can work out ahead of time because the seed is volatile.
    let counted = |from: i128| ROOM * from + ROOM * (ROOM - 1) / 2;

    for &shape in SHAPES {
        if !sink.wants(Facet::StackSlots) {
            return;
        }
        let mut program = Program::new(format!("two runs of frame bytes, {shape}"));
        match shape {
            "address-escapes" => {
                // Not static and not inline, so that a compiler which cannot see through the
                // call has to assume the address went somewhere it cannot follow. One that can
                // see through it still has to keep the object, because the pointer is read
                // below while the array it points into is still in scope.
                program.top("static long long *escaped;");
                program.top("static void stash(long long *p) {");
                program.top("    escaped = p;");
                program.top("}");
            }
            "spilled-around-a-call" => {
                program.top("static long long opaque(long long v) {");
                program.top("    return v + 1;");
                program.top("}");
            }
            _ => {}
        }
        program.input(Ty::I64, "seed", SEED);
        program.blank();
        program.line("long long total = 0;");

        let total: i128 = match shape {
            // Two arrays in scopes that do not overlap, which is the shape the sharing exists
            // for. Nothing may read either of them outside the braces it was declared in, so the
            // second is free to be the same bytes as the first.
            "two-scopes" => {
                for (name, more) in [("first", ""), ("second", " + 10")] {
                    program.line("{");
                    program.line_at(1, format!("long long {name}[{ROOM}];"));
                    program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                    program.line_at(2, format!("{name}[i] = seed{more} + i;"));
                    program.line_at(1, "}");
                    program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                    program.line_at(2, format!("total += {name}[i];"));
                    program.line_at(1, "}");
                    program.line("}");
                }
                counted(SEED) + counted(SEED + 10)
            }
            // The same two arrays, both in scope at the same point and both read after both were
            // written. A compiler that gives them one run of bytes here prints a different
            // number, which is what makes this the pair of the shape above rather than a
            // repetition of it.
            "both-at-once" => {
                program.line(format!("long long first[{ROOM}];"));
                program.line(format!("long long second[{ROOM}];"));
                program.line(format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(1, "first[i] = seed + i;");
                program.line_at(1, "second[i] = seed + 10 + i;");
                program.line("}");
                program.line(format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(1, "total += first[i] * 2 + second[i];");
                program.line("}");
                2 * counted(SEED) + counted(SEED + 10)
            }
            // The array the call was handed the address of is mentioned by name for the last
            // time at the call. Everything below reads it through the global, so a compiler that
            // decides a local is finished with when its name stops appearing hands those bytes
            // to the array in the braces and prints a number sixty four bytes of rubbish larger
            // or smaller. The array is still in scope where it is read, so nothing here is
            // undefined.
            "address-escapes" => {
                program.line(format!("long long kept[{ROOM}];"));
                program.line(format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(1, "kept[i] = seed + i;");
                program.line("}");
                program.line("stash(kept);");
                program.line("{");
                program.line_at(1, format!("long long other[{ROOM}];"));
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "other[i] = seed + 10 + i;");
                program.line_at(1, "}");
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "total += other[i];");
                program.line_at(1, "}");
                program.line("}");
                program.line(format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(1, "total += escaped[i];");
                program.line("}");
                counted(SEED) + counted(SEED + 10)
            }
            // One of the two wants a stricter alignment than the other, which on x86-64 is
            // sixteen bytes against eight. Bytes shared by both have to be aligned for the
            // stricter of them, so a compiler that takes the alignment of whichever it placed
            // first gets an unaligned `long double` and either a wrong answer or a fault.
            "wide-alignment" => {
                program.line("{");
                program.line_at(1, format!("long double wide[{ROOM}];"));
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "wide[i] = (long double)(seed + i);");
                program.line_at(1, "}");
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "total += (long long)wide[i];");
                program.line_at(1, "}");
                program.line("}");
                program.line("{");
                program.line_at(1, format!("long long narrow[{ROOM}];"));
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "narrow[i] = seed + 10 + i;");
                program.line_at(1, "}");
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "total += narrow[i];");
                program.line_at(1, "}");
                program.line("}");
                counted(SEED) + counted(SEED + 10)
            }
            // An array whose scope has closed and a pile of values held across a call, which is
            // the other half of the same question: a value the allocator had to write out is a
            // run of frame bytes too, and the ones it writes out here are wanted only after the
            // array is finished with. Twelve of them is past what the convention leaves free
            // across a call on either target.
            "spilled-around-a-call" => {
                const LIVE: i128 = 12;
                program.line("{");
                program.line_at(1, format!("long long room[{ROOM}];"));
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "room[i] = seed + i;");
                program.line_at(1, "}");
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "total += room[i];");
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                for at in 0..LIVE {
                    program.line(format!("long long v{at} = seed + {at};"));
                }
                program.line("long long kept = opaque(seed);");
                let reads: Vec<String> = (0..LIVE).map(|at| format!("v{at}")).collect();
                program.line(format!("total += {} + kept;", reads.join(" + ")));
                counted(SEED) + LIVE * SEED + LIVE * (LIVE - 1) / 2 + SEED + 1
            }
            // An array declared inside a loop body, so it is a different object every turn and
            // none of them is wanted at the same time as any other, and one more after the loop
            // that may have all of the same bytes.
            _ => {
                const TURNS: i128 = 4;
                program.line(format!("for (int k = 0; k < {TURNS}; k++) {{"));
                program.line_at(1, format!("long long room[{ROOM}];"));
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "room[i] = seed + i + k;");
                program.line_at(1, "}");
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "total += room[i];");
                program.line_at(1, "}");
                program.line("}");
                program.line("{");
                program.line_at(1, format!("long long after[{ROOM}];"));
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "after[i] = seed + 10 + i;");
                program.line_at(1, "}");
                program.line_at(1, format!("for (int i = 0; i < {ROOM}; i++) {{"));
                program.line_at(2, "total += after[i];");
                program.line_at(1, "}");
                program.line("}");
                TURNS * counted(SEED) + ROOM * (TURNS * (TURNS - 1) / 2) + counted(SEED + 10)
            }
        };

        program.blank();
        // Every element of both arrays is in the sum, so a compiler that hands one of them away
        // while the other is still wanted prints something else.
        program.check(Ty::I64, "total", total);
        sink.push(Facet::StackSlots, Axes::of([("shape", shape)]), Dialect::C17, program);
    }
}

/// How many generator states a division case draws before it starts dividing, which stays in the
/// first level cache.
const DIVISION_STREAM: u32 = 2048;

/// How many times a division case walks its states, so that it does two million divisions and a
/// `div` against a multiply is milliseconds apart.
const DIVISION_ROUNDS: u32 = 1024;

/// The generator the division cases draw their dividends from, Knuth's MMIX constants.
fn next_state(state: u64) -> u64 {
    state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407)
}

/// The seed the division cases start from, read through a `volatile` so nothing folds.
const DIVISION_SEED: u64 = 7;

/// A type a program divides at, and how its dividend is drawn from the generator.
struct Dividend {
    /// The axis value.
    name: &'static str,
    /// The C that declares `x` from `state`.
    declare: &'static str,
    /// Whether a negative divisor means anything for it.
    signed: bool,
    /// The value `x` gets from a state, as C works it out.
    value: fn(u64) -> i128,
}

/// Every width a division happens at, with both signs.
///
/// The narrow ones are divided as `int`, which is how C promotes them, so a compiler that knows
/// the value came from sixteen bits can use a smaller magic number. The widened ones are sixty
/// four bit divisions of a value that fits in thirty two, which is what `size_t` arithmetic on an
/// `unsigned` often is. The last two could be anything, and a compiler without a high multiply
/// has to keep the `div` for them.
const DIVIDENDS: &[Dividend] = &[
    Dividend {
        name: "u8",
        declare: "unsigned char x = (unsigned char)(state >> 33);",
        signed: false,
        value: |state| i128::from((state >> 33) as u8),
    },
    Dividend {
        name: "i8",
        declare: "signed char x = (signed char)(state >> 33);",
        signed: true,
        value: |state| i128::from((state >> 33) as u8 as i8),
    },
    Dividend {
        name: "u16",
        declare: "unsigned short x = (unsigned short)(state >> 33);",
        signed: false,
        value: |state| i128::from((state >> 33) as u16),
    },
    Dividend {
        name: "i16",
        declare: "short x = (short)(state >> 33);",
        signed: true,
        value: |state| i128::from((state >> 33) as u16 as i16),
    },
    Dividend {
        name: "u32",
        declare: "unsigned x = (unsigned)(state >> 32);",
        signed: false,
        value: |state| i128::from((state >> 32) as u32),
    },
    Dividend {
        name: "i32",
        declare: "int x = (int)(state >> 32);",
        signed: true,
        value: |state| i128::from((state >> 32) as u32 as i32),
    },
    Dividend {
        name: "u32-widened",
        declare: "unsigned long long x = (unsigned)(state >> 32);",
        signed: false,
        value: |state| i128::from((state >> 32) as u32),
    },
    Dividend {
        name: "i32-widened",
        declare: "long long x = (int)(state >> 32);",
        signed: true,
        value: |state| i128::from((state >> 32) as u32 as i32),
    },
    Dividend {
        name: "u64",
        declare: "unsigned long long x = state;",
        signed: false,
        value: i128::from,
    },
    Dividend {
        name: "i64",
        declare: "long long x = (long long)state;",
        signed: true,
        value: |state| i128::from(state as i64),
    },
];

/// The divisors, with the axis value each is written as.
///
/// Seven needs the extra add at thirty two bits and ten does not, eight is a shift, with a bias
/// when the division is signed, and minus seven is the signed rewrite with the answer negated.
const DIVISORS: &[(&str, i128)] = &[("7", 7), ("10", 10), ("8", 8), ("minus-7", -7)];

/// What a division case adds up, worked out the way the C works it out.
///
/// Every round walks the same states, so one walk times the rounds is the answer.
fn divided(dividend: &Dividend, divisor: i128, remainder: bool) -> u64 {
    let mut state = DIVISION_SEED;
    let mut pass = 0u64;
    for _ in 0..DIVISION_STREAM {
        state = next_state(state);
        let x = (dividend.value)(state);
        // Rust's `/` and `%` on a signed integer truncate towards zero, as C's do, and the cast
        // keeps the low sixty four bits, which is what C's conversion to unsigned does.
        let answer = if remainder { x % divisor } else { x / divisor };
        pass = pass.wrapping_add(answer as u64);
    }
    pass.wrapping_mul(u64::from(DIVISION_ROUNDS))
}

/// Writes the part every division case shares: the states drawn from the seed, and the rounds
/// that walk them with `state` holding the one this step divides.
fn division_walk(program: &mut Program) {
    program.input(Ty::U64, "seed", i128::from(DIVISION_SEED));
    program.blank();
    program.line(format!("static unsigned long long states[{DIVISION_STREAM}];"));
    program.line("unsigned long long drawn = seed;");
    program.line(format!("for (int i = 0; i < {DIVISION_STREAM}; i++) {{"));
    program.line_at(1, "drawn = drawn * 6364136223846793005ull + 1442695040888963407ull;");
    program.line_at(1, "states[i] = drawn;");
    program.line("}");
    program.line(format!("for (int round = 0; round < {DIVISION_ROUNDS}; round++) {{"));
    program.line_at(1, format!("for (int i = 0; i < {DIVISION_STREAM}; i++) {{"));
    program.line_at(2, "unsigned long long state = states[i];");
}

/// A division or a remainder by a constant, done two million times over values nothing can see.
///
/// Section 19.5 of the rucc plan turns a division by a constant into a multiply by a magic number
/// and a few shifts, which is three or four cycles where the `div` it replaces is twenty to forty.
/// Whether a compiler does that, at which widths and for which signs, shows up in the clock and
/// in the count of `div` instructions, and that is what these are for. Every width C divides at is
/// here, each with a divisor that needs the extra add, one that does not, a power of two and a
/// negative one for the signed types. Each is a quotient and a remainder apart, since a remainder
/// is the quotient multiplied back and taken off, and that is more to get right.
fn division(sink: &mut Sink<'_>) {
    for dividend in DIVIDENDS {
        for &(name, divisor) in DIVISORS {
            if divisor < 0 && !dividend.signed {
                continue;
            }
            for (op, spelling, remainder) in [("quotient", "/", false), ("remainder", "%", true)] {
                if !sink.wants(Facet::Division) {
                    return;
                }
                let mut program = Program::new(format!(
                    "the {op} of {} values by {divisor}, done many times",
                    dividend.name
                ));
                program.line("unsigned long long total = 0;");
                division_walk(&mut program);
                program.line_at(2, dividend.declare);
                program.line_at(
                    2,
                    format!("total += (unsigned long long)(x {spelling} ({divisor}));"),
                );
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                program.check(Ty::U64, "total", i128::from(divided(dividend, divisor, remainder)));
                let axes = Axes::of([("type", dividend.name), ("divisor", name), ("op", op)]);
                sink.push(Facet::Division, axes, Dialect::C17, program);
            }
        }
    }
}

/// The sizes of the structures the exact division cases subtract pointers into.
const EXACT_SIZES: &[u32] = &[12, 7];

/// How many elements the exact division cases have to point at.
const EXACT_SLOTS: u64 = 1024;

/// What picks an element, which is one less than the count since the count is a power of two.
const EXACT_MASK: u64 = EXACT_SLOTS - 1;

/// A pointer subtraction over structures whose size is not a power of two.
///
/// C promises the two pointers are into the same array, so the byte difference is a multiple of
/// the size and the division has no remainder. A compiler that knows that multiplies by the
/// inverse of the odd part of the size and shifts, which is cheaper than a magic number.
fn exact_division(sink: &mut Sink<'_>) {
    for &size in EXACT_SIZES {
        if !sink.wants(Facet::Division) {
            return;
        }
        let mut program =
            Program::new(format!("pointers subtracted in an array of {size} byte structures"));
        program.line(format!("static struct {{ char bytes[{size}]; }} items[{EXACT_SLOTS}];"));
        program.line("long long total = 0;");
        division_walk(&mut program);
        // Both indexes are masks rather than remainders, so the only division is the one the
        // subtraction makes.
        program.line_at(2, format!("int from = (int)((state >> 33) & {EXACT_MASK}u);"));
        program.line_at(2, format!("int to = (int)((state >> 13) & {EXACT_MASK}u);"));
        program.line_at(2, "total += &items[to] - &items[from];");
        program.line_at(1, "}");
        program.line("}");
        program.blank();
        let mut state = DIVISION_SEED;
        let mut pass = 0i64;
        for _ in 0..DIVISION_STREAM {
            state = next_state(state);
            let from = i64::try_from((state >> 33) & EXACT_MASK).expect("a masked index");
            let to = i64::try_from((state >> 13) & EXACT_MASK).expect("a masked index");
            pass += to - from;
        }
        program.check(Ty::I64, "total", i128::from(pass) * i128::from(DIVISION_ROUNDS));
        let size = size.to_string();
        let axes = Axes::of([("type", "pointer"), ("divisor", size.as_str()), ("op", "exact")]);
        sink.push(Facet::Division, axes, Dialect::C17, program);
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
    fn the_three_hints_at_one_rate_print_the_same_number_and_a_higher_rate_prints_more() {
        // The hint may move the branch, but it must never move the answer, and since the taken
        // arm triples a byte where the other adds seven, taking it more often has to add up to
        // more. A rate axis that printed the same total at every rate would not be a rate.
        let cases = cases_for(Facet::IfConversion);
        let mut last = 0u64;
        for rate in super::RATES {
            let rate = rate.to_string();
            let totals: Vec<u64> = cases
                .iter()
                .filter(|c| c.axes.get("rate") == Some(rate.as_str()))
                .map(|c| output(c).trim().parse().unwrap())
                .collect();
            assert_eq!(totals.len(), 3, "rate {rate} has {} cases", totals.len());
            assert!(totals.iter().all(|&t| t == totals[0]), "rate {rate} disagrees: {totals:?}");
            assert!(totals[0] > last, "rate {rate} adds up to no more than the rate below it");
            last = totals[0];
        }
    }

    #[test]
    fn the_chain_shapes_with_the_same_widening_add_up_what_the_control_does() {
        // The control widens one arm without a sign, which is the same number for a byte, so it
        // has to print what the plain shape prints. The masked one prints something else, or the
        // mask did nothing.
        let cases = cases_for(Facet::IfConversion);
        let total = |name: &str| {
            let case = cases.iter().find(|c| c.axes.get("shape") == Some(name)).unwrap();
            output(case)
        };
        assert_eq!(total("shared-widening"), total("different-widening"));
        assert_ne!(total("shared-widening"), total("shared-mask"));
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
    fn the_call_off_the_path_cases_read_their_values_only_in_the_arm_without_the_call() {
        let cases = cases_for(Facet::RegisterPressure);
        let case = cases
            .iter()
            .find(|c| {
                c.axes.get("shape") == Some("call-off-the-path")
                    && c.axes.get("type") == Some("i32")
                    && c.axes.get("live") == Some("6")
            })
            .unwrap();
        // One read of each value, and all six inside the else arm, which is what makes the call
        // in the other arm something none of them is live over.
        for at in 0..6 {
            assert_eq!(case.source.matches(&format!("v{at};")).count(), 1, "v{at} in {}", case.id);
            assert!(case.source.contains(&format!("        kept += v{at};")), "{}", case.id);
        }
        assert_eq!(output(case), "21\n");
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
    fn every_memory_scheduling_shape_is_generated_for_both_unsigned_wide_types() {
        let shapes = ["read-after-write", "write-after-read", "two-writes", "read-only"];
        let cases = cases_for(Facet::Scheduling);
        for ty in ["u32", "u64"] {
            for shape in shapes {
                assert!(
                    cases.iter().any(|c| {
                        c.axes.get("type") == Some(ty) && c.axes.get("access") == Some(shape)
                    }),
                    "no {shape} case for {ty}"
                );
            }
        }
    }

    #[test]
    fn the_four_memory_scheduling_shapes_each_print_a_different_number() {
        // Which is the whole point of them. A case whose answer does not change when the store
        // and the load are swapped is a case that cannot catch the swap, and four shapes that
        // agree on a number would be four ways of testing one of them.
        let cases = cases_for(Facet::Scheduling);
        let mut seen: Vec<&str> = cases
            .iter()
            .filter(|c| c.axes.get("type") == Some("u64") && c.axes.get("access").is_some())
            .map(output)
            .collect();
        assert_eq!(seen.len(), 4);
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 4, "two shapes print the same number");
    }

    #[test]
    fn the_read_only_memory_shape_is_the_only_one_that_never_stores_in_its_loop() {
        for case in cases_for(Facet::Scheduling) {
            let Some(shape) = case.axes.get("access") else {
                continue;
            };
            // The fill loop stores once, and every shape has one of those. Anything past that is
            // a store inside the loop the case is actually about.
            let stores = case.source.matches("buffer[").count();
            let fill = case.source.matches("buffer[i] =").count();
            assert_eq!(fill, 1, "{} does not fill its buffer once", case.id);
            if shape == "read-only" {
                assert!(
                    !case.source.contains("buffer[at] ="),
                    "{} stores and says it does not",
                    case.id
                );
            } else {
                assert!(stores > 2, "{} has nothing for the order to matter to", case.id);
            }
        }
    }

    #[test]
    fn every_switch_case_visits_every_label_and_the_default() {
        // Except the hot case shapes, which draw their labels at random.
        for case in cases_for(Facet::SwitchLowering) {
            if case.axes.get("shape") == Some("hot-case") {
                continue;
            }
            let labels: i128 = case.axes.get("labels").unwrap().parse().unwrap();
            let expected: i128 = (1..=labels).sum();
            assert_eq!(output(&case), format!("{expected}\n"), "{}", case.id);
            assert!(case.source.contains("classify(-1)"), "{}", case.id);
        }
    }

    #[test]
    fn a_hot_case_hint_names_the_hot_label_or_a_cold_one_and_leaves_the_total_alone() {
        let cases = cases_for(Facet::SwitchLowering);
        let hot: Vec<&Case> =
            cases.iter().filter(|c| c.axes.get("shape") == Some("hot-case")).collect();
        assert_eq!(hot.len(), 12);
        for case in &hot {
            let stride = if case.axes.get("density") == Some("dense") { 1 } else { 17 };
            let named = |label: i128| format!("__builtin_expect(value, {})", label * stride);
            let hot_named = case.source.contains(&named(super::HOT_LABEL));
            let cold_named = case.source.contains(&named(super::COLD_LABEL));
            match case.axes.get("hint") {
                Some("right") => assert!(hot_named && !cold_named, "{}", case.id),
                Some("wrong") => assert!(cold_named && !hot_named, "{}", case.id),
                _ => assert!(!case.source.contains("__builtin_expect"), "{}", case.id),
            }
            let twin = hot
                .iter()
                .find(|c| {
                    c.axes.get("hint") == Some("none")
                        && c.axes.get("density") == case.axes.get("density")
                        && c.axes.get("rate") == case.axes.get("rate")
                })
                .unwrap();
            assert_eq!(output(case), output(twin), "{}", case.id);
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
    fn the_switch_sizes_sit_either_side_of_the_cluster_limit() {
        let cases = cases_for(Facet::SwitchLowering);
        let labels: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("labels")).collect();
        assert!(labels.contains(&"31"), "nothing below the limit and near it");
        assert!(labels.contains(&"33"), "nothing above the limit and near it");
    }

    #[test]
    fn every_shape_of_a_run_is_generated() {
        let cases = cases_for(Facet::SwitchRuns);
        let shapes: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("shape")).collect();
        for wanted in [
            "one-run",
            "several-runs",
            "runs-and-singles",
            "edge-run",
            "classifier",
            "look-alike-arms",
        ] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    #[test]
    fn a_value_in_a_hole_goes_to_the_default_and_a_label_gets_its_answer() {
        let holes = super::SHAPES.iter().find(|shape| shape.name == "holes").expect("holes");
        assert_eq!(super::dispatch_answer(holes, 0), super::SCATTER[0]);
        assert_eq!(super::dispatch_answer(holes, 1), 0);
        assert_eq!(super::dispatch_answer(holes, 30), super::SCATTER[15]);
        assert_eq!(super::dispatch_answer(holes, 31), 0);
        assert_eq!(super::dispatch_answer(holes, 32), 0);
        let wide = super::SHAPES.iter().find(|shape| shape.name == "wide-answers").expect("wide");
        assert!(super::dispatch_answer(wide, 1) > i128::from(i16::MAX));
        let below =
            super::SHAPES.iter().find(|shape| shape.name == "negative-answers").expect("neg");
        assert!((i128::from(i8::MIN)..0).contains(&super::dispatch_answer(below, 2)));
    }

    #[test]
    fn a_masked_switch_has_a_label_for_every_value_the_mask_gives() {
        for shape in super::SHAPES.iter().filter(|shape| super::MASKED.contains(&shape.name)) {
            assert_eq!(shape.labels.count_ones(), 1, "the {} mask leaves a value out", shape.name);
            for value in 0..shape.span {
                let answer = super::dispatch_answer(shape, value);
                assert_ne!(answer, 0, "{} at {value} reaches the default", shape.name);
                assert_eq!(answer, super::dispatch_answer(shape, value & (shape.labels - 1)));
            }
        }
        let cases = cases_for(Facet::SwitchDispatch);
        let masked = cases.iter().find(|c| c.axes.get("shape") == Some("masked")).expect("masked");
        assert!(masked.source.contains("switch (value & 15)"), "{}", masked.source);
    }

    #[test]
    fn an_address_switch_reads_inside_every_string_it_returns() {
        assert!(super::WORDS.iter().chain([&super::NO_WORD]).all(|word| word.len() >= 4));
        for step in -1..16 {
            assert!(super::letter_offset(step) + 4 <= super::LETTERS.len() as i128, "{step}");
        }
        let holes = &super::ADDRESSED[1];
        assert_eq!(super::addressed_step(holes, 30), Some(15));
        assert_eq!(super::addressed_step(holes, 31), None);
        assert_eq!(super::addressed_step(holes, 32), None);
        assert_eq!(super::addressed_byte(holes, 2, 1), i128::from(b'e'));
        assert_eq!(super::addressed_byte(holes, 3, 0), i128::from(b'n'));
        let into = &super::ADDRESSED[2];
        assert_eq!(super::addressed_byte(into, 0, 0), i128::from(b'J'));
        let cases = cases_for(Facet::SwitchDispatch);
        for shape in super::ADDRESSED {
            let case = cases.iter().find(|c| c.axes.get("shape") == Some(shape.name));
            let case = case.unwrap_or_else(|| panic!("no case for {}", shape.name));
            assert!(case.source.contains("static const char *"), "{}", case.source);
            assert!(case.source.contains("[at & 3]"), "{}", case.source);
        }
    }

    #[test]
    fn every_dispatch_shape_is_generated() {
        let cases = cases_for(Facet::SwitchDispatch);
        let shapes: Vec<&str> = cases.iter().filter_map(|c| c.axes.get("shape")).collect();
        for wanted in [
            "affine",
            "scattered",
            "constant-arms",
            "below-zero",
            "near-the-edge",
            "shared-default",
            "interpreter",
        ] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
        for wanted in super::SMALL.iter().chain(super::TABLED).chain(super::MASKED) {
            assert!(shapes.contains(wanted), "no case for {wanted}");
        }
        for paired in super::PAIRED {
            let both = cases
                .iter()
                .filter(|c| c.axes.get("shape") == Some(*paired))
                .filter_map(|c| c.axes.get("stream"))
                .count();
            assert_eq!(both, 2, "the {paired} shape has nothing to be compared against");
        }
    }

    #[test]
    fn a_dispatch_case_walks_its_stream_a_whole_number_of_times() {
        assert_eq!(super::STREAM.count_ones(), 1, "the walk masks the index rather than dividing");
        assert_eq!(super::DISPATCHES % super::STREAM, 0);
        for case in cases_for(Facet::SwitchDispatch) {
            assert!(case.source.contains("count_in = 10240000"), "{}", case.id);
            assert!(case.source.contains("at & 4095"), "{}", case.id);
        }
    }

    // The whole facet is about branches nobody can guess, and the only thing that actually makes
    // a stream unguessable is its length. Every stream here repeats, because the loop walks the
    // same array over and over, so the question is never whether it repeats but whether the
    // period is longer than the history a predictor keeps. Measured, 512 values are learned
    // outright and 4096 mispredict about half the time, so the bar is the measurement and not a
    // guess about what a pattern looks like. See [`super::STREAM`] for the numbers.
    #[test]
    fn the_unpredictable_stream_is_longer_than_a_predictor_remembers() {
        const { assert!(super::STREAM >= 4096, "the stream is short enough to be learned") };
        for shape in super::SHAPES {
            let stream = super::dispatch_stream(shape, false);
            assert_eq!(stream.len() as i128, super::STREAM);
            // A period shorter than the array would undo the length, whatever the array holds.
            for period in 1..=64 {
                let repeats = stream.iter().zip(stream.iter().skip(period)).all(|(a, b)| a == b);
                assert!(!repeats, "the {} stream repeats every {period}", shape.name);
            }
        }
    }

    #[test]
    fn every_dispatch_stream_reaches_the_default_and_every_label() {
        for shape in super::SHAPES {
            let stream = super::dispatch_stream(shape, false);
            for step in 0..shape.labels {
                let label = shape.base + step;
                assert!(stream.contains(&label), "the {} stream misses {label}", shape.name);
            }
            if super::MASKED.contains(&shape.name) {
                continue;
            }
            let misses = stream.iter().filter(|&&v| v >= shape.base + shape.labels).count();
            assert!(misses > 0, "the {} stream never takes the default", shape.name);
        }
    }

    #[test]
    fn the_shape_that_is_not_a_line_really_is_not() {
        let answers = super::SCATTER;
        let scale = answers[1] - answers[0];
        let offset = answers[0];
        let holds = answers
            .iter()
            .enumerate()
            .all(|(step, &answer)| scale * step as i128 + offset == answer);
        assert!(!holds, "the negative control is a line after all");
    }

    // The point of putting the labels here is that a compiler working the answers out as a
    // multiple of the label has to do it in arithmetic that wraps. If the multiple times the
    // first label still fits in an `int`, nothing wraps and the case tests nothing new.
    #[test]
    fn the_answers_near_the_top_of_int_need_arithmetic_that_wraps() {
        let shape = super::SHAPES
            .iter()
            .find(|shape| shape.name == "near-the-edge")
            .expect("the near-the-edge shape");
        let scale = super::dispatch_answer(shape, shape.base + 1)
            - super::dispatch_answer(shape, shape.base);
        assert!(
            scale * shape.base > i128::from(i32::MAX),
            "nothing wraps at {scale} times the base"
        );
    }

    #[test]
    fn the_shared_default_label_has_no_arm_of_its_own() {
        let case = cases_for(Facet::SwitchDispatch)
            .into_iter()
            .find(|c| c.axes.get("shape") == Some("shared-default"))
            .expect("the shared-default case");
        assert!(case.source.contains("    case 15:\n    default: return 0;"), "{}", case.source);
    }

    #[test]
    fn no_interpreter_arm_throws_the_accumulator_away() {
        for work in super::STEPS {
            assert!(work.contains("acc"), "{work} does not read the accumulator");
        }
        // An `and` with an operand under two hundred and fifty six would clear every high bit in
        // one step. It is the cheapest way to break the oracle, so it is worth naming, but it is
        // not the whole property. The test below is what actually holds the arms to their rule.
        assert!(
            !super::STEPS.iter().any(|work| work.contains('&')),
            "an arm masks the accumulator"
        );
    }

    // Reading the arms is not proof, because a mixture of arms can lose the accumulator even
    // when no single one of them does. `(acc >> 5) | operand` keeps every bit it is given, but
    // enough of them in a row would still walk a sixty four bit value down to nothing. So ask
    // the loop directly: start it somewhere else and see whether ten million steps later it
    // still remembers. If it does not, the case prints the same number whatever happened in it.
    #[test]
    fn the_interpreter_answer_still_depends_on_where_the_loop_started() {
        let from_one = super::interpreter_run(1);
        assert_ne!(from_one, super::interpreter_run(2), "the accumulator was thrown away");
        assert_ne!(from_one, super::interpreter_run(u64::MAX), "the high bits were thrown away");
        assert_eq!(i128::from(from_one), super::interpreter_answer());
    }

    #[test]
    fn a_run_really_is_a_stretch_of_labels_going_to_one_arm() {
        let cases = cases_for(Facet::SwitchRuns);
        let long = cases
            .iter()
            .find(|c| {
                c.axes.get("shape") == Some("one-run") && c.axes.get("variant") == Some("100")
            })
            .unwrap();
        assert_eq!(long.source.matches("    case ").count(), 100, "{}", long.id);
        assert_eq!(long.source.matches("return 1;").count(), 1, "{}", long.id);
        assert_eq!(output(long), "100\n", "{}", long.id);
    }

    #[test]
    fn a_run_at_the_edge_reaches_the_last_value_of_its_type() {
        let cases = cases_for(Facet::SwitchRuns);
        let top = cases.iter().find(|c| c.axes.get("variant") == Some("u32-top")).unwrap();
        assert!(top.source.contains("case 4294967295u:"), "{}", top.source);
        let bottom = cases.iter().find(|c| c.axes.get("variant") == Some("i32-bottom")).unwrap();
        assert!(bottom.source.contains("case (-2147483647 - 1):"), "{}", bottom.source);
        assert_eq!(output(top), "3\n");
        assert_eq!(output(bottom), "3\n");
    }

    #[test]
    fn the_classifier_sends_sixty_five_labels_to_four_arms() {
        for variant in ["char", "unsigned-char"] {
            let cases = cases_for(Facet::SwitchRuns);
            let case = cases
                .iter()
                .find(|c| {
                    c.axes.get("shape") == Some("classifier")
                        && c.axes.get("variant") == Some(variant)
                })
                .unwrap();
            assert_eq!(case.source.matches("    case ").count(), 65, "{}", case.id);
            assert!(case.source.contains("case 'z':"), "{}", case.id);
            assert!(case.source.contains("case '\\t':"), "{}", case.id);
        }
    }

    #[test]
    fn the_look_alike_arms_each_give_a_different_answer() {
        let cases = cases_for(Facet::SwitchRuns);
        let case = cases.iter().find(|c| c.axes.get("shape") == Some("look-alike-arms")).unwrap();
        for step in 1..=5 {
            assert!(case.source.contains(&format!("out = value + {step};")), "{}", case.id);
        }
        assert_eq!(output(case), "75\n", "{}", case.id);
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
        // Except the rate shapes, whose condition holds as often as their rate says.
        for case in cases_for(Facet::IfConversion) {
            let shape = case.axes.get("shape").unwrap_or_default();
            if shape == "at-a-rate" || super::CHAINED.iter().any(|one| one.name == shape) {
                continue;
            }
            assert!(case.source.contains("int odd = i & 1;"), "{}", case.id);
        }
    }

    #[test]
    fn every_layout_case_expects_a_thousand_iterations_of_the_common_path() {
        for case in cases_for(Facet::BlockLayout) {
            assert_eq!(output(&case), "1000\n", "{}", case.id);
        }
    }

    #[test]
    fn every_hinted_shape_is_written_once_with_a_hint_and_once_without() {
        let cases = cases_for(Facet::BlockLayout);
        for shape in super::HINTED {
            for hint in super::HINTS {
                assert!(
                    cases.iter().any(|c| {
                        c.axes.get("shape") == Some(shape.name) && c.axes.get("hint") == Some(hint)
                    }),
                    "no {} program with the {hint} hint",
                    shape.name
                );
            }
        }
    }

    #[test]
    fn a_layout_case_is_tagged_gnu_exactly_when_it_writes_a_builtin() {
        for case in cases_for(Facet::BlockLayout) {
            assert_eq!(
                case.has_tag("gnu"),
                case.source.contains("__builtin_"),
                "{} is tagged wrong",
                case.id
            );
        }
    }

    #[test]
    fn the_two_spellings_of_a_hint_differ_only_in_the_hint() {
        // Which is what makes a triple evidence rather than three unrelated programs. Strip the
        // hint back out of each one and the three sources have to be the same text.
        let cases = cases_for(Facet::BlockLayout);
        for shape in super::HINTED {
            let plain = cases
                .iter()
                .find(|c| {
                    c.axes.get("shape") == Some(shape.name) && c.axes.get("hint") == Some("none")
                })
                .unwrap_or_else(|| panic!("no plain {} program", shape.name));
            for hint in ["builtin", "probability"] {
                let other = cases
                    .iter()
                    .find(|c| {
                        c.axes.get("shape") == Some(shape.name) && c.axes.get("hint") == Some(hint)
                    })
                    .unwrap_or_else(|| panic!("no {hint} {} program", shape.name));
                assert_eq!(
                    plain.expect, other.expect,
                    "{} answers differently with the {hint} hint",
                    shape.name
                );
                assert_eq!(
                    plain.source.lines().count(),
                    other.source.lines().count(),
                    "{} is a different program with the {hint} hint",
                    shape.name
                );
            }
        }
    }

    #[test]
    fn every_hinted_layout_case_puts_a_call_on_the_arm_it_says_is_cold() {
        for case in cases_for(Facet::BlockLayout) {
            if case.axes.get("hint").is_none() {
                continue;
            }
            assert!(case.source.contains("cold();"), "{} has no cold arm", case.id);
            // Through a volatile pointer, or GCC inlines the body and the arm is not a call.
            assert!(case.source.contains("(*volatile cold)"), "{} inlines away", case.id);
        }
    }

    #[test]
    fn every_address_fold_case_reads_back_exactly_what_it_wrote() {
        let cases = cases_for(Facet::AddressFold);
        assert!(!cases.is_empty());
        for case in &cases {
            // Every shape stores through an address and then adds up what it stored, so the
            // number is fixed by the seed and the offsets rather than by anything the compiler
            // decides. A case printing something else would mean a store and a load that were
            // meant to be the same address were not.
            let wanted = match case.axes.get("shape") {
                Some("several-offsets") => "18\n",
                Some("store-then-load") => "6\n",
                Some("computed-index" | "base-written-between" | "reader-in-another-block") => {
                    "11\n"
                }
                Some("filled-then-read") => "68\n",
                _ => "26\n",
            };
            assert_eq!(output(case), wanted, "{}", case.id);
        }
    }

    #[test]
    fn the_address_shapes_the_fold_has_to_refuse_are_all_present() {
        let cases = cases_for(Facet::AddressFold);
        let shapes: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("shape")).collect();
        for wanted in ["base-written-between", "reader-in-another-block", "global-many-readers"] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    #[test]
    fn the_global_many_readers_shape_puts_its_array_at_file_scope() {
        for case in cases_for(Facet::AddressFold) {
            if case.axes.get("shape") == Some("global-many-readers") {
                // The point of the shape is that the address is a symbol rather than a
                // register, which is only true of an array outside any function.
                assert!(case.source.contains("shared[4];\n"), "{}", case.id);
                let at = case.source.find("shared[4];").unwrap();
                assert!(at < case.source.find("int main").unwrap(), "{}", case.id);
            }
        }
    }

    #[test]
    fn every_load_fold_case_prints_the_number_its_shape_fixes() {
        let cases = cases_for(Facet::LoadFold);
        assert!(!cases.is_empty());
        for case in &cases {
            // Every shape is arithmetic over values the inputs fix, so the number belongs to the
            // shape and not to the type or to anything the compiler decides. A case printing
            // something else is a load that moved past something it had to stop at.
            let wanted = match case.axes.get("shape") {
                Some("two-readers") => "21\n",
                Some("load-between") => "5\n",
                Some("index-written-between") => "19\n",
                Some("subtract-left") => "3\n",
                Some("subtract-right") => "6\n",
                _ => "13\n",
            };
            assert_eq!(output(case), wanted, "{}", case.id);
        }
    }

    #[test]
    fn the_load_fold_shapes_the_pass_has_to_refuse_are_all_present() {
        let cases = cases_for(Facet::LoadFold);
        let shapes: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("shape")).collect();
        for wanted in [
            "two-readers",
            "store-between",
            "call-between",
            "load-between",
            "index-written-between",
            "subtract-left",
            "reader-in-another-block",
            "narrower-load",
        ] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    #[test]
    fn every_load_fold_case_with_an_array_fills_it_before_it_reads_it_at_an_unknown_index() {
        for case in cases_for(Facet::LoadFold) {
            if case.axes.get("shape") == Some("load-between") {
                continue;
            }
            // Written at one index and read back at the same one is a store the optimizer hands
            // to the reader, and then the case has no load in it and says nothing about a pass
            // that is about loads. Filling in a loop and reading at `at` is what keeps it.
            assert!(case.source.contains("for (int i = 0; i < 8; i++) {"), "{}", case.id);
            assert!(case.source.contains("[at]"), "{}", case.id);
        }
    }

    #[test]
    fn the_load_between_shape_reads_two_volatile_globals_in_the_order_it_subtracts_them() {
        for case in cases_for(Facet::LoadFold) {
            if case.axes.get("shape") != Some("load-between") {
                continue;
            }
            // The whole of the shape is that both reads are volatile, that the read of `first`
            // comes before the read of `second`, and that the subtraction reads `first`. Take any
            // one of the three away and the case stops saying anything about ordering.
            assert!(case.source.contains("volatile"), "{}", case.id);
            let first = case.source.find("first = first_in").unwrap();
            let second = case.source.find("second = second_in").unwrap();
            assert!(first < second, "{}", case.id);
            assert!(case.source.contains("total = second - first;"), "{}", case.id);
        }
    }

    #[test]
    fn the_call_between_shape_calls_through_a_pointer_the_compiler_cannot_see_through() {
        for case in cases_for(Facet::LoadFold) {
            if case.axes.get("shape") != Some("call-between") {
                continue;
            }
            // A direct call to an empty function is inlined into nothing and the case loses the
            // one instruction it was written around.
            assert!(case.source.contains("jump();"), "{} has no call", case.id);
            assert!(case.source.contains("(*volatile jump)"), "{} inlines away", case.id);
        }
    }

    #[test]
    fn every_store_fold_case_prints_the_number_its_shape_fixes() {
        let cases = cases_for(Facet::StoreFold);
        assert!(!cases.is_empty());
        for case in &cases {
            // Every shape works on the same array filled the same way, so `buffer[3]` is 8
            // everywhere and the number belongs to the shape rather than to the type. A case
            // printing something else is three instructions that became one when they could
            // not.
            let wanted = match case.axes.get("shape") {
                Some("subtract") => "3\n",
                Some("subtract-reversed") => "17\n",
                Some("bitwise-and") => "8\n",
                Some("two-readers") => "21\n",
                Some("answer-read") => "26\n",
                Some("store-between") => "18\n",
                Some("other-place" | "other-offset" | "index-written-between") => "21\n",
                Some("frame-slots") => "15\n",
                _ => "13\n",
            };
            assert_eq!(output(case), wanted, "{}", case.id);
        }
    }

    #[test]
    fn the_store_fold_shapes_the_pass_has_to_refuse_are_all_present() {
        let cases = cases_for(Facet::StoreFold);
        let shapes: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("shape")).collect();
        for wanted in [
            "subtract-reversed",
            "two-readers",
            "answer-read",
            "store-between",
            "call-between",
            "other-place",
            "other-offset",
            "index-written-between",
            "frame-slots",
        ] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    #[test]
    fn the_store_fold_shapes_cover_every_operation_that_has_a_memory_destination() {
        let cases = cases_for(Facet::StoreFold);
        let shapes: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("shape")).collect();
        // Five operations share the column of opcodes that write memory. The multiply is not
        // among them and there is no shape for it, because the machine has no such encoding.
        for wanted in ["add", "subtract", "bitwise-and", "bitwise-or", "exclusive-or"] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
        for case in &cases {
            assert!(!case.source.contains("*="), "{} multiplies", case.id);
        }
    }

    #[test]
    fn both_arrangements_of_the_store_fold_subtraction_are_present_and_differ() {
        let cases = cases_for(Facet::StoreFold);
        let one = cases
            .iter()
            .find(|case| case.axes.get("shape") == Some("subtract"))
            .expect("no subtract case");
        let other = cases
            .iter()
            .find(|case| case.axes.get("shape") == Some("subtract-reversed"))
            .expect("no subtract-reversed case");
        // The memory minus the register and the register minus the memory. One of them is the
        // instruction that writes memory and the other is not, and they print different numbers
        // so a report showing the same answer for both is a finding.
        assert!(one.source.contains("buffer[at] -= seed;"), "{}", one.id);
        assert!(other.source.contains("- buffer[at];"), "{}", other.id);
        assert_ne!(output(one), output(other));
    }

    #[test]
    fn every_store_fold_case_with_an_array_reads_the_answer_back_at_an_index_nothing_knows() {
        for case in cases_for(Facet::StoreFold) {
            if case.axes.get("shape") == Some("frame-slots") {
                continue;
            }
            // Written at one index and read back at the same one is a store the optimizer hands
            // to the reader, and then the store is dead and the case has nothing in it. `back`
            // holds what `at` holds and nothing says so, which is what keeps the store.
            assert!(case.source.contains("[back]"), "{}", case.id);
            assert!(case.source.contains("back = back_in;"), "{}", case.id);
            assert!(case.source.contains("for (int i = 0; i < 8; i++) {"), "{}", case.id);
        }
    }

    #[test]
    fn the_frame_slots_shape_puts_both_locals_in_the_frame() {
        for case in cases_for(Facet::StoreFold) {
            if case.axes.get("shape") != Some("frame-slots") {
                continue;
            }
            // Two locals whose addresses got out, so neither can live in a register and both are
            // a distance from the stack pointer that nothing has fixed yet. Take either address
            // away and the pair goes into registers and the case says nothing.
            assert!(case.source.contains("hold = &one;"), "{}", case.id);
            assert!(case.source.contains("hold = &two;"), "{}", case.id);
            assert!(case.source.contains("*volatile hold;"), "{}", case.id);
            assert!(case.source.contains("two = one + seed;"), "{}", case.id);
        }
    }

    #[test]
    fn every_store_fold_constant_case_prints_the_number_its_shape_fixes() {
        let cases = cases_for(Facet::StoreFoldConstant);
        assert!(!cases.is_empty());
        for case in &cases {
            // The same array filled the same way as the register form, so `buffer[3]` is 8 in
            // every one of these and the number belongs to the shape and not to the type.
            let wanted = match case.axes.get("shape") {
                Some("subtract") => "3\n",
                Some("subtract-reversed") => "12\n",
                Some("bitwise-and") => "8\n",
                Some("big-constant") => "4668\n",
                Some("two-readers") => "21\n",
                Some("answer-read") => "26\n",
                Some("store-between") => "18\n",
                Some("other-place" | "other-offset" | "index-written-between") => "21\n",
                Some("frame-slots") => "15\n",
                _ => "13\n",
            };
            assert_eq!(output(case), wanted, "{}", case.id);
        }
    }

    #[test]
    fn the_store_fold_constant_shapes_the_walk_has_to_refuse_are_all_present() {
        let cases = cases_for(Facet::StoreFoldConstant);
        let shapes: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("shape")).collect();
        for wanted in [
            "subtract-reversed",
            "two-readers",
            "answer-read",
            "store-between",
            "call-between",
            "other-place",
            "other-offset",
            "index-written-between",
            "frame-slots",
        ] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
    }

    #[test]
    fn the_store_fold_constant_shapes_cover_every_operation_that_has_a_memory_destination() {
        let cases = cases_for(Facet::StoreFoldConstant);
        let shapes: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("shape")).collect();
        // The same five that share the column of opcodes writing memory, and no multiply,
        // because the machine has no encoding for one.
        for wanted in ["add", "subtract", "bitwise-and", "bitwise-or", "exclusive-or"] {
            assert!(shapes.contains(&wanted), "no case for {wanted}");
        }
        for case in &cases {
            assert!(!case.source.contains("*="), "{} multiplies", case.id);
        }
    }

    #[test]
    fn every_store_fold_constant_case_works_against_a_number_written_down() {
        // The whole difference between this facet and the one above is where the second operand
        // is. A case that reached for the input again would be a second copy of `store-fold`
        // under another name, and the two would agree whatever a compiler did.
        for case in cases_for(Facet::StoreFoldConstant) {
            for reached in ["+= seed", "-= seed", "|= seed", "&= seed", "^= seed", "+ seed;"] {
                assert!(!case.source.contains(reached), "{} uses {reached}", case.id);
            }
        }
    }

    #[test]
    fn the_store_fold_constant_family_runs_from_a_byte_up_to_eight_of_them() {
        let cases = cases_for(Facet::StoreFoldConstant);
        let types: Vec<&str> = cases.iter().filter_map(|case| case.axes.get("type")).collect();
        // Every width the instruction has, which is what a facet about narrowing has to carry.
        // The register form stops at `int`, because promotion never changes its result type and
        // the widths are not what it is asking about.
        for wanted in ["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"] {
            assert!(types.contains(&wanted), "no case at {wanted}");
        }
        // A constant too big for a sign extended byte is not a question a byte can be asked.
        for case in &cases {
            if case.axes.get("shape") == Some("big-constant") {
                assert_ne!(case.axes.get("type"), Some("i8"), "{}", case.id);
                assert_ne!(case.axes.get("type"), Some("u8"), "{}", case.id);
            }
        }
    }

    #[test]
    fn the_frame_address_family_counts_from_one_offset_to_six() {
        let cases = cases_for(Facet::FrameAddress);
        let counted: Vec<Option<&str>> =
            cases.iter().map(|case| case.axes.get("offsets")).collect();
        let wanted = ["one", "two", "three", "four", "five", "six"];
        assert_eq!(counted, wanted.map(Some), "one program per count and no others");
    }

    #[test]
    fn every_frame_address_case_uses_one_more_offset_than_the_one_before() {
        // The sum of the first n of 5, 6, 7 and so on, which is what the offsets hold once they
        // have been written. A case printing anything else has lost one of them.
        let wanted = ["5\n", "11\n", "18\n", "26\n", "35\n", "45\n"];
        for (at, (case, answer)) in cases_for(Facet::FrameAddress).iter().zip(wanted).enumerate() {
            assert_eq!(output(case), answer, "{}", case.id);
            // The last offset the program uses, and the first one it does not. An array sized to
            // the count is what keeps the frame small enough for a one byte displacement.
            assert!(case.source.contains(&format!("long long room[{}];", at + 1)), "{}", case.id);
            assert!(case.source.contains(&format!("room[{at}] = seed")), "{}", case.id);
            assert!(!case.source.contains(&format!("room[{}] = seed", at + 1)), "{}", case.id);
            // An index would take the slot a folded address needs, so there is never one.
            assert!(!case.source.contains("room[i]"), "{}", case.id);
        }
    }

    #[test]
    fn the_stack_slot_shapes_cover_both_the_ones_that_may_share_and_the_ones_that_may_not() {
        let cases = cases_for(Facet::StackSlots);
        let shapes: Vec<Option<&str>> = cases.iter().map(|case| case.axes.get("shape")).collect();
        for wanted in ["two-scopes", "both-at-once", "address-escapes", "wide-alignment"] {
            assert!(shapes.contains(&Some(wanted)), "no case for {wanted}");
        }
        // The pair that makes the facet an experiment rather than an assertion: the same amount
        // of C, once where the two arrays are never both wanted and once where they are.
        let apart = cases.iter().find(|c| c.axes.get("shape") == Some("two-scopes")).unwrap();
        let together = cases.iter().find(|c| c.axes.get("shape") == Some("both-at-once")).unwrap();
        assert_eq!(output(apart), "216\n");
        assert_eq!(output(together), "284\n");
    }

    #[test]
    fn the_escaping_case_reads_its_array_through_the_pointer_it_handed_out() {
        let cases = cases_for(Facet::StackSlots);
        let case = cases.iter().find(|c| c.axes.get("shape") == Some("address-escapes")).unwrap();
        // The name is mentioned for the last time at the call, and the reads below it go through
        // the global. That is the whole of what the case is about, so a generator change that
        // put another `kept[` below the call would quietly turn it into a different case.
        let (before, after) = case.source.split_once("stash(kept);").unwrap();
        assert!(before.contains("kept[i] = seed + i;"), "{}", case.id);
        assert!(!after.contains("kept["), "{}", case.id);
        assert!(after.contains("total += escaped[i];"), "{}", case.id);
        assert_eq!(output(case), "216\n");
    }

    #[test]
    fn every_bit_liveness_case_has_a_block_boundary_between_the_widening_and_its_reader() {
        let cases = cases_for(Facet::BitLiveness);
        assert!(!cases.is_empty());
        for case in &cases {
            // The facet is for the pairs a peephole cannot see. A case with the widening and
            // the narrowing next to each other in one block would be testing the rewrite rules
            // instead, and would pass whether or not anything counted bits.
            assert!(
                case.source.contains("if (flag) {") || case.source.contains("for (int i"),
                "{} has no block boundary in it",
                case.id
            );
        }
    }

    #[test]
    fn every_bit_liveness_case_prints_what_the_narrow_type_still_holds() {
        for case in &cases_for(Facet::BitLiveness) {
            let signed = matches!(case.axes.get("type"), Some("i8" | "i16"));
            let wanted = match case.axes.get("shape") {
                Some("in-a-loop") => "15\n",
                Some("narrowed-from-a-double") => "41\n",
                Some("upper-bits-read") => {
                    if signed {
                        "-55\n"
                    } else {
                        "201\n"
                    }
                }
                _ => "40\n",
            };
            assert_eq!(output(case), wanted, "{}", case.id);
        }
    }

    #[test]
    fn the_bit_liveness_shape_that_catches_a_wrong_count_reads_the_widened_value_whole() {
        let mut seen = 0;
        for case in cases_for(Facet::BitLiveness) {
            if case.axes.get("shape") != Some("upper-bits-read") {
                continue;
            }
            // A comparison against zero on the widened value, which on a signed type is a
            // question about a bit the narrow type has not got.
            let signed = matches!(case.axes.get("type"), Some("i8" | "i16"));
            let test = if signed { "beyond = wide < 0;" } else { "beyond = wide > 127;" };
            assert!(case.source.contains(test), "{}", case.id);
            seen += 1;
        }
        assert_eq!(seen, 4, "one shape that must not change per narrow type");
    }

    #[test]
    fn every_compare_elim_case_has_two_comparisons_or_arithmetic_before_one() {
        let cases = cases_for(Facet::CompareElim);
        assert!(!cases.is_empty());
        for case in &cases {
            // The facet is about a comparison whose answer is already there, so a case with
            // nothing in front of the comparison would be a case about nothing. Either the
            // program compares the same pair twice or it works a value out and then asks
            // about that value.
            let twice =
                case.source.matches("a == b").count() + case.source.matches("a != b").count();
            let after = case.source.contains(" & mask")
                || case.source.contains(" | b")
                || case.source.contains("(a - b)");
            assert!(twice >= 2 || after, "{} has nothing in front of its comparison", case.id);
        }
    }

    #[test]
    fn every_compare_elim_case_prints_what_the_comparisons_really_answer() {
        for case in &cases_for(Facet::CompareElim) {
            let wanted = match case.axes.get("shape") {
                Some("same-comparison") => "1\n",
                Some("byte-and-branch") => "7\n",
                Some("after-and") => "19\n",
                Some("after-or") => "8\n",
                Some("after-sub") => "6\n",
                Some("after-sub-ordered") => "9\n",
                _ => "11\n",
            };
            assert_eq!(output(case), wanted, "{}", case.id);
        }
    }

    #[test]
    fn the_compare_elim_shape_that_must_stay_is_there_for_every_signed_type() {
        let cases = cases_for(Facet::CompareElim);
        let ordered: Vec<&str> = cases
            .iter()
            .filter(|case| case.axes.get("shape") == Some("after-sub-ordered"))
            .filter_map(|case| case.axes.get("type"))
            .collect();
        // An unsigned value is never below zero, so the only comparison against zero an
        // unsigned type has is an equality, which is the one a subtraction does answer. The
        // shape has no unsigned program to write rather than a missing one.
        assert_eq!(ordered, ["i32", "i64"]);
        for case in &cases {
            if case.axes.get("shape") == Some("after-sub-ordered") {
                assert!(case.source.contains("(d < 0) ? 4 : 8"), "{}", case.id);
            }
        }
    }

    #[test]
    fn the_compare_elim_case_the_issue_asks_for_writes_the_compared_value_in_between() {
        let mut seen = 0;
        for case in cases_for(Facet::CompareElim) {
            if case.axes.get("shape") != Some("written-in-between") {
                continue;
            }
            // A move between the arithmetic and the comparison, writing the same variable.
            // A move leaves the condition bits alone, so what stops a compiler getting this
            // wrong is noticing that the value they are about has been written over.
            let lines: Vec<&str> = case.source.lines().map(str::trim).collect();
            let masked = lines.iter().position(|line| line.starts_with("int first"));
            let moved = lines.iter().position(|line| *line == "value = spare;");
            let asked = lines.iter().position(|line| line.starts_with("int second"));
            assert!(masked < moved && moved < asked, "{}", case.id);
            seen += 1;
        }
        assert_eq!(seen, 4, "one per type");
    }

    #[test]
    fn a_division_case_covers_every_width_sign_and_divisor() {
        let cases = cases_for(Facet::Division);
        // Ten widths, three divisors each and a fourth for the five signed ones, a quotient and a
        // remainder of every one, and the two pointer subtractions.
        assert_eq!(cases.len(), (10 * 3 + 5) * 2 + 2);
        for case in &cases {
            let divisor = case.axes.get("divisor").expect("a divisor");
            if divisor == "minus-7" {
                assert!(case.source.contains("(-7)"), "{}", case.id);
                assert!(case.axes.get("type").is_some_and(|ty| ty.starts_with('i')), "{}", case.id);
            }
        }
    }

    #[test]
    fn a_division_answer_is_what_the_type_itself_gives() {
        let find = |name: &str| super::DIVIDENDS.iter().find(|one| one.name == name).expect(name);
        // The same walk done in the type's own arithmetic rather than in `i128`, so a slip in
        // how a value is drawn or how a negative answer is widened shows up as a difference.
        let mut state = super::DIVISION_SEED;
        let (mut narrow, mut signed, mut wide) = (0u64, 0u64, 0u64);
        for _ in 0..super::DIVISION_STREAM {
            state = super::next_state(state);
            narrow = narrow.wrapping_add(u64::from((state >> 33) as u16 / 10));
            signed = signed.wrapping_add(i64::from((state >> 32) as u32 as i32 % -7) as u64);
            wide = wide.wrapping_add(state / 7);
        }
        let rounds = u64::from(super::DIVISION_ROUNDS);
        assert_eq!(super::divided(find("u16"), 10, false), narrow.wrapping_mul(rounds));
        assert_eq!(super::divided(find("i32"), -7, true), signed.wrapping_mul(rounds));
        assert_eq!(super::divided(find("u64"), 7, false), wide.wrapping_mul(rounds));
    }
}
