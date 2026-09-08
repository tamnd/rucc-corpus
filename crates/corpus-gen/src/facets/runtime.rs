//! The three things a language runtime asks for that no other facet covers.
//!
//! They arrived together because the same projects want all three. An interpreter unwinds its
//! errors with `setjmp` and `longjmp`, builds a scratch buffer whose size it only learns at
//! run time, and has a numeric tower that reaches past `double`. None of those is an
//! optimization, and that is the point: each one is a constraint on what the optimizer is
//! allowed to do, and a compiler that gets the transformations right and these wrong will
//! still fail to build anything real.
//!
//! `vla-and-alloca` is a floor facet. A variable length array is the one object whose size
//! the compiler cannot see, in the same way that a computed goto is the one edge whose target
//! it cannot see, and everything above has to survive not knowing.
//!
//! `long-double` is a back end facet. Its width, its padding and the register it travels in
//! are all decided below the machine independent IR.
//!
//! `setjmp-longjmp` is a correctness facet. It sits next to `barrier` and `atomics` because
//! what it mostly asks the compiler for is restraint.
//!
//! # How these cases stay target independent
//!
//! The corpus rule is that no answer may depend on the machine. That is easy for the first
//! two and the whole difficulty of the third. `long double` is eighty bits on x86, a hundred
//! and twenty eight on aarch64, and the same as `double` on more targets than people expect,
//! so nothing here may depend on how wide it is. Every case is written against what C
//! actually promises: that every `double` value is also a `long double` value, that the
//! limits in `float.h` are at least as generous, and that the arithmetic on small integers is
//! exact in any of the three. The values are chosen to be exact in `double` as well, so the
//! answers hold even where the two types are the same type.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// Emits everything in this module.
pub(crate) fn generate(sink: &mut Sink<'_>) {
    vla_and_alloca(sink);
    long_double(sink);
    setjmp_longjmp(sink);
}

/// Objects whose size the compiler learns at run time.
///
/// Every bound here comes out of a `volatile` global, so the size really is unknown to the
/// optimizer rather than a constant it can fold back into an ordinary array. That is the
/// difference between a case about variable length arrays and a case about arrays.
///
/// Nothing here prints a size. The corpus never prints `sizeof`, because the answer would
/// differ between two correct compilers on two machines, so a case proves the array was laid
/// out correctly by reading back what it wrote to both ends of it instead.
fn vla_and_alloca(sink: &mut Sink<'_>) {
    /// The shapes, and whether the shape needs a GCC extension.
    const SHAPES: &[(&str, bool)] = &[
        ("one-dimension", false),
        ("two-dimensions", false),
        ("as-a-parameter", false),
        ("pointer-to-a-row", false),
        ("size-changes-each-time", false),
        ("jumping-out-of-the-scope", false),
        ("alloca", true),
        ("alloca-in-a-loop", true),
        ("both-in-one-frame", true),
    ];
    for &(shape, gnu) in SHAPES {
        if !sink.wants(Facet::VlaAndAlloca) {
            return;
        }
        let mut program = Program::new(format!("a run time sized object, {shape}"));
        match shape {
            "one-dimension" => {
                program.input(Ty::I32, "count", 6);
                program.blank();
                program.line("int values[count];");
                program.line("for (int i = 0; i < count; i++) {");
                program.line_at(1, "values[i] = i * i;");
                program.line("}");
                program.line("int total = 0;");
                program.line("for (int i = 0; i < count; i++) {");
                program.line_at(1, "total = total + values[i];");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total", 55);
                program.check(Ty::I32, "values[0] + values[count - 1]", 25);
            }
            "two-dimensions" => {
                program.input(Ty::I32, "rows", 3);
                program.input(Ty::I32, "cols", 4);
                program.blank();
                program.line("int grid[rows][cols];");
                program.line("for (int i = 0; i < rows; i++) {");
                program.line_at(1, "for (int j = 0; j < cols; j++) {");
                program.line_at(2, "grid[i][j] = i * 10 + j;");
                program.line_at(1, "}");
                program.line("}");
                program.line("int total = 0;");
                program.line("for (int i = 0; i < rows; i++) {");
                program.line_at(1, "for (int j = 0; j < cols; j++) {");
                program.line_at(2, "total = total + grid[i][j];");
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                // The stride between rows is a multiply the compiler cannot fold, which is the
                // one thing a two dimensional variable length array asks for that a one
                // dimensional one does not.
                program.check(Ty::I32, "total", 138);
                program.check(Ty::I32, "grid[2][3]", 23);
                program.check(Ty::I32, "grid[0][0]", 0);
            }
            "as-a-parameter" => {
                program.top("static int corner(int rows, int cols, int grid[rows][cols]) {");
                program.top("    return grid[rows - 1][cols - 1];");
                program.top("}");
                program.top("static int total(int rows, int cols, int grid[rows][cols]) {");
                program.top("    int sum = 0;");
                program.top("    for (int i = 0; i < rows; i++) {");
                program.top("        for (int j = 0; j < cols; j++) {");
                program.top("            sum = sum + grid[i][j];");
                program.top("        }");
                program.top("    }");
                program.top("    return sum;");
                program.top("}");
                program.input(Ty::I32, "rows", 3);
                program.input(Ty::I32, "cols", 4);
                program.blank();
                program.line("int grid[rows][cols];");
                program.line("for (int i = 0; i < rows; i++) {");
                program.line_at(1, "for (int j = 0; j < cols; j++) {");
                program.line_at(2, "grid[i][j] = i * 10 + j;");
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total(rows, cols, grid)", 138);
                program.check(Ty::I32, "corner(rows, cols, grid)", 23);
            }
            "pointer-to-a-row" => {
                program.input(Ty::I32, "rows", 3);
                program.input(Ty::I32, "cols", 4);
                program.blank();
                program.line("int grid[rows][cols];");
                program.line("for (int i = 0; i < rows; i++) {");
                program.line_at(1, "for (int j = 0; j < cols; j++) {");
                program.line_at(2, "grid[i][j] = i * 10 + j;");
                program.line_at(1, "}");
                program.line("}");
                program.line("int (*row)[cols] = grid;");
                program.line("int walked = 0;");
                program.line("for (int i = 0; i < rows; i++) {");
                program.line_at(1, "walked = walked + (*row)[i];");
                program.line_at(1, "row = row + 1;");
                program.line("}");
                program.blank();
                // Incrementing the pointer moves it by a whole row, and the width of a row is
                // not a constant, so this is the same multiply as above arrived at from the
                // other direction. The walk reads down the diagonal, so it is nought, eleven
                // and twenty two.
                program.check(Ty::I32, "walked", 33);
            }
            "size-changes-each-time" => {
                program.input(Ty::I32, "base", 2);
                program.blank();
                program.line("int total = 0;");
                program.line("for (int round = 0; round < 5; round++) {");
                program.line_at(1, "int room = base + round;");
                program.line_at(1, "int values[room];");
                program.line_at(1, "for (int j = 0; j < room; j++) {");
                program.line_at(2, "values[j] = j + 1;");
                program.line_at(1, "}");
                program.line_at(1, "total = total + values[room - 1];");
                program.line("}");
                program.blank();
                // Five different frame sizes in one function. The stack pointer has to come
                // back to where it started on every iteration or the sixth one runs out.
                program.check(Ty::I32, "total", 20);
            }
            "jumping-out-of-the-scope" => {
                program.input(Ty::I32, "count", 4);
                program.blank();
                program.line("int total = 0;");
                program.line("for (int round = 0; round < 3; round++) {");
                program.line_at(1, "int values[count + round];");
                program.line_at(1, "for (int j = 0; j < count + round; j++) {");
                program.line_at(2, "values[j] = 1;");
                program.line_at(1, "}");
                program.line_at(1, "total = total + values[0] + values[count + round - 1];");
                program.line_at(1, "if (round == 1) {");
                program.line_at(2, "goto done;");
                program.line_at(1, "}");
                program.line("}");
                program.line("done:");
                program.blank();
                // Leaving the block by a jump rather than by falling off the end. The array
                // still has to be given back, and the label is outside its scope.
                program.check(Ty::I32, "total", 4);
            }
            "alloca" => {
                program.input(Ty::I32, "count", 8);
                program.blank();
                // Bytes rather than ints, because the corpus never writes `sizeof` and one
                // unsigned char is one byte by definition, so the count is exact everywhere.
                program.line("unsigned char *room = (unsigned char *)__builtin_alloca(count);");
                program.line("for (int i = 0; i < count; i++) {");
                program.line_at(1, "room[i] = (unsigned char)(i * 3);");
                program.line("}");
                program.line("int total = 0;");
                program.line("for (int i = 0; i < count; i++) {");
                program.line_at(1, "total = total + room[i];");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total", 84);
                program.check(Ty::I32, "room[count - 1]", 21);
            }
            "alloca-in-a-loop" => {
                program.input(Ty::I32, "rounds", 4);
                program.blank();
                // Every one of these lives until main returns, which is exactly the difference
                // between alloca and a variable length array and the reason the loop is short.
                program.line("int total = 0;");
                program.line("for (int i = 0; i < rounds; i++) {");
                program
                    .line_at(1, "unsigned char *room = (unsigned char *)__builtin_alloca(i + 1);");
                program.line_at(1, "room[i] = (unsigned char)(i + 1);");
                program.line_at(1, "total = total + room[i];");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "total", 10);
            }
            _ => {
                program.input(Ty::I32, "count", 5);
                program.blank();
                program.line("int values[count];");
                program.line("unsigned char *room = (unsigned char *)__builtin_alloca(count);");
                program.line("for (int i = 0; i < count; i++) {");
                program.line_at(1, "values[i] = i + 1;");
                program.line_at(1, "room[i] = (unsigned char)(i * 2);");
                program.line("}");
                program.line("int from_array = 0;");
                program.line("int from_alloca = 0;");
                program.line("for (int i = 0; i < count; i++) {");
                program.line_at(1, "from_array = from_array + values[i];");
                program.line_at(1, "from_alloca = from_alloca + room[i];");
                program.line("}");
                program.blank();
                // Two run time sized objects in the same frame, which is where a back end that
                // keeps only one stack pointer adjustment gets it wrong.
                program.check(Ty::I32, "from_array", 15);
                program.check(Ty::I32, "from_alloca", 20);
                program.check(Ty::I32, "from_array + from_alloca", 35);
            }
        }
        let axes = Axes::of([("shape", shape)]);
        if gnu {
            sink.push_tagged(Facet::VlaAndAlloca, axes, Dialect::C17, program, &["gnu"]);
        } else {
            sink.push(Facet::VlaAndAlloca, axes, Dialect::C17, program);
        }
    }
}

/// The widest floating type, tested only for what C promises about it everywhere.
///
/// Two rules keep these answers off the machine. Nothing is printed as a floating value, only
/// as the integer it converts to, and every value in every case is one that a `double` holds
/// exactly, so a target where `long double` is `double` gets the same answers as one where it
/// is eighty bits. What is left to test is where the value travels, which is the part the back
/// end decides and the part that is actually wrong in practice.
fn long_double(sink: &mut Sink<'_>) {
    /// The shapes, and the tag the shape needs, if any.
    const SHAPES: &[(&str, &str)] = &[
        ("exact-on-small-integers", ""),
        ("every-double-is-one", ""),
        ("at-least-as-wide-as-double", "headers"),
        ("as-an-argument", ""),
        ("mixed-with-integers", ""),
        ("through-varargs", "headers"),
        ("in-a-struct", ""),
        ("converted-both-ways", ""),
        ("compared-against-integers", ""),
        ("written-as-a-literal", ""),
    ];
    for &(shape, tag) in SHAPES {
        if !sink.wants(Facet::LongDouble) {
            return;
        }
        let mut program = Program::new(format!("long double, {}", shape.replace('-', " ")));
        match shape {
            "exact-on-small-integers" => {
                program.top("static volatile double left_in = 123456.0;");
                program.top("static volatile double right_in = 7000.0;");
                program.blank();
                program.line("long double left = left_in;");
                program.line("long double right = right_in;");
                program.blank();
                // 864192000 is under two to the thirty first, so it is exact in every floating
                // type C has, and the case says nothing about how wide this one is.
                program.check(Ty::I64, "left + right", 130_456);
                program.check(Ty::I64, "left - right", 116_456);
                program.check(Ty::I64, "left * right", 864_192_000);
                program.check(Ty::I64, "(left * right) / right", 123_456);
            }
            "every-double-is-one" => {
                program.top("static volatile double values[4] = { 0.5, -2.25, 1048576.0, 1e9 };");
                program.blank();
                // C says the values of double are a subset of the values of long double, so
                // the round trip is exact on every conforming target, however wide either is.
                program.line("int kept = 1;");
                program.line("for (int i = 0; i < 4; i++) {");
                program.line_at(1, "double narrow = values[i];");
                program.line_at(1, "long double wide = narrow;");
                program.line_at(1, "if ((double)wide != narrow) {");
                program.line_at(2, "kept = 0;");
                program.line_at(1, "}");
                program.line_at(1, "if (wide != narrow) {");
                program.line_at(2, "kept = 0;");
                program.line_at(1, "}");
                program.line("}");
                program.blank();
                program.check(Ty::I32, "kept", 1);
            }
            "at-least-as-wide-as-double" => {
                program.include("float.h");
                program.blank();
                // Every one of these is required by the limits section of C rather than by a
                // target, so the answer is one on an eighty bit long double, on a hundred and
                // twenty eight bit one, and on a target where it is double under another name.
                program.check(Ty::I32, "LDBL_MANT_DIG >= DBL_MANT_DIG", 1);
                program.check(Ty::I32, "LDBL_MAX_EXP >= DBL_MAX_EXP", 1);
                program.check(Ty::I32, "LDBL_MIN_EXP <= DBL_MIN_EXP", 1);
                program.check(Ty::I32, "LDBL_DIG >= DBL_DIG", 1);
                program.check(Ty::I32, "LDBL_EPSILON <= DBL_EPSILON", 1);
            }
            "as-an-argument" => {
                program
                    .top("static long double blend(long double a, long double b, long double c,");
                program
                    .top("                         long double d, long double e, long double f,");
                program.top("                         long double g, long double h) {");
                program.top("    return a * 1.0L + b * 2.0L + c * 3.0L + d * 4.0L");
                program.top("         + e * 5.0L + f * 6.0L + g * 7.0L + h * 8.0L;");
                program.top("}");
                program.top("static long double back(long double value) { return value * 2.0L; }");
                program.input(Ty::I32, "seed", 1);
                program.blank();
                program.line("long double one = seed;");
                program.blank();
                // Eight arguments is more than any target passes in registers, so some of them
                // go through memory, and how wide the slot is and what it is aligned to is the
                // whole question. The answer is the sum of the first eight squares.
                program.check(Ty::I64, "blend(one, 2.0L, 3.0L, 4.0L, 5.0L, 6.0L, 7.0L, 8.0L)", 204);
                program.check(Ty::I64, "back(21.0L)", 42);
                program.check(Ty::I64, "back(back(back(one)))", 8);
            }
            "mixed-with-integers" => {
                program
                    .top("static long double mixed(int a, long double b, int c, long double d) {");
                program.top("    return b * a + d * c;");
                program.top("}");
                program.top("static long double after(long double a, int b, long double c) {");
                program.top("    return a * b - c;");
                program.top("}");
                program.input(Ty::I32, "three", 3);
                program.blank();
                // Integers and long doubles go in different places on every target worth
                // naming, so interleaving them is what puts the two sequences out of step.
                program.check(Ty::I64, "mixed(three, 4.0L, 5, 6.0L)", 42);
                program.check(Ty::I64, "after(10.0L, three, 8.0L)", 22);
            }
            "through-varargs" => {
                program.include("stdarg.h");
                program.top("static long double total(int count, ...) {");
                program.top("    va_list rest;");
                program.top("    va_start(rest, count);");
                program.top("    long double sum = 0.0L;");
                program.top("    for (int i = 0; i < count; i++) {");
                program.top("        sum = sum + va_arg(rest, long double);");
                program.top("    }");
                program.top("    va_end(rest);");
                program.top("    return sum;");
                program.top("}");
                program.top("static long double weighted(int count, ...) {");
                program.top("    va_list rest;");
                program.top("    va_start(rest, count);");
                program.top("    long double sum = 0.0L;");
                program.top("    for (int i = 0; i < count; i++) {");
                program.top("        int weight = va_arg(rest, int);");
                program.top("        sum = sum + va_arg(rest, long double) * weight;");
                program.top("    }");
                program.top("    va_end(rest);");
                program.top("    return sum;");
                program.top("}");
                program.blank();
                // A long double is never promoted on its way through the ellipsis, unlike a
                // float, so the reader has to take exactly the width the caller pushed.
                program.check(Ty::I64, "total(5, 1.0L, 2.0L, 4.0L, 8.0L, 16.0L)", 31);
                program.check(Ty::I64, "weighted(3, 2, 10.0L, 3, 100.0L, 4, 1000.0L)", 4320);
            }
            "in-a-struct" => {
                program.top("struct wide { long double value; int tag; };");
                program.top("static struct wide scale(struct wide held, int by) {");
                program.top("    struct wide out;");
                program.top("    out.value = held.value * by;");
                program.top("    out.tag = held.tag + by;");
                program.top("    return out;");
                program.top("}");
                program.top("static long double sum(struct wide first, struct wide second) {");
                program.top("    return first.value + second.value;");
                program.top("}");
                program.input(Ty::I32, "seven", 7);
                program.blank();
                program.line("struct wide start;");
                program.line("start.value = 12.0L;");
                program.line("start.tag = 5;");
                program.line("struct wide grown = scale(start, seven);");
                program.blank();
                // A struct with a long double in it is passed by memory almost everywhere, and
                // the padding between the member and the tag is target specific, which is a
                // layout question and not an answer question.
                program.check(Ty::I64, "grown.value", 84);
                program.check(Ty::I32, "grown.tag", 12);
                program.check(Ty::I64, "sum(start, grown)", 96);
            }
            "converted-both-ways" => {
                program.top("static volatile long long big_in = 1099511627776ll;");
                program.top("static volatile unsigned long long huge_in = 4000000000ull;");
                program.blank();
                program.line("long long big = big_in;");
                program.line("unsigned long long huge = huge_in;");
                program.line("long double from_signed = big;");
                program.line("long double from_unsigned = huge;");
                program.blank();
                // Two to the fortieth and four thousand million are both exact in a double, so
                // the round trip has to come back unchanged whatever long double is here.
                program.check(Ty::I64, "from_signed", 1_099_511_627_776);
                program.check(Ty::U64, "from_unsigned", 4_000_000_000);
                program.check(Ty::I64, "from_signed / 1024.0L", 1_073_741_824);
                program.check(Ty::I64, "-2.75L * 4.0L", -11);
                program.check(Ty::I32, "3.99L", 3);
                program.check(Ty::I32, "-3.99L", -3);
            }
            "compared-against-integers" => {
                program.input(Ty::I32, "seven", 7);
                program.blank();
                program.line("long double wide = seven;");
                program.blank();
                // The usual arithmetic conversions take the integer up to the wide type rather
                // than the other way round, so the half never gets rounded away.
                program.check(Ty::I32, "wide > 6", 1);
                program.check(Ty::I32, "wide == 7", 1);
                program.check(Ty::I32, "wide < 7.5L", 1);
                program.check(Ty::I32, "wide + 0.5L > seven", 1);
                program.check(Ty::I32, "wide / 2.0L == 3.5L", 1);
                program.check(Ty::I32, "(wide / 2.0L) == (seven / 2)", 0);
            }
            _ => {
                program.blank();
                // The suffix and the hexadecimal form, which are the two ways a long double
                // constant gets written and the two ways a front end mislays the width.
                program.check(Ty::I64, "1.5e3L", 1500);
                program.check(Ty::I64, "0x1p10L", 1024);
                program.check(Ty::I64, "0x1.8p1L", 3);
                program.check(Ty::I64, "2.0L * 0x1p20L", 2_097_152);
                // Not a tenth. Whether the long double nearest to a tenth is a different
                // number from the double nearest to it depends on whether the two types are
                // the same type on this target, and the corpus does not ask that question.
                program.check(Ty::I64, "0x1p-2L * 8.0L", 2);
                program.check(Ty::I32, "0.5L == 0.5", 1);
            }
        }
        let axes = Axes::of([("shape", shape)]);
        if tag.is_empty() {
            sink.push(Facet::LongDouble, axes, Dialect::C17, program);
        } else {
            sink.push_tagged(Facet::LongDouble, axes, Dialect::C17, program, &[tag]);
        }
    }
}

/// The jump out of a function that never returns from it.
///
/// Two rules make these cases well defined rather than merely likely to work. Every local that
/// is written between the `setjmp` and the `longjmp` and read afterwards is `volatile`, which
/// is the one thing C promises about a jumped over frame. And `setjmp` only ever appears where
/// the standard allows it, which is the whole controlling expression of an `if`, a `while` or
/// a `switch`, or one side of a comparison against a constant in one. Assigning its result to
/// a variable is the way everybody writes it and is not something C actually permits.
fn setjmp_longjmp(sink: &mut Sink<'_>) {
    const SHAPES: &[&str] = &[
        "the-value-comes-back",
        "zero-becomes-one",
        "volatile-survives",
        "unchanged-locals-survive",
        "a-retry-loop",
        "out-of-nested-frames",
        "past-an-inner-handler",
        "stores-are-not-sunk",
    ];
    for &shape in SHAPES {
        if !sink.wants(Facet::SetjmpLongjmp) {
            return;
        }
        let mut program = Program::new(format!("setjmp and longjmp, {}", shape.replace('-', " ")));
        program.include("setjmp.h");
        match shape {
            "the-value-comes-back" => {
                program.top("static jmp_buf home;");
                program.top("static volatile int arrivals;");
                program.blank();
                program.line("volatile int answer = 0;");
                program.line("switch (setjmp(home)) {");
                program.line("case 0:");
                program.line_at(1, "arrivals = arrivals + 1;");
                program.line_at(1, "longjmp(home, 7);");
                program.line_at(1, "break;");
                program.line("case 7:");
                program.line_at(1, "answer = 70;");
                program.line_at(1, "break;");
                program.line("default:");
                program.line_at(1, "answer = -1;");
                program.line_at(1, "break;");
                program.line("}");
                program.blank();
                // A switch on the result is the dispatch every interpreter writes, and it is
                // also the shape that makes the second arrival land in a different arm.
                program.check(Ty::I32, "arrivals", 1);
                program.check(Ty::I32, "answer", 70);
            }
            "zero-becomes-one" => {
                program.top("static jmp_buf home;");
                program.blank();
                program.line("volatile int answer = -1;");
                program.line("switch (setjmp(home)) {");
                program.line("case 0:");
                program.line_at(1, "longjmp(home, 0);");
                program.line_at(1, "break;");
                program.line("case 1:");
                program.line_at(1, "answer = 1;");
                program.line_at(1, "break;");
                program.line("default:");
                program.line_at(1, "answer = 99;");
                program.line_at(1, "break;");
                program.line("}");
                program.blank();
                // Asking for zero has to arrive as one, because zero is how the first return
                // says it was the first. A compiler that passes the value straight through
                // turns the handler into an infinite loop, which is a real bug and not a
                // pedantic one.
                program.check(Ty::I32, "answer", 1);
            }
            "volatile-survives" => {
                program.top("static jmp_buf home;");
                program.blank();
                program.line("volatile int counter = 0;");
                program.line("counter = counter + 10;");
                program.line("if (setjmp(home) == 0) {");
                program.line_at(1, "counter = counter + 5;");
                program.line_at(1, "longjmp(home, 1);");
                program.line("}");
                program.blank();
                // The one guarantee C gives about a frame that was jumped over. A volatile
                // local keeps whatever was last written to it, so the five is still there.
                program.check(Ty::I32, "counter", 15);
            }
            "unchanged-locals-survive" => {
                program.top("static jmp_buf home;");
                program.top("static volatile int noise = 2;");
                program.blank();
                program.line("int fixed = 40;");
                program.line("volatile int answer = 0;");
                program.line("if (setjmp(home) == 0) {");
                program.line_at(1, "longjmp(home, 1);");
                program.line("}");
                program.line("answer = fixed + noise;");
                program.blank();
                // The other half of the rule, and the half a compiler breaks by being clever
                // rather than by being careless. Nothing writes to fixed between the setjmp
                // and the longjmp, so it is not indeterminate and it has to still be forty.
                program.check(Ty::I32, "answer", 42);
                program.check(Ty::I32, "fixed", 40);
            }
            "a-retry-loop" => {
                program.top("static jmp_buf home;");
                program.top("static void fail_at(int step) { longjmp(home, step); }");
                program.blank();
                program.line("volatile int attempts = 0;");
                program.line("volatile int recoveries = 0;");
                program.line("if (setjmp(home) != 0) {");
                program.line_at(1, "recoveries = recoveries + 1;");
                program.line("}");
                program.line("attempts = attempts + 1;");
                program.line("if (attempts < 4) {");
                program.line_at(1, "fail_at(attempts);");
                program.line("}");
                program.blank();
                // The error unwinding shape every interpreter on the ladder is built from. The
                // handler runs three times and the fourth pass falls through it.
                program.check(Ty::I32, "attempts", 4);
                program.check(Ty::I32, "recoveries", 3);
            }
            "out-of-nested-frames" => {
                program.top("static jmp_buf home;");
                program.top("static volatile int trail;");
                program.top("static void third(void) {");
                program.top("    trail = trail * 10 + 3;");
                program.top("    longjmp(home, 5);");
                program.top("}");
                program.top("static void second(void) {");
                program.top("    trail = trail * 10 + 2;");
                program.top("    third();");
                program.top("    trail = trail * 10 + 9;");
                program.top("}");
                program.top("static void first(void) {");
                program.top("    trail = trail * 10 + 1;");
                program.top("    second();");
                program.top("    trail = trail * 10 + 8;");
                program.top("}");
                program.blank();
                program.line("volatile int answer = 0;");
                program.line("if (setjmp(home) == 0) {");
                program.line_at(1, "first();");
                program.line_at(1, "trail = trail * 10 + 7;");
                program.line("} else {");
                program.line_at(1, "answer = 5;");
                program.line("}");
                program.blank();
                // Three frames go away at once and none of them finishes. The trail says which
                // statements ran, so a compiler that inlined the chain and then let the tail
                // of one of them run anyway is caught by the digits rather than by a crash.
                program.check(Ty::I32, "trail", 123);
                program.check(Ty::I32, "answer", 5);
            }
            "past-an-inner-handler" => {
                program.top("static jmp_buf outer;");
                program.top("static jmp_buf inner;");
                program.top("static volatile int trail;");
                program.blank();
                program.line("volatile int answer = 0;");
                program.line("if (setjmp(outer) == 0) {");
                program.line_at(1, "if (setjmp(inner) == 0) {");
                program.line_at(2, "trail = trail * 10 + 1;");
                program.line_at(2, "longjmp(outer, 2);");
                program.line_at(1, "}");
                program.line_at(1, "trail = trail * 10 + 9;");
                program.line("} else {");
                program.line_at(1, "answer = 2;");
                program.line("}");
                program.blank();
                // Two live buffers and the jump goes to the outer one, so the inner handler
                // never runs. That is how a runtime rethrows past a handler it has finished
                // with, and it is the case where a compiler that keeps one buffer gets it
                // wrong in a way a single buffer program would never show.
                program.check(Ty::I32, "trail", 1);
                program.check(Ty::I32, "answer", 2);
            }
            _ => {
                program.top("static jmp_buf home;");
                program.top("static volatile int port;");
                program.top("static void raise_it(void) { longjmp(home, 3); }");
                program.blank();
                program.line("volatile int answer = 0;");
                program.line("if (setjmp(home) == 0) {");
                program.line_at(1, "port = 11;");
                program.line_at(1, "raise_it();");
                program.line_at(1, "port = 22;");
                program.line("} else {");
                program.line_at(1, "answer = port;");
                program.line("}");
                program.blank();
                // The barrier half of the facet. The call looks ordinary and the store after
                // it looks dead, so a compiler that has not been told the call can leave
                // sideways will happily move the twenty two up or drop the eleven.
                program.check(Ty::I32, "port", 11);
                program.check(Ty::I32, "answer", 11);
            }
        }
        let axes = Axes::of([("shape", shape)]);
        sink.push_tagged(Facet::SetjmpLongjmp, axes, Dialect::C17, program, &["headers"]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, Sink};
    use corpus_model::{Expect, Facet};

    /// Every case this module emits.
    fn cases() -> Vec<corpus_model::Case> {
        let opts = Options::all();
        let mut sink = Sink::new(&opts);
        super::generate(&mut sink);
        sink.into_cases()
    }

    #[test]
    fn all_three_facets_produce_cases() {
        let cases = cases();
        for facet in [Facet::VlaAndAlloca, Facet::LongDouble, Facet::SetjmpLongjmp] {
            assert!(cases.iter().any(|case| case.facet == facet), "nothing for {facet}");
        }
    }

    #[test]
    fn no_case_prints_a_size_because_a_size_is_a_property_of_the_machine() {
        for case in cases() {
            assert!(!case.source.contains("sizeof"), "{} would print a machine number", case.id);
        }
    }

    #[test]
    fn setjmp_only_appears_where_c_allows_it() {
        for case in cases().iter().filter(|case| case.facet == Facet::SetjmpLongjmp) {
            for line in case.source.lines().filter(|line| line.contains("setjmp(")) {
                let trimmed = line.trim();
                let allowed = trimmed.starts_with("if (setjmp(")
                    || trimmed.starts_with("switch (setjmp(")
                    || trimmed.starts_with("while (setjmp(");
                assert!(allowed, "{} writes setjmp somewhere C does not allow: {trimmed}", case.id);
            }
        }
    }

    #[test]
    fn every_local_read_after_a_jump_is_volatile() {
        for case in cases().iter().filter(|case| case.facet == Facet::SetjmpLongjmp) {
            let non_volatile = case
                .source
                .lines()
                .filter(|line| line.trim().starts_with("int ") && line.starts_with("    "))
                .count();
            // The one local that is not volatile is the one no case writes after the setjmp,
            // which is the whole subject of the case that has it.
            let expected = usize::from(case.id.contains("unchanged-locals-survive"));
            assert_eq!(
                non_volatile, expected,
                "{} has a local C would call indeterminate",
                case.id
            );
        }
    }

    #[test]
    fn the_extension_cases_say_so_and_the_header_cases_say_so() {
        for case in cases() {
            if case.source.contains("__builtin_alloca") {
                assert!(case.tags.iter().any(|tag| tag == "gnu"), "{} is untagged", case.id);
            }
            if case.source.contains("#include") {
                assert!(case.tags.iter().any(|tag| tag == "headers"), "{} is untagged", case.id);
            }
        }
    }

    #[test]
    fn every_bound_of_a_run_time_sized_object_comes_from_somewhere_opaque() {
        for case in cases().iter().filter(|case| case.facet == Facet::VlaAndAlloca) {
            assert!(
                case.source.contains("static volatile"),
                "{} has a bound the optimizer can fold back into a constant",
                case.id
            );
        }
    }

    #[test]
    fn every_case_carries_the_answer_the_generator_worked_out() {
        for case in cases() {
            match &case.expect {
                Expect::Output(text) => assert!(!text.is_empty(), "{} has no oracle", case.id),
                Expect::Rejected(_) => panic!("{} is not a rejection case", case.id),
            }
        }
    }
}
