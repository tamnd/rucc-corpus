//! Constructs Postgres leans on that nothing else in the corpus covered closely.
//!
//! Most are correctness facets, and all of them are here because building Postgres with rucc
//! found them. None is an optimization. Each is a promise the compiler makes to the program, and
//! Postgres relies on it in thousands of places, so getting it wrong in one shape is enough to
//! break the server in a way no test inside Postgres points at.
//!
//! `sigsetjmp` is the error handling. `PG_TRY` saves a `sigjmp_buf` that the function owns,
//! `elog(ERROR)` jumps back to it from however deep the error was raised, `PG_CATCH` runs on
//! the second return, and `PG_RE_THROW` jumps on to the handler outside. Every value the catch
//! arm reads was set before the save, and the back end has to have kept it where the second
//! return looks for it. `setjmp-longjmp` asks whether the jump works at all. This facet asks
//! whether it works with many values alive across it, some in registers the jump restores, some
//! in the frame, and an arm that ran first and wanted the same registers and slots for itself.
//!
//! `builtin-setjmp` is the same error handling as MinGW builds it. There the `sigsetjmp` in
//! `PG_TRY` is `__builtin_setjmp` on a buffer of five words, and the `siglongjmp` in
//! `elog(ERROR)` is `__builtin_longjmp` with a second argument of 1, because the C library's
//! `setjmp` on Windows unwinds through frames it has no tables for. The compiler writes both
//! itself, so the save, the jump and where the values are when it lands are all its own work.
//! It has the shapes `sigsetjmp` has, a jump from the bottom of a deep chain of calls, and a
//! buffer that is a field of a global struct rather than a local.
//!
//! `overflow-builtins` is the arithmetic. `common/int.h` checks every add, subtract and
//! multiply that could overflow with `__builtin_add_overflow` and its two siblings, over
//! `int16`, `int32`, `int64`, their unsigned forms and `int128`. The builtins take the exact
//! result, store it wrapped into the type the third argument points at, and return whether it
//! fitted, and the three types involved are independent of each other.
//!
//! `target-attribute` is how Postgres uses an instruction set extension without asking for it
//! on the command line: a function marked `__attribute__((target(...)))`, called only after
//! `__builtin_cpu_supports` or `cpuid` said the processor has it, with a plain version beside
//! it. `crc32c` is the one Postgres ships, the SSE4.2 `crc32` instructions against slicing by
//! eight, over every alignment and length the hardware loop splits on. `simd-lfind` is the SSE2
//! search loops from `port/simd.h` and `pg_lfind.h`. Every program in these three works on any
//! x86-64 processor, because it asks before it takes the fast path and prints the same answer
//! either way.
//!
//! `frame-size` is the back end one: functions with many locals, big arrays and structs in the
//! frame, and calls out of it, with an answer simple enough to check, to hold the frame layout
//! against what `gcc -fstack-usage` reports for the same code.
//!
//! `interpreter-dispatch` is the other back end one. `ExecInterpExpr` runs every expression a
//! query evaluates as a loop of handlers reached through `goto *`, with the state, the step and
//! a dozen more values alive across every jump and every call a handler makes. The same
//! interpreter built over a `switch` sits beside it in each program and has to agree with it.
//!
//! `crc32c-armv8` and `simd-lfind-neon` are the AArch64 halves of `crc32c` and `simd-lfind`,
//! the ARMv8 CRC instructions from `arm_acle.h` and the NEON paths through `port/simd.h`. They
//! live in [`arm`].
//!
//! `bundle` is the shape every extension has: an executable and the modules it opens with
//! `dlopen`, which call back into it for functions, globals and hooks it defines. Its cases are
//! more than one file, one executable and one or more modules, and carry the `dlopen` tag so a
//! run on Windows, where a module is built another way, can leave them out. They live in
//! [`bundle`].
//!
//! `dllimport` is the same boundary as Windows builds it. A module reaches the server's data and
//! functions through `PGDLLIMPORT` and an import library, and the server finds what the module
//! marked `PGDLLEXPORT`. Its cases load their modules with `LoadLibraryA` there and with
//! `dlopen` anywhere else, so they carry no `dlopen` tag. They live in [`dllimport`].
//!
//! Every case here carries the `provenance:postgres` tag, so a run can pick them out. The ones
//! that only build on x86-64 also carry `x86-64`, and the ones that only build on AArch64 carry
//! `aarch64`, so a run elsewhere can leave them out.

mod arm;
mod bundle;
mod dllimport;

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};
use std::fmt::Write as _;

/// The tag every case in this module carries.
const PROVENANCE: &str = "provenance:postgres";

/// Emits everything in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    sigsetjmp(sink);
    builtin_setjmp(sink);
    overflow_builtins(sink);
    target_attribute(sink);
    crc32c_facet(sink);
    simd_lfind(sink);
    arm::generate(sink);
    frame_size(sink);
    interpreter_dispatch(sink);
    bundle::generate(sink);
    dllimport::generate(sink);
}

/// The opaque number every value in a `sigsetjmp` case is computed from.
const SEED: i128 = 3;

/// How many cells the pointer values point into.
const CELLS: i128 = 64;

/// What a cell holds, which is what a pointer value prints.
const fn cell(index: i128) -> i128 {
    index * 7 + 1
}

/// The four kinds of value a case keeps alive, handed out in turn.
///
/// They are the four that end up in different places. An `int` and a `long` go in general
/// registers, a `double` goes in a vector register that no x86-64 call preserves, so it is in
/// the frame across every call whatever else is going on, and a pointer is a general register
/// whose value is only checked through what it points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Int,
    Long,
    Double,
    Pointer,
}

const KINDS: [Kind; 4] = [Kind::Int, Kind::Long, Kind::Double, Kind::Pointer];

impl Kind {
    /// The kind of the value at one position.
    const fn at(index: usize) -> Self {
        KINDS[index % KINDS.len()]
    }
}

/// One value set before the save and read after it.
struct Kept {
    kind: Kind,
    name: String,
    /// Whether the arm that runs first writes it. Only a `volatile` one may be, because a local
    /// that is not volatile and was changed between the save and the jump is indeterminate
    /// afterwards, and a case with no defined answer is not a case.
    written: bool,
    /// Where it starts, in the form it is printed in.
    start: i128,
    /// Where a pointer starts, as an index into the cells.
    slot: i128,
}

impl Kept {
    fn new(index: usize) -> Self {
        let kind = Kind::at(index);
        let k = index as i128;
        let slot = (SEED + k) % 32;
        let start = match kind {
            Kind::Int => SEED * (k + 2) + k,
            Kind::Long => SEED * (1000 + k) - k,
            // Printed four times over, so a quarter step survives as an integer.
            Kind::Double => SEED * 4 + k + 1,
            Kind::Pointer => cell(slot),
        };
        Self { kind, name: format!("v{index}"), written: index % 3 == 2, start, slot }
    }

    /// The declaration, which sets it from the seed so that none of it folds to a constant.
    fn declaration(&self, index: usize) -> String {
        let volatile = if self.written { "volatile " } else { "" };
        let name = &self.name;
        match self.kind {
            Kind::Int => format!("{volatile}int {name} = seed * {} + {index};", index + 2),
            Kind::Long => format!("{volatile}long {name} = seed * {}L - {index};", 1000 + index),
            Kind::Double => format!("{volatile}double {name} = seed + 0.25 * {};", index + 1),
            Kind::Pointer => {
                let volatile = if self.written { "volatile " } else { "" };
                format!("int *{volatile}{name} = &cells[(seed + {index}) % 32];")
            }
        }
    }

    /// The statement the first arm writes it with.
    fn update(&self, index: usize) -> String {
        let name = &self.name;
        match self.kind {
            Kind::Int => format!("{name} = {name} + {};", 100 + index),
            Kind::Long => format!("{name} = {name} * 2;"),
            Kind::Double => format!("{name} = {name} + 1.5;"),
            Kind::Pointer => format!("{name} = {name} + 1;"),
        }
    }

    /// What it prints after the first arm has run `times` times.
    fn after(&self, index: usize, times: u32) -> i128 {
        let times_wide = i128::from(times);
        if !self.written {
            return self.start;
        }
        match self.kind {
            Kind::Int => self.start + times_wide * (100 + index as i128),
            Kind::Long => self.start << times,
            Kind::Double => self.start + 6 * times_wide,
            Kind::Pointer => cell(self.slot + times_wide),
        }
    }

    /// An integer expression that reads it, and what that reads before anything writes it.
    fn term(&self) -> (String, i128) {
        let name = &self.name;
        let expr = match self.kind {
            Kind::Int | Kind::Long => name.clone(),
            Kind::Double => format!("(long long)({name} * 4.0)"),
            Kind::Pointer => format!("*{name}"),
        };
        (expr, self.start)
    }
}

/// The values the arm that runs first works out for itself around the call that raises.
///
/// This is the half of the facet that tamnd/rucc#2018 was about. The first arm reads the kept
/// values that nobody writes, works out a crowd of its own from them, and keeps every one of
/// those alive across the call that raises, so they want registers and, once there are enough
/// of them, frame slots. None of the kept values is read again on the path the first return
/// takes, so an allocator that only looks along that path sees them die here and hands their
/// slots to these. The second return then reads a kept value out of a slot this arm overwrote.
///
/// Returns the sum the arm stores before the call and adds again after it, which is what the
/// arm leaves in `busy_total` when it runs to the end, and half of it when the call raises.
fn busy_arm(program: &mut Program, depth: usize, count: usize, kept: &[Kept], call: &str) -> i128 {
    let mut total = 0;
    for index in 0..count {
        let j = index as i128;
        // A kept value nobody writes, so reading it here is reading a value C still defines.
        let (term, value) = kept.get(index % kept.len()).filter(|value| !value.written).map_or(
            (String::new(), 0),
            |value| {
                let (expr, value) = value.term();
                (format!(" + {expr}"), value)
            },
        );
        let line = match Kind::at(index) {
            Kind::Int => {
                total += SEED * (j + 13) + 2 * j + value;
                format!("int b{index} = seed * {} + {}{term};", index + 13, 2 * index)
            }
            Kind::Long => {
                total += SEED * (2000 + j) + j + value;
                format!("long b{index} = seed * {}L + {index}{term};", 2000 + index)
            }
            Kind::Double => {
                total += SEED + 2 * j + 2 * value;
                format!("double b{index} = seed * 0.5 + {index}{term};")
            }
            Kind::Pointer => {
                total += cell((SEED * 3 + j + value) % 32);
                format!("int *b{index} = &cells[(seed * 3 + {index}{term}) % 32];")
            }
        };
        program.line_at(depth, line);
    }
    let sum = |program: &mut Program, name: &str| {
        program.line_at(depth, format!("long long {name} = 0;"));
        for index in 0..count {
            let term = match Kind::at(index) {
                Kind::Int | Kind::Long => format!("b{index}"),
                Kind::Double => format!("(long long)(b{index} * 2.0)"),
                Kind::Pointer => format!("*b{index}"),
            };
            program.line_at(depth, format!("{name} = {name} + {term};"));
        }
    };
    program.line_at(depth, "work(0);");
    sum(program, "busy");
    program.line_at(depth, "busy_total = busy;");
    program.line_at(depth, call);
    sum(program, "again");
    program.line_at(depth, "busy_total = busy_total + again;");
    total
}

/// Adds every kept value into the running record of what the handler saw.
///
/// The handler is the only place the kept values are read after the save, which is the shape
/// `PG_CATCH` has and the shape that lets them look dead to the arm that ran first.
fn record(program: &mut Program, depth: usize, kept: &[Kept]) {
    for (index, value) in kept.iter().enumerate() {
        let (expr, _) = value.term();
        program.line_at(depth, format!("seen[{index}] = seen[{index}] + {expr};"));
    }
}

/// Locals live across a `sigsetjmp`, read after the `siglongjmp` back to it.
///
/// The axes are the ones that decide where a value lives when the jump lands. `live` is how
/// many values are set before the save and read after it, from one, which stays in a register,
/// to thirty, which is far more than the six registers x86-64 preserves across a call, so most
/// of them are in the frame. `first-arm` is whether the arm that runs before the jump works out
/// a crowd of values of its own. `shape` is the way Postgres uses the save: a catch, a catch
/// that rethrows to an outer one, a catch inside a loop, and a try that finishes without an
/// error at all. `api` is `sigsetjmp` with a mask of zero, which is what `PG_TRY` writes, and
/// plain `setjmp`, which has the same rules.
///
/// Only what C defines is checked. A value is either never written after the save, in which
/// case it has to still hold what it held, or it is `volatile`, in which case it has to hold
/// what was last written to it. One in three is the second sort.
fn sigsetjmp(sink: &mut Sink<'_>) {
    const APIS: &[&str] = &["sigsetjmp", "setjmp"];
    const SHAPES: &[&str] = &["catch", "rethrow", "loop", "no-error"];
    const LIVE: &[usize] = &[1, 2, 4, 7, 12, 20, 30];
    const ARMS: &[&str] = &["quiet", "busy"];
    for &api in APIS {
        for &shape in SHAPES {
            for &live in LIVE {
                for &arm in ARMS {
                    if !sink.wants(Facet::Sigsetjmp) {
                        return;
                    }
                    let program = sigsetjmp_program(api, shape, live, arm == "busy");
                    let axes = Axes::of([
                        ("api", api),
                        ("shape", shape),
                        ("live", &live.to_string()),
                        ("first-arm", arm),
                    ]);
                    sink.push_tagged(
                        Facet::Sigsetjmp,
                        axes,
                        Dialect::C17,
                        program,
                        &["headers", PROVENANCE],
                    );
                }
            }
        }
    }
}

/// Locals live across a `__builtin_setjmp`, read after the `__builtin_longjmp` back to it.
///
/// The axes are the ones `sigsetjmp` has, with fewer points on `live`, and two more shapes.
/// `deep` raises from the bottom of a recursive chain of a dozen calls, so the jump throws
/// away a dozen frames rather than two. `global` saves into a buffer that is a field of a
/// static struct, after an `int`, the way a handler kept in a global would be, so the save and
/// the jump reach the buffer through an address that is not the frame's.
///
/// The buffer is five pointers, as GCC and MinGW Postgres have it. The jump is always in a
/// function other than the one that saved, and its second argument is always the constant 1,
/// since those are the only terms the builtins are defined on, and the function that jumps is
/// never inlined, so it stays that way at every level.
fn builtin_setjmp(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &["catch", "rethrow", "loop", "no-error", "deep", "global"];
    const LIVE: &[usize] = &[1, 4, 12, 30];
    const ARMS: &[&str] = &["quiet", "busy"];
    for &shape in SHAPES {
        for &live in LIVE {
            for &arm in ARMS {
                if !sink.wants(Facet::BuiltinSetjmp) {
                    return;
                }
                let program = sigsetjmp_program("builtin", shape, live, arm == "busy");
                let axes =
                    Axes::of([("shape", shape), ("live", &live.to_string()), ("first-arm", arm)]);
                sink.push_tagged(
                    Facet::BuiltinSetjmp,
                    axes,
                    Dialect::C17,
                    program,
                    &["gnu", PROVENANCE],
                );
            }
        }
    }
}

/// One `sigsetjmp` case, or with `api` set to `builtin`, one `builtin-setjmp` case.
fn sigsetjmp_program(api: &str, shape: &str, live: usize, busy: bool) -> Program {
    let posix = api == "sigsetjmp";
    let builtin = api == "builtin";
    let (buffer, save, jump) = if posix {
        ("sigjmp_buf", "sigsetjmp", "siglongjmp")
    } else if builtin {
        ("builtin_jmp_buf", "__builtin_setjmp", "__builtin_longjmp")
    } else {
        ("jmp_buf", "setjmp", "longjmp")
    };
    let saved = |name: &str| {
        if posix { format!("{save}({name}, 0)") } else { format!("{save}({name})") }
    };
    let mut program = Program::new(format!(
        "{live} value{} live across {} in the {shape} shape, with a {} first arm",
        if live == 1 { "" } else { "s" },
        if builtin { save } else { api },
        if busy { "busy" } else { "quiet" }
    ));
    if posix {
        // With -std=c17 the C library shows only ISO C, and sigsetjmp is POSIX.
        program.define("_POSIX_C_SOURCE", "200809L");
    }
    if builtin {
        program.top("typedef void *builtin_jmp_buf[5];");
    } else {
        program.include("setjmp.h");
    }
    program.top(format!("static {buffer} *exception_stack;"));
    program.top("static volatile int depth;");
    program.top("static volatile long long busy_total;");
    program.top(format!("static int cells[{CELLS}];"));
    program.top(format!("static volatile long long seen[{live}];"));
    if shape == "global" {
        program.top(format!("static struct {{ int active; {buffer} buf; }} handler;"));
    }
    if builtin {
        // The builtins only allow a jump from another function and a value of 1, and the
        // code only ever says that there was an error, so it goes no further.
        program.top("static __attribute__((noinline)) void raise_error(int code) {");
        program.top("    (void)code;");
        program.top(format!("    {jump}(*exception_stack, 1);"));
    } else {
        program.top("static void raise_error(int code) {");
        program.top(format!("    {jump}(*exception_stack, code);"));
    }
    program.top("}");
    program.top("static void work(int code) {");
    program.top("    depth = depth + 1;");
    program.top("    if (code != 0) {");
    program.top("        raise_error(code);");
    program.top("    }");
    program.top("}");
    if shape == "deep" {
        // The volatile local is read after the call returns, so every level keeps a frame
        // of its own rather than the recursion becoming a loop.
        program.top("static int descend(int levels, int code) {");
        program.top("    volatile int frame = levels;");
        program.top("    if (levels == 0) {");
        program.top("        work(code);");
        program.top("    } else {");
        program.top("        descend(levels - 1, code);");
        program.top("    }");
        program.top("    return frame;");
        program.top("}");
    }
    program.input(Ty::I32, "seed", SEED);
    program.line(format!("for (int i = 0; i < {CELLS}; i++) {{"));
    program.line_at(1, "cells[i] = i * 7 + 1;");
    program.line("}");
    program.blank();

    let kept: Vec<Kept> = (0..live).map(Kept::new).collect();
    for (index, value) in kept.iter().enumerate() {
        program.line(value.declaration(index));
    }
    program.line("volatile int caught = 0;");
    program.line("volatile int progress = 0;");
    if shape == "rethrow" {
        program.line("volatile int cleanups = 0;");
    }
    program.blank();

    let busy_count = live.max(8);
    let mut busy_value = 0;
    let mut first_arm = |program: &mut Program, depth: usize, call: &str| {
        for (index, value) in kept.iter().enumerate() {
            if value.written {
                program.line_at(depth, value.update(index));
            }
        }
        if busy {
            busy_value = busy_arm(program, depth, busy_count, &kept, call);
        } else {
            program.line_at(depth, call);
        }
    };

    // How many times the first arm had run each time the values were recorded, what it left
    // progress at, how many errors reached the outermost handler, how many times work was
    // called, and how many times over the busy sum ended up in busy_total.
    let (recorded, progress, caught, works, sums): (&[u32], _, _, _, _) = match shape {
        "deep" | "global" => {
            // A catch like the first, where the error comes from a dozen calls down or the
            // buffer is the global handler's.
            let global = shape == "global";
            let target = if global { "handler.buf" } else { "local_buf" };
            program.line(format!("{buffer} *save_stack = exception_stack;"));
            if !global {
                program.line(format!("{buffer} local_buf;"));
            }
            program.line(format!("if ({} == 0) {{", saved(target)));
            program.line_at(1, format!("exception_stack = &{target};"));
            program.line_at(1, "progress = 1;");
            let call = if global { "work(5);" } else { "descend(12, 5);" };
            first_arm(&mut program, 1, call);
            program.line_at(1, "progress = 2;");
            program.line("} else {");
            program.line_at(1, "exception_stack = save_stack;");
            program.line_at(1, "caught = caught + 1;");
            if global {
                program.line_at(1, "handler.active = handler.active + 1;");
            }
            record(&mut program, 1, &kept);
            program.line("}");
            program.line("exception_stack = save_stack;");
            (&[1], 1, 1, 1 + i128::from(busy), 1)
        }
        "catch" | "no-error" => {
            let raises = shape == "catch";
            program.line(format!("{buffer} *save_stack = exception_stack;"));
            program.line(format!("{buffer} local_buf;"));
            program.line(format!("if ({} == 0) {{", saved("local_buf")));
            program.line_at(1, "exception_stack = &local_buf;");
            program.line_at(1, "progress = 1;");
            first_arm(&mut program, 1, &format!("work({});", if raises { 5 } else { 0 }));
            program.line_at(1, "progress = 2;");
            program.line("} else {");
            program.line_at(1, "exception_stack = save_stack;");
            program.line_at(1, "caught = caught + 1;");
            if raises {
                record(&mut program, 1, &kept);
            }
            program.line("}");
            program.line("exception_stack = save_stack;");
            if !raises {
                // PG_END_TRY with nothing raised, where the values are read on the path the
                // first return took.
                record(&mut program, 0, &kept);
            }
            let works = 1 + i128::from(busy);
            (
                &[1],
                if raises { 1 } else { 2 },
                i128::from(raises),
                works,
                if raises { 1 } else { 2 },
            )
        }
        "rethrow" => {
            // PG_RE_THROW from inside a nested PG_TRY. The inner catch puts the outer buffer
            // back, cleans up and jumps on, so the values are read by a handler two saves out
            // from the arm that raised the error.
            program.line(format!("{buffer} *save_stack = exception_stack;"));
            program.line(format!("{buffer} outer_buf;"));
            program.line(format!("if ({} == 0) {{", saved("outer_buf")));
            program.line_at(1, "exception_stack = &outer_buf;");
            program.line_at(1, "progress = 1;");
            program.line_at(1, format!("{buffer} *inner_save = exception_stack;"));
            program.line_at(1, format!("{buffer} inner_buf;"));
            program.line_at(1, format!("if ({} == 0) {{", saved("inner_buf")));
            program.line_at(2, "exception_stack = &inner_buf;");
            program.line_at(2, "progress = 2;");
            first_arm(&mut program, 2, "work(7);");
            program.line_at(2, "progress = 3;");
            program.line_at(1, "} else {");
            program.line_at(2, "exception_stack = inner_save;");
            program.line_at(2, "cleanups = cleanups + 1;");
            program.line_at(2, "raise_error(1);");
            program.line_at(1, "}");
            program.line_at(1, "progress = 4;");
            program.line("} else {");
            program.line_at(1, "exception_stack = save_stack;");
            program.line_at(1, "caught = caught + 1;");
            record(&mut program, 1, &kept);
            program.line("}");
            program.line("exception_stack = save_stack;");
            (&[1], 2, 1, 1 + i128::from(busy), 1)
        }
        _ => {
            // A PG_TRY in a loop body, where every other pass raises. Each pass is a new save,
            // and round is not written between a save and the jump back to it, so reading it
            // on the second return is defined. The handler runs on the second pass and the
            // fourth, by which time the first arm has run two times and four.
            program.line("for (int round = 0; round < 4; round++) {");
            program.line_at(1, format!("{buffer} *save_stack = exception_stack;"));
            program.line_at(1, format!("{buffer} local_buf;"));
            program.line_at(1, format!("if ({} == 0) {{", saved("local_buf")));
            program.line_at(2, "exception_stack = &local_buf;");
            program.line_at(2, "progress = progress + 1;");
            first_arm(&mut program, 2, "work(round % 2);");
            program.line_at(1, "} else {");
            program.line_at(2, "exception_stack = save_stack;");
            program.line_at(2, "caught = caught + 1;");
            record(&mut program, 2, &kept);
            program.line_at(1, "}");
            program.line_at(1, "exception_stack = save_stack;");
            program.line("}");
            // The last pass raises, so the sum after the call never got added.
            (&[2, 4], 4, 2, 4 * (1 + i128::from(busy)), 1)
        }
    };
    program.blank();

    for (index, value) in kept.iter().enumerate() {
        let total = recorded.iter().map(|&times| value.after(index, times)).sum();
        program.check(Ty::I64, &format!("seen[{index}]"), total);
    }
    program.check(Ty::I32, "caught", caught);
    program.check(Ty::I32, "progress", progress);
    program.check(Ty::I32, "depth", works);
    if shape == "rethrow" {
        program.check(Ty::I32, "cleanups", 1);
    }
    if shape == "global" {
        program.check(Ty::I32, "handler.active", 1);
    }
    if busy {
        program.check(Ty::I64, "busy_total", busy_value * sums);
    }
    program
}

/// An integer type the overflow builtins take, including the two C does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Int {
    /// The name on an axis.
    name: &'static str,
    /// How C spells it.
    c_name: &'static str,
    bits: u32,
    signed: bool,
}

impl Int {
    /// The type of the same width with the other signedness.
    fn flipped(self) -> Self {
        let at = INTS.iter().position(|ty| *ty == self).unwrap_or(0);
        INTS[at ^ 1]
    }

    /// The signed and the unsigned type at one position of [`WIDTH_RING`].
    fn at(width: usize, signed: bool) -> Self {
        INTS[width % WIDTH_RING * 2 + usize::from(!signed)]
    }

    /// Where its width sits in [`WIDTH_RING`].
    fn width(self) -> usize {
        INTS.iter().position(|ty| *ty == self).unwrap_or(0) / 2
    }

    /// The highest value, as a magnitude.
    fn max(self) -> u128 {
        let bits = if self.signed { self.bits - 1 } else { self.bits };
        if bits == 128 { u128::MAX } else { (1u128 << bits) - 1 }
    }

    /// The magnitude of the lowest value, which is zero for an unsigned type.
    fn min_magnitude(self) -> u128 {
        if self.signed { 1u128 << (self.bits - 1) } else { 0 }
    }

    /// Whether the type only has the width it is given here on an LP64 target.
    fn is_long(self) -> bool {
        self.c_name.ends_with("long") && !self.c_name.ends_with("long long")
    }
}

/// The twelve integer types, signed then unsigned at each width, narrowest first.
///
/// `long` is here because Postgres uses it, and it is the one type in this corpus whose width
/// is the target's choice. It is 64 bits on every LP64 target and 32 on Windows, so every case
/// that uses it carries the `lp64` tag and a run on an LLP64 target leaves those out.
const INTS: [Int; 12] = [
    Int { name: "i8", c_name: "signed char", bits: 8, signed: true },
    Int { name: "u8", c_name: "unsigned char", bits: 8, signed: false },
    Int { name: "i16", c_name: "short", bits: 16, signed: true },
    Int { name: "u16", c_name: "unsigned short", bits: 16, signed: false },
    Int { name: "i32", c_name: "int", bits: 32, signed: true },
    Int { name: "u32", c_name: "unsigned int", bits: 32, signed: false },
    Int { name: "long", c_name: "long", bits: 64, signed: true },
    Int { name: "ulong", c_name: "unsigned long", bits: 64, signed: false },
    Int { name: "i64", c_name: "long long", bits: 64, signed: true },
    Int { name: "u64", c_name: "unsigned long long", bits: 64, signed: false },
    Int { name: "i128", c_name: "__int128", bits: 128, signed: true },
    Int { name: "u128", c_name: "unsigned __int128", bits: 128, signed: false },
];

/// How many widths [`INTS`] walks, counting `long` and `long long` as two.
const WIDTH_RING: usize = INTS.len() / 2;

/// A whole number with no width, which is what the builtins compute before they store.
///
/// Sign and magnitude, with the magnitude in two halves of 128 bits. Every operand fits in 128
/// bits of magnitude, so a product fits in 256 and a sum in 129, and nothing here can overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Exact {
    negative: bool,
    high: u128,
    low: u128,
}

impl Exact {
    /// A value with a sign and a magnitude of at most 128 bits.
    fn of(negative: bool, magnitude: u128) -> Self {
        Self { negative: negative && magnitude != 0, high: 0, low: magnitude }
    }

    fn is_zero(self) -> bool {
        self.high == 0 && self.low == 0
    }

    fn magnitude(self) -> (u128, u128) {
        (self.high, self.low)
    }

    fn negated(self) -> Self {
        Self { negative: !self.negative && !self.is_zero(), ..self }
    }

    fn add(self, other: Self) -> Self {
        let (high, low) = if self.negative == other.negative {
            let (low, carry) = self.low.overflowing_add(other.low);
            (self.high + other.high + u128::from(carry), low)
        } else {
            let (big, small) =
                if self.magnitude() >= other.magnitude() { (self, other) } else { (other, self) };
            let (low, borrow) = big.low.overflowing_sub(small.low);
            let result = Self {
                negative: big.negative,
                high: big.high - small.high - u128::from(borrow),
                low,
            };
            return Self { negative: result.negative && !result.is_zero(), ..result };
        };
        Self { negative: self.negative, high, low }
    }

    fn sub(self, other: Self) -> Self {
        self.add(other.negated())
    }

    /// The product, by schoolbook multiplication over 64 bit halves.
    fn mul(self, other: Self) -> Self {
        assert!(self.high == 0 && other.high == 0, "an operand wider than 128 bits");
        let half = |value: u128| (value >> 64, value & u128::from(u64::MAX));
        let (a1, a0) = half(self.low);
        let (b1, b0) = half(other.low);
        let mut high = a1 * b1;
        let mut low = a0 * b0;
        for cross in [a0 * b1, a1 * b0] {
            let (sum, carry) = low.overflowing_add(cross << 64);
            low = sum;
            high += (cross >> 64) + u128::from(carry);
        }
        let result = Self { negative: self.negative != other.negative, high, low };
        Self { negative: result.negative && !result.is_zero(), ..result }
    }

    /// Whether the value is one the type can hold.
    fn fits(self, ty: Int) -> bool {
        if self.high != 0 {
            return false;
        }
        if self.negative { self.low <= ty.min_magnitude() } else { self.low <= ty.max() }
    }

    /// The low bits of the two's complement form, which is what the builtin stores.
    fn wrapped(self, ty: Int) -> u128 {
        let bits = if self.negative { self.low.wrapping_neg() } else { self.low };
        if ty.bits == 128 { bits } else { bits & ((1u128 << ty.bits) - 1) }
    }
}

/// How a value of a type is written in C, as a constant expression of that value.
///
/// There is no literal wider than `unsigned long long`, so a value past that is built from its
/// two halves with a shift, and a negative one past that is the negation of one less than its
/// magnitude, less one, which never makes a positive value the type cannot hold.
fn int_literal(value: Exact) -> String {
    let positive = |magnitude: u128| {
        if let Ok(small) = u64::try_from(magnitude) {
            format!("{small}ull")
        } else {
            format!(
                "(((unsigned __int128){}ull << 64) | {}ull)",
                magnitude >> 64,
                magnitude & u128::from(u64::MAX)
            )
        }
    };
    if !value.negative {
        return positive(value.low);
    }
    if value.low <= 1u128 << 63 {
        if value.low == 1u128 << 63 {
            return "(-9223372036854775807ll - 1)".to_owned();
        }
        return format!("(-{}ll)", value.low);
    }
    format!("(-(__int128){} - 1)", positive(value.low - 1))
}

/// The operand values for one type and one builtin.
///
/// Five of each, so a case is twenty five calls. For an add or a subtract they are the ends of
/// the range and the values next to zero, which between them reach every way a sum can step
/// over an end: the lowest less one, the highest plus one, zero less the lowest, and an
/// unsigned operand a half range from either end. For a multiply they are the values around
/// the square root of the range, where a product first stops fitting. Two to the half width,
/// less one and plus one, multiply to exactly the unsigned maximum, and the signed values
/// multiply to exactly the signed minimum one way round and overflow by one the other.
fn operands(ty: Int, op: &str) -> Vec<Exact> {
    let bits = ty.bits;
    let top = Exact::of(false, ty.max());
    let one = |negative: bool, magnitude: u128| Exact::of(negative, magnitude);
    let half = 1u128 << (bits / 2);
    match (op, ty.signed) {
        ("mul", true) => vec![
            one(true, ty.min_magnitude()),
            one(true, half),
            one(true, 1),
            one(false, half >> 1),
            one(false, half),
        ],
        ("mul", false) => {
            vec![one(false, 1), one(false, half - 1), one(false, half), one(false, half + 1), top]
        }
        (_, true) => {
            vec![one(true, ty.min_magnitude()), one(true, 1), one(false, 0), one(false, 1), top]
        }
        (_, false) => vec![
            one(false, 0),
            one(false, 1),
            one(false, 1u128 << (bits - 1)),
            one(false, ty.max() - 1),
            top,
        ],
    }
}

/// The two operand types a case pairs with its result type.
///
/// `same` is how `common/int.h` calls them, three of one type. `flipped` has the operands in
/// the other signedness at the same width, so every value has to be converted on the way
/// out. `mixed` has one of each signedness at the result's width. `crossed` has a signed
/// operand one width narrower and an unsigned one a width wider, going round the ring at the
/// ends, so the three types are all different.
fn operand_types(shape: &str, result: Int) -> (Int, Int) {
    match shape {
        "same" => (result, result),
        "flipped" => (result.flipped(), result.flipped()),
        "mixed" => {
            (Int::at(result.width(), result.signed), Int::at(result.width(), !result.signed))
        }
        _ => (Int::at(result.width() + WIDTH_RING - 1, true), Int::at(result.width() + 1, false)),
    }
}

/// The checked arithmetic builtins, over mixed operand and result types.
///
/// One program per builtin, result type and operand shape, and each program checks every pair
/// of its operand values: whether the builtin said it overflowed, and what it stored. The
/// operands are read out of `volatile` arrays, because Postgres checks numbers it parsed, and
/// a compiler that folds the constant case right and has no lowering for the other is exactly
/// what this has to catch.
fn overflow_builtins(sink: &mut Sink<'_>) {
    const OPS: &[&str] = &["add", "sub", "mul"];
    const SHAPES: &[&str] = &["same", "flipped", "mixed", "crossed"];
    for &op in OPS {
        for result in INTS {
            for &shape in SHAPES {
                if !sink.wants(Facet::OverflowBuiltins) {
                    return;
                }
                let (left, right) = operand_types(shape, result);
                let program = overflow_program(op, left, right, result);
                let axes = Axes::of([
                    ("op", op),
                    ("result", result.name),
                    ("shape", shape),
                    ("left", left.name),
                    ("right", right.name),
                ]);
                let mut tags = vec!["gnu", PROVENANCE];
                if [left, right, result].iter().any(|ty| ty.is_long()) {
                    tags.push("lp64");
                }
                sink.push_tagged(Facet::OverflowBuiltins, axes, Dialect::C17, program, &tags);
            }
        }
    }
}

/// One overflow builtin case.
fn overflow_program(op: &str, left: Int, right: Int, result: Int) -> Program {
    let mut program = Program::new(format!(
        "__builtin_{op}_overflow of {} and {} into {}",
        left.c_name, right.c_name, result.c_name
    ));
    let lefts = operands(left, op);
    let rights = operands(right, op);
    let table = |values: &[Exact]| {
        values.iter().map(|value| int_literal(*value)).collect::<Vec<_>>().join(", ")
    };
    program.top(format!(
        "static volatile {} lhs[{}] = {{ {} }};",
        left.c_name,
        lefts.len(),
        table(&lefts)
    ));
    program.top(format!(
        "static volatile {} rhs[{}] = {{ {} }};",
        right.c_name,
        rights.len(),
        table(&rights)
    ));
    program.line(format!("{} result;", result.c_name));
    program.line("int flag;");
    for (i, &a) in lefts.iter().enumerate() {
        program.blank();
        for (j, &b) in rights.iter().enumerate() {
            let exact = match op {
                "add" => a.add(b),
                "sub" => a.sub(b),
                _ => a.mul(b),
            };
            program.line(format!("flag = __builtin_{op}_overflow(lhs[{i}], rhs[{j}], &result);"));
            program.check(Ty::I32, "flag", i128::from(!exact.fits(result)));
            let bits = exact.wrapped(result);
            if result.bits == 128 {
                program.check(Ty::U64, "(unsigned __int128)result >> 64", (bits >> 64) as i128);
                program.check(Ty::U64, "result", (bits & u128::from(u64::MAX)) as i128);
            } else if result.signed {
                let sign = 1u128 << (result.bits - 1);
                let value = if bits & sign == 0 {
                    bits as i128
                } else {
                    bits as i128 - (1i128 << result.bits)
                };
                program.check(Ty::I64, "result", value);
            } else {
                program.check(Ty::U64, "result", bits as i128);
            }
        }
    }
    program
}

/// The tag on every case that only builds and runs on x86-64.
///
/// Everything else in the corpus runs anywhere. These cases are about x86-64 instructions by
/// name, so a run on another target leaves them out with `--exclude-tag x86-64`.
const X86_64: &str = "x86-64";

/// One step of the 32 bit linear congruential generator the cases fill their buffers with.
const fn lcg(x: u32) -> u32 {
    x.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

/// The same step, in C, on an `unsigned int` called `x`.
const LCG_C: &str = "x = x * 1103515245u + 12345u;";

/// Writes `cpu_has_feature`, which asks the processor the way the program is built to ask.
///
/// `builtin` is `__builtin_cpu_supports`, and `cpuid` is `__get_cpuid` from `cpuid.h` with the
/// bit tested by hand, which is what Postgres does in `pg_crc32c_sse42_choose.c`. Both return
/// zero on a processor without the feature, and the program then takes its portable path and
/// prints the same thing.
fn cpu_check(program: &mut Program, how: &str, builtin: &str, ecx_bit: u32) {
    program.top("static int cpu_has_feature(void) {");
    if how == "builtin" {
        program.top("    __builtin_cpu_init();");
        program.top(format!("    return __builtin_cpu_supports(\"{builtin}\") != 0;"));
    } else {
        program.include("cpuid.h");
        program.top("    unsigned int eax = 0, ebx = 0, ecx = 0, edx = 0;");
        program.top("    if (!__get_cpuid(1, &eax, &ebx, &ecx, &edx)) {");
        program.top("        return 0;");
        program.top("    }");
        program.top(format!("    return (ecx & (1u << {ecx_bit})) != 0;"));
    }
    program.top("}");
}

/// The CRC-32C of some bytes, without the inversions at either end, one bit at a time.
///
/// This is the definition, and it is what the SSE4.2 instructions compute: each one takes the
/// running value and more bytes, least significant first, and hands back the running value.
fn crc32c_raw(mut crc: u32, bytes: &[u8]) -> u32 {
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 { (crc >> 1) ^ 0x82f6_3b78 } else { crc >> 1 };
        }
    }
    crc
}

/// The CRC-32C Postgres stores, with the inversions it applies before and after.
fn crc32c(bytes: &[u8]) -> u32 {
    crc32c_raw(0xffff_ffff, bytes) ^ 0xffff_ffff
}

/// An instruction set extension a function can be built for, and what it computes with it.
///
/// Every feature has the same shape, so the programs differ only in the thing under test. The
/// kernel reads four `unsigned int` at `d` and sets `r`, once with the extension's intrinsics
/// and once in portable C, and [`Feature::model`] is what both have to come to.
struct Feature {
    /// The name on the axis, which cannot have a dot in it because the case id is dotted.
    name: &'static str,
    /// What goes inside `target("...")`.
    target: &'static str,
    /// What `__builtin_cpu_supports` calls it, or nothing when Postgres only asks `cpuid`.
    builtin: Option<&'static str>,
    /// The bit in ECX of `cpuid` leaf one that says the program may use it.
    ecx_bit: u32,
    /// The header the intrinsics come from, if any.
    header: Option<&'static str>,
    /// The kernel with the extension.
    fast: &'static [&'static str],
    /// The same kernel in portable C.
    slow: &'static [&'static str],
    /// What the kernel sets `r` to for one chunk of four values.
    model: fn(&[u32]) -> u64,
}

/// A byte swap and an absolute value, as `_mm_shuffle_epi8` and `_mm_abs_epi32` do them.
fn ssse3_model(d: &[u32]) -> u64 {
    let out: Vec<u64> = (0..4)
        .map(|k| {
            let x = d[3 - k].swap_bytes();
            u64::from(if x & 0x8000_0000 != 0 { x.wrapping_neg() } else { x })
        })
        .collect();
    weighted(&out)
}

/// A multiply and a signed minimum, as `_mm_mullo_epi32` and `_mm_min_epi32` do them.
fn sse41_model(d: &[u32]) -> u64 {
    let out: Vec<u64> = d
        .iter()
        .map(|&a| {
            let p = a.wrapping_mul(0x9e37_79b1);
            u64::from(if (p as i32) < (a as i32) { p } else { a })
        })
        .collect();
    weighted(&out)
}

/// A signed 64 bit comparison of the two halves and a CRC-32C step, the SSE4.2 pair.
fn sse42_model(d: &[u32]) -> u64 {
    let lo = u64::from(d[0]) | (u64::from(d[1]) << 32);
    let hi = u64::from(d[2]) | (u64::from(d[3]) << 32);
    let mut mask = 0u64;
    if (lo as i64) > (hi as i64) {
        mask |= 0xff;
    }
    if (hi as i64) > (lo as i64) {
        mask |= 0xff00;
    }
    (mask << 32) | u64::from(crc32c_raw(0xffff_ffff, &(d[0] ^ d[3]).to_le_bytes()))
}

fn popcnt_model(d: &[u32]) -> u64 {
    let wide = u64::from(d[0]) | (u64::from(d[1]) << 32);
    u64::from(wide.count_ones()) * 100 + u64::from((d[2] ^ d[3]).count_ones())
}

fn crc32_model(d: &[u32]) -> u64 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&d[0].to_le_bytes());
    bytes.extend_from_slice(&d[1].to_le_bytes());
    bytes.extend_from_slice(&d[2].to_le_bytes());
    bytes.push(d[3] as u8);
    bytes.extend_from_slice(&((d[3] >> 16) as u16).to_le_bytes());
    u64::from(crc32c(&bytes))
}

/// The four lanes of a result folded into one number, each with its own weight.
fn weighted(out: &[u64]) -> u64 {
    out[0] + 3 * out[1] + 5 * out[2] + 7 * out[3]
}

/// The instruction set extensions Postgres builds single functions for, or could.
///
/// The ones from SSE3 to SSE4.2, `popcnt`, `crc32` and `xsave`, which are the ones every x86-64
/// that Postgres cares about has or does not have one at a time. AVX-512 is left out on
/// purpose: a case has to be right on any x86-64 that runs it, and the machines that check the
/// corpus mostly do not have it, so its fast path would never be checked.
const FEATURES: &[Feature] = &[
    Feature {
        name: "sse3",
        target: "sse3",
        builtin: Some("sse3"),
        ecx_bit: 0,
        header: Some("pmmintrin.h"),
        fast: &[
            "__m128i v = _mm_lddqu_si128((const __m128i *)d);",
            "v = _mm_add_epi32(v, _mm_srli_si128(v, 8));",
            "v = _mm_add_epi32(v, _mm_srli_si128(v, 4));",
            "r = (unsigned int)_mm_cvtsi128_si32(v);",
        ],
        slow: &["r = (unsigned int)(d[0] + d[1] + d[2] + d[3]);"],
        model: |d| u64::from(d.iter().fold(0u32, |sum, &x| sum.wrapping_add(x))),
    },
    Feature {
        name: "ssse3",
        target: "ssse3",
        builtin: Some("ssse3"),
        ecx_bit: 9,
        header: Some("tmmintrin.h"),
        fast: &[
            "__m128i v = _mm_loadu_si128((const __m128i *)d);",
            "__m128i order = _mm_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);",
            "__m128i a = _mm_abs_epi32(_mm_shuffle_epi8(v, order));",
            "unsigned int out[4];",
            "_mm_storeu_si128((__m128i *)out, a);",
            "r = out[0] + 3ull * out[1] + 5ull * out[2] + 7ull * out[3];",
        ],
        slow: &[
            "unsigned int out[4];",
            "for (int k = 0; k < 4; k++) {",
            "    unsigned int x = d[3 - k];",
            "    x = (x >> 24) | ((x >> 8) & 0xff00u) | ((x << 8) & 0xff0000u) | (x << 24);",
            "    out[k] = (x & 0x80000000u) != 0 ? 0u - x : x;",
            "}",
            "r = out[0] + 3ull * out[1] + 5ull * out[2] + 7ull * out[3];",
        ],
        model: ssse3_model,
    },
    Feature {
        name: "sse4-1",
        target: "sse4.1",
        builtin: Some("sse4.1"),
        ecx_bit: 19,
        header: Some("smmintrin.h"),
        fast: &[
            "__m128i a = _mm_loadu_si128((const __m128i *)d);",
            "__m128i p = _mm_mullo_epi32(a, _mm_set1_epi32(-1640531535));",
            "__m128i m = _mm_min_epi32(p, a);",
            "r = (unsigned int)_mm_extract_epi32(m, 0) + 3ull * (unsigned int)_mm_extract_epi32(m, 1)",
            "    + 5ull * (unsigned int)_mm_extract_epi32(m, 2) + 7ull * (unsigned int)_mm_extract_epi32(m, 3);",
        ],
        slow: &[
            "unsigned int m[4];",
            "for (int k = 0; k < 4; k++) {",
            "    unsigned int p = d[k] * 0x9e3779b1u;",
            "    m[k] = (p ^ 0x80000000u) < (d[k] ^ 0x80000000u) ? p : d[k];",
            "}",
            "r = m[0] + 3ull * m[1] + 5ull * m[2] + 7ull * m[3];",
        ],
        model: sse41_model,
    },
    Feature {
        name: "sse4-2",
        target: "sse4.2",
        builtin: Some("sse4.2"),
        ecx_bit: 20,
        header: Some("nmmintrin.h"),
        fast: &[
            "__m128i a = _mm_loadu_si128((const __m128i *)d);",
            "__m128i gt = _mm_cmpgt_epi64(a, _mm_shuffle_epi32(a, 0x4e));",
            "unsigned int mask = (unsigned int)_mm_movemask_epi8(gt);",
            "r = ((unsigned long long)mask << 32) | _mm_crc32_u32(0xffffffffu, d[0] ^ d[3]);",
        ],
        slow: &[
            "unsigned long long lo = d[0] | ((unsigned long long)d[1] << 32);",
            "unsigned long long hi = d[2] | ((unsigned long long)d[3] << 32);",
            "unsigned long long flip = 0x8000000000000000ull;",
            "unsigned int mask = 0;",
            "if ((lo ^ flip) > (hi ^ flip)) {",
            "    mask = mask | 0xffu;",
            "}",
            "if ((hi ^ flip) > (lo ^ flip)) {",
            "    mask = mask | 0xff00u;",
            "}",
            "unsigned int c = 0xffffffffu ^ d[0] ^ d[3];",
            "for (int k = 0; k < 32; k++) {",
            "    c = (c & 1u) != 0 ? (c >> 1) ^ 0x82f63b78u : c >> 1;",
            "}",
            "r = ((unsigned long long)mask << 32) | c;",
        ],
        model: sse42_model,
    },
    Feature {
        name: "popcnt",
        target: "popcnt",
        builtin: Some("popcnt"),
        ecx_bit: 23,
        header: None,
        fast: &[
            "r = (unsigned long long)__builtin_popcountll(d[0] | ((unsigned long long)d[1] << 32)) * 100",
            "    + (unsigned long long)__builtin_popcount(d[2] ^ d[3]);",
        ],
        slow: &[
            "unsigned long long w = d[0] | ((unsigned long long)d[1] << 32);",
            "unsigned int v = d[2] ^ d[3];",
            "unsigned long long bits = 0;",
            "unsigned long long more = 0;",
            "while (w != 0) {",
            "    w = w & (w - 1);",
            "    bits = bits + 1;",
            "}",
            "while (v != 0) {",
            "    v = v & (v - 1);",
            "    more = more + 1;",
            "}",
            "r = bits * 100 + more;",
        ],
        model: popcnt_model,
    },
    Feature {
        name: "crc32",
        target: "crc32",
        // The instruction is SSE4.2 as far as cpuid is concerned.
        builtin: Some("sse4.2"),
        ecx_bit: 20,
        header: Some("nmmintrin.h"),
        fast: &[
            "unsigned int c = 0xffffffffu;",
            "c = _mm_crc32_u32(c, d[0]);",
            "c = (unsigned int)_mm_crc32_u64(c, d[1] | ((unsigned long long)d[2] << 32));",
            "c = _mm_crc32_u8(c, (unsigned char)d[3]);",
            "c = _mm_crc32_u16(c, (unsigned short)(d[3] >> 16));",
            "r = c ^ 0xffffffffu;",
        ],
        slow: &[
            "unsigned char bytes[15];",
            "for (int k = 0; k < 12; k++) {",
            "    bytes[k] = (unsigned char)(d[k / 4] >> (8 * (k % 4)));",
            "}",
            "bytes[12] = (unsigned char)d[3];",
            "bytes[13] = (unsigned char)(d[3] >> 16);",
            "bytes[14] = (unsigned char)(d[3] >> 24);",
            "unsigned int c = 0xffffffffu;",
            "for (int k = 0; k < 15; k++) {",
            "    c = c ^ bytes[k];",
            "    for (int b = 0; b < 8; b++) {",
            "        c = (c & 1u) != 0 ? (c >> 1) ^ 0x82f63b78u : c >> 1;",
            "    }",
            "}",
            "r = c ^ 0xffffffffu;",
        ],
        model: crc32_model,
    },
    Feature {
        name: "xsave",
        target: "xsave",
        // Postgres asks cpuid for OSXSAVE, which says the operating system turned XGETBV on,
        // rather than for XSAVE, which only says the processor has it.
        builtin: None,
        ecx_bit: 27,
        header: Some("immintrin.h"),
        fast: &["r = (_xgetbv(0) & 3u) * 1000 + (d[0] & 0xffu);"],
        // The x87 and SSE state are on in XCR0 on every x86-64 system that has it at all.
        slow: &["r = 3 * 1000 + (d[0] & 0xffu);"],
        model: |d| 3000 + u64::from(d[0] & 0xff),
    },
];

/// How many chunks of four values a `target-attribute` case works through.
const CHUNKS: usize = 64;

/// The prefixes of the data a `target-attribute` case runs the chosen version over.
const PREFIXES: &[usize] = &[1, 5, 16, 64];

/// The values a `target-attribute` or `crc32c` case fills its buffer with from `seed`.
fn words(seed: u32, count: usize) -> Vec<u32> {
    let mut x = seed;
    let mut out: Vec<u32> = (0..count)
        .map(|_| {
            x = lcg(x);
            x ^ (x >> 11)
        })
        .collect();
    // The values that sit on an edge, where a signed and an unsigned reading disagree.
    for (at, value) in [(5, 0x8000_0000), (10, 0), (15, 0xffff_ffff), (22, 0x7fff_ffff)] {
        if at < out.len() {
            out[at] = value;
        }
    }
    out
}

/// The same fill, in C, into `data`.
fn fill_words(program: &mut Program, count: usize) {
    program.line("unsigned int x = seed;");
    program.line(format!("for (int i = 0; i < {count}; i++) {{"));
    program.line_at(1, LCG_C);
    program.line_at(1, "data[i] = x ^ (x >> 11);");
    program.line("}");
    program.line("data[5] = 0x80000000u;");
    program.line("data[10] = 0;");
    program.line("data[15] = 0xffffffffu;");
    program.line("data[22] = 0x7fffffffu;");
}

/// The seed every case in the second half of this module starts from.
const PG_SEED: u32 = 20_250_928;

/// Functions built for one extension and called only after the processor says it has it.
///
/// The axes are the ways Postgres does it. `feature` is the extension. `check` is whether it
/// asks with `__builtin_cpu_supports` or with `cpuid`. `dispatch` is an `if` at every call, or
/// a function pointer that starts at a chooser and is overwritten by the first call, which is
/// how `pg_comp_crc32c` works. `loop` is where the boundary between the two instruction sets
/// is: `inside` has the whole loop in the function with the attribute, and `outside` calls a
/// function with the attribute once per chunk from one without it, so every call crosses it.
///
/// A case prints the checksum of the version it chose over several prefixes of the data and
/// how many times that disagreed with the portable version. Neither depends on whether the
/// processor had the extension, which is what makes the output the same on every x86-64.
fn target_attribute(sink: &mut Sink<'_>) {
    const CHECKS: &[&str] = &["builtin", "cpuid"];
    const DISPATCHES: &[&str] = &["direct", "pointer"];
    const LOOPS: &[&str] = &["inside", "outside"];
    for feature in FEATURES {
        for &check in CHECKS {
            if check == "builtin" && feature.builtin.is_none() {
                continue;
            }
            for &dispatch in DISPATCHES {
                for &place in LOOPS {
                    if !sink.wants(Facet::TargetAttribute) {
                        return;
                    }
                    let program = target_program(feature, check, dispatch, place);
                    let axes = Axes::of([
                        ("feature", feature.name),
                        ("check", check),
                        ("dispatch", dispatch),
                        ("loop", place),
                    ]);
                    sink.push_tagged(
                        Facet::TargetAttribute,
                        axes,
                        Dialect::C17,
                        program,
                        &["gnu", "headers", X86_64, PROVENANCE],
                    );
                }
            }
        }
    }
}

/// One `target-attribute` case.
fn target_program(feature: &Feature, check: &str, dispatch: &str, place: &str) -> Program {
    let mut program = Program::new(format!(
        "a function built with target(\"{}\"), called after a {check} check through {} {}",
        feature.target,
        if dispatch == "direct" { "an if" } else { "a chosen pointer" },
        if place == "inside" { "with the loop inside it" } else { "once per chunk" },
    ));
    if let Some(header) = feature.header {
        program.include(header);
    }
    program.top(format!("static unsigned int data[{}];", 4 * CHUNKS));
    cpu_check(&mut program, check, feature.builtin.unwrap_or_default(), feature.ecx_bit);
    let attribute = format!("__attribute__((target(\"{}\")))", feature.target);
    let kernel = |program: &mut Program, lines: &[&str], indent: &str| {
        program.top(format!("{indent}unsigned long long r;"));
        for line in lines {
            program.top(format!("{indent}{line}"));
        }
    };
    for (suffix, lines) in [("fast", feature.fast), ("slow", feature.slow)] {
        let marked = if suffix == "fast" { format!("{attribute} ") } else { String::new() };
        if place == "inside" || suffix == "slow" {
            program.top(format!(
                "{marked}static unsigned long long run_{suffix}(const unsigned int *base, int chunks) {{"
            ));
            program.top("    unsigned long long acc = 0;");
            program.top("    for (int i = 0; i < chunks; i++) {");
            program.top("        const unsigned int *d = base + 4 * i;");
            kernel(&mut program, lines, "        ");
            program.top("        acc = acc * 31 + r;");
            program.top("    }");
            program.top("    return acc;");
            program.top("}");
        } else {
            program.top(format!(
                "{marked}static unsigned long long one_{suffix}(const unsigned int *d) {{"
            ));
            kernel(&mut program, lines, "    ");
            program.top("    return r;");
            program.top("}");
            program.top(format!(
                "static unsigned long long run_{suffix}(const unsigned int *base, int chunks) {{"
            ));
            program.top("    unsigned long long acc = 0;");
            program.top("    for (int i = 0; i < chunks; i++) {");
            program.top(format!("        acc = acc * 31 + one_{suffix}(base + 4 * i);"));
            program.top("    }");
            program.top("    return acc;");
            program.top("}");
        }
    }
    if dispatch == "direct" {
        program.top("static unsigned long long run(const unsigned int *base, int chunks) {");
        program.top("    if (cpu_has_feature()) {");
        program.top("        return run_fast(base, chunks);");
        program.top("    }");
        program.top("    return run_slow(base, chunks);");
        program.top("}");
    } else {
        // The shape of pg_comp_crc32c: the pointer starts at a chooser, and the first call
        // asks the processor once and points it at the version to use from then on.
        program.top("static unsigned long long run_choose(const unsigned int *base, int chunks);");
        program.top("static unsigned long long (*run)(const unsigned int *, int) = run_choose;");
        program.top("static unsigned long long run_choose(const unsigned int *base, int chunks) {");
        program.top("    run = cpu_has_feature() ? run_fast : run_slow;");
        program.top("    return run(base, chunks);");
        program.top("}");
    }

    program.input(Ty::U32, "seed", i128::from(PG_SEED));
    fill_words(&mut program, 4 * CHUNKS);
    program.line("int mismatches = 0;");
    program.line("unsigned long long got = 0;");
    let data = words(PG_SEED, 4 * CHUNKS);
    for &prefix in PREFIXES {
        let want = data[..4 * prefix]
            .chunks(4)
            .fold(0u64, |acc, chunk| acc.wrapping_mul(31).wrapping_add((feature.model)(chunk)));
        program.blank();
        program.line(format!("got = run(data, {prefix});"));
        program.check(Ty::U64, "got", i128::from(want));
        program.line(format!("if (got != run_slow(data, {prefix})) {{"));
        program.line_at(1, "mismatches = mismatches + 1;");
        program.line("}");
    }
    program.blank();
    program.check(Ty::I32, "mismatches", 0);
    program
}

/// The buffer lengths a `crc32c` case checksums.
///
/// Every length up to one past two words, so each of the tails the instruction loops leave
/// is there, then the lengths around the larger powers of two and one the size of a page less
/// a little.
const CRC_LENGTHS: &[usize] =
    &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 15, 16, 17, 31, 32, 33, 63, 64, 65, 100, 255, 1000, 4000];

/// How many bytes a `crc32c` case fills, which covers the longest length at the largest offset.
const CRC_BYTES: usize = 4112;

/// The bytes a `crc32c` case fills its buffer with.
fn crc_bytes(seed: u32) -> Vec<u8> {
    let mut x = seed;
    (0..CRC_BYTES)
        .map(|_| {
            x = lcg(x);
            (x >> 24) as u8
        })
        .collect()
}

/// CRC-32C with the SSE4.2 instructions, checked against Postgres's slicing by eight.
///
/// `step` is the widest instruction the loop uses: `u8` only, `u32` then bytes, `u64` then one
/// `u32` then bytes, which is `pg_comp_crc32c_sse42`, and `aligned`, which takes bytes until
/// the pointer is on an eight byte boundary before it takes words. `offset` is how far the
/// buffer starts from an eight byte boundary, and `calls` is whether each checksum is one call
/// or three, chained the way `COMP_CRC32C` is when a record is checksummed in pieces.
fn crc32c_facet(sink: &mut Sink<'_>) {
    const STEPS: &[&str] = &["u8", "u32", "u64", "aligned"];
    const OFFSETS: &[usize] = &[0, 1, 3, 7];
    const CALLS: &[&str] = &["whole", "pieces"];
    const CHECKS: &[&str] = &["builtin", "cpuid"];
    for &step in STEPS {
        for &offset in OFFSETS {
            for &calls in CALLS {
                for &check in CHECKS {
                    if !sink.wants(Facet::Crc32c) {
                        return;
                    }
                    let program = crc32c_program(step, offset, calls, check);
                    let axes = Axes::of([
                        ("step", step),
                        ("offset", &offset.to_string()),
                        ("calls", calls),
                        ("check", check),
                    ]);
                    sink.push_tagged(
                        Facet::Crc32c,
                        axes,
                        Dialect::C17,
                        program,
                        &["gnu", "headers", X86_64, PROVENANCE],
                    );
                }
            }
        }
    }
}

/// Writes `crc_init` and `crc_sb8`, which are Postgres's slicing by eight, `pg_comp_crc32c_sb8`.
fn slicing_by_eight(program: &mut Program) {
    program.top("static unsigned int crc_table[8][256];");
    program.top("static void crc_init(void) {");
    program.top("    for (unsigned int i = 0; i < 256; i++) {");
    program.top("        unsigned int c = i;");
    program.top("        for (int k = 0; k < 8; k++) {");
    program.top("            c = (c & 1u) != 0 ? (c >> 1) ^ 0x82f63b78u : c >> 1;");
    program.top("        }");
    program.top("        crc_table[0][i] = c;");
    program.top("    }");
    program.top("    for (unsigned int i = 0; i < 256; i++) {");
    program.top("        for (int k = 1; k < 8; k++) {");
    program.top("            unsigned int prev = crc_table[k - 1][i];");
    program.top("            crc_table[k][i] = (prev >> 8) ^ crc_table[0][prev & 0xff];");
    program.top("        }");
    program.top("    }");
    program.top("}");
    // pg_comp_crc32c_sb8, with the words put together from bytes rather than loaded, which is
    // the same thing on a little endian machine without reading a char buffer as an int.
    program.top("static unsigned int load32(const unsigned char *p) {");
    program.top(
        "    return p[0] | ((unsigned int)p[1] << 8) | ((unsigned int)p[2] << 16) | ((unsigned int)p[3] << 24);",
    );
    program.top("}");
    program.top("static unsigned int crc_sb8(unsigned int crc, const unsigned char *p, int len) {");
    program.top("    while (len > 0 && ((unsigned long)p & 3) != 0) {");
    program.top("        crc = crc_table[0][(crc ^ *p++) & 0xff] ^ (crc >> 8);");
    program.top("        len--;");
    program.top("    }");
    program.top("    while (len >= 8) {");
    program.top("        unsigned int a = load32(p) ^ crc;");
    program.top("        unsigned int b = load32(p + 4);");
    program.top("        crc = crc_table[7][a & 0xff] ^ crc_table[6][(a >> 8) & 0xff] ^");
    program.top("              crc_table[5][(a >> 16) & 0xff] ^ crc_table[4][a >> 24] ^");
    program.top("              crc_table[3][b & 0xff] ^ crc_table[2][(b >> 8) & 0xff] ^");
    program.top("              crc_table[1][(b >> 16) & 0xff] ^ crc_table[0][b >> 24];");
    program.top("        p += 8;");
    program.top("        len -= 8;");
    program.top("    }");
    program.top("    while (len > 0) {");
    program.top("        crc = crc_table[0][(crc ^ *p++) & 0xff] ^ (crc >> 8);");
    program.top("        len--;");
    program.top("    }");
    program.top("    return crc;");
    program.top("}");
}

/// One `crc32c` case.
fn crc32c_program(step: &str, offset: usize, calls: &str, check: &str) -> Program {
    let mut program = Program::new(format!(
        "CRC-32C with the {step} SSE4.2 loop at offset {offset}, in {calls} calls, after a {check} check"
    ));
    program.include("nmmintrin.h");
    program.top(format!("static unsigned long long storage[{}];", CRC_BYTES / 8));
    slicing_by_eight(&mut program);

    let bytes = |program: &mut Program| {
        program.top("    while (len > 0) {");
        program.top("        crc = _mm_crc32_u8(crc, *p);");
        program.top("        p++;");
        program.top("        len--;");
        program.top("    }");
    };
    let word = |program: &mut Program, keyword: &str| {
        program.top(format!("    {keyword} (len >= 4) {{"));
        program.top("        unsigned int w;");
        program.top("        __builtin_memcpy(&w, p, 4);");
        program.top("        crc = _mm_crc32_u32(crc, w);");
        program.top("        p += 4;");
        program.top("        len -= 4;");
        program.top("    }");
    };
    let quad = |program: &mut Program| {
        program.top("    while (len >= 8) {");
        program.top("        unsigned long long w;");
        program.top("        __builtin_memcpy(&w, p, 8);");
        program.top("        crc = (unsigned int)_mm_crc32_u64(crc, w);");
        program.top("        p += 8;");
        program.top("        len -= 8;");
        program.top("    }");
    };
    program.top("__attribute__((target(\"sse4.2\")))");
    program
        .top("static unsigned int crc_sse42(unsigned int crc, const unsigned char *p, int len) {");
    match step {
        "u8" => bytes(&mut program),
        "u32" => {
            word(&mut program, "while");
            bytes(&mut program);
        }
        "u64" => {
            quad(&mut program);
            word(&mut program, "if");
            bytes(&mut program);
        }
        _ => {
            program.top("    while (len > 0 && ((unsigned long)p & 7) != 0) {");
            program.top("        crc = _mm_crc32_u8(crc, *p);");
            program.top("        p++;");
            program.top("        len--;");
            program.top("    }");
            quad(&mut program);
            bytes(&mut program);
        }
    }
    program.top("    return crc;");
    program.top("}");
    cpu_check(&mut program, check, "sse4.2", 20);
    program.top("static unsigned int (*comp_crc)(unsigned int, const unsigned char *, int);");
    program.top("static unsigned int crc_of(const unsigned char *p, int len) {");
    program.top("    unsigned int crc = 0xffffffffu;");
    if calls == "whole" {
        program.top("    crc = comp_crc(crc, p, len);");
    } else {
        program.top("    int first = len / 3;");
        program.top("    int second = (len - first) / 2;");
        program.top("    crc = comp_crc(crc, p, first);");
        program.top("    crc = comp_crc(crc, p + first, second);");
        program.top("    crc = comp_crc(crc, p + first + second, len - first - second);");
    }
    program.top("    return crc ^ 0xffffffffu;");
    program.top("}");

    program.input(Ty::U32, "seed", i128::from(PG_SEED));
    program.line("unsigned char *bytes = (unsigned char *)storage;");
    program.line("unsigned int x = seed;");
    program.line(format!("for (int i = 0; i < {CRC_BYTES}; i++) {{"));
    program.line_at(1, LCG_C);
    program.line_at(1, "bytes[i] = (unsigned char)(x >> 24);");
    program.line("}");
    program.line("crc_init();");
    program.line("comp_crc = cpu_has_feature() ? crc_sse42 : crc_sb8;");
    program.line(format!("const unsigned char *base = bytes + {offset};"));
    program.line("int mismatches = 0;");
    program.line("unsigned int got = 0;");
    // The check value every CRC-32C implementation publishes.
    program.line(
        "static const unsigned char digits[9] = { '1', '2', '3', '4', '5', '6', '7', '8', '9' };",
    );
    program.check(Ty::U32, "crc_of(digits, 9)", i128::from(crc32c(b"123456789")));
    let data = crc_bytes(PG_SEED);
    for &len in CRC_LENGTHS {
        program.blank();
        program.line(format!("got = crc_of(base, {len});"));
        program.check(Ty::U32, "got", i128::from(crc32c(&data[offset..offset + len])));
        program.line(format!("if (got != (crc_sb8(0xffffffffu, base, {len}) ^ 0xffffffffu)) {{"));
        program.line_at(1, "mismatches = mismatches + 1;");
        program.line("}");
    }
    program.blank();
    program.check(Ty::I32, "mismatches", 0);
    program
}

/// One of the search loops from `pg_lfind.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Search {
    /// `pg_lfind32` with one vector per iteration.
    Find32One,
    /// `pg_lfind32` as Postgres writes it, four vectors per iteration folded with `or`.
    Find32Four,
    /// `pg_lfind8`.
    Find8,
    /// `pg_lfind8_le` with the saturating subtract `vector8_has_le` uses.
    Find8LeSubs,
    /// `pg_lfind8_le` with an unsigned maximum and an equality, the other way to do it.
    Find8LeMax,
}

impl Search {
    const ALL: [Self; 5] =
        [Self::Find32One, Self::Find32Four, Self::Find8, Self::Find8LeSubs, Self::Find8LeMax];

    const fn name(self) -> &'static str {
        match self {
            Self::Find32One => "lfind32-x1",
            Self::Find32Four => "lfind32-x4",
            Self::Find8 => "lfind8",
            Self::Find8LeSubs => "lfind8-le-subs",
            Self::Find8LeMax => "lfind8-le-max",
        }
    }

    const fn wide(self) -> bool {
        matches!(self, Self::Find32One | Self::Find32Four)
    }

    const fn le(self) -> bool {
        matches!(self, Self::Find8LeSubs | Self::Find8LeMax)
    }

    /// The element type in C.
    const fn elem(self) -> &'static str {
        if self.wide() { "unsigned int" } else { "unsigned char" }
    }

    /// A key that no element matches, for data in the low range or the high one.
    ///
    /// Each is on the far side of the sign bit from the data, so a loop that compared signed
    /// where it had to compare unsigned finds it.
    const fn absent(self, high: bool) -> u32 {
        match (self.wide(), self.le(), high) {
            (true, _, false) => 0x8000_0001,
            (true, _, true) => 1,
            (false, false, false) => 0xc8,
            (false, false, true) => 0xff,
            (false, true, false) => 0,
            (false, true, true) => 0x7f,
        }
    }

    /// Whether the scalar version finds the key in these elements.
    fn scalar(self, key: u32, elems: &[u32]) -> bool {
        if self.le() { elems.iter().any(|&e| e <= key) } else { elems.contains(&key) }
    }
}

/// The lengths a `simd-lfind` case searches.
///
/// Every length up to two and a half of the widest step, so each tail length is there for
/// each loop, then the lengths either side of the larger multiples and one long run.
fn lfind_lengths() -> Vec<usize> {
    let mut lengths: Vec<usize> = (0..=40).collect();
    lengths.extend([47, 48, 49, 63, 64, 65, 100, 127, 128, 129, 255, 256, 1000]);
    lengths
}

/// How many elements a `simd-lfind` case fills.
const LFIND_ELEMS: usize = 1024;

/// The elements a `simd-lfind` case fills its array with.
fn lfind_elems(search: Search, high: bool) -> Vec<u32> {
    let mut x = PG_SEED;
    (0..LFIND_ELEMS)
        .map(|_| {
            x = lcg(x);
            let t = x >> 16;
            match (search.wide(), high) {
                (false, false) => 1 + t % 100,
                (false, true) => 0x80 + t % 127,
                (true, false) => 1 + (x >> 8) % 1_000_000,
                (true, true) => 0x8000_0000 + (x >> 8) % 1_000_000,
            }
        })
        .collect()
}

/// The C that fills the array, the same as [`lfind_elems`].
fn lfind_fill(search: Search, high: bool) -> &'static str {
    match (search.wide(), high) {
        (false, false) => "elems[i] = (unsigned char)(1 + (x >> 16) % 100);",
        (false, true) => "elems[i] = (unsigned char)(0x80 + (x >> 16) % 127);",
        (true, false) => "elems[i] = 1 + (x >> 8) % 1000000;",
        (true, true) => "elems[i] = 0x80000000u + (x >> 8) % 1000000;",
    }
}

/// The SSE2 search loops from `port/simd.h` and `port/pg_lfind.h`, against scalar loops.
///
/// `search` is the loop. `values` is whether the elements are below the sign bit of their
/// type or above it, and `offset` is how many elements into an aligned array the search
/// starts, which makes every load an unaligned one the way a pointer into the middle of a
/// Postgres array is. The loops need nothing past SSE2, which every x86-64 has, so there is
/// no processor check, the same as in Postgres.
fn simd_lfind(sink: &mut Sink<'_>) {
    const VALUES: &[&str] = &["low", "high"];
    const OFFSETS: &[usize] = &[0, 1, 3];
    for search in Search::ALL {
        for &values in VALUES {
            for &offset in OFFSETS {
                if !sink.wants(Facet::SimdLfind) {
                    return;
                }
                let program = lfind_program(search, values == "high", offset);
                let axes = Axes::of([
                    ("search", search.name()),
                    ("values", values),
                    ("offset", &offset.to_string()),
                ]);
                sink.push_tagged(
                    Facet::SimdLfind,
                    axes,
                    Dialect::C17,
                    program,
                    &["headers", X86_64, PROVENANCE],
                );
            }
        }
    }
}

/// One `simd-lfind` case.
fn lfind_program(search: Search, high: bool, offset: usize) -> Program {
    let elem = search.elem();
    let mut program = Program::new(format!(
        "{} over {} values starting {offset} elements in, against a scalar loop",
        search.name(),
        if high { "high" } else { "low" }
    ));
    program.include("emmintrin.h");
    program.top(format!("static {elem} elems[{}];", LFIND_ELEMS + 16));
    program.top(format!(
        "static int search_simd({elem} key, const {elem} *base, unsigned int nelem) {{"
    ));
    program.top("    unsigned int i = 0;");
    let found = "            return 1;";
    match search {
        Search::Find32One => {
            program.top("    const __m128i keys = _mm_set1_epi32((int)key);");
            program.top("    const unsigned int tail_idx = nelem & ~(4u - 1);");
            program.top("    for (i = 0; i < tail_idx; i += 4) {");
            program.top("        const __m128i vals = _mm_loadu_si128((const __m128i *)&base[i]);");
            program.top("        if (_mm_movemask_epi8(_mm_cmpeq_epi32(keys, vals)) != 0) {");
        }
        Search::Find32Four => {
            program.top("    const unsigned int nelem_per_vector = 4;");
            program.top("    const unsigned int nelem_per_iteration = 4 * nelem_per_vector;");
            program.top("    const unsigned int tail_idx = nelem & ~(nelem_per_iteration - 1);");
            program.top("    const __m128i keys = _mm_set1_epi32((int)key);");
            program.top("    for (i = 0; i < tail_idx; i += nelem_per_iteration) {");
            for k in 1..=4 {
                program.top(format!(
                    "        const __m128i vals{k} = _mm_loadu_si128((const __m128i *)&base[i + {}]);",
                    4 * (k - 1)
                ));
            }
            for k in 1..=4 {
                program.top(format!(
                    "        const __m128i result{k} = _mm_cmpeq_epi32(keys, vals{k});"
                ));
            }
            program.top("        const __m128i tmp1 = _mm_or_si128(result1, result2);");
            program.top("        const __m128i tmp2 = _mm_or_si128(result3, result4);");
            program.top("        const __m128i result = _mm_or_si128(tmp1, tmp2);");
            program.top("        if (_mm_movemask_epi8(result) != 0) {");
        }
        _ => {
            program.top("    const __m128i keys = _mm_set1_epi8((char)key);");
            program.top("    const unsigned int tail_idx = nelem & ~(16u - 1);");
            program.top("    for (i = 0; i < tail_idx; i += 16) {");
            program
                .top("        const __m128i chunk = _mm_loadu_si128((const __m128i *)&base[i]);");
            let test = match search {
                Search::Find8 => "_mm_cmpeq_epi8(chunk, keys)",
                // vector8_has_le: a byte at or below the key saturates to zero.
                Search::Find8LeSubs => {
                    "_mm_cmpeq_epi8(_mm_subs_epu8(chunk, keys), _mm_setzero_si128())"
                }
                _ => "_mm_cmpeq_epi8(_mm_max_epu8(chunk, keys), keys)",
            };
            program.top(format!("        if (_mm_movemask_epi8({test}) != 0) {{"));
        }
    }
    program.top(found);
    program.top("        }");
    program.top("    }");
    let compare = if search.le() { "base[i] <= key" } else { "base[i] == key" };
    program.top("    for (; i < nelem; i++) {");
    program.top(format!("        if ({compare}) {{"));
    program.top(found);
    program.top("        }");
    program.top("    }");
    program.top("    return 0;");
    program.top("}");
    program.top(format!(
        "static int search_scalar({elem} key, const {elem} *base, unsigned int nelem) {{"
    ));
    program.top("    for (unsigned int i = 0; i < nelem; i++) {");
    program.top(format!("        if ({compare}) {{"));
    program.top("            return 1;");
    program.top("        }");
    program.top("    }");
    program.top("    return 0;");
    program.top("}");

    let lengths = lfind_lengths();
    let list = lengths.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
    program.top(format!("static const unsigned int lengths[{}] = {{ {list} }};", lengths.len()));

    program.input(Ty::U32, "seed", i128::from(PG_SEED));
    program.line("unsigned int x = seed;");
    program.line(format!("for (int i = 0; i < {LFIND_ELEMS}; i++) {{"));
    program.line_at(1, LCG_C);
    program.line_at(1, lfind_fill(search, high));
    program.line("}");
    program.line(format!("const {elem} *base = elems + {offset};"));
    program.line("int found[4] = { 0, 0, 0, 0 };");
    program.line("unsigned long long pattern = 0;");
    program.line("int mismatches = 0;");
    program.line(format!("for (int n = 0; n < {}; n++) {{", lengths.len()));
    program.line_at(1, "unsigned int len = lengths[n];");
    program.line_at(1, format!("{elem} keys[4];"));
    // For the equality searches the keys are elements, at the start, the middle and the end.
    // For the at-or-below ones they are one less than those, which some other element may or
    // may not be at or below.
    let less = if search.le() { " - 1" } else { "" };
    program.line_at(1, format!("keys[0] = ({elem})(base[0]{less});"));
    program.line_at(1, format!("keys[1] = ({elem})(base[len / 2]{less});"));
    program.line_at(1, format!("keys[2] = ({elem})(base[len > 0 ? len - 1 : 0]{less});"));
    program.line_at(1, format!("keys[3] = {:#x};", search.absent(high)));
    program.line_at(1, "for (int k = 0; k < 4; k++) {");
    program.line_at(2, "int got = search_simd(keys[k], base, len);");
    program.line_at(2, "found[k] = found[k] + got;");
    program.line_at(2, "pattern = pattern * 3 + (unsigned long long)got;");
    program.line_at(2, "if (got != search_scalar(keys[k], base, len)) {");
    program.line_at(3, "mismatches = mismatches + 1;");
    program.line_at(2, "}");
    program.line_at(1, "}");
    program.line("}");
    program.blank();

    let all = lfind_elems(search, high);
    let elems = &all[offset..];
    let mask = if search.wide() { u32::MAX } else { 0xff };
    let mut found = [0i128; 4];
    let mut pattern = 0u64;
    for &len in &lengths {
        let at = |index: usize| {
            if search.le() { elems[index].wrapping_sub(1) & mask } else { elems[index] }
        };
        let keys = [at(0), at(len / 2), at(len.saturating_sub(1)), search.absent(high)];
        for (k, &key) in keys.iter().enumerate() {
            let got = search.scalar(key, &elems[..len]);
            found[k] += i128::from(got);
            pattern = pattern.wrapping_mul(3).wrapping_add(u64::from(got));
        }
    }
    for (k, count) in found.iter().enumerate() {
        program.check(Ty::I32, &format!("found[{k}]"), *count);
    }
    program.check(Ty::U64, "pattern", i128::from(pattern));
    program.check(Ty::I32, "mismatches", 0);
    program
}

/// The shape of one `frame-size` function.
#[derive(Debug, Clone, Copy)]
struct Frame {
    /// How many scalar locals it sets before its calls and reads after them.
    locals: usize,
    /// How many `unsigned int` its array has, or none.
    array: usize,
    /// How many structs it has.
    structs: usize,
    /// `leaf`, `calls` or `recursive`.
    calls: &'static str,
}

/// How deep a recursive `frame-size` function goes.
const FRAME_DEPTH: i32 = 3;

impl Frame {
    /// The recursion depth `main` asks for.
    fn depth(self) -> i32 {
        if self.calls == "recursive" { FRAME_DEPTH } else { 0 }
    }

    /// What `frame_fn(seed, depth)` returns, worked out the way the C does it.
    fn model(self, seed: u64, depth: i32) -> u64 {
        let mut locals = vec![seed.wrapping_mul(3).wrapping_add(1)];
        for i in 1..self.locals {
            let prev = locals[i - 1];
            locals.push(prev.wrapping_mul(2 * i as u64 + 3).wrapping_add(i as u64));
        }
        let array: Vec<u32> = (0..self.array as u64)
            .map(|i| seed.wrapping_add(i.wrapping_mul(2_654_435_761)) as u32)
            .collect();
        struct Record {
            key: u64,
            vals: [u32; 6],
            tag: u16,
            name: [u8; 21],
        }
        let mut records: Vec<Record> = (0..self.structs)
            .map(|s| {
                let base = locals[(s + 1) % self.locals];
                Record {
                    key: locals[s % self.locals].wrapping_add(s as u64),
                    vals: std::array::from_fn(|i| base.wrapping_add(7 * i as u64) as u32),
                    tag: (s + 1) as u16,
                    name: std::array::from_fn(|i| seed.wrapping_add((i * (s + 3)) as u64) as u8),
                }
            })
            .collect();
        let mut acc = 0u64;
        if !array.is_empty() {
            let sum =
                array.iter().fold(0u64, |s, &w| s.wrapping_mul(33).wrapping_add(u64::from(w)));
            acc = acc.wrapping_add(sum);
        }
        for record in &mut records {
            record.tag = record.tag.wrapping_add(1);
            let mut s = record.key;
            for &v in &record.vals {
                s = s.wrapping_mul(7).wrapping_add(u64::from(v));
            }
            for &c in &record.name {
                s = s.wrapping_add(u64::from(c));
            }
            acc = acc.wrapping_add(s.wrapping_add(u64::from(record.tag)));
        }
        acc = acc.wrapping_mul(3).wrapping_add(locals[0]);
        if self.calls == "recursive" && depth > 0 {
            acc = acc.wrapping_add(self.model(seed.wrapping_add(1), depth - 1));
        }
        for &local in locals.iter().rev() {
            acc = acc.wrapping_mul(31).wrapping_add(local);
        }
        if !array.is_empty() {
            acc = acc
                .wrapping_add(u64::from(array[self.array - 1]))
                .wrapping_add(u64::from(array[self.array / 2]));
        }
        for record in &records {
            acc = acc
                .wrapping_mul(31)
                .wrapping_add(record.key)
                .wrapping_add(u64::from(record.tag))
                .wrapping_add(u64::from(record.vals[5]))
                .wrapping_add(u64::from(record.name[20]));
        }
        acc
    }
}

/// Functions with large frames and a simple, checked answer.
///
/// The axes are what makes a frame large. `locals` is how many scalars are set before the
/// calls and read after them, so every one of them is alive across every call. `array` is an
/// `unsigned int` array of none, a kilobyte or sixty four kilobytes, the last being the size of
/// the buffers Postgres keeps on the stack for a page or a tuple. `structs` is a handful of
/// records with an awkward layout. `calls` is a leaf that does all of it inline, one that hands
/// the array and the records to functions by address, and one that also recurses, which is the
/// shape that turns a large frame into a deep stack.
///
/// The checksum is what is checked here. The frame size is what a later comparison against
/// `gcc -fstack-usage` reads off the same program.
fn frame_size(sink: &mut Sink<'_>) {
    const LOCALS: &[usize] = &[8, 32, 96];
    const ARRAYS: &[usize] = &[0, 256, 16384];
    const STRUCTS: &[usize] = &[0, 4];
    const CALLS: &[&str] = &["leaf", "calls", "recursive"];
    for &locals in LOCALS {
        for &array in ARRAYS {
            for &structs in STRUCTS {
                for &calls in CALLS {
                    if !sink.wants(Facet::FrameSize) {
                        return;
                    }
                    let frame = Frame { locals, array, structs, calls };
                    let axes = Axes::of([
                        ("locals", &locals.to_string()),
                        ("array", &array.to_string()),
                        ("structs", &structs.to_string()),
                        ("calls", calls),
                    ]);
                    sink.push_tagged(
                        Facet::FrameSize,
                        axes,
                        Dialect::C17,
                        frame_program(frame),
                        &["gnu", PROVENANCE],
                    );
                }
            }
        }
    }
}

/// One `frame-size` case.
fn frame_program(frame: Frame) -> Program {
    let Frame { locals, array, structs, calls } = frame;
    let mut program = Program::new(format!(
        "a {calls} function with {locals} locals, an array of {array} and {structs} structs"
    ));
    let inline = calls == "leaf";
    if structs > 0 {
        program.top("struct record {");
        program.top("    unsigned long long key;");
        program.top("    unsigned int vals[6];");
        program.top("    unsigned short tag;");
        program.top("    unsigned char name[21];");
        program.top("};");
    }
    if !inline {
        if array > 0 {
            program.top("__attribute__((noinline))");
            program.top("static unsigned long long sum_words(const unsigned int *p, int n) {");
            program.top("    unsigned long long s = 0;");
            program.top("    for (int i = 0; i < n; i++) {");
            program.top("        s = s * 33 + p[i];");
            program.top("    }");
            program.top("    return s;");
            program.top("}");
        }
        if structs > 0 {
            program.top("__attribute__((noinline))");
            program.top("static unsigned long long mix_record(struct record *r) {");
            program.top("    r->tag = (unsigned short)(r->tag + 1);");
            program.top("    unsigned long long s = r->key;");
            program.top("    for (int i = 0; i < 6; i++) {");
            program.top("        s = s * 7 + r->vals[i];");
            program.top("    }");
            program.top("    for (int i = 0; i < 21; i++) {");
            program.top("        s = s + r->name[i];");
            program.top("    }");
            program.top("    return s + r->tag;");
            program.top("}");
        }
        program.top("__attribute__((noinline))");
        program
            .top("static unsigned long long touch(unsigned long long a, unsigned long long b) {");
        program.top("    return a * 3 + b;");
        program.top("}");
    }
    program.top("__attribute__((noinline))");
    program.top("static unsigned long long frame_fn(unsigned long long seed, int depth) {");
    program.top("    unsigned long long l0 = seed * 3 + 1;");
    for i in 1..locals {
        program.top(format!("    unsigned long long l{i} = l{} * {} + {i};", i - 1, 2 * i + 3));
    }
    if array > 0 {
        program.top(format!("    unsigned int arr[{array}];"));
        program.top(format!("    for (int i = 0; i < {array}; i++) {{"));
        program.top("        arr[i] = (unsigned int)(seed + (unsigned long long)i * 2654435761u);");
        program.top("    }");
    }
    for s in 0..structs {
        program.top(format!("    struct record r{s};"));
        program.top(format!("    r{s}.key = l{} + {s};", s % locals));
        program.top("    for (int i = 0; i < 6; i++) {");
        program.top(format!(
            "        r{s}.vals[i] = (unsigned int)(l{} + 7 * (unsigned long long)i);",
            (s + 1) % locals
        ));
        program.top("    }");
        program.top(format!("    r{s}.tag = {};", s + 1));
        program.top("    for (int i = 0; i < 21; i++) {");
        program.top(format!("        r{s}.name[i] = (unsigned char)(seed + i * {});", s + 3));
        program.top("    }");
    }
    program.top("    unsigned long long acc = 0;");
    if array > 0 {
        if inline {
            program.top("    unsigned long long words = 0;");
            program.top(format!("    for (int i = 0; i < {array}; i++) {{"));
            program.top("        words = words * 33 + arr[i];");
            program.top("    }");
            program.top("    acc = acc + words;");
        } else {
            program.top(format!("    acc = acc + sum_words(arr, {array});"));
        }
    }
    for s in 0..structs {
        if inline {
            program.top(format!("    r{s}.tag = (unsigned short)(r{s}.tag + 1);"));
            program.top(format!("    unsigned long long mix{s} = r{s}.key;"));
            program.top("    for (int i = 0; i < 6; i++) {");
            program.top(format!("        mix{s} = mix{s} * 7 + r{s}.vals[i];"));
            program.top("    }");
            program.top("    for (int i = 0; i < 21; i++) {");
            program.top(format!("        mix{s} = mix{s} + r{s}.name[i];"));
            program.top("    }");
            program.top(format!("    acc = acc + mix{s} + r{s}.tag;"));
        } else {
            program.top(format!("    acc = acc + mix_record(&r{s});"));
        }
    }
    if inline {
        program.top("    acc = acc * 3 + l0;");
    } else {
        program.top("    acc = touch(acc, l0);");
    }
    if calls == "recursive" {
        program.top("    if (depth > 0) {");
        program.top("        acc = acc + frame_fn(seed + 1, depth - 1);");
        program.top("    }");
    } else {
        program.top("    (void)depth;");
    }
    for i in (0..locals).rev() {
        program.top(format!("    acc = acc * 31 + l{i};"));
    }
    if array > 0 {
        program.top(format!("    acc = acc + arr[{}] + arr[{}];", array - 1, array / 2));
    }
    for s in 0..structs {
        program.top(format!(
            "    acc = acc * 31 + r{s}.key + r{s}.tag + r{s}.vals[5] + r{s}.name[20];"
        ));
    }
    program.top("    return acc;");
    program.top("}");

    let seed = u64::from(PG_SEED);
    program.input(Ty::U64, "seed", i128::from(seed));
    for k in 0..3u64 {
        program.check(
            Ty::U64,
            &format!("frame_fn(seed + {k}, {})", frame.depth()),
            i128::from(frame.model(seed + k, frame.depth())),
        );
    }
    program
}

/// What one opcode of an `interpreter-dispatch` program does, handed out in turn.
///
/// They are the shapes of the handlers in `ExecInterpExpr`: arithmetic on the value being
/// built, a call out to a function the step names, a conditional jump forward, a jump back
/// that `loops` bounds, and a store through the state pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Handler {
    Accum,
    Call,
    Branch,
    Xor,
    Loop,
    Store,
}

const HANDLERS: [Handler; 6] =
    [Handler::Accum, Handler::Call, Handler::Branch, Handler::Xor, Handler::Loop, Handler::Store];

impl Handler {
    const fn name(self) -> &'static str {
        match self {
            Self::Accum => "ACCUM",
            Self::Call => "CALL",
            Self::Branch => "BRANCH",
            Self::Xor => "XOR",
            Self::Loop => "LOOP",
            Self::Store => "STORE",
        }
    }
}

/// How many values `data` holds, which the pointer values index into.
const DISPATCH_DATA: usize = 64;

/// What `data` holds.
fn dispatch_data() -> [u64; DISPATCH_DATA] {
    std::array::from_fn(|i| {
        let x = (i as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        x ^ (x >> 31)
    })
}

/// The bound `main` puts on the jumps back, read out of a `volatile`.
const DISPATCH_LOOPS: u32 = 200;

/// One step of an `interpreter-dispatch` program.
#[derive(Debug, Clone, Copy)]
struct Step {
    /// An index into the opcodes, where `ops` is `EEOP_AGAIN` and `ops + 1` is `EEOP_DONE`.
    opcode: usize,
    arg: u64,
    jump: usize,
}

/// What one run of the model saw.
struct Dispatched {
    hash: u64,
    /// How many handlers ran, `EEOP_DONE` included.
    dispatches: usize,
    /// Whether each opcode ran at least once.
    seen: Vec<bool>,
}

/// The shape of one `interpreter-dispatch` case.
#[derive(Debug, Clone, Copy)]
struct Interp {
    /// How many values are alive across every dispatch, besides the accumulator, the loop
    /// bound, the state and the step pointer, which every case has.
    live: usize,
    /// How many opcodes there are besides `EEOP_AGAIN` and `EEOP_DONE`.
    ops: usize,
    /// `table` or `threaded`.
    dispatch: &'static str,
    /// `none`, `direct` or `pointer`.
    calls: &'static str,
    /// Whether the steps are read through the state, sixty four bytes each, and jumped to as
    /// `&state->steps[op->jump]`, rather than handed in.
    state: bool,
}

impl Interp {
    fn handler(k: usize) -> Handler {
        HANDLERS[k % HANDLERS.len()]
    }

    /// The name of an opcode in the enum and, with `CASE_` in front, of its label.
    fn opcode(self, k: usize) -> String {
        if k == self.ops {
            "EEOP_AGAIN".to_owned()
        } else if k == self.ops + 1 {
            "EEOP_DONE".to_owned()
        } else {
            format!("EEOP_{}_{k}", Self::handler(k).name())
        }
    }

    /// The value opcode `k` reads.
    const fn reads(self, k: usize) -> usize {
        (k * 7 + 3) % self.live
    }

    /// The value opcode `k` writes, if it writes one.
    ///
    /// Only the even ones are ever written, so half the values are set once at the top and
    /// read at the bottom, and have to come through every jump and every call untouched.
    fn writes(self, k: usize) -> Option<usize> {
        match Self::handler(k) {
            Handler::Loop | Handler::Store => None,
            _ => Some(2 * ((k * 5 + 1) % (self.live / 2))),
        }
    }

    fn shift(k: usize) -> u32 {
        (k % 13 + 1) as u32
    }

    fn multiplier(k: usize) -> u64 {
        2 * k as u64 + 3
    }

    /// The program: every opcode twice over, then a jump back to the start and the end.
    fn steps(self) -> Vec<Step> {
        let body = 2 * self.ops;
        let mut steps: Vec<Step> = (0..body)
            .map(|p| {
                let opcode = (p * 5 + 3) % self.ops;
                // A branch in the first pass lands on the next step either way, and a loop only
                // ever jumps back, so the first pass runs every handler whatever the seed. The
                // second pass skips ahead for real.
                let jump = match Self::handler(opcode) {
                    Handler::Branch if p < self.ops => p + 1,
                    Handler::Branch => (p + 2 + p % 3).min(body),
                    Handler::Loop => p.saturating_sub(1 + p % 4),
                    _ => 0,
                };
                Step { opcode, arg: ((p * 37 + opcode * 11 + 5) % 997) as u64, jump }
            })
            .collect();
        steps.push(Step { opcode: self.ops, arg: 0, jump: 0 });
        steps.push(Step { opcode: self.ops + 1, arg: 0, jump: 0 });
        steps
    }

    /// What `run(interp, seed, loops)` returns, worked out the way either interpreter does it.
    fn model(self, seed: u64, loops: u32) -> Dispatched {
        let data = dispatch_data();
        let steps = self.steps();
        // A pointer value is held as its index into `data`, and an `unsigned int` one as its
        // value widened.
        let mut values: Vec<u64> = (0..self.live)
            .map(|i| match i % 3 {
                0 => seed.wrapping_mul(2 * i as u64 + 3).wrapping_add(i as u64),
                1 => u64::from(((seed >> (i % 29)) as u32).wrapping_add(i as u32)),
                _ => seed.wrapping_add(i as u64) & 63,
            })
            .collect();
        let read = |values: &[u64], i: usize| {
            if i % 3 == 2 { data[values[i] as usize] } else { values[i] }
        };
        let mut acc = seed;
        let mut loops = loops;
        let mut out: [u64; 8] = std::array::from_fn(|i| i as u64);
        let mut seen = vec![false; self.ops + 2];
        let mut dispatches = 0;
        let mut pc = 0;
        loop {
            let step = steps[pc];
            dispatches += 1;
            seen[step.opcode] = true;
            if step.opcode == self.ops + 1 {
                break;
            }
            if step.opcode == self.ops {
                if loops != 0 {
                    loops -= 1;
                    pc = step.jump;
                } else {
                    pc += 1;
                }
                continue;
            }
            let k = step.opcode;
            let arg = step.arg;
            let r = read(&values, self.reads(k));
            let mut next = pc + 1;
            let mut write = true;
            match Self::handler(k) {
                Handler::Accum => acc = acc.wrapping_mul(31).wrapping_add(r).wrapping_add(arg),
                Handler::Call => {
                    acc = (acc ^ (r >> (arg & 31)))
                        .wrapping_mul(Self::multiplier(k))
                        .wrapping_add(arg);
                }
                Handler::Branch => {
                    if (acc >> (arg & 31)) & 1 == 1 {
                        next = step.jump;
                        write = false;
                    }
                }
                Handler::Xor => acc = (acc ^ (r << Self::shift(k))).wrapping_add(arg),
                Handler::Loop => {
                    if loops != 0 {
                        loops -= 1;
                        acc = acc.wrapping_add(r);
                        next = step.jump;
                    }
                }
                Handler::Store => {
                    let at = (arg & 7) as usize;
                    out[at] = out[at].wrapping_mul(5).wrapping_add(acc ^ r);
                }
            }
            if let Some(w) = self.writes(k).filter(|_| write) {
                values[w] = match w % 3 {
                    0 => values[w].wrapping_mul(3).wrapping_add(acc),
                    1 => u64::from((values[w] as u32).wrapping_add(acc as u32)),
                    _ => (values[w] + (acc & 7) + 1) & 63,
                };
            }
            pc = next;
        }
        let mut hash = acc;
        for i in 0..self.live {
            hash = hash.wrapping_mul(1_000_003).wrapping_add(read(&values, i));
        }
        hash = hash.wrapping_mul(1_000_003).wrapping_add(u64::from(loops));
        for value in out {
            hash = hash.wrapping_mul(31).wrapping_add(value);
        }
        Dispatched { hash, dispatches, seen }
    }

    /// A value read as an `unsigned long long`.
    fn read_expr(i: usize) -> String {
        match i % 3 {
            0 => format!("v{i}"),
            1 => format!("(unsigned long long)v{i}"),
            _ => format!("*v{i}"),
        }
    }

    /// The statement that writes a value from `acc`.
    fn write_stmt(w: usize) -> String {
        match w % 3 {
            0 => format!("v{w} = v{w} * 3ull + acc;"),
            1 => format!("v{w} = v{w} + (unsigned int)acc;"),
            _ => format!("v{w} = &data[(v{w} - data + (long long)(acc & 7ull) + 1) & 63];"),
        }
    }

    /// The body of the interpreter, the same text for both builds of it, in terms of the
    /// `EEO_` macros the way `execExprInterp.c` is written.
    fn body(self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("    unsigned long long acc = state->seed;".to_owned());
        lines.push("    unsigned int loops = state->loops;".to_owned());
        for i in 0..self.live {
            lines.push(match i % 3 {
                0 => format!(
                    "    unsigned long long v{i} = state->seed * {}ull + {i}ull;",
                    2 * i + 3
                ),
                1 => format!(
                    "    unsigned int v{i} = (unsigned int)(state->seed >> {}) + {i}u;",
                    i % 29
                ),
                _ => format!(
                    "    const unsigned long long *v{i} = &data[(state->seed + {i}ull) & 63];"
                ),
            });
        }
        lines.push(
            if self.state {
                "    struct step *op = state->steps;"
            } else {
                "    struct step *op = steps;"
            }
            .to_owned(),
        );
        lines.push(String::new());
        lines.push("    EEO_DISPATCH();".to_owned());
        lines.push("    EEO_SWITCH()".to_owned());
        lines.push("    {".to_owned());
        for k in 0..self.ops {
            let read = Self::read_expr(self.reads(k));
            let write = self.writes(k).map(Self::write_stmt);
            lines.push(format!("        EEO_CASE({})", self.opcode(k)));
            lines.push("        {".to_owned());
            match Self::handler(k) {
                Handler::Accum => lines.push(format!(
                    "            acc = acc * 31ull + {read} + (unsigned long long)op->arg;"
                )),
                Handler::Call => lines.push(match self.calls {
                    "none" => format!(
                        "            acc = (acc ^ ({read} >> (op->arg & 31))) * {}ull + (unsigned long long)op->arg;",
                        Self::multiplier(k)
                    ),
                    "direct" => format!("            acc = fn_{k}(acc, {read}, op->arg);"),
                    _ => format!("            acc = op->fn(acc, {read}, op->arg);"),
                }),
                Handler::Branch => {
                    lines.push("            if ((acc >> (op->arg & 31)) & 1ull)".to_owned());
                    lines.push("                EEO_JUMP(op->jump);".to_owned());
                }
                Handler::Xor => lines.push(format!(
                    "            acc = (acc ^ ({read} << {})) + (unsigned long long)op->arg;",
                    Self::shift(k)
                )),
                Handler::Loop => {
                    lines.push("            if (loops != 0) {".to_owned());
                    lines.push("                loops = loops - 1;".to_owned());
                    lines.push(format!("                acc = acc + {read};"));
                    lines.push("                EEO_JUMP(op->jump);".to_owned());
                    lines.push("            }".to_owned());
                }
                Handler::Store => lines.push(if self.calls == "none" {
                    format!(
                        "            state->out[op->arg & 7] = state->out[op->arg & 7] * 5ull + (acc ^ {read});"
                    )
                } else {
                    format!("            note(state, op->arg & 7, acc ^ {read});")
                }),
            }
            if let Some(write) = write {
                lines.push(format!("            {write}"));
            }
            lines.push("            EEO_NEXT();".to_owned());
            lines.push("        }".to_owned());
        }
        lines.push("        EEO_CASE(EEOP_AGAIN)".to_owned());
        lines.push("        {".to_owned());
        lines.push("            if (loops != 0) {".to_owned());
        lines.push("                loops = loops - 1;".to_owned());
        lines.push("                EEO_JUMP(op->jump);".to_owned());
        lines.push("            }".to_owned());
        lines.push("            EEO_NEXT();".to_owned());
        lines.push("        }".to_owned());
        lines.push("        EEO_CASE(EEOP_DONE)".to_owned());
        lines.push("        {".to_owned());
        lines.push("            goto out;".to_owned());
        lines.push("        }".to_owned());
        lines.push("    }".to_owned());
        lines.push(String::new());
        lines.push("out:".to_owned());
        lines.push("    {".to_owned());
        lines.push("        unsigned long long h = acc;".to_owned());
        for i in 0..self.live {
            lines.push(format!("        h = h * 1000003ull + {};", Self::read_expr(i)));
        }
        lines.push("        return h * 1000003ull + loops;".to_owned());
        lines.push("    }".to_owned());
        lines
    }
}

/// A bytecode interpreter that jumps through label addresses, checked against itself over a
/// `switch`.
///
/// The axes are the ones `ExecInterpExpr` varies on. `live` is how many values the handlers
/// keep alive across every dispatch on top of the accumulator, the loop bound, the state and
/// the step pointer: four fit in registers anywhere, twelve is about what x86-64 keeps across a
/// call, and twenty four is more than it has. Values of three kinds, `unsigned long long` like a
/// `Datum`, `unsigned int` like a flag, and a pointer like a slot, and only some handlers write
/// some of them, so the rest are set once and must come through every jump unchanged.
/// `dispatch` is `goto *` through the table of label addresses, or through the address each
/// step was threaded with before the first run, which is what `ExecReadyInterpretedExpr` does
/// to every step Postgres builds. `calls` is whether the handlers compute everything inline,
/// call out to functions that are not inlined, or call through a function pointer the step
/// carries, the way `EEOP_FUNCEXPR` calls `fn_addr`. `ops` is how many handlers there are.
///
/// Every program builds the same body twice with different `EEO_` macros, once over computed
/// goto and once over a `switch` the way Postgres builds it where `EEO_USE_COMPUTED_GOTO` is
/// off, and prints what both return for three seeds. Both have to print the answer the model
/// works out.
///
/// The cases with 32 opcodes are built a second time with `steps=state`, where the steps are
/// read through the state and padded to the sixty four bytes of an `ExprEvalStep`, so a jump is
/// `op = &state->steps[op->jump]`, a shift and an add over a pointer loaded in the handler. That
/// is the jump rucc got wrong at `-O2` in tamnd/rucc#3395, where the add wrote the step into
/// the register the next handler reads it in and the copy of the steps in front of it went over
/// the index. Whether the index is in that register depends on everything else the function keeps
/// in registers, and in an interpreter this small the add comes out as a `lea` into a free one, so
/// these do not crash a rucc without the fix. They hold the shape, which none of the others had.
fn interpreter_dispatch(sink: &mut Sink<'_>) {
    const LIVE: &[usize] = &[4, 12, 24];
    const DISPATCHES: &[&str] = &["table", "threaded"];
    const CALLS: &[&str] = &["none", "direct", "pointer"];
    const OPS: &[usize] = &[8, 32];
    for &live in LIVE {
        for &dispatch in DISPATCHES {
            for &calls in CALLS {
                for &ops in OPS {
                    if !sink.wants(Facet::InterpreterDispatch) {
                        return;
                    }
                    let axes = Axes::of([
                        ("live", &live.to_string()),
                        ("dispatch", dispatch),
                        ("calls", calls),
                        ("ops", &ops.to_string()),
                    ]);
                    sink.push_tagged(
                        Facet::InterpreterDispatch,
                        axes,
                        Dialect::C17,
                        dispatch_program(Interp { live, ops, dispatch, calls, state: false }),
                        &["gnu", PROVENANCE],
                    );
                }
            }
        }
    }
    for &live in LIVE {
        for &dispatch in DISPATCHES {
            for &calls in CALLS {
                if !sink.wants(Facet::InterpreterDispatch) {
                    return;
                }
                let axes = Axes::of([
                    ("live", &live.to_string()),
                    ("dispatch", dispatch),
                    ("calls", calls),
                    ("ops", "32"),
                    ("steps", "state"),
                ]);
                sink.push_tagged(
                    Facet::InterpreterDispatch,
                    axes,
                    Dialect::C17,
                    dispatch_program(Interp { live, ops: 32, dispatch, calls, state: true }),
                    &["gnu", PROVENANCE],
                );
            }
        }
    }
}

/// One `interpreter-dispatch` case.
fn dispatch_program(interp: Interp) -> Program {
    let Interp { live, ops, dispatch, calls, state } = interp;
    let mut program = Program::new(format!(
        "an interpreter of {ops} opcodes with {live} values alive, dispatched through the {}{} and \
         checked against a switch",
        if dispatch == "table" { "label table" } else { "threaded steps" },
        if state { " from steps read through the state" } else { "" }
    ));
    let threaded = dispatch == "threaded";
    let pointer = calls == "pointer";

    program.top("struct exprstate {");
    program.top("    unsigned long long seed;");
    program.top("    unsigned int loops;");
    program.top("    unsigned long long out[8];");
    if state {
        program.top("    struct step *steps;");
    }
    program.top("};");
    program.top("");
    program.top("struct step {");
    program.top("    int opcode;");
    program.top("    int arg;");
    program.top("    int jump;");
    if pointer {
        program.top("    unsigned long long (*fn)(unsigned long long, unsigned long long, int);");
    }
    if threaded {
        program.top("    void *code;");
    }
    if state {
        // Sixty four bytes on LP64, the size of an `ExprEvalStep`, so the index is scaled by a
        // shift and the jump is an add of two registers.
        program.top(format!(
            "    unsigned long long pad[{}];",
            6 - usize::from(pointer) - usize::from(threaded)
        ));
    }
    program.top("};");
    program.top("");
    program.top(format!("static const unsigned long long data[{DISPATCH_DATA}] = {{"));
    for row in dispatch_data().chunks(4) {
        let row: Vec<String> = row.iter().map(|v| format!("0x{v:016x}ull")).collect();
        program.top(format!("    {},", row.join(", ")));
    }
    program.top("};");
    program.top("");

    if calls != "none" {
        for k in (0..ops).filter(|&k| Interp::handler(k) == Handler::Call) {
            program.top("__attribute__((noinline))");
            program.top(format!(
                "static unsigned long long fn_{k}(unsigned long long x, unsigned long long y, int arg) {{"
            ));
            program.top(format!(
                "    return (x ^ (y >> (arg & 31))) * {}ull + (unsigned long long)arg;",
                Interp::multiplier(k)
            ));
            program.top("}");
            program.top("");
        }
        program.top("__attribute__((noinline))");
        program
            .top("static void note(struct exprstate *state, int at, unsigned long long value) {");
        program.top("    state->out[at] = state->out[at] * 5ull + value;");
        program.top("}");
        program.top("");
    }

    let names: Vec<String> = (0..ops + 2).map(|k| interp.opcode(k)).collect();
    program.top("enum {");
    for name in &names {
        program.top(format!("    {name},"));
    }
    program.top("};");
    program.top("");

    let steps = interp.steps();
    program.top(format!("static struct step steps[{}] = {{", steps.len()));
    for step in &steps {
        let mut fields =
            format!(".opcode = {}, .arg = {}, .jump = {}", names[step.opcode], step.arg, step.jump);
        if pointer && step.opcode < ops && Interp::handler(step.opcode) == Handler::Call {
            let _ = write!(fields, ", .fn = fn_{}", step.opcode);
        }
        program.top(format!("    {{ {fields} }},"));
    }
    program.top("};");
    program.top("");

    if threaded {
        // The table is only in scope inside the interpreter, so the interpreter hands it out
        // when it is called with no steps, the way ExecInterpExpr does with no state.
        program.top("static const void *const *thread_table;");
        program.top("");
    }
    program.top("#define EEO_CASE(name) CASE_##name:");
    program.top("#define EEO_SWITCH()");
    if threaded {
        program.top("#define EEO_DISPATCH() goto *op->code");
    } else {
        program.top("#define EEO_DISPATCH() goto *((void *) dispatch_table[op->opcode])");
    }
    program.top("#define EEO_NEXT() do { op++; EEO_DISPATCH(); } while (0)");
    if state {
        program
            .top("#define EEO_JUMP(to) do { op = &state->steps[to]; EEO_DISPATCH(); } while (0)");
    } else {
        program.top("#define EEO_JUMP(to) do { op = steps + (to); EEO_DISPATCH(); } while (0)");
    }
    program.top("");
    program.top("__attribute__((noinline))");
    program.top(
        "static unsigned long long interp_goto(struct step *steps, struct exprstate *state) {",
    );
    program.top("    static const void *const dispatch_table[] = {");
    for name in &names {
        program.top(format!("        &&CASE_{name},"));
    }
    program.top("    };");
    if threaded {
        program.top("    if (steps == 0) {");
        program.top("        thread_table = dispatch_table;");
        program.top("        return 0;");
        program.top("    }");
    }
    for line in interp.body() {
        program.top(line);
    }
    program.top("}");
    program.top("");
    program.top("#undef EEO_CASE");
    program.top("#undef EEO_SWITCH");
    program.top("#undef EEO_DISPATCH");
    program.top("#define EEO_CASE(name) case name:");
    program.top("#define EEO_SWITCH() starteval: switch (op->opcode)");
    program.top("#define EEO_DISPATCH() goto starteval");
    program.top("");
    program.top("__attribute__((noinline))");
    program.top(
        "static unsigned long long interp_switch(struct step *steps, struct exprstate *state) {",
    );
    for line in interp.body() {
        program.top(line);
    }
    program.top("}");
    program.top("");

    if threaded {
        program.top("static void thread_steps(void) {");
        program.top("    interp_goto(0, 0);");
        program.top(format!("    for (int i = 0; i < {}; i++) {{", steps.len()));
        program.top("        steps[i].code = (void *) thread_table[steps[i].opcode];");
        program.top("    }");
        program.top("}");
        program.top("");
    }
    program.top("static unsigned long long run(");
    program.top("    unsigned long long (*interp)(struct step *, struct exprstate *),");
    program.top("    unsigned long long seed, unsigned int loops) {");
    program.top("    struct exprstate state;");
    program.top("    state.seed = seed;");
    program.top("    state.loops = loops;");
    if state {
        program.top("    state.steps = steps;");
    }
    program.top("    for (int i = 0; i < 8; i++) {");
    program.top("        state.out[i] = (unsigned long long)i;");
    program.top("    }");
    program.top("    unsigned long long h = interp(steps, &state);");
    program.top("    for (int i = 0; i < 8; i++) {");
    program.top("        h = h * 31ull + state.out[i];");
    program.top("    }");
    program.top("    return h;");
    program.top("}");

    let seed = u64::from(PG_SEED);
    program.input(Ty::U64, "seed", i128::from(seed));
    program.input(Ty::U32, "loops", i128::from(DISPATCH_LOOPS));
    if threaded {
        program.line("thread_steps();");
    }
    for t in 0..3u64 {
        let run = interp.model(seed + t, DISPATCH_LOOPS);
        // A handler that never runs is a handler nobody checked, and a program that never went
        // round has hardly dispatched at all.
        debug_assert!(run.seen.iter().all(|&seen| seen), "{interp:?} leaves a handler dead");
        debug_assert!(run.dispatches > 2 * steps.len(), "{interp:?} barely loops");
        let expected = i128::from(run.hash);
        program.check(Ty::U64, &format!("run(interp_goto, seed + {t}, loops)"), expected);
        program.check(Ty::U64, &format!("run(interp_switch, seed + {t}, loops)"), expected);
    }
    program
}

#[cfg(test)]
mod tests {
    use super::{Exact, INTS, Int, Interp, crc32c, int_literal, operand_types};
    use crate::{Options, Sink};
    use corpus_model::{Case, Facet};

    fn cases() -> Vec<Case> {
        let opts = Options::all();
        let mut sink = Sink::new(&opts);
        super::generate(&mut sink);
        sink.into_cases()
    }

    fn exact(value: i128) -> Exact {
        Exact::of(value < 0, value.unsigned_abs())
    }

    fn ty(name: &str) -> Int {
        INTS.into_iter().find(|ty| ty.name == name).unwrap()
    }

    #[test]
    fn every_facet_produces_cases_and_every_case_says_where_it_came_from() {
        let cases = cases();
        for facet in [
            Facet::Sigsetjmp,
            Facet::OverflowBuiltins,
            Facet::TargetAttribute,
            Facet::Crc32c,
            Facet::SimdLfind,
            Facet::FrameSize,
            Facet::InterpreterDispatch,
        ] {
            assert!(cases.iter().any(|case| case.facet == facet), "nothing for {facet}");
        }
        for case in &cases {
            assert!(case.has_tag("provenance:postgres"), "{} is untagged", case.id);
        }
    }

    #[test]
    fn only_the_cases_that_need_x86_64_say_so() {
        for case in cases() {
            let needs =
                matches!(case.facet, Facet::TargetAttribute | Facet::Crc32c | Facet::SimdLfind);
            assert_eq!(case.has_tag("x86-64"), needs, "{}", case.id);
        }
    }

    #[test]
    fn the_crc32c_model_gives_the_published_check_value() {
        assert_eq!(crc32c(b"123456789"), 0xE306_9283);
        assert_eq!(crc32c(b""), 0);
    }

    #[test]
    fn exact_arithmetic_agrees_with_i128_wherever_i128_is_wide_enough() {
        let values = [i64::MIN as i128, -65_536, -3, -1, 0, 1, 7, 65_535, i64::MAX as i128];
        for &a in &values {
            for &b in &values {
                assert_eq!(exact(a).add(exact(b)), exact(a + b), "{a} + {b}");
                assert_eq!(exact(a).sub(exact(b)), exact(a - b), "{a} - {b}");
                assert_eq!(exact(a).mul(exact(b)), exact(a * b), "{a} * {b}");
            }
        }
    }

    #[test]
    fn a_product_past_128_bits_keeps_its_high_half() {
        let top = Exact::of(false, u128::MAX);
        let square = top.mul(top);
        // (2^128 - 1)^2 is 2^256 - 2^129 + 1.
        assert_eq!(square.high, u128::MAX - 1);
        assert_eq!(square.low, 1);
        assert!(!square.fits(ty("u128")));
        assert_eq!(square.wrapped(ty("u128")), 1);
    }

    #[test]
    fn fitting_and_wrapping_are_what_a_conversion_in_c_would_do() {
        let i8 = ty("i8");
        assert!(exact(-128).fits(i8));
        assert!(!exact(128).fits(i8));
        assert_eq!(exact(128).wrapped(i8), 0x80);
        assert_eq!(exact(-1).wrapped(ty("u16")), 0xffff);
        assert!(!exact(-1).fits(ty("u64")));
        assert!(exact(i128::MIN).fits(ty("i128")));
        assert!(Exact::of(false, u128::MAX).fits(ty("u128")));
        assert!(!Exact::of(false, u128::MAX).fits(ty("i128")));
    }

    #[test]
    fn a_literal_never_writes_a_positive_value_the_type_cannot_hold() {
        assert_eq!(int_literal(exact(-128)), "(-128ll)");
        assert_eq!(int_literal(exact(i64::MIN as i128)), "(-9223372036854775807ll - 1)");
        assert_eq!(
            int_literal(Exact::of(false, u128::MAX)),
            "(((unsigned __int128)18446744073709551615ull << 64) | 18446744073709551615ull)"
        );
        assert_eq!(
            int_literal(exact(i128::MIN)),
            "(-(__int128)(((unsigned __int128)9223372036854775807ull << 64) | 18446744073709551615ull) - 1)"
        );
    }

    #[test]
    fn the_crossed_shape_gives_three_different_types() {
        for result in INTS {
            let (left, right) = operand_types("crossed", result);
            assert!(left.signed && !right.signed);
            assert_ne!(left, result);
            assert_ne!(right, result);
        }
    }

    #[test]
    fn a_case_that_uses_long_says_it_needs_lp64_and_no_other_case_does() {
        for case in cases().iter().filter(|case| case.facet == Facet::OverflowBuiltins) {
            // The type of each operand table and of the result, read back out of the source.
            let types: Vec<&str> = case
                .source
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if let Some(rest) = line.strip_prefix("static volatile ") {
                        rest.split_once(" lhs[").or_else(|| rest.split_once(" rhs[")).map(|p| p.0)
                    } else {
                        line.strip_suffix(" result;")
                    }
                })
                .collect();
            assert_eq!(types.len(), 3, "{}", case.id);
            let uses_long = types.iter().any(|ty| *ty == "long" || *ty == "unsigned long");
            assert_eq!(case.has_tag("lp64"), uses_long, "{}", case.id);
        }
    }

    #[test]
    fn every_save_is_where_c_allows_it_and_every_written_local_is_volatile() {
        for case in cases().iter().filter(|case| case.facet == Facet::Sigsetjmp) {
            for line in case.source.lines().filter(|line| line.contains("setjmp(")) {
                let trimmed = line.trim();
                assert!(
                    trimmed.starts_with("if (setjmp(") || trimmed.starts_with("if (sigsetjmp("),
                    "{} saves somewhere C does not allow: {trimmed}",
                    case.id
                );
            }
            // A kept value the first arm writes has to be volatile, or reading it after the
            // jump is reading something indeterminate.
            for line in case.source.lines() {
                let trimmed = line.trim();
                let Some((name, _)) = trimmed.split_once(" = ") else { continue };
                if name.starts_with('v') && name[1..].bytes().all(|b| b.is_ascii_digit()) {
                    let declared = case
                        .source
                        .lines()
                        .find(|decl| decl.contains(&format!(" {name} = ")) && decl.contains("seed"))
                        .unwrap_or_default();
                    assert!(
                        declared.contains("volatile"),
                        "{} writes {name} after the save",
                        case.id
                    );
                }
            }
        }
    }

    #[test]
    fn the_sigsetjmp_cases_ask_for_posix_before_the_header() {
        for case in cases().iter().filter(|case| case.facet == Facet::Sigsetjmp) {
            if case.source.contains("sigsetjmp(") {
                let define = case.source.find("#define _POSIX_C_SOURCE").unwrap();
                let include = case.source.find("#include <setjmp.h>").unwrap();
                assert!(define < include, "{}", case.id);
            }
        }
    }

    #[test]
    fn there_are_cases_with_more_values_alive_than_x86_64_has_registers_to_keep_them_in() {
        let crowded = cases()
            .iter()
            .filter(|case| {
                case.facet == Facet::Sigsetjmp
                    && case.id.contains(".busy.")
                    && case.source.matches(" = seed ").count() > 12
            })
            .count();
        assert!(crowded > 0);
    }

    /// Every shape the facet walks, whether or not the calls go out of line.
    fn interpreters() -> Vec<Interp> {
        let mut out = Vec::new();
        for live in [4, 12, 24] {
            for ops in [8, 32] {
                for dispatch in ["table", "threaded"] {
                    for calls in ["none", "direct", "pointer"] {
                        out.push(Interp { live, ops, dispatch, calls, state: false });
                        if ops == 32 {
                            out.push(Interp { live, ops, dispatch, calls, state: true });
                        }
                    }
                }
            }
        }
        out
    }

    #[test]
    fn every_interpreter_is_built_twice_and_both_builds_print_the_same_answer() {
        let dispatch: Vec<Case> =
            cases().into_iter().filter(|case| case.facet == Facet::InterpreterDispatch).collect();
        assert_eq!(dispatch.len(), 54);
        for case in &dispatch {
            assert!(case.has_tag("gnu"), "{}", case.id);
            assert!(case.source.contains("&&CASE_EEOP_DONE"), "{} has no label table", case.id);
            assert!(case.source.contains("goto *"), "{} never jumps indirectly", case.id);
            assert!(case.source.contains("switch (op->opcode)"), "{} has no switch", case.id);
            let lines: Vec<&str> = case.expect.text().lines().collect();
            assert_eq!(lines.len(), 6, "{}", case.id);
            for pair in lines.chunks(2) {
                assert_eq!(pair[0], pair[1], "{}", case.id);
            }
            assert_ne!(lines[0], lines[2], "{} prints the same for two seeds", case.id);
        }
    }

    #[test]
    fn only_the_threaded_cases_store_label_addresses_in_the_steps() {
        for case in cases().iter().filter(|case| case.facet == Facet::InterpreterDispatch) {
            let threaded = case.axes.get("dispatch") == Some("threaded");
            assert_eq!(case.source.contains("steps[i].code = "), threaded, "{}", case.id);
            assert_eq!(case.source.contains("goto *op->code"), threaded, "{}", case.id);
            let calls = case.axes.get("calls") != Some("none");
            assert_eq!(case.source.contains("__attribute__((noinline))\nstatic void note("), calls);
            assert_eq!(case.source.contains("op->fn("), case.axes.get("calls") == Some("pointer"));
        }
    }

    #[test]
    fn only_the_state_cases_read_the_steps_through_the_state() {
        let dispatch: Vec<Case> =
            cases().into_iter().filter(|case| case.facet == Facet::InterpreterDispatch).collect();
        assert_eq!(
            dispatch.iter().filter(|case| case.axes.get("steps") == Some("state")).count(),
            18
        );
        for case in &dispatch {
            let state = case.axes.get("steps") == Some("state");
            assert_eq!(case.source.contains("op = &state->steps[to]"), state, "{}", case.id);
            assert_eq!(
                case.source.contains("struct step *op = state->steps;"),
                state,
                "{}",
                case.id
            );
            assert_eq!(case.source.contains("state.steps = steps;"), state, "{}", case.id);
            assert_eq!(case.source.contains("unsigned long long pad["), state, "{}", case.id);
        }
    }

    #[test]
    fn every_handler_runs_and_the_program_goes_round_more_than_once() {
        for interp in interpreters() {
            let steps = interp.steps();
            for seed in 0..3 {
                let run = interp.model(u64::from(super::PG_SEED) + seed, super::DISPATCH_LOOPS);
                assert!(run.seen.iter().all(|&seen| seen), "{interp:?} leaves a handler dead");
                assert!(run.dispatches > 2 * steps.len(), "{interp:?} barely loops");
            }
        }
    }

    #[test]
    fn some_values_are_written_by_some_handlers_and_the_rest_by_none() {
        for interp in interpreters() {
            let written: Vec<usize> = (0..interp.ops).filter_map(|k| interp.writes(k)).collect();
            assert!(!written.is_empty(), "{interp:?}");
            assert!((0..interp.live).any(|i| !written.contains(&i)), "{interp:?}");
            // Every kind of value is written somewhere once there are enough of them.
            if interp.live >= 12 {
                for kind in 0..3 {
                    assert!(written.iter().any(|w| w % 3 == kind), "{interp:?} kind {kind}");
                }
            }
        }
    }
}
