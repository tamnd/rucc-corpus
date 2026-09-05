//! The transformations that need the whole function.
//!
//! These are the cases where the answer depends on a path through the control flow graph
//! rather than on a window of straight line code. They are also where a compiler starts being
//! able to get things wrong in ways that only show up on one path in four, so almost every
//! case here computes its answer along several paths and prints all of them.

use super::lit;
use crate::Sink;
use crate::emit::Program;
use crate::lang::{Op, Ty, eval};
use corpus_model::{Axes, Dialect, Facet};

/// Emits every facet in this phase.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    common_subexpr(sink);
    load_forwarding(sink);
    code_motion(sink);
    copy_propagation(sink);
    constant_propagation(sink);
    value_range(sink);
    alias_analysis(sink);
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
    const SHAPES: &[&str] = &["same-slot", "other-slot-between", "across-branch", "through-pointer"];
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
            let mut program =
                Program::new(format!("a chain of {depth} {} copies", ty.c_name()));
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
                    program.line(format!("{name} table[3] = {{ {}, {}, {} }};", lit(ty, 21), lit(ty, 22), lit(ty, 23)));
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
        sink.push(
            Facet::ValueRange,
            Axes::of([("type", ty.name())]),
            Dialect::C17,
            program,
        );
    }
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
            let mut program =
                Program::new(format!("two {} references, {shape}", ty.c_name()));
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

/// Aggregates and address taken locals that never escape.
///
/// Each of these can be split into plain values, because nothing outside the function can see
/// the memory. The answer is what is checked, and the size of the frame is what the report
/// notices, since a compiler that failed to split still has to allocate the whole struct.
fn scalar_replacement(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["struct", "nested-struct", "array-constant-index", "address-taken", "union"];
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
        let aliasing = cases
            .iter()
            .find(|c| c.axes.get("shape") == Some("may-alias"))
            .unwrap();
        let distinct = cases
            .iter()
            .find(|c| c.axes.get("shape") == Some("distinct-locals"))
            .unwrap();
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
            let Expect::Output(text) = &case.expect else {
                panic!("{} should run", case.id)
            };
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
}
