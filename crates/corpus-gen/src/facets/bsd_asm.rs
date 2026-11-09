//! The file scope `asm` of the FreeBSD and NetBSD headers, as a facet of its own.
//!
//! The BSD headers make symbols and sections with top level `asm`.
//! FreeBSD's `__weak_reference` is two blocks, `.weak` and then `.equ`, and `__FBSDID` is a `.ident`.
//! NetBSD's `__weak_alias` and `__strong_alias` are one block with `=`, and its `__RCSID` pushes a `.ident` section with the merge and string flags.
//! The C runtime of each system adds an ident note in a section that gets its type from its name.
//! `__warn_references` opens a section in one block, writes into it in the next and closes it in a third.
//!
//! Plan 03.2 of tamnd/rucc-bsd found three faults in rucc here.
//! The alias of `__weak_reference` was a local symbol (A), the `.ident` string was lost (B), and `.pushsection` took the wrong flags and type (C).
//! Each case is built twice, once as it is and once with `-frucc-asm-reader=off`, which turns off rucc's reader of file scope `asm`.
//! The harness gives a `-frucc-` flag only to rucc, so on the reference the two cases are the same program.
//!
//! An alias case links a second unit that defines the symbol and the alias, and the unit with `main` calls the alias.
//! A local alias does not link.
//! The other cases look at their own executable.
//! They read the section headers from `/proc/self/exe` and walk the notes of the program headers from `__ehdr_start`, so they need an ELF64 target.
//! The `asm` is the GNU syntax of x86-64, so every case carries the `x86-64` and `elf` tags.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, Unit};

/// The flag that turns off rucc's reader of file scope `asm`.
pub(crate) const READER_OFF: &str = "-frucc-asm-reader=off";

/// The ELF64 header, program header and section header, written out because no system header is included.
pub(crate) const ELF_TYPES: &[&str] = &[
    "typedef struct {",
    "    unsigned char ident[16];",
    "    unsigned short type, machine;",
    "    unsigned int version;",
    "    unsigned long entry, phoff, shoff;",
    "    unsigned int flags;",
    "    unsigned short ehsize, phentsize, phnum, shentsize, shnum, shstrndx;",
    "} elf_ehdr;",
    "",
    "typedef struct {",
    "    unsigned int type, flags;",
    "    unsigned long offset, vaddr, paddr, filesz, memsz, align;",
    "} elf_phdr;",
    "",
    "typedef struct {",
    "    unsigned int name, type;",
    "    unsigned long flags, addr, offset, size;",
    "    unsigned int link, info;",
    "    unsigned long addralign, entsize;",
    "} elf_shdr;",
    "",
];

/// `note_desc(name, type)` walks the `PT_NOTE` segments of the running program and gives the four byte descriptor of the note with that name and type, or 0.
/// A note only gets into a `PT_NOTE` segment when its section has the type `SHT_NOTE` and the flag `SHF_ALLOC`.
pub(crate) const NOTE_WALK: &[&str] = &[
    "extern const elf_ehdr __ehdr_start;",
    "",
    "static unsigned int note_desc(const char *name, unsigned int type) {",
    "    const char *image = (const char *)&__ehdr_start;",
    "    const elf_phdr *ph = (const elf_phdr *)(image + __ehdr_start.phoff);",
    "    unsigned long first = 0;",
    "    for (int i = 0; i < __ehdr_start.phnum; i++) {",
    "        if (ph[i].type == 1u && ph[i].offset == 0u) {",
    "            first = ph[i].vaddr;",
    "            break;",
    "        }",
    "    }",
    "    for (int i = 0; i < __ehdr_start.phnum; i++) {",
    "        if (ph[i].type != 4u) {",
    "            continue;",
    "        }",
    "        const unsigned char *p = (const unsigned char *)image + (ph[i].vaddr - first);",
    "        const unsigned char *end = p + ph[i].memsz;",
    "        while (p + 12 <= end) {",
    "            const unsigned int *word = (const unsigned int *)p;",
    "            unsigned int namesz = word[0], descsz = word[1], kind = word[2];",
    "            const char *n = (const char *)(p + 12);",
    "            const unsigned char *desc = p + 12 + ((namesz + 3u) & ~3u);",
    "            unsigned int length = 0;",
    "            while (name[length]) {",
    "                length++;",
    "            }",
    "            int same = namesz == length + 1u;",
    "            for (unsigned int k = 0; same && k < length; k++) {",
    "                same = n[k] == name[k];",
    "            }",
    "            if (same && kind == type && descsz == 4u) {",
    "                return *(const unsigned int *)desc;",
    "            }",
    "            p = desc + ((descsz + 3u) & ~3u);",
    "        }",
    "    }",
    "    return 0;",
    "}",
    "",
];

/// `section_of(name, text)` reads the running program's file and finds the section of that name.
/// It gives the section type times 4096, plus the flags `SHF_WRITE`, `SHF_ALLOC`, `SHF_MERGE` and `SHF_STRINGS`, plus 256 when the text is in the section.
/// It gives 0 when there is no such section.
pub(crate) const SECTION_WALK: &[&str] = &[
    "int open(const char *, int, ...);",
    "long read(int, void *, unsigned long);",
    "int close(int);",
    "",
    "#define FILE_BYTES (1ul << 23)",
    "static unsigned char file[FILE_BYTES];",
    "",
    "static unsigned int section_of(const char *name, const char *text) {",
    "    int fd = open(\"/proc/self/exe\", 0);",
    "    long size = 0, got;",
    "    while (fd >= 0 && (got = read(fd, file + size, FILE_BYTES - (unsigned long)size)) > 0) {",
    "        size += got;",
    "    }",
    "    if (fd >= 0) {",
    "        close(fd);",
    "    }",
    "    const elf_ehdr *eh = (const elf_ehdr *)file;",
    "    if (size < 64 || eh->shoff == 0u) {",
    "        return 0;",
    "    }",
    "    const elf_shdr *sh = (const elf_shdr *)(file + eh->shoff);",
    "    const char *names = (const char *)file + sh[eh->shstrndx].offset;",
    "    for (int i = 0; i < eh->shnum; i++) {",
    "        const char *n = names + sh[i].name;",
    "        int k = 0;",
    "        while (name[k] && n[k] == name[k]) {",
    "            k++;",
    "        }",
    "        if (name[k] || n[k]) {",
    "            continue;",
    "        }",
    "        unsigned int found = 0;",
    "        const char *data = (const char *)file + sh[i].offset;",
    "        for (unsigned long at = 0; at < sh[i].size && !found; at++) {",
    "            unsigned long j = 0;",
    "            while (text[j] && at + j < sh[i].size && data[at + j] == text[j]) {",
    "                j++;",
    "            }",
    "            found = text[j] == 0;",
    "        }",
    "        return sh[i].type * 4096u + (unsigned int)(sh[i].flags & 0x33u) + found * 256u;",
    "    }",
    "    return 0;",
    "}",
    "",
];

/// What `section_of` gives for a section of this type and these flags that holds the text.
pub(crate) const fn section(kind: u32, flags: u32) -> i128 {
    (kind * 4096 + flags + 256) as i128
}

/// `SHT_PROGBITS`.
pub(crate) const PROGBITS: u32 = 1;
/// `SHT_NOTE`.
pub(crate) const NOTE: u32 = 7;
/// `SHF_ALLOC`.
pub(crate) const ALLOC: u32 = 0x2;
/// `SHF_MERGE` and `SHF_STRINGS`.
pub(crate) const MERGE_STRINGS: u32 = 0x30;

/// The FreeBSD release the FreeBSD cases come from.
const FREEBSD: &str = "provenance:freebsd";
/// The NetBSD release the NetBSD cases come from.
const NETBSD: &str = "provenance:netbsd";

/// Each shape of the facet.
const SHAPES: &[&str] = &[
    "weak-equ-before",
    "weak-equ-after",
    "weak-equ-override",
    "weak-equ-data",
    "weak-alias",
    "strong-alias",
    "ident",
    "rcsid",
    "note-typed",
    "note-named",
    "section-previous",
    "pushsection-nested",
];

/// What a case is.
struct Shape {
    /// The header the shape is written after.
    after: &'static str,
    /// The provenance tag.
    from: &'static str,
    /// The C above `main`.
    top: Vec<String>,
    /// The other units, by name.
    units: Vec<(&'static str, Vec<String>)>,
    /// What `main` prints, as an `unsigned int` expression and its value.
    checks: Vec<(&'static str, i128)>,
}

/// Emits every case of the facet.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    for &shape in SHAPES {
        for reader in ["on", "off"] {
            if !sink.wants(Facet::FileScopeAsm) {
                return;
            }
            let found = shape_of(shape);
            let mut program = Program::new(format!(
                "{shape} after {}, with the asm reader {reader}",
                found.after
            ));
            for line in &found.top {
                program.top(line.clone());
            }
            for &(expr, value) in &found.checks {
                program.check(Ty::U32, expr, value);
            }
            let (source, expected) = program.finish();
            let units = found
                .units
                .into_iter()
                .map(|(name, text)| Unit::new(name, unit_source(shape, &text)))
                .collect();
            let flags = if reader == "off" { vec![READER_OFF.to_owned()] } else { Vec::new() };
            let case = Case::linked(
                Facet::FileScopeAsm,
                Axes::of([("shape", shape), ("reader", reader)]),
                Dialect::C17,
                source,
                units,
                flags,
                Expect::Output(expected),
            )
            .tagged(&["gnu", "x86-64", "elf", found.from]);
            sink.push_case(case);
        }
    }
}

/// A unit other than the one with `main`, with the comment the generated files start with.
fn unit_source(shape: &str, lines: &[String]) -> String {
    let mut out = format!(
        "// The library unit of the {shape} case.\n// Generated by rucc-corpus. Edit the generator, not this file.\n\n"
    );
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Owned lines from string slices.
fn lines(parts: &[&[&str]]) -> Vec<String> {
    parts.concat().iter().map(|line| (*line).to_owned()).collect()
}

/// FreeBSD's `__weak_reference` from `sys/sys/cdefs.h`.
const WEAK_REFERENCE: &[&str] = &[
    "#define __weak_reference(sym, alias) \\",
    "    __asm__(\".weak \" #alias); \\",
    "    __asm__(\".equ \" #alias \", \" #sym)",
    "",
];

/// The function the alias cases name.
const REAL: &[&str] =
    &["unsigned int _corpus_real(unsigned int x) {", "    return x * 3u + 1u;", "}", ""];

/// The object the data alias cases name, and a function that reads it by its own name.
const TABLE: &[&str] = &[
    "unsigned int _corpus_table = 41u;",
    "",
    "unsigned int corpus_table_get(void) {",
    "    return _corpus_table;",
    "}",
    "",
];

/// `real(5)`.
const REAL_5: i128 = 16;

/// The main unit of a function alias case.
fn calls_alias() -> Vec<String> {
    lines(&[&[
        "unsigned int corpus_alias(unsigned int);",
        "unsigned int _corpus_real(unsigned int);",
    ]])
}

/// The main unit of a data alias case.
/// It writes through the alias and reads through the name.
fn uses_table() -> Vec<String> {
    lines(&[&[
        "extern unsigned int corpus_table;",
        "unsigned int corpus_table_get(void);",
        "",
        "static unsigned int bump(void) {",
        "    corpus_table += 1u;",
        "    return corpus_table_get();",
        "}",
    ]])
}

fn shape_of(shape: &str) -> Shape {
    match shape {
        "weak-equ-before" => Shape {
            after: "sys/sys/cdefs.h in FreeBSD 15.1",
            from: FREEBSD,
            top: calls_alias(),
            units: vec![(
                "lib",
                lines(&[
                    WEAK_REFERENCE,
                    &["__weak_reference(_corpus_real, corpus_alias);", ""],
                    REAL,
                ]),
            )],
            checks: vec![("corpus_alias(5u)", REAL_5), ("_corpus_real(7u)", 22)],
        },
        "weak-equ-after" => Shape {
            after: "sys/sys/cdefs.h in FreeBSD 15.1",
            from: FREEBSD,
            top: calls_alias(),
            units: vec![(
                "lib",
                lines(&[WEAK_REFERENCE, REAL, &["__weak_reference(_corpus_real, corpus_alias);"]]),
            )],
            checks: vec![("corpus_alias(5u)", REAL_5), ("_corpus_real(7u)", 22)],
        },
        "weak-equ-override" => Shape {
            after: "sys/sys/cdefs.h in FreeBSD 15.1",
            from: FREEBSD,
            top: calls_alias(),
            units: vec![
                (
                    "lib",
                    lines(&[
                        WEAK_REFERENCE,
                        REAL,
                        &["__weak_reference(_corpus_real, corpus_alias);"],
                    ]),
                ),
                (
                    "strong",
                    lines(&[&[
                        "unsigned int corpus_alias(unsigned int x) {",
                        "    return x + 100u;",
                        "}",
                    ]]),
                ),
            ],
            checks: vec![("corpus_alias(5u)", 105), ("_corpus_real(5u)", REAL_5)],
        },
        "weak-equ-data" => Shape {
            after: "sys/sys/cdefs.h in FreeBSD 15.1",
            from: FREEBSD,
            top: uses_table(),
            units: vec![(
                "lib",
                lines(&[
                    WEAK_REFERENCE,
                    TABLE,
                    &["__weak_reference(_corpus_table, corpus_table);"],
                ]),
            )],
            checks: vec![("bump()", 42), ("corpus_table", 42)],
        },
        "weak-alias" => Shape {
            after: "sys/sys/cdefs_elf.h in NetBSD 11.1",
            from: NETBSD,
            top: calls_alias(),
            units: vec![(
                "lib",
                lines(&[
                    &[
                        "#define __weak_alias(alias, sym) \\",
                        "    __asm(\".weak \" #alias \"\\n\" #alias \" = \" #sym);",
                        "",
                    ],
                    REAL,
                    &["__weak_alias(corpus_alias, _corpus_real)"],
                ]),
            )],
            checks: vec![("corpus_alias(5u)", REAL_5), ("_corpus_real(7u)", 22)],
        },
        "strong-alias" => Shape {
            after: "sys/sys/cdefs_elf.h in NetBSD 11.1",
            from: NETBSD,
            top: uses_table(),
            units: vec![(
                "lib",
                lines(&[
                    &[
                        "#define __strong_alias(alias, sym) \\",
                        "    __asm(\".global \" #alias \"\\n\" #alias \" = \" #sym);",
                        "",
                    ],
                    TABLE,
                    &["__strong_alias(corpus_table, _corpus_table)"],
                ]),
            )],
            checks: vec![("bump()", 42), ("corpus_table", 42)],
        },
        "ident" => Shape {
            after: "sys/sys/cdefs.h in FreeBSD 15.1",
            from: FREEBSD,
            top: lines(&[
                ELF_TYPES,
                SECTION_WALK,
                &[
                    "#define __IDSTRING(name, string) __asm__(\".ident\\t\\\"\" string \"\\\"\")",
                    "#define __FBSDID(s) __IDSTRING(__rcsid, s)",
                    "",
                    "__FBSDID(\"$FreeBSD: corpus ident 1 $\");",
                ],
            ]),
            units: Vec::new(),
            checks: vec![(
                "section_of(\".comment\", \"$FreeBSD: corpus ident 1 $\")",
                section(PROGBITS, MERGE_STRINGS),
            )],
        },
        "rcsid" => Shape {
            after: "sys/sys/cdefs_elf.h in NetBSD 11.1",
            from: NETBSD,
            top: lines(&[
                ELF_TYPES,
                SECTION_WALK,
                &[
                    "#define __SECTIONSTRING(_sec, _str) \\",
                    "    __asm(\".pushsection \" #_sec \",\\\"MS\\\",@progbits,1\\n\" \\",
                    "          \".asciz \\\"\" _str \"\\\"\\n\" \\",
                    "          \".popsection\")",
                    "#define __IDSTRING(_n, _s) __SECTIONSTRING(.ident, _s)",
                    "#define __RCSID(_s) __IDSTRING(rcsid, _s)",
                    "",
                    "__RCSID(\"$NetBSD: corpus rcsid 1 $\");",
                    "",
                    "static unsigned int after_ident = 3u;",
                ],
            ]),
            units: Vec::new(),
            checks: vec![
                (
                    "section_of(\".ident\", \"$NetBSD: corpus rcsid 1 $\")",
                    section(PROGBITS, MERGE_STRINGS),
                ),
                ("after_ident += 1u", 4),
            ],
        },
        "note-typed" => Shape {
            after: "lib/csu/common/crtbrand.S in FreeBSD 15.1",
            from: FREEBSD,
            top: lines(&[
                ELF_TYPES,
                NOTE_WALK,
                SECTION_WALK,
                &[
                    "__asm(\".pushsection .note.corpus.tag,\\\"a\\\",@note\\n\"",
                    "      \"\\t.p2align 2\\n\"",
                    "      \"\\t.long 7, 4, 1\\n\"",
                    "      \"\\t.asciz \\\"Corpus\\\"\\n\"",
                    "      \"\\t.p2align 2\\n\"",
                    "      \"\\t.long 1501000\\n\"",
                    "      \".popsection\");",
                ],
            ]),
            units: Vec::new(),
            checks: vec![
                ("note_desc(\"Corpus\", 1u)", 1_501_000),
                ("section_of(\".note.corpus.tag\", \"Corpus\")", section(NOTE, ALLOC)),
            ],
        },
        "note-named" => Shape {
            after: "lib/csu/common/sysident.S in NetBSD 11.1",
            from: NETBSD,
            top: lines(&[
                ELF_TYPES,
                NOTE_WALK,
                SECTION_WALK,
                &[
                    "__asm(\".section \\\".note.netbsd.ident\\\", \\\"a\\\"\\n\"",
                    "      \"\\t.p2align 2\\n\"",
                    "      \"\\t.long 7\\n\"",
                    "      \"\\t.long 4\\n\"",
                    "      \"\\t.long 1\\n\"",
                    "      \"\\t.ascii \\\"NetBSD\\\\0\\\\0\\\"\\n\"",
                    "      \"\\t.long 1100000000\\n\"",
                    "      \"\\t.previous\");",
                    "",
                    "static unsigned int after_note = 5u;",
                ],
            ]),
            units: Vec::new(),
            checks: vec![
                ("note_desc(\"NetBSD\", 1u)", 1_100_000_000),
                ("section_of(\".note.netbsd.ident\", \"NetBSD\")", section(NOTE, ALLOC)),
                ("after_note *= 3u", 15),
            ],
        },
        "section-previous" => Shape {
            after: "sys/sys/cdefs.h in FreeBSD 15.1",
            from: FREEBSD,
            top: lines(&[&[
                "#define __corpus_warn(a, b) \\",
                "    __asm__(\".section corpus_warn,\\\"a\\\"\"); \\",
                "    __asm__(\".balign 4\\n\\t.long \" #a \", \" #b); \\",
                "    __asm__(\".previous\")",
                "",
                "__corpus_warn(0x11, 0x22);",
                "unsigned int after_warn = 5u;",
                "__corpus_warn(0x100, 0x200);",
                "",
                "extern const unsigned int __start_corpus_warn[], __stop_corpus_warn[];",
                "",
                "static unsigned int warn_count(void) {",
                "    return (unsigned int)(__stop_corpus_warn - __start_corpus_warn);",
                "}",
                "",
                "static unsigned int warn_sum(void) {",
                "    unsigned int sum = 0;",
                "    for (const unsigned int *p = __start_corpus_warn; p < __stop_corpus_warn; p++) {",
                "        sum += *p;",
                "    }",
                "    return sum;",
                "}",
            ]]),
            units: Vec::new(),
            checks: vec![
                ("warn_count()", 4),
                ("warn_sum()", 0x333),
                ("after_warn += warn_count()", 9),
            ],
        },
        _ => Shape {
            after: "sys/sys/cdefs_elf.h in NetBSD 11.1",
            from: NETBSD,
            top: lines(&[&[
                "__asm(\".pushsection corpus_outer,\\\"a\\\"\\n\"",
                "      \"\\t.balign 4\\n\\t.long 1\\n\"",
                "      \".pushsection corpus_inner,\\\"a\\\"\\n\"",
                "      \"\\t.balign 4\\n\\t.long 10\\n\\t.long 20\\n\"",
                "      \".popsection\\n\"",
                "      \"\\t.long 2\\n\"",
                "      \".popsection\");",
                "",
                "__asm__(\".pushsection corpus_outer,\\\"a\\\"\");",
                "__asm__(\".balign 4\\n\\t.long 4\");",
                "__asm__(\".popsection\");",
                "",
                "static unsigned int counter = 3u;",
                "",
                "extern const unsigned int __start_corpus_outer[], __stop_corpus_outer[];",
                "extern const unsigned int __start_corpus_inner[], __stop_corpus_inner[];",
                "",
                "static unsigned int sum(const unsigned int *p, const unsigned int *end) {",
                "    unsigned int total = 0;",
                "    for (; p < end; p++) {",
                "        total = total * 16u + *p;",
                "    }",
                "    return total;",
                "}",
            ]]),
            units: Vec::new(),
            checks: vec![
                ("sum(__start_corpus_outer, __stop_corpus_outer)", 0x124),
                ("sum(__start_corpus_inner, __stop_corpus_inner)", 10 * 16 + 20),
                ("counter += 1u", 4),
            ],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{SHAPES, shape_of};

    #[test]
    fn every_shape_prints_something() {
        for &shape in SHAPES {
            assert!(!shape_of(shape).checks.is_empty(), "{shape}");
        }
    }

    #[test]
    fn the_last_arm_is_the_last_shape() {
        assert_eq!(SHAPES.last(), Some(&"pushsection-nested"));
    }
}
