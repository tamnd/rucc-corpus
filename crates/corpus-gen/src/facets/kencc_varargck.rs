//! The format check of kencc `-F`, which the Plan 9 tree has on in every mkfile.
//!
//! `kencc-varargck` is the `#pragma varargck` lines of kencc. An `argpos` line names a function whose argument is a format, and a `type` line gives the type that a verb with its flags takes.
//! kencc checks each call of such a function in `dpcheck` of `cc/dpchk.c`, and rucc does the same under `-fkencc -fkencc-format`.
//! The verbs are the ones that `/sys/include/libc.h` of 9front registers, with the lines of that file, and the struct verbs of `draw.h` and the pointer verbs of `ip.h`.
//! Each verb has a case with an argument of its type, which must compile with no warning, and a case with an argument of another type, which must give the kencc message.
//! The cases are built with `-Werror`, so a warning is a rejection.
//! The messages were compared with `6c -F` on 9front.
//! Only rucc has the dialect, so each case has the tag `rucc-dialect`.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, RUCC_DIALECT};

/// The types of `u.h` for amd64 that the lines name.
const TYPES: &[&str] = &[
    "typedef unsigned short ushort;",
    "typedef unsigned char uchar;",
    "typedef unsigned long ulong;",
    "typedef unsigned int uint;",
    "typedef long long vlong;",
    "typedef unsigned long long uvlong;",
    "typedef long long intptr;",
    "typedef unsigned long long uintptr;",
    "typedef uint Rune;",
    "typedef struct Point { int x; int y; } Point;",
    "typedef struct Rectangle { Point min; Point max; } Rectangle;",
];

/// The functions that the `argpos` lines name. Each gives the first byte of its format, which is `%` (37) in every call here.
const FUNCTIONS: &[&str] = &[
    "static int print(char *fmt, ...) { return fmt[0]; }",
    "static int fprint(int fd, char *fmt, ...) { return fmt[0] + fd - fd; }",
];

/// The lines of `libc.h`, in its order, and the lines of `draw.h` and `ip.h` that the cases use.
const LINES: &[&str] = &[
    "#pragma varargck argpos fprint 2",
    "#pragma varargck argpos print 1",
    "#pragma varargck type \"lld\" vlong",
    "#pragma varargck type \"llo\" vlong",
    "#pragma varargck type \"llx\" vlong",
    "#pragma varargck type \"llb\" vlong",
    "#pragma varargck type \"lld\" uvlong",
    "#pragma varargck type \"llo\" uvlong",
    "#pragma varargck type \"llx\" uvlong",
    "#pragma varargck type \"llb\" uvlong",
    "#pragma varargck type \"ld\" long",
    "#pragma varargck type \"lo\" long",
    "#pragma varargck type \"lx\" long",
    "#pragma varargck type \"lb\" long",
    "#pragma varargck type \"ld\" ulong",
    "#pragma varargck type \"lo\" ulong",
    "#pragma varargck type \"lx\" ulong",
    "#pragma varargck type \"lb\" ulong",
    "#pragma varargck type \"zd\" intptr",
    "#pragma varargck type \"zo\" intptr",
    "#pragma varargck type \"zx\" intptr",
    "#pragma varargck type \"zb\" intptr",
    "#pragma varargck type \"zd\" uintptr",
    "#pragma varargck type \"zo\" uintptr",
    "#pragma varargck type \"zx\" uintptr",
    "#pragma varargck type \"zb\" uintptr",
    "#pragma varargck type \"d\" int",
    "#pragma varargck type \"o\" int",
    "#pragma varargck type \"x\" int",
    "#pragma varargck type \"c\" int",
    "#pragma varargck type \"C\" int",
    "#pragma varargck type \"b\" int",
    "#pragma varargck type \"d\" uint",
    "#pragma varargck type \"x\" uint",
    "#pragma varargck type \"c\" uint",
    "#pragma varargck type \"C\" uint",
    "#pragma varargck type \"b\" uint",
    "#pragma varargck type \"f\" double",
    "#pragma varargck type \"e\" double",
    "#pragma varargck type \"g\" double",
    "#pragma varargck type \"s\" char*",
    "#pragma varargck type \"q\" char*",
    "#pragma varargck type \"S\" Rune*",
    "#pragma varargck type \"Q\" Rune*",
    "#pragma varargck type \"r\" void",
    "#pragma varargck type \"%\" void",
    "#pragma varargck type \"n\" int*",
    "#pragma varargck type \"p\" uintptr",
    "#pragma varargck type \"p\" void*",
    "#pragma varargck flag ','",
    "#pragma varargck flag ' '",
    "#pragma varargck flag 'h'",
    "#pragma varargck type \"<\" void*",
    "#pragma varargck type \"[\" void*",
    "#pragma varargck type \"H\" void*",
    "#pragma varargck type \"lH\" void*",
    "#pragma varargck type \"P\" Point",
    "#pragma varargck type \"R\" Rectangle",
    "#pragma varargck type \"E\" uchar*",
    "#pragma varargck type \"I\" uchar*",
    "#pragma varargck type \"M\" uchar*",
    "#pragma varargck type \"V\" uchar*",
];

/// One verb: the verb with its flags, the declaration of an argument of a type that a line gives, and a wrong argument with the text of its type in kencc.
/// A verb that takes no argument has no declaration, and its wrong case gives one argument too many.
struct Verb {
    /// The verb with its flags, as it follows `%`.
    verb: &'static str,
    /// The name of the verb in the axes, since a verb such as `%` or `<` cannot be in a file name.
    name: &'static str,
    /// The statement that declares `a`, an argument of a type that the lines give.
    good: Option<&'static str>,
    /// An argument of a type that no line gives the verb, and the text of the type.
    bad: (&'static str, &'static str),
}

/// A verb whose argument is an integer.
const fn int(verb: &'static str, name: &'static str, good: &'static str) -> Verb {
    Verb { verb, name, good: Some(good), bad: ("1.5", "DOUBLE") }
}

/// A verb whose argument is a pointer or a record.
const fn other(verb: &'static str, name: &'static str, good: &'static str) -> Verb {
    Verb { verb, name, good: Some(good), bad: ("1", "INT") }
}

/// The verbs, each with one argument of a type that its lines give.
const VERBS: &[Verb] = &[
    int("lld", "lld", "vlong a = -3;"),
    int("llo", "llo", "uvlong a = 3;"),
    int("llx", "llx", "vlong a = 3;"),
    int("llb", "llb", "uvlong a = 3;"),
    int("ld", "ld", "long a = -3;"),
    int("lo", "lo", "ulong a = 3;"),
    int("lx", "lx", "long a = 3;"),
    int("lb", "lb", "ulong a = 3;"),
    int("zd", "zd", "intptr a = -3;"),
    int("zo", "zo", "uintptr a = 3;"),
    int("zx", "zx", "intptr a = 3;"),
    int("zb", "zb", "uintptr a = 3;"),
    int("d", "d", "int a = -3;"),
    int("o", "o", "uint a = 3;"),
    int("x", "x", "int a = 3;"),
    int("c", "c", "int a = 'c';"),
    int("C", "rune", "uint a = 0x263a;"),
    int("b", "b", "uint a = 5;"),
    other("f", "f", "double a = 1.5;"),
    other("e", "e", "double a = 1.5;"),
    other("g", "g", "double a = 1.5;"),
    other("s", "s", "char *a = \"s\";"),
    other("q", "q", "char *a = \"q\";"),
    Verb {
        verb: "S",
        name: "rune-string",
        good: Some("Rune a[2] = { 'S', 0 };"),
        bad: ("\"S\"", "IND CHAR"),
    },
    Verb {
        verb: "Q",
        name: "rune-quoted",
        good: Some("Rune a[2] = { 'Q', 0 };"),
        bad: ("\"Q\"", "IND CHAR"),
    },
    other("n", "n", "int b; int *a = &b;"),
    int("p", "p", "void *a = 0;"),
    other("<", "less", "void *a = 0;"),
    other("[", "bracket", "void *a = 0;"),
    other("H", "hex", "void *a = 0;"),
    other("lH", "long-hex", "void *a = 0;"),
    other("P", "point", "Point a = { 1, 2 };"),
    other("R", "rectangle", "Rectangle a = { { 1, 2 }, { 3, 4 } };"),
    other("E", "ether", "uchar a[6];"),
    other("I", "ip", "uchar a[16];"),
    other("M", "mask", "uchar a[16];"),
    other("V", "v4", "uchar a[4];"),
    Verb { verb: "r", name: "error", good: None, bad: ("1", "INT") },
    Verb { verb: "%", name: "percent", good: None, bad: ("1", "INT") },
];

/// A shape that is not one verb: the statements in `main`, the call, and the text of the message, or nothing when the call must compile.
struct Shape {
    /// The name of the shape in the axes.
    name: &'static str,
    /// The statements before the call.
    body: &'static [&'static str],
    /// The call.
    call: &'static str,
    /// The text of the message.
    message: Option<&'static str>,
}

/// The shapes that are not one verb.
const SHAPES: &[Shape] = &[
    Shape {
        name: "promotion",
        body: &[
            "char c = 'a';",
            "short s = -1;",
            "uchar uc = 2;",
            "ushort us = 3;",
            "float f = 1.5;",
        ],
        call: "print(\"%d %d %d %d %f\", c, s, uc, us, f)",
        message: None,
    },
    Shape {
        name: "size",
        body: &["char buf[8];", "buf[0] = 0;"],
        call: "print(\"%d %.*s\", sizeof buf, sizeof buf - 1, buf)",
        message: None,
    },
    Shape {
        name: "flags",
        body: &["int i = 5;", "long l = 6;"],
        call: "print(\"%,d % d %hd %-5d %+d %#x %08ld %5.2f\", i, i, i, i, i, i, l, 1.5)",
        message: None,
    },
    Shape {
        name: "format-in-a-variable",
        body: &["char *f = \"%d\";"],
        call: "print(f, 1.5)",
        message: None,
    },
    Shape {
        name: "second-argument",
        body: &[],
        call: "fprint(1, \"%d\", 1.5)",
        message: Some("format mismatch d DOUBLE, arg 3"),
    },
    Shape {
        name: "star",
        body: &[],
        call: "print(\"%.*s\", 1.5, \"x\")",
        message: Some("format mismatch '*' in .*s DOUBLE, arg 2"),
    },
    Shape {
        name: "unknown-verb",
        body: &[],
        call: "print(\"%Z\", 1)",
        message: Some("format mismatch Z INT, arg 2"),
    },
    Shape {
        name: "too-few-arguments",
        body: &[],
        call: "print(\"%d %d\", 1)",
        message: Some("more format than arguments d"),
    },
    Shape {
        name: "too-many-arguments",
        body: &[],
        call: "print(\"%d\", 1, 2)",
        message: Some("more arguments than format INT"),
    },
    Shape {
        name: "pointer-for-int",
        body: &["int b = 0;"],
        call: "print(\"%d\", &b)",
        message: Some("format mismatch d IND INT, arg 2"),
    },
    Shape {
        name: "const-string-for-int",
        body: &["const char *s = \"s\";"],
        call: "print(\"%d\", s)",
        message: Some("format mismatch d IND CONST CHAR, arg 2"),
    },
];

/// The flags of every case.
fn flags() -> Vec<String> {
    ["-fkencc", "-fkencc-format", "-Werror"].iter().map(|flag| (*flag).to_owned()).collect()
}

/// A case with `body` and `call` in `main`, which must print what `print` gives or be rejected with `message`.
fn case(name: &str, purpose: &str, body: &[&str], call: &str, message: Option<&str>) -> Case {
    let mut program = Program::new(purpose);
    for line in TYPES.iter().chain(FUNCTIONS).chain(LINES) {
        program.top(*line);
    }
    for line in body {
        program.line(*line);
    }
    program.check(Ty::I32, call, 37);
    let (source, expected) = program.finish();
    let expect = match message {
        Some(text) => Expect::Rejected(text.to_owned()),
        None => Expect::Output(expected),
    };
    Case::linked(
        Facet::KenccVarargck,
        Axes::of([("shape", name), ("dialect", "kencc")]),
        Dialect::C17,
        source,
        Vec::new(),
        flags(),
        expect,
    )
    .tagged(&["gnu", RUCC_DIALECT])
}

pub(crate) fn generate(sink: &mut Sink<'_>) {
    for verb in VERBS {
        if !sink.wants(Facet::KenccVarargck) {
            return;
        }
        let format = verb.verb;
        let (good, bad) = match verb.good {
            Some(decl) => (
                (vec![decl], format!("print(\"%{format}\", a)")),
                format!("print(\"%{format}\", {})", verb.bad.0),
            ),
            None => (
                (Vec::new(), format!("print(\"%{format}\")")),
                format!("print(\"%{format}\", {})", verb.bad.0),
            ),
        };
        let purpose =
            format!("the verb %{} with an argument of a type that its lines give", verb.verb);
        sink.push_case(case(&format!("{}-right", verb.name), &purpose, &good.0, &good.1, None));
        let message = match verb.good {
            Some(_) => format!("format mismatch {} {}, arg 2", verb.verb, verb.bad.1),
            None => format!("more arguments than format {}", verb.bad.1),
        };
        let purpose =
            format!("the verb %{} with an argument of a type that no line gives", verb.verb);
        sink.push_case(case(&format!("{}-wrong", verb.name), &purpose, &[], &bad, Some(&message)));
    }
    for shape in SHAPES {
        if !sink.wants(Facet::KenccVarargck) {
            return;
        }
        let purpose = format!("the format check of kencc for the {} shape", shape.name);
        sink.push_case(case(shape.name, &purpose, shape.body, shape.call, shape.message));
    }
}

#[cfg(test)]
mod tests {
    use super::{LINES, VERBS};

    #[test]
    fn each_verb_has_a_type_line() {
        for verb in VERBS {
            let line = format!("\"{}\"", verb.verb);
            assert!(LINES.iter().any(|known| known.contains(&line)), "{}", verb.verb);
        }
    }

    #[test]
    fn the_names_are_distinct() {
        for (at, verb) in VERBS.iter().enumerate() {
            assert!(VERBS[at + 1..].iter().all(|other| other.name != verb.name), "{}", verb.name);
        }
    }
}
