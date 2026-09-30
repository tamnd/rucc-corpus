//! An executable and the modules it loads with `dlopen`, which call back into it.
//!
//! Every Postgres extension and procedural language is one of these. The server opens the
//! module, looks up `_PG_init` in it and calls it, and from then on the two call each other.
//! The module calls `palloc`, `elog` and hundreds more that are defined in the executable and
//! left undefined in the module, it reads and writes GUC variables that live in the executable,
//! and it saves the hook the executable had and puts its own in its place. The executable calls
//! the hooks and callbacks the module handed it, and every module it loads defines `_PG_init`
//! under the same name.
//!
//! None of that is an optimization. It is the code on both sides of a boundary that the
//! compiler never sees both sides of at once: a module built as position independent code that
//! reaches data and functions in the executable through its own tables, its own globals through
//! the same tables or around them, variadic and wide calls in both directions, pointers to
//! functions in the other image stored in data and called through, constructors that run while
//! `dlopen` is still loading the module, and thread local storage that lives in the other image.
//!
//! # How a case is built
//!
//! The file with `main` in it is the executable. Every other file is a module, a
//! [`corpus_model::UnitKind::Module`], built on its own into its name with `.so` on the end,
//! and the executable opens it by that path from the directory it runs in. The harness knows
//! the link for each target: `-rdynamic` for the executable and `-shared -fPIC` for the module
//! on ELF, and `-bundle -bundle_loader` naming the executable on Mach-O. Windows builds a module
//! against an import library for the executable instead, which is not written yet, so every
//! case here carries the `dlopen` tag and a run there leaves them out.
//!
//! CI builds these on x86-64 Linux, with GCC 16 and with rucc, and there is no macOS job to build
//! them with clang. When there is one, its rucc half waits on tamnd/rucc#2010, since rucc's
//! Darwin driver does not take `-bundle` or `-bundle_loader` yet.
//!
//! Every case is generated three times over, with its constants, its counts and how many
//! modules it loads moved each time, and every answer is worked out here before any compiler is
//! asked. Nothing printed is an address. Two handles compared for equality is the nearest the
//! corpus comes, and that prints one or nought.

use super::PROVENANCE;
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, Unit};
use std::fmt::Write as _;

/// The tag on every case that loads a module, which a run on Windows leaves out.
pub(super) const DLOPEN: &str = "dlopen";

/// How many times each shape is generated.
const VARIANTS: usize = 3;

/// Emits the facet.
pub(super) fn generate(sink: &mut Sink<'_>) {
    for variant in 0..VARIANTS {
        if !sink.wants(Facet::Bundle) {
            return;
        }
        call_exe(sink, variant);
        elog_varargs(sink, variant);
        exe_globals(sink, variant);
        hook_chain(sink, variant);
        callbacks(sink, variant);
        dlsym_calls(sink, variant);
        constructor(sink, variant);
        module_statics(sink, variant);
        same_names(sink, variant);
        thread_local(sink, variant);
        guc(sink, variant);
    }
}

/// The executable half of every case, with the two helpers that open a module and find a name.
///
/// A module that will not open, or a name that is not there, prints what the loader said, which
/// is not an answer the case expects, so the case fails with the reason in its output rather
/// than with a crash somewhere after it.
fn executable(purpose: String) -> Program {
    let mut program = Program::new(purpose);
    program.include("dlfcn.h");
    program.top("static void *load(const char *path)");
    program.top("{");
    program.top("    void *handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);");
    program.top("    if (!handle) {");
    program.top("        printf(\"dlopen %s: %s\\n\", path, dlerror());");
    program.top("    }");
    program.top("    return handle;");
    program.top("}");
    program.top("");
    program.top("static void *find(void *handle, const char *name)");
    program.top("{");
    program.top("    void *found = dlsym(handle, name);");
    program.top("    if (!found) {");
    program.top("        printf(\"dlsym %s: %s\\n\", name, dlerror());");
    program.top("    }");
    program.top("    return found;");
    program.top("}");
    program.top("");
    program
}

/// Opens a module into a local of the same name, and stops the program if it would not open.
fn open(program: &mut Program, name: &str) {
    program.line(format!("void *{name} = load(\"./{name}.so\");"));
    program.line(format!("if (!{name}) {{"));
    program.line_at(1, "return 1;");
    program.line("}");
}

/// One module, written the way the generated units are.
fn module(name: &str, purpose: &str, body: &str) -> Unit {
    Unit::module(
        name,
        format!(
            "// {purpose}\n// Generated by rucc-corpus. Edit the generator, not this file.\n\n{body}"
        ),
    )
}

/// Emits one case.
fn push(sink: &mut Sink<'_>, shape: &str, variant: usize, program: Program, units: Vec<Unit>) {
    let (source, expected) = program.finish();
    let case = Case::linked(
        Facet::Bundle,
        Axes::of([("shape", shape), ("variant", &format!("v{variant}"))]),
        Dialect::C17,
        source,
        units,
        Vec::new(),
        Expect::Output(expected),
    )
    .tagged(&["gnu", "headers", DLOPEN, PROVENANCE]);
    sink.push_case(case);
}

/// The module calls functions the executable defines, of every shape a call takes.
///
/// Integers of every width, a `double` and a `float`, a small struct that comes back in two
/// registers and a larger one that comes back through memory the caller passed, a pointer into
/// the module's own frame, and ten arguments so the last of them go on the stack. Postgres
/// calls `palloc`, `pstrdup`, `hash_bytes` and the rest this way from every module it has.
fn call_exe(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let seed = 3 + 5 * v;
    let k = v + 2;
    let counts = [v + 2, 2 * v + 5];

    let model = |n: i64| -> i64 {
        let mut acc = 0i64;
        for i in 0..n {
            acc += i + seed;
            acc += (i + 1) * (i + 2) * k;
            acc += 4 * i + 2;
            acc += i64::from(((i + seed) as u32).wrapping_mul(37) as u8);
            acc += i * 3 - 1000;
            acc += i + (i + seed) * 2;
            acc += (i - 2) + (i - 1) + (i - 2) * 3;
            acc += i;
            acc += (0..10).map(|j| (j + 1) * (i + j)).sum::<i64>();
            acc += 2 * i + 1;
        }
        acc
    };

    let body = format!(
        "\
struct pair {{
    int lo;
    long long hi;
}};

struct wide {{
    long long a;
    long long b;
    long long c;
}};

int exe_add(int, int);
long long exe_mul3(long long, long long, long long);
double exe_scale(double, int);
unsigned char exe_byte(unsigned);
short exe_short(int);
struct pair exe_pair(int, long long);
struct wide exe_wide(long long);
void exe_store(long long *, int);
long long exe_many(int, int, int, int, int, int, int, int, long long, long long);
float exe_half(float);

long long mod_run(int n)
{{
    long long acc = 0;
    for (int i = 0; i < n; i++) {{
        acc += exe_add(i, {seed});
        acc += exe_mul3(i + 1, i + 2, {k});
        acc += (long long)exe_scale(i + 0.5, 4);
        acc += exe_byte((unsigned)(i + {seed}));
        acc += exe_short(i * 3);
        struct pair p = exe_pair(i, i + {seed});
        acc += p.lo + p.hi;
        struct wide w = exe_wide(i - 2);
        acc += w.a + w.b + w.c;
        exe_store(&acc, i);
        acc += exe_many(i, i + 1, i + 2, i + 3, i + 4, i + 5, i + 6, i + 7, i + 8, i + 9);
        acc += (long long)(exe_half(i * 2.0f + 1.0f) * 2.0f);
    }}
    return acc;
}}
"
    );
    let m0 = module("m0", "Calls every function it uses back in the executable.", &body);

    let mut program = executable(format!(
        "A module calling back into the executable with every shape of call, seed {seed}"
    ));
    for line in [
        "struct pair {",
        "    int lo;",
        "    long long hi;",
        "};",
        "",
        "struct wide {",
        "    long long a;",
        "    long long b;",
        "    long long c;",
        "};",
        "",
        "int exe_add(int a, int b) { return a + b; }",
        "long long exe_mul3(long long a, long long b, long long c) { return a * b * c; }",
        "double exe_scale(double x, int k) { return x * k; }",
        "unsigned char exe_byte(unsigned v) { return (unsigned char)(v * 37u); }",
        "short exe_short(int v) { return (short)(v - 1000); }",
        "struct pair exe_pair(int lo, long long hi) { struct pair p = { lo, hi * 2 }; return p; }",
        "struct wide exe_wide(long long x) { struct wide w = { x, x + 1, x * 3 }; return w; }",
        "void exe_store(long long *out, int v) { *out += v; }",
        "float exe_half(float x) { return x / 2.0f; }",
        "",
        "long long exe_many(int a, int b, int c, int d, int e, int f, int g, int h, long long i,",
        "                   long long j)",
        "{",
        "    return a + 2LL * b + 3LL * c + 4LL * d + 5LL * e + 6LL * f + 7LL * g + 8LL * h +",
        "           9 * i + 10 * j;",
        "}",
    ] {
        program.top(line);
    }
    open(&mut program, "m0");
    program.line("long long (*run)(int) = (long long (*)(int))find(m0, \"mod_run\");");
    for (at, &n) in counts.iter().enumerate() {
        let name = format!("n{at}");
        program.input(Ty::I32, &name, i128::from(n));
        program.check(Ty::I64, &format!("run({name})"), i128::from(model(n)));
    }
    push(sink, "call-exe", variant, program, vec![m0]);
}

/// One argument to an `elog` call.
#[derive(Clone)]
enum Arg {
    Int(i64),
    Long(i64),
    Unsigned(u32),
    Double(f64),
    Text(&'static str),
    Char(u8),
}

impl Arg {
    /// The conversion `elog` reads it with.
    const fn spec(&self) -> &'static str {
        match self {
            Self::Int(_) => "%d",
            Self::Long(_) => "%L",
            Self::Unsigned(_) => "%u",
            Self::Double(_) => "%f",
            Self::Text(_) => "%s",
            Self::Char(_) => "%c",
        }
    }

    /// The argument as it is written in the call.
    fn literal(&self) -> String {
        match self {
            Self::Int(x) => x.to_string(),
            Self::Long(x) => format!("{x}LL"),
            Self::Unsigned(x) => format!("{x}u"),
            Self::Double(x) => format!("{x:?}"),
            Self::Text(s) => format!("\"{s}\""),
            Self::Char(c) => format!("'{}'", *c as char),
        }
    }

    /// What `elog` folds into its hash for it, in order.
    fn folds(&self) -> Vec<u64> {
        match self {
            Self::Int(x) | Self::Long(x) => vec![*x as u64],
            Self::Unsigned(x) => vec![u64::from(*x)],
            Self::Double(x) => vec![(*x * 8.0) as i64 as u64],
            Self::Text(s) => s.bytes().map(u64::from).collect(),
            Self::Char(c) => vec![u64::from(*c)],
        }
    }
}

/// The FNV-1a step `elog` folds everything it is handed through.
const fn mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(1_099_511_628_211)
}

/// The module reports through a variadic function in the executable, and through one of its own.
///
/// `elog` and `ereport` are variadic and live in the server, so every message a module raises
/// is a variadic call across the boundary, with ints, `long long`, `double` and strings mixed
/// and as many as a dozen of them, enough that the last go on the stack. Half the calls here go
/// straight to `elog`. The other half go through a variadic function in the module that starts
/// its own `va_list` and hands it to `elog_valist` in the executable, which is how `errmsg`
/// hands its arguments on to the formatting code.
fn elog_varargs(sink: &mut Sink<'_>, variant: usize) {
    const LEVELS: [i64; 5] = [10, 15, 17, 19, 21];
    const WORDS: [&str; 5] = ["heap", "index", "toast", "vacuum", "wal"];
    let v = variant as i64;
    let messages = 6 + 2 * variant;

    let mut calls = String::new();
    let mut hash = 1_469_598_103_934_665_603u64;
    let mut errors = 0i64;
    for j in 0..messages {
        let jj = j as i64;
        let level = LEVELS[(j + variant) % LEVELS.len()];
        let count = (j * 3 + variant) % 13;
        let args: Vec<Arg> = (0..count)
            .map(|a| {
                let aa = a as i64;
                match (j + a) % 6 {
                    0 => Arg::Int(jj * 7 + aa * 3 + v - 20),
                    1 => Arg::Long((jj + 1) * 1_000_000_007 * (aa + 1) - 5_000_000_000),
                    2 => Arg::Unsigned(4_000_000_000 - (j * 1000 + a) as u32),
                    3 => Arg::Double((jj + aa) as f64 * 0.375 - 2.0),
                    4 => Arg::Text(WORDS[(j + a + variant) % WORDS.len()]),
                    _ => Arg::Char(b'a' + ((j + a + variant) % 26) as u8),
                }
            })
            .collect();
        let mut fmt = format!("m{j}");
        for arg in &args {
            fmt.push(' ');
            fmt.push_str(arg.spec());
        }
        let through = if j % 2 == 0 { "elog" } else { "mylog" };
        let mut call = format!("    {through}({level}, \"{fmt}\"");
        for arg in &args {
            call.push_str(", ");
            call.push_str(&arg.literal());
        }
        call.push_str(");\n");
        calls.push_str(&call);

        if level >= 21 {
            errors += 1;
        }
        hash = mix(hash, level as u64);
        let mut next = args.iter();
        let mut chars = fmt.bytes();
        while let Some(c) = chars.next() {
            if c == b'%' {
                chars.next();
                if let Some(arg) = next.next() {
                    for value in arg.folds() {
                        hash = mix(hash, value);
                    }
                }
            } else {
                hash = mix(hash, u64::from(c));
            }
        }
    }

    let body = format!(
        "\
#include <stdarg.h>

void elog(int, const char *, ...);
void elog_valist(int, const char *, va_list);

static void mylog(int level, const char *fmt, ...)
{{
    va_list ap;
    va_start(ap, fmt);
    elog_valist(level, fmt, ap);
    va_end(ap);
}}

void mod_run(void)
{{
{calls}}}
"
    );
    let m0 = module("m0", "Raises messages through the executable's variadic elog.", &body);

    let mut program =
        executable(format!("A module raising {messages} messages through a variadic elog"));
    program.include("stdarg.h");
    for line in [
        "static unsigned long long elog_hash = 1469598103934665603ULL;",
        "static int elog_count;",
        "static int elog_errors;",
        "",
        "static void mix(unsigned long long v)",
        "{",
        "    elog_hash = (elog_hash ^ v) * 1099511628211ULL;",
        "}",
        "",
        "void elog_valist(int level, const char *fmt, va_list ap)",
        "{",
        "    elog_count++;",
        "    if (level >= 21) {",
        "        elog_errors++;",
        "    }",
        "    mix((unsigned long long)level);",
        "    for (const char *p = fmt; *p; p++) {",
        "        if (*p != '%') {",
        "            mix((unsigned char)*p);",
        "            continue;",
        "        }",
        "        p++;",
        "        switch (*p) {",
        "        case 'd':",
        "            mix((unsigned long long)(long long)va_arg(ap, int));",
        "            break;",
        "        case 'L':",
        "            mix((unsigned long long)va_arg(ap, long long));",
        "            break;",
        "        case 'u':",
        "            mix((unsigned long long)va_arg(ap, unsigned));",
        "            break;",
        "        case 'f':",
        "            mix((unsigned long long)(long long)(va_arg(ap, double) * 8.0));",
        "            break;",
        "        case 's':",
        "            for (const char *s = va_arg(ap, const char *); *s; s++) {",
        "                mix((unsigned char)*s);",
        "            }",
        "            break;",
        "        case 'c':",
        "            mix((unsigned long long)va_arg(ap, int));",
        "            break;",
        "        }",
        "    }",
        "}",
        "",
        "void elog(int level, const char *fmt, ...)",
        "{",
        "    va_list ap;",
        "    va_start(ap, fmt);",
        "    elog_valist(level, fmt, ap);",
        "    va_end(ap);",
        "}",
    ] {
        program.top(line);
    }
    open(&mut program, "m0");
    program.line("void (*run)(void) = (void (*)(void))find(m0, \"mod_run\");");
    program.line("run();");
    program.check(Ty::I32, "elog_count", messages as i128);
    program.check(Ty::I32, "elog_errors", i128::from(errors));
    program.check(Ty::U64, "elog_hash", i128::from(hash));
    push(sink, "elog-varargs", variant, program, vec![m0]);
}

/// The module reads and writes globals the executable defines.
///
/// An `int` like `NBuffers`, a `long long`, a struct like the one behind a GUC group, an array
/// and a table of strings, all defined in the executable and reached from the module through
/// its global offset table. The executable reads them back after each call, so a value either
/// side kept in a register across the call is caught.
fn exe_globals(sink: &mut Sink<'_>, variant: usize) {
    const NAMES: [&str; 3] = ["alpha", "beta", "gamma"];
    let v = variant as i64;
    let n = 4 + variant;
    let mut nbuffers = 64 + v;
    let mut counter = 1000 * (v + 1);
    let a = 7 + v;
    let mut b = 100_000 + v;
    let mut c = (200 + v * 20) as u8;
    let d = -30 - v;
    let e = 2.25 + v as f64;
    let mut table: Vec<i64> = (0..n as i64).map(|i| i * i + v + 1).collect();
    let rounds = [2i64, 3];

    let initial_table = table.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
    let initial = (nbuffers, counter, b, c);

    let mut results = Vec::new();
    for &count in &rounds {
        let mut acc = 0i64;
        for r in 0..count {
            acc += nbuffers * (r + 1);
            for cell in &mut table {
                acc += *cell;
                *cell = *cell * 2 + r;
            }
            acc += a + b + i64::from(c) + d + (e * 4.0) as i64;
            b += r + 1;
            c = c.wrapping_add(7);
            for name in NAMES {
                acc += i64::from(name.as_bytes()[r as usize % 4]);
            }
            counter += acc % 1000;
        }
        nbuffers += 1;
        results.push(acc);
    }

    let body = format!(
        "\
struct config {{
    int a;
    long long b;
    unsigned char c;
    short d;
    double e;
}};

extern int NBuffers;
extern long long counter;
extern struct config Config;
extern int table[{n}];
extern const char *const names[3];

long long mod_run(int rounds)
{{
    long long acc = 0;
    for (int r = 0; r < rounds; r++) {{
        acc += (long long)NBuffers * (r + 1);
        for (int i = 0; i < {n}; i++) {{
            acc += table[i];
            table[i] = table[i] * 2 + r;
        }}
        acc += Config.a + Config.b + Config.c + Config.d + (long long)(Config.e * 4);
        Config.b += r + 1;
        Config.c = (unsigned char)(Config.c + 7);
        for (int k = 0; k < 3; k++) {{
            acc += (unsigned char)names[k][r % 4];
        }}
        counter += acc % 1000;
    }}
    NBuffers += 1;
    return acc;
}}
"
    );
    let m0 = module("m0", "Reads and writes globals the executable defines.", &body);

    let mut program =
        executable(format!("A module reading and writing {n} kinds of executable global"));
    program.top("struct config {");
    program.top("    int a;");
    program.top("    long long b;");
    program.top("    unsigned char c;");
    program.top("    short d;");
    program.top("    double e;");
    program.top("};");
    program.top("");
    program.top(format!("int NBuffers = {};", initial.0));
    program.top(format!("long long counter = {};", initial.1));
    program.top(format!(
        "struct config Config = {{ {a}, {}, {}, {d}, {e:?} }};",
        initial.2, initial.3
    ));
    program.top(format!("int table[{n}] = {{ {initial_table} }};"));
    program.top("const char *const names[3] = { \"alpha\", \"beta\", \"gamma\" };");
    program.top("");
    open(&mut program, "m0");
    program.line("long long (*run)(int) = (long long (*)(int))find(m0, \"mod_run\");");
    for (at, (&count, &result)) in rounds.iter().zip(&results).enumerate() {
        let name = format!("r{at}");
        program.input(Ty::I32, &name, i128::from(count));
        program.check(Ty::I64, &format!("run({name})"), i128::from(result));
    }
    program.check(Ty::I64, "counter", i128::from(counter));
    program.check(Ty::I32, "NBuffers", i128::from(nbuffers));
    program.line("long long sum = 0;");
    program.line(format!("for (int i = 0; i < {n}; i++) {{"));
    program.line_at(1, "sum += table[i];");
    program.line("}");
    program.check(Ty::I64, "sum", i128::from(table.iter().sum::<i64>()));
    program.check(Ty::I64, "Config.b", i128::from(b));
    program.check(Ty::U8, "Config.c", i128::from(c));
    push(sink, "exe-globals", variant, program, vec![m0]);
}

/// Each module saves the hook the executable had and installs its own, which calls the saved one.
///
/// `planner_hook`, `ExecutorStart_hook` and the rest are function pointers in the server, and
/// an extension's `_PG_init` chains onto them. With more than one module loaded the last one
/// installed runs first and calls down through the others to the standard function, which is in
/// the executable. Every module defines `_PG_init`, `mod_calls` and the same statics, so a
/// name found in the wrong module is caught as well.
fn hook_chain(sink: &mut Sink<'_>, variant: usize) {
    let modules = variant + 1;
    let queries = [1i64, 4 + variant as i64, 9];
    let scale = |m: usize| m as i64 + 2;
    let offset = |m: usize| 10 * m as i64 + 1;
    let plan =
        |q: i64, installed: usize| (0..installed).fold(q * 3 + 1, |r, m| r * scale(m) + offset(m));

    let mut units = Vec::new();
    for m in 0..modules {
        let body = format!(
            "\
typedef long long (*planner_hook_type)(int);

extern planner_hook_type planner_hook;
long long standard_planner(int);

static planner_hook_type prev_planner_hook;
static int calls;

static long long my_planner(int q)
{{
    calls++;
    long long r = prev_planner_hook ? prev_planner_hook(q) : standard_planner(q);
    return r * {} + {};
}}

void _PG_init(void)
{{
    prev_planner_hook = planner_hook;
    planner_hook = my_planner;
}}

int mod_calls(void)
{{
    return calls;
}}
",
            scale(m),
            offset(m)
        );
        units.push(module(
            &format!("m{m}"),
            "Chains a planner hook onto the one before it.",
            &body,
        ));
    }

    let mut program = executable(format!("{modules} modules chaining onto one planner hook"));
    program.top("typedef long long (*planner_hook_type)(int);");
    program.top("");
    program.top("planner_hook_type planner_hook;");
    program.top("");
    program.top("long long standard_planner(int q)");
    program.top("{");
    program.top("    return q * 3LL + 1;");
    program.top("}");
    program.top("");
    program.top("static long long plan(int q)");
    program.top("{");
    program.top("    return planner_hook ? planner_hook(q) : standard_planner(q);");
    program.top("}");
    program.top("");
    program.input(Ty::I32, "q", i128::from(queries[0]));
    program.check(Ty::I64, "plan(q)", i128::from(plan(queries[0], 0)));
    for m in 0..modules {
        let name = format!("m{m}");
        open(&mut program, &name);
        program
            .line(format!("void (*init{m})(void) = (void (*)(void))find({name}, \"_PG_init\");"));
        program.line(format!("init{m}();"));
        program.check(Ty::I64, "plan(q)", i128::from(plan(queries[0], m + 1)));
    }
    for &q in &queries {
        program.check(Ty::I64, &format!("plan({q})"), i128::from(plan(q, modules)));
    }
    for m in 0..modules {
        program.line(format!("int (*calls{m})(void) = (int (*)(void))find(m{m}, \"mod_calls\");"));
        let calls = (modules - m) + queries.len();
        program.check(Ty::I32, &format!("calls{m}()"), calls as i128);
    }
    push(sink, "hook-chain", variant, program, units);
}

/// Two modules register callbacks in a table in the executable, which calls them in order.
///
/// `RegisterXactCallback` and `on_proc_exit` keep a list of function pointers that modules
/// hand the server. Each module here registers its callbacks out of a constant table of its own
/// static functions, which is data holding the addresses of code in a position independent
/// image, and the executable sorts them by priority and calls each through the table.
fn callbacks(sink: &mut Sink<'_>, variant: usize) {
    struct Registered {
        index: i64,
        arg: i64,
        priority: i64,
        weight: i64,
    }
    let mut units = Vec::new();
    let mut table: Vec<Registered> = Vec::new();
    let mut counts = Vec::new();
    for m in 0..2usize {
        let count = 2 + variant + m;
        counts.push(count as i64);
        let weight = m as i64 * 3 + 2;
        let base = 10 * (m as i64 + 1);
        let mut body = String::from(
            "\
typedef long long (*callback_fn)(long long, int);

void RegisterCallback(callback_fn, long long, int);

static long long seen;
",
        );
        let mut priorities = Vec::new();
        for i in 0..count {
            let priority = ((i * 5 + m * 3 + variant) % 4) as i64;
            priorities.push(priority.to_string());
            let _ = write!(
                body,
                "\nstatic long long cb{i}(long long arg, int event)\n{{\n    seen += event;\n    return arg * {} + event * {weight};\n}}\n",
                i + 1
            );
            let at = table.iter().rposition(|r| r.priority <= priority).map_or(0, |p| p + 1);
            table
                .insert(at, Registered { index: i as i64, arg: base + i as i64, priority, weight });
        }
        let names = (0..count).map(|i| format!("cb{i}")).collect::<Vec<_>>().join(", ");
        let _ = write!(
            body,
            "\nstatic callback_fn const mine[{count}] = {{ {names} }};\nstatic const int priorities[{count}] = {{ {} }};\n\nvoid _PG_init(void)\n{{\n    for (int i = 0; i < {count}; i++) {{\n        RegisterCallback(mine[i], {base} + i, priorities[i]);\n    }}\n}}\n\nlong long mod_seen(void)\n{{\n    return seen;\n}}\n",
            priorities.join(", ")
        );
        units.push(module(
            &format!("m{m}"),
            "Registers callbacks in the executable's table.",
            &body,
        ));
    }

    let fire = |event: i64| {
        table.iter().fold(0i64, |acc, r| acc * 3 + r.arg * (r.index + 1) + event * r.weight)
    };

    let mut program = executable(format!(
        "Two modules registering {} callbacks in a table the executable calls",
        table.len()
    ));
    for line in [
        "typedef long long (*callback_fn)(long long, int);",
        "",
        "struct callback {",
        "    callback_fn fn;",
        "    long long arg;",
        "    int priority;",
        "};",
        "",
        "static struct callback callbacks[16];",
        "static int ncallbacks;",
        "",
        "void RegisterCallback(callback_fn fn, long long arg, int priority)",
        "{",
        "    int at = ncallbacks;",
        "    while (at > 0 && callbacks[at - 1].priority > priority) {",
        "        callbacks[at] = callbacks[at - 1];",
        "        at--;",
        "    }",
        "    callbacks[at].fn = fn;",
        "    callbacks[at].arg = arg;",
        "    callbacks[at].priority = priority;",
        "    ncallbacks++;",
        "}",
        "",
        "static long long fire(int event)",
        "{",
        "    long long acc = 0;",
        "    for (int i = 0; i < ncallbacks; i++) {",
        "        acc = acc * 3 + callbacks[i].fn(callbacks[i].arg, event);",
        "    }",
        "    return acc;",
        "}",
    ] {
        program.top(line);
    }
    for m in 0..2 {
        let name = format!("m{m}");
        open(&mut program, &name);
        program
            .line(format!("void (*init{m})(void) = (void (*)(void))find({name}, \"_PG_init\");"));
        program.line(format!("init{m}();"));
    }
    program.check(Ty::I32, "ncallbacks", table.len() as i128);
    for event in 1..=3 {
        program.check(Ty::I64, &format!("fire({event})"), i128::from(fire(event)));
    }
    for (m, count) in counts.iter().enumerate() {
        program.line(format!(
            "long long (*seen{m})(void) = (long long (*)(void))find(m{m}, \"mod_seen\");"
        ));
        program.check(Ty::I64, &format!("seen{m}()"), i128::from(6 * count));
    }
    push(sink, "callbacks", variant, program, units);
}

/// The executable finds functions and data in the module by name and uses them.
///
/// Every shape of call again, the other way round: a module function that takes a `short` and
/// an `unsigned char`, one that returns a `double`, one that returns a struct, and one that is
/// variadic, called through a pointer the executable got from `dlsym`. And the module's own
/// globals, written by the executable through the address `dlsym` handed back and read by the
/// module from its side.
fn dlsym_calls(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let start = 11 + v;
    let table: Vec<i64> = (0..4).map(|i| 100 * (i + 1) + v).collect();
    let set = 40 + v;
    let after_inc = set + 5;
    let (ma, mb, mc, md) = (1000 + v, -7 - v, 300i64, 200u8);
    let mixed = ma * 3 + mb - mc + i64::from(md);
    let average = 7 + 2 * v;
    let pair_x = 7 + v;
    let pair_hi = pair_x * pair_x + after_inc;
    let new_cell = 100 + v;
    let mut table_after = table.clone();
    table_after[2] = new_cell;
    let short_sum: i64 = (1..=5).map(|i| i * i).sum();
    let long_values: Vec<i64> = (0..9).map(|i| 1_000_000_000_000 + i * (v + 1)).collect();
    let long_sum: i64 = long_values.iter().enumerate().map(|(i, x)| x * (i as i64 + 1)).sum();

    let body = format!(
        "\
#include <stdarg.h>

struct pair {{
    int lo;
    long long hi;
}};

int mod_counter = {start};
long long mod_table[4] = {{ {} }};

int mod_inc(int by)
{{
    mod_counter += by;
    return mod_counter;
}}

long long mod_mix(long long a, int b, short c, unsigned char d)
{{
    return a * 3 + b - c + d;
}}

double mod_avg(double a, double b, double c)
{{
    return (a + b + c) / 4;
}}

struct pair mod_pair(int x)
{{
    struct pair p = {{ x * 2, (long long)x * x + mod_counter }};
    return p;
}}

long long mod_sum(int n, ...)
{{
    va_list ap;
    va_start(ap, n);
    long long s = 0;
    for (int i = 0; i < n; i++) {{
        s += va_arg(ap, long long) * (i + 1);
    }}
    va_end(ap);
    return s;
}}

long long mod_table_sum(void)
{{
    long long s = 0;
    for (int i = 0; i < 4; i++) {{
        s += mod_table[i];
    }}
    return s;
}}
",
        table.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
    );
    let m0 = module("m0", "Functions and data the executable finds by name.", &body);

    let mut program =
        executable(format!("The executable calling into a module found by dlsym, {start}"));
    program.top("struct pair {");
    program.top("    int lo;");
    program.top("    long long hi;");
    program.top("};");
    program.top("");
    open(&mut program, "m0");
    for line in [
        "int *counter = (int *)find(m0, \"mod_counter\");",
        "long long *table = (long long *)find(m0, \"mod_table\");",
        "int (*inc)(int) = (int (*)(int))find(m0, \"mod_inc\");",
        "long long (*mix)(long long, int, short, unsigned char) =",
        "    (long long (*)(long long, int, short, unsigned char))find(m0, \"mod_mix\");",
        "double (*avg)(double, double, double) =",
        "    (double (*)(double, double, double))find(m0, \"mod_avg\");",
        "struct pair (*pair)(int) = (struct pair (*)(int))find(m0, \"mod_pair\");",
        "long long (*sum)(int, ...) = (long long (*)(int, ...))find(m0, \"mod_sum\");",
        "long long (*table_sum)(void) = (long long (*)(void))find(m0, \"mod_table_sum\");",
    ] {
        program.line(line);
    }
    program.check(Ty::I32, "*counter", i128::from(start));
    program.line(format!("*counter = {set};"));
    program.check(Ty::I32, "inc(5)", i128::from(after_inc));
    program.check(Ty::I32, "*counter", i128::from(after_inc));
    program.check(Ty::I64, &format!("mix({ma}LL, {mb}, {mc}, {md})"), i128::from(mixed));
    program.check(
        Ty::I64,
        &format!("(long long)(avg({v}.5, 2.25, {}.25) * 4)", 4 + v),
        i128::from(average),
    );
    program.line(format!("struct pair p = pair({pair_x});"));
    program.check(Ty::I32, "p.lo", i128::from(pair_x * 2));
    program.check(Ty::I64, "p.hi", i128::from(pair_hi));
    program.check(Ty::I64, "table[3]", i128::from(table[3]));
    program.line(format!("table[2] = {new_cell};"));
    program.check(Ty::I64, "table_sum()", i128::from(table_after.iter().sum::<i64>()));
    program.check(Ty::I64, "sum(5, 1LL, 2LL, 3LL, 4LL, 5LL)", i128::from(short_sum));
    let longs = long_values.iter().map(|x| format!("{x}LL")).collect::<Vec<_>>().join(", ");
    program.check(Ty::I64, &format!("sum(9, {longs})"), i128::from(long_sum));
    push(sink, "dlsym-calls", variant, program, vec![m0]);
}

/// Each module has a constructor that runs while `dlopen` is loading it and calls back in.
///
/// The constructor reads a global in the executable through a pointer the module keeps in its
/// own data, which is a relocation against a symbol in another image resolved at load, and
/// reports through a function pointer kept the same way. Opening a module that is already open
/// hands back the same handle and does not run the constructor again.
fn constructor(sink: &mut Sink<'_>, variant: usize) {
    let modules = variant + 1;
    let generation = |m: usize| (m + 1 + variant) as i64;

    let mut units = Vec::new();
    for m in 0..modules {
        let body = format!(
            "\
extern int load_generation;
void note_init(int, int);

static int *const generation_at = &load_generation;
static void (*const report)(int, int) = note_init;
static int my_generation = -1;

__attribute__((constructor)) static void mod_init(void)
{{
    my_generation = *generation_at;
    report({}, my_generation);
}}

int mod_generation(void)
{{
    return my_generation;
}}
",
            m + 1
        );
        units.push(module(&format!("m{m}"), "Calls back into the executable as it loads.", &body));
    }

    let mut program =
        executable(format!("{modules} modules whose constructors call back while they load"));
    for line in [
        "int load_generation;",
        "static long long init_log;",
        "static int init_count;",
        "",
        "void note_init(int who, int generation)",
        "{",
        "    init_count++;",
        "    init_log = init_log * 100 + who * 10 + generation;",
        "}",
    ] {
        program.top(line);
    }
    program.check(Ty::I32, "init_count", 0);
    let mut log = 0i64;
    for m in 0..modules {
        program.line(format!("load_generation = {};", generation(m)));
        open(&mut program, &format!("m{m}"));
        log = log * 100 + (m as i64 + 1) * 10 + generation(m);
        program.check(Ty::I32, "init_count", (m + 1) as i128);
        program.check(Ty::I64, "init_log", i128::from(log));
    }
    program.line("load_generation = 99;");
    program.line("void *again = load(\"./m0.so\");");
    program.check(Ty::I32, "again == m0", 1);
    program.check(Ty::I32, "init_count", modules as i128);
    for m in 0..modules {
        program.line(format!(
            "int (*generation{m})(void) = (int (*)(void))find(m{m}, \"mod_generation\");"
        ));
        program.check(Ty::I32, &format!("generation{m}()"), i128::from(generation(m)));
    }
    push(sink, "constructor", variant, program, units);
}

/// A module working on its own globals, statics and constant tables, position independently.
///
/// A static array, a static struct, an exported global that another image could interpose on
/// and a hidden one that nothing outside can see, a constant table of strings and one of
/// pointers to static functions, all of which need the module's own base address to reach. The
/// executable finds the exported one by name and checks the hidden one and a static cannot be
/// found at all.
fn module_statics(sink: &mut Sink<'_>, variant: usize) {
    const WORDS: [&str; 3] = ["one", "three", "seven"];
    let v = variant as i64;
    let k = v + 4;
    let weights: Vec<i64> = (0..8).map(|i| i * 3 + v + 1).collect();
    let xs: Vec<i64> = (0..6 + v).map(|j| j * 5 + v * 3 + 1).collect();

    let mut counts = [0i64; 8];
    let mut total = 1000 * v;
    let mut last = -1i64;
    let mut visible = 0i64;
    let mut hidden = 0i64;
    let mut steps = Vec::new();
    for &x in &xs {
        let slot = (x & 7) as usize;
        counts[slot] += weights[slot];
        let op = match x % 3 {
            0 => x + k,
            1 => x * 3,
            _ => x - k,
        };
        total += op + i64::from(WORDS[(x % 3) as usize].as_bytes()[slot % 3]);
        last = x;
        visible += x;
        hidden += 2 * x;
        steps.push(total);
    }
    let report: i64 = counts.iter().enumerate().map(|(i, c)| c * (i as i64 + 1)).sum::<i64>()
        + last
        + visible
        + hidden;

    let body = format!(
        "\
static int counts[8];
static struct {{
    long long total;
    int last;
}} state = {{ {}, -1 }};
int visible_total;
__attribute__((visibility(\"hidden\"))) int hidden_total;

static const int weights[8] = {{ {} }};
static const char *const words[3] = {{ \"one\", \"three\", \"seven\" }};

static int op_add(int x)
{{
    return x + {k};
}}

static int op_mul(int x)
{{
    return x * 3;
}}

static int op_sub(int x)
{{
    return x - {k};
}}

static int (*const ops[3])(int) = {{ op_add, op_mul, op_sub }};

long long mod_step(int x)
{{
    int slot = x & 7;
    counts[slot] += weights[slot];
    state.total += ops[x % 3](x) + words[x % 3][slot % 3];
    state.last = x;
    visible_total += x;
    hidden_total += 2 * x;
    return state.total;
}}

long long mod_report(void)
{{
    long long sum = 0;
    for (int i = 0; i < 8; i++) {{
        sum += (long long)counts[i] * (i + 1);
    }}
    return sum + state.last + visible_total + hidden_total;
}}
",
        1000 * v,
        weights.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
    );
    let m0 = module("m0", "Works on its own globals and tables.", &body);

    let mut program = executable(format!("A module stepping its own statics {} times", xs.len()));
    open(&mut program, "m0");
    program.line("long long (*step)(int) = (long long (*)(int))find(m0, \"mod_step\");");
    program.line("long long (*report)(void) = (long long (*)(void))find(m0, \"mod_report\");");
    for (&x, &after) in xs.iter().zip(&steps) {
        program.check(Ty::I64, &format!("step({x})"), i128::from(after));
    }
    program.check(Ty::I64, "report()", i128::from(report));
    program.line("int *visible = (int *)find(m0, \"visible_total\");");
    program.check(Ty::I32, "*visible", i128::from(visible));
    program.check(Ty::I32, "dlsym(m0, \"hidden_total\") == 0", 1);
    program.check(Ty::I32, "dlsym(m0, \"counts\") == 0", 1);
    push(sink, "module-statics", variant, program, vec![m0]);
}

/// Two modules built from one source, open at once, each with its own copy of every name.
///
/// Every Postgres module defines `_PG_init` and `Pg_magic_func`, and many define the same
/// statics, so this is the ordinary case rather than an odd one. Both are opened with
/// `RTLD_LOCAL`, so each one's references to its exported global have to land on its own copy
/// and not the one in the module opened before it.
fn same_names(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let start = [1 + v, 100 + v];
    let shared = [5i64, 50 + v];

    let mut units = Vec::new();
    for m in 0..2 {
        let body = format!(
            "\
static long long state = {};
int shared_name = {};

static int bump(void)
{{
    return ++shared_name;
}}

long long step(int x)
{{
    state = state * 2 + x + shared_name;
    bump();
    return state;
}}

int get_shared(void)
{{
    return shared_name;
}}
",
            start[m], shared[m]
        );
        units.push(module(
            &format!("m{m}"),
            "One of two modules with exactly the same names in them.",
            &body,
        ));
    }

    let mut program = executable("Two modules open at once with the same names in them".to_owned());
    for m in 0..2 {
        let name = format!("m{m}");
        open(&mut program, &name);
        program.line(format!(
            "long long (*step{m})(int) = (long long (*)(int))find({name}, \"step\");"
        ));
        program.line(format!("int (*get{m})(void) = (int (*)(void))find({name}, \"get_shared\");"));
    }
    program.check(Ty::I32, "step0 != step1", 1);
    let mut state = start;
    let mut names = shared;
    for j in 0..4 + variant {
        let m = j % 2;
        let x = j as i64 + 1;
        state[m] = state[m] * 2 + x + names[m];
        names[m] += 1;
        program.check(Ty::I64, &format!("step{m}({x})"), i128::from(state[m]));
    }
    for (m, &name) in names.iter().enumerate() {
        program.check(Ty::I32, &format!("get{m}()"), i128::from(name));
        program.check(Ty::I32, &format!("*(int *)find(m{m}, \"shared_name\")"), i128::from(name));
    }
    push(sink, "same-names", variant, program, units);
}

/// Thread local storage in the module and in the executable, reached from the module.
///
/// A module's own thread locals live in a block the loader allocates when it is opened, and one
/// in the executable lives in the block the program started with, so the module reaches the
/// two in different ways and neither is the way the executable reaches its own.
fn thread_local(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let mut depth = 5 + v;
    let mut exe_acc = 0i64;
    let mut calls = 10 * (v + 1);
    let mut acc = 7 + v;
    let xs: Vec<i64> = (1..=4 + v).map(|x| x * 3 - v).collect();

    let initial = (depth, calls, acc);
    let mut results = Vec::new();
    for &x in &xs {
        depth += 1;
        calls += 1;
        acc += x * depth;
        exe_acc += calls;
        results.push(acc);
    }

    let body = format!(
        "\
extern _Thread_local int exe_depth;
extern _Thread_local long long exe_tl_acc;

static _Thread_local int mod_calls = {};
_Thread_local long long mod_acc = {};

long long mod_enter(int x)
{{
    exe_depth++;
    mod_calls++;
    mod_acc += (long long)x * exe_depth;
    exe_tl_acc += mod_calls;
    return mod_acc;
}}

int mod_get_calls(void)
{{
    return mod_calls;
}}
",
        initial.1, initial.2
    );
    let m0 = module("m0", "Thread locals of its own and of the executable's.", &body);

    let mut program =
        executable(format!("Thread locals across the module boundary, {} calls", xs.len()));
    program.top(format!("_Thread_local int exe_depth = {};", initial.0));
    program.top("_Thread_local long long exe_tl_acc;");
    program.top("");
    open(&mut program, "m0");
    program.line("long long (*enter)(int) = (long long (*)(int))find(m0, \"mod_enter\");");
    program.line("int (*get_calls)(void) = (int (*)(void))find(m0, \"mod_get_calls\");");
    for (&x, &after) in xs.iter().zip(&results) {
        program.check(Ty::I64, &format!("enter({x})"), i128::from(after));
    }
    program.check(Ty::I32, "exe_depth", i128::from(depth));
    program.check(Ty::I64, "exe_tl_acc", i128::from(exe_acc));
    program.check(Ty::I32, "get_calls()", i128::from(calls));
    push(sink, "thread-local", variant, program, vec![m0]);
}

/// The module defines configuration variables that the executable then sets.
///
/// `DefineCustomIntVariable` hands the server the address of a variable in the module, static
/// or not, and `SetConfigOption` writes through it later, clamped to the range the module gave.
/// The module reads its own variables and one of the server's, `work_mem`, after each change.
fn guc(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let mut threshold = 10 + 5 * v;
    let mut limit = 3 + v;
    let mut work_mem = 4096 + v;
    let xs = [5i64, 20 + v, 60];
    let eval = |x: i64, threshold: i64, limit: i64, work_mem: i64| {
        if x > threshold { x * limit + work_mem } else { work_mem - x }
    };

    let body = format!(
        "\
extern int work_mem;
void DefineCustomIntVariable(const char *, int *, int, int, int);

static int threshold;
int limit;

void _PG_init(void)
{{
    DefineCustomIntVariable(\"mod.threshold\", &threshold, {threshold}, 0, 100);
    DefineCustomIntVariable(\"mod.limit\", &limit, {limit}, 1, 50);
}}

long long mod_eval(int x)
{{
    return x > threshold ? (long long)x * limit + work_mem : work_mem - x;
}}
"
    );
    let m0 = module("m0", "Defines configuration variables the executable sets.", &body);

    let mut program =
        executable(format!("Configuration variables defined by a module, threshold {threshold}"));
    for line in [
        "struct guc {".to_owned(),
        "    const char *name;".to_owned(),
        "    int *addr;".to_owned(),
        "    int min;".to_owned(),
        "    int max;".to_owned(),
        "};".to_owned(),
        String::new(),
        "static struct guc gucs[8];".to_owned(),
        "static int ngucs;".to_owned(),
        format!("int work_mem = {work_mem};"),
        String::new(),
        "static int same(const char *a, const char *b)".to_owned(),
        "{".to_owned(),
        "    while (*a && *a == *b) {".to_owned(),
        "        a++;".to_owned(),
        "        b++;".to_owned(),
        "    }".to_owned(),
        "    return *a == *b;".to_owned(),
        "}".to_owned(),
        String::new(),
        "void DefineCustomIntVariable(const char *name, int *addr, int boot, int min, int max)"
            .to_owned(),
        "{".to_owned(),
        "    *addr = boot;".to_owned(),
        "    gucs[ngucs].name = name;".to_owned(),
        "    gucs[ngucs].addr = addr;".to_owned(),
        "    gucs[ngucs].min = min;".to_owned(),
        "    gucs[ngucs].max = max;".to_owned(),
        "    ngucs++;".to_owned(),
        "}".to_owned(),
        String::new(),
        "static int SetConfigOption(const char *name, int value)".to_owned(),
        "{".to_owned(),
        "    for (int i = 0; i < ngucs; i++) {".to_owned(),
        "        if (same(gucs[i].name, name)) {".to_owned(),
        "            if (value < gucs[i].min) {".to_owned(),
        "                value = gucs[i].min;".to_owned(),
        "            }".to_owned(),
        "            if (value > gucs[i].max) {".to_owned(),
        "                value = gucs[i].max;".to_owned(),
        "            }".to_owned(),
        "            *gucs[i].addr = value;".to_owned(),
        "            return 1;".to_owned(),
        "        }".to_owned(),
        "    }".to_owned(),
        "    return 0;".to_owned(),
        "}".to_owned(),
    ] {
        program.top(line);
    }
    open(&mut program, "m0");
    program.line("void (*init)(void) = (void (*)(void))find(m0, \"_PG_init\");");
    program.line("long long (*eval)(int) = (long long (*)(int))find(m0, \"mod_eval\");");
    program.line("init();");
    program.check(Ty::I32, "ngucs", 2);
    for &x in &xs {
        program.check(
            Ty::I64,
            &format!("eval({x})"),
            i128::from(eval(x, threshold, limit, work_mem)),
        );
    }
    program.check(Ty::I32, "SetConfigOption(\"mod.threshold\", 150)", 1);
    threshold = 100;
    program.check(Ty::I32, "SetConfigOption(\"mod.limit\", 0)", 1);
    limit = 1;
    program.check(Ty::I32, "SetConfigOption(\"mod.missing\", 3)", 0);
    program.line("work_mem = 64;");
    work_mem = 64;
    for &x in &xs {
        program.check(
            Ty::I64,
            &format!("eval({x})"),
            i128::from(eval(x, threshold, limit, work_mem)),
        );
    }
    program.check(Ty::I32, "SetConfigOption(\"mod.threshold\", 3)", 1);
    threshold = 3;
    for &x in &xs {
        program.check(
            Ty::I64,
            &format!("eval({x})"),
            i128::from(eval(x, threshold, limit, work_mem)),
        );
    }
    push(sink, "guc", variant, program, vec![m0]);
}

#[cfg(test)]
mod tests {
    use super::DLOPEN;
    use crate::{Options, generate};
    use corpus_model::Facet;

    #[test]
    fn every_case_loads_at_least_one_module_and_is_tagged_so_windows_can_leave_it_out() {
        let corpus = generate(&Options::all().only(&[Facet::Bundle])).unwrap();
        assert!(corpus.cases.len() >= 30, "only {} cases", corpus.cases.len());
        for case in &corpus.cases {
            assert!(case.has_tag(DLOPEN), "{}", case.id);
            assert!(!case.units.is_empty(), "{}", case.id);
            assert!(case.units.iter().all(corpus_model::Unit::is_module), "{}", case.id);
            for unit in &case.units {
                assert!(
                    case.source.contains(&format!("\"./{}\"", unit.module_file_name())),
                    "{} never opens {}",
                    case.id,
                    unit.name
                );
            }
        }
    }

    #[test]
    fn the_variadic_hash_folds_the_same_way_the_c_does() {
        // FNV-1a of one byte from the offset basis, which is a published test vector.
        assert_eq!(super::mix(0xcbf2_9ce4_8422_2325, u64::from(b'a')), 0xaf63_dc4c_8601_ec8c);
    }
}
