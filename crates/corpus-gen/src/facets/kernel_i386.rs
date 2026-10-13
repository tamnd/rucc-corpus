//! The facet that is the way the 32 bit x86 kernel is built.
//!
//! `i386-kernel` is programs built with the flags `arch/x86/Makefile` gives an i386 kernel:
//! `-m32 -march=i686 -mregparm=3 -freg-struct-return` and no PIE. Every function takes its first
//! three words in `eax`, `edx` and `ecx`, and a small structure comes back in registers. `printf`
//! takes a variable number of arguments, so it keeps the stack convention, and the cases need no
//! other function from the C library.
//!
//! The kernel links without libgcc on i386, so a compiler may not call a libgcc helper where GCC
//! does not. Each case defines the helpers itself, and each one only counts that it was called.
//! A case whose code called one prints 0 instead of its answer. The cases pin `-O2`, as the kernel
//! does, and only use 64 bit operations that GCC 14 and GCC 16 keep inline there: shifts by a
//! variable, multiplies, division by a power of two, division of two 32 bit values, `clzll`,
//! `bswap64` and compares.
//!
//! The cases need a 32 bit C library to link and run, so they carry the `i386` tag, and a run on
//! a machine without one leaves them out with `--exclude-tag i386`. Each case names the kernel
//! file it has the shape of.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet};

/// The tag that says where a case came from.
const PROVENANCE: &str = "provenance:linux";

/// The tags every case carries.
const TAGS: &[&str] = &["gnu", "x86-64", PROVENANCE, "elf", "i386"];

/// The flags of an i386 kernel object, and `-no-pie` for the link.
const FLAGS: &[&str] =
    &["-m32", "-march=i686", "-mregparm=3", "-freg-struct-return", "-fno-pie", "-no-pie", "-O2"];

/// The shapes, in the order they are emitted.
const SHAPES: &[&str] = &[
    "regparm-calls",
    "regparm-pointer",
    "struct-return",
    "shift-64",
    "multiply-64",
    "divide-power-of-two",
    "divide-narrow",
    "bit-scan-64",
    "compare-64",
];

/// The seed every case reads through a `volatile` global.
const SEED: u32 = 7;

/// How many times each case runs its step.
const ROUNDS: u32 = 64;

/// The libgcc helpers a compiler could call for a 64 bit operation on i386, as C definitions
/// that count the call. The kernel defines none of them.
const HELPERS: &[&str] = &[
    "static unsigned int introduced;",
    "",
    "typedef unsigned long long rk_u64;",
    "typedef long long rk_s64;",
    "",
    "rk_u64 __ashldi3(rk_u64 a, int b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_u64 __lshrdi3(rk_u64 a, int b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_s64 __ashrdi3(rk_s64 a, int b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_s64 __muldi3(rk_s64 a, rk_s64 b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_s64 __divdi3(rk_s64 a, rk_s64 b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_s64 __moddi3(rk_s64 a, rk_s64 b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_u64 __udivdi3(rk_u64 a, rk_u64 b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_u64 __umoddi3(rk_u64 a, rk_u64 b) { (void)a; (void)b; introduced++; return 0; }",
    "rk_u64 __udivmoddi4(rk_u64 a, rk_u64 b, rk_u64 *c) {",
    "    (void)a; (void)b; (void)c; introduced++; return 0;",
    "}",
    "rk_s64 __divmoddi4(rk_s64 a, rk_s64 b, rk_s64 *c) {",
    "    (void)a; (void)b; (void)c; introduced++; return 0;",
    "}",
    "int __cmpdi2(rk_s64 a, rk_s64 b) { (void)a; (void)b; introduced++; return 1; }",
    "int __ucmpdi2(rk_u64 a, rk_u64 b) { (void)a; (void)b; introduced++; return 1; }",
    "rk_s64 __negdi2(rk_s64 a) { (void)a; introduced++; return 0; }",
    "int __clzdi2(rk_u64 a) { (void)a; introduced++; return 0; }",
    "int __ctzdi2(rk_u64 a) { (void)a; introduced++; return 0; }",
    "int __ffsdi2(rk_u64 a) { (void)a; introduced++; return 0; }",
    "int __popcountdi2(rk_u64 a) { (void)a; introduced++; return 0; }",
    "int __popcountsi2(unsigned int a) { (void)a; introduced++; return 0; }",
    "rk_u64 __bswapdi2(rk_u64 a) { (void)a; introduced++; return 0; }",
    "",
];

/// Emits every case of the facet.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &shape in SHAPES {
        if !sink.wants(Facet::I386Kernel) {
            break;
        }
        let (source, expected) = case(shape).finish();
        let case = Case::linked(
            Facet::I386Kernel,
            Axes::of([("shape", shape)]),
            Dialect::C17,
            source,
            Vec::new(),
            FLAGS.iter().map(|flag| (*flag).to_owned()).collect(),
            Expect::Output(expected),
        )
        .tagged(TAGS);
        sink.push_case(case);
    }
}

/// What a case is: the file it has the shape of, the C above `main`, and the step it runs as
/// Rust. The step is `step(x, i)` in the C.
struct Shape {
    after: &'static str,
    top: &'static [&'static str],
    step: fn(u32, u32) -> u32,
}

/// One case.
fn case(shape: &str) -> Program {
    let found = shape_of(shape);
    let mut program = Program::new(format!("{shape} after {} in Linux 7.2", found.after));
    program.top(format!("static volatile unsigned int seed_in = {SEED};"));
    program.top("");
    for line in HELPERS.iter().chain(found.top) {
        program.top(*line);
    }
    program.line("unsigned int acc = seed_in;");
    program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
    program.line_at(1, "acc = step(acc, i);");
    program.line("}");
    program.line("if (introduced) {");
    program.line_at(1, "acc = 0u;");
    program.line("}");
    program.blank();
    let mut acc = SEED;
    for i in 0..ROUNDS {
        acc = (found.step)(acc, i);
    }
    program.check(Ty::U32, "acc", i128::from(acc));
    program
}

/// The 64 bit value most steps start from.
fn wide(x: u32, i: u32) -> u64 {
    (u64::from(x) << 32) | u64::from(x ^ i.wrapping_mul(0x9e37_79b9))
}

/// The same, as C.
const WIDE: &str = "    rk_u64 wide = ((rk_u64)x << 32) | (x ^ i * 0x9e3779b9u);";

/// The two halves of a 64 bit value folded into one word.
const fn fold(v: u64) -> u32 {
    (v as u32) ^ ((v >> 32) as u32)
}

fn shape_of(shape: &str) -> Shape {
    match shape {
        "regparm-calls" => Shape {
            after: "arch/x86/include/asm/linkage.h",
            top: &[
                "__attribute__((noinline))",
                "unsigned int rk_mix3(unsigned int a, rk_u64 b, unsigned int c) {",
                "    return a * 3u + (unsigned int)(b >> 29) + (unsigned int)b * c;",
                "}",
                "",
                "__attribute__((noinline))",
                "unsigned int rk_spill(unsigned int a, unsigned int b, unsigned int c,",
                "                      unsigned int d, rk_u64 e) {",
                "    return (a ^ b) + c * d + (unsigned int)(e >> 7);",
                "}",
                "",
                "__attribute__((regparm(0), noinline))",
                "unsigned int rk_asmlinkage(unsigned int a, rk_u64 b, unsigned int c) {",
                "    return a - (unsigned int)(b >> 32) + c;",
                "}",
                "",
                "__attribute__((regparm(1), noinline))",
                "unsigned int rk_one(rk_u64 a, unsigned int b) {",
                "    return (unsigned int)(a >> 3) ^ b;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                WIDE,
                "    unsigned int r = rk_mix3(x, wide, i) ^ rk_spill(x, i, x >> 3, i | 1u, wide);",
                "    return r + (rk_asmlinkage(i, wide, x) ^ rk_one(wide, i));",
                "}",
            ],
            step: |x, i| {
                let wide = wide(x, i);
                let mix3 = x
                    .wrapping_mul(3)
                    .wrapping_add((wide >> 29) as u32)
                    .wrapping_add((wide as u32).wrapping_mul(i));
                let spill = (x ^ i)
                    .wrapping_add((x >> 3).wrapping_mul(i | 1))
                    .wrapping_add((wide >> 7) as u32);
                let asmlinkage = i.wrapping_sub((wide >> 32) as u32).wrapping_add(x);
                let one = ((wide >> 3) as u32) ^ i;
                (mix3 ^ spill).wrapping_add(asmlinkage ^ one)
            },
        },
        "regparm-pointer" => Shape {
            after: "arch/x86/kernel/irq_32.c",
            top: &[
                "typedef unsigned int (*rk_handler)(unsigned int, rk_u64, unsigned int);",
                "",
                "__attribute__((noinline))",
                "unsigned int rk_add(unsigned int a, rk_u64 b, unsigned int c) {",
                "    return a + (unsigned int)b + c;",
                "}",
                "",
                "__attribute__((noinline))",
                "unsigned int rk_xor(unsigned int a, rk_u64 b, unsigned int c) {",
                "    return a * 0x01000193u ^ (unsigned int)(b >> 24) ^ c;",
                "}",
                "",
                "__attribute__((noinline))",
                "unsigned int rk_mul(unsigned int a, rk_u64 b, unsigned int c) {",
                "    return a * (c | 1u) + (unsigned int)(b >> 16);",
                "}",
                "",
                "__attribute__((noinline))",
                "unsigned int rk_sub(unsigned int a, rk_u64 b, unsigned int c) {",
                "    return a - c - (unsigned int)(b >> 8);",
                "}",
                "",
                "static rk_handler const handlers[4] = { rk_add, rk_xor, rk_mul, rk_sub };",
                "static rk_handler volatile slot;",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                WIDE,
                "    slot = handlers[(x ^ i) & 3u];",
                "    return slot(x, wide, i) + i;",
                "}",
            ],
            step: |x, i| {
                let wide = wide(x, i);
                let r = match (x ^ i) & 3 {
                    0 => x.wrapping_add(wide as u32).wrapping_add(i),
                    1 => x.wrapping_mul(0x0100_0193) ^ ((wide >> 24) as u32) ^ i,
                    2 => x.wrapping_mul(i | 1).wrapping_add((wide >> 16) as u32),
                    _ => x.wrapping_sub(i).wrapping_sub((wide >> 8) as u32),
                };
                r.wrapping_add(i)
            },
        },
        "struct-return" => Shape {
            after: "arch/x86/include/asm/pgtable_types.h",
            top: &[
                "struct rk_pte { rk_u64 val; };",
                "struct rk_pair { unsigned int lo, hi; };",
                "struct rk_flag { unsigned char on; };",
                "struct rk_three { unsigned int a, b, c; };",
                "",
                "__attribute__((noinline))",
                "struct rk_pte rk_make_pte(unsigned int x, unsigned int i) {",
                "    struct rk_pte p = { ((rk_u64)x << 12) | i };",
                "    return p;",
                "}",
                "",
                "__attribute__((noinline))",
                "struct rk_pair rk_split(rk_u64 v) {",
                "    struct rk_pair p = { (unsigned int)v, (unsigned int)(v >> 32) };",
                "    return p;",
                "}",
                "",
                "__attribute__((noinline))",
                "struct rk_flag rk_flag_of(unsigned int x) {",
                "    struct rk_flag f = { (unsigned char)(x >> 5) };",
                "    return f;",
                "}",
                "",
                "__attribute__((noinline))",
                "struct rk_three rk_spread(unsigned int x) {",
                "    struct rk_three t = { x, x * 3u, x ^ 0x55u };",
                "    return t;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    struct rk_pte p = rk_make_pte(x, i);",
                "    struct rk_pair q = rk_split(p.val * 0x9e3779b97f4a7c15ull);",
                "    struct rk_flag f = rk_flag_of(q.lo ^ q.hi);",
                "    struct rk_three t = rk_spread(q.lo + f.on);",
                "    return (t.a + t.b * 7u + t.c) ^ i;",
                "}",
            ],
            step: |x, i| {
                let pte = (u64::from(x) << 12) | u64::from(i);
                let v = pte.wrapping_mul(0x9e37_79b9_7f4a_7c15);
                let (lo, hi) = (v as u32, (v >> 32) as u32);
                let on = ((lo ^ hi) >> 5) & 0xff;
                let a = lo.wrapping_add(on);
                let (b, c) = (a.wrapping_mul(3), a ^ 0x55);
                a.wrapping_add(b.wrapping_mul(7)).wrapping_add(c) ^ i
            },
        },
        "shift-64" => Shape {
            after: "include/linux/math64.h",
            top: &[
                "__attribute__((noinline))",
                "static rk_u64 shifted(rk_u64 v, unsigned int n) {",
                "    return (v << (n & 63u)) ^ (v >> ((n * 3u) & 63u))",
                "        ^ (rk_u64)((rk_s64)v >> ((n * 7u) & 63u));",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                WIDE,
                "    rk_u64 r = shifted(wide, x + i);",
                "    return (unsigned int)r ^ (unsigned int)(r >> 32);",
                "}",
            ],
            step: |x, i| {
                let v = wide(x, i);
                let n = x.wrapping_add(i);
                let r = (v << (n & 63))
                    ^ (v >> (n.wrapping_mul(3) & 63))
                    ^ ((v as i64) >> (n.wrapping_mul(7) & 63)) as u64;
                fold(r)
            },
        },
        "multiply-64" => Shape {
            after: "include/linux/math64.h",
            top: &[
                "__attribute__((noinline))",
                "static rk_u64 product(rk_u64 a, rk_u64 b, unsigned int c) {",
                "    return a * b + (rk_u64)c * (unsigned int)a",
                "        + (((rk_u64)(unsigned int)a * c) >> 32)",
                "        + (rk_u64)((rk_s64)(int)c * (rk_s64)(int)b);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                WIDE,
                "    rk_u64 other = ((rk_u64)(x + i) << 17) ^ 0x1234567890abcdefull;",
                "    return (unsigned int)(product(wide, other, x ^ i) >> 11);",
                "}",
            ],
            step: |x, i| {
                let a = wide(x, i);
                let b = (u64::from(x.wrapping_add(i)) << 17) ^ 0x1234_5678_90ab_cdef;
                let c = x ^ i;
                let r = a
                    .wrapping_mul(b)
                    .wrapping_add(u64::from(c).wrapping_mul(u64::from(a as u32)))
                    .wrapping_add((u64::from(a as u32) * u64::from(c)) >> 32)
                    .wrapping_add(i64::from(c as i32).wrapping_mul(i64::from(b as i32)) as u64);
                (r >> 11) as u32
            },
        },
        "divide-power-of-two" => Shape {
            after: "include/linux/log2.h",
            top: &[
                "__attribute__((noinline))",
                "static rk_u64 scaled(rk_u64 a, rk_s64 b) {",
                "    return a / 4096u + a % 4096u + (rk_u64)(b / 8) + (rk_u64)(b % 16)",
                "        + a / (1ull << 40) + (rk_u64)(b >> 3);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    rk_u64 a = ((rk_u64)x << 29) ^ (rk_u64)i * 0x9e3779b97f4a7c15ull;",
                "    rk_s64 b = (rk_s64)(a - ((rk_u64)x << 40));",
                "    rk_u64 r = scaled(a, b);",
                "    return (unsigned int)r ^ (unsigned int)(r >> 32);",
                "}",
            ],
            step: |x, i| {
                let a = (u64::from(x) << 29) ^ u64::from(i).wrapping_mul(0x9e37_79b9_7f4a_7c15);
                let b = a.wrapping_sub(u64::from(x) << 40) as i64;
                let r = (a / 4096)
                    .wrapping_add(a % 4096)
                    .wrapping_add((b / 8) as u64)
                    .wrapping_add((b % 16) as u64)
                    .wrapping_add(a / (1 << 40))
                    .wrapping_add((b >> 3) as u64);
                fold(r)
            },
        },
        "divide-narrow" => Shape {
            after: "kernel/time/time.c",
            top: &[
                "__attribute__((noinline))",
                "static rk_u64 ratio(unsigned int a, unsigned int b) {",
                "    return (rk_u64)a / b + ((rk_u64)a % b << 32);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    rk_u64 r = ratio(x * 2654435761u + i, (x >> 7) | 1u);",
                "    return (unsigned int)r ^ (unsigned int)(r >> 32);",
                "}",
            ],
            step: |x, i| {
                let a = u64::from(x.wrapping_mul(2_654_435_761).wrapping_add(i));
                let b = u64::from((x >> 7) | 1);
                fold((a / b).wrapping_add((a % b) << 32))
            },
        },
        "bit-scan-64" => Shape {
            after: "include/asm-generic/bitops/fls64.h",
            top: &[
                "__attribute__((noinline))",
                "static rk_u64 scanned(rk_u64 v) {",
                "    return (rk_u64)__builtin_clzll(v | 1u) * 0x10001u + __builtin_bswap64(v);",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                WIDE,
                "    rk_u64 r = scanned(wide >> (i & 63u));",
                "    return (unsigned int)r ^ (unsigned int)(r >> 32);",
                "}",
            ],
            step: |x, i| {
                let v = wide(x, i) >> (i & 63);
                let r = u64::from((v | 1).leading_zeros())
                    .wrapping_mul(0x10001)
                    .wrapping_add(v.swap_bytes());
                fold(r)
            },
        },
        _ => Shape {
            after: "include/linux/jiffies.h",
            top: &[
                "__attribute__((noinline))",
                "static unsigned int ordered(rk_u64 a, rk_u64 b, rk_s64 c, rk_s64 d) {",
                "    unsigned int r = 0;",
                "    if (a < b) {",
                "        r |= 1u;",
                "    }",
                "    if (c < d) {",
                "        r |= 2u;",
                "    }",
                "    if (a == b) {",
                "        r |= 4u;",
                "    }",
                "    if (c >= d) {",
                "        r |= 8u;",
                "    }",
                "    if ((rk_s64)(a - b) > 0) {",
                "        r |= 16u;",
                "    }",
                "    return r;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    rk_u64 a = ((rk_u64)x << 32) | i;",
                "    rk_u64 b = ((rk_u64)(x ^ (i << 3)) << 32) | (x & 0xffu);",
                "    rk_s64 c = (rk_s64)(a * 0x9e3779b97f4a7c15ull);",
                "    rk_s64 d = (rk_s64)(b ^ 0x8000000000000000ull);",
                "    return x * 31u + ordered(a, b, c, d) + ordered(a, a, d, d) * 32u;",
                "}",
            ],
            step: |x, i| {
                let ordered = |a: u64, b: u64, c: i64, d: i64| {
                    u32::from(a < b)
                        | u32::from(c < d) << 1
                        | u32::from(a == b) << 2
                        | u32::from(c >= d) << 3
                        | u32::from(a.wrapping_sub(b) as i64 > 0) << 4
                };
                let a = (u64::from(x) << 32) | u64::from(i);
                let b = (u64::from(x ^ (i << 3)) << 32) | u64::from(x & 0xff);
                let c = a.wrapping_mul(0x9e37_79b9_7f4a_7c15) as i64;
                let d = (b ^ 0x8000_0000_0000_0000) as i64;
                x.wrapping_mul(31)
                    .wrapping_add(ordered(a, b, c, d))
                    .wrapping_add(ordered(a, a, d, d).wrapping_mul(32))
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{FLAGS, SHAPES};
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_shape_is_there_with_the_kernel_flags_and_the_tag() {
        let manifest = generate(&Options::all()).unwrap();
        let cases: Vec<_> =
            manifest.cases.iter().filter(|case| case.facet == Facet::I386Kernel).collect();
        assert_eq!(cases.len(), SHAPES.len());
        for case in cases {
            assert_eq!(case.flags, FLAGS, "{}", case.id);
            assert!(case.has_tag("i386"), "{}", case.id);
            assert!(case.has_tag("provenance:linux"), "{}", case.id);
        }
    }

    #[test]
    fn every_case_counts_each_helper() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest.cases.iter().filter(|case| case.facet == Facet::I386Kernel) {
            for helper in ["__udivdi3", "__ashldi3", "__muldi3", "__clzdi2", "__bswapdi2"] {
                assert!(case.source.contains(&format!(" {helper}(")), "{}", case.id);
            }
            assert!(case.source.contains("if (introduced) {"), "{}", case.id);
        }
    }
}
