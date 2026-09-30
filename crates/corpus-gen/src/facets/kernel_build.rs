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
//! `objtool-shapes` is the code objtool has to follow through an object: a jump table, a call that
//! does not return, a realigned stack, a function that ends in `__builtin_unreachable` and a
//! sibling call. Each is a shape objtool refused in a rucc object on the way to K3. The program
//! only proves the code runs, and objtool from the pinned kernel is what checks the object.
//!
//! The kernel's `frame-size` cases are frames that GCC keeps under `FRAME_WARN`, 2048 bytes on
//! x86-64, only because it puts buffers whose lifetimes never overlap in the same bytes. Each case
//! has four buffers of 640 bytes, in scopes of their own, in callees inlined one after another, in
//! the arms of a switch, in an inline function inside another, and in an inline function called
//! from a loop. A compiler that gives each buffer its own slot has a frame of 2560 bytes.
//!
//! All but `frame-size` are x86-64 only, and each case names the kernel file it has the shape of.

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

/// The tags of a case that is plain C and runs anywhere.
const PORTABLE: &[&str] = &["gnu", PROVENANCE];

/// How many `unsigned int` each `frame-size` buffer holds, 640 bytes, so four of them go over
/// `FRAME_WARN` and one does not.
const WORDS: u32 = 160;

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
            .tagged(match facet {
                Facet::McmodelKernel => ELF,
                Facet::FrameSize => PORTABLE,
                _ => &ELF[..3],
            });
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
    (
        Facet::ObjtoolShapes,
        &["-O2"],
        &["jump-table", "noreturn", "stack-realign", "unreachable-end", "sibling-call"],
    ),
    (
        Facet::FrameSize,
        &["-O2"],
        &["disjoint-scopes", "inlined-callees", "switch-arms", "nested-inline", "loop-inline"],
    ),
];

/// What a case is: the file it has the shape of, the C above `main`, and the step it runs as
/// Rust, which may keep state between rounds the way the C keeps it in globals. The step is
/// `step(x, i)` in the C.
struct Shape {
    after: &'static str,
    top: Vec<String>,
    step: Step,
}

/// The step of a case, as Rust.
type Step = Box<dyn FnMut(u32, u32) -> u32>;

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
        Facet::ObjtoolShapes => objtool_shapes(shape),
        Facet::FrameSize => frame_size(shape),
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

/// The function every buffer is handed to, so that it has to be in memory, and what it returns
/// for a buffer that was filled with `x + k * c`.
const CONSUME: &[&str] = &[
    "__attribute__((noinline))",
    "static unsigned int consume(unsigned int *buf, unsigned int n, unsigned int x) {",
    "    unsigned int s = x;",
    "    for (unsigned int k = 0; k < n; k++) {",
    "        buf[k] = buf[k] * 2654435761u + s;",
    "        s = (s ^ buf[k]) * 16777619u;",
    "    }",
    "    return s;",
    "}",
    "",
];

fn consumed(x: u32, c: u32, n: u32) -> u32 {
    let mut s = x;
    for k in 0..n {
        let word = x.wrapping_add(k.wrapping_mul(c)).wrapping_mul(2_654_435_761).wrapping_add(s);
        s = (s ^ word).wrapping_mul(16_777_619);
    }
    s
}

/// The lines that declare a buffer, fill it from `x` and hand it to `consume`, at an indent.
fn buffer(lines: &mut Vec<String>, depth: usize, name: &str, step: &str, end: &str) {
    let pad = "    ".repeat(depth);
    lines.push(format!("{pad}unsigned int {name}[{WORDS}];"));
    lines.push(format!("{pad}for (unsigned int k = 0; k < {WORDS}u; k++) {{"));
    lines.push(format!("{pad}    {name}[k] = x + k * {step};"));
    lines.push(format!("{pad}}}"));
    lines.push(format!("{pad}{end} consume({name}, {WORDS}u, x);"));
}

fn with_consume(body: Vec<String>) -> Vec<String> {
    let mut top: Vec<String> = CONSUME.iter().map(|line| (*line).to_owned()).collect();
    top.extend(body);
    top
}

fn lines(text: &[&str]) -> Vec<String> {
    text.iter().map(|line| (*line).to_owned()).collect()
}

fn frame_size(shape: &str) -> Shape {
    let mut body = Vec::new();
    let (after, step): (&'static str, Step) = match shape {
        "disjoint-scopes" => {
            body.extend(lines(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
            ]));
            for (bit, name, c) in [(1, "a", 3), (2, "b", 5), (4, "c", 7)] {
                body.push(format!("    if (i & {bit}u) {{"));
                buffer(&mut body, 2, name, &format!("{c}u"), "x =");
                body.push("    }".to_owned());
            }
            body.push("    {".to_owned());
            buffer(&mut body, 2, "d", "11u", "x =");
            body.push("    }".to_owned());
            body.extend(lines(&["    return x + i;", "}"]));
            (
                "fs/select.c",
                Box::new(|mut x, i| {
                    for (bit, c) in [(1, 3), (2, 5), (4, 7)] {
                        if i & bit != 0 {
                            x = consumed(x, c, WORDS);
                        }
                    }
                    consumed(x, 11, WORDS).wrapping_add(i)
                }),
            )
        }
        "inlined-callees" => {
            for (name, c) in [("a", 3), ("b", 5), ("c", 7), ("d", 11)] {
                body.push("static inline __attribute__((always_inline))".to_owned());
                body.push(format!("unsigned int part_{name}(unsigned int x) {{"));
                buffer(&mut body, 1, "buf", &format!("{c}u"), "return");
                body.extend(lines(&["}", ""]));
            }
            body.extend(lines(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    x = part_a(x);",
                "    x = part_b(x ^ i);",
                "    x = part_c(x + i);",
                "    return part_d(x);",
                "}",
            ]));
            (
                "lib/vsprintf.c",
                Box::new(|x, i| {
                    let x = consumed(x, 3, WORDS);
                    let x = consumed(x ^ i, 5, WORDS);
                    let x = consumed(x.wrapping_add(i), 7, WORDS);
                    consumed(x, 11, WORDS)
                }),
            )
        }
        "switch-arms" => {
            body.extend(lines(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    switch (i & 3u) {",
            ]));
            for (arm, name, c) in [
                ("case 0u", "a", 3),
                ("case 1u", "b", 5),
                ("case 2u", "c", 7),
                ("default", "d", 11),
            ] {
                body.push(format!("    {arm}: {{"));
                buffer(&mut body, 2, name, &format!("{c}u"), "x =");
                body.extend(lines(&["        break;", "    }"]));
            }
            body.extend(lines(&["    }", "    return x ^ i;", "}"]));
            (
                "net/core/sock.c",
                Box::new(|x, i| {
                    let c = [3, 5, 7, 11][(i & 3) as usize];
                    consumed(x, c, WORDS) ^ i
                }),
            )
        }
        "nested-inline" => {
            body.extend(lines(&[
                "static inline __attribute__((always_inline))",
                "unsigned int inner(unsigned int x) {",
            ]));
            buffer(&mut body, 1, "buf", "5u", "return");
            body.extend(lines(&[
                "}",
                "",
                "static inline __attribute__((always_inline))",
                "unsigned int outer(unsigned int x) {",
            ]));
            buffer(&mut body, 1, "buf", "3u", "x =");
            body.extend(lines(&[
                "    return inner(x + 1u);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    x = outer(x ^ i);",
                "    return outer(x + i);",
                "}",
            ]));
            let outer = |x: u32| consumed(consumed(x, 3, WORDS).wrapping_add(1), 5, WORDS);
            ("lib/crypto/sha256.c", Box::new(move |x, i| outer(outer(x ^ i).wrapping_add(i))))
        }
        _ => {
            body.extend(lines(&[
                "static inline __attribute__((always_inline))",
                "unsigned int part(unsigned int x, unsigned int j) {",
            ]));
            buffer(&mut body, 1, "buf", "(3u + j)", "return");
            body.extend(lines(&[
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    for (unsigned int j = 0; j < 4u; j++) {",
                "        x = part(x + j, j);",
                "    }",
                "    return x ^ i;",
                "}",
            ]));
            (
                "lib/xz/xz_dec_lzma2.c",
                Box::new(|mut x, i| {
                    for j in 0..4 {
                        x = consumed(x.wrapping_add(j), 3 + j, WORDS);
                    }
                    x ^ i
                }),
            )
        }
    };
    Shape { after, top: with_consume(body), step }
}

fn objtool_shapes(shape: &str) -> Shape {
    match shape {
        "jump-table" => Shape::new(
            "arch/x86/lib/insn.c",
            &[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    switch ((x + i) & 7u) {",
                "    case 0u: return x * 3u + 1u;",
                "    case 1u: return x ^ 0x5bd1e995u;",
                "    case 2u: return x + i * 7u;",
                "    case 3u: return (x << 5) | (x >> 27);",
                "    case 4u: return x - 0x1234u;",
                "    case 5u: return (x * 33u) ^ i;",
                "    case 6u: return ~x + i;",
                "    default: return x + 0x9e3779b9u;",
                "    }",
                "}",
            ],
            |x, i| match x.wrapping_add(i) & 7 {
                0 => x.wrapping_mul(3).wrapping_add(1),
                1 => x ^ 0x5bd1_e995,
                2 => x.wrapping_add(i * 7),
                3 => x.rotate_left(5),
                4 => x.wrapping_sub(0x1234),
                5 => x.wrapping_mul(33) ^ i,
                6 => (!x).wrapping_add(i),
                _ => x.wrapping_add(0x9e37_79b9),
            },
        ),
        "noreturn" => Shape::new(
            "lib/bug.c",
            &[
                "__attribute__((noreturn, noinline))",
                "static void fail(unsigned int x) {",
                "    asm volatile(\"\" : : \"r\"(x));",
                "    __builtin_trap();",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int checked(unsigned int x) {",
                "    if (x == 0xdeadbeefu) {",
                "        fail(x);",
                "    }",
                "    return x * 5u + 3u;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    if (i > 1000u) {",
                "        fail(i);",
                "    }",
                "    return checked(x ^ i);",
                "}",
            ],
            |x, i| {
                assert_ne!(x ^ i, 0xdead_beef, "the noreturn case would trap");
                (x ^ i).wrapping_mul(5).wrapping_add(3)
            },
        ),
        "stack-realign" => {
            let mut top: Vec<String> = CONSUME.iter().map(|line| (*line).to_owned()).collect();
            top.extend(lines(&[
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int buf[16] __attribute__((aligned(64)));",
                "    for (unsigned int k = 0; k < 16u; k++) {",
                "        buf[k] = x + k * (i | 1u);",
                "    }",
                "    x = consume(buf, 16u, x);",
                "    return x + (unsigned int)((unsigned long)buf & 63u);",
                "}",
            ]));
            Shape {
                after: "arch/x86/kernel/fpu/xstate.c",
                top,
                step: Box::new(|x, i| consumed(x, i | 1, 16)),
            }
        }
        "unreachable-end" => Shape::new(
            "arch/x86/kvm/emulate.c",
            &[
                "__attribute__((noinline))",
                "static unsigned int pick(unsigned int k, unsigned int x) {",
                "    switch (k & 3u) {",
                "    case 0u: return x + 1u;",
                "    case 1u: return x * 3u;",
                "    case 2u: return x ^ 0x55aa55aau;",
                "    case 3u: return x - 7u;",
                "    }",
                "    __builtin_unreachable();",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    return pick(x + i, x) + i;",
                "}",
            ],
            |x, i| {
                let picked = match x.wrapping_add(i) & 3 {
                    0 => x.wrapping_add(1),
                    1 => x.wrapping_mul(3),
                    2 => x ^ 0x55aa_55aa,
                    _ => x.wrapping_sub(7),
                };
                picked.wrapping_add(i)
            },
        ),
        _ => Shape::new(
            "fs/read_write.c",
            &[
                "__attribute__((noinline))",
                "static unsigned int odd(unsigned int x, unsigned int i) {",
                "    return x * 7u + i;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int even(unsigned int x, unsigned int i) {",
                "    return (x >> 1) ^ (i * 0x01000193u);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    if (x & 1u) {",
                "        return odd(x, i);",
                "    }",
                "    return even(x + i, i);",
                "}",
            ],
            |x, i| {
                if x & 1 == 1 {
                    x.wrapping_mul(7).wrapping_add(i)
                } else {
                    (x.wrapping_add(i) >> 1) ^ i.wrapping_mul(0x0100_0193)
                }
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
            (Facet::ObjtoolShapes, 5, "-O2"),
            (Facet::FrameSize, 5, "-O2"),
        ] {
            let cases: Vec<_> = manifest
                .cases
                .iter()
                .filter(|case| case.facet == facet && case.has_tag("provenance:linux"))
                .collect();
            assert_eq!(cases.len(), count, "{facet:?}");
            for case in cases {
                assert!(case.flags.iter().any(|given| given == flag), "{}", case.id);
                assert_eq!(case.has_tag("x86-64"), facet != Facet::FrameSize, "{}", case.id);
                assert_eq!(case.has_tag("elf"), facet == Facet::McmodelKernel, "{}", case.id);
            }
        }
    }

    #[test]
    fn every_kernel_frame_has_four_buffers_of_640_bytes() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest
            .cases
            .iter()
            .filter(|case| case.facet == Facet::FrameSize && case.has_tag("provenance:linux"))
        {
            let buffers = case.source.matches("[160];").count();
            let expected = if case.id.contains("nested-inline") {
                2
            } else if case.id.contains("loop-inline") {
                1
            } else {
                4
            };
            assert_eq!(buffers, expected, "{}", case.id);
            assert!(case.source.contains("consume("), "{}", case.id);
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
