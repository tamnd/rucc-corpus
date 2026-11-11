//! Brace elision over arrays and records, which the Plan 9 tree depends on.
//!
//! `brace-elision` is the initializers that leave out the inner braces of an aggregate, as C11 6.7.9p20 lets them.
//! The 9front tree has many tables of this form, such as `char *d[][2] = { "a", "b", ... }`, and rucc read them wrongly before tamnd/rucc#4200.
//! There a string literal took a whole row of pointers, where it is the value of one pointer.
//! A string literal fills an array as a whole only where the elements of the array are characters.
//! Each case declares one object at file scope or in a block, and prints each element as a number.
//! A pointer prints as the first character of its string, or 0 when it is null.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Dialect, Facet};

/// One shape: the declaration, and each expression with the value that it must print.
struct Shape {
    /// The name of the shape in the axes.
    name: &'static str,
    /// The types that the declaration uses, at file scope.
    types: &'static [&'static str],
    /// The declaration of the object, which is named `d`.
    decl: &'static str,
    /// Each expression and its value.
    checks: &'static [(&'static str, i128)],
}

/// The first character of the string that a pointer points to, or 0 for a null pointer.
const FIRST: &str = "static int first(const char *s) { return s ? s[0] : 0; }";

/// The shapes of `brace-elision`.
const SHAPES: &[Shape] = &[
    Shape {
        name: "pointer-matrix",
        types: &[],
        decl: "char *d[2][2] = { \"a\", \"b\", \"c\", \"d\" };",
        checks: &[
            ("first(d[0][0])", 97),
            ("first(d[0][1])", 98),
            ("first(d[1][0])", 99),
            ("first(d[1][1])", 100),
        ],
    },
    Shape {
        name: "pointer-matrix-unsized",
        types: &[],
        decl: "char *d[][2] = { \"a\", \"b\", \"c\" };",
        checks: &[
            ("(int)(sizeof d / sizeof d[0])", 2),
            ("first(d[0][1])", 98),
            ("first(d[1][0])", 99),
            ("first(d[1][1])", 0),
        ],
    },
    Shape {
        name: "pointer-matrix-mixed",
        types: &[],
        decl: "char *d[3][2] = { { \"a\" }, \"c\", \"d\", \"e\" };",
        checks: &[
            ("first(d[0][0])", 97),
            ("first(d[0][1])", 0),
            ("first(d[1][0])", 99),
            ("first(d[1][1])", 100),
            ("first(d[2][0])", 101),
            ("first(d[2][1])", 0),
        ],
    },
    Shape {
        name: "pointer-cube",
        types: &[],
        decl: "const char *d[2][2][2] = { \"a\", \"b\", \"c\", \"d\", \"e\" };",
        checks: &[
            ("first(d[0][0][0])", 97),
            ("first(d[0][1][1])", 100),
            ("first(d[1][0][0])", 101),
            ("first(d[1][1][1])", 0),
        ],
    },
    Shape {
        name: "record-of-pointers",
        types: &["struct Tab { char *name; char *alias; int code; };"],
        decl: "struct Tab d[] = { \"add\", \"plus\", 1, \"sub\", 0, 2, \"mul\" };",
        checks: &[
            ("(int)(sizeof d / sizeof d[0])", 3),
            ("first(d[0].alias)", 112),
            ("d[0].code", 1),
            ("first(d[1].name)", 115),
            ("first(d[1].alias)", 0),
            ("d[1].code", 2),
            ("first(d[2].name)", 109),
            ("d[2].code", 0),
        ],
    },
    Shape {
        name: "record-with-pointer-array",
        types: &["struct Op { char *names[2]; int arity; };"],
        decl: "struct Op d[2] = { \"neg\", \"not\", 1, \"and\", \"or\", 2 };",
        checks: &[
            ("first(d[0].names[0])", 110),
            ("first(d[0].names[1])", 110),
            ("d[0].arity", 1),
            ("first(d[1].names[0])", 97),
            ("first(d[1].names[1])", 111),
            ("d[1].arity", 2),
        ],
    },
    Shape {
        name: "character-matrix",
        types: &[],
        decl: "char d[3][4] = { \"ab\", \"cde\", \"f\" };",
        checks: &[
            ("d[0][0]", 97),
            ("d[0][2]", 0),
            ("d[1][2]", 101),
            ("d[1][3]", 0),
            ("d[2][0]", 102),
            ("d[2][1]", 0),
        ],
    },
    Shape {
        name: "character-matrix-flat",
        types: &[],
        decl: "char d[2][3] = { 'a', 'b', 'c', 'd' };",
        checks: &[("d[0][2]", 99), ("d[1][0]", 100), ("d[1][1]", 0), ("d[1][2]", 0)],
    },
    Shape {
        name: "record-with-character-array",
        types: &["struct Key { char name[4]; int value; };"],
        decl: "struct Key d[] = { \"esc\", 27, \"del\", 127 };",
        checks: &[
            ("(int)(sizeof d / sizeof d[0])", 2),
            ("d[0].name[0]", 101),
            ("d[0].name[3]", 0),
            ("d[0].value", 27),
            ("d[1].name[2]", 108),
            ("d[1].value", 127),
        ],
    },
    Shape {
        name: "integer-matrix",
        types: &[],
        decl: "int d[2][3] = { 1, 2, 3, 4 };",
        checks: &[("d[0][2]", 3), ("d[1][0]", 4), ("d[1][1]", 0), ("d[1][2]", 0)],
    },
];

/// Where the object is declared.
const SCOPES: &[&str] = &["file", "block"];

pub(crate) fn generate(sink: &mut Sink<'_>) {
    for shape in SHAPES {
        for &scope in SCOPES {
            if !sink.wants(Facet::BraceElision) {
                return;
            }
            let mut program =
                Program::new(format!("brace elision in the {} shape at {scope} scope", shape.name));
            for line in shape.types {
                program.top(*line);
            }
            if shape.checks.iter().any(|(expr, _)| expr.contains("first(")) {
                program.top(FIRST);
            }
            if scope == "file" {
                program.top(format!("static {}", shape.decl));
            } else {
                program.line(shape.decl);
                program.blank();
            }
            for &(expr, value) in shape.checks {
                program.check(Ty::I32, expr, value);
            }
            let axes = Axes::of([("shape", shape.name), ("scope", scope)]);
            sink.push(Facet::BraceElision, axes, Dialect::C17, program);
        }
    }
}
