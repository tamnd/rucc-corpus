//! The transformations that need more than one function.
//!
//! Most of it is a whole program in one translation unit, with the helpers marked `static` so
//! the compiler is allowed to reason about all of their call sites. That is the setting these
//! optimizations were designed for, and it is also the setting most C programs are actually
//! compiled in once the headers are expanded.
//!
//! The exceptions are the few shapes that are only a question when the body is somewhere the
//! compiler cannot see it, which is `declared_purity` and the last three shapes of
//! `call_motion`. Those build their cases out of more than one unit, the way `link.rs` does.

use super::lit;
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, Unit};

/// The types used here.
const TYPES: &[Ty] = Ty::WIDE;

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    inline(sink);
    tail_call(sink);
    tail_dispatch(sink);
    function_purity(sink);
    constant_args(sink);
    unused_params(sink);
    unused_returns(sink);
    declared_purity(sink);
    reachability(sink);
    devirtualize(sink);
    memory_effects(sink);
    call_motion(sink);
    calls_across_units(sink);
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
/// the same idea.
///
/// The rest of the shapes split into the ones a propagation has to reach through and the ones it
/// has to refuse. `handed-on` and `one-more-level` are the transitive cases, where the constant
/// only arrives at the inner function by way of a wrapper, and the second of the two puts
/// arithmetic in the way so the answer needs the constant to be folded before it can be read off.
/// `recursive` is the shape that wants an optimistic start, because the self call passes on
/// whatever the parameter already is and reading that as a disagreement loses the real call site.
///
/// The refusals are `sites-disagree`, where one site passes something else so the constant is not
/// in fact known, `address-escapes`, where the helper is also reached through a pointer so there
/// are call sites nobody counted, `other-objects-call-it`, where the helper is not static and its
/// other call sites are in files this compilation never sees, and `through-varargs`, where the
/// position a call passes and the position the body reads are not the same list. A compiler that
/// substituted a constant in any of those would print a different total, which is what the check
/// at the end is there to catch.
fn constant_args(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "one-site",
        "all-sites-agree",
        "sites-disagree",
        "handed-on",
        "one-more-level",
        "recursive",
        "address-escapes",
        "other-objects-call-it",
        "through-varargs",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::ConstantArgs) {
                return;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("a {} argument that is constant, {shape}", ty.c_name()));
            let total: i128 = match shape {
                "one-site" => {
                    scale(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    30
                }
                "all-sites-agree" => {
                    scale(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    for _ in 0..3 {
                        program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    }
                    90
                }
                "sites-disagree" => {
                    scale(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    program.line(format!("total += scale(seed, {});", lit(ty, 4)));
                    72
                }
                "handed-on" => {
                    scale(&mut program, ty, "static ");
                    program.top(format!("static {name} outer({name} value, {name} factor) {{"));
                    program.top("    return scale(value, factor);".to_owned());
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += outer(seed, {});", lit(ty, 10)));
                    }
                    60
                }
                "one-more-level" => {
                    scale(&mut program, ty, "static ");
                    program.top(format!("static {name} outer({name} value, {name} factor) {{"));
                    program.top(format!("    return scale(value, ({name})(factor + 1));"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += outer(seed, {});", lit(ty, 9)));
                    }
                    60
                }
                "recursive" => {
                    program.top(format!(
                        "static {name} down({name} value, {name} factor, {name} depth) {{"
                    ));
                    program.top(format!("    if (depth == {}) {{", lit(ty, 0)));
                    program.top("        return value * factor;".to_owned());
                    program.top("    }".to_owned());
                    program.top(format!("    return down(value, factor, ({name})(depth - 1));"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!(
                            "total += down(seed, {}, {});",
                            lit(ty, 10),
                            lit(ty, 2)
                        ));
                    }
                    60
                }
                "address-escapes" => {
                    scale(&mut program, ty, "static ");
                    program.top(format!("static {name} (*chosen)({name}, {name}) = scale;"));
                    running_total(&mut program, ty);
                    program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    program.line(format!("total += chosen(seed, {});", lit(ty, 4)));
                    42
                }
                "other-objects-call-it" => {
                    scale(&mut program, ty, "");
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += scale(seed, {});", lit(ty, 10)));
                    }
                    60
                }
                _ => {
                    program.include("stdarg.h");
                    program.top(format!("static {name} pick({name} value, ...) {{"));
                    program.top("    va_list rest;".to_owned());
                    program.top(format!("    {name} factor;"));
                    program.top("    va_start(rest, value);".to_owned());
                    program.top(format!("    factor = va_arg(rest, {name});"));
                    program.top("    va_end(rest);".to_owned());
                    program.top("    return value * factor;".to_owned());
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += pick(seed, {});", lit(ty, 10)));
                    }
                    60
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

/// The multiplying helper every shape above leans on, at the linkage the shape wants.
fn scale(program: &mut Program, ty: Ty, linkage: &str) {
    let name = ty.c_name();
    program.top(format!("{linkage}{name} scale({name} value, {name} factor) {{"));
    program.top("    return value * factor;".to_owned());
    program.top("}".to_owned());
}

/// The seed the helper is called with and the total the calls add up into.
///
/// The seed is read out of a volatile so the value the helper multiplies is not itself a
/// constant. What the shapes are about is the other argument.
fn running_total(program: &mut Program, ty: Ty) {
    program.input(ty, "seed", 3);
    program.blank();
    program.line(format!("{} total = 0;", ty.c_name()));
}

/// Parameters the body never names, and the arguments the calls were passing to them.
///
/// The shapes are the two halves of what the transformation has to get right. One half is the
/// removal itself, where the parameter goes, the argument goes with it, and the answer does not
/// move: a parameter in the middle of the list, every parameter at once, a parameter that only
/// became unread because the propagation put a constant in its place, and a parameter reached
/// through a second helper, which is the case where two locals the caller was only adding
/// together go as well.
///
/// The other half is what has to be left alone, and it is the half worth measuring, because the
/// cost of getting it wrong is a program that stops working rather than a program that is
/// slower. A function whose address the unit hands out, a function another object can call and a
/// variadic function all keep every parameter they have. So does a parameter only the recursive
/// call hands on, which is a real limitation rather than a rule, and it is here so that the
/// report says whether it is still one.
///
/// Two shapes watch what has to survive the argument going away. One passes a call that bumps a
/// counter, and the counter has to end up bumped. One passes the address of a local, and the
/// local has to keep the value it was given.
fn unused_params(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "one-unread",
        "middle-unread",
        "all-unread",
        "transitive-sum",
        "constant-then-unread",
        "argument-has-a-side-effect",
        "argument-is-an-address",
        "recursive-only",
        "address-escapes",
        "other-objects-call-it",
        "variadic",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::UnusedParams) {
                return;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("a {} parameter nothing reads, {shape}", ty.c_name()));
            let total: i128 = match shape {
                "one-unread" => {
                    ignores(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += keep(seed, ({name})(seed * 5));"));
                    }
                    8
                }
                "middle-unread" => {
                    program.top(format!(
                        "static {name} pick({name} first, {name} ignored, {name} last) {{"
                    ));
                    program.top(format!("    return ({name})(first * 10 + last);"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program.line(format!(
                        "total += pick(seed, ({name})(seed + 7), ({name})(seed + 1));"
                    ));
                    program.line(format!("total += pick(seed, seed, ({name})(seed + 2));"));
                    69
                }
                "all-unread" => {
                    program.top(format!("static {name} fixed({name} one, {name} two) {{"));
                    program.top(format!("    return {};", lit(ty, 7)));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += fixed(seed, ({name})(seed + 1));"));
                    }
                    14
                }
                "transitive-sum" => {
                    ignores(&mut program, ty, "static ");
                    program.top(format!(
                        "static {name} outer({name} value, {name} left, {name} right) {{"
                    ));
                    program.top(format!("    return keep(value, ({name})(left + right));"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!(
                            "total += outer(seed, ({name})(seed * 2), ({name})(seed + 5));"
                        ));
                    }
                    8
                }
                "constant-then-unread" => {
                    program.top(format!("static {name} scaled({name} value, {name} factor) {{"));
                    program.top(format!("    return ({name})(value * factor);"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line(format!("total += scaled(seed, {});", lit(ty, 10)));
                    }
                    60
                }
                "argument-has-a-side-effect" => {
                    ignores(&mut program, ty, "static ");
                    program.top(format!("static {name} counter = {};", lit(ty, 0)));
                    program.top(format!("static {name} bump(void) {{"));
                    program.top(format!("    counter = ({name})(counter + 1);"));
                    program.top("    return counter;".to_owned());
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line("total += keep(seed, bump());".to_owned());
                    }
                    program.blank();
                    program.check(ty.promoted(), "counter", 2);
                    8
                }
                "argument-is-an-address" => {
                    program.top(format!("static {name} note({name} value, {name} *ignored) {{"));
                    program.top(format!("    return ({name})(value + 1);"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program.line(format!("{name} spot = ({name})(seed * 3);"));
                    program.line("total += note(seed, &spot);".to_owned());
                    program.blank();
                    program.check(ty.promoted(), "spot", 9);
                    4
                }
                "recursive-only" => {
                    program.top(format!(
                        "static {name} walk({name} value, {name} carried, {name} depth) {{"
                    ));
                    program.top(format!("    if (depth == {}) {{", lit(ty, 0)));
                    program.top("        return value;".to_owned());
                    program.top("    }".to_owned());
                    program.top(format!(
                        "    return walk(({name})(value + 1), carried, ({name})(depth - 1));"
                    ));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program
                        .line(format!("total += walk(seed, ({name})(seed * 4), {});", lit(ty, 3)));
                    6
                }
                "address-escapes" => {
                    ignores(&mut program, ty, "static ");
                    program.top(format!("static {name} (*chosen)({name}, {name}) = keep;"));
                    running_total(&mut program, ty);
                    program.line("total += keep(seed, seed);".to_owned());
                    program.line("total += chosen(seed, seed);".to_owned());
                    8
                }
                "other-objects-call-it" => {
                    ignores(&mut program, ty, "");
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line("total += keep(seed, seed);".to_owned());
                    }
                    8
                }
                _ => {
                    program.include("stdarg.h");
                    program.top(format!("static {name} first({name} value, ...) {{"));
                    program.top(format!("    return ({name})(value + 1);"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    for _ in 0..2 {
                        program.line("total += first(seed, seed);".to_owned());
                    }
                    8
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", total);
            sink.push(
                Facet::UnusedParams,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The helper most of the shapes above lean on: two parameters, and a body that only names one.
fn ignores(program: &mut Program, ty: Ty, linkage: &str) {
    let name = ty.c_name();
    program.top(format!("{linkage}{name} keep({name} value, {name} ignored) {{"));
    program.top(format!("    return ({name})(value + 1);"));
    program.top("}".to_owned());
}

/// Return values no call reads, and the results the calls were producing.
///
/// The sibling of `unused-params` asked the other way round. A parameter is dead because of what
/// the callee does not do, and a return value is dead because of what the callers do not do, so
/// every shape here is about the call sites rather than about the body.
///
/// The helper marks a global on its way past, which is the point. Whether the value it hands back
/// is read has nothing to do with whether the call has to happen, and a compiler that takes the
/// return away and takes the mark with it is caught by the check on the global rather than by the
/// check on the total.
///
/// Three shapes are the removal: both calls ignoring the value, a helper reached through a second
/// helper where the outer return has to go before the inner one can, and a helper handing back a
/// pointer into a table it just wrote, where the write has to survive.
///
/// Four shapes keep the return value, and they are what the facet is for. One call reading it is
/// enough, whether the reader is an addition or the condition of an if. A helper whose address the
/// unit hands out keeps it, and so does a helper another object can call. The recursive one is
/// here to be watched: its own call reads what it hands back, so the answer depends on whether the
/// compiler works round a cycle or stops at it.
///
/// The last shape has two helpers side by side, one read and one not, so the report says the
/// decision is per function and not per file.
fn unused_returns(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "result-ignored",
        "one-call-reads",
        "read-in-a-condition",
        "through-a-helper",
        "returns-a-pointer",
        "recursive",
        "address-escapes",
        "other-objects-call-it",
        "one-helper-of-two",
    ];
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::UnusedReturns) {
                return;
            }
            let name = ty.c_name();
            let mut program =
                Program::new(format!("a {} return value no call reads, {shape}", ty.c_name()));
            let total: i128 = match shape {
                "result-ignored" => {
                    noting(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    program.line("note(seed);".to_owned());
                    program.line(format!("note(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    0
                }
                "one-call-reads" => {
                    noting(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    program.line("total += note(seed);".to_owned());
                    program.line(format!("note(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    9
                }
                "read-in-a-condition" => {
                    noting(&mut program, ty, "static ");
                    running_total(&mut program, ty);
                    program.line("if (note(seed)) {".to_owned());
                    program.line(format!("    total += {};", lit(ty, 5)));
                    program.line("}".to_owned());
                    program.line(format!("note(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    5
                }
                "through-a-helper" => {
                    noting(&mut program, ty, "static ");
                    program.top(format!("static {name} outer({name} value) {{"));
                    program.top("    return note(value);".to_owned());
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program.line("outer(seed);".to_owned());
                    program.line(format!("outer(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    0
                }
                "returns-a-pointer" => {
                    program.top(format!("static {name} table[4] = {{ 0, 0, 0, 0 }};"));
                    program.top(format!("static {name} *slot({name} at) {{"));
                    program.top(format!("    table[at] = ({name})(table[at] + 1);"));
                    program.top("    return &table[at];".to_owned());
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program.line(format!("slot(({name})(seed - 2));"));
                    program.line(format!("slot(({name})(seed - 2));"));
                    program.blank();
                    program.check(ty.promoted(), "table[1]", 2);
                    0
                }
                "recursive" => {
                    program.top(format!("static {name} seen = {};", lit(ty, 0)));
                    program.top(format!("static {name} walk({name} value) {{"));
                    program.top(format!("    seen = ({name})(seen + value);"));
                    program.top(format!("    if (value == {}) {{", lit(ty, 0)));
                    program.top(format!("        return {};", lit(ty, 0)));
                    program.top("    }".to_owned());
                    program.top(format!("    return walk(({name})(value - 1));"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program.line("walk(seed);".to_owned());
                    program.blank();
                    program.check(ty.promoted(), "seen", 6);
                    0
                }
                "address-escapes" => {
                    noting(&mut program, ty, "static ");
                    program.top(format!("static {name} (*chosen)({name}) = note;"));
                    running_total(&mut program, ty);
                    program.line("note(seed);".to_owned());
                    program.line(format!("chosen(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    0
                }
                "other-objects-call-it" => {
                    noting(&mut program, ty, "");
                    running_total(&mut program, ty);
                    program.line("note(seed);".to_owned());
                    program.line(format!("note(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    0
                }
                _ => {
                    noting(&mut program, ty, "static ");
                    program.top(format!("static {name} lift({name} value) {{"));
                    program.top(format!("    seen = ({name})(seen + value);"));
                    program.top(format!("    return ({name})(value + 100);"));
                    program.top("}".to_owned());
                    running_total(&mut program, ty);
                    program.line("total += lift(seed);".to_owned());
                    program.line(format!("note(({name})(seed + 1));"));
                    program.blank();
                    program.check(ty.promoted(), "seen", 7);
                    103
                }
            };
            program.blank();
            program.check(ty.promoted(), "total", total);
            sink.push(
                Facet::UnusedReturns,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
            );
        }
    }
}

/// The helper most of the shapes above lean on: a mark on a global, and a value handed back.
///
/// The mark is what says the call happened. Taking the return value away must not take it with it,
/// and the check on `seen` is what would notice if it did.
fn noting(program: &mut Program, ty: Ty, linkage: &str) {
    let name = ty.c_name();
    program.top(format!("static {name} seen = {};", lit(ty, 0)));
    program.top(format!("{linkage}{name} note({name} value) {{"));
    program.top(format!("    seen = ({name})(seen + value);"));
    program.top(format!("    return ({name})(value * value);"));
    program.top("}".to_owned());
}

/// A callee in another file, and what its declaration promised about memory.
///
/// Everything else in this phase is one translation unit, because a compiler that can see a body
/// does not have to be told anything about it. This is the case where it cannot see one. The
/// helper is compiled on its own and the main unit holds nothing but a declaration of it, so the
/// only thing in the program that can say whether two calls are one call is
/// `__attribute__((const))` or `__attribute__((pure))` written on that declaration.
///
/// Each shape is generated twice, once with the attribute written and once without, which is the
/// pair `link-time-optimization` uses and is here for the same reason. One case that passes says
/// the compiler accepted the attribute, which is worth knowing and is not what this is about. The
/// pair says what believing the promise was worth, and the two rows sit next to each other in the
/// report so the size and instruction columns can be subtracted.
///
/// Both promises are true of the helper, so neither half of any pair is a program with undefined
/// behaviour in it. `corpus_weigh` works its answer out from its argument and touches nothing at
/// all, and `corpus_look` reads the array it is handed and writes nothing.
///
/// The last shape is the one that keeps the facet honest. A write lands between the two reads, so
/// `pure` does not make the two calls one and a compiler that merged them anyway would print the
/// wrong number rather than a smaller one.
fn declared_purity(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "the-same-argument-twice",
        "a-result-nothing-reads",
        "an-array-read-twice",
        "a-write-between-two-reads",
    ];
    /// Whether the declaration in the main unit carries the attribute.
    const PROMISE: &[(&str, bool)] = &[("written", true), ("silent", false)];
    for &ty in TYPES {
        for &shape in SHAPES {
            for &(point, written) in PROMISE {
                if !sink.wants(Facet::DeclaredPurity) {
                    return;
                }
                let name = ty.c_name();
                let reads = matches!(shape, "an-array-read-twice" | "a-write-between-two-reads");
                let attribute = match (written, reads) {
                    (false, _) => "",
                    (true, false) => "__attribute__((const)) ",
                    (true, true) => "__attribute__((pure)) ",
                };
                let helper = if reads {
                    unit(
                        "helper",
                        "A callee that reads the array it is handed and writes nothing.",
                        &format!(
                            "{name} corpus_look(const {name} *at)\n{{\n    return ({name})(at[0] + at[2]);\n}}\n"
                        ),
                    )
                } else {
                    unit(
                        "helper",
                        "A callee that works its answer out from its argument and touches nothing.",
                        &format!(
                            "{name} corpus_weigh({name} of)\n{{\n    return ({name})(of * 3 + 1);\n}}\n"
                        ),
                    )
                };
                let mut program = Program::new(format!(
                    "a {name} callee in another file, {}, promise {point}",
                    shape.replace('-', " ")
                ));
                if reads {
                    program.top(format!("{attribute}{name} corpus_look(const {name} *at);"));
                } else {
                    program.top(format!("{attribute}{name} corpus_weigh({name} of);"));
                }
                running_total(&mut program, ty);
                let total: i128 = match shape {
                    "the-same-argument-twice" => {
                        program.line("total += corpus_weigh(seed);".to_owned());
                        program.line("total += corpus_weigh(seed);".to_owned());
                        20
                    }
                    "a-result-nothing-reads" => {
                        program.line("corpus_weigh(seed);".to_owned());
                        program.line("total += corpus_weigh(seed);".to_owned());
                        10
                    }
                    "an-array-read-twice" => {
                        table(&mut program, ty);
                        program.line("total += corpus_look(table);".to_owned());
                        program.line("total += corpus_look(table);".to_owned());
                        16
                    }
                    _ => {
                        table(&mut program, ty);
                        program.line("total += corpus_look(table);".to_owned());
                        program.line(format!("table[2] = ({name})(seed + 10);"));
                        program.line("total += corpus_look(table);".to_owned());
                        24
                    }
                };
                program.blank();
                program.check(ty.promoted(), "total", total);
                let (source, expected) = program.finish();
                sink.push_case(Case::linked(
                    Facet::DeclaredPurity,
                    Axes::of([("type", ty.name()), ("shape", shape), ("promise", point)]),
                    Dialect::C17,
                    source,
                    vec![helper],
                    Vec::new(),
                    Expect::Output(expected),
                ));
            }
        }
    }
}

/// The four values the reading shapes hand to the callee.
///
/// A local rather than a global, and filled from the seed rather than written as literals, so that
/// a compiler cannot work the answer out without believing something about the call.
fn table(program: &mut Program, ty: Ty) {
    let name = ty.c_name();
    program.line(format!("{name} table[4] = {{"));
    program.line_at(1, format!("seed, ({name})(seed + 1), ({name})(seed + 2), ({name})(seed + 3)"));
    program.line("};".to_owned());
}

/// One translation unit with no `main` in it, written the way the generated ones are.
fn unit(name: &str, purpose: &str, body: &str) -> Unit {
    Unit::new(
        name,
        format!(
            "// {purpose}\n// Generated by rucc-corpus. Edit the generator, not this file.\n\n{body}"
        ),
    )
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

/// Whether a call came out of a loop.
///
/// `memory_effects` asks what a callee does to memory, and reads the answer off the loads
/// around the call. This asks the next question, which is what the caller may do with the call
/// itself. A call to a callee that reads a table the loop never writes may be taken out of the
/// loop and run once. A call to a callee that touches no memory at all may be taken out on the
/// strength of its argument alone. Neither move shows in the output when it is right, so every
/// shape here comes in a pair with the one line that makes the move wrong.
///
/// The pairs are a callee reading a table nothing writes against the same callee where the loop
/// writes that table every time round, and a callee touching no memory on an argument the loop
/// leaves alone against the same callee on an argument that changes. After them come the two
/// shapes where the output does say the answer. A callee that writes through the pointer it was
/// handed loses every write but the first if the call comes out, and the count at the end says
/// so. A call under a test that is false is handed a zero to divide by, so a compiler that puts
/// it in front of the loop faults rather than prints.
///
/// The loop runs a thousand times, as in `memory_effects`, because the evidence for the half
/// that should move is what the call costs, and one call of a few instructions does not show
/// above the noise of starting a process.
///
/// Every callee is `noinline`, which is why these are tagged `gnu`. Without it a compiler with
/// an inliner answers all of these by inlining, and the case then measures the inliner rather
/// than what the callee was worked out to do.
fn call_motion(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "reads-a-table-nothing-writes",
        "reads-a-table-the-loop-writes",
        "touches-nothing",
        "an-argument-the-loop-changes",
        "writes-through-an-argument",
        "a-call-under-a-false-test",
    ];
    // A thousand times round, for the same reason as in `memory_effects`.
    const TRIPS: i128 = 1000;
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::CallMotion) {
                return;
            }
            let name = ty.c_name();
            let mut program = Program::new(format!("{name} call, {}", shape.replace('-', " ")));
            let apart = "static __attribute__((noinline)) ";
            let one = lit(ty, 1);
            match shape {
                "reads-a-table-nothing-writes" | "reads-a-table-the-loop-writes" => {
                    program.top(format!("static {name} table[4];"));
                    if shape == "reads-a-table-nothing-writes" {
                        program.top(format!("static {name} scratch[4];"));
                    }
                    program.top(format!("{apart}{name} scan({name} *in) {{"));
                    program.top("    return in[0] + in[3];".to_owned());
                    program.top("}".to_owned());
                }
                "touches-nothing" | "an-argument-the-loop-changes" => {
                    program.top(format!("static {name} scratch[4];"));
                    program.top(format!("{apart}{name} square({name} of) {{"));
                    program.top("    return of * of;".to_owned());
                    program.top("}".to_owned());
                }
                "writes-through-an-argument" => {
                    program.top(format!("static {name} counter[2];"));
                    program.top(format!("{apart}{name} bump({name} *out) {{"));
                    program.top(format!("    out[0] = out[0] + {one};"));
                    program.top("    return out[1];".to_owned());
                    program.top("}".to_owned());
                }
                _ => {
                    program.top(format!("{apart}{name} divide({name} top, {name} bottom) {{"));
                    program.top("    return top / bottom;".to_owned());
                    program.top("}".to_owned());
                }
            }
            program.input(ty, "seed", 6);
            if shape == "a-call-under-a-false-test" {
                // Two zeroes and not one. Inside the test the compiler knows the gate was not
                // zero, so a divisor named there would be a divisor it could prove safe, and
                // the shape would stop asking anything.
                program.input(ty, "gate", 0);
                program.input(ty, "bottom", 0);
            }
            program.blank();
            match shape {
                "reads-a-table-nothing-writes" | "reads-a-table-the-loop-writes" => {
                    program.line("table[0] = seed;");
                    program.line(format!("table[3] = seed + {one};"));
                }
                "writes-through-an-argument" => program.line("counter[1] = seed;"),
                _ => (),
            }
            program.line(format!("{name} total = 0;"));
            program.line(format!("for (int i = 0; i < {TRIPS}; i++) {{"));
            // The write to `scratch` is what makes these loops write memory at all. Without it
            // there is nothing for the call to be weighed against and the move is free.
            let stir = format!("scratch[i & 3] = scratch[i & 3] + {one};");
            match shape {
                "reads-a-table-nothing-writes" => {
                    program.line_at(1, "total += scan(table);");
                    program.line_at(1, stir);
                }
                "reads-a-table-the-loop-writes" => {
                    program.line_at(1, "total += scan(table);");
                    program.line_at(1, format!("table[3] = table[3] + {one};"));
                }
                "touches-nothing" => {
                    program.line_at(1, "total += square(seed);");
                    program.line_at(1, stir);
                }
                "an-argument-the-loop-changes" => {
                    program.line_at(1, format!("total += square(seed + ({name})i);"));
                    program.line_at(1, stir);
                }
                "writes-through-an-argument" => program.line_at(1, "total += bump(counter);"),
                _ => {
                    program.line_at(1, "if (gate) {");
                    program.line_at(2, "total += divide(seed, bottom);");
                    program.line_at(1, "}");
                    program.line_at(1, format!("total += {one};"));
                }
            }
            program.line("}");
            program.blank();
            // Every number here is worked out from a seed of six. The checks after the total
            // are what tells a loop that called a thousand times from one that called once and
            // kept the answer, which is the whole question this facet asks.
            let climb = TRIPS * (TRIPS - 1) / 2;
            match shape {
                "reads-a-table-nothing-writes" => {
                    program.check(ty.promoted(), "total", 13 * TRIPS);
                    program.check(ty.promoted(), "table[3]", 7);
                    program.check(ty.promoted(), "scratch[3]", TRIPS / 4);
                }
                "reads-a-table-the-loop-writes" => {
                    program.check(ty.promoted(), "total", 13 * TRIPS + climb);
                    program.check(ty.promoted(), "table[0]", 6);
                    program.check(ty.promoted(), "table[3]", 7 + TRIPS);
                }
                "touches-nothing" => {
                    program.check(ty.promoted(), "total", 36 * TRIPS);
                    program.check(ty.promoted(), "scratch[3]", TRIPS / 4);
                }
                "an-argument-the-loop-changes" => {
                    let squares: i128 = (0..TRIPS).map(|at| (6 + at) * (6 + at)).sum();
                    program.check(ty.promoted(), "total", squares);
                    program.check(ty.promoted(), "scratch[3]", TRIPS / 4);
                }
                "writes-through-an-argument" => {
                    program.check(ty.promoted(), "total", 6 * TRIPS);
                    program.check(ty.promoted(), "counter[0]", TRIPS);
                    program.check(ty.promoted(), "counter[1]", 6);
                }
                _ => program.check(ty.promoted(), "total", TRIPS),
            }
            sink.push_tagged(
                Facet::CallMotion,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                program,
                &["gnu"],
            );
        }
    }
}

/// A call in a loop whose callee is in another translation unit.
///
/// The six shapes above all put the callee in the same file, so the compiler can work out what it
/// does and the question is only whether it acted on the answer. This is the other half. The
/// helper is compiled on its own and the main unit holds nothing but a declaration of it, so there
/// is nothing to work out. The only summary the compiler can have is the one the declaration
/// gives it, and when the declaration gives it none, leaving the call in the loop is the only
/// right answer.
///
/// Three shapes, and the three are meant to be read next to each other.
///
/// `another-unit-writes-a-counter` is the one the output catches. The helper counts its own calls
/// in a `static` of its own, and the main unit asks for the count after the loop. A compiler that
/// takes the call out prints one rather than a thousand, so this shape fails loudly rather than
/// quietly costing something.
///
/// `another-unit-is-silent` and `another-unit-promises-const` are the same program twice, over a
/// helper that genuinely touches nothing, with `__attribute__((const))` written on the
/// declaration in the second. Both print the same number. The difference between the two rows is
/// what the promise was worth, and it is the loop shaped version of what `declared_purity` asks
/// about two calls standing next to each other.
///
/// A thousand times round, as above, for the same reason.
fn calls_across_units(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["another-unit-writes-a-counter", "another-unit-is-silent", "another-unit-promises-const"];
    // A thousand times round, matching the shapes above so the rows are comparable.
    const TRIPS: i128 = 1000;
    for &ty in TYPES {
        for &shape in SHAPES {
            if !sink.wants(Facet::CallMotion) {
                return;
            }
            let name = ty.c_name();
            let counting = shape == "another-unit-writes-a-counter";
            let helper = match counting {
                true => unit(
                    "helper",
                    "A callee that counts its own calls, so the caller can say how many there were.",
                    &format!(
                        "static {name} made = {};\n\n\
                         {name} corpus_step({name} of)\n{{\n    \
                         made = ({name})(made + 1);\n    return ({name})(of * 2);\n}}\n\n\
                         {name} corpus_calls(void)\n{{\n    return made;\n}}\n",
                        lit(ty, 0)
                    ),
                ),
                false => unit(
                    "helper",
                    "A callee that works its answer out from its argument and touches nothing.",
                    &format!(
                        "{name} corpus_weigh({name} of)\n{{\n    return ({name})(of * 3 + 1);\n}}\n"
                    ),
                ),
            };
            let mut program =
                Program::new(format!("{name} call in a loop, {}", shape.replace('-', " ")));
            if counting {
                program.top(format!("{name} corpus_step({name} of);"));
                program.top(format!("{name} corpus_calls(void);"));
            } else {
                let attribute = match shape == "another-unit-promises-const" {
                    true => "__attribute__((const)) ",
                    false => "",
                };
                program.top(format!("{attribute}{name} corpus_weigh({name} of);"));
                program.top(format!("static {name} scratch[4];"));
            }
            program.input(ty, "seed", 6);
            program.blank();
            program.line(format!("{name} total = 0;"));
            program.line(format!("for (int i = 0; i < {TRIPS}; i++) {{"));
            if counting {
                program.line_at(1, "total += corpus_step(seed);");
            } else {
                program.line_at(1, "total += corpus_weigh(seed);");
                // The same write the shapes above use, so that the loop does something the call
                // has to be weighed against rather than being the only thing in it.
                program.line_at(1, format!("scratch[i & 3] = scratch[i & 3] + {};", lit(ty, 1)));
            }
            program.line("}".to_owned());
            program.blank();
            if counting {
                program.check(ty.promoted(), "total", 12 * TRIPS);
                // The line that says whether the call came out. Nothing else in the program can.
                program.check(ty.promoted(), "corpus_calls()", TRIPS);
            } else {
                program.check(ty.promoted(), "total", 19 * TRIPS);
                program.check(ty.promoted(), "scratch[3]", TRIPS / 4);
            }
            let (source, expected) = program.finish();
            let mut case = Case::linked(
                Facet::CallMotion,
                Axes::of([("type", ty.name()), ("shape", shape)]),
                Dialect::C17,
                source,
                vec![helper],
                Vec::new(),
                Expect::Output(expected),
            );
            if shape == "another-unit-promises-const" {
                case = case.tagged(&["gnu"]);
            }
            sink.push_case(case);
        }
    }
}

/// How many bytes a dispatch case walks, one call per byte, which stays in the first level cache.
const DISPATCH_TEXT: usize = 2048;

/// How many times a dispatch case walks its bytes, so that it makes two million calls and a call
/// against a jump is milliseconds apart.
const DISPATCH_ROUNDS: u64 = 1024;

/// The seed the dispatch bytes are drawn from, read through a `volatile` so nothing folds.
const DISPATCH_SEED: u64 = 7;

/// The multiplier the many state machines mix with, which is the 64 bit FNV prime.
const DISPATCH_PRIME: u64 = 0x0100_0000_01b3;

/// The shapes a dispatch case takes, and how many functions call each other in it.
///
/// Two states that alternate, four and sixteen that pick the next one with a `switch`, one
/// function of six arguments that calls itself with them rotated, and four states that pick the
/// next one out of a table of pointers, which is the one a compiler has to jump through a register
/// for.
const DISPATCH_SHAPES: &[(&str, usize)] =
    &[("alternate", 2), ("switch", 4), ("switch", 16), ("rotate", 1), ("table", 4)];

/// The bytes a dispatch case walks, drawn the way the C draws them.
fn dispatch_text() -> Vec<u8> {
    let mut drawn = DISPATCH_SEED;
    (0..DISPATCH_TEXT)
        .map(|_| {
            drawn = drawn
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (drawn >> 33) as u8
        })
        .collect()
}

/// What one walk of a dispatch case gives back, started from `round`, worked out the way the C
/// works it out.
fn dispatched(shape: &str, states: usize, text: &[u8], round: u64) -> u64 {
    let mut acc = round;
    let mut state = 0usize;
    if shape == "rotate" {
        let (mut a, mut b, mut c, mut d) = (round, 1u64, 2u64, 3u64);
        for &byte in text {
            (a, b, c, d) = (b, c, d, a.wrapping_add(u64::from(byte)));
        }
        return a
            .wrapping_add(b.wrapping_mul(3))
            .wrapping_add(c.wrapping_mul(5))
            .wrapping_add(d.wrapping_mul(7));
    }
    for &byte in text {
        let byte64 = u64::from(byte);
        match shape {
            "alternate" if state == 0 => acc = acc.wrapping_mul(31).wrapping_add(byte64),
            "alternate" => acc ^= byte64 << 3,
            "switch" => {
                acc = acc
                    .wrapping_mul(DISPATCH_PRIME)
                    .wrapping_add(byte64)
                    .wrapping_add(state as u64);
            }
            _ => acc = acc.wrapping_mul(31).wrapping_add(byte64).wrapping_add(state as u64),
        }
        state = if shape == "alternate" {
            (state + 1) % states
        } else {
            (usize::from(byte) + state) % states
        };
    }
    acc
}

/// The C of a dispatch case's functions, each of which does one byte's work and calls the next in
/// tail position.
fn dispatch_functions(program: &mut Program, shape: &str, states: usize) {
    const RETURNS: &str = "static __attribute__((noinline)) unsigned long long";
    const PARAMS: &str = "const unsigned char *p, const unsigned char *end, unsigned long long acc";
    if shape == "rotate" {
        program.top(format!(
            "{RETURNS} rotate(const unsigned char *p, const unsigned char *end, unsigned long long              a, unsigned long long b, unsigned long long c, unsigned long long d) {{"
        ));
        program.top("    if (p == end) {".to_owned());
        program.top("        return a + 3 * b + 5 * c + 7 * d;".to_owned());
        program.top("    }".to_owned());
        program.top("    return rotate(p + 1, end, b, c, d, a + *p);".to_owned());
        program.top("}".to_owned());
        return;
    }
    for state in 0..states {
        program.top(format!("{RETURNS} s{state}({PARAMS});"));
    }
    if shape == "table" {
        let names: Vec<String> = (0..states).map(|state| format!("s{state}")).collect();
        program.top(format!(
            "static unsigned long long (*const table[{states}])(const unsigned char *, const              unsigned char *, unsigned long long) = {{{}}};",
            names.join(", ")
        ));
    }
    for state in 0..states {
        program.top(format!("{RETURNS} s{state}({PARAMS}) {{"));
        program.top("    if (p == end) {".to_owned());
        program.top("        return acc;".to_owned());
        program.top("    }".to_owned());
        match shape {
            "alternate" => {
                let next = (state + 1) % states;
                let step = if state == 0 {
                    "acc * 31 + *p"
                } else {
                    "acc ^ ((unsigned long long)*p << 3)"
                };
                program.top(format!("    return s{next}(p + 1, end, {step});"));
            }
            "switch" => {
                program.top(format!(
                    "    unsigned long long next = acc * {DISPATCH_PRIME}ull + *p + {state};"
                ));
                program.top(format!("    switch ((*p + {state}) % {states}) {{"));
                for next in 0..states {
                    let label = if next + 1 == states {
                        "default:".to_owned()
                    } else {
                        format!("case {next}:")
                    };
                    program.top(format!("    {label}"));
                    program.top(format!("        return s{next}(p + 1, end, next);"));
                }
                program.top("    }".to_owned());
            }
            _ => {
                program.top(format!(
                    "    return table[(*p + {state}) % {states}](p + 1, end, acc * 31 + *p +                      {state});"
                ));
            }
        }
        program.top("}".to_owned());
    }
}

/// State machines whose states call each other in tail position, walked over two thousand bytes a
/// thousand times.
///
/// The `tail-call` facet above says whether the answer is right. These say what the calls cost,
/// which is where turning a call into a jump shows: a lexer or an interpreter written as functions
/// calling each other makes one call per byte, and a call and a return per byte is most of what it
/// does. Every function is `noinline`, so the calls are still calls when the back end sees them,
/// and a walk is two thousand deep, which fits in any stack at `-O0`.
fn tail_dispatch(sink: &mut Sink<'_>) {
    let text = dispatch_text();
    for &(shape, states) in DISPATCH_SHAPES {
        if !sink.wants(Facet::TailDispatch) {
            return;
        }
        let mut program = Program::new(format!(
            "{shape} dispatch between {states} functions in tail position, done many times"
        ));
        dispatch_functions(&mut program, shape, states);
        program.line("unsigned long long total = 0;");
        program.input(Ty::U64, "seed", i128::from(DISPATCH_SEED));
        program.blank();
        program.line(format!("static unsigned char text[{DISPATCH_TEXT}];"));
        program.line("unsigned long long drawn = seed;");
        program.line(format!("for (int i = 0; i < {DISPATCH_TEXT}; i++) {{"));
        program.line_at(1, "drawn = drawn * 6364136223846793005ull + 1442695040888963407ull;");
        program.line_at(1, "text[i] = (unsigned char)(drawn >> 33);");
        program.line("}");
        program.line(format!("for (int round = 0; round < {DISPATCH_ROUNDS}; round++) {{"));
        let end = format!("text + {DISPATCH_TEXT}");
        if shape == "rotate" {
            program.line_at(
                1,
                format!("total += rotate(text, {end}, (unsigned long long)round, 1, 2, 3);"),
            );
        } else {
            program.line_at(1, format!("total += s0(text, {end}, (unsigned long long)round);"));
        }
        program.line("}");
        program.blank();
        let expected = (0..DISPATCH_ROUNDS)
            .fold(0u64, |total, round| total.wrapping_add(dispatched(shape, states, &text, round)));
        program.check(Ty::U64, "total", i128::from(expected));
        let axes = Axes::of([("shape", shape), ("states", &states.to_string())]);
        sink.push_tagged(Facet::TailDispatch, axes, Dialect::C17, program, &["gnu"]);
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
    fn a_dispatch_case_is_every_shape_and_calls_the_next_state_in_tail_position() {
        let cases = cases_for(Facet::TailDispatch);
        assert_eq!(cases.len(), super::DISPATCH_SHAPES.len());
        for case in &cases {
            assert!(case.has_tag("gnu"), "{}", case.id);
            assert!(
                case.source.contains("return s0(p + 1")
                    || case.source.contains("return rotate(p + 1")
                    || case.source.contains("return s1(p + 1")
                    || case.source.contains("return table["),
                "{}",
                case.id
            );
        }
    }

    #[test]
    fn a_dispatch_answer_is_the_state_machine_run_by_hand() {
        // Three bytes through the two states and through the table, written out step by step so a
        // slip in the walk shows up as a difference.
        let text = [5u8, 250, 3];
        let alternate = (((9u64 * 31 + 5) ^ (250 << 3)) * 31) + 3;
        assert_eq!(super::dispatched("alternate", 2, &text, 9), alternate);
        // State 0 reads 5 and goes to 5 % 4 = 1, state 1 reads 250 and goes to 251 % 4 = 3.
        let table = ((9u64 * 31 + 5) * 31 + 250 + 1) * 31 + 3 + 3;
        assert_eq!(super::dispatched("table", 4, &text, 9), table);
        // The rotation moves a to the end with the byte added, so after three bytes d is b + 3.
        let (a, b, c, d) = (3u64, 9 + 5, 1 + 250, 2 + 3);
        assert_eq!(super::dispatched("rotate", 1, &text, 9), a + 3 * b + 5 * c + 7 * d);
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
    fn the_shapes_a_propagation_reaches_through_and_the_ones_it_refuses_agree_on_the_total() {
        // Every one of these prints the same number whether or not a constant was substituted, so
        // a total that comes out wrong is a compiler that substituted where it should not have.
        let cases = cases_for(Facet::ConstantArgs);
        for shape in ["handed-on", "one-more-level", "recursive", "other-objects-call-it"] {
            let case = cases
                .iter()
                .find(|c| c.axes.get("shape") == Some(shape) && c.axes.get("type") == Some("i32"))
                .unwrap();
            assert_eq!(output(case), "60\n", "{shape}");
        }
        let escaping = cases
            .iter()
            .find(|c| {
                c.axes.get("shape") == Some("address-escapes") && c.axes.get("type") == Some("i32")
            })
            .unwrap();
        assert_eq!(output(escaping), "42\n");
    }

    #[test]
    fn the_varargs_shape_asks_for_the_header_that_makes_it_legal() {
        let cases = cases_for(Facet::ConstantArgs);
        let case = cases
            .iter()
            .find(|c| {
                c.axes.get("shape") == Some("through-varargs") && c.axes.get("type") == Some("i64")
            })
            .unwrap();
        assert!(case.source.contains("#include <stdarg.h>"));
        assert_eq!(output(case), "60\n");
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

    #[test]
    fn each_call_motion_pair_agrees_on_the_total_or_disagrees_on_the_count() {
        let cases = cases_for(Facet::CallMotion);
        let shape = |want: &str| {
            let case = cases
                .iter()
                .find(|c| c.axes.get("shape") == Some(want) && c.axes.get("type") == Some("i32"))
                .unwrap();
            output(case).to_owned()
        };
        // Thirteen a thousand times over when the loop leaves the table alone, and thirteen
        // climbing by one a time round when it does not. The second number of each pair is the
        // element the loop either wrote or did not.
        assert_eq!(shape("reads-a-table-nothing-writes"), "13000\n7\n250\n");
        assert_eq!(shape("reads-a-table-the-loop-writes"), "512500\n6\n1007\n");
        // Six squared a thousand times, against the squares of six upwards.
        assert_eq!(shape("touches-nothing"), "36000\n250\n");
        assert_eq!(shape("an-argument-the-loop-changes"), "338863500\n250\n");
        // The total is the same whether the call ran once or a thousand times. The count after
        // it is the only thing that can tell, which is what this shape is for.
        assert_eq!(shape("writes-through-an-argument"), "6000\n1000\n6\n");
        // The gate is zero, so the divide never runs and the total is one per trip.
        assert_eq!(shape("a-call-under-a-false-test"), "1000\n");
    }

    #[test]
    fn every_call_motion_callee_the_case_can_see_is_kept_out_of_line_and_says_so() {
        let cases = cases_for(Facet::CallMotion);
        assert_eq!(cases.len(), 36);
        let (together, apart): (Vec<_>, Vec<_>) =
            cases.iter().partition(|case| case.units.is_empty());
        assert_eq!(together.len(), 24);
        for case in together {
            assert!(case.source.contains("__attribute__((noinline))"), "{}", case.id);
            assert!(case.has_tag("gnu"), "{}", case.id);
        }
        // The other twelve do not need `noinline`, because a body in another translation unit
        // cannot be inlined into this one without a flag no run here passes. Only the shape that
        // writes an attribute on its declaration is a gnu case.
        assert_eq!(apart.len(), 12);
        for case in apart {
            assert_eq!(case.units.len(), 1, "{}", case.id);
            assert!(!case.source.contains("noinline"), "{}", case.id);
            let promises = case.axes.get("shape") == Some("another-unit-promises-const");
            assert_eq!(case.has_tag("gnu"), promises, "{}", case.id);
            assert_eq!(case.source.contains("__attribute__"), promises, "{}", case.id);
        }
    }

    #[test]
    fn the_shapes_across_units_print_the_same_answer_whatever_the_compiler_believes() {
        let cases = cases_for(Facet::CallMotion);
        let shape = |want: &str| {
            let case = cases
                .iter()
                .find(|c| c.axes.get("shape") == Some(want) && c.axes.get("type") == Some("i32"))
                .unwrap();
            output(case).to_owned()
        };
        // Twice six a thousand times, and then the count the helper kept of its own calls. That
        // second number is the only thing in the program that can say whether the call came out
        // of the loop, and a compiler that took it out with no summary to go on prints one.
        assert_eq!(shape("another-unit-writes-a-counter"), "12000\n1000\n");
        // Nineteen a thousand times either way. The pair is not about the number, which is why
        // both halves print the same one. It is about what the promise cost to leave out.
        assert_eq!(shape("another-unit-is-silent"), "19000\n250\n");
        assert_eq!(shape("another-unit-promises-const"), "19000\n250\n");
    }
}
