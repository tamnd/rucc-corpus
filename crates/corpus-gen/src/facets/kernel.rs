//! The facets the Linux kernel asks for.
//!
//! `lkmm` is the first. The kernel memory model in `tools/memory-model` is written in terms of
//! what a compiler does with plain, `volatile` and atomic accesses, and it leans on promises that
//! C11 never makes. Section 6.7 of the rucc-kernel plan lists them: a `volatile` access of a word
//! or less is one instruction that is never removed, split or repeated, a store the program only
//! makes on one arm of a branch is never made on the other, and a store to one field never
//! becomes a wider store over its neighbour.
//!
//! None of that changes what a single threaded program prints, which is why it needs a trick to
//! be a corpus case at all. There are two. A store that must not happen goes to a location on a
//! page the program made read only with `mprotect`, so a compiler that invents the store gets a
//! fault rather than a wrong number. A load that must happen every time is a load of something a
//! second thread writes, so a compiler that hoists it out of a loop gets a hang, which the
//! harness times out, and a compiler that splits it gets a value neither thread ever wrote.
//!
//! The rule about address and data dependencies is not here. Replacing a pointer loaded with
//! `READ_ONCE` by an equal one the compiler already knew about breaks ordering only on a machine
//! that reorders dependent loads, and no run of a program on one core pair can show that. It
//! wants a pattern check on the assembly, which the corpus does not have yet.
//!
//! `mitigations` is the second. The kernel is built with the flags that route every indirect
//! branch and every return through a thunk it links in, put a trap after every `ret` and every
//! indirect jump, and mark every place an indirect branch may land. Each of those changes the
//! instructions at the two ends of every function, so each is a new way for a back end to get
//! the prologue, the epilogue or a tail call wrong. The cases are ordinary programs built with
//! those flags, and they print what they would print without them. The thunks are in the
//! program, as a few lines of top level assembly that do what the kernel's do without the
//! speculation trap, so both compilers link the same ones.
//!
//! Each case names the kernel file whose idiom it has the shape of in its first line, and never
//! copies code from it.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet};

/// The tag that says where a case came from.
const PROVENANCE: &str = "provenance:linux";

/// The seed every case reads through a `volatile` global.
const SEED: i128 = 7;

/// How many times the reader in `tearing` looks at the word.
///
/// A compiler that writes the word as two halves is caught within a few thousand reads on any
/// machine with two cores, so this is plenty, and it keeps the case under a tenth of a second.
const READS: i128 = 200_000;

/// Emits every facet in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    lkmm(sink);
    mitigations(sink);
}

/// One case per rule and access width.
///
/// `spin` and `tearing` are the first rule, about `READ_ONCE` and `WRITE_ONCE` themselves.
/// `guarded-store`, `guarded-loop` and `narrow-field` are the second, about stores nobody wrote.
/// `control-dependency` is the third, the store after a branch on a `READ_ONCE` that
/// `memory-barriers.txt` says must stay after it.
fn lkmm(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["spin", "tearing", "guarded-store", "guarded-loop", "narrow-field", "control-dependency"];
    const WIDTHS: &[Ty] = &[Ty::U8, Ty::U16, Ty::U32, Ty::U64];
    for &shape in SHAPES {
        for &ty in WIDTHS {
            if !sink.wants(Facet::Lkmm) {
                return;
            }
            let program = lkmm_program(shape, ty);
            let axes = Axes::of([("shape", shape), ("width", ty.name())]);
            let tags: &[&str] = if matches!(shape, "spin" | "tearing") {
                &["gnu", "headers", "threads", PROVENANCE]
            } else {
                &["gnu", "headers", PROVENANCE]
            };
            sink.push_tagged(Facet::Lkmm, axes, Dialect::C17, program, tags);
        }
    }
}

/// One `lkmm` case.
fn lkmm_program(shape: &str, ty: Ty) -> Program {
    let t = ty.c_name();
    let from = match shape {
        "spin" | "tearing" => "include/asm-generic/rwonce.h",
        "control-dependency" => "Documentation/memory-barriers.txt",
        _ => "tools/memory-model/Documentation/explanation.txt",
    };
    let mut program = Program::new(format!(
        "the kernel memory model's {shape} rule on {t}, after {from} in Linux 7.2"
    ));
    program.define("_POSIX_C_SOURCE", "200809L");
    program.top("#define READ_ONCE(x) (*(const volatile __typeof__(x) *)&(x))");
    program.top("#define WRITE_ONCE(x, v) (*(volatile __typeof__(x) *)&(x) = (v))");
    program.top("");
    program.top(format!("static volatile {t} seed_in = {};", ty.literal(SEED)));
    program.top("");
    match shape {
        "spin" | "tearing" => threaded(&mut program, shape, ty),
        _ => sealed(&mut program, shape, ty),
    }
    program
}

/// The two cases that share a location with a second thread.
fn threaded(program: &mut Program, shape: &str, ty: Ty) {
    let t = ty.c_name();
    program.include("pthread.h");
    if shape == "spin" {
        // A handshake in both directions. Each side waits in a loop whose only exit is a
        // `READ_ONCE` of something the other side writes, so a load hoisted out of either loop
        // is a program that never finishes.
        program.top(format!("static {t} go;"));
        program.top(format!("static {t} payload;"));
        program.top(format!("static {t} reply;"));
        program.top("");
        program.top("static void *answer(void *arg) {");
        program.top("    (void)arg;");
        program.top("    while (!READ_ONCE(go)) {");
        program.top("    }");
        program.top(format!("    WRITE_ONCE(reply, ({t})(READ_ONCE(payload) * 3 + 1));"));
        program.top("    return 0;");
        program.top("}");
        program.line("pthread_t other;");
        program.line("pthread_create(&other, 0, answer, 0);");
        program.line("WRITE_ONCE(payload, seed_in);");
        program.line("WRITE_ONCE(go, 1);");
        program.line("while (!READ_ONCE(reply)) {");
        program.line("}");
        program.line("pthread_join(other, 0);");
        program.blank();
        program.check(ty, "reply", ty.convert(SEED * 3 + 1));
        return;
    }
    // The writer flips the word between nothing and everything. A reader that sees anything else
    // saw half of one write and half of another, which is the tearing `READ_ONCE` rules out.
    let ones = ty.max();
    let all = ty.literal(ones);
    program.top(format!("static {t} word;"));
    program.top("static int stop;");
    program.top("static int started;");
    program.top("");
    program.top("static void *flip(void *arg) {");
    program.top("    (void)arg;");
    program.top("    WRITE_ONCE(started, 1);");
    program.top("    for (int i = 0; !READ_ONCE(stop); i++) {");
    program.top(format!("        WRITE_ONCE(word, (i & 1) ? ({t})0 : ({t}){all});"));
    program.top("    }");
    program.top("    return 0;");
    program.top("}");
    program.line("pthread_t writer;");
    program.line("pthread_create(&writer, 0, flip, 0);");
    program.line("while (!READ_ONCE(started)) {");
    program.line("}");
    program.line("long long torn = 0;");
    program.line(format!("for (int i = 0; i < {READS}; i++) {{"));
    program.line_at(1, format!("{t} seen = READ_ONCE(word);"));
    program.line_at(1, format!("if (seen != 0 && seen != ({t}){all}) {{"));
    program.line_at(2, "torn++;");
    program.line_at(1, "}");
    program.line("}");
    program.line("WRITE_ONCE(stop, 1);");
    program.line("pthread_join(writer, 0);");
    program.blank();
    program.check(Ty::I64, "torn", 0);
}

/// The cases where a store must not reach a location on a read only page.
///
/// `straddle` finds a page boundary inside a static buffer three times the largest page any
/// machine the corpus runs on uses, and puts a pair of fields across it: `field` ends on the
/// last byte of the writable page and `guarded` starts on the first byte of the one `seal` makes
/// read only. The buffer rather than an aligned global, because Mach-O will not align a section
/// to 64K and the page is whatever `sysconf` says it is.
fn sealed(program: &mut Program, shape: &str, ty: Ty) {
    let t = ty.c_name();
    let bytes = ty.bits() / 8;
    program.include("stdint.h");
    program.include("sys/mman.h");
    program.include("unistd.h");
    program.top("struct pair {");
    program.top(format!("    {t} field;"));
    program.top(format!("    {t} guarded;"));
    program.top("};");
    program.top("");
    program.top("static unsigned char arena[3 * 65536];");
    program.top("");
    program.top("static struct pair *straddle(void) {");
    program.top("    uintptr_t page = (uintptr_t)sysconf(_SC_PAGESIZE);");
    program.top(
        "    unsigned char *edge = (unsigned char *)(((uintptr_t)arena + 2 * page - 1) & ~(page - 1));",
    );
    program.top(format!("    return (struct pair *)(edge - {bytes});"));
    program.top("}");
    program.top("");
    program.top("static int seal(struct pair *p) {");
    program.top("    uintptr_t page = (uintptr_t)sysconf(_SC_PAGESIZE);");
    program.top("    return mprotect(&p->guarded, (size_t)page, PROT_READ);");
    program.top("}");
    program.line(format!("{t} seed = seed_in;"));
    program.line("struct pair *p = straddle();");
    program.line(format!("p->guarded = ({t})(seed + 3);"));
    program.line(format!("p->field = ({t})(seed + 1);"));
    program.line("int sealed = seal(p);");
    program.blank();
    let guarded = ty.convert(SEED + 3);
    let mut field = ty.convert(SEED + 1);
    match shape {
        "guarded-store" => {
            // Not taken. A compiler that stores unconditionally and puts the old value back on
            // the arm that should not store has invented a store, and here that store faults.
            program.line("if (seed > 200) {");
            program.line_at(1, "p->guarded = seed;");
            program.line("}");
        }
        "guarded-loop" => {
            // The store motion case. Moving the conditional store out of the loop as a load
            // before it and a store after it is what `-fno-allow-store-data-races` forbids, and
            // the kernel builds with it.
            program.line(format!("{t} data[64];"));
            program.line("for (int i = 0; i < 64; i++) {");
            program.line_at(1, format!("data[i] = ({t})((i * 37 + seed) & 127);"));
            program.line("}");
            program.line("for (int i = 0; i < 64; i++) {");
            program.line_at(1, "if (data[i] > 127) {");
            program.line_at(2, "p->guarded = data[i];");
            program.line_at(1, "}");
            program.line("}");
        }
        "narrow-field" => {
            // A store and an update of the field next to the sealed one. Either done as a wider
            // access that carries the neighbour along is a store to the sealed page.
            program.line(format!("p->field = ({t})(seed * 5);"));
            program.line("p->field += 1;");
            field = ty.convert(SEED * 5 + 1);
        }
        _ => {
            // The `memory-barriers.txt` shape. The store to the sealed field depends on the
            // load, so it may not be hoisted above the branch or made into a select that
            // writes back what was there, and the other arm's store is to a different place.
            program.line(format!("{t} gate = READ_ONCE(seed_in);"));
            program.line("if (gate > 200) {");
            program.line_at(1, "WRITE_ONCE(p->guarded, 1);");
            program.line("} else {");
            program.line_at(1, "WRITE_ONCE(p->field, 2);");
            program.line("}");
            field = 2;
        }
    }
    program.blank();
    program.check(Ty::I32, "sealed", 0);
    program.check(ty, "p->guarded", guarded);
    program.check(ty, "p->field", field);
}

/// The mitigations the kernel builds with, and the flags that ask for each one.
///
/// `kernel` is all of them at once with `-fno-jump-tables`, which is what an x86-64 build with
/// `CONFIG_MITIGATION_RETPOLINE`, `CONFIG_MITIGATION_RETHUNK`, `CONFIG_MITIGATION_SLS` and
/// `CONFIG_X86_KERNEL_IBT` passes. `-fzero-call-used-regs` is not here yet, since rucc does not
/// have it (tamnd/rucc#2281), and a facet that only says so would be a facet of one failure.
const MITIGATIONS: &[(&str, &[&str])] = &[
    ("retpoline", &["-mindirect-branch=thunk-extern", "-mindirect-branch-register"]),
    ("rethunk", &["-mfunction-return=thunk-extern"]),
    ("sls", &["-mharden-sls=all"]),
    ("ibt", &["-fcf-protection=branch"]),
    (
        "kernel",
        &[
            "-mindirect-branch=thunk-extern",
            "-mindirect-branch-register",
            "-mfunction-return=thunk-extern",
            "-mharden-sls=all",
            "-fcf-protection=branch",
            "-fno-jump-tables",
        ],
    ),
];

/// The registers an indirect branch thunk is named after, which is every one but `rsp`.
const THUNK_REGISTERS: &[&str] = &[
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "r8", "r9", "r10", "r11", "r12", "r13", "r14",
    "r15",
];

/// How many times each case goes round its loop.
const ROUNDS: u32 = 64;

/// The four operations every table of function pointers in the facet holds.
fn operation(op: u32, x: u32, i: u32) -> u32 {
    match op & 3 {
        0 => x.wrapping_add(i.wrapping_mul(3)).wrapping_add(1),
        1 => x ^ i.wrapping_mul(0x9e37_79b9),
        2 => x.wrapping_mul(33).wrapping_add(i),
        _ => x.wrapping_sub(i),
    }
}

/// What the `switch` in the `switch` shape gives for a key.
fn switched(key: u32, x: u32) -> u32 {
    match key & 7 {
        0 => x.wrapping_add(11),
        1 => x ^ 0x5a5a,
        2 => x.wrapping_mul(5),
        3 => x >> 1,
        4 => x.wrapping_add(x >> 3),
        5 => !x,
        6 => x.wrapping_sub(7),
        _ => x.rotate_left(3),
    }
}

/// One case per mitigation and shape.
///
/// `indirect-call` calls through a table of function pointers, `indirect-tail` returns what a
/// call through one returns, which is the one a thunk turns into a jump, and `ops-struct` calls
/// through the members of a structure of them, the way the kernel calls a `file_operations`.
/// `switch` is a switch big enough to be a jump table when the flags allow one, and
/// `early-return` is a function with a return on every arm, each of which becomes a thunk or a
/// trap.
fn mitigations(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] =
        &["indirect-call", "indirect-tail", "ops-struct", "switch", "early-return"];
    for &(mitigation, flags) in MITIGATIONS {
        for &shape in SHAPES {
            if !sink.wants(Facet::Mitigations) {
                return;
            }
            let program = mitigations_program(mitigation, flags, shape);
            let axes = Axes::of([("mitigation", mitigation), ("shape", shape)]);
            let (source, expected) = program.finish();
            let case = Case::linked(
                Facet::Mitigations,
                axes,
                Dialect::C17,
                source,
                Vec::new(),
                flags.iter().map(|flag| (*flag).to_owned()).collect(),
                Expect::Output(expected),
            )
            .tagged(&["gnu", "x86-64", PROVENANCE]);
            sink.push_case(case);
        }
    }
}

/// One `mitigations` case.
fn mitigations_program(mitigation: &str, flags: &[&str], shape: &str) -> Program {
    let mut program = Program::new(format!(
        "{shape} built with the {mitigation} mitigation, after arch/x86/Makefile in Linux 7.2"
    ));
    let thunks = flags.iter().any(|flag| flag.starts_with("-mindirect-branch="));
    let returns = flags.iter().any(|flag| flag.starts_with("-mfunction-return="));
    if thunks || returns {
        program.top("__asm__(");
        program.top("    \".text\\n\"");
        if thunks {
            for register in THUNK_REGISTERS {
                program.top(format!("    \".globl __x86_indirect_thunk_{register}\\n\""));
                program.top(format!("    \"__x86_indirect_thunk_{register}:\\n\""));
                program.top(format!("    \"\\tjmp *%{register}\\n\""));
            }
        }
        if returns {
            program.top("    \".globl __x86_return_thunk\\n\"");
            program.top("    \"__x86_return_thunk:\\n\"");
            program.top("    \"\\tret\\n\"");
        }
        program.top(");");
        program.top("");
    }
    program.top(format!("static volatile unsigned int seed_in = {SEED};"));
    program.top("");
    let ops = [
        "return x + i * 3u + 1u;",
        "return x ^ (i * 0x9e3779b9u);",
        "return x * 33u + i;",
        "return x - i;",
    ];
    if shape != "switch" && shape != "early-return" {
        for (at, body) in ops.iter().enumerate() {
            program.top("__attribute__((noinline))");
            program.top(format!("static unsigned int op{at}(unsigned int x, unsigned int i) {{"));
            program.top(format!("    {body}"));
            program.top("}");
        }
        program.top("");
    }
    let seed = u32::try_from(SEED).unwrap_or(0);
    let mut acc = seed;
    match shape {
        "indirect-call" => {
            program.top("static unsigned int (*const table[4])(unsigned int, unsigned int) = {");
            program.top("    op0, op1, op2, op3,");
            program.top("};");
            program.line("unsigned int acc = seed_in;");
            program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
            program.line_at(1, "acc = table[(acc + i) & 3](acc, i);");
            program.line("}");
            for i in 0..ROUNDS {
                acc = operation(acc.wrapping_add(i), acc, i);
            }
        }
        "indirect-tail" => {
            program.top("static unsigned int (*table[4])(unsigned int, unsigned int) = {");
            program.top("    op0, op1, op2, op3,");
            program.top("};");
            program.top("");
            program.top("__attribute__((noinline))");
            program.top(
                "static unsigned int dispatch(unsigned int op, unsigned int x, unsigned int i) {",
            );
            program.top("    return table[op & 3](x, i);");
            program.top("}");
            program.line("unsigned int acc = seed_in;");
            program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
            program.line_at(1, "acc = dispatch(acc ^ i, acc, i);");
            program.line("}");
            for i in 0..ROUNDS {
                acc = operation(acc ^ i, acc, i);
            }
        }
        "ops-struct" => {
            program.top("struct ops {");
            program.top("    unsigned int (*open)(unsigned int, unsigned int);");
            program.top("    unsigned int (*read)(unsigned int, unsigned int);");
            program.top("};");
            program.top("");
            program.top("static const struct ops first = { op0, op1 };");
            program.top("static const struct ops second = { op2, op3 };");
            program.top("");
            program.top("__attribute__((noinline))");
            program.top(
                "static unsigned int use(const struct ops *o, unsigned int x, unsigned int i) {",
            );
            program.top("    x = o->open(x, i);");
            program.top("    return o->read(x, i + 1u);");
            program.top("}");
            program.line("unsigned int acc = seed_in;");
            program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
            program.line_at(1, "acc = use((acc & 1u) ? &second : &first, acc, i);");
            program.line("}");
            for i in 0..ROUNDS {
                let base = if acc & 1 == 1 { 2 } else { 0 };
                acc = operation(base, acc, i);
                acc = operation(base + 1, acc, i + 1);
            }
        }
        "switch" => {
            program.top("__attribute__((noinline))");
            program.top("static unsigned int pick(unsigned int key, unsigned int x) {");
            program.top("    switch (key & 7u) {");
            let arms = [
                "return x + 11u;",
                "return x ^ 0x5a5au;",
                "return x * 5u;",
                "return x >> 1;",
                "return x + (x >> 3);",
                "return ~x;",
                "return x - 7u;",
            ];
            for (key, arm) in arms.iter().enumerate() {
                program.top(format!("    case {key}:"));
                program.top(format!("        {arm}"));
            }
            program.top("    default:");
            program.top("        return (x << 3) | (x >> 29);");
            program.top("    }");
            program.top("}");
            program.line("unsigned int acc = seed_in;");
            program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
            program.line_at(1, "acc = pick(acc + i, acc) + i;");
            program.line("}");
            for i in 0..ROUNDS {
                acc = switched(acc.wrapping_add(i), acc).wrapping_add(i);
            }
        }
        _ => {
            program.top("__attribute__((noinline))");
            program.top("static unsigned int settle(unsigned int x, unsigned int i) {");
            program.top("    if (x < 1000u) {");
            program.top("        return x * 3u + i;");
            program.top("    }");
            program.top("    if ((x & 15u) == 3u) {");
            program.top("        return x >> 2;");
            program.top("    }");
            program.top("    if (i & 1u) {");
            program.top("        return x - i;");
            program.top("    }");
            program.top("    return (x >> 1) + i;");
            program.top("}");
            program.line("unsigned int acc = seed_in;");
            program.line(format!("for (unsigned int i = 0; i < {ROUNDS}u; i++) {{"));
            program.line_at(1, "acc = settle(acc, i);");
            program.line("}");
            for i in 0..ROUNDS {
                acc = if acc < 1000 {
                    acc.wrapping_mul(3).wrapping_add(i)
                } else if acc & 15 == 3 {
                    acc >> 2
                } else if i & 1 == 1 {
                    acc.wrapping_sub(i)
                } else {
                    (acc >> 1).wrapping_add(i)
                };
            }
        }
    }
    program.blank();
    program.check(Ty::U32, "acc", i128::from(acc));
    program
}

#[cfg(test)]
mod tests {
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_mitigation_has_every_shape_and_carries_its_flags() {
        let manifest = generate(&Options::all()).unwrap();
        let cases: Vec<_> =
            manifest.cases.iter().filter(|case| case.facet == Facet::Mitigations).collect();
        assert_eq!(cases.len(), 25);
        for case in cases {
            assert!(!case.flags.is_empty(), "{}", case.id);
            assert!(case.tags.iter().any(|tag| tag == "x86-64"), "{}", case.id);
            let thunks = case.flags.iter().any(|flag| flag.starts_with("-mindirect-branch="));
            assert_eq!(thunks, case.source.contains("__x86_indirect_thunk_r15:"), "{}", case.id);
        }
    }

    #[test]
    fn every_width_of_every_shape_is_there() {
        let manifest = generate(&Options::all()).unwrap();
        let cases: Vec<_> =
            manifest.cases.iter().filter(|case| case.facet == Facet::Lkmm).collect();
        assert_eq!(cases.len(), 24);
    }

    #[test]
    fn every_sealed_case_gives_the_guarded_field_its_value_before_sealing_it() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest.cases.iter().filter(|case| case.facet == Facet::Lkmm) {
            if let Some(seal) = case.source.find("int sealed = seal(p);") {
                let first = case.source.find("p->guarded = (").unwrap();
                assert!(first < seal, "{} writes the guarded field after sealing it", case.id);
            }
        }
    }

    #[test]
    fn every_case_says_where_it_came_from_and_that_it_needs_headers() {
        let manifest = generate(&Options::all()).unwrap();
        for case in manifest.cases.iter().filter(|case| case.facet == Facet::Lkmm) {
            assert!(case.tags.iter().any(|tag| tag == "provenance:linux"), "{}", case.id);
            assert!(case.tags.iter().any(|tag| tag == "headers"), "{}", case.id);
        }
    }
}
