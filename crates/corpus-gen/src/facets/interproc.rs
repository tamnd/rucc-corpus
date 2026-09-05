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
fn function_purity(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["inferred-const", "inferred-pure", "declared-const", "declared-pure", "impure"];
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
            let reads_memory = matches!(shape, "inferred-pure" | "declared-pure");
            if reads_memory {
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
}
