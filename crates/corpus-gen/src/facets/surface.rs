//! The four facets named for a piece of language surface rather than for a pass.
//!
//! Every other facet in this corpus is named for a transformation, which is right when the
//! question is whether an optimization is correct and whether it pays. It is the wrong shape
//! for the other question a compiler has to answer, which is whether it implements a
//! construct at all. Real projects do not fail on `loop-unswitch`. They fail on an atomic
//! builtin with no lowering, on a bit counting builtin the back end never learned, on the
//! address of a label, and on a float to unsigned conversion. Those four are here.
//!
//! Each one lands in the phase where its failure actually is.
//!
//! `bit-builtins` and `float-conversion` are backend facets, because a compiler that gets
//! them wrong gets them wrong in instruction selection, and the failure a real project sees
//! is a missing lowering rule rather than a wrong answer.
//!
//! `computed-goto` is a floor facet, because the address of a label is a control flow shape
//! before it is anything else. An indirect jump is the one edge a compiler cannot see the
//! target of, and every analysis under every pass has to survive that.
//!
//! `atomics` is a correctness facet, for the same reason `barrier` is. Most of what an atomic
//! asks of a compiler is restraint, and a pass that fires across one is a bug even when the
//! single threaded answer stays right.
//!
//! All four are single threaded, print an answer worked out in Rust, and print nothing that
//! depends on the machine. Where a construct's answer is target specific, it is left out and
//! the reason is written down next to where it would have gone.

use crate::Sink;
use crate::emit::{Program, ident};
use crate::facets::{all_ones, lit};
use crate::lang::{Ty, interesting};
use corpus_model::{Axes, Dialect, Facet};

/// The widths where the bit builtins have an answer that is the same on every machine.
///
/// Two widths and not three. `__builtin_clzl` takes a `long`, which is 64 bits on Linux and
/// 32 bits on Windows, so `__builtin_clzl(1)` is 63 on one correct compiler and 31 on
/// another. Every program in this corpus prints the same text everywhere and those would not,
/// so the `l` family is left out and the two widths whose answers are fixed are covered over
/// every bit position instead.
const WIDTHS: &[(u32, &str)] = &[(32, ""), (64, "ll")];

/// The bit counting builtins, at both widths and over every bit position.
///
/// The `form` axis is the one that matters here. A compiler can pass the constant half of
/// this facet with a folder in the front end and still have no instruction for any of it, and
/// that is exactly the failure a real project hits, because a real project counts the bits of
/// a number it read from a file. The opaque half reads its argument out of `volatile` memory
/// so the instruction has to exist.
pub(crate) fn bit_builtins(sink: &mut Sink<'_>) {
    const BUILTINS: &[&str] = &["clz", "ctz", "popcount", "parity", "ffs"];
    for &(width, suffix) in WIDTHS {
        for &builtin in BUILTINS {
            for &form in &["constant", "opaque"] {
                if !sink.wants(Facet::BitBuiltins) {
                    return;
                }
                let ty = argument_ty(builtin, width);
                let values = bit_patterns(width);
                let mut program = Program::new(format!(
                    "__builtin_{builtin}{suffix} over every bit position at {width} bits, with the argument {}",
                    if form == "constant" { "written as a constant" } else { "read from memory" }
                ));
                let table = ident(&[builtin, &width.to_string(), "in"]);
                if form == "opaque" {
                    let items: Vec<String> =
                        values.iter().map(|v| ty.literal(ty.convert(*v as i128))).collect();
                    program.top(format!(
                        "static volatile {} {table}[{}] = {{ {} }};",
                        ty.c_name(),
                        values.len(),
                        items.join(", ")
                    ));
                    program.blank();
                }
                for (at, &value) in values.iter().enumerate() {
                    let argument = if form == "constant" {
                        lit(ty, ty.convert(value as i128))
                    } else {
                        format!("{table}[{at}]")
                    };
                    program.check(
                        Ty::I32,
                        &format!("__builtin_{builtin}{suffix}({argument})"),
                        bit_answer(builtin, width, value),
                    );
                }
                let axes =
                    Axes::of([("builtin", builtin), ("width", &width.to_string()), ("form", form)]);
                sink.push_tagged(Facet::BitBuiltins, axes, Dialect::C17, program, &["gnu"]);
            }
        }
    }
    zero_argument(sink);
}

/// The zero argument, where half the family is defined and half of it is not.
///
/// `__builtin_clz(0)` and `__builtin_ctz(0)` are undefined, and this corpus does not emit
/// undefined behaviour even to cover an edge, because a program with no defined answer has no
/// oracle and running it proves nothing. What real code writes instead is the conditional,
/// and the conditional is what is here. The other three are defined at zero and are checked
/// directly.
fn zero_argument(sink: &mut Sink<'_>) {
    for &(width, suffix) in WIDTHS {
        if !sink.wants(Facet::BitBuiltins) {
            return;
        }
        let unsigned = argument_ty("clz", width);
        let signed = argument_ty("ffs", width);
        let mut program =
            Program::new(format!("the zero argument at {width} bits, guarded where it has to be"));
        program.top(format!(
            "static volatile {} zero_bits = {};",
            unsigned.c_name(),
            unsigned.literal(0)
        ));
        program.top(format!(
            "static volatile {} one_bit = {};",
            unsigned.c_name(),
            unsigned.literal(1)
        ));
        program.top(format!(
            "static volatile {} zero_signed = {};",
            signed.c_name(),
            signed.literal(0)
        ));
        program.top(format!(
            "static volatile {} one_signed = {};",
            signed.c_name(),
            signed.literal(1)
        ));
        program.blank();
        program.check(Ty::I32, &format!("__builtin_popcount{suffix}(zero_bits)"), 0);
        program.check(Ty::I32, &format!("__builtin_parity{suffix}(zero_bits)"), 0);
        program.check(Ty::I32, &format!("__builtin_ffs{suffix}(zero_signed)"), 0);
        program.check(Ty::I32, &format!("__builtin_ffs{suffix}(one_signed)"), 1);
        program.blank();
        let width_value = i128::from(width);
        for name in ["clz", "ctz"] {
            program.check(
                Ty::I32,
                &format!("zero_bits ? __builtin_{name}{suffix}(zero_bits) : {width}"),
                width_value,
            );
            let answer = if name == "clz" { width_value - 1 } else { 0 };
            program.check(
                Ty::I32,
                &format!("one_bit ? __builtin_{name}{suffix}(one_bit) : {width}"),
                answer,
            );
        }
        let axes = Axes::of([
            ("builtin", "family"),
            ("width", &width.to_string()),
            ("form", "zero-argument"),
        ]);
        sink.push_tagged(Facet::BitBuiltins, axes, Dialect::C17, program, &["gnu"]);
    }
}

/// The type the builtin takes at a width.
///
/// `ffs` counts from one and takes a signed argument, which is the one asymmetry in the
/// family and the one a hand written lowering gets wrong first.
fn argument_ty(builtin: &str, width: u32) -> Ty {
    match (builtin, width) {
        ("ffs", 32) => Ty::I32,
        ("ffs", _) => Ty::I64,
        (_, 32) => Ty::U32,
        _ => Ty::U64,
    }
}

/// Every single bit, and a handful of patterns that are not one bit.
///
/// The single bits are the point. They walk the answer of `clz` and `ctz` through its whole
/// range one value at a time, which is the only way to catch a lowering that is right in the
/// middle and off by one at the ends. Zero is not in here, because two of these builtins are
/// undefined at zero and the zero case has a program of its own.
fn bit_patterns(width: u32) -> Vec<u64> {
    let mask = if width == 64 { u64::MAX } else { (1u64 << width) - 1 };
    let mut out: Vec<u64> = (0..width).map(|bit| 1u64 << bit).collect();
    for pattern in [mask, mask / 3, (mask / 3) << 1, 0x0f0f_0f0f_0f0f_0f0f, 3, 0x8000_0001] {
        let value = pattern & mask;
        if value != 0 && !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

/// What the builtin returns for a value, worked out here rather than asked of a compiler.
fn bit_answer(builtin: &str, width: u32, value: u64) -> i128 {
    let unused = 64 - width;
    match builtin {
        "clz" => i128::from(value.leading_zeros() - unused),
        "ctz" => i128::from(value.trailing_zeros()),
        "popcount" => i128::from(value.count_ones()),
        "parity" => i128::from(value.count_ones() % 2),
        _ => {
            if value == 0 {
                0
            } else {
                i128::from(value.trailing_zeros() + 1)
            }
        }
    }
}

/// The two floating types whose answers this corpus can state.
///
/// `long double` is not here. Its width, its precision and its exponent range are all target
/// specific, so an answer computed for the 80 bit format on x86 is the wrong answer on
/// aarch64, and a facet whose expected output depends on the machine is worse than no facet.
const FLOATS: &[(&str, bool)] = &[("float", true), ("double", false)];

/// Every conversion between an integer type and a floating one, in both directions.
///
/// Nothing here prints a floating value. Every check ends in an integer, which is what keeps
/// the expected output the same everywhere: two correct compilers can print a `double`
/// differently and neither is wrong, and a corpus that compared that text would be measuring
/// the library rather than the compiler.
pub(crate) fn float_conversion(sink: &mut Sink<'_>) {
    round_trips(sink);
    truncations(sink);
    narrowing(sink);
}

/// An integer out to a float and back, for the values that survive the trip exactly.
///
/// A value that does not survive is dropped rather than emitted with the rounded answer.
/// Rounding a `long long` into a `float` is well defined but it is a different question from
/// the one this shape asks, and the shape that asks it is `narrowing` below.
fn round_trips(sink: &mut Sink<'_>) {
    for ty in Ty::ALL {
        for &(float, single) in FLOATS {
            for &form in &["constant", "opaque"] {
                if !sink.wants(Facet::FloatConversion) {
                    return;
                }
                let values: Vec<i128> =
                    interesting(*ty).into_iter().filter(|v| survives(*v, single)).collect();
                if values.is_empty() {
                    continue;
                }
                let mut program = Program::new(format!(
                    "{} out to a {float} and back, with the value {}",
                    ty.c_name(),
                    if form == "constant" { "written as a constant" } else { "read from memory" }
                ));
                let table = ident(&[ty.name(), float, "in"]);
                if form == "opaque" {
                    let items: Vec<String> = values.iter().map(|v| ty.literal(*v)).collect();
                    program.top(format!(
                        "static volatile {} {table}[{}] = {{ {} }};",
                        ty.c_name(),
                        values.len(),
                        items.join(", ")
                    ));
                    program.blank();
                }
                for (at, &value) in values.iter().enumerate() {
                    let source =
                        if form == "constant" { lit(*ty, value) } else { format!("{table}[{at}]") };
                    let expr = format!("({})({float})({source})", ty.c_name());
                    program.check(*ty, &expr, value);
                }
                let axes = Axes::of([
                    ("shape", "round-trip"),
                    ("float", float),
                    ("type", ty.name()),
                    ("form", form),
                ]);
                sink.push(Facet::FloatConversion, axes, Dialect::C17, program);
            }
        }
    }
}

/// A float truncated into an integer type, which is where the unsigned cases live.
///
/// The values above `INT_MAX` going into an `unsigned int`, and the ones above `LLONG_MAX`
/// going into an `unsigned long long`, are the conversions real projects fail on, because a
/// back end that lowers the signed conversion and forgets the unsigned one passes everything
/// else. A value whose truncation does not fit the target type is undefined and is dropped
/// rather than emitted.
fn truncations(sink: &mut Sink<'_>) {
    for ty in Ty::ALL {
        for &(float, single) in FLOATS {
            for &form in &["constant", "opaque"] {
                if !sink.wants(Facet::FloatConversion) {
                    return;
                }
                let values: Vec<f64> = SOURCES
                    .iter()
                    .copied()
                    .filter(|v| representable(*v, single) && ty.holds(truncate(*v)))
                    .collect();
                if values.is_empty() {
                    continue;
                }
                let mut program = Program::new(format!(
                    "a {float} truncated into {}, with the value {}",
                    ty.c_name(),
                    if form == "constant" { "written as a constant" } else { "read from memory" }
                ));
                let table = ident(&[float, ty.name(), "in"]);
                if form == "opaque" {
                    let items: Vec<String> =
                        values.iter().map(|v| float_literal(*v, single)).collect();
                    program.top(format!(
                        "static volatile {float} {table}[{}] = {{ {} }};",
                        values.len(),
                        items.join(", ")
                    ));
                    program.blank();
                }
                for (at, &value) in values.iter().enumerate() {
                    let source = if form == "constant" {
                        float_literal(value, single)
                    } else {
                        format!("{table}[{at}]")
                    };
                    let expr = format!("({})({source})", ty.c_name());
                    program.check(*ty, &expr, truncate(value));
                }
                let axes = Axes::of([
                    ("shape", "truncate"),
                    ("float", float),
                    ("type", ty.name()),
                    ("form", form),
                ]);
                sink.push(Facet::FloatConversion, axes, Dialect::C17, program);
            }
        }
    }
}

/// A double narrowed to a float, where the value does not survive and the rounding is the
/// answer.
///
/// The result is printed as an integer, so what is checked is which `float` the `double`
/// landed on rather than how anybody prints it. None of these values is halfway between two
/// floats, so the answer does not depend on the rounding mode being the default one, and a
/// corpus that quietly assumed the default would be assuming something a program is allowed
/// to change.
fn narrowing(sink: &mut Sink<'_>) {
    for &form in &["constant", "opaque"] {
        if !sink.wants(Facet::FloatConversion) {
            return;
        }
        let mut program = Program::new(format!(
            "a double narrowed to a float, with the value {}",
            if form == "constant" { "written as a constant" } else { "read from memory" }
        ));
        if form == "opaque" {
            let items: Vec<String> = NARROWED.iter().map(|v| float_literal(*v, false)).collect();
            program.top(format!(
                "static volatile double wide_in[{}] = {{ {} }};",
                NARROWED.len(),
                items.join(", ")
            ));
            program.blank();
        }
        for (at, &value) in NARROWED.iter().enumerate() {
            let source = if form == "constant" {
                float_literal(value, false)
            } else {
                format!("wide_in[{at}]")
            };
            let landed = truncate(f64::from(value as f32));
            program.check(Ty::I64, &format!("(long long)(float)({source})"), landed);
            program.check(Ty::I64, &format!("(long long)(double)(float)({source})"), landed);
        }
        let axes = Axes::of([("shape", "narrowing"), ("form", form)]);
        sink.push(Facet::FloatConversion, axes, Dialect::C17, program);
    }
}

/// The floating values worth converting into an integer type.
///
/// The fractions are there because truncation is toward zero and not toward the nearest, and
/// a lowering that rounds instead of truncating is right on every whole number. The boundary
/// values are one below the top of each integer type, and the last two are the ones that only
/// fit somewhere unsigned.
const SOURCES: &[f64] = &[
    0.0,
    -0.0,
    0.5,
    -0.5,
    1.0,
    -1.0,
    1.5,
    -1.5,
    3.75,
    -3.75,
    127.5,
    -128.5,
    255.75,
    32767.5,
    -32768.5,
    65535.5,
    2_147_483_647.0,
    -2_147_483_648.0,
    4_294_967_295.0,
    9_007_199_254_740_992.0,
    9_223_372_036_854_774_784.0,
    1.0e19,
    18_446_744_073_709_549_568.0,
];

/// The doubles that do not fit in a float, chosen so none of them is a tie.
const NARROWED: &[f64] = &[
    16_777_216.0,
    33_554_437.0,
    33_554_443.0,
    1.0e9,
    1_234_567_890.0,
    -1_234_567_890.0,
    4_503_599_627_370_497.0,
];

/// Whether an integer comes back the same after a trip through a floating type.
fn survives(value: i128, single: bool) -> bool {
    if single { (value as f32) as i128 == value } else { (value as f64) as i128 == value }
}

/// Whether a floating value is exactly what the type holds, rather than the nearest thing.
fn representable(value: f64, single: bool) -> bool {
    !single || f64::from(value as f32) == value
}

/// A floating value truncated toward zero, which is what a cast to an integer type does.
fn truncate(value: f64) -> i128 {
    value.trunc() as i128
}

/// A C floating constant that parses back to exactly this value.
///
/// Exponent notation always. Rust prints a large double as a run of digits with no point, and
/// a run of digits with no point is an integer constant in C, which for these values is one
/// that does not fit in any integer type. The shortest form Rust writes is the shortest one
/// that round trips, so a compiler that reads it correctly gets the value back.
fn float_literal(value: f64, single: bool) -> String {
    if single { format!("{:e}f", value as f32) } else { format!("{value:e}") }
}

/// Dispatch through the address of a label, at every arm count worth having.
///
/// This is what an interpreter loop is, which is why it matters: `lua`, `bash` and every
/// bytecode engine in the corpus of real projects is one of these, and a compiler with no
/// lowering for the address of a block cannot build any of them. The opcode stream is
/// `volatile` so the compiler cannot fold the interpreter into its answer, which means the
/// indirect jump has to be emitted and the arms have to survive as separate blocks.
pub(crate) fn computed_goto(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "dispatch-block",
        "static-table",
        "address-in-variable",
        "direct-threading",
        "dispatch-in-loop",
    ];
    for &shape in SHAPES {
        for &arms in &[2usize, 4, 8, 16, 32] {
            if !sink.wants(Facet::ComputedGoto) {
                return;
            }
            let stream: Vec<usize> = (0..arms * 3).map(|at| (at * 7 + 3) % arms).collect();
            let mut accumulator = 1u64;
            for &arm in &stream {
                accumulator = accumulator.wrapping_mul(31).wrapping_add(arm as u64);
            }
            let length = stream.len();
            let mut program = Program::new(format!(
                "a {arms} arm dispatch loop, {}",
                match shape {
                    "dispatch-block" => "with one dispatch point the arms jump back to",
                    "static-table" => "with the table of label addresses at static storage",
                    "address-in-variable" =>
                        "with the address in a variable before the jump uses it",
                    "direct-threading" => "with every arm dispatching the next one itself",
                    _ => "with the dispatch inside a for loop the arms continue",
                }
            ));
            let opcodes: Vec<String> = stream.iter().map(usize::to_string).collect();
            program.top(format!(
                "static volatile unsigned char stream[{length}] = {{ {} }};",
                opcodes.join(", ")
            ));
            program.blank();

            let labels: Vec<String> = (0..arms).map(|arm| format!("&&arm{arm}")).collect();
            let storage = if shape == "static-table" { "static " } else { "" };
            program.line(format!("{storage}void *table[{arms}] = {{ {} }};", labels.join(", ")));
            program.line("unsigned long long acc = 1ull;");
            program.line("unsigned int at = 0u;");
            program.blank();

            match shape {
                "dispatch-in-loop" => {
                    program.line(format!("for (at = 0u; at < {length}u; at = at + 1u) {{"));
                    program.line_at(1, "goto *table[stream[at]];");
                    for arm in 0..arms {
                        program.line(format!("arm{arm}:"));
                        program.line_at(1, format!("acc = acc * 31ull + {arm}ull;"));
                        program.line_at(1, "continue;");
                    }
                    program.line("}");
                }
                "direct-threading" => {
                    program.line("goto *table[stream[at]];");
                    for arm in 0..arms {
                        program.line(format!("arm{arm}:"));
                        program.line_at(1, format!("acc = acc * 31ull + {arm}ull;"));
                        program.line_at(1, "at = at + 1u;");
                        program.line_at(1, format!("if (at >= {length}u) goto done;"));
                        program.line_at(1, "goto *table[stream[at]];");
                    }
                    program.line("done:");
                    program.line_at(1, ";");
                }
                "address-in-variable" => {
                    program.line("void *target = table[stream[at]];");
                    program.line("goto *target;");
                    for arm in 0..arms {
                        program.line(format!("arm{arm}:"));
                        program.line_at(1, format!("acc = acc * 31ull + {arm}ull;"));
                        program.line_at(1, "goto next;");
                    }
                    program.line("next:");
                    program.line_at(1, "at = at + 1u;");
                    program.line_at(1, format!("if (at < {length}u) {{"));
                    program.line_at(2, "target = table[stream[at]];");
                    program.line_at(2, "goto *target;");
                    program.line_at(1, "}");
                }
                _ => {
                    program.line("goto *table[stream[at]];");
                    for arm in 0..arms {
                        program.line(format!("arm{arm}:"));
                        program.line_at(1, format!("acc = acc * 31ull + {arm}ull;"));
                        program.line_at(1, "goto next;");
                    }
                    program.line("next:");
                    program.line_at(1, "at = at + 1u;");
                    program.line_at(1, format!("if (at < {length}u) goto *table[stream[at]];"));
                }
            }

            program.blank();
            program.check(Ty::U64, "acc", i128::from(accumulator));
            program.check(Ty::U32, "at", length as i128);
            let axes = Axes::of([("shape", shape), ("arms", &arms.to_string())]);
            sink.push_tagged(Facet::ComputedGoto, axes, Dialect::C17, program, &["gnu"]);
        }
    }
}

/// The load and store orderings, which are not the same set.
///
/// A load with release ordering and a store with acquire ordering are both nonsense, and a
/// compiler is entitled to reject them, so a corpus that emitted them would be testing its
/// own carelessness. Read modify write takes all six.
const LOAD_ORDERS: &[&str] =
    &["__ATOMIC_RELAXED", "__ATOMIC_CONSUME", "__ATOMIC_ACQUIRE", "__ATOMIC_SEQ_CST"];
const STORE_ORDERS: &[&str] = &["__ATOMIC_RELAXED", "__ATOMIC_RELEASE", "__ATOMIC_SEQ_CST"];
const RMW_ORDERS: &[&str] = &[
    "__ATOMIC_RELAXED",
    "__ATOMIC_CONSUME",
    "__ATOMIC_ACQUIRE",
    "__ATOMIC_RELEASE",
    "__ATOMIC_ACQ_REL",
    "__ATOMIC_SEQ_CST",
];

/// The `__atomic_*` family at every ordering, and `<stdatomic.h>` alongside it.
///
/// Every program here is single threaded, which is the only way the answer can be computed
/// rather than observed. That is not a weakness of the facet. A missing lowering, a wrong
/// argument order and a fetch builtin that returns the new value where it should return the
/// old one all show up on one thread, and those are what block a real project.
pub(crate) fn atomics(sink: &mut Sink<'_>) {
    load_store(sink);
    exchanges(sink);
    fetch_ops(sink);
    flag_and_fences(sink);
    stdatomic(sink);
}

/// Loading and storing at every ordering each of them takes, in both spellings.
fn load_store(sink: &mut Sink<'_>) {
    for ty in Ty::ALL {
        if !sink.wants(Facet::Atomics) {
            return;
        }
        let mut program =
            Program::new(format!("atomic load and store of {} at every ordering", ty.c_name()));
        program.top(format!("static {} cell = {};", ty.c_name(), ty.literal(0)));
        program.blank();
        program.line(format!("{} held;", ty.c_name()));
        program.line(format!("{} want;", ty.c_name()));
        program.blank();
        let mut value = 3i128;
        let mut last = value;
        for &store in STORE_ORDERS {
            program.line(format!("__atomic_store_n(&cell, {}, {store});", lit(*ty, value)));
            for &load in LOAD_ORDERS {
                program.check(*ty, &format!("__atomic_load_n(&cell, {load})"), value);
            }
            last = value;
            value = ty.convert(value * 3 + 1);
        }
        program.blank();
        // The forms that take a pointer to the result rather than returning it. They are the
        // ones a compiler has to lower for a type it cannot hold in a register, and a back
        // end that only implements the `_n` spelling builds nothing that uses the other.
        program.line("__atomic_load(&cell, &held, __ATOMIC_SEQ_CST);");
        program.check(*ty, "held", last);
        program.line(format!("want = {};", lit(*ty, 9)));
        program.line("__atomic_store(&cell, &want, __ATOMIC_RELEASE);");
        program.check(*ty, "__atomic_load_n(&cell, __ATOMIC_ACQUIRE)", 9);
        let axes = Axes::of([("shape", "load-store"), ("type", ty.name())]);
        sink.push_tagged(Facet::Atomics, axes, Dialect::C17, program, &["gnu"]);
    }
}

/// Exchange and compare exchange, which is where the arguments are easiest to get backwards.
fn exchanges(sink: &mut Sink<'_>) {
    for ty in Ty::ALL {
        if !sink.wants(Facet::Atomics) {
            return;
        }
        let mut program = Program::new(format!(
            "atomic exchange and compare exchange of {} at every ordering",
            ty.c_name()
        ));
        program.top(format!("static {} cell = {};", ty.c_name(), ty.literal(1)));
        program.blank();
        program.line(format!("{} expected;", ty.c_name()));
        program.line(format!("{} want;", ty.c_name()));
        program.line(format!("{} got;", ty.c_name()));
        program.blank();

        let mut current = 1i128;
        for (step, &order) in RMW_ORDERS.iter().enumerate() {
            let next = ty.convert(10 + step as i128);
            program.check(
                *ty,
                &format!("__atomic_exchange_n(&cell, {}, {order})", lit(*ty, next)),
                current,
            );
            current = next;
        }
        program.check(*ty, "__atomic_load_n(&cell, __ATOMIC_RELAXED)", current);
        program.blank();

        program.line(format!("want = {};", lit(*ty, 7)));
        program.line("__atomic_exchange(&cell, &want, &got, __ATOMIC_ACQ_REL);");
        program.check(*ty, "got", current);
        current = 7;
        program.check(*ty, "__atomic_load_n(&cell, __ATOMIC_SEQ_CST)", current);
        program.blank();

        // Strong only, and never weak. The weak form is allowed to fail for no reason at all,
        // so a program that printed whether it succeeded would be printing something this
        // repository cannot compute. The retry loop the weak form needs is in the
        // `<stdatomic.h>` program instead, where the loop makes the outcome certain again.
        for &(success, failure) in &[
            ("__ATOMIC_SEQ_CST", "__ATOMIC_SEQ_CST"),
            ("__ATOMIC_ACQ_REL", "__ATOMIC_ACQUIRE"),
            ("__ATOMIC_RELEASE", "__ATOMIC_RELAXED"),
        ] {
            let next = ty.convert(current + 5);
            program.line(format!("expected = {};", lit(*ty, current)));
            program.check(
                Ty::I32,
                &format!(
                    "__atomic_compare_exchange_n(&cell, &expected, {}, 0, {success}, {failure})",
                    lit(*ty, next)
                ),
                1,
            );
            program.check(*ty, "expected", current);
            program.check(*ty, "__atomic_load_n(&cell, __ATOMIC_RELAXED)", next);
            current = next;
        }
        program.blank();

        // The comparison that fails. What matters is that the expected value is written back
        // with what was really there, because that is how every lock free loop in every real
        // project makes progress.
        let wrong = ty.convert(current + 1);
        program.line(format!("expected = {};", lit(*ty, wrong)));
        program.check(
            Ty::I32,
            &format!(
                "__atomic_compare_exchange_n(&cell, &expected, {}, 0, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST)",
                lit(*ty, 0)
            ),
            0,
        );
        program.check(*ty, "expected", current);
        program.check(*ty, "__atomic_load_n(&cell, __ATOMIC_RELAXED)", current);

        let axes = Axes::of([("shape", "exchange"), ("type", ty.name())]);
        sink.push_tagged(Facet::Atomics, axes, Dialect::C17, program, &["gnu"]);
    }
}

/// The read modify write family, in both the fetch first and the fetch last spelling.
fn fetch_ops(sink: &mut Sink<'_>) {
    const OPS: &[&str] = &["add", "sub", "and", "or", "xor", "nand"];
    for ty in Ty::ALL {
        if !sink.wants(Facet::Atomics) {
            return;
        }
        let mut program = Program::new(format!(
            "the atomic read modify write family on {}, both ways round",
            ty.c_name()
        ));
        program.top(format!("static {} cell = {};", ty.c_name(), ty.literal(0)));
        program.blank();

        let base = 50i128;
        let operand = 25i128;
        for &op in OPS {
            let after = apply(*ty, op, base, operand);
            // The old value out of `fetch_op` and the new value out of `op_fetch`. A back end
            // that wires one of them to the other is wrong in a way that no single check on
            // its own would show.
            program.line(format!("__atomic_store_n(&cell, {}, __ATOMIC_SEQ_CST);", lit(*ty, base)));
            program.check(
                *ty,
                &format!("__atomic_fetch_{op}(&cell, {}, __ATOMIC_SEQ_CST)", lit(*ty, operand)),
                base,
            );
            program.check(*ty, "__atomic_load_n(&cell, __ATOMIC_RELAXED)", after);
            program.line(format!("__atomic_store_n(&cell, {}, __ATOMIC_SEQ_CST);", lit(*ty, base)));
            program.check(
                *ty,
                &format!("__atomic_{op}_fetch(&cell, {}, __ATOMIC_SEQ_CST)", lit(*ty, operand)),
                after,
            );
            program.blank();
        }

        // Add at every ordering, since the ordering is the axis the facet is named for and
        // one operation carrying all six is enough to prove each of them lowers.
        program.line(format!("__atomic_store_n(&cell, {}, __ATOMIC_SEQ_CST);", lit(*ty, 0)));
        let mut running = 0i128;
        for &order in RMW_ORDERS {
            running = ty.convert(running + 1);
            program.check(
                *ty,
                &format!("__atomic_add_fetch(&cell, {}, {order})", lit(*ty, 1)),
                running,
            );
        }
        program.blank();

        // Every bit flipped, which is the one operand that reaches the top of a wide type.
        let ones = all_ones(*ty);
        program.line(format!("__atomic_store_n(&cell, {}, __ATOMIC_SEQ_CST);", lit(*ty, base)));
        program.check(
            *ty,
            &format!("__atomic_xor_fetch(&cell, {}, __ATOMIC_SEQ_CST)", lit(*ty, ones)),
            apply(*ty, "xor", base, ones),
        );

        let axes = Axes::of([("shape", "fetch-op"), ("type", ty.name())]);
        sink.push_tagged(Facet::Atomics, axes, Dialect::C17, program, &["gnu"]);
    }
}

/// What a read modify write leaves behind, worked out here.
fn apply(ty: Ty, op: &str, left: i128, right: i128) -> i128 {
    let raw = match op {
        "add" => left + right,
        "sub" => left - right,
        "and" => left & right,
        "or" => left | right,
        "xor" => left ^ right,
        _ => !(left & right),
    };
    ty.convert(raw)
}

/// The test and set flag, and the fences, which have no value to check on their own.
fn flag_and_fences(sink: &mut Sink<'_>) {
    if !sink.wants(Facet::Atomics) {
        return;
    }
    let mut program = Program::new("the atomic test and set flag, which is a lock in one byte");
    program.top("static char gate;");
    program.blank();
    // The byte itself is never printed. `__atomic_test_and_set` writes some nonzero value
    // that the implementation chooses, so the byte is not the same everywhere and what it
    // returned is.
    program.check(Ty::I32, "__atomic_test_and_set(&gate, __ATOMIC_SEQ_CST)", 0);
    program.check(Ty::I32, "__atomic_test_and_set(&gate, __ATOMIC_ACQUIRE)", 1);
    program.line("__atomic_clear(&gate, __ATOMIC_RELEASE);");
    program.check(Ty::I32, "__atomic_test_and_set(&gate, __ATOMIC_RELAXED)", 0);
    program.line("__atomic_clear(&gate, __ATOMIC_SEQ_CST);");
    program.check(Ty::I32, "__atomic_test_and_set(&gate, __ATOMIC_ACQ_REL)", 0);
    sink.push_tagged(
        Facet::Atomics,
        Axes::of([("shape", "flag")]),
        Dialect::C17,
        program,
        &["gnu"],
    );

    if !sink.wants(Facet::Atomics) {
        return;
    }
    let mut program =
        Program::new("the fences, which the ordinary work around them has to survive");
    program.top("static int step;");
    program.top("static int seen;");
    program.blank();
    program.line("step = 1;");
    program.line("__atomic_thread_fence(__ATOMIC_RELEASE);");
    program.line("seen = step + 1;");
    program.line("__atomic_thread_fence(__ATOMIC_ACQUIRE);");
    program.line("step = seen + 1;");
    program.line("__atomic_signal_fence(__ATOMIC_ACQ_REL);");
    program.blank();
    for &order in RMW_ORDERS {
        program.line(format!("__atomic_thread_fence({order});"));
        program.line(format!("__atomic_signal_fence({order});"));
    }
    program.blank();
    program.check(Ty::I32, "step", 3);
    program.check(Ty::I32, "seen", 2);
    sink.push_tagged(
        Facet::Atomics,
        Axes::of([("shape", "fence")]),
        Dialect::C17,
        program,
        &["gnu"],
    );
}

/// The same operations through the header, which is what portable code actually writes.
///
/// These carry the `headers` tag rather than the `gnu` one. They are standard C and not an
/// extension, and they are the only programs in this facet that need a header to be found at
/// all, so a run against a compiler whose header search is not finished can leave them out
/// and still be a real run of the rest.
fn stdatomic(sink: &mut Sink<'_>) {
    const TYPES: &[(Ty, &str)] = &[
        (Ty::I32, "atomic_int"),
        (Ty::U32, "atomic_uint"),
        (Ty::I64, "atomic_llong"),
        (Ty::U64, "atomic_ullong"),
    ];
    for &(ty, atomic) in TYPES {
        if !sink.wants(Facet::Atomics) {
            return;
        }
        let mut program =
            Program::new(format!("{atomic} through the header rather than the builtins"));
        program.include("stdatomic.h");
        program.top(format!("static {atomic} counter;"));
        program.blank();
        program.line(format!("{} expected;", ty.c_name()));
        program.blank();
        program.line(format!("atomic_init(&counter, {});", lit(ty, 5)));
        program.check(ty, "atomic_load_explicit(&counter, memory_order_relaxed)", 5);
        program.line(format!(
            "atomic_store_explicit(&counter, {}, memory_order_release);",
            lit(ty, 9)
        ));
        program.check(ty, "atomic_load(&counter)", 9);
        program.check(
            ty,
            &format!("atomic_fetch_add_explicit(&counter, {}, memory_order_acq_rel)", lit(ty, 3)),
            9,
        );
        program.check(ty, "atomic_load(&counter)", 12);
        program.check(ty, &format!("atomic_fetch_sub(&counter, {})", lit(ty, 4)), 12);
        program.check(ty, "atomic_load(&counter)", 8);
        program.blank();
        program.line(format!("expected = {};", lit(ty, 8)));
        program.check(Ty::I32, "atomic_compare_exchange_strong(&counter, &expected, 20)", 1);
        program.check(ty, "expected", 8);
        program.check(ty, "atomic_load(&counter)", 20);
        program.line(format!("expected = {};", lit(ty, 3)));
        program.check(Ty::I32, "atomic_compare_exchange_strong(&counter, &expected, 99)", 0);
        program.check(ty, "expected", 20);
        program.check(ty, "atomic_load(&counter)", 20);
        program.blank();
        // The weak form in the loop it is documented to need. It may fail for no reason, and
        // when it does it writes back the value that was really there, which is the same one,
        // so the loop goes round again and ends the same way every time.
        program.line("while (!atomic_compare_exchange_weak(&counter, &expected, 41)) {");
        program.line_at(1, ";");
        program.line("}");
        program.check(ty, "atomic_load(&counter)", 41);
        program.check(ty, &format!("atomic_exchange(&counter, {})", lit(ty, 7)), 41);
        program.check(ty, "atomic_load(&counter)", 7);
        program.line("atomic_thread_fence(memory_order_seq_cst);");
        program.line("atomic_signal_fence(memory_order_acq_rel);");
        program.check(ty, "atomic_load(&counter)", 7);
        let axes = Axes::of([("shape", "stdatomic"), ("type", ty.name())]);
        sink.push_tagged(Facet::Atomics, axes, Dialect::C17, program, &["headers"]);
    }

    if !sink.wants(Facet::Atomics) {
        return;
    }
    let mut program =
        Program::new("atomic_flag through the header, initialised the way the header says");
    program.include("stdatomic.h");
    program.top("static atomic_flag gate = ATOMIC_FLAG_INIT;");
    program.blank();
    program.check(Ty::I32, "atomic_flag_test_and_set(&gate)", 0);
    program.check(Ty::I32, "atomic_flag_test_and_set(&gate)", 1);
    program.line("atomic_flag_clear(&gate);");
    program.check(Ty::I32, "atomic_flag_test_and_set_explicit(&gate, memory_order_acquire)", 0);
    program.line("atomic_flag_clear_explicit(&gate, memory_order_release);");
    program.check(Ty::I32, "atomic_flag_test_and_set(&gate)", 0);
    sink.push_tagged(
        Facet::Atomics,
        Axes::of([("shape", "stdatomic-flag")]),
        Dialect::C17,
        program,
        &["headers"],
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;

    fn cases(facet: Facet) -> Vec<corpus_model::Case> {
        crate::generate(&Options::all().only(&[facet])).unwrap().cases
    }

    #[test]
    fn a_float_literal_parses_back_to_the_value_it_was_written_from() {
        for &value in SOURCES.iter().chain(NARROWED) {
            let text = float_literal(value, false);
            assert!(text.contains('e'), "{text} would be an integer constant in C");
            assert_eq!(text.parse::<f64>().unwrap(), value, "{text}");
        }
    }

    #[test]
    fn a_single_precision_literal_is_only_written_for_a_value_a_float_holds() {
        for &value in SOURCES {
            if representable(value, true) {
                let text = float_literal(value, true);
                assert_eq!(f64::from(text.trim_end_matches('f').parse::<f32>().unwrap()), value);
            }
        }
    }

    #[test]
    fn the_bit_patterns_walk_every_position_and_never_include_zero() {
        for &(width, _) in WIDTHS {
            let values = bit_patterns(width);
            assert!(!values.contains(&0), "zero is undefined for two of the family");
            for bit in 0..width {
                assert!(values.contains(&(1u64 << bit)), "{width} bits is missing bit {bit}");
            }
        }
    }

    #[test]
    fn the_answers_for_a_single_bit_are_the_position_and_its_complement() {
        assert_eq!(bit_answer("clz", 32, 1), 31);
        assert_eq!(bit_answer("clz", 64, 1), 63);
        assert_eq!(bit_answer("ctz", 32, 1 << 17), 17);
        assert_eq!(bit_answer("ffs", 32, 1 << 17), 18);
        assert_eq!(bit_answer("ffs", 32, 0), 0);
        assert_eq!(bit_answer("popcount", 64, u64::MAX), 64);
        assert_eq!(bit_answer("parity", 64, u64::MAX), 0);
        assert_eq!(bit_answer("parity", 32, 7), 1);
    }

    #[test]
    fn nothing_in_the_bit_facet_names_the_long_builtins_whose_width_moves() {
        for case in cases(Facet::BitBuiltins) {
            for banned in ["__builtin_clzl(", "__builtin_ctzl(", "__builtin_popcountl("] {
                assert!(!case.source.contains(banned), "{} uses {banned}", case.id);
            }
        }
    }

    #[test]
    fn no_case_asks_for_the_count_of_a_zero_without_guarding_it() {
        for case in cases(Facet::BitBuiltins) {
            for banned in ["__builtin_clz(0", "__builtin_ctz(0", "__builtin_clzll(0"] {
                assert!(!case.source.contains(banned), "{} is undefined", case.id);
            }
        }
    }

    #[test]
    fn every_dispatch_program_takes_the_address_of_a_label_and_jumps_through_it() {
        let cases = cases(Facet::ComputedGoto);
        assert_eq!(cases.len(), 25);
        for case in cases {
            assert!(case.source.contains("&&arm0"), "{} takes no label address", case.id);
            assert!(case.source.contains("goto *"), "{} never jumps indirectly", case.id);
            assert!(case.tags.iter().any(|t| t == "gnu"), "{} is not tagged gnu", case.id);
        }
    }

    #[test]
    fn no_atomic_case_uses_the_weak_compare_exchange_outside_a_loop() {
        for case in cases(Facet::Atomics) {
            if case.source.contains("compare_exchange_weak") {
                assert!(
                    case.source.contains("while (!atomic_compare_exchange_weak"),
                    "{}",
                    case.id
                );
            }
            assert!(!case.source.contains(", 1, __ATOMIC"), "{} passes weak", case.id);
        }
    }

    #[test]
    fn a_load_is_never_asked_for_an_ordering_a_load_does_not_take() {
        for case in cases(Facet::Atomics) {
            for banned in [
                "__atomic_load_n(&cell, __ATOMIC_RELEASE",
                "__atomic_load_n(&cell, __ATOMIC_ACQ_REL",
            ] {
                assert!(!case.source.contains(banned), "{} uses {banned}", case.id);
            }
            assert!(
                !case.source.contains("__atomic_store_n(&cell, 0, __ATOMIC_ACQUIRE"),
                "{}",
                case.id
            );
        }
    }

    #[test]
    fn a_read_modify_write_is_worked_out_here_and_wraps_the_way_the_type_does() {
        assert_eq!(apply(Ty::I32, "add", 50, 25), 75);
        assert_eq!(apply(Ty::I32, "nand", 50, 25), -17);
        assert_eq!(apply(Ty::U8, "nand", 50, 25), 239);
        assert_eq!(apply(Ty::U8, "xor", 50, 255), 205);
        assert_eq!(apply(Ty::I8, "xor", 50, -1), -51);
    }

    #[test]
    fn a_conversion_is_only_emitted_when_the_value_comes_back_the_same() {
        assert!(survives(16_777_216, true));
        assert!(!survives(16_777_217, true));
        assert!(survives(16_777_217, false));
        assert!(!survives(i128::from(u64::MAX), false));
    }

    #[test]
    fn nothing_in_the_float_facet_converts_a_value_the_target_type_cannot_hold() {
        // The filter the truncation shape runs on, checked on the values that decide it. A
        // negative fraction truncates to zero and an unsigned type does hold that, which is
        // why the filter is on the truncated value and not on the value itself.
        assert!(!Ty::U8.holds(truncate(-3.75)));
        assert!(Ty::U8.holds(truncate(-0.5)));
        assert!(!Ty::I32.holds(truncate(4_294_967_295.0)));
        assert!(Ty::U32.holds(truncate(4_294_967_295.0)));

        for case in cases(Facet::FloatConversion) {
            let Some(name) = case.axes.get("type") else {
                continue;
            };
            let ty = Ty::ALL.iter().find(|t| t.name() == name).expect("a type axis names a type");
            let corpus_model::Expect::Output(expected) = &case.expect else {
                panic!("{} expects something other than output", case.id);
            };
            for line in expected.lines() {
                let value: i128 = line.parse().expect("every expected line is an integer");
                assert!(ty.holds(value), "{} prints {value}, which {name} cannot hold", case.id);
            }
        }
    }
}
