//! The transformations that need more than one function.
//!
//! Everything here is a whole program in one translation unit, with the helpers marked
//! `static` so the compiler is allowed to reason about all of their call sites. That is the
//! setting these optimizations were designed for, and it is also the setting most C programs
//! are actually compiled in once the headers are expanded.

use super::lit;
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// The types used here.
const TYPES: &[Ty] = Ty::WIDE;

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    inline(sink);
    tail_call(sink);
    function_purity(sink);
    constant_args(sink);
    reachability(sink);
    devirtualize(sink);
    memory_effects(sink);
}

/// Calls of every size, from every number of places.
///
/// The two axes are the ones an inliner actually weighs. A tiny function called from one place
/// should always be inlined and the call should disappear. A large function called from twenty
/// places should not, or the code doubles for nothing. Everything in between is where a cost
/// model earns its keep, and where the report has something to say.
fn inline(sink: &mut Sink<'_>) {
    for &ty in TYPES {
        for &body in &[1usize, 4, 16] {
            for &sites in &[1usize, 4, 20] {
                if !sink.wants(Facet::Inline) {
                    return;
                }
                let name = ty.c_name();
                let mut program = Program::new(format!(
                    "a {body} statement {} helper called from {sites} places",
                    ty.c_name()
                ));
                program.top(format!("static {name} helper({name} value) {{"));
                for step in 0..body {
                    program.top(format!("    value = value + {};", lit(ty, step as i128 + 1)));
                }
                program.top("    return value;".to_owned());
                program.top("}".to_owned());

                // The helper adds one, two and so on up to the body size, so its effect is
                // the triangular number of the body size. Working that out here rather than
                // in the program is what keeps the case honest.
                let step: i128 = (1..=body as i128).sum();
                program.input(ty, "seed", 0);
                program.blank();
                program.line(format!("{name} total = 0;"));
                for at in 0..sites {
                    program.line(format!("total += helper(seed + {});", lit(ty, at as i128)));
                }
                program.blank();
                let total: i128 = (0..sites as i128).map(|at| at + step).sum();
                if !ty.promoted().holds(total) {
                    continue;
                }
                program.check(ty.promoted(), "total", total);
                sink.push(
                    Facet::Inline,
                    Axes::of([
                        ("type", ty.name()),
                        ("body", &body.to_string()),
                        ("sites", &sites.to_string()),
                    ]),
                    Dialect::C17,
                    program,
                );
            }
        }
    }
}

/// Recursion that is really a loop.
///
/// Self recursion with an accumulator, mutual recursion between two functions, and a plain
/// tail call to a different function. All three should become jumps.
///
/// The depths stop at five thousand rather than going somewhere that would overflow a stack.
/// A case that only passes when the call became a jump would fail at `-O0` for every compiler
/// ever written, which makes it a bad correctness case. The evidence that the jump happened
/// belongs in the size and time columns of the report, where it can be read as a number
/// instead of as a crash.
fn tail_call(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["self", "mutual", "forward"];
    for &ty in TYPES {
        for &depth in &[8i128, 1000, 5000] {
            for &shape in SHAPES {
                if !sink.wants(Facet::TailCall) {
                    return;
                }
                let name = ty.c_name();
                let mut program = Program::new(format!(
                    "{shape} tail recursion {depth} deep accumulating into {}",
                    ty.c_name()
                ));
                match shape {
                    "self" => {
                        program.top(format!("static {name} walk(int left, {name} total) {{"));
                        program.top("    if (left == 0) {".to_owned());
                        program.top("        return total;".to_owned());
                        program.top("    }".to_owned());
                        program.top("    return walk(left - 1, total + 1);".to_owned());
                        program.top("}".to_owned());
                    }
                    "mutual" => {
                        program.top(format!("static {name} odd(int left, {name} total);"));
                        program.top(format!("static {name} even(int left, {name} total) {{"));
                        program.top("    if (left == 0) {".to_owned());
                        program.top("        return total;".to_owned());
                        program.top("    }".to_owned());
                        program.top("    return odd(left - 1, total + 1);".to_owned());
                        program.top("}".to_owned());
                        program.top(format!("static {name} odd(int left, {name} total) {{"));
                        program.top("    if (left == 0) {".to_owned());
                        program.top("        return total;".to_owned());
                        program.top("    }".to_owned());
                        program.top("    return even(left - 1, total + 1);".to_owned());
                        program.top("}".to_owned());
                        program.top(format!("static {name} walk(int left, {name} total) {{"));
                        program.top("    return even(left, total);".to_owned());
                        program.top("}".to_owned());
                    }
                    _ => {
                        program.top(format!("static {name} finish({name} total) {{"));
                        program.top("    return total;".to_owned());
                        program.top("}".to_owned());
                        program.top(format!("static {name} step(int left, {name} total) {{"));
                        program.top("    if (left == 0) {".to_owned());
                        program.top("        return finish(total);".to_owned());
                        program.top("    }".to_owned());
                        program.top("    return step(left - 1, total + 1);".to_owned());
                        program.top("}".to_owned());
                        program.top(format!("static {name} walk(int left, {name} total) {{"));
                        program.top("    return step(left, total);".to_owned());
                        program.top("}".to_owned());
                    }
                }
                if !ty.promoted().holds(depth) {
                    continue;
                }
                program.input(Ty::I32, "depth", depth);
                program.blank();
                program.check(ty.promoted(), &format!("walk(depth, {})", lit(ty, 0)), depth);
                let deep = depth >= 5000;
                sink.push_tagged(
                    Facet::TailCall,
                    Axes::of([
                        ("type", ty.name()),
                        ("depth", &depth.to_string()),
                        ("shape", shape),
                    ]),
                    Dialect::C17,
                    program,
                    if deep { &["needs-tail-calls"] } else { &[] },
                );
            }
        }
    }
}

/// Functions that do not look at memory, called more than once with the same arguments.
///
/// A function that reads nothing and writes nothing can be called once instead of five times.
/// Whether the compiler works that out from the body or is told with an attribute is the axis.
/// The attribute shapes are tagged `gnu`, because they use a GCC extension and a run that is
/// only checking standard C should be able to leave them out.
///
/// The last two shapes are about the other thing knowing a function is pure buys, which is
/// deleting a call to it whose result nobody reads. `unused-result` throws one call away every
/// time round the loop and keeps a second, so a compiler that deletes the first and keeps the
/// second still prints the same total, and the instruction count says whether it did. That
/// count is the whole point of the case: the output cannot tell a deleted call from a kept one
/// and the counter can. `looping-const` is the same shape with a helper that reaches its answer
/// with a loop rather than a multiply. Nothing about the loop stops the helper being `const`,
/// but a compiler working from the shape of the control flow graph cannot see that it ends, and
/// one that cannot see that has to keep the call. So the two shapes bracket the decision: the
/// first is a call that may go, the second is the same call with one reason to stay.
fn function_purity(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "inferred-const",
        "inferred-pure",
        "declared-const",
        "declared-pure",
        "impure",
        "unused-result",
        "looping-const",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::FunctionPurity) {
                return;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("a {shape} {} function called five times", ty.c_name()));
            let attribute = match shape {
                "declared-const" => "__attribute__((const)) ",
                "declared-pure" => "__attribute__((pure)) ",
                _ => "",
            };
            let reads_memory = matches!(shape, "inferred-pure" | "declared-pure" | "unused-result");
            if shape == "looping-const" {
                program.top(format!("static {name} helper({name} value) {{"));
                program.top(format!("    {name} sum = 0;"));
                program.top(format!("    for ({name} i = 0; i < value; i++) {{"));
                program.top(format!("        sum += {};", lit(ty, 2)));
                program.top("    }".to_owned());
                program.top("    return sum;".to_owned());
                program.top("}".to_owned());
            } else if reads_memory {
                program.top(format!("static {name} table[4] = {{ 1, 2, 3, 4 }};"));
                program.top(format!("static {attribute}{name} helper({name} value) {{"));
                program.top("    return table[1] * value;".to_owned());
                program.top("}".to_owned());
            } else if shape == "impure" {
                program.top(format!("static {name} calls = 0;"));
                program.top(format!("static {name} helper({name} value) {{"));
                program.top("    calls = calls + 1;".to_owned());
                program.top(format!("    return {} * value;", lit(ty, 2)));
                program.top("}".to_owned());
            } else {
                program.top(format!("static {attribute}{name} helper({name} value) {{"));
                program.top(format!("    return {} * value;", lit(ty, 2)));
                program.top("}".to_owned());
            }
            program.input(ty, "seed", 6);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line("for (int i = 0; i < 5; i++) {");
            if matches!(shape, "unused-result" | "looping-const") {
                program.line_at(1, "helper(seed);");
            }
            program.line_at(1, "total += helper(seed);");
            program.line("}");
            program.blank();
            program.check(ty.promoted(), "total", 60);
            if shape == "impure" {
                program.check(ty.promoted(), "calls", 5);
            }
            let tags: &[&str] = if attribute.is_empty() { &[] } else { &["gnu"] };
            sink.push_tagged(
                Facet::FunctionPurity,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
                tags,
            );
        }
    }
}

/// A function whose argument is the same constant at every call site.
///
/// With one call site the constant can simply be propagated in. With several sites that all
/// pass the same value it takes a scan of every call, which is the interprocedural version of
/// the same idea. The mixed shape is the control: one site passes something else, so the
/// constant is not in fact known and a compiler that specialized anyway is wrong.
fn constant_args(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["one-site", "all-sites-agree", "sites-disagree"];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::ConstantArgs) {
                return;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("a {} argument that is constant, {shape}", ty.c_name()));
            program.top(format!("static {name} scale({name} value, {name} factor) {{"));
            program.top("    return value * factor;".to_owned());
            program.top("}".to_owned());
            program.input(ty, "seed", 3);
            program.blank();
            program.line(format!("{name} total = 0;"));
            let total: i128 = match shape {
                "one-site" => {
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    30
                }
                "all-sites-agree" => {
                    for _ in 0..3 {
                        program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    }
                    90
                }
                _ => {
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    program.line(format!("total += scale(seed, {});", lit(ty, 4)));
                    72
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", total);
            sink.push(
                Facet::ConstantArgs,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// Definitions nothing refers to.
///
/// A static function nobody calls and a static variable nobody reads should not reach the
/// object file at all. The count axis makes the difference measurable: twenty unused functions
/// that survive are twenty times the evidence that removal did not happen.
fn reachability(sink: &mut Sink<'_>) {
    for &count in &[1usize, 5, 20] {
        for &kind in &["functions", "variables", "chain"] {
            if !sink.wants(Facet::Reachability) {
                return;
            }
            let mut program = Program::new(format!("{count} unreferenced {kind}"));
            match kind {
                "functions" => {
                    for at in 0..count {
                        program.top(format!("static int unused{at}(int value) {{"));
                        program.top(format!("    return value * {} + {at};", at + 1));
                        program.top("}".to_owned());
                    }
                }
                "variables" => {
                    for at in 0..count {
                        program.top(format!("static int unused{at} = {at};"));
                    }
                }
                _ => {
                    // A chain, so removing it takes more than one pass: the last function is
                    // called by the one before it and nothing calls the first.
                    for at in 0..count {
                        if at == 0 {
                            program.top(
                                "static int link0(int value) { return value + 1; }".to_owned(),
                            );
                        } else {
                            program.top(format!(
                                "static int link{at}(int value) {{ return link{}(value) + 1; }}",
                                at - 1
                            ));
                        }
                    }
                }
            }
            program.input(Ty::I32, "seed", 4);
            program.blank();
            program.check(Ty::I32, "seed + 1", 5);
            sink.push(
                Facet::Reachability,
                Axes::of([("count", &count.to_string()), ("kind", kind)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// A call through a pointer whose target is known.
///
/// The three shapes need three different amounts of work to see through. A pointer assigned a
/// function and called straight away is a peephole. A pointer that comes out of a table with
/// a constant index needs the table to be known constant. A pointer chosen by a branch has two
/// possible targets, which a compiler can turn into a test and two direct calls or can leave
/// alone. The last shape is the control, where the target genuinely is not known.
fn devirtualize(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["direct-assign", "constant-table", "two-targets", "unknown"];
    for &shape in SHAPES {
        if !sink.wants(Facet::Devirtualize) {
            return;
        }
        let mut program = Program::new(format!("an indirect call, {shape}"));
        program.top("static int twice(int value) { return value * 2; }".to_owned());
        program.top("static int thrice(int value) { return value * 3; }".to_owned());
        program.top("static int (*const table[2])(int) = { twice, thrice };".to_owned());
        program.input(Ty::I32, "seed", 7);
        program.input(Ty::I32, "pick", 1);
        program.blank();
        let answer = match shape {
            "direct-assign" => {
                program.line("int (*call)(int) = twice;");
                14
            }
            "constant-table" => {
                program.line("int (*call)(int) = table[1];");
                21
            }
            "two-targets" => {
                program.line("int (*call)(int) = pick ? thrice : twice;");
                21
            }
            _ => {
                program.line("int (*call)(int) = table[pick & 1];");
                21
            }
        };
        program.blank();
        program.check(Ty::I32, "call(seed)", answer);
        sink.push(Facet::Devirtualize, Axes::of([("shape", shape)]), Dialect::C17, program);
    }
}

/// What a callee writes, and what the caller is allowed to do about it.
///
/// Purity answers this question with one bit for the whole function. These are the finer
/// answers, and there are three of them here. A callee handed two pointers that writes through
/// one of them and not the other. A callee that writes nothing at all. A callee lent the
/// address of a local that does not keep it, so the call after it cannot reach that local. In
/// each of those the caller may take a pair of loads of the object the call did not touch out
/// of the loop, and the program prints the same number either way.
///
/// Beside each of them is the shape one line different where the same move is wrong. The
/// callee writes through the second pointer as well, or it writes the global the loop is
/// reading, or it keeps the address it was lent and the call after it writes through that. Each
/// of those prints a different number, so every pair is a test of the answer and not only a
/// measurement of it, and a compiler that gets the optimistic half by guessing gets the other
/// half wrong.
///
/// The loop reads two elements rather than one, and that is on purpose. A single load that
/// feeds an add folds into the add on x86 and costs nothing extra, so hoisting it changes the
/// instruction count by zero and the evidence disappears. Two loads and the add between them do
/// not fold, so the shape that hoists runs two fewer instructions a time round, which is two
/// thousand over the loop and far above the few dozen that one run differs from the next.
///
/// Every helper is `noinline`, which is why these are tagged `gnu`. Without it a compiler with
/// an inliner answers all of these by inlining the callee, and then the case says nothing about
/// what the effects of that callee were worked out to be.
fn memory_effects(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "writes-one-array",
        "writes-both-arrays",
        "reads-only",
        "writes-what-is-read",
        "lends-a-local",
        "keeps-a-local",
    ];
    // A thousand times round, because the evidence here is the instruction count and a handful
    // of saved loads does not show above the noise of starting a process.
    const TRIPS: i128 = 1000;
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::MemoryEffects) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("{name} callee, {}", shape.replace('-', " ")));
            let apart = "static __attribute__((noinline)) ";
            let one = lit(ty, 1);
            match shape {
                "writes-one-array" | "writes-both-arrays" => {
                    program.top(format!("static {name} left[4];"));
                    program.top(format!("static {name} right[4];"));
                    program.top(format!("{apart}void bump({name} *out, {name} *in) {{"));
                    program.top("    out[0] = out[0] + in[1];".to_owned());
                    if shape == "writes-one-array" {
                        program.top(format!("    out[2] = out[2] + {one};"));
                    } else {
                        program.top(format!("    in[2] = in[2] + {one};"));
                    }
                    program.top("}".to_owned());
                }
                "reads-only" | "writes-what-is-read" => {
                    program.top(format!("static {name} table[4];"));
                    program.top(format!("static {name} watch[4];"));
                    program.top(format!("{apart}{name} scan({name} *in) {{"));
                    if shape == "writes-what-is-read" {
                        program.top(format!("    watch[1] = watch[1] + {one};"));
                    }
                    program.top("    return in[0] + in[3];".to_owned());
                    program.top("}".to_owned());
                }
                "lends-a-local" => {
                    program.top(format!("static {name} anchor;"));
                    program.top(format!("{apart}{name} sum_of({name} *of) {{"));
                    program.top("    return of[0] + of[1];".to_owned());
                    program.top("}".to_owned());
                    program.top(format!("{apart}void tick(void) {{"));
                    program.top(format!("    anchor = anchor + {one};"));
                    program.top("}".to_owned());
                }
                _ => {
                    program.top(format!("static {name} *kept;"));
                    program.top(format!("{apart}{name} sum_of({name} *of) {{"));
                    program.top("    kept = of;".to_owned());
                    program.top("    return of[0] + of[1];".to_owned());
                    program.top("}".to_owned());
                    program.top(format!("{apart}void tick(void) {{"));
                    program.top(format!("    *kept = *kept + {one};"));
                    program.top("}".to_owned());
                }
            }
            program.input(ty, "seed", 6);
            program.blank();
            match shape {
                "writes-one-array" | "writes-both-arrays" => {
                    program.line("left[0] = seed;");
                    program.line(format!("right[1] = seed + {};", lit(ty, 1)));
                    program.line(format!("right[2] = seed + {};", lit(ty, 2)));
                    program.line(format!("right[3] = seed + {};", lit(ty, 3)));
                }
                "reads-only" | "writes-what-is-read" => {
                    program.line("table[0] = seed;");
                    program.line(format!("table[3] = seed + {};", lit(ty, 1)));
                    program.line(format!("watch[1] = seed + {};", lit(ty, 2)));
                    program.line(format!("watch[2] = seed + {};", lit(ty, 3)));
                }
                _ => {
                    program.line(format!("{name} local[3];"));
                    program.line("local[0] = seed;");
                    program.line(format!("local[1] = seed + {};", lit(ty, 1)));
                    program.line(format!("local[2] = seed + {};", lit(ty, 2)));
                }
            }
            program.line(format!("{name} total = 0;"));
            program.line(format!("for (int i = 0; i < {TRIPS}; i++) {{"));
            match shape {
                "writes-one-array" | "writes-both-arrays" => {
                    program.line_at(1, "bump(left, right);");
                    program.line_at(1, "total += right[2] + right[3];");
                }
                "reads-only" | "writes-what-is-read" => {
                    program.line_at(1, "total += scan(table);");
                    program.line_at(1, "total += watch[1] + watch[2];");
                }
                _ => {
                    program.line_at(1, "total += sum_of(local);");
                    program.line_at(1, "tick();");
                    program.line_at(1, "total += local[0] + local[2];");
                }
            }
            program.line("}");
            program.blank();
            // Every number here is worked out from a seed of six. The checks after the total
            // are what says the calls really ran, since the total on its own cannot tell a loop
            // that called a thousand times from one that called once and kept the answer. They
            // are also what keeps the write inside the callee alive: a store to a static
            // nothing ever reads is a store a whole program pass is entitled to throw away, and
            // then there would be no call left to reason about.
            let climb = TRIPS * (TRIPS - 1) / 2;
            match shape {
                "writes-one-array" => {
                    program.check(ty.promoted(), "total", 17 * TRIPS);
                    program.check(ty.promoted(), "left[0]", 6 + 7 * TRIPS);
                    program.check(ty.promoted(), "left[2]", TRIPS);
                }
                "writes-both-arrays" => {
                    program.check(ty.promoted(), "total", 18 * TRIPS + climb);
                    program.check(ty.promoted(), "left[0]", 6 + 7 * TRIPS);
                    program.check(ty.promoted(), "right[2]", 8 + TRIPS);
                }
                "reads-only" => {
                    program.check(ty.promoted(), "total", 30 * TRIPS);
                    program.check(ty.promoted(), "table[0]", 6);
                    program.check(ty.promoted(), "watch[1]", 8);
                }
                "writes-what-is-read" => {
                    program.check(ty.promoted(), "total", 31 * TRIPS + climb);
                    program.check(ty.promoted(), "table[0]", 6);
                    program.check(ty.promoted(), "watch[1]", 8 + TRIPS);
                }
                "lends-a-local" => {
                    program.check(ty.promoted(), "total", 27 * TRIPS);
                    program.check(ty.promoted(), "local[0]", 6);
                    program.check(ty.promoted(), "anchor", TRIPS);
                }
                _ => {
                    program.check(ty.promoted(), "total", 28 * TRIPS + 2 * climb);
                    program.check(ty.promoted(), "local[0]", 6 + TRIPS);
                    program.check(ty.promoted(), "local[2]", 8);
                }
            }
            sink.push_tagged(
                Facet::MemoryEffects,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
                &["gnu"],
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
    fn inlining_covers_the_small_and_the_large_from_one_site_and_from_twenty() {
        let cases = cases_for(Facet::Inline);
        let corners: Vec<(&str, &str)> = cases
            .iter()
            .filter(|c| c.axes.get("type") == Some("i32"))
            .filter_map(|c| Some((c.axes.get("body")?, c.axes.get("sites")?)))
            .collect();
        for wanted in [("1", "1"), ("16", "20"), ("1", "20"), ("16", "1")] {
            assert!(corners.contains(&wanted), "no case for {wanted:?}");
        }
    }

    #[test]
    fn an_inlined_helper_adds_the_triangular_number_of_its_body_size() {
        let cases = cases_for(Facet::Inline);
        let case = cases
            .iter()
            .find(|c| {
                c.axes.get("type") == Some("i32")
                    && c.axes.get("body") == Some("4")
                    && c.axes.get("sites") == Some("1")
            })
            .unwrap();
        // One call site passing zero, and a helper that adds one, two, three and four.
        assert_eq!(output(case), "10\n");
    }

    #[test]
    fn the_deep_tail_recursion_cases_say_so_with_a_tag() {
        let cases = cases_for(Facet::TailCall);
        for case in &cases {
            let deep = case.axes.get("depth") == Some("5000");
            assert_eq!(case.has_tag("needs-tail-calls"), deep, "{}", case.id);
        }
    }

    #[test]
    fn every_tail_recursion_shape_counts_up_to_its_depth() {
        for case in cases_for(Facet::TailCall) {
            let depth = case.axes.get("depth").unwrap();
            assert_eq!(output(&case), format!("{depth}\n"), "{}", case.id);
        }
    }

    #[test]
    fn the_cases_that_use_an_attribute_are_tagged_and_the_others_are_not() {
        for case in cases_for(Facet::FunctionPurity) {
            let declared = case.axes.get("shape").is_some_and(|s| s.starts_with("declared"));
            assert_eq!(case.has_tag("gnu"), declared, "{}", case.id);
            assert_eq!(case.source.contains("__attribute__"), declared, "{}", case.id);
        }
    }

    #[test]
    fn the_impure_function_also_checks_how_many_times_it_ran() {
        let cases = cases_for(Facet::FunctionPurity);
        let case = cases
            .iter()
            .find(|c| c.axes.get("shape") == Some("impure") && c.axes.get("type") == Some("i32"))
            .unwrap();
        assert_eq!(output(case), "60\n5\n");
    }

    #[test]
    fn the_disagreeing_call_sites_expect_a_different_total_from_the_agreeing_ones() {
        let cases = cases_for(Facet::ConstantArgs);
        let agree = cases
            .iter()
            .find(|c| {
                c.axes.get("shape") == Some("all-sites-agree") && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        let disagree = cases
            .iter()
            .find(|c| {
                c.axes.get("shape") == Some("sites-disagree") && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        assert_eq!(output(agree), "90\n");
        assert_eq!(output(disagree), "72\n");
    }

    #[test]
    fn twenty_unused_definitions_make_a_longer_program_than_one() {
        let cases = cases_for(Facet::Reachability);
        let one = cases
            .iter()
            .find(|c| c.axes.get("count") == Some("1") && c.axes.get("kind") == Some("functions"))
            .unwrap();
        let many = cases
            .iter()
            .find(|c| c.axes.get("count") == Some("20") && c.axes.get("kind") == Some("functions"))
            .unwrap();
        assert_eq!(one.source.matches("static int unused").count(), 1);
        assert_eq!(many.source.matches("static int unused").count(), 20);
        assert!(many.source.len() > one.source.len());
        assert_eq!(output(one), output(many));
    }

    #[test]
    fn every_devirtualization_shape_calls_something_and_knows_what_it_returns() {
        let cases = cases_for(Facet::Devirtualize);
        assert_eq!(cases.len(), 4);
        let direct = cases.iter().find(|c| c.axes.get("shape") == Some("direct-assign")).unwrap();
        assert_eq!(output(direct), "14\n");
        for case in &cases {
            assert!(case.source.contains("call(seed)"), "{}", case.id);
        }
    }

    #[test]
    fn each_memory_effect_pair_prints_a_different_total() {
        let cases = cases_for(Facet::MemoryEffects);
        let shape = |want: &str| {
            let case = cases
                .iter()
                .find(|c| c.axes.get("shape") == Some(want) && c.axes.get("type") == Some("i32"))
                .unwrap();
            output(case).to_owned()
        };
        // The loop reads right[2] and right[3] a thousand times over. The callee that leaves
        // that array alone sees eight and nine every time round, and the one that adds to it
        // sees nine and upwards.
        assert_eq!(shape("writes-one-array"), "17000\n7006\n1000\n");
        assert_eq!(shape("writes-both-arrays"), "517500\n7006\n1008\n");
        // The callee that writes nothing leaves the watched pair at eight and nine. The one
        // that writes the first of them walks it up to a thousand and eight.
        assert_eq!(shape("reads-only"), "30000\n6\n8\n");
        assert_eq!(shape("writes-what-is-read"), "530500\n6\n1008\n");
        // The callee that keeps the address it was lent gives the tick beside it something to
        // write, so the local goes up by one a time round instead of staying at the seed.
        assert_eq!(shape("lends-a-local"), "27000\n6\n1000\n");
        assert_eq!(shape("keeps-a-local"), "1027000\n1006\n8\n");
    }

    #[test]
    fn every_memory_effect_callee_is_kept_out_of_line_and_says_so() {
        let cases = cases_for(Facet::MemoryEffects);
        assert_eq!(cases.len(), 24);
        for case in &cases {
            assert!(case.source.contains("__attribute__((noinline))"), "{}", case.id);
            assert!(case.has_tag("gnu"), "{}", case.id);
            assert_eq!(output(case).lines().count(), 3, "{}", case.id);
        }
    }
}
