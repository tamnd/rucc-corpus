//! The facets that are the Linux kernel's front end: what counts as a constant, and where a
//! definition goes.
//!
//! `null-pointer-constant` is the null pointer constants the kernel builds out of integer
//! constant expressions. `__is_constexpr` casts `(long)(x) * 0l` to `void *` and asks whether the
//! conditional it sits in takes the type of the other arm, which it only does when the cast is a
//! null pointer constant, so when `x` is a constant. `is_const` asks the same through
//! `_Generic`, and the old `min` and `max` pick between a plain conditional and a statement
//! expression with the answer. The other cases are the spellings of a null pointer that are and
//! are not null pointer constants, told apart by the type of a conditional, and null pointers
//! compared and converted.
//!
//! `const-ice` is the integer constant expressions built out of `sizeof` and `offsetof`: array
//! sizes with `ARRAY_SIZE`, enumerators, case labels that are ioctl numbers, `static_assert`,
//! `BUILD_BUG_ON_ZERO` inside `GENMASK`, `__builtin_constant_p` on all of these, and casts and
//! conditionals that stay constant. One case is the GCC extension that folds a `const` local with
//! a constant initializer wherever a constant is needed, which Linux 7.2 leans on now that its
//! minimum Clang does the same. GCC only folds them with the optimizer on, so that case pins
//! `-O2`.
//!
//! `section-attr` is definitions put in named sections: tables of initcalls and parameters walked
//! from the `__start_` symbol to the `__stop_` symbol the linker defines, entries that only
//! survive because they are `used`, entries that must not be aligned past their type, init text,
//! and the data sections. A section a case walks has a name that is a C identifier, since that is
//! when GNU ld defines the two symbols. Mach-O and COFF have neither, so these cases carry the
//! `elf` tag.
//!
//! Each case runs a step function over a counter the way the other kernel facets do, and every
//! constant a case is about feeds the step, so a probe that answers wrongly changes what the
//! program prints. Every size the first two facets take is of `unsigned int`, `char` or a struct
//! made of them, which is the same on every target the corpus runs on, and no size is printed.

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

/// The tags of a case that is plain C and runs anywhere.
const PORTABLE: &[&str] = &["gnu", PROVENANCE];

/// The tags of a case that needs an ELF linker to define `__start_` and `__stop_`.
const ELF: &[&str] = &["gnu", "elf", PROVENANCE];

/// The one shape that only holds with the optimizer on. GCC folds a `const` local into an integer
/// constant expression at `-O1` and above and rejects the same program at `-O0`.
const OPTIMIZED: &str = "const-local";

/// Emits every facet in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &(facet, shapes) in FACETS {
        for &shape in shapes {
            if !sink.wants(facet) {
                break;
            }
            let (source, expected) = case(facet, shape).finish();
            let flags = if shape == OPTIMIZED { vec!["-O2".to_owned()] } else { Vec::new() };
            let case = Case::linked(
                facet,
                Axes::of([("shape", shape)]),
                Dialect::C17,
                source,
                Vec::new(),
                flags,
                Expect::Output(expected),
            )
            .tagged(if facet == Facet::SectionAttr { ELF } else { PORTABLE });
            sink.push_case(case);
        }
    }
}

/// Each facet and its shapes.
const FACETS: &[(Facet, &[&str])] = &[
    (
        Facet::NullPointerConstant,
        &["is-constexpr", "is-const", "choose-expr", "conditional-type", "compare", "convert"],
    ),
    (
        Facet::ConstIce,
        &[
            "array-size",
            "enum",
            "case-label",
            "static-assert",
            "build-bug-on-zero",
            "constant-p",
            "const-local",
            "cast-conditional",
        ],
    ),
    (
        Facet::SectionAttr,
        &[
            "initcall-table",
            "initcall-levels",
            "param-table",
            "aligned-entries",
            "init-text",
            "data-sections",
        ],
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
    fn new(
        after: &'static str,
        top: Vec<String>,
        step: impl FnMut(u32, u32) -> u32 + 'static,
    ) -> Self {
        Self { after, top, step: Box::new(step) }
    }
}

/// One case.
fn case(facet: Facet, shape: &str) -> Program {
    let mut found = match facet {
        Facet::NullPointerConstant => null_pointer_constant(shape),
        Facet::ConstIce => const_ice(shape),
        _ => section_attr(shape),
    };
    let mut program = Program::new(format!("{shape} after {} in Linux 7.2", found.after));
    program.top(format!("static volatile unsigned int seed_in = {SEED};"));
    program.top("");
    for line in &found.top {
        program.top(line.clone());
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
    program
}

fn lines(text: &[&str]) -> Vec<String> {
    text.iter().map(|line| (*line).to_owned()).collect()
}

fn joined(parts: &[&[&str]]) -> Vec<String> {
    lines(&parts.concat())
}

/// The lines of a declaration that packs one bit per probe, and the value the bits must make. A
/// probe is the C that asks and whether it should answer yes. `first` starts the declaration and
/// `end` closes its last line.
fn packed(first: &str, end: &str, probes: &[(&str, bool)]) -> (Vec<String>, u32) {
    let mut text = Vec::new();
    let mut value = 0;
    for (bit, &(probe, yes)) in probes.iter().enumerate() {
        let lead = if bit == 0 { format!("    {first}") } else { "        | ".to_owned() };
        text.push(format!("{lead}({probe} << {bit})"));
        if yes {
            value |= 1 << bit;
        }
    }
    if let Some(last) = text.last_mut() {
        last.push_str(end);
    }
    (text, value)
}

/// The kernel's test for an integer constant expression, from include/linux/const.h.
const IS_CONSTEXPR: &[&str] = &[
    "#define __is_constexpr(x) \\",
    "    (sizeof(int) == sizeof(*(8 ? ((void *)((long)(x) * 0l)) : (int *)8)))",
    "",
];

/// A constant and a struct for the probes to take the size of. It is twelve bytes everywhere.
const PAIR: &[&str] = &[
    "#define RK_LIMIT 24",
    "",
    "struct rk_pair {",
    "    unsigned int a;",
    "    unsigned char b[6];",
    "};",
    "",
];

/// A request of 32 bytes on every target, for the `const-ice` cases to measure.
const REQ: &[&str] = &[
    "struct rk_req {",
    "    unsigned int op;",
    "    unsigned int flags;",
    "    unsigned char key[8];",
    "    unsigned char data[16];",
    "};",
    "",
];

/// The probes at file scope, which have to be integer constant expressions themselves, and the
/// probes inside the step, which may ask about its parameters. The step is `x` mixed with both.
fn probed(
    after: &'static str,
    head: &[&str],
    outside: &[(&str, bool)],
    inside: &[(&str, bool)],
    mix: fn(u32, u32, u32, u32) -> u32,
    ret: &str,
) -> Shape {
    let mut top = lines(head);
    let (found, found_value) = packed("rk_found = ", "", outside);
    top.push("enum {".to_owned());
    top.extend(found);
    top.extend(lines(&[
        "};",
        "",
        "__attribute__((noinline))",
        "static unsigned int step(unsigned int x, unsigned int i) {",
    ]));
    let (seen, seen_value) = packed("unsigned int seen = ", ";", inside);
    top.extend(seen);
    top.push(format!("    return {ret};"));
    top.push("}".to_owned());
    Shape::new(after, top, move |x, i| mix(x, i, found_value, seen_value))
}

fn null_pointer_constant(shape: &str) -> Shape {
    match shape {
        "is-constexpr" => probed(
            "include/linux/const.h",
            &[IS_CONSTEXPR, PAIR].concat(),
            &[
                ("__is_constexpr(7)", true),
                ("__is_constexpr(RK_LIMIT * 3 - 1)", true),
                ("__is_constexpr((unsigned char)300)", true),
                ("__is_constexpr('a' - 'A')", true),
                ("__is_constexpr(sizeof(struct rk_pair))", true),
                ("__is_constexpr(__builtin_offsetof(struct rk_pair, b))", true),
                ("__is_constexpr((int)2.5)", true),
                ("__is_constexpr(1 ? 3 : 4)", true),
                ("__is_constexpr(seed_in)", false),
                ("__is_constexpr(seed_in * 0)", false),
                ("__is_constexpr(0 && seed_in)", false),
            ],
            &[
                ("__is_constexpr(x)", false),
                ("__is_constexpr(i * 0u)", false),
                ("__is_constexpr(RK_LIMIT + 1)", true),
            ],
            |x, i, found, seen| (x ^ found).wrapping_mul(33).wrapping_add(seen).wrapping_add(i),
            "(x ^ rk_found) * 33u + seen + i",
        ),
        "is-const" => probed(
            "include/linux/compiler.h",
            &[
                &[
                    "#define __is_const_zero(x) \\",
                    "    _Generic(0 ? (void *)(long)(x) : (char *)0, char *: 1, void *: 0)",
                    "#define is_const(x) __is_const_zero(0 * (x))",
                    "#define is_const_true(x) __is_const_zero(!(x))",
                    "#define is_const_false(x) __is_const_zero(x)",
                    "",
                ],
                PAIR,
            ]
            .concat(),
            &[
                ("is_const(7)", true),
                ("is_const(RK_LIMIT * 3 - 1)", true),
                ("is_const(sizeof(struct rk_pair))", true),
                ("is_const(__builtin_offsetof(struct rk_pair, b))", true),
                ("is_const(seed_in)", false),
                ("is_const_true(RK_LIMIT > 3)", true),
                ("is_const_true(RK_LIMIT < 3)", false),
                ("is_const_false(RK_LIMIT < 3)", true),
                ("is_const_false(RK_LIMIT > 3)", false),
                ("is_const_false(seed_in)", false),
            ],
            &[
                ("is_const(x)", false),
                ("is_const(i)", false),
                ("is_const_true(i < 64u)", false),
                ("is_const(RK_LIMIT)", true),
            ],
            |x, i, found, seen| x.wrapping_add(found).wrapping_mul(17).wrapping_add(seen ^ i),
            "(x + rk_found) * 17u + (seen ^ i)",
        ),
        "choose-expr" => {
            let mut window = [0u8; 40];
            let mut calls = 0u32;
            Shape::new(
                "include/linux/minmax.h",
                joined(&[
                    IS_CONSTEXPR,
                    &[
                        "#define __cmp(x, y, op) ((x) op (y) ? (x) : (y))",
                        "#define __cmp_once(x, y, op) ({ \\",
                        "    __typeof__(x) __x = (x); \\",
                        "    __typeof__(y) __y = (y); \\",
                        "    __cmp(__x, __y, op); })",
                        "#define __careful_cmp(x, y, op) \\",
                        "    __builtin_choose_expr(__is_constexpr(x) && __is_constexpr(y), \\",
                        "        __cmp(x, y, op), __cmp_once(x, y, op))",
                        "#define min(x, y) __careful_cmp(x, y, <)",
                        "#define max(x, y) __careful_cmp(x, y, >)",
                        "",
                        "#define RK_LIMIT 24u",
                        "#define RK_WIDTH 40u",
                        "",
                        "static unsigned int calls;",
                        "",
                        "__attribute__((noinline))",
                        "static unsigned int counted(unsigned int v) {",
                        "    calls++;",
                        "    return v;",
                        "}",
                        "",
                        "__attribute__((noinline))",
                        "static unsigned int step(unsigned int x, unsigned int i) {",
                        "    static unsigned char window[max(RK_LIMIT, RK_WIDTH)];",
                        "    unsigned int hi = max(counted(x & 63u), counted(i));",
                        "    unsigned int lo = min(counted(x >> 26), RK_LIMIT);",
                        "    unsigned int picked = __builtin_choose_expr(__is_constexpr(x), 100u, 7u)",
                        "        + __builtin_choose_expr(__is_constexpr(RK_WIDTH - RK_LIMIT), 1000u, 3u);",
                        "    window[(x + i) % max(RK_LIMIT, RK_WIDTH)] += (unsigned char)hi;",
                        "    return x * 5u + hi + lo + calls + picked + window[(x ^ i) % max(RK_LIMIT, RK_WIDTH)];",
                        "}",
                    ],
                ]),
                move |x, i| {
                    calls += 3;
                    let hi = (x & 63).max(i);
                    let lo = (x >> 26).min(24);
                    let at = (x.wrapping_add(i) % 40) as usize;
                    window[at] = window[at].wrapping_add((hi & 0xff) as u8);
                    x.wrapping_mul(5)
                        .wrapping_add(hi)
                        .wrapping_add(lo)
                        .wrapping_add(calls)
                        .wrapping_add(1007)
                        .wrapping_add(u32::from(window[((x ^ i) % 40) as usize]))
                },
            )
        }
        "conditional-type" => {
            let mut word = 5u32;
            let (found, found_value) = packed(
                "rk_found = ",
                "",
                &[
                    ("rk_kind(0)", true),
                    ("rk_kind(0L)", true),
                    ("rk_kind('\\0')", true),
                    ("rk_kind((void *)0)", true),
                    ("rk_kind((void *)(1 - 1))", true),
                    ("rk_kind((void *)(unsigned char)256)", true),
                    ("rk_kind((void *)(RK_LIMIT - 24))", true),
                    ("rk_kind((void *)'\\0')", true),
                    ("rk_kind((unsigned int *)0)", true),
                    ("rk_kind((void *)rk_zero)", false),
                    ("rk_kind((void *)(void *)0)", false),
                    ("rk_kind((const void *)0)", false),
                ],
            );
            let mut top = lines(&[
                "#define RK_LIMIT 24",
                "#define rk_kind(e) _Generic(1 ? &rk_word : (e), unsigned int *: 1, default: 0)",
                "",
                "static unsigned int rk_word = 5u;",
                "static __UINTPTR_TYPE__ rk_zero;",
                "",
                "enum {",
            ]);
            top.extend(found);
            top.extend(lines(&[
                "};",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    rk_word += i & 3u;",
                "    unsigned int *p = (x & 1u) ? &rk_word : (void *)(RK_LIMIT - 24);",
                "    unsigned int *q = (i & 2u) ? 0 : &rk_word;",
                "    return (x ^ rk_found) + (p ? *p : 3u) * 7u + (q ? 11u : 13u) + i;",
                "}",
            ]));
            Shape::new("include/linux/stddef.h", top, move |x, i| {
                word = word.wrapping_add(i & 3);
                let p = if x & 1 == 1 { word } else { 3 };
                let q = if i & 2 == 2 { 13 } else { 11 };
                (x ^ found_value).wrapping_add(p.wrapping_mul(7)).wrapping_add(q).wrapping_add(i)
            })
        }
        "compare" => {
            let mut slots = [0u32; 4];
            Shape::new(
                "include/linux/err.h",
                lines(&[
                    "#define NULL ((void *)0)",
                    "#define RK_LIMIT 24",
                    "",
                    "static unsigned int slots[4];",
                    "static unsigned int *const table[8] = {",
                    "    &slots[0], 0, &slots[1], (void *)0,",
                    "    &slots[2], (void *)(RK_LIMIT - 24), &slots[3], NULL,",
                    "};",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    unsigned int *p = table[(x + i) & 7u];",
                    "    unsigned int seen = ((p == 0) << 0)",
                    "        | ((p != (void *)0) << 1)",
                    "        | (!p << 2)",
                    "        | (((void *)(1 - 1) == p) << 3)",
                    "        | ((NULL != p) << 4)",
                    "        | ((p == 0L) << 5);",
                    "    if (p) {",
                    "        *p += i;",
                    "    }",
                    "    return x * 7u + seen + (p ? *p : 0u);",
                    "}",
                ]),
                move |x, i| {
                    let at = x.wrapping_add(i) & 7;
                    let (seen, value) = if at & 1 == 1 {
                        (0b10_1101, 0)
                    } else {
                        let slot = &mut slots[(at / 2) as usize];
                        *slot = slot.wrapping_add(i);
                        (0b1_0010, *slot)
                    };
                    x.wrapping_mul(7).wrapping_add(seen).wrapping_add(value)
                },
            )
        }
        _ => {
            let mut released = 0u32;
            Shape::new(
                "include/linux/fs.h",
                lines(&[
                    "#define RK_LIMIT 24",
                    "",
                    "struct rk_ops {",
                    "    unsigned int (*open)(unsigned int);",
                    "    unsigned int (*read)(unsigned int, unsigned int);",
                    "    void (*release)(unsigned int);",
                    "};",
                    "",
                    "static unsigned int released;",
                    "",
                    "static unsigned int open_twice(unsigned int x) {",
                    "    return x * 2u + 1u;",
                    "}",
                    "",
                    "static unsigned int read_mixed(unsigned int x, unsigned int i) {",
                    "    return x ^ (i * 0x9e3779b9u);",
                    "}",
                    "",
                    "static void release_count(unsigned int x) {",
                    "    released += x & 15u;",
                    "}",
                    "",
                    "static const struct rk_ops full = { open_twice, read_mixed, release_count };",
                    "static const struct rk_ops empty = { (void *)0, 0, (void *)(RK_LIMIT - 24) };",
                    "static const struct rk_ops partial = { .read = read_mixed };",
                    "static const struct rk_ops *const all[3] = { &full, &empty, &partial };",
                    "",
                    "static unsigned int deref_or(const unsigned int *p, unsigned int otherwise) {",
                    "    return p ? *p : otherwise;",
                    "}",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    const struct rk_ops *ops = all[(x + i) % 3u];",
                    "    unsigned int v = x;",
                    "    if (ops->open) {",
                    "        v = ops->open(v);",
                    "    }",
                    "    if (ops->read != (void *)0) {",
                    "        v = ops->read(v, i);",
                    "    }",
                    "    if (ops->release != 0) {",
                    "        ops->release(v);",
                    "    }",
                    "    _Bool opens = ops->open;",
                    "    unsigned int *none = (void *)(1 - 1);",
                    "    return v + opens * 3u + released + (unsigned int)(__UINTPTR_TYPE__)none",
                    "        + deref_or(0, 5u) + deref_or((void *)0, i) + deref_or(&released, 0u);",
                    "}",
                ]),
                move |x, i| {
                    let read = |v: u32| v ^ i.wrapping_mul(0x9e37_79b9);
                    let (v, opens) = match x.wrapping_add(i) % 3 {
                        0 => {
                            let v = read(x.wrapping_mul(2).wrapping_add(1));
                            released = released.wrapping_add(v & 15);
                            (v, 3)
                        }
                        1 => (x, 0),
                        _ => (read(x), 0),
                    };
                    v.wrapping_add(opens)
                        .wrapping_add(released)
                        .wrapping_add(5)
                        .wrapping_add(i)
                        .wrapping_add(released)
                },
            )
        }
    }
}

/// `GENMASK` for 32 bits, the way include/linux/bits.h builds it.
fn genmask(h: u32, l: u32) -> u32 {
    (!0u32).wrapping_sub(1 << l).wrapping_add(1) & (!0u32 >> (31 - h))
}

/// An ioctl number, the way include/uapi/asm-generic/ioctl.h builds it.
fn ioc(dir: u32, kind: u32, nr: u32, size: u32) -> u32 {
    (dir << 30) | (size << 16) | (kind << 8) | nr
}

fn const_ice(shape: &str) -> Shape {
    match shape {
        "array-size" => {
            let mut words = [0u32; 12];
            let mut copy = [0u8; 20];
            let mut head = [0u8; 10];
            let mut lens = [0u32; 4];
            Shape::new(
                "include/linux/array_size.h",
                lines(&[
                    "#define __same_type(a, b) __builtin_types_compatible_p(__typeof__(a), __typeof__(b))",
                    "#define __is_array(a) (!__same_type((a), &(a)[0]))",
                    "#define __BUILD_BUG_ON_ZERO_MSG(e, msg) ((int)sizeof(struct { _Static_assert(!(e), msg); }))",
                    "#define __must_be_array(a) __BUILD_BUG_ON_ZERO_MSG(!__is_array(a), \"must be array\")",
                    "#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]) + __must_be_array(arr))",
                    "",
                    "struct rk_hdr {",
                    "    unsigned int magic;",
                    "    unsigned char tag[6];",
                    "    unsigned char body[10];",
                    "};",
                    "",
                    "static unsigned int words[12];",
                    "static unsigned char copy[sizeof(struct rk_hdr)];",
                    "static unsigned char head[__builtin_offsetof(struct rk_hdr, body)];",
                    "static unsigned int lens[ARRAY_SIZE(words) / 3];",
                    "static const char names[][8] = { \"ext4\", \"xfs\", \"btrfs\" };",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    words[i % ARRAY_SIZE(words)] += x;",
                    "    copy[(x + i) % ARRAY_SIZE(copy)] ^= (unsigned char)i;",
                    "    head[x % ARRAY_SIZE(head)] += 1u;",
                    "    lens[i % ARRAY_SIZE(lens)] = x >> 3;",
                    "    const char *name = names[(x ^ i) % ARRAY_SIZE(names)];",
                    "    return x * 3u + words[(x >> 4) % ARRAY_SIZE(words)] + copy[i % ARRAY_SIZE(copy)]",
                    "        + head[i % ARRAY_SIZE(head)] + lens[x % ARRAY_SIZE(lens)]",
                    "        + (unsigned char)name[1] + ARRAY_SIZE(names);",
                    "}",
                ]),
                move |x, i| {
                    const SECOND: [u8; 3] = *b"xft";
                    let at = (i % 12) as usize;
                    words[at] = words[at].wrapping_add(x);
                    copy[(x.wrapping_add(i) % 20) as usize] ^= (i & 0xff) as u8;
                    let at = (x % 10) as usize;
                    head[at] = head[at].wrapping_add(1);
                    lens[(i % 4) as usize] = x >> 3;
                    let name = SECOND[((x ^ i) % 3) as usize];
                    x.wrapping_mul(3)
                        .wrapping_add(words[((x >> 4) % 12) as usize])
                        .wrapping_add(u32::from(copy[(i % 20) as usize]))
                        .wrapping_add(u32::from(head[(i % 10) as usize]))
                        .wrapping_add(lens[(x % 4) as usize])
                        .wrapping_add(u32::from(name))
                        .wrapping_add(3)
                },
            )
        }
        "enum" => {
            let mut ring = [0u32; 8];
            Shape::new(
                "include/linux/stddef.h",
                joined(&[
                    &[
                        "#define sizeof_field(TYPE, MEMBER) sizeof((((TYPE *)0)->MEMBER))",
                        "#define offsetofend(TYPE, MEMBER) \\",
                        "    (__builtin_offsetof(TYPE, MEMBER) + sizeof_field(TYPE, MEMBER))",
                        "",
                    ],
                    REQ,
                    &[
                        "enum rk_layout {",
                        "    RK_KEY_AT = __builtin_offsetof(struct rk_req, key),",
                        "    RK_KEY_END = offsetofend(struct rk_req, key),",
                        "    RK_DATA_AT = __builtin_offsetof(struct rk_req, data[4]),",
                        "    RK_REQ_SIZE = sizeof(struct rk_req),",
                        "    RK_SLOTS = 256 / sizeof(struct rk_req),",
                        "    RK_LAST = RK_SLOTS - 1,",
                        "    RK_DATA_WORDS = sizeof_field(struct rk_req, data) / sizeof(unsigned int),",
                        "    RK_MASK = (1 << RK_LAST) - 1,",
                        "};",
                        "",
                        "static unsigned int ring[RK_SLOTS];",
                        "",
                        "__attribute__((noinline))",
                        "static unsigned int step(unsigned int x, unsigned int i) {",
                        "    unsigned int slot = (x + i) & RK_LAST;",
                        "    ring[slot] += x >> RK_DATA_WORDS;",
                        "    return (x >> (i & RK_LAST)) + slot * RK_REQ_SIZE + RK_KEY_AT * i",
                        "        + (RK_DATA_AT ^ RK_KEY_END) + (x & RK_MASK) + ring[(x ^ i) & RK_LAST];",
                        "}",
                    ],
                ]),
                move |x, i| {
                    let slot = x.wrapping_add(i) & 7;
                    ring[slot as usize] = ring[slot as usize].wrapping_add(x >> 4);
                    (x >> (i & 7))
                        .wrapping_add(slot * 32)
                        .wrapping_add(i * 8)
                        .wrapping_add(20 ^ 16)
                        .wrapping_add(x & 127)
                        .wrapping_add(ring[((x ^ i) & 7) as usize])
                },
            )
        }
        "case-label" => {
            let get = ioc(2, 0x72, 1, 32);
            let set = ioc(1, 0x72, 2, 32);
            let swap = ioc(3, 0x72, 3, 16);
            let reset = ioc(0, 0x72, 4, 0);
            let cmds = [get, set, swap, reset, ioc(0, 0x72, 1, 0), set, 16, 1];
            Shape::new(
                "include/uapi/asm-generic/ioctl.h",
                joined(&[
                    &[
                        "#define _IOC(dir, type, nr, size) \\",
                        "    (((dir) << 30) | ((size) << 16) | ((type) << 8) | (nr))",
                        "#define _IO(type, nr) _IOC(0u, (type), (nr), 0u)",
                        "#define _IOW(type, nr, t) _IOC(1u, (type), (nr), sizeof(t))",
                        "#define _IOR(type, nr, t) _IOC(2u, (type), (nr), sizeof(t))",
                        "#define _IOWR(type, nr, t) _IOC(3u, (type), (nr), sizeof(t))",
                        "",
                    ],
                    REQ,
                    &[
                        "#define RK_GET _IOR('r', 1, struct rk_req)",
                        "#define RK_SET _IOW('r', 2, struct rk_req)",
                        "#define RK_SWAP _IOWR('r', 3, unsigned int[4])",
                        "#define RK_RESET _IO('r', 4)",
                        "",
                        "static const unsigned int cmds[8] = {",
                        "    RK_GET, RK_SET, RK_SWAP, RK_RESET,",
                        "    _IO('r', 1), RK_SET, __builtin_offsetof(struct rk_req, data), 1u,",
                        "};",
                        "",
                        "__attribute__((noinline))",
                        "static unsigned int step(unsigned int x, unsigned int i) {",
                        "    switch (cmds[(x + i) & 7u]) {",
                        "    case RK_GET:",
                        "        return x + 0x10u;",
                        "    case RK_SET:",
                        "        return x * 3u + i;",
                        "    case RK_SWAP:",
                        "        return (x << 4) | (x >> 28);",
                        "    case RK_RESET:",
                        "        return x ^ i;",
                        "    case __builtin_offsetof(struct rk_req, data):",
                        "        return x ^ 0xffu;",
                        "    case 1 ... sizeof(unsigned int):",
                        "        return x + 77u;",
                        "    default:",
                        "        return x - 1u;",
                        "    }",
                        "}",
                    ],
                ]),
                move |x, i| {
                    let cmd = cmds[(x.wrapping_add(i) & 7) as usize];
                    if cmd == get {
                        x.wrapping_add(0x10)
                    } else if cmd == set {
                        x.wrapping_mul(3).wrapping_add(i)
                    } else if cmd == swap {
                        x.rotate_left(4)
                    } else if cmd == reset {
                        x ^ i
                    } else if cmd == 16 {
                        x ^ 0xff
                    } else if (1..=4).contains(&cmd) {
                        x.wrapping_add(77)
                    } else {
                        x.wrapping_sub(1)
                    }
                },
            )
        }
        "static-assert" => {
            let mut frames = [(0u32, 0u32, [0u8; 56]); 4];
            let mut head = 0u32;
            Shape::new(
                "include/linux/build_bug.h",
                lines(&[
                    "#define static_assert(expr, ...) __static_assert(expr, ##__VA_ARGS__, #expr)",
                    "#define __static_assert(expr, msg, ...) _Static_assert(expr, msg)",
                    "",
                    "struct rk_frame {",
                    "    unsigned int len;",
                    "    unsigned int csum;",
                    "    unsigned char data[56];",
                    "};",
                    "",
                    "static_assert(sizeof(struct rk_frame) == 64);",
                    "static_assert(__builtin_offsetof(struct rk_frame, data) == 2 * sizeof(unsigned int),",
                    "              \"data follows the header\");",
                    "",
                    "struct rk_ring {",
                    "    struct rk_frame frames[4];",
                    "    unsigned int head;",
                    "    _Static_assert(sizeof(struct rk_frame) % 16 == 0, \"a frame is whole lines\");",
                    "};",
                    "",
                    "static_assert((unsigned char)0x1ff == 0xff);",
                    "static_assert(sizeof(struct rk_ring) > 256 ? 1 : 0, \"the ring holds four frames\");",
                    "",
                    "static struct rk_ring ring;",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    static_assert(__builtin_offsetof(struct rk_ring, head) == 4 * sizeof(struct rk_frame));",
                    "    struct rk_frame *f = &ring.frames[i & 3u];",
                    "    f->len += i;",
                    "    f->csum ^= x;",
                    "    f->data[(x + i) % 56u] += (unsigned char)x;",
                    "    return x * 9u + f->len + f->csum + f->data[x % 56u] + ring.head++;",
                    "}",
                ]),
                move |x, i| {
                    let (len, csum, data) = &mut frames[(i & 3) as usize];
                    *len = len.wrapping_add(i);
                    *csum ^= x;
                    let at = (x.wrapping_add(i) % 56) as usize;
                    data[at] = data[at].wrapping_add((x & 0xff) as u8);
                    let was = head;
                    head = head.wrapping_add(1);
                    x.wrapping_mul(9)
                        .wrapping_add(*len)
                        .wrapping_add(*csum)
                        .wrapping_add(u32::from(data[(x % 56) as usize]))
                        .wrapping_add(was)
                },
            )
        }
        "build-bug-on-zero" => {
            let field = genmask(11, 4);
            let modes = [3u32, 0x5a, 0xff, 0].map(|mode| (mode << 4) & field);
            let mut regs = [0u32; 8];
            Shape::new(
                "include/linux/bits.h",
                joined(&[
                    IS_CONSTEXPR,
                    &[
                        "#define BUILD_BUG_ON_ZERO(e) ((int)(sizeof(struct { int:(-!!(e)); })))",
                        "#define GENMASK_INPUT_CHECK(h, l) BUILD_BUG_ON_ZERO(__builtin_choose_expr( \\",
                        "    __is_constexpr((l) > (h)), (l) > (h), 0))",
                        "#define __GENMASK_U32(h, l) (((~0u) - (1u << (l)) + 1u) & (~0u >> (31 - (h))))",
                        "#define GENMASK_U32(h, l) (GENMASK_INPUT_CHECK(h, l) + __GENMASK_U32(h, l))",
                        "#define __bf_shf(x) (__builtin_ffsll(x) - 1)",
                        "#define FIELD_PREP_CONST(mask, val) \\",
                        "    (BUILD_BUG_ON_ZERO((mask) == 0) \\",
                        "     + BUILD_BUG_ON_ZERO(~((mask) >> __bf_shf(mask)) & (val)) \\",
                        "     + (((__typeof__(mask))(val) << __bf_shf(mask)) & (mask)))",
                        "",
                        "#define RK_FIELD GENMASK_U32(11, 4)",
                        "",
                        "static unsigned int regs[8 + BUILD_BUG_ON_ZERO(8 & 7)];",
                        "static const unsigned int modes[4] = {",
                        "    FIELD_PREP_CONST(RK_FIELD, 3u), FIELD_PREP_CONST(RK_FIELD, 0x5au),",
                        "    FIELD_PREP_CONST(RK_FIELD, 0xffu), FIELD_PREP_CONST(RK_FIELD, 0u),",
                        "};",
                        "",
                        "__attribute__((noinline))",
                        "static unsigned int step(unsigned int x, unsigned int i) {",
                        "    unsigned int reg = regs[i & 7u];",
                        "    reg = (reg & ~RK_FIELD) | modes[x & 3u] | (x & GENMASK_U32(31, 24));",
                        "    regs[i & 7u] = reg;",
                        "    unsigned int low = x & GENMASK_U32(i & 15u, 0);",
                        "    return x * 5u + ((reg & RK_FIELD) >> 4) + low + regs[(x >> 5) & 7u];",
                        "}",
                    ],
                ]),
                move |x, i| {
                    let at = (i & 7) as usize;
                    regs[at] =
                        (regs[at] & !field) | modes[(x & 3) as usize] | (x & genmask(31, 24));
                    let low = x & genmask(i & 15, 0);
                    x.wrapping_mul(5)
                        .wrapping_add((regs[at] & field) >> 4)
                        .wrapping_add(low)
                        .wrapping_add(regs[((x >> 5) & 7) as usize])
                },
            )
        }
        "constant-p" => {
            let mut cache = [0u32; 6];
            let (found, found_value) = packed(
                "rk_found = ",
                "",
                &[
                    ("__builtin_constant_p(RK_LIMIT * 3)", true),
                    ("__builtin_constant_p(sizeof(struct rk_req))", true),
                    ("__builtin_constant_p(__builtin_offsetof(struct rk_req, data))", true),
                    ("__builtin_constant_p((unsigned char)0x1ff)", true),
                    ("__builtin_constant_p(1 ? 5 : 6)", true),
                    ("statically_true(RK_LIMIT > 3)", true),
                    ("statically_true(RK_LIMIT < 3)", false),
                    ("__builtin_constant_p(seed_in)", false),
                ],
            );
            let (seen, seen_value) = packed(
                "unsigned int seen = ",
                ";",
                &[
                    ("__builtin_constant_p(seed_in)", false),
                    ("__builtin_constant_p(RK_LIMIT + 1)", true),
                    ("__builtin_constant_p(sizeof(struct rk_req) / 4)", true),
                    ("statically_true(sizeof(struct rk_req) == 32)", true),
                ],
            );
            let mut top = lines(&[
                "#define statically_true(x) (__builtin_constant_p(x) && (x))",
                "#define RK_LIMIT 24",
                "",
            ]);
            top.extend(lines(REQ));
            top.extend(lines(&[
                "static unsigned int cache[__builtin_constant_p(sizeof(struct rk_req)) ? 6 : 1];",
                "",
                "enum {",
            ]));
            top.extend(found);
            top.extend(lines(&[
                "};",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
            ]));
            top.extend(seen);
            top.extend(lines(&[
                "    cache[i % 6u] += x;",
                "    switch (x & 3u) {",
                "    case __builtin_constant_p(RK_LIMIT) ? 3 : 0:",
                "        return x + seen + rk_found + cache[(x >> 2) % 6u];",
                "    default:",
                "        return (x ^ i) + seen * 7u + rk_found + cache[i % 6u];",
                "    }",
                "}",
            ]));
            Shape::new("include/linux/compiler.h", top, move |x, i| {
                let at = (i % 6) as usize;
                cache[at] = cache[at].wrapping_add(x);
                if x & 3 == 3 {
                    x.wrapping_add(seen_value)
                        .wrapping_add(found_value)
                        .wrapping_add(cache[((x >> 2) % 6) as usize])
                } else {
                    (x ^ i)
                        .wrapping_add(seen_value * 7)
                        .wrapping_add(found_value)
                        .wrapping_add(cache[at])
                }
            })
        }
        "const-local" => {
            let mut window = [0u8; 12];
            Shape::new(
                "scripts/min-tool-version.sh",
                lines(&[
                    "static unsigned char window[12];",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    const unsigned int n = 4u;",
                    "    const unsigned int m = n * 2u + 1u;",
                    "    _Static_assert(n == 4u, \"n folds\");",
                    "    _Static_assert(m == 9u, \"m folds\");",
                    "    enum { RK_N = n, RK_M = m << 1, RK_SEEN = __builtin_constant_p(m) ? 3 : 1 };",
                    "    window[(x ^ i) % 12u] += (unsigned char)i;",
                    "    switch ((x + i) & 15u) {",
                    "    case n:",
                    "        return x + RK_M;",
                    "    case m:",
                    "        return x ^ RK_N;",
                    "    case RK_M - 10u:",
                    "        return x * 3u + RK_SEEN;",
                    "    default:",
                    "        return x + i + window[x % 12u];",
                    "    }",
                    "}",
                ]),
                move |x, i| {
                    let at = ((x ^ i) % 12) as usize;
                    window[at] = window[at].wrapping_add((i & 0xff) as u8);
                    match x.wrapping_add(i) & 15 {
                        4 => x.wrapping_add(18),
                        9 => x ^ 4,
                        8 => x.wrapping_mul(3).wrapping_add(3),
                        _ => x.wrapping_add(i).wrapping_add(u32::from(window[(x % 12) as usize])),
                    }
                },
            )
        }
        _ => {
            let mut table = [0u32; 4];
            Shape::new(
                "include/linux/limits.h",
                joined(&[
                    &[
                        "#define U8_MAX ((unsigned char)~0u)",
                        "#define S8_MAX ((signed char)(U8_MAX >> 1))",
                        "#define S8_MIN ((signed char)(-S8_MAX - 1))",
                        "#define U16_MAX ((unsigned short)~0u)",
                        "#define S16_MAX ((short)(U16_MAX >> 1))",
                        "#define S16_MIN ((short)(-S16_MAX - 1))",
                        "",
                    ],
                    REQ,
                    &[
                        "enum {",
                        "    RK_TRUNC = (int)6.75,",
                        "    RK_BOOL = (_Bool)42,",
                        "    RK_PICK = sizeof(struct rk_req) > 16 ? 3 : 5,",
                        "    RK_NESTED = (1 ? (unsigned char)300 : 7) + (0 ? 1 : RK_TRUNC),",
                        "};",
                        "",
                        "_Static_assert(U8_MAX == 255 && S8_MIN == -128, \"the byte limits\");",
                        "_Static_assert(U16_MAX == 65535 && S16_MIN == -32768, \"the short limits\");",
                        "",
                        "static unsigned int table[RK_PICK + RK_BOOL];",
                        "",
                        "__attribute__((noinline))",
                        "static unsigned int step(unsigned int x, unsigned int i) {",
                        "    table[x % (RK_PICK + RK_BOOL)] += i;",
                        "    switch ((x ^ i) & 7u) {",
                        "    case RK_BOOL:",
                        "        x += U8_MAX;",
                        "        break;",
                        "    case RK_PICK:",
                        "        x ^= U16_MAX;",
                        "        break;",
                        "    case RK_TRUNC:",
                        "        x -= S8_MIN;",
                        "        break;",
                        "    case (unsigned char)0x105:",
                        "        x += (unsigned int)S16_MIN;",
                        "        break;",
                        "    default:",
                        "        x += RK_NESTED;",
                        "        break;",
                        "    }",
                        "    return x * 3u + table[i % (RK_PICK + RK_BOOL)] + i;",
                        "}",
                    ],
                ]),
                move |x, i| {
                    let at = (x % 4) as usize;
                    table[at] = table[at].wrapping_add(i);
                    let x = match (x ^ i) & 7 {
                        1 => x.wrapping_add(255),
                        3 => x ^ 0xffff,
                        6 => x.wrapping_add(128),
                        5 => x.wrapping_sub(32768),
                        _ => x.wrapping_add(50),
                    };
                    x.wrapping_mul(3).wrapping_add(table[(i % 4) as usize]).wrapping_add(i)
                },
            )
        }
    }
}

/// The four functions the initcall cases register, as C and as Rust.
const INITCALLS: &[&str] = &[
    "static unsigned int init_clock(unsigned int x) {",
    "    return x * 3u + 1u;",
    "}",
    "",
    "static unsigned int init_irq(unsigned int x) {",
    "    return x ^ 0x5bd1e995u;",
    "}",
    "",
    "static unsigned int init_mm(unsigned int x) {",
    "    return (x << 5) | (x >> 27);",
    "}",
    "",
    "static unsigned int init_net(unsigned int x) {",
    "    return x + 0x9e3779b9u;",
    "}",
    "",
];

/// One of them, the way a level of the initcall table holds it.
type Initcall = fn(u32) -> u32;

fn init_clock(x: u32) -> u32 {
    x.wrapping_mul(3).wrapping_add(1)
}

fn init_irq(x: u32) -> u32 {
    x ^ 0x5bd1_e995
}

fn init_mm(x: u32) -> u32 {
    x.rotate_left(5)
}

fn init_net(x: u32) -> u32 {
    x.wrapping_add(0x9e37_79b9)
}

fn section_attr(shape: &str) -> Shape {
    match shape {
        "initcall-table" => Shape::new(
            "include/linux/init.h",
            joined(&[
                &[
                    "typedef unsigned int (*initcall_t)(unsigned int);",
                    "",
                    "#define __define_initcall(fn, id) \\",
                    "    static initcall_t __initcall_##fn##id \\",
                    "        __attribute__((used, section(\"rk_initcall\"))) = fn",
                    "#define device_initcall(fn) __define_initcall(fn, 6)",
                    "",
                    "extern initcall_t __start_rk_initcall[];",
                    "extern initcall_t __stop_rk_initcall[];",
                    "",
                ],
                INITCALLS,
                &[
                    "device_initcall(init_clock);",
                    "device_initcall(init_irq);",
                    "device_initcall(init_mm);",
                    "device_initcall(init_net);",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    unsigned int sum = 0, count = 0;",
                    "    for (initcall_t *fn = __start_rk_initcall; fn < __stop_rk_initcall; fn++) {",
                    "        sum += (*fn)(x + i);",
                    "        count++;",
                    "    }",
                    "    return (x ^ sum) + count * i;",
                    "}",
                ],
            ]),
            |x, i| {
                let y = x.wrapping_add(i);
                let sum = [init_clock, init_irq, init_mm, init_net]
                    .iter()
                    .fold(0u32, |sum, call| sum.wrapping_add(call(y)));
                (x ^ sum).wrapping_add(4 * i)
            },
        ),
        "initcall-levels" => Shape::new(
            "init/main.c",
            joined(&[
                &[
                    "typedef unsigned int (*initcall_t)(unsigned int);",
                    "",
                    "#define __define_initcall(fn, level) \\",
                    "    static initcall_t __initcall_##fn##level \\",
                    "        __attribute__((used, section(\"rk_initcall\" #level))) = fn",
                    "#define pure_initcall(fn) __define_initcall(fn, 0)",
                    "#define core_initcall(fn) __define_initcall(fn, 1)",
                    "#define arch_initcall(fn) __define_initcall(fn, 3)",
                    "#define device_initcall(fn) __define_initcall(fn, 6)",
                    "",
                    "extern initcall_t __start_rk_initcall0[], __stop_rk_initcall0[];",
                    "extern initcall_t __start_rk_initcall1[], __stop_rk_initcall1[];",
                    "extern initcall_t __start_rk_initcall3[], __stop_rk_initcall3[];",
                    "extern initcall_t __start_rk_initcall6[], __stop_rk_initcall6[];",
                    "",
                    "static initcall_t *const levels[4][2] = {",
                    "    { __start_rk_initcall0, __stop_rk_initcall0 },",
                    "    { __start_rk_initcall1, __stop_rk_initcall1 },",
                    "    { __start_rk_initcall3, __stop_rk_initcall3 },",
                    "    { __start_rk_initcall6, __stop_rk_initcall6 },",
                    "};",
                    "",
                ],
                INITCALLS,
                &[
                    "pure_initcall(init_mm);",
                    "core_initcall(init_clock);",
                    "core_initcall(init_net);",
                    "arch_initcall(init_irq);",
                    "device_initcall(init_clock);",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    for (unsigned int level = 0; level < 4u; level++) {",
                    "        unsigned int sum = 0;",
                    "        for (initcall_t *fn = levels[level][0]; fn < levels[level][1]; fn++) {",
                    "            sum += (*fn)(x + level);",
                    "        }",
                    "        x = x * 31u + sum;",
                    "    }",
                    "    return x + i;",
                    "}",
                ],
            ]),
            |mut x, i| {
                let levels: [&[Initcall]; 4] =
                    [&[init_mm], &[init_clock, init_net], &[init_irq], &[init_clock]];
                for (level, calls) in (0u32..).zip(levels) {
                    let y = x.wrapping_add(level);
                    let sum = calls.iter().fold(0u32, |sum, call| sum.wrapping_add(call(y)));
                    x = x.wrapping_mul(31).wrapping_add(sum);
                }
                x.wrapping_add(i)
            },
        ),
        "param-table" => Shape::new(
            "include/linux/moduleparam.h",
            lines(&[
                "struct kernel_param {",
                "    const char *name;",
                "    unsigned int key;",
                "    unsigned int (*set)(unsigned int, unsigned int);",
                "};",
                "",
                "#define __module_param_call(name, key, set) \\",
                "    static const struct kernel_param __param_##name \\",
                "        __attribute__((used, section(\"rk_param\"), \\",
                "                       aligned(__alignof__(struct kernel_param)))) \\",
                "        = { #name, key, set }",
                "",
                "extern const struct kernel_param __start_rk_param[];",
                "extern const struct kernel_param __stop_rk_param[];",
                "",
                "static unsigned int set_add(unsigned int x, unsigned int i) {",
                "    return x + i * 3u;",
                "}",
                "",
                "static unsigned int set_xor(unsigned int x, unsigned int i) {",
                "    return x ^ (i << 7);",
                "}",
                "",
                "static unsigned int set_mul(unsigned int x, unsigned int i) {",
                "    return x * 5u + i;",
                "}",
                "",
                "__module_param_call(debug, 1u, set_add);",
                "__module_param_call(quiet, 3u, set_xor);",
                "__module_param_call(loglevel, 4u, set_mul);",
                "__module_param_call(panic, 6u, set_add);",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int key = (x ^ i) & 7u;",
                "    unsigned int count = 0;",
                "    for (const struct kernel_param *p = __start_rk_param; p < __stop_rk_param; p++) {",
                "        count++;",
                "        if (p->key == key) {",
                "            x = p->set(x, i) + (unsigned char)p->name[0];",
                "        }",
                "    }",
                "    return x + count * i;",
                "}",
            ]),
            |x, i| {
                let add = |x: u32| x.wrapping_add(i.wrapping_mul(3));
                let x = match (x ^ i) & 7 {
                    1 => add(x).wrapping_add(u32::from(b'd')),
                    3 => (x ^ (i << 7)).wrapping_add(u32::from(b'q')),
                    4 => x.wrapping_mul(5).wrapping_add(i).wrapping_add(u32::from(b'l')),
                    6 => add(x).wrapping_add(u32::from(b'p')),
                    _ => x,
                };
                x.wrapping_add(4 * i)
            },
        ),
        "aligned-entries" => Shape::new(
            "include/linux/serial_core.h",
            lines(&[
                "struct earlycon_id {",
                "    char name[16];",
                "    char compatible[16];",
                "    unsigned int flags;",
                "};",
                "",
                "#define EARLYCON_DECLARE(_name, _flags) \\",
                "    static const struct earlycon_id __earlycon_##_name \\",
                "        __attribute__((used, section(\"rk_earlycon\"), \\",
                "                       aligned(__alignof__(struct earlycon_id)))) \\",
                "        = { .name = #_name, .compatible = \"rk,\" #_name, .flags = _flags }",
                "",
                "extern const struct earlycon_id __start_rk_earlycon[];",
                "extern const struct earlycon_id __stop_rk_earlycon[];",
                "",
                "EARLYCON_DECLARE(uart8250, 1u);",
                "EARLYCON_DECLARE(pl011, 2u);",
                "EARLYCON_DECLARE(ns16550, 4u);",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int h = 0, count = 0;",
                "    for (const struct earlycon_id *e = __start_rk_earlycon; e < __stop_rk_earlycon; e++) {",
                "        h += e->flags * (unsigned char)e->name[1] + (unsigned char)e->compatible[3];",
                "        count++;",
                "    }",
                "    return (x ^ h) + count * (i + 1u);",
                "}",
            ]),
            |x, i| {
                let h = [("uart8250", 1u32), ("pl011", 2), ("ns16550", 4)].iter().fold(
                    0u32,
                    |h, &(name, flags)| {
                        let bytes = name.as_bytes();
                        h.wrapping_add(flags * u32::from(bytes[1]))
                            .wrapping_add(u32::from(bytes[0]))
                    },
                );
                (x ^ h).wrapping_add(3 * (i + 1))
            },
        ),
        "init-text" => Shape::new(
            "include/linux/init.h",
            lines(&[
                "#define __init __attribute__((section(\"rk_init_text\")))",
                "",
                "extern const char __start_rk_init_text[];",
                "extern const char __stop_rk_init_text[];",
                "",
                "static unsigned int __init setup_once(unsigned int x) {",
                "    return x * 7u + 3u;",
                "}",
                "",
                "static unsigned int __init setup_twice(unsigned int x) {",
                "    return setup_once(x) ^ 0x1234u;",
                "}",
                "",
                "static unsigned int inside(__UINTPTR_TYPE__ at) {",
                "    return at >= (__UINTPTR_TYPE__)__start_rk_init_text",
                "        && at < (__UINTPTR_TYPE__)__stop_rk_init_text;",
                "}",
                "",
                "__attribute__((noinline))",
                "static unsigned int step(unsigned int x, unsigned int i) {",
                "    unsigned int in = inside((__UINTPTR_TYPE__)setup_once)",
                "        + inside((__UINTPTR_TYPE__)setup_twice) * 2u",
                "        + inside((__UINTPTR_TYPE__)step) * 4u;",
                "    return setup_twice(x + i) + in;",
                "}",
            ]),
            |x, i| (x.wrapping_add(i).wrapping_mul(7).wrapping_add(3) ^ 0x1234).wrapping_add(3),
        ),
        _ => {
            let mut mix = 0x9e37_79b9u32;
            let mut counters = [0u32; 16];
            let mut scratch = vec![0u8; 4096];
            const WEIGHTS: [u32; 8] = [3, 5, 7, 11, 13, 17, 19, 23];
            Shape::new(
                "include/linux/cache.h",
                lines(&[
                    "#define __read_mostly __attribute__((section(\".data..read_mostly\")))",
                    "#define __page_aligned_bss __attribute__((section(\".bss..page_aligned\"), aligned(4096)))",
                    "",
                    "static unsigned int mix __read_mostly = 0x9e3779b9u;",
                    "static unsigned char scratch[4096] __page_aligned_bss;",
                    "static const unsigned int weights[8] __attribute__((section(\"rk_rodata\"))) = {",
                    "    3u, 5u, 7u, 11u, 13u, 17u, 19u, 23u,",
                    "};",
                    "static unsigned int counters[16] __attribute__((section(\"rk_counters\")));",
                    "",
                    "extern const char __start_rk_rodata[];",
                    "extern const char __stop_rk_rodata[];",
                    "extern char __start_rk_counters[];",
                    "extern char __stop_rk_counters[];",
                    "",
                    "__attribute__((noinline))",
                    "static unsigned int step(unsigned int x, unsigned int i) {",
                    "    mix += i & 1u;",
                    "    counters[(x + i) & 15u] += 1u;",
                    "    scratch[(x * 13u) & 4095u] ^= (unsigned char)i;",
                    "    unsigned int span = (unsigned int)(__stop_rk_rodata - __start_rk_rodata)",
                    "        + (unsigned int)(__stop_rk_counters - __start_rk_counters);",
                    "    unsigned int offset = (unsigned int)((__UINTPTR_TYPE__)scratch & 4095u);",
                    "    return (x ^ mix) + weights[(x ^ i) & 7u] + counters[i & 15u]",
                    "        + scratch[(x * 13u) & 4095u] + span + offset;",
                    "}",
                ]),
                move |x, i| {
                    mix = mix.wrapping_add(i & 1);
                    let at = (x.wrapping_add(i) & 15) as usize;
                    counters[at] = counters[at].wrapping_add(1);
                    let byte = (x.wrapping_mul(13) & 4095) as usize;
                    scratch[byte] ^= (i & 0xff) as u8;
                    (x ^ mix)
                        .wrapping_add(WEIGHTS[((x ^ i) & 7) as usize])
                        .wrapping_add(counters[(i & 15) as usize])
                        .wrapping_add(u32::from(scratch[byte]))
                        .wrapping_add(32 + 64)
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_shape_is_there_with_its_tags_and_flags() {
        let manifest = generate(&Options::all()).unwrap();
        for (facet, count) in
            [(Facet::NullPointerConstant, 6), (Facet::ConstIce, 8), (Facet::SectionAttr, 6)]
        {
            let cases: Vec<_> = manifest.cases.iter().filter(|case| case.facet == facet).collect();
            assert_eq!(cases.len(), count, "{facet:?}");
            for case in cases {
                assert!(case.has_tag("provenance:linux"), "{}", case.id);
                assert_eq!(case.has_tag("elf"), facet == Facet::SectionAttr, "{}", case.id);
                assert!(!case.has_tag("x86-64"), "{}", case.id);
                let pinned = case.id.contains(".const-local.");
                assert_eq!(case.flags, if pinned { vec!["-O2".to_owned()] } else { vec![] });
            }
        }
    }

    /// The two facets about constant expressions are let off the ban on sizeof, so this is what
    /// keeps them honest. A struct whose size or layout is asked for is built from unsigned int,
    /// unsigned char and other structs like it, so it has the same size on every target, and the
    /// only thing printed is the running total.
    #[test]
    fn every_size_taken_is_the_same_on_every_target() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest
            .cases
            .iter()
            .filter(|case| matches!(case.facet, Facet::NullPointerConstant | Facet::ConstIce))
        {
            assert_eq!(case.source.matches("printf(\"").count(), 1, "{}", case.id);
            for (at, _) in case.source.match_indices("\nstruct rk_") {
                let head = &case.source[at + 1..];
                let name = &head[..head.find(" {").unwrap()];
                let measured =
                    [format!("sizeof({name})"), format!("({name}, "), format!("{name})")]
                        .iter()
                        .any(|use_| case.source.contains(use_.as_str()));
                if !measured {
                    continue;
                }
                let body = &head[head.find('{').unwrap() + 1..head.find("};").unwrap()];
                let members = body.lines().map(str::trim);
                for member in
                    members.filter(|line| !line.is_empty() && !line.starts_with("_Static"))
                {
                    assert!(
                        ["unsigned int ", "unsigned char ", "struct rk_"]
                            .iter()
                            .any(|kind| member.starts_with(kind)),
                        "{}: {member}",
                        case.id
                    );
                }
            }
        }
    }
}
