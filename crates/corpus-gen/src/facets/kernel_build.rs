//! The facets that are the way the Linux kernel is built rather than what it writes.
//!
//! `constant-p-after-inline` is the promise the kernel takes from `__builtin_constant_p`: asked
//! inside an inline function about one of its parameters, it answers for the argument at the call
//! site once the function has been inlined. `test_bit`, `hweight32` and every `BUILD_BUG_ON` lean
//! on it. The cases pin `-O2`, because the promise only holds once the optimizer has run, and the
//! kernel is never built without it. The branch the answer rules out calls a function that is
//! defined nowhere, so a compiler that answers wrongly fails to link rather than printing the
//! same number either way.
//!
//! `mcmodel-kernel` is ordinary programs built with `-mcmodel=kernel`, which says every symbol is
//! in the top two gigabytes of the address space, sign extended from 32 bits. A program linked
//! without PIE sits in the bottom two gigabytes, and a 32 bit address sign extends to the same
//! place there, so the same code runs in user space. Each case reaches its data a different way:
//! an index into a table, a table of addresses, a table of functions, a jump table, strings and an
//! array far bigger than any displacement.
//!
//! `general-regs-only` is programs built with `-mgeneral-regs-only`, which the kernel uses so that
//! it never touches the floating point and vector state it has not saved. A program cannot see
//! which registers its own code uses, so each case puts a marker in every vector register first,
//! does work a compiler would like to vectorize, and checks the markers are all still there
//! before it prints. A compiler that ignores the flag prints a different answer.
//!
//! All three are x86-64 only, and each case names the kernel file it has the shape of.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet};

/// The tag that says where a case came from.
const PROVENANCE: &str = "provenance:linux";

/// The seed every case reads through a `volatile` global.
const SEED: u32 = 7;

/// How many times each case runs its step.
const ROUNDS: u32 = 64;

/// What `general-regs-only` puts in every vector register.
const MARK: u64 = 0x5a5a_a5a5_c3c3_3c3c;

/// The tags every case carries, and `elf` last for the ones that need a Linux loader too. A
/// Mach-O program is always position independent and loaded above four gigabytes, where no
/// address fits in 32 bits.
const ELF: &[&str] = &["gnu", "x86-64", PROVENANCE, "elf"];

/// Emits every facet in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &(facet, flags, shapes) in FACETS {
        for &shape in shapes {
            if !sink.wants(facet) {
                break;
            }
            let (source, expected) = case(facet, shape).finish();
            let axes = Axes::of([("shape", shape)]);
            let case = Case::linked(
                facet,
                axes,
                Dialect::C17,
                source,
                Vec::new(),
                flags.iter().map(|flag| (*flag).to_owned()).collect(),
                Expect::Output(expected),
            )
            .tagged(if facet == Facet::McmodelKernel { ELF } else { &ELF[..3] });
            sink.push_case(case);
        }
    }
}

/// Each facet, the flags every case of it is built with, and its shapes.
const FACETS: &[(Facet, &[&str], &[&str])] = &[
    (
        Facet::ConstantPAfterInline,
        &["-O2"],
        &["argument", "two-levels", "expression", "local", "runtime-argument"],
    ),
    (
        Facet::McmodelKernel,
        &["-mcmodel=kernel", "-mno-red-zone", "-fno-pie", "-no-pie"],
        &["global-index", "address-table", "function-table", "switch", "strings", "large-array"],
    ),
    (
        Facet::GeneralRegsOnly,
        &["-mgeneral-regs-only"],
        &["struct-copy", "array-sum", "byte-mix", "zeroed", "u64-math"],
    ),
];

/// What a case is: the file it has the shape of, the C above `main`, and the step it runs as
/// Rust, which may keep state between rounds the way the C keeps it in globals. The step is
/// `step(x, i)` in the C.
struct Shape {
    after: &'static str,
    top: Vec<String>,
    step: Box<dyn FnMut(u32, u32) -> u32>,
}

impl Shape {
    fn new(after: &'static str, top: &[&str], step: impl FnMut(u32, u32) -> u32 + 'static) -> Self {
        Self {
            after,
            top: top.iter().map(|line| (*line).to_owned()).collect(),
            step: Box::new(step),
        }
    }
}

/// One case.
fn case(facet: Facet, shape: &str) -> Program {
    let mut found = match facet {
        Facet::ConstantPAfterInline => constant_p(shape),
        Facet::McmodelKernel => mcmodel_kernel(shape),
        _ => general_regs_only(shape),
    };
    let marked = facet == Facet::GeneralRegsOnly;
    let mut program = Program::new(format!("{shape} after {} in Linux 7.2", found.after));
    program.top(format!("static volatile unsigned int seed_in = {SEED};"));
    program.top("");
    if marked {
        marking(&mut program);
    }
    for line in &found.top {
        program.top(line.clone());
    }
    if marked {
        program.line(format!("mark_vectors({MARK:#x}ull);"));
    }
    program.line("unsigned int acc = seed_in;");
    program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
    program.line_at(1, "acc = step(acc, i);");
    program.line("}");
    if marked {
        program.line(format!("if (vectors_changed({MARK:#x}ull)) {{"));
        program.line_at(1, "acc = 0u;");
        program.line("}");
    }
    program.blank();
    let mut acc = SEED;
    for i in 0..ROUNDS {
        acc = (found.step)(acc, i);
    }
    program.check(Ty::U32, "acc", i128::from(acc));
    program
}

/// The two functions a `general-regs-only` case puts its marker in the vector registers with and
/// checks it with. Both are only text to the compiler, so they touch no vector register the
/// program did not name.
fn marking(program: &mut Program) {
    program.top("__attribute__((noinline))");
    program.top("static void mark_vectors(unsigned long long mark) {");
    program.top("    asm volatile(");
    for register in 0..16 {
        let end = if register == 15 { "" } else { "\\n\\t" };
        program.top(format!("        \"movq %0, %%xmm{register}{end}\""));
    }
    program.top("        : : \"r\"(mark));");
    program.top("}");
    program.top("");
    program.top("__attribute__((noinline))");
    program.top("static unsigned long long vectors_changed(unsigned long long mark) {");
    program.top("    unsigned long long seen, changed = 0;");
    for register in 0..16 {
        program.top(format!("    asm volatile(\"movq %%xmm{register}, %0\" : \"=r\"(seen));"));
        program.top("    changed |= seen ^ mark;");
    }
    program.top("    return changed;");
    program.top("}");
    program.top("");
}

/// The two functions defined nowhere, one for each answer a case rules out.
const UNDEFINED: &[&str] = &[
    "extern void rk_not_constant(void);",
    "extern void rk_constant(void);",
    "",
    "static inline __attribute__((always_inline)) unsigned int scaled(unsigned int n) {",
    "    if (!__builtin_constant_p(n)) {",
    "        rk_not_constant();",
    "    }",
    "    return n * 3u;",
    "}",
    "",
];

fn constant_p(shape: &str) -> Shape {
    let with = |rest: &[&'static str]| -> Vec<&'static str> {
        UNDEFINED.iter().chain(rest).copied().collect()
    };
    match shape {
        "argument" => Shape::new(
            "arch/x86/include/asm/bitops.h",
            &with(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return x + scaled(5u) + scaled(9u) * i;",
                "}",
            ]),
            |x, i| x.wrapping_add(15).wrapping_add(i.wrapping_mul(27)),
        ),
        "two-levels" => Shape::new(
            "include/asm-generic/bitops/const_hweight.h",
            &with(&[
                "static inline __attribute__((always_inline)) unsigned int outer(unsigned int n) {",
                "    return scaled(n + 1u);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return (x ^ outer(40u)) + i;",
                "}",
            ]),
            |x, i| (x ^ 123).wrapping_add(i),
        ),
        "expression" => Shape::new(
            "include/linux/build_bug.h",
            &with(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return x * 7u + scaled((3u << 2) + 5u * 1u) + i;",
                "}",
            ]),
            |x, i| x.wrapping_mul(7).wrapping_add(51).wrapping_add(i),
        ),
        "local" => Shape::new(
            "include/linux/log2.h",
            &with(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int k = 12u;",
                "    k += 4u;",
                "    return x + scaled(k) - i;",
                "}",
            ]),
            |x, i| x.wrapping_add(48).wrapping_sub(i),
        ),
        _ => Shape::new(
            "include/linux/uaccess.h",
            &with(&[
                "static inline __attribute__((always_inline)) unsigned int picked(unsigned int n) {",
                "    if (__builtin_constant_p(n)) {",
                "        rk_constant();",
                "    }",
                "    return n * 3u;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return x + picked(x ^ i);",
                "}",
            ]),
            |x, i| x.wrapping_add((x ^ i).wrapping_mul(3)),
        ),
    }
}

/// The value each entry of the `global-index` table holds.
fn entry(k: u32) -> u32 {
    k.wrapping_mul(k).wrapping_mul(2_654_435_761).wrapping_add(k)
}

fn mcmodel_kernel(shape: &str) -> Shape {
    match shape {
        "global-index" => {
            let mut top = vec!["static const unsigned int table[16] = {".to_owned()];
            for row in 0..4 {
                let values: Vec<String> =
                    (0..4).map(|k| format!("{:#010x}u", entry(row * 4 + k))).collect();
                top.push(format!("    {},", values.join(", ")));
            }
            top.push("};".to_owned());
            top.push(String::new());
            for line in [
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return x + table[(x ^ i) & 15u];",
                "}",
            ] {
                top.push(line.to_owned());
            }
            Shape {
                after: "arch/x86/kernel/cpu/common.c",
                top,
                step: Box::new(|x, i| x.wrapping_add(entry((x ^ i) & 15))),
            }
        }
        "address-table" => {
            let mut slots = [0u32; 4];
            Shape::new(
                "kernel/sysctl.c",
                &[
                    "static unsigned int a, b, c, d;",
                    "static unsigned int *const slots[4] = { &a, &b, &c, &d };",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    *slots[i & 3u] += x;",
                    "    return x * 3u + *slots[(x + i) & 3u];",
                    "}",
                ],
                move |x, i| {
                    let at = (i & 3) as usize;
                    slots[at] = slots[at].wrapping_add(x);
                    x.wrapping_mul(3).wrapping_add(slots[(x.wrapping_add(i) & 3) as usize])
                },
            )
        }
        "function-table" => Shape::new(
            "kernel/sys_ni.c",
            &[
                "static unsigned int op0(unsigned int x, unsigned int i) { return x + i * 3u + 1u; }",
                "static unsigned int op1(unsigned int x, unsigned int i) { return x ^ (i * 0x9e3779b9u); }",
                "static unsigned int op2(unsigned int x, unsigned int i) { return x * 33u + i; }",
                "static unsigned int op3(unsigned int x, unsigned int i) { return x - i; }",
                "",
                "static unsigned int (*const calls[4])(unsigned int, unsigned int) = {",
                "    op0, op1, op2, op3,",
                "};",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return calls[(x + i) & 3u](x, i);",
                "}",
            ],
            |x, i| match x.wrapping_add(i) & 3 {
                0 => x.wrapping_add(i * 3).wrapping_add(1),
                1 => x ^ i.wrapping_mul(0x9e37_79b9),
                2 => x.wrapping_mul(33).wrapping_add(i),
                _ => x.wrapping_sub(i),
            },
        ),
        "switch" => Shape::new(
            "arch/x86/kernel/traps.c",
            &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    switch ((x + i) & 7u) {",
                "    case 0: return x + 11u;",
                "    case 1: return x ^ 0x5a5au;",
                "    case 2: return x * 5u;",
                "    case 3: return x >> 1;",
                "    case 4: return x + (x >> 3);",
                "    case 5: return ~x;",
                "    case 6: return x - 7u;",
                "    default: return (x << 3) | (x >> 29);",
                "    }",
                "}",
            ],
            |x, i| match x.wrapping_add(i) & 7 {
                0 => x.wrapping_add(11),
                1 => x ^ 0x5a5a,
                2 => x.wrapping_mul(5),
                3 => x >> 1,
                4 => x.wrapping_add(x >> 3),
                5 => !x,
                6 => x.wrapping_sub(7),
                _ => x.rotate_left(3),
            },
        ),
        "strings" => {
            const NAMES: [&str; 4] = ["alpha", "beta", "gamma", "delta"];
            Shape::new(
                "kernel/printk/printk.c",
                &[
                    "static const char *const names[4] = { \"alpha\", \"beta\", \"gamma\", \"delta\" };",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    const char *name = names[(x ^ i) & 3u];",
                    "    unsigned int h = x;",
                    "    while (*name) {",
                    "        h = h * 31u + (unsigned char)*name++;",
                    "    }",
                    "    return h + i;",
                    "}",
                ],
                |x, i| {
                    let name = NAMES[((x ^ i) & 3) as usize];
                    let h = name
                        .bytes()
                        .fold(x, |h, byte| h.wrapping_mul(31).wrapping_add(u32::from(byte)));
                    h.wrapping_add(i)
                },
            )
        }
        _ => {
            let mut big = vec![0u8; 1 << 20];
            Shape::new(
                "mm/page_alloc.c",
                &[
                    "static unsigned char big[1u << 20];",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    big[(x * 4099u) & 0xfffffu] ^= (unsigned char)(i + 1u);",
                    "    return x * 5u + big[(x * 4099u) & 0xfffffu] + big[0xffff0u + (i & 15u)];",
                    "}",
                ],
                move |x, i| {
                    let at = (x.wrapping_mul(4099) & 0xf_ffff) as usize;
                    big[at] ^= (i.wrapping_add(1) & 0xff) as u8;
                    let far = big[0xf_fff0 + (i & 15) as usize];
                    x.wrapping_mul(5).wrapping_add(u32::from(big[at])).wrapping_add(u32::from(far))
                },
            )
        }
    }
}

fn general_regs_only(shape: &str) -> Shape {
    match shape {
        "struct-copy" => Shape::new(
            "include/linux/sched.h",
            &[
                "struct blob { unsigned int w[12]; };",
                "static struct blob kept;",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    struct blob local;",
                "    for (unsigned int k = 0; k < 12u; k++) {",
                "        local.w[k] = x + k * i;",
                "    }",
                "    kept = local;",
                "    return (kept.w[i % 12u] ^ (kept.w[(x ^ i) % 12u] << 3)) + i;",
                "}",
            ],
            |x, i| {
                let w = |k: u32| x.wrapping_add(k.wrapping_mul(i));
                (w(i % 12) ^ (w((x ^ i) % 12) << 3)).wrapping_add(i)
            },
        ),
        "array-sum" => {
            let mut data = [0u32; 64];
            Shape::new(
                "lib/checksum.c",
                &[
                    "static unsigned int data[64];",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    data[i & 63u] = x;",
                    "    unsigned int sum = 0;",
                    "    for (unsigned int k = 0; k < 64u; k++) {",
                    "        sum += data[k] * (k + 1u);",
                    "    }",
                    "    return sum ^ i;",
                    "}",
                ],
                move |x, i| {
                    data[(i & 63) as usize] = x;
                    let sum = data
                        .iter()
                        .zip(1u32..)
                        .fold(0u32, |sum, (&value, k)| sum.wrapping_add(value.wrapping_mul(k)));
                    sum ^ i
                },
            )
        }
        "byte-mix" => Shape::new(
            "lib/string.c",
            &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned char buf[64];",
                "    for (unsigned int k = 0; k < 64u; k++) {",
                "        buf[k] = (unsigned char)((x >> (k & 24u)) ^ k ^ i);",
                "    }",
                "    unsigned int h = 0;",
                "    for (unsigned int k = 0; k < 64u; k++) {",
                "        h += buf[k] * (k | 1u);",
                "    }",
                "    return x + h;",
                "}",
            ],
            |x, i| {
                let h = (0u32..64).fold(0u32, |h, k| {
                    let byte = ((x >> (k & 24)) ^ k ^ i) & 0xff;
                    h.wrapping_add(byte.wrapping_mul(k | 1))
                });
                x.wrapping_add(h)
            },
        ),
        "zeroed" => Shape::new(
            "include/linux/string.h",
            &[
                "struct state { unsigned int w[16]; };",
                "",
                "__attribute__((noinline))",
                "static unsigned int fold(const struct state *s) {",
                "    unsigned int h = 0;",
                "    for (unsigned int k = 0; k < 16u; k++) {",
                "        h = h * 17u + s->w[k];",
                "    }",
                "    return h;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    struct state s = { { 0 } };",
                "    s.w[i & 15u] = x;",
                "    s.w[(x >> 4) & 15u] += i;",
                "    return fold(&s) + x;",
                "}",
            ],
            |x, i| {
                let mut w = [0u32; 16];
                w[(i & 15) as usize] = x;
                let at = ((x >> 4) & 15) as usize;
                w[at] = w[at].wrapping_add(i);
                let h = w.iter().fold(0u32, |h, &v| h.wrapping_mul(17).wrapping_add(v));
                h.wrapping_add(x)
            },
        ),
        _ => Shape::new(
            "include/linux/hash.h",
            &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned long long wide = (unsigned long long)x * 0x9e3779b97f4a7c15ull;",
                "    return (unsigned int)(wide >> 32) ^ (unsigned int)(wide % 1000003ull) ^ i;",
                "}",
            ],
            |x, i| {
                let wide = u64::from(x).wrapping_mul(0x9e37_79b9_7f4a_7c15);
                ((wide >> 32) as u32) ^ ((wide % 1_000_003) as u32) ^ i
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_shape_is_there_with_its_flags() {
        let manifest = generate(&Options::all()).unwrap();
        for (facet, count, flag) in [
            (Facet::ConstantPAfterInline, 5, "-O2"),
            (Facet::McmodelKernel, 6, "-mcmodel=kernel"),
            (Facet::GeneralRegsOnly, 5, "-mgeneral-regs-only"),
        ] {
            let cases: Vec<_> = manifest.cases.iter().filter(|case| case.facet == facet).collect();
            assert_eq!(cases.len(), count, "{facet:?}");
            for case in cases {
                assert!(case.flags.iter().any(|given| given == flag), "{}", case.id);
                assert!(case.has_tag("x86-64"), "{}", case.id);
                assert_eq!(case.has_tag("elf"), facet == Facet::McmodelKernel, "{}", case.id);
            }
        }
    }

    #[test]
    fn every_vector_register_is_marked_and_checked() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest.cases.iter().filter(|case| case.facet == Facet::GeneralRegsOnly) {
            for register in 0..16 {
                assert!(case.source.contains(&format!("%0, %%xmm{register}")), "{}", case.id);
                assert!(case.source.contains(&format!("%%xmm{register}, %0")), "{}", case.id);
            }
        }
    }
}
