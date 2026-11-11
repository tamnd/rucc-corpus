//! Unnamed members as Plan 9 C has them, which the Plan 9 tree depends on.
//!
//! `kencc-members` is the unnamed members of kencc. A record can hold a `struct` or a `union` by its type name only, as `struct Ref { Lock; long ref; }` in the 9front tree.
//! The members of such a member are members of the record, and the type name names the member itself, as `r.Lock`.
//! A pointer to the record converts to a pointer to the member, and under `-fkencc` the record converts to the member as a value too.
//! gcc has a part of this as `-fplan9-extensions`, and rucc has all of it as `-fkencc`.
//! A shape that gcc can compile is built under `-fplan9-extensions` and the run compares the two compilers.
//! Each shape is built under `-fkencc` too, and has the tag `rucc-dialect`, so the run gives it only to rucc.
//! The answer that such a case must print is in the case, so the run still checks rucc.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, RUCC_DIALECT};

/// One shape: the types, the statements that set the object, and each expression with its value.
struct Shape {
    /// The name of the shape in the axes.
    name: &'static str,
    /// Whether gcc compiles the shape under `-fplan9-extensions`.
    gcc: bool,
    /// The types and the functions that the shape uses, at file scope.
    top: &'static [&'static str],
    /// The statements in `main` that declare and set the objects.
    body: &'static [&'static str],
    /// Each expression and its value.
    checks: &'static [(&'static str, i128)],
}

/// The types that most of the shapes use, as `lib9.h` and `9p.h` declare them.
const LOCK: &str = "typedef struct Lock { int key; } Lock;";
const REF: &str = "typedef struct Ref { Lock; long ref; } Ref;";

/// The shapes of `kencc-members`.
const SHAPES: &[Shape] = &[
    Shape {
        name: "member-of-member",
        gcc: true,
        top: &[LOCK, REF],
        body: &["Ref r;", "r.key = 3;", "r.ref = 40;"],
        checks: &[("r.key", 3), ("(int)r.ref", 40)],
    },
    Shape {
        name: "member-by-type-name",
        gcc: true,
        top: &[LOCK, REF],
        body: &["Ref r;", "r.Lock.key = 5;", "r.ref = 1;"],
        checks: &[("r.key", 5), ("r.Lock.key", 5), ("(int)r.ref", 1)],
    },
    Shape {
        name: "pointer-argument",
        gcc: true,
        top: &[LOCK, REF, "static int key(Lock *l) { return l->key * 2; }"],
        body: &["Ref r;", "r.key = 21;", "r.ref = 0;"],
        checks: &[("key(&r)", 42), ("key(&r.Lock)", 42)],
    },
    Shape {
        name: "pointer-assignment",
        gcc: true,
        top: &[
            "typedef struct Pad { char c[3]; } Pad;",
            LOCK,
            "typedef struct Big { Pad; Lock; long n; } Big;",
        ],
        body: &[
            "Big b;",
            "Big *bp = &b;",
            "Lock *lp = bp;",
            "b.key = 9;",
            "b.n = 4;",
            "lp->key += 1;",
        ],
        checks: &[("b.key", 10), ("lp->key", 10), ("(int)((char *)lp - (char *)bp)", 4)],
    },
    Shape {
        name: "nested",
        gcc: true,
        top: &[LOCK, REF, "typedef struct Outer { Ref; int o; } Outer;"],
        body: &["Outer o;", "Lock *lp = &o;", "o.key = 5;", "o.ref = 6;", "o.o = 7;"],
        checks: &[
            ("o.key", 5),
            ("o.Lock.key", 5),
            ("(int)o.Ref.ref", 6),
            ("lp->key", 5),
            ("o.o", 7),
        ],
    },
    Shape {
        name: "union-member",
        gcc: true,
        top: &[
            "typedef union Word { int i; unsigned char b[4]; } Word;",
            "typedef struct Cell { int tag; Word; } Cell;",
        ],
        body: &["Cell c;", "c.tag = 1;", "c.i = 0;", "c.b[0] = 7;"],
        checks: &[("c.tag", 1), ("c.Word.b[0]", 7)],
    },
    Shape {
        name: "member-by-tag",
        gcc: false,
        top: &["struct Frame { int r; int maxtab; };", "struct Window { struct Frame; int w; };"],
        body: &["struct Window w;", "w.r = 2;", "w.maxtab = 3;", "w.w = 4;"],
        checks: &[("w.Frame.r", 2), ("w.Frame.maxtab", 3), ("w.w", 4)],
    },
    Shape {
        name: "own-member-first",
        gcc: false,
        top: &[LOCK, "typedef struct Key { Lock; int key; } Key;"],
        body: &["Key k;", "k.key = 1;", "k.Lock.key = 2;"],
        checks: &[("k.key", 1), ("k.Lock.key", 2)],
    },
    Shape {
        name: "same-name-twice",
        gcc: false,
        top: &[
            "typedef struct QLock { int locked; long head; } QLock;",
            "typedef struct Rendez { QLock *l; long head; } Rendez;",
            "typedef struct Queue { QLock; Rendez; int pid; } Queue;",
        ],
        body: &["Queue q;", "q.QLock.head = 1;", "q.Rendez.head = 2;", "q.l = 0;", "q.pid = 3;"],
        checks: &[
            ("(int)q.QLock.head", 1),
            ("(int)q.Rendez.head", 2),
            ("q.l == 0", 1),
            ("q.pid", 3),
        ],
    },
    Shape {
        name: "value-conversion",
        gcc: false,
        top: &[LOCK, REF, "static int value(Lock l) { return l.key + 1; }"],
        body: &["Ref r;", "r.key = 8;", "r.ref = 0;", "Lock l = r;"],
        checks: &[("value(r)", 9), ("l.key", 8)],
    },
];

/// A program that kencc refuses, and the text that its diagnostic has.
const AMBIGUOUS: &str = "int printf(const char *, ...);\n\n\
    typedef struct A { int x; } A;\n\
    typedef struct B { int x; } B;\n\
    typedef struct C { A; B; } C;\n\n\
    int main(void) {\n\
    \x20   C c;\n\
    \x20   c.A.x = 1;\n\
    \x20   c.B.x = 2;\n\
    \x20   printf(\"%d\\n\", c.x);\n\
    \x20   return 0;\n\
    }\n";

pub(crate) fn generate(sink: &mut Sink<'_>) {
    for shape in SHAPES {
        let dialects: &[&str] = if shape.gcc { &["plan9-extensions", "kencc"] } else { &["kencc"] };
        for &dialect in dialects {
            if !sink.wants(Facet::KenccMembers) {
                return;
            }
            let mut program = Program::new(format!(
                "the unnamed members of the {} shape under -f{dialect}",
                shape.name
            ));
            for line in shape.top {
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
            let axes = Axes::of([("shape", shape.name), ("dialect", dialect)]);
            let flags = vec![format!("-f{dialect}")];
            let case = Case::linked(
                Facet::KenccMembers,
                axes,
                Dialect::C17,
                source,
                Vec::new(),
                flags,
                Expect::Output(expected),
            );
            let case = if dialect == "kencc" {
                case.tagged(&["gnu", RUCC_DIALECT])
            } else {
                case.tagged(&["gnu"])
            };
            sink.push_case(case);
        }
    }
    let case = Case::linked(
        Facet::KenccMembers,
        Axes::of([("shape", "ambiguous"), ("dialect", "kencc")]),
        Dialect::C17,
        format!("// An access to a name that two unnamed members have, which kencc refuses.\n// Generated by rucc-corpus. Edit the generator, not this file.\n\n{AMBIGUOUS}"),
        Vec::new(),
        vec!["-fkencc".to_owned()],
        Expect::Rejected("ambiguous structure element".to_owned()),
    )
    .tagged(&["gnu", RUCC_DIALECT]);
    sink.push_case(case);
}
