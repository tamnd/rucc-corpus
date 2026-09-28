//! Two constructs Postgres leans on everywhere that nothing else in the corpus covered closely.
//!
//! Both are correctness facets, and both are here because building Postgres with rucc found
//! them. Neither is an optimization. Each is a promise the compiler makes to the program, and
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
//! `overflow-builtins` is the arithmetic. `common/int.h` checks every add, subtract and
//! multiply that could overflow with `__builtin_add_overflow` and its two siblings, over
//! `int16`, `int32`, `int64`, their unsigned forms and `int128`. The builtins take the exact
//! result, store it wrapped into the type the third argument points at, and return whether it
//! fitted, and the three types involved are independent of each other.
//!
//! Every case here carries the `provenance:postgres` tag, so a run can pick them out.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// The tag every case in this module carries.
const PROVENANCE: &str = "provenance:postgres";

/// Emits everything in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    sigsetjmp(sink);
    overflow_builtins(sink);
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

/// One `sigsetjmp` case.
fn sigsetjmp_program(api: &str, shape: &str, live: usize, busy: bool) -> Program {
    let posix = api == "sigsetjmp";
    let (buffer, save, jump) = if posix {
        ("sigjmp_buf", "sigsetjmp", "siglongjmp")
    } else {
        ("jmp_buf", "setjmp", "longjmp")
    };
    let saved = |name: &str| {
        if posix { format!("{save}({name}, 0)") } else { format!("{save}({name})") }
    };
    let mut program = Program::new(format!(
        "{live} value{} live across {api} in the {shape} shape, with a {} first arm",
        if live == 1 { "" } else { "s" },
        if busy { "busy" } else { "quiet" }
    ));
    if posix {
        // With -std=c17 the C library shows only ISO C, and sigsetjmp is POSIX.
        program.define("_POSIX_C_SOURCE", "200809L");
    }
    program.include("setjmp.h");
    program.top(format!("static {buffer} *exception_stack;"));
    program.top("static volatile int depth;");
    program.top("static volatile long long busy_total;");
    program.top(format!("static int cells[{CELLS}];"));
    program.top(format!("static volatile long long seen[{live}];"));
    program.top("static void raise_error(int code) {");
    program.top(format!("    {jump}(*exception_stack, code);"));
    program.top("}");
    program.top("static void work(int code) {");
    program.top("    depth = depth + 1;");
    program.top("    if (code != 0) {");
    program.top("        raise_error(code);");
    program.top("    }");
    program.top("}");
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

#[cfg(test)]
mod tests {
    use super::{Exact, INTS, Int, int_literal, operand_types};
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
    fn both_facets_produce_cases_and_every_case_says_where_it_came_from() {
        let cases = cases();
        for facet in [Facet::Sigsetjmp, Facet::OverflowBuiltins] {
            assert!(cases.iter().any(|case| case.facet == facet), "nothing for {facet}");
        }
        for case in &cases {
            assert!(case.has_tag("provenance:postgres"), "{} is untagged", case.id);
        }
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
}
