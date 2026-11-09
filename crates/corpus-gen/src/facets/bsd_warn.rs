//! The FreeBSD build under `-Werror`, and the kernel printf of FreeBSD.
//!
//! `werror-shapes` is the units that failed the FreeBSD world build with rucc and not with GCC, in the warning parity run of tamnd/rucc-bsd.
//! The run built each unit with the build's own `-Werror` warning line, so any diagnostic that GCC does not give stops the build.
//! It found no warning that rucc gives and GCC does not, and a few errors.
//! Each case here is the shape of one of them.
//! The FreeBSD headers give `qsort_r`, `basename` and `dirname` an old version with `.symver`, and the unit only declares the old name (E0697).
//! `__weak_reference` makes a weak `.equ` to a name in another unit (E0653).
//! `setcontext.c` gives its function a default version with `.symver`, and the world builds it with `-g` (E0652).
//! The `AFTER` chain of `wlanstat.c` nests more than 256 brackets deep (E0402).
//! The 32-bit compat build passes an empty `--sysroot=` (E0341).
//! Each case is built with the GCC warning line of a world unit at `WARNS=6`, from `share/mk/bsd.sys.mk`.
//!
//! `kprintf-format` is the kernel printf.
//! The cases link a copy of `kvprintf` from `sys/kern/subr_prf.c` and check what `%b`, `%D`, `%r` and `%y` write, with their flags, widths and lengths.
//! The kernel declares its printf functions with `__printflike`, and `-D__printf__=__freebsd_kprintf__` turns that into the format of the FreeBSD GCC.
//! GCC on Linux does not know that format and warns about it, so the cases give the declarations the format only when `__FreeBSD__` is defined.
//! The warnings of the format itself are checked against the FreeBSD GCC by the `kprintf-*` probes of bsd-probes in rucc-compat.
//! No call gives `%b` a length modifier, because the FreeBSD GCC 14 and 15 crash on one.
//! Each case is built with the kernel's warning line from `sys/conf/kern.mk`, and `-Werror` from `kern.pre.mk`.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, Unit};

/// The FreeBSD release the cases come from.
const FREEBSD: &str = "provenance:freebsd";

/// The GCC warning line of a FreeBSD world unit at `WARNS=6`, from `share/mk/bsd.sys.mk`.
const WORLD: &[&str] = &[
    "-Wsystem-headers",
    "-Werror",
    "-Wall",
    "-Wno-format-y2k",
    "-W",
    "-Wno-unused-parameter",
    "-Wmissing-prototypes",
    "-Wpointer-arith",
    "-Wreturn-type",
    "-Wcast-qual",
    "-Wwrite-strings",
    "-Wswitch",
    "-Wshadow",
    "-Wunused-parameter",
    "-Wcast-align",
    "-Wchar-subscripts",
    "-Wnested-externs",
    "-Wold-style-definition",
    "-Wno-pointer-sign",
    "-Wdate-time",
    "-Wformat=2",
    "-Wno-format-extra-args",
    "-Wno-return-type",
    "-Wno-format-zero-length",
];

/// The kernel's warning line from `sys/conf/kern.mk`, with `-Werror` from `sys/conf/kern.pre.mk`.
const KERNEL: &[&str] = &[
    "-Werror",
    "-Wall",
    "-Wstrict-prototypes",
    "-Wmissing-prototypes",
    "-Wpointer-arith",
    "-Wcast-qual",
    "-Wundef",
    "-Wno-pointer-sign",
    "-Wmissing-include-dirs",
    "-fdiagnostics-show-option",
    "-Wno-unknown-pragmas",
    "-Wswitch",
    "-Wno-format-zero-length",
];

/// The shapes of `werror-shapes`.
const WERROR: &[&str] = &[
    "sym-compat",
    "sym-compat-generic",
    "weak-reference-units",
    "sym-default-debug",
    "after-chain",
    "empty-sysroot",
];

/// The shapes of `kprintf-format`.
const KPRINTF: &[&str] =
    &["b-bits", "b-width-alt", "d-hexdump", "r-radix", "r-signed-width", "y-signed", "device-line"];

/// The macros of FreeBSD's `sys/sys/cdefs.h` that the `werror-shapes` cases use.
const CDEFS: &[&str] = &[
    "#define __weak_reference(sym, alias) \\",
    "    __asm__(\".weak \" #alias); \\",
    "    __asm__(\".equ \" #alias \", \" #sym)",
    "#define __sym_compat(sym, impl, verid) __asm__(\".symver \" #impl \", \" #sym \"@\" #verid)",
    "#define __sym_default(sym, impl, verid) __asm__(\".symver \" #impl \", \" #sym \"@@@\" #verid)",
    "#define __generic(expr, t, yes, no) _Generic(expr, t: yes, default: no)",
    "",
];

/// One `werror-shapes` case.
struct Shape {
    /// Where the shape comes from.
    after: &'static str,
    /// The C above `main`.
    top: Vec<String>,
    /// The other units, by name.
    units: Vec<(&'static str, Vec<String>)>,
    /// What `main` prints, as an `int` expression and its value.
    checks: Vec<(&'static str, i128)>,
    /// A line that prints text, and the text.
    text: Option<(&'static str, &'static str)>,
    /// The flags the case adds to the warning line.
    flags: &'static [&'static str],
    /// The tags the case adds to `gnu` and the provenance.
    tags: &'static [&'static str],
}

impl Shape {
    fn new(after: &'static str, top: Vec<String>) -> Self {
        Self {
            after,
            top,
            units: Vec::new(),
            checks: Vec::new(),
            text: None,
            flags: &[],
            tags: &["elf"],
        }
    }
}

/// Emits every case of both facets.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &shape in WERROR {
        if !sink.wants(Facet::WerrorShapes) {
            break;
        }
        let found = werror(shape);
        let mut program = Program::new(format!("{shape} after {}", found.after));
        for line in &found.top {
            program.top(line.clone());
        }
        for &(expr, value) in &found.checks {
            program.check(Ty::I32, expr, value);
        }
        if let Some((line, text)) = found.text {
            program.line(line);
            program.expects_text(text);
        }
        let (source, expected) = program.finish();
        let mut flags: Vec<String> = WORLD.iter().map(|flag| (*flag).to_owned()).collect();
        flags.extend(found.flags.iter().map(|flag| (*flag).to_owned()));
        let mut tags = vec!["gnu", FREEBSD];
        tags.extend(found.tags);
        sink.push_case(build(
            Facet::WerrorShapes,
            shape,
            &found.units,
            source,
            flags,
            expected,
            &tags,
        ));
    }
    for &shape in KPRINTF {
        if !sink.wants(Facet::KprintfFormat) {
            break;
        }
        let (after, calls) = kprintf(shape);
        let mut program = Program::new(format!("{shape} after {after}"));
        for line in KPRINTF_TOP {
            program.top(*line);
        }
        let buffers: Vec<&str> = BUFFERS
            .iter()
            .filter(|(name, _)| calls.iter().any(|(call, _)| call.contains(&format!(" {name},"))))
            .map(|&(_, line)| line)
            .collect();
        if !buffers.is_empty() {
            program.top("");
        }
        for line in buffers {
            program.top(line);
        }
        program.line("char buf[128];");
        let mut expected = String::new();
        for &(call, text) in calls {
            program.line(format!("{call};"));
            program.line("printf(\"%s\\n\", buf);");
            expected.push_str(text);
            expected.push('\n');
        }
        program.expects_text(&expected);
        let (source, expected) = program.finish();
        let flags = KERNEL.iter().map(|flag| (*flag).to_owned()).collect();
        let units = vec![("subr_prf", lines(&[SUBR_PRF.lines().collect::<Vec<_>>().as_slice()]))];
        sink.push_case(build(
            Facet::KprintfFormat,
            shape,
            &units,
            source,
            flags,
            expected,
            &["gnu", FREEBSD],
        ));
    }
}

/// The case of a shape, from its source and what it prints.
fn build(
    facet: Facet,
    shape: &str,
    units: &[(&'static str, Vec<String>)],
    source: String,
    flags: Vec<String>,
    expected: String,
    tags: &[&str],
) -> Case {
    let units = units
        .iter()
        .map(|(name, text)| {
            let mut out = format!(
                "// The {name} unit of the {shape} case.\n// Generated by rucc-corpus. Edit the generator, not this file.\n\n"
            );
            for line in text {
                out.push_str(line);
                out.push('\n');
            }
            Unit::new(*name, out)
        })
        .collect();
    Case::linked(
        facet,
        Axes::of([("shape", shape)]),
        Dialect::C17,
        source,
        units,
        flags,
        Expect::Output(expected),
    )
    .tagged(tags)
}

/// Owned lines from string slices.
fn lines(parts: &[&[&str]]) -> Vec<String> {
    parts.concat().iter().map(|line| (*line).to_owned()).collect()
}

/// The `werror-shapes` case of a shape.
fn werror(shape: &str) -> Shape {
    match shape {
        // `include/stdlib.h` gives the old `qsort_r` its version with `__sym_compat`, in every unit that includes it.
        // The unit declares `__qsort_r_compat` and does not call it, so the object has no symbol for it, and the assembler drops the `.symver`.
        "sym-compat" => {
            let mut found = Shape::new(
                "the qsort_r of FreeBSD's include/stdlib.h",
                lines(&[
                    CDEFS,
                    &[
                        "void corpus_sort_r(void *, unsigned long, unsigned long,",
                        "    int (*)(const void *, const void *, void *), void *);",
                        "void corpus_sort_r_compat(void *, unsigned long, unsigned long, void *,",
                        "    int (*)(void *, const void *, const void *));",
                        "__sym_compat(corpus_sort_r, corpus_sort_r_compat, FBSD_1.0);",
                        "",
                        "static int sum(const int *v, int n) {",
                        "    int s = 0;",
                        "    for (int i = 0; i < n; i++) {",
                        "        s += v[i];",
                        "    }",
                        "    return s;",
                        "}",
                        "",
                        "static const int numbers[] = {1, 2, 3};",
                    ],
                ]),
            );
            found.checks = vec![("sum(numbers, 3)", 6)];
            found
        }
        // `include/libgen.h` does the same for `basename` and `dirname`, and its macros choose the old function with `_Generic` when the argument is a `const char *`.
        // A call with a `char *` takes the new one, so the old names are never used.
        "sym-compat-generic" => {
            let mut found = Shape::new(
                "the basename and dirname of FreeBSD's include/libgen.h",
                lines(&[
                    CDEFS,
                    &[
                        "char *corpus_old_basename(char *);",
                        "char *corpus_old_dirname(char *);",
                        "char *corpus_basename(char *);",
                        "char *corpus_dirname(char *);",
                        "__sym_compat(corpus_basename, corpus_old_basename, FBSD_1.0);",
                        "__sym_compat(corpus_dirname, corpus_old_dirname, FBSD_1.0);",
                        "#define corpus_basename(x) __generic(x, const char *, corpus_old_basename, corpus_basename)(x)",
                        "#define corpus_dirname(x) __generic(x, const char *, corpus_old_dirname, corpus_dirname)(x)",
                        "",
                        "char *(corpus_basename)(char *path) {",
                        "    char *last = path;",
                        "    for (char *p = path; *p; p++) {",
                        "        if (*p == '/') {",
                        "            last = p + 1;",
                        "        }",
                        "    }",
                        "    return last;",
                        "}",
                        "",
                        "char *(corpus_dirname)(char *path) {",
                        "    char *last = 0;",
                        "    for (char *p = path; *p; p++) {",
                        "        if (*p == '/') {",
                        "            last = p;",
                        "        }",
                        "    }",
                        "    if (last) {",
                        "        *last = '\\0';",
                        "    }",
                        "    return path;",
                        "}",
                        "",
                        "static char tool[] = \"/usr/bin/cc\";",
                        "static char where[] = \"/usr/bin/cc\";",
                    ],
                ]),
            );
            found.text = Some((
                "printf(\"%s %s\\n\", corpus_basename(tool), corpus_dirname(where));",
                "cc /usr/bin\n",
            ));
            found
        }
        // `lib/libmd/sha1c.c` ends with `__weak_reference(_libmd_SHA_Init, SHA_Init)`, and `_libmd_SHA_Init` is in `sha0c.c`.
        // Here the C of the unit also calls the weak name.
        "weak-reference-units" => {
            let mut found = Shape::new(
                "the __weak_reference of FreeBSD's lib/libmd/sha1c.c",
                lines(&[
                    CDEFS,
                    &[
                        "__weak_reference(corpus_impl_init, corpus_init);",
                        "void corpus_init(int *);",
                        "",
                        "static int first(void) {",
                        "    int x = 0;",
                        "    corpus_init(&x);",
                        "    return x;",
                        "}",
                    ],
                ]),
            );
            found.units = vec![(
                "sha0c",
                lines(&[&[
                    "void corpus_impl_init(int *);",
                    "",
                    "void corpus_impl_init(int *p) {",
                    "    *p = 42;",
                    "}",
                ]]),
            )];
            found.checks = vec![("first()", 42)];
            found
        }
        // `lib/libc/gen/setcontext.c` renames its function with `__sym_default` next to a `__weak_reference`, and the world builds it with `-g`.
        // The `@@@` rename takes the function's own name away, and the debug information still has to find the function.
        "sym-default-debug" => {
            let mut found = Shape::new(
                "the __sym_default of FreeBSD's lib/libc/gen/setcontext.c",
                lines(&[
                    CDEFS,
                    &[
                        "int corpus_impl_ctx(int);",
                        "int corpus_alias_ctx(int);",
                        "",
                        "int corpus_impl_ctx(int x) {",
                        "    return x * 2;",
                        "}",
                        "",
                        "__weak_reference(corpus_impl_ctx, corpus_alias_ctx);",
                        "__sym_default(corpus_ctx, corpus_impl_ctx, FBSD_1.2);",
                    ],
                ]),
            );
            found.checks = vec![("corpus_impl_ctx(3)", 6), ("corpus_alias_ctx(4)", 8)];
            found.flags = &["-g"];
            found
        }
        // `usr.sbin/wlanstat/wlanstat.c` numbers 162 counters with `#define S_X AFTER(S_PREV)`, so a case label is a constant two brackets deeper for each counter.
        "after-chain" => {
            let mut top =
                vec!["#define AFTER(prev) ((prev)+1)".to_owned(), "#define S_0 0".to_owned()];
            top.extend((1..=170).map(|i| format!("#define S_{i} AFTER(S_{})", i - 1)));
            top.extend(lines(&[&[
                "",
                "static int pick(int n) {",
                "    switch (n) {",
                "    case S_0:",
                "        return 0;",
                "    case S_80:",
                "        return 80;",
                "    case S_160:",
                "        return 160;",
                "    case S_170:",
                "        return 170;",
                "    default:",
                "        return -1;",
                "    }",
                "}",
            ]]));
            let mut found =
                Shape::new("the AFTER chain of FreeBSD's usr.sbin/wlanstat/wlanstat.c", top);
            found.checks = vec![("pick(170)", 170), ("pick(80)", 80), ("pick(5)", -1)];
            found.tags = &[];
            found
        }
        // `share/mk/bsd.compat.mk` adds `--sysroot=${WORLDTMP}` to the 32-bit compat build, and `WORLDTMP` is empty when one directory is built.
        // GCC joins the empty sysroot to its directories, so it still finds `/usr/include`.
        "empty-sysroot" => {
            let mut found = Shape::new(
                "the --sysroot= of FreeBSD's share/mk/bsd.compat.mk",
                lines(&[&["#include <string.h>"]]),
            );
            found.checks = vec![("(int)strlen(\"corpus\")", 6)];
            found.flags = &["--sysroot="];
            found.tags = &["elf", "headers"];
            found
        }
        _ => unreachable!("{shape}"),
    }
}

/// The C above `main` in every `kprintf-format` case: the declarations of `kvprintf`'s callers, as the kernel's `sys/systm.h` gives them.
const KPRINTF_TOP: &[&str] = &[
    "typedef __SIZE_TYPE__ size_t;",
    "",
    "#ifdef __FreeBSD__",
    "#define __printflike(fmtarg, firstvararg) \\",
    "    __attribute__((__format__(__freebsd_kprintf__, fmtarg, firstvararg)))",
    "#else",
    "#define __printflike(fmtarg, firstvararg)",
    "#endif",
    "",
    "int ksprintf(char *buf, const char *cfmt, ...) __printflike(2, 3);",
    "int ksnrprintf(char *str, size_t size, int radix, const char *format, ...) __printflike(4, 5);",
];

/// The buffers that `%D` dumps, by name.
/// A case has only the buffers its calls use, because GCC's `-Wall` warns about a `static const` that is not used.
const BUFFERS: &[(&str, &str)] = &[
    ("mac", "static const unsigned char mac[6] = {0x00, 0x1b, 0x21, 0xa0, 0xff, 0x07};"),
    (
        "dead",
        "static const unsigned char dead[16] = {0xde, 0xad, 0xbe, 0xef, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};",
    ),
];

/// The calls of a `kprintf-format` shape, each with what it writes.
fn kprintf(shape: &str) -> (&'static str, &'static [(&'static str, &'static str)]) {
    match shape {
        "b-bits" => (
            "the %b of FreeBSD's sys/kern/subr_prf.c",
            &[
                (r#"ksprintf(buf, "reg=%b", 3, "\10\2BITTWO\1BITONE")"#, "reg=3<BITTWO,BITONE>"),
                (
                    r#"ksprintf(buf, "reg=%b", 3, "\10\201BITTWO\200BITONE")"#,
                    "reg=3<BITTWO,BITONE>",
                ),
                (
                    r#"ksprintf(buf, "flags=%b", 0x1a, "\20\1CARRY\2ZERO\4NEG\5OVF")"#,
                    "flags=1a<ZERO,NEG,OVF>",
                ),
                (r#"ksprintf(buf, "flags=%b", 0, "\20\1CARRY\2ZERO")"#, "flags=0"),
            ],
        ),
        "b-width-alt" => (
            "the %b of FreeBSD's sys/kern/subr_prf.c",
            &[
                (r#"ksprintf(buf, "[%12b]", 5, "\20\1A\2B\3C")"#, "[5<A,C>      ]"),
                (r#"ksprintf(buf, "[%-12b]", 5, "\20\1A\2B\3C")"#, "[5<A,C>      ]"),
                (r#"ksprintf(buf, "%#b", 10, "\10\2ONE\4THREE")"#, "012<ONE,THREE>"),
            ],
        ),
        "d-hexdump" => (
            "the %D of FreeBSD's sys/kern/subr_prf.c",
            &[
                (r#"ksprintf(buf, "%6D", mac, ":")"#, "00:1b:21:a0:ff:07"),
                (r#"ksprintf(buf, "%*D", 4, dead, " ")"#, "de ad be ef"),
                (r#"ksprintf(buf, "%D", dead, "")"#, "deadbeef000102030405060708090a0b"),
                (r#"ksprintf(buf, "%3D|%2D", mac, "-", dead, "")"#, "00-1b-21|dead"),
            ],
        ),
        "r-radix" => (
            "the %r of FreeBSD's sys/kern/subr_prf.c",
            &[
                (r#"ksnrprintf(buf, 128, 8, "%r", 64u)"#, "100"),
                (r#"ksnrprintf(buf, 128, 16, "%r %#r", 255u, 255u)"#, "ff 0xff"),
                (r#"ksnrprintf(buf, 128, 36, "%r", 35u)"#, "z"),
                (r#"ksnrprintf(buf, 128, 2, "%r", 5u)"#, "101"),
                (r#"ksnrprintf(buf, 128, 1, "%r", 99u)"#, "99"),
            ],
        ),
        "r-signed-width" => (
            "the %r of FreeBSD's sys/kern/subr_prf.c",
            &[
                (r#"ksnrprintf(buf, 128, 10, "%+r %+r", -42, 42)"#, "-42 42"),
                (r#"ksnrprintf(buf, 128, 16, "%+lr", -255L)"#, "-ff"),
                (
                    r#"ksnrprintf(buf, 128, 10, "%5r|%-5r|%05r", 42u, 42u, 42u)"#,
                    "   42|42   |00042",
                ),
            ],
        ),
        "y-signed" => (
            "the %y of FreeBSD's sys/kern/subr_prf.c",
            &[
                (r#"ksprintf(buf, "%y %y", -255, 255)"#, "-ff ff"),
                (r#"ksprintf(buf, "%ly %jy", -1L, (__INTMAX_TYPE__)-4096)"#, "-1 -1000"),
                (r#"ksprintf(buf, "[%6y][%-6y][%06y]", 26, 26, -26)"#, "[    1a][1a    ][-0001a]"),
                (r#"ksprintf(buf, "%hy %hhy", 0x18000, 0x1ff)"#, "-8000 -1"),
            ],
        ),
        "device-line" => (
            "the attach lines of FreeBSD's drivers",
            &[
                (
                    r#"ksprintf(buf, "%s%d: <%b> mac %6D irq %y", "em", 0, 0x9, "\20\1UP\2BROADCAST\4LOOPBACK", mac, ":", 11)"#,
                    "em0: <9<UP,LOOPBACK>> mac 00:1b:21:a0:ff:07 irq b",
                ),
                (
                    r#"ksprintf(buf, "%c%c %5s|%-5s|%.2s %% %u %x %X %o %#o", 'o', 'k', "ab", "cd", "efgh", 7u, 255u, 255u, 8u, 8u)"#,
                    "ok    ab|cd   |ef % 7 ff FF 10 010",
                ),
            ],
        ),
        _ => unreachable!("{shape}"),
    }
}

/// `kvprintf` from FreeBSD's `sys/kern/subr_prf.c`, with the `sprintf` and the `vsnrprintf` around it.
/// The names of the callers are `ksprintf` and `ksnrprintf`, so that GCC does not take them for the C library's.
/// The C library parts it uses (`va_list`, `toupper`, `isprint` and `strlen`) are written out, so the unit needs no header.
const SUBR_PRF: &str = r##"// kvprintf of FreeBSD's sys/kern/subr_prf.c, with ksprintf and ksnrprintf, the sprintf and the vsnrprintf around it.

typedef __builtin_va_list va_list;
#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_arg(ap, type) __builtin_va_arg(ap, type)
#define va_end(ap) __builtin_va_end(ap)
#define NULL ((void *)0)

typedef unsigned char u_char;
typedef unsigned short u_short;
typedef unsigned int u_int;
typedef unsigned long u_long;
typedef long long quad_t;
typedef unsigned long long u_quad_t;
typedef __INTMAX_TYPE__ intmax_t;
typedef __UINTMAX_TYPE__ uintmax_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;
typedef __PTRDIFF_TYPE__ ssize_t;
typedef __SIZE_TYPE__ size_t;
typedef __UINTPTR_TYPE__ uintptr_t;

// FreeBSD sizes this from intmax_t, which has 64 bits on every target.
#define MAXNBUF (64 + 1)

int kvprintf(char const *fmt, void (*func)(int, void *), void *arg, int radix, va_list ap);
int ksprintf(char *buf, const char *cfmt, ...);
int ksnrprintf(char *str, size_t size, int radix, const char *format, ...);

struct snprintf_arg {
    char *str;
    size_t remain;
};

static char const hex2ascii_data[] = "0123456789abcdefghijklmnopqrstuvwxyz";
#define hex2ascii(hex) (hex2ascii_data[hex])

static int kp_toupper(int c) { return (c >= 'a' && c <= 'z') ? c - ('a' - 'A') : c; }
static int kp_isprint(int c) { return (c >= ' ' && c <= '~'); }
static int imax(int a, int b) { return (a > b ? a : b); }

static char *
ksprintn(char *nbuf, uintmax_t num, int base, int *lenp, int upper)
{
    char *p, c;

    p = nbuf;
    *p = '\0';
    do {
        c = hex2ascii(num % base);
        *++p = upper ? kp_toupper(c) : c;
    } while (num /= base);
    if (lenp)
        *lenp = p - nbuf;
    return (p);
}

static int
isbitpos(char c)
{
    return (c != '\0' && (c <= ' ' || (c & 0x80) != 0));
}

static int
isprintnospace(char c)
{
    return (kp_isprint(c) && c != ' ');
}

int
kvprintf(char const *fmt, void (*func)(int, void *), void *arg, int radix, va_list ap)
{
#define PCHAR(c) {int cc=(c); if (func) (*func)(cc,arg); else *d++ = cc; retval++; }
    char nbuf[MAXNBUF];
    char *d;
    const char *p, *percent, *q;
    u_char *up;
    int ch, n, sign;
    uintmax_t num;
    int base, lflag, qflag, tmp, width, ladjust, sharpflag, dot;
    int cflag, hflag, jflag, tflag, zflag;
    int bconv, dwidth, upper;
    char padc;
    int stop = 0, retval = 0;

    num = 0;
    q = NULL;
    if (!func)
        d = (char *) arg;
    else
        d = NULL;

    if (fmt == NULL)
        fmt = "(fmt null)\n";

    if (radix < 2 || radix > 36)
        radix = 10;

    for (;;) {
        padc = ' ';
        width = 0;
        while ((ch = (u_char)*fmt++) != '%' || stop) {
            if (ch == '\0')
                return (retval);
            PCHAR(ch);
        }
        percent = fmt - 1;
        qflag = 0; lflag = 0; ladjust = 0; sharpflag = 0;
        sign = 0; dot = 0; bconv = 0; dwidth = 0; upper = 0;
        cflag = 0; hflag = 0; jflag = 0; tflag = 0; zflag = 0;
reswitch:   switch (ch = (u_char)*fmt++) {
        case '.':
            dot = 1;
            goto reswitch;
        case '#':
            sharpflag = 1;
            goto reswitch;
        case '+':
            sign = '+';
            goto reswitch;
        case '-':
            ladjust = 1;
            goto reswitch;
        case '%':
            PCHAR(ch);
            break;
        case '*':
            if (!dot) {
                width = va_arg(ap, int);
                if (width < 0) {
                    ladjust = !ladjust;
                    width = -width;
                }
            } else {
                dwidth = va_arg(ap, int);
            }
            goto reswitch;
        case '0':
            if (!dot) {
                padc = '0';
                goto reswitch;
            }
            /* FALLTHROUGH */
        case '1': case '2': case '3': case '4':
        case '5': case '6': case '7': case '8': case '9':
                for (n = 0;; ++fmt) {
                    n = n * 10 + ch - '0';
                    ch = *fmt;
                    if (ch < '0' || ch > '9')
                        break;
                }
            if (dot)
                dwidth = n;
            else
                width = n;
            goto reswitch;
        case 'b':
            ladjust = 1;
            bconv = 1;
            goto handle_nosign;
        case 'c':
            width -= 1;

            if (!ladjust && width > 0)
                while (width--)
                    PCHAR(padc);
            PCHAR(va_arg(ap, int));
            if (ladjust && width > 0)
                while (width--)
                    PCHAR(padc);
            break;
        case 'D':
            up = va_arg(ap, u_char *);
            p = va_arg(ap, char *);
            if (!width)
                width = 16;
            while(width--) {
                PCHAR(hex2ascii(*up >> 4));
                PCHAR(hex2ascii(*up & 0x0f));
                up++;
                if (width)
                    for (q = p; *q; q++)
                        PCHAR(*q);
            }
            break;
        case 'd':
        case 'i':
            base = 10;
            goto handle_sign;
        case 'h':
            if (hflag) {
                hflag = 0;
                cflag = 1;
            } else
                hflag = 1;
            goto reswitch;
        case 'j':
            jflag = 1;
            goto reswitch;
        case 'l':
            if (lflag) {
                lflag = 0;
                qflag = 1;
            } else
                lflag = 1;
            goto reswitch;
        case 'n':
            /*
             * We do not support %n in kernel, but consume the
             * argument.
             */
            if (jflag)
                (void)va_arg(ap, intmax_t *);
            else if (qflag)
                (void)va_arg(ap, quad_t *);
            else if (lflag)
                (void)va_arg(ap, long *);
            else if (zflag)
                (void)va_arg(ap, size_t *);
            else if (hflag)
                (void)va_arg(ap, short *);
            else if (cflag)
                (void)va_arg(ap, char *);
            else
                (void)va_arg(ap, int *);
            break;
        case 'o':
            base = 8;
            goto handle_nosign;
        case 'p':
            base = 16;
            sharpflag = (width == 0);
            sign = 0;
            num = (uintptr_t)va_arg(ap, void *);
            goto number;
        case 'q':
            qflag = 1;
            goto reswitch;
        case 'r':
            base = radix;
            if (sign) {
                sign = 0;
                goto handle_sign;
            }
            goto handle_nosign;
        case 's':
            p = va_arg(ap, char *);
            if (p == NULL)
                p = "(null)";
            if (!dot)
                n = __builtin_strlen (p);
            else
                for (n = 0; n < dwidth && p[n]; n++)
                    continue;

            width -= n;

            if (!ladjust && width > 0)
                while (width--)
                    PCHAR(padc);
            while (n--)
                PCHAR(*p++);
            if (ladjust && width > 0)
                while (width--)
                    PCHAR(padc);
            break;
        case 't':
            tflag = 1;
            goto reswitch;
        case 'u':
            base = 10;
            goto handle_nosign;
        case 'X':
            upper = 1;
            /* FALLTHROUGH */
        case 'x':
            base = 16;
            goto handle_nosign;
        case 'y':
            base = 16;
            goto handle_sign;
        case 'z':
            zflag = 1;
            goto reswitch;
handle_nosign:
            if (jflag)
                num = va_arg(ap, uintmax_t);
            else if (qflag)
                num = va_arg(ap, u_quad_t);
            else if (tflag)
                num = va_arg(ap, ptrdiff_t);
            else if (lflag)
                num = va_arg(ap, u_long);
            else if (zflag)
                num = va_arg(ap, size_t);
            else if (hflag)
                num = (u_short)va_arg(ap, int);
            else if (cflag)
                num = (u_char)va_arg(ap, int);
            else
                num = va_arg(ap, u_int);
            if (bconv) {
                q = va_arg(ap, char *);
                base = *q++;
            }
            goto number;
handle_sign:
            if (jflag)
                num = va_arg(ap, intmax_t);
            else if (qflag)
                num = va_arg(ap, quad_t);
            else if (tflag)
                num = va_arg(ap, ptrdiff_t);
            else if (lflag)
                num = va_arg(ap, long);
            else if (zflag)
                num = va_arg(ap, ssize_t);
            else if (hflag)
                num = (short)va_arg(ap, int);
            else if (cflag)
                num = (signed char)va_arg(ap, int);
            else
                num = va_arg(ap, int);
            if ((intmax_t)num < 0) {
                sign = '-';
                num = -(intmax_t)num;
            }
number:
            p = ksprintn(nbuf, num, base, &n, upper);
            tmp = 0;
            if (sharpflag && num != 0) {
                if (base == 8)
                    tmp++;
                else if (base == 16)
                    tmp += 2;
            }
            if (sign)
                tmp++;

            if (!ladjust && padc == '0')
                dwidth = width - tmp;
            width -= tmp + imax(dwidth, n);
            dwidth -= n;
            if (!ladjust)
                while (width-- > 0)
                    PCHAR(' ');
            if (sign)
                PCHAR(sign);
            if (sharpflag && num != 0) {
                if (base == 8) {
                    PCHAR('0');
                } else if (base == 16) {
                    PCHAR('0');
                    PCHAR('x');
                }
            }
            while (dwidth-- > 0)
                PCHAR('0');

            while (*p)
                PCHAR(*p--);

            if (bconv && num != 0) {
                /* %b conversion flag format. */
                tmp = retval;
                while (isbitpos(*q)) {
                    if ((*q & 0x80) != 0)
                        n = *q++ & 0x7f;
                    else
                        n = *q++ - 1;
                    if (num & (1ULL << n)) {
                        PCHAR(retval != tmp ?
                            ',' : '<');
                        for (; isprintnospace(*q); ++q)
                            PCHAR(*q);
                    } else
                        for (; isprintnospace(*q); ++q)
                            continue;
                }
                if (retval != tmp) {
                    PCHAR('>');
                    width -= retval - tmp;
                }
            }

            if (ladjust)
                while (width-- > 0)
                    PCHAR(' ');

            break;
        default:
            while (percent < fmt)
                PCHAR(*percent++);
            /*
             * Since we ignore a formatting argument it is no
             * longer safe to obey the remaining formatting
             * arguments as the arguments will no longer match
             * the format specs.
             */
            stop = 1;
            break;
        }
    }
#undef PCHAR
}

int
ksprintf(char *buf, const char *cfmt, ...)
{
    int retval;
    va_list ap;

    va_start(ap, cfmt);
    retval = kvprintf(cfmt, NULL, (void *)buf, 10, ap);
    buf[retval] = '\0';
    va_end(ap);
    return (retval);
}

static void
snprintf_func(int ch, void *arg)
{
    struct snprintf_arg *const info = arg;

    if (info->remain >= 2) {
        *info->str++ = ch;
        info->remain--;
    }
}

int
ksnrprintf(char *str, size_t size, int radix, const char *format, ...)
{
    struct snprintf_arg info;
    int retval;
    va_list ap;

    va_start(ap, format);
    info.str = str;
    info.remain = size;
    retval = kvprintf(format, snprintf_func, &info, radix, ap);
    if (info.remain >= 1)
        *info.str++ = '\0';
    va_end(ap);
    return (retval);
}
"##;

#[cfg(test)]
mod tests {
    use super::{KPRINTF, kprintf};
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_shape_is_there_with_the_freebsd_provenance() {
        let manifest = generate(&Options::all()).unwrap();
        for (facet, count) in [(Facet::WerrorShapes, 6), (Facet::KprintfFormat, 7)] {
            let cases: Vec<_> = manifest.cases.iter().filter(|case| case.facet == facet).collect();
            assert_eq!(cases.len(), count, "{facet:?}");
            for case in cases {
                assert!(case.has_tag("provenance:freebsd"), "{}", case.id);
            }
        }
    }

    /// The FreeBSD GCC 14 and 15 crash on `%b` with a length modifier, so no call may use one.
    #[test]
    fn no_b_conversion_has_a_length_modifier() {
        for &shape in KPRINTF {
            for &(call, _) in kprintf(shape).1 {
                let format = call.split('"').nth(1).unwrap();
                for spec in format.split('%').skip(1) {
                    let spec = spec.trim_start_matches(|c: char| "#-+ 0123456789.*".contains(c));
                    let length = spec.trim_start_matches(|c: char| "hlqjzt".contains(c));
                    assert!(
                        !(length.starts_with('b') && length.len() < spec.len()),
                        "{shape}: {call}"
                    );
                }
            }
        }
    }
}
