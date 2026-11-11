//! The record layout of kencc, which an object from rucc must share with the objects of `6c`.
//!
//! `kencc-layout` is `sualign` of `cc/dcl.c` in kencc, with the widths of `align` in `6c/swt.c` and `7c/swt.c`.
//! A scalar member goes at a multiple of its width, but a `struct` or a `union` member, and an array of one, goes at a multiple of 8.
//! The size of every `struct` and `union` rounds up to 8, so `struct { char c; }` is 8 bytes.
//! A bit-field is in a unit of 4 bytes whatever its type, and its bits go from the low bit up.
//! A member with a name that is not a bit-field ends the unit, and so does a bit-field of width zero.
//! The cases are built under `-fkencc -fkencc-layout`, which gives the layout of `6c` to the target of the run.
//! The layout gives a scalar the alignment of its width, so a case that has only `char`, `short`, `int`, `long long`, `float` and `double` prints the same on each target.
//! A case prints the size and the offsets of its records, and the bytes that a store to its bit-fields sets.
//! The answers were compared with `6c` on 9front amd64.
//! Only rucc has the dialect, so each case has the tag `rucc-dialect`.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, RUCC_DIALECT};

/// The helpers of each case: the offset of a member, a byte of an object, and a store of zero
/// to each byte of an object.
const HELPERS: &[&str] = &[
    "#define OFF(t, m) ((int)(long long)&((t *)0)->m)",
    "static int byte(void *p, int i) { return ((unsigned char *)p)[i]; }",
    "static void zero(void *p, int n) { unsigned char *b = p; while (n-- > 0) *b++ = 0; }",
];

/// The record that most of the shapes hold as a member.
const IN: &str = "typedef struct In { char d; } In;";

/// One shape: the types, the statements that set the object, and each expression with its value.
struct Shape {
    /// The name of the shape in the axes.
    name: &'static str,
    /// The types that the shape uses, at file scope.
    top: &'static [&'static str],
    /// The statements in `main` that declare and set the objects.
    body: &'static [&'static str],
    /// Each expression and its value.
    checks: &'static [(&'static str, i128)],
}

/// The shapes of `kencc-layout`.
const SHAPES: &[Shape] = &[
    Shape {
        name: "one-char",
        top: &["typedef struct C { char c; } C;"],
        body: &[],
        checks: &[("sizeof(C)", 8)],
    },
    Shape {
        name: "scalars",
        top: &["typedef struct M { char c; short s; int i; char d; } M;"],
        body: &[],
        checks: &[("sizeof(M)", 16), ("OFF(M, s)", 2), ("OFF(M, i)", 4), ("OFF(M, d)", 8)],
    },
    Shape {
        name: "floating",
        top: &["typedef struct D { char c; double d; float f; } D;"],
        body: &[],
        checks: &[("sizeof(D)", 24), ("OFF(D, d)", 8), ("OFF(D, f)", 16)],
    },
    Shape {
        name: "vlong",
        top: &["typedef struct V { int i; long long v; char c; } V;"],
        body: &[],
        checks: &[("sizeof(V)", 24), ("OFF(V, v)", 8), ("OFF(V, c)", 16)],
    },
    Shape {
        name: "record-member",
        top: &[IN, "typedef struct R { char c; In t; char e; } R;"],
        body: &[],
        checks: &[("sizeof(R)", 24), ("OFF(R, t)", 8), ("OFF(R, e)", 16)],
    },
    Shape {
        name: "record-array",
        top: &[IN, "typedef struct A { char c; In a[2]; char e; } A;"],
        body: &[],
        checks: &[("sizeof(A)", 32), ("OFF(A, a)", 8), ("OFF(A, e)", 24)],
    },
    Shape {
        name: "scalar-array",
        top: &["typedef struct S3 { char c; char a[3]; short s; } S3;"],
        body: &[],
        checks: &[("sizeof(S3)", 8), ("OFF(S3, a)", 1), ("OFF(S3, s)", 4)],
    },
    Shape {
        name: "union",
        top: &[
            "typedef union U { char c; short s; int i; } U;",
            "typedef union U9 { char a[9]; } U9;",
        ],
        body: &[],
        checks: &[("sizeof(U)", 8), ("sizeof(U9)", 16)],
    },
    Shape {
        name: "bit-fields",
        top: &["typedef struct B { int a:3; int b:5; char c; int d:4; } B;"],
        body: &["B x;", "zero(&x, sizeof x);", "x.a = 1;", "x.b = 2;", "x.c = 3;", "x.d = 5;"],
        checks: &[
            ("sizeof(B)", 16),
            ("OFF(B, c)", 4),
            ("byte(&x, 0)", 17),
            ("byte(&x, 4)", 3),
            ("byte(&x, 8)", 5),
            ("x.a + x.b * 10 + x.d * 100", 521),
        ],
    },
    Shape {
        name: "zero-width",
        top: &["typedef struct Z { char a:3; int :0; char b:2; } Z;"],
        body: &["Z x;", "zero(&x, sizeof x);", "x.a = 3;", "x.b = 1;"],
        checks: &[("sizeof(Z)", 8), ("byte(&x, 0)", 3), ("byte(&x, 4)", 1), ("x.a + x.b * 10", 13)],
    },
    Shape {
        name: "full-unit",
        top: &["typedef struct O { int a:30; int b:4; } O;"],
        body: &["O x;", "zero(&x, sizeof x);", "x.a = 1;", "x.b = 5;"],
        checks: &[("sizeof(O)", 8), ("byte(&x, 0)", 1), ("byte(&x, 4)", 5), ("x.a + x.b * 10", 51)],
    },
    Shape {
        name: "narrow-types",
        top: &["typedef struct N { char a:3; short b:9; } N;"],
        body: &["N x;", "zero(&x, sizeof x);", "x.a = 3;", "x.b = 129;"],
        checks: &[
            ("sizeof(N)", 8),
            ("byte(&x, 0)", 11),
            ("byte(&x, 1)", 4),
            ("x.a + x.b * 10", 1293),
        ],
    },
    Shape {
        name: "unnamed-field",
        top: &["typedef struct Un { int a:4; int :4; int b:4; } Un;"],
        body: &["Un x;", "zero(&x, sizeof x);", "x.a = 1;", "x.b = 2;"],
        checks: &[("sizeof(Un)", 8), ("byte(&x, 0)", 1), ("byte(&x, 1)", 2)],
    },
    Shape {
        name: "union-bit-field",
        top: &["typedef union UB { int a:3; char c; } UB;"],
        body: &[],
        checks: &[("sizeof(UB)", 8)],
    },
    Shape {
        name: "bit-field-after-record",
        top: &[IN, "typedef struct BR { In t; int a:2; } BR;"],
        body: &["BR x;", "zero(&x, sizeof x);", "x.t.d = 7;", "x.a = 1;"],
        checks: &[("sizeof(BR)", 16), ("byte(&x, 0)", 7), ("byte(&x, 8)", 1), ("x.a", 1)],
    },
];

pub(crate) fn generate(sink: &mut Sink<'_>) {
    for shape in SHAPES {
        if !sink.wants(Facet::KenccLayout) {
            return;
        }
        let mut program = Program::new(format!("the layout of kencc for the {} shape", shape.name));
        for line in HELPERS.iter().chain(shape.top) {
            program.top(*line);
        }
        for line in shape.body {
            program.line(*line);
        }
        program.blank();
        for &(expr, value) in shape.checks {
            program.check(Ty::I32, expr, value);
        }
        let (source, expected) = program.finish();
        let case = Case::linked(
            Facet::KenccLayout,
            Axes::of([("shape", shape.name), ("dialect", "kencc")]),
            Dialect::C17,
            source,
            Vec::new(),
            vec!["-fkencc".to_owned(), "-fkencc-layout".to_owned()],
            Expect::Output(expected),
        )
        .tagged(&["gnu", RUCC_DIALECT]);
        sink.push_case(case);
    }
}
