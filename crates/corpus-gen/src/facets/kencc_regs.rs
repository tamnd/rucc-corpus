//! The `extern register` variables of kencc, which the Plan 9 kernel keeps `m` and `up` in.
//!
//! `6c` gives the first `extern register` of a unit `R15` and the next `R14`, and `7c` gives `R27` and `R26`, as `cc/dcl.c` and the `exregoffset` of each compiler do.
//! The code generator of kencc never gives these registers to a value, and no prologue saves them, so a write in one function stays after it returns.
//! The kernel depends on this in each file that includes `dat.h`, which `port/portdat.h` cites.
//! Each case writes the registers, calls a function through a pointer so that the call stays, and prints what the registers hold after it.
//! A shape has a recursion, a function with more live values than the allocator has registers, and a `longjmp` out of a call.
//! The `longjmp` case does not write the register between `setjmp` and `longjmp`, because the C library of the host puts back a register that a callee saves, and Plan 9 does not.
//! A `notify` handler of Plan 9 has no case here, because the host has no such call, and the guest checks it.
//! The cases are built under `-fkencc`, where x86-64 and AArch64 have the registers of kencc on each OS.
//! The answers were compared with `6c` on 9front amd64.
//! Only rucc has the dialect, so each case has the tag `rucc-dialect`.

use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, RUCC_DIALECT};

/// One shape: the declarations, the statements that set the registers, and each expression with
/// its value.
struct Shape {
    /// The name of the shape in the axes.
    name: &'static str,
    /// The declarations and the functions, at file scope.
    top: &'static [&'static str],
    /// The statements in `main` before the checks.
    body: &'static [&'static str],
    /// Each expression and its value, in the order the case prints them.
    checks: &'static [(&'static str, i128)],
}

/// The shapes of `kencc-regs`.
const SHAPES: &[Shape] = &[
    Shape {
        name: "across-calls",
        top: &[
            "typedef struct Mach { int machno; int ticks; } Mach;",
            "extern register Mach *m;",
            "Mach machs[2];",
            "static int tick(void) { m->ticks = m->ticks + 1; return m->machno; }",
            "int (*ticks[1])(void) = { tick };",
        ],
        body: &[
            "int a, b;",
            "m = &machs[1];",
            "m->machno = 5;",
            "a = ticks[0]();",
            "b = ticks[0]();",
        ],
        checks: &[
            ("a + b", 10),
            ("m == &machs[1]", 1),
            ("m->ticks", 2),
            ("machs[0].ticks", 0),
            ("m->machno", 5),
        ],
    },
    Shape {
        name: "write-in-callee",
        top: &[
            "extern register int level;",
            "static void bump(void) { level = level + 1; }",
            "void (*bumps[1])(void) = { bump };",
            "static int twice(void) { bumps[0](); bumps[0](); return level; }",
        ],
        body: &["level = 40;"],
        checks: &[("twice()", 42), ("level", 42), ("twice() + level", 88)],
    },
    Shape {
        name: "recursion",
        top: &[
            "extern register int depth;",
            "static int down(int n) {",
            "    int r;",
            "    depth = depth + 1;",
            "    if (n == 0)",
            "        return depth;",
            "    r = down(n - 1);",
            "    depth = depth + 100;",
            "    return r;",
            "}",
        ],
        body: &["depth = 0;"],
        checks: &[("down(4)", 5), ("depth", 405)],
    },
    Shape {
        name: "two-registers",
        top: &[
            "typedef struct Proc { int pid; } Proc;",
            "extern register Proc *up;",
            "extern register int cpu;",
            "Proc procs[3];",
            "static int swap(int i) { up = &procs[i]; cpu = i * 10; return up->pid; }",
            "int (*swaps[1])(int) = { swap };",
        ],
        body: &[
            "procs[0].pid = 7;",
            "procs[1].pid = 8;",
            "procs[2].pid = 9;",
            "up = &procs[0];",
            "cpu = 1;",
        ],
        checks: &[
            ("up->pid + cpu", 8),
            ("swaps[0](2)", 9),
            ("up->pid * 100 + cpu", 920),
            ("up - procs", 2),
        ],
    },
    Shape {
        name: "many-live-values",
        top: &[
            "extern register int keep;",
            "static int same(int x) { return x; }",
            "int (*sames[1])(int) = { same };",
            "static int busy(int n) {",
            "    int a = n, b = n * 2, c = n * 3, d = n * 5, e = n * 7, f = n * 11, g = n * 13;",
            "    int h = n * 17, i = n * 19, j = n * 23, k = n * 29, l = n * 31, o = n * 37;",
            "    int p = n * 41, q = n * 43;",
            "    n = sames[0](n);",
            "    return a + b + c + d + e + f + g + h + i + j + k + l + o + p + q + n + keep;",
            "}",
        ],
        body: &["keep = 1000;"],
        checks: &[("busy(1)", 1283), ("keep", 1000), ("busy(2)", 1566)],
    },
    Shape {
        name: "longjmp",
        top: &[
            "typedef long long Jmp[64];",
            "int setjmp(long long *);",
            "void longjmp(long long *, int);",
            "extern register int mark;",
            "Jmp env;",
            "static void leave(int v) { if (mark == 77) longjmp(env, v); longjmp(env, 1); }",
            "void (*leaves[1])(int) = { leave };",
        ],
        body: &[
            "int got;",
            "mark = 77;",
            "got = setjmp(env);",
            "if (got == 0)",
            "    leaves[0](3);",
        ],
        checks: &[("got", 3), ("mark", 77), ("mark + got", 80)],
    },
];

pub(crate) fn generate(sink: &mut Sink<'_>) {
    for shape in SHAPES {
        if !sink.wants(Facet::KenccRegs) {
            return;
        }
        let mut program =
            Program::new(format!("the extern register of kencc, {}", shape.name.replace('-', " ")));
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
        let case = Case::linked(
            Facet::KenccRegs,
            Axes::of([("shape", shape.name), ("dialect", "kencc")]),
            Dialect::C17,
            source,
            Vec::new(),
            vec!["-fkencc".to_owned()],
            Expect::Output(expected),
        )
        .tagged(&["gnu", RUCC_DIALECT]);
        sink.push_case(case);
    }
}
