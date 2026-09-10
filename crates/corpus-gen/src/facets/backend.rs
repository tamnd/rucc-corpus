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
    switch_runs(sink);
    switch_dispatch(sink);
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
/// arithmetic is not what the case ends up measuring. Long enough that a branch predictor
/// cannot learn the whole sequence, short enough that two kilobytes of it stay in the first
/// level cache and the case measures the switch rather than the memory system.
const STREAM: i128 = 512;

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
];

/// The shapes that are also dispatched on a stream a branch predictor can guess.
const PAIRED: &[&str] = &["affine", "scattered"];

/// A switch called often enough that how it was lowered shows up in the clock.
///
/// Every other switch facet is about size and about getting the right answer. This one is about
/// time, and it is the only place the corpus can see the thing document 24.3 says is the whole
/// reason to care about switch shape. A chain of equality tests on a value drawn at random from
/// the label range mispredicts about half the time, and the cost of that is several times the
/// cost of the instructions themselves. A range check does not mispredict, so a compiler that
/// reduces the switch to one comparison wins far more clock than it wins instructions, and no
/// case that dispatches a switch a few dozen times can tell you that.
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
}

/// The C name of a shape's function.
fn dispatch_name(shape: &Shape) -> String {
    shape.name.replace('-', "_")
}

/// The answer one of these switches gives for a value, with zero for the default.
fn dispatch_answer(shape: &Shape, value: i128) -> i128 {
    let step = value - shape.base;
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
    program.top("    switch (value) {");
    for step in 0..shape.labels {
        let label = shape.base + step;
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
/// No arm may throw the accumulator away. An `acc & operand` with an operand under two hundred
/// and fifty six clears every high bit, and after one of those the answer no longer depends on
/// anything that came before, so a run that dispatched a million steps wrongly could still print
/// the right number. Every arm here either carries all sixty four bits forward or mixes them, and
/// the multiply is there to make sure the high ones keep moving.
const STEPS: &[&str] = &[
    "acc + operand",
    "acc - operand",
    "acc ^ operand",
    "(acc << 3) ^ operand",
    "(acc >> 5) | operand",
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
    let mut acc: u64 = 1;
    for at in 0..DISPATCHES as usize {
        let operand = args[at & mask];
        acc = match ops[at & mask] {
            0 => acc.wrapping_add(operand),
            1 => acc.wrapping_sub(operand),
            2 => acc ^ operand,
            3 => (acc << 3) ^ operand,
            4 => (acc >> 5) | operand,
            5 => acc.wrapping_add(operand << 1),
            6 => acc.wrapping_sub(operand >> 1),
            7 => acc.wrapping_mul(3).wrapping_add(1),
            _ => acc,
        };
    }
    i128::from(acc)
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
            assert!(case.source.contains("at & 511"), "{}", case.id);
        }
    }

    // The whole facet is about branches nobody can guess, so a stream that repeats every few
    // values would leave every case here measuring a well predicted branch and saying nothing.
    // Thirty two is well inside what a modern predictor learns, so a period at or under it is a
    // broken case rather than a slightly weaker one.
    #[test]
    fn the_unpredictable_stream_does_not_repeat_quickly() {
        for shape in super::SHAPES {
            let stream = super::dispatch_stream(shape, false);
            for period in 1..=32 {
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
        // An `and` with an operand under two hundred and fifty six would clear every high bit,
        // and the answer would stop depending on what came before it.
        assert!(
            !super::STEPS.iter().any(|work| work.contains('&')),
            "an arm masks the accumulator"
        );
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
