//! The inline assembly the Linux kernel writes, as facets of their own.
//!
//! The kernel leans on three things about `asm` that ordinary programs rarely touch, and each is
//! a facet here.
//!
//! `asm-goto` is `asm goto`, the statement that jumps to a C label. Every static key in the
//! kernel is one: a five byte nop that the kernel patches into a jump at run time, with the
//! unlikely code behind a label the compiler must keep. The user copy routines use the form with
//! outputs, which only reads the outputs on the path that falls through.
//!
//! `asm-local-labels` is the labels inside a template. A numeric label like `1:` can be defined
//! again and again and is found with `1b` or `1f`, and `%=` is a number that is different for
//! every copy of the statement the compiler emits. Both matter once a function with an `asm` in
//! it is inlined into the same caller more than once, which is how every accessor in the kernel
//! is used. The exception table is the other user: an entry goes into another section with
//! `.pushsection` and names a label back in the code.
//!
//! `gas-macros` is the assembler's own macro language used from C. The kernel defines macros in
//! top level `asm` and calls them from inline templates, and writes `.rept` and `.irp` loops
//! inside templates. A compiler that assembles a template on its own, without the rest of the
//! file, gets these wrong.
//!
//! Every case is x86-64 only and in the AT&T syntax gcc and the kernel use. Each one runs a small
//! step function over a counter and prints where it ends up, with the answer worked out here the
//! same way.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// The tag that says where a case came from.
const PROVENANCE: &str = "provenance:linux";

/// The seed every case reads through a `volatile` global.
const SEED: u32 = 7;

/// How many times each case runs its step.
const ROUNDS: u32 = 64;

/// Emits every facet in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &(facet, shapes) in FACETS {
        for &shape in shapes {
            if !sink.wants(facet) {
                break;
            }
            let (program, elf) = case(facet, shape);
            let axes = Axes::of([("shape", shape)]);
            let tags: &[&str] = if elf {
                &["gnu", "x86-64", "elf", PROVENANCE]
            } else {
                &["gnu", "x86-64", PROVENANCE]
            };
            sink.push_tagged(facet, axes, Dialect::C17, program, tags);
        }
    }
}

/// Each facet and the shapes it has.
const FACETS: &[(Facet, &[&str])] = &[
    (
        Facet::AsmGoto,
        &["branch", "static-key-off", "static-key-on", "two-labels", "output", "in-loop"],
    ),
    (
        Facet::AsmLocalLabels,
        &["backward", "forward", "inlined-copies", "unique-number", "pushsection"],
    ),
    (Facet::GasMacros, &["macro-args", "default-arg", "rept", "irp", "conditional", "set"]),
];

/// What a case is: the file it has the shape of, the C above `main`, and the step it runs as
/// Rust. The step is `step(x, i)` in the C.
struct Shape {
    after: &'static str,
    top: &'static [&'static str],
    step: fn(u32, u32) -> u32,
    elf: bool,
}

/// One case, and whether it needs an ELF assembler.
fn case(facet: Facet, shape: &str) -> (Program, bool) {
    let found = match (facet, shape) {
        (Facet::AsmGoto, _) => asm_goto(shape),
        (Facet::AsmLocalLabels, _) => local_labels(shape),
        _ => gas_macros(shape),
    };
    let mut program = Program::new(format!("{shape} after {} in Linux 7.2", found.after));
    program.top(format!("static volatile unsigned int seed_in = {SEED};"));
    program.top("");
    for line in found.top {
        program.top(*line);
    }
    program.line("unsigned int acc = seed_in;");
    program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
    program.line_at(1, "acc = step(acc, i);");
    program.line("}");
    program.blank();
    let mut acc = SEED;
    for i in 0..ROUNDS {
        acc = (found.step)(acc, i);
    }
    program.check(Ty::U32, "acc", i128::from(acc));
    (program, found.elf)
}

fn asm_goto(shape: &str) -> Shape {
    match shape {
        "branch" => Shape {
            after: "arch/x86/include/asm/rmwcc.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm goto(\"testl $1, %0\\n\\tjnz %l[odd]\" : : \"r\"(x) : \"cc\" : odd);",
                "    return x / 2u + i;",
                "odd:",
                "    return x * 3u + 1u + i;",
                "}",
            ],
            step: |x, i| {
                if x & 1 == 1 {
                    x.wrapping_mul(3).wrapping_add(1).wrapping_add(i)
                } else {
                    x / 2 + i
                }
            },
            elf: false,
        },
        "static-key-off" => Shape {
            after: "arch/x86/include/asm/jump_label.h",
            top: &[
                "static inline __attribute__((always_inline)) int key_enabled(void) {",
                "    asm goto(\"1: .byte 0x0f, 0x1f, 0x44, 0x00, 0x00\" : : : : yes);",
                "    return 0;",
                "yes:",
                "    return 1;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    if (key_enabled()) {",
                "        return (x ^ i) * 5u + 1u;",
                "    }",
                "    return x + i * 7u;",
                "}",
            ],
            step: |x, i| x.wrapping_add(i.wrapping_mul(7)),
            elf: false,
        },
        "static-key-on" => Shape {
            after: "arch/x86/include/asm/jump_label.h",
            top: &[
                "static inline __attribute__((always_inline)) int key_enabled(void) {",
                "    asm goto(\"jmp %l[yes]\" : : : : yes);",
                "    return 0;",
                "yes:",
                "    return 1;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    if (key_enabled()) {",
                "        return (x ^ i) * 5u + 1u;",
                "    }",
                "    return x + i * 7u;",
                "}",
            ],
            step: |x, i| (x ^ i).wrapping_mul(5).wrapping_add(1),
            elf: false,
        },
        "two-labels" => Shape {
            after: "arch/x86/include/asm/uaccess.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm goto(\"cmpl $3, %0\\n\\tjb %l[low]\\n\\tje %l[mid]\"",
                "             : : \"r\"(x & 7u) : \"cc\" : low, mid);",
                "    return x - i;",
                "low:",
                "    return x + 13u * i;",
                "mid:",
                "    return x * 9u;",
                "}",
            ],
            step: |x, i| match (x & 7).cmp(&3) {
                std::cmp::Ordering::Less => x.wrapping_add(i.wrapping_mul(13)),
                std::cmp::Ordering::Equal => x.wrapping_mul(9),
                std::cmp::Ordering::Greater => x.wrapping_sub(i),
            },
            elf: false,
        },
        "output" => Shape {
            after: "arch/x86/include/asm/uaccess.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int low;",
                "    asm goto(\"movl %1, %0\\n\\tandl $3, %0\\n\\tjz %l[zero]\"",
                "             : \"=r\"(low) : \"r\"(x) : \"cc\" : zero);",
                "    return x + low * i;",
                "zero:",
                "    return x + 17u;",
                "}",
            ],
            step: |x, i| {
                if x & 3 == 0 {
                    x.wrapping_add(17)
                } else {
                    x.wrapping_add((x & 3).wrapping_mul(i))
                }
            },
            elf: false,
        },
        _ => Shape {
            after: "include/linux/bitops.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int bits(unsigned int x) {",
                "    unsigned int n = 0;",
                "    for (unsigned int k = 0; k < 16u; k++) {",
                "        asm goto(\"btl %1, %0\\n\\tjc %l[set]\" : : \"r\"(x), \"r\"(k) : \"cc\" : set);",
                "        continue;",
                "    set:",
                "        n += k;",
                "    }",
                "    return n;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return x * 31u + bits(x ^ i);",
                "}",
            ],
            step: |x, i| {
                let y = x ^ i;
                let bits: u32 = (0..16).filter(|k| y >> k & 1 == 1).sum();
                x.wrapping_mul(31).wrapping_add(bits)
            },
            elf: false,
        },
    }
}

fn local_labels(shape: &str) -> Shape {
    match shape {
        "backward" => Shape {
            after: "arch/x86/lib/delay.c",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int n = (i & 7u) + 1u;",
                "    asm(\"1:\\n\\taddl %2, %0\\n\\tdecl %1\\n\\tjnz 1b\"",
                "        : \"+r\"(x), \"+r\"(n) : \"r\"(i | 1u) : \"cc\");",
                "    return x;",
                "}",
            ],
            step: |x, i| x.wrapping_add(((i & 7) + 1).wrapping_mul(i | 1)),
            elf: false,
        },
        "forward" => Shape {
            after: "arch/x86/include/asm/bitops.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\"testl %1, %1\\n\\tjz 1f\\n\\txorl %1, %0\\n1:\"",
                "        : \"+r\"(x) : \"r\"(i & 3u) : \"cc\");",
                "    return x * 3u + i;",
                "}",
            ],
            step: |x, i| (x ^ (i & 3)).wrapping_mul(3).wrapping_add(i),
            elf: false,
        },
        "inlined-copies" => Shape {
            after: "include/linux/minmax.h",
            top: &[
                "static inline __attribute__((always_inline))",
                "unsigned int at_least(unsigned int x, unsigned int floor) {",
                "    asm(\"cmpl %1, %0\\n\\tjae 1f\\n\\tmovl %1, %0\\n1:\"",
                "        : \"+r\"(x) : \"r\"(floor) : \"cc\");",
                "    return x;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int a = at_least(x & 1023u, i * 17u);",
                "    unsigned int b = at_least(i, x >> 24);",
                "    unsigned int c = at_least(x >> 3, a ^ b);",
                "    return a + b * 5u + c;",
                "}",
            ],
            step: |x, i| {
                let a = (x & 1023).max(i.wrapping_mul(17));
                let b = i.max(x >> 24);
                let c = (x >> 3).max(a ^ b);
                a.wrapping_add(b.wrapping_mul(5)).wrapping_add(c)
            },
            elf: false,
        },
        "unique-number" => Shape {
            after: "arch/x86/include/asm/alternative.h",
            top: &[
                "static inline __attribute__((always_inline))",
                "unsigned int add_if_odd(unsigned int x, unsigned int k) {",
                "    asm(\"testl $1, %1\\n\\tjz .Lrk_skip%=\\n\\taddl %1, %0\\n.Lrk_skip%=:\"",
                "        : \"+r\"(x) : \"r\"(k) : \"cc\");",
                "    return x;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    x = add_if_odd(x, i);",
                "    x = add_if_odd(x, i + 1u);",
                "    x = add_if_odd(x, x >> 5);",
                "    return add_if_odd(x * 3u, i * 7u);",
                "}",
            ],
            step: |x, i| {
                let add = |x: u32, k: u32| if k & 1 == 1 { x.wrapping_add(k) } else { x };
                let x = add(x, i);
                let x = add(x, i + 1);
                let x = add(x, x >> 5);
                add(x.wrapping_mul(3), i.wrapping_mul(7))
            },
            elf: false,
        },
        _ => Shape {
            after: "arch/x86/include/asm/asm.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int k;",
                "    asm(\".pushsection .rodata\\n\"",
                "        \"2:\\t.long 0x9e3779b9\\n\"",
                "        \".popsection\\n\\t\"",
                "        \"movl 2b(%%rip), %0\" : \"=r\"(k));",
                "    return (x ^ k) + i;",
                "}",
            ],
            step: |x, i| (x ^ 0x9e37_79b9).wrapping_add(i),
            elf: true,
        },
    }
}

fn gas_macros(shape: &str) -> Shape {
    match shape {
        "macro-args" => Shape {
            after: "arch/x86/include/asm/asm.h",
            top: &[
                "__asm__(\".macro rk_mix dst, src\\n\"",
                "        \"\\taddl \\\\src, \\\\dst\\n\"",
                "        \"\\troll $5, \\\\dst\\n\"",
                "        \".endm\\n\");",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\"rk_mix %0, %1\" : \"+r\"(x) : \"r\"(i) : \"cc\");",
                "    return x;",
                "}",
            ],
            step: |x, i| x.wrapping_add(i).rotate_left(5),
            elf: false,
        },
        "default-arg" => Shape {
            after: "arch/x86/include/asm/asm.h",
            top: &[
                "__asm__(\".macro rk_scale reg, by=3\\n\"",
                "        \"\\timull $\\\\by, \\\\reg, \\\\reg\\n\"",
                "        \".endm\\n\");",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\"rk_scale %0\\n\\trk_scale %0, 7\" : \"+r\"(x));",
                "    return x + i;",
                "}",
            ],
            step: |x, i| x.wrapping_mul(21).wrapping_add(i),
            elf: false,
        },
        "rept" => Shape {
            after: "arch/x86/include/asm/nops.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\".rept 3\\n\\taddl %1, %0\\n\\t.endr\" : \"+r\"(x) : \"r\"(i | 1u) : \"cc\");",
                "    return x ^ (x >> 7);",
                "}",
            ],
            step: |x, i| {
                let x = x.wrapping_add((i | 1).wrapping_mul(3));
                x ^ (x >> 7)
            },
            elf: false,
        },
        "irp" => Shape {
            after: "arch/x86/include/asm/cpufeature.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\".irp v, 3, 5, 7\\n\\timull $\\\\v, %0, %0\\n\\t.endr\" : \"+r\"(x));",
                "    return x + i;",
                "}",
            ],
            step: |x, i| x.wrapping_mul(105).wrapping_add(i),
            elf: false,
        },
        "conditional" => Shape {
            after: "arch/x86/include/asm/asm.h",
            top: &[
                "__asm__(\".macro rk_bump reg, by\\n\"",
                "        \".if \\\\by == 1\\n\"",
                "        \"\\tincl \\\\reg\\n\"",
                "        \".else\\n\"",
                "        \"\\taddl $\\\\by, \\\\reg\\n\"",
                "        \".endif\\n\"",
                "        \".endm\\n\");",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\"rk_bump %0, 1\\n\\trk_bump %0, 9\" : \"+r\"(x) : : \"cc\");",
                "    return x * 5u + i;",
                "}",
            ],
            step: |x, i| x.wrapping_add(10).wrapping_mul(5).wrapping_add(i),
            elf: false,
        },
        _ => Shape {
            after: "arch/x86/include/asm/page_types.h",
            top: &[
                "__asm__(\".set rk_shift, 11\");",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    asm(\"roll $rk_shift, %0\" : \"+r\"(x) : : \"cc\");",
                "    return x + i;",
                "}",
            ],
            step: |x, i| x.rotate_left(11).wrapping_add(i),
            elf: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_shape_of_every_asm_facet_is_there_and_is_x86_64_only() {
        let manifest = generate(&Options::all()).unwrap();
        for (facet, count) in
            [(Facet::AsmGoto, 6), (Facet::AsmLocalLabels, 5), (Facet::GasMacros, 6)]
        {
            let cases: Vec<_> = manifest.cases.iter().filter(|case| case.facet == facet).collect();
            assert_eq!(cases.len(), count, "{facet:?}");
            for case in cases {
                assert!(case.has_tag("x86-64"), "{}", case.id);
                assert!(case.has_tag("provenance:linux"), "{}", case.id);
                assert!(case.source.contains("asm"), "{}", case.id);
            }
        }
    }

    #[test]
    fn only_the_case_that_writes_another_section_needs_elf() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest.cases.iter().filter(|case| case.facet == Facet::AsmLocalLabels) {
            assert_eq!(case.has_tag("elf"), case.source.contains(".pushsection"), "{}", case.id);
        }
    }
}
