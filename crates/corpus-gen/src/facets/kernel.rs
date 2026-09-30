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
//! Each case names the kernel file whose idiom it has the shape of in its first line, and never
//! copies code from it.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

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

#[cfg(test)]
mod tests {
    use crate::{Options, generate};
    use corpus_model::Facet;

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
