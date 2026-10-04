//! Modules that import functions and data from the executable that loads them, as Windows has it.
//!
//! A MinGW build of Postgres links the server with `--export-all-symbols` and writes an import
//! library for it, and every module is linked against that library. Everything the server shares
//! with a module is declared `PGDLLIMPORT`, which is `__declspec(dllimport)` in a module, so the
//! compiler reaches it through a pointer named with `__imp_` on the front that the loader fills in
//! when the module is loaded. A function the module calls without the attribute is reached through
//! a thunk the import library supplies, which jumps through the same pointer and has an address of
//! its own. Every function the server looks up in a module, `_PG_init` and `Pg_magic_func` first,
//! is declared `PGDLLEXPORT`, which is `__declspec(dllexport)`.
//!
//! So the questions are about code the compiler writes on one side of a boundary it never sees
//! across. A load of imported data is two loads, one of the pointer and one through it, and a
//! compiler that treats it as one reads the pointer as the value. A call to an imported function
//! is an indirect call through the pointer, of every shape a call takes, wide, variadic and with a
//! struct coming back through memory. The address of an imported function has to be the address
//! the executable has for it, so a hook compared or chained on one side works on the other. And a
//! module has to tell the linker what it exports, or the server finds nothing in it.
//!
//! # How a case is built
//!
//! Like the `bundle` facet, the file with `main` in it is the executable and every other file is
//! a module the executable loads. On Windows the harness links the executable with
//! `--export-all-symbols` and `--out-implib`, and each module with `-shared` against that import
//! library, and the executable loads it with `LoadLibraryA`. On every other system the two
//! macros expand to nothing and the same program is a shared object loaded with `dlopen`, which
//! is how a reader checks the expected answers without a Windows machine. Nothing here carries
//! the `dlopen` tag, because these are the cases that are meant to run on Windows.
//!
//! The Windows functions are declared in the case rather than taken from `windows.h`, which is a
//! header the size of a small program and would make every case depend on a compiler parsing it.

use super::PROVENANCE;
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet, Unit};

/// How many times each shape is generated.
const VARIANTS: usize = 3;

/// What every module starts with: the two macros, as Postgres defines them for a module.
const MODULE_MACROS: &str = "\
#ifdef _WIN32
#define PGDLLIMPORT __declspec(dllimport)
#define PGDLLEXPORT __declspec(dllexport)
#else
#define PGDLLIMPORT
#define PGDLLEXPORT
#endif
";

/// Emits the facet.
pub(super) fn generate(sink: &mut Sink<'_>) {
    for variant in 0..VARIANTS {
        if !sink.wants(Facet::Dllimport) {
            return;
        }
        imported_data(sink, variant);
        imported_calls(sink, variant);
        thunk_calls(sink, variant);
        addresses(sink, variant);
        hooks(sink, variant);
        exports(sink, variant);
    }
}

/// The executable half of every case, with the two helpers that load a module and find a name.
///
/// A module that will not load, or a name that is not there, prints what went wrong, which is not
/// an answer the case expects, so the case fails with the reason in its output rather than with a
/// crash somewhere after it. `LoadLibraryA` wants a backslash in a path, so the directory a module
/// is loaded from is spelled once for each system.
fn executable(purpose: String) -> Program {
    let mut program = Program::new(purpose);
    program.top("#ifdef _WIN32");
    program.top("#define PGDLLEXPORT __declspec(dllexport)");
    program.top("#define HERE \".\\\\\"");
    program.top("__declspec(dllimport) void *LoadLibraryA(const char *);");
    program.top("__declspec(dllimport) void *GetProcAddress(void *, const char *);");
    program.top("__declspec(dllimport) unsigned long GetLastError(void);");
    program.top("#else");
    program.top("#include <dlfcn.h>");
    program.top("#define PGDLLEXPORT");
    program.top("#define HERE \"./\"");
    program.top("#endif");
    program.top("");
    program.top("static void *load(const char *path)");
    program.top("{");
    program.top("#ifdef _WIN32");
    program.top("    void *handle = LoadLibraryA(path);");
    program.top("    if (!handle) {");
    program.top("        printf(\"LoadLibraryA %s: error %lu\\n\", path, GetLastError());");
    program.top("    }");
    program.top("#else");
    program.top("    void *handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);");
    program.top("    if (!handle) {");
    program.top("        printf(\"dlopen %s: %s\\n\", path, dlerror());");
    program.top("    }");
    program.top("#endif");
    program.top("    return handle;");
    program.top("}");
    program.top("");
    program.top("static void *find(void *handle, const char *name)");
    program.top("{");
    program.top("#ifdef _WIN32");
    program.top("    void *found = GetProcAddress(handle, name);");
    program.top("#else");
    program.top("    void *found = dlsym(handle, name);");
    program.top("#endif");
    program.top("    if (!found) {");
    program.top("        printf(\"%s is not exported\\n\", name);");
    program.top("    }");
    program.top("    return found;");
    program.top("}");
    program.top("");
    program
}

/// Loads a module into a local of the same name, and stops the program if it would not load.
fn open(program: &mut Program, name: &str) {
    program.line(format!("void *{name} = load(HERE \"{name}.so\");"));
    program.line(format!("if (!{name}) {{"));
    program.line_at(1, "return 1;");
    program.line("}");
}

/// One module, written the way the generated units are, with the macros on top.
fn module(name: &str, purpose: &str, body: &str) -> Unit {
    Unit::module(
        name,
        format!(
            "// {purpose}\n// Generated by rucc-corpus. Edit the generator, not this file.\n\n{MODULE_MACROS}\n{body}"
        ),
    )
}

/// Emits one case.
fn push(sink: &mut Sink<'_>, shape: &str, variant: usize, program: Program, units: Vec<Unit>) {
    let (source, expected) = program.finish();
    let case = Case::linked(
        Facet::Dllimport,
        Axes::of([("shape", shape), ("variant", &format!("v{variant}"))]),
        Dialect::C17,
        source,
        units,
        Vec::new(),
        Expect::Output(expected),
    )
    .tagged(&["headers", PROVENANCE]);
    sink.push_case(case);
}

/// The module reads and writes data the executable defines, every kind of it, through imports.
///
/// `NBuffers`, `work_mem` and the rest are `PGDLLIMPORT` integers, `ConfigureNames` is a table
/// of structs and the error level names are an array of strings. Each read is a load of the
/// `__imp_` pointer and then a load through it, and each write a load of the pointer and a store
/// through it, with the struct members and the array cells at an offset from what was loaded.
fn imported_data(sink: &mut Sink<'_>, variant: usize) {
    const NAMES: [&str; 3] = ["debug", "notice", "warning"];
    let v = variant as i64;
    let n = 4 + variant;
    let mut nbuffers = 128 + v;
    let mut counter = 500 * (v + 1);
    let a = 9 + v;
    let mut b = 300_000 + v;
    let mut c = (210 + v * 15) as u8;
    let d = -40 - v;
    let e = 1.75 + v as f64;
    let mut table: Vec<i64> = (0..n as i64).map(|i| 3 * i + v + 2).collect();
    let rounds = [3i64, 2];

    let initial_table = table.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
    let initial = (nbuffers, counter, b, c);

    let mut results = Vec::new();
    for &count in &rounds {
        let mut acc = 0i64;
        for r in 0..count {
            acc += nbuffers * (r + 2);
            for cell in &mut table {
                acc += *cell;
                *cell = *cell * 2 - r;
            }
            acc += a + b + i64::from(c) + d + (e * 4.0) as i64;
            b += 2 * r + 1;
            c = c.wrapping_add(11);
            for name in NAMES {
                acc += i64::from(name.as_bytes()[r as usize + 1]);
            }
            counter += acc % 997;
        }
        nbuffers += 2;
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

extern PGDLLIMPORT int NBuffers;
extern PGDLLIMPORT long long counter;
extern PGDLLIMPORT struct config Config;
extern PGDLLIMPORT int table[{n}];
extern PGDLLIMPORT const char *const names[3];

PGDLLEXPORT long long mod_run(int rounds)
{{
    long long acc = 0;
    for (int r = 0; r < rounds; r++) {{
        acc += (long long)NBuffers * (r + 2);
        for (int i = 0; i < {n}; i++) {{
            acc += table[i];
            table[i] = table[i] * 2 - r;
        }}
        acc += Config.a + Config.b + Config.c + Config.d + (long long)(Config.e * 4);
        Config.b += 2 * r + 1;
        Config.c = (unsigned char)(Config.c + 11);
        for (int k = 0; k < 3; k++) {{
            acc += (unsigned char)names[k][r + 1];
        }}
        counter += acc % 997;
    }}
    NBuffers += 2;
    return acc;
}}
"
    );
    let m0 = module("m0", "Reads and writes data the executable exports, through imports.", &body);

    let mut program =
        executable(format!("A module reading and writing {n} kinds of imported data"));
    program.top("struct config {");
    program.top("    int a;");
    program.top("    long long b;");
    program.top("    unsigned char c;");
    program.top("    short d;");
    program.top("    double e;");
    program.top("};");
    program.top("");
    program.top(format!("PGDLLEXPORT int NBuffers = {};", initial.0));
    program.top(format!("PGDLLEXPORT long long counter = {};", initial.1));
    program.top(format!(
        "PGDLLEXPORT struct config Config = {{ {a}, {}, {}, {d}, {e:?} }};",
        initial.2, initial.3
    ));
    program.top(format!("PGDLLEXPORT int table[{n}] = {{ {initial_table} }};"));
    program.top("PGDLLEXPORT const char *const names[3] = { \"debug\", \"notice\", \"warning\" };");
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
    push(sink, "imported-data", variant, program, vec![m0]);
}

/// The module calls functions the executable exports, through imports, of every shape a call has.
///
/// Eight integers of every width, so four of them go on the stack, a `double` and a `float`
/// together, a small struct that comes back in a register and a larger one that comes back
/// through memory the caller passed, a variadic call with a `double` among the arguments, which
/// Windows passes in an integer register as well, and ten arguments. `palloc`, `elog` and
/// `hash_bytes` are all called this way from every module on Windows.
fn imported_calls(sink: &mut Sink<'_>, variant: usize) {
    let s = 7 + variant as i64;
    let x = s * 100_000 + 12_345;
    let answers = [
        (s - 100) + (-30 * s) + 1000 * s + (s << 33) + (s + 200) + 500 * s + 70_000 * s + 3 * s,
        2 * s * s + 9 * s,
        x & 0xffff,
        x >> 16,
        (s - 1) + 10 * (s * s) + 100 * (3 * s + 5),
        2 * s + 40,
        55 * s + 385,
    ];

    let body = "\
struct pair {
    int lo;
    int hi;
};

struct wide {
    long long a;
    long long b;
    long long c;
};

extern PGDLLIMPORT long long add_wide(signed char a, short b, int c, long long d, unsigned char e,
                                      unsigned short f, unsigned g, unsigned long long h);
extern PGDLLIMPORT double scale(double x, float y, int k);
extern PGDLLIMPORT struct pair split(long long x);
extern PGDLLIMPORT struct wide spread(int x);
extern PGDLLIMPORT long long fmt_sum(const char *fmt, ...);
extern PGDLLIMPORT long long ten(int a1, int a2, int a3, int a4, int a5, int a6, int a7, int a8,
                                 int a9, int a10);

PGDLLEXPORT void mod_calls(int s, long long *out)
{
    out[0] = add_wide((signed char)(s - 100), (short)(s * -30), s * 1000, (long long)s << 33,
                      (unsigned char)(s + 200), (unsigned short)(s * 500), (unsigned)s * 70000u,
                      (unsigned long long)s * 3);
    out[1] = (long long)(scale(s + 0.5, 0.25f * s, s) * 8);
    struct pair p = split(s * 100000LL + 12345);
    out[2] = p.lo;
    out[3] = p.hi;
    struct wide w = spread(s);
    out[4] = w.a + w.b * 10 + w.c * 100;
    out[5] = fmt_sum(\"ddfd\", s, -2 * s, 1.5 * s, 40);
    out[6] = ten(s + 1, s + 2, s + 3, s + 4, s + 5, s + 6, s + 7, s + 8, s + 9, s + 10);
}
";
    let m0 = module("m0", "Calls functions the executable exports, through imports.", body);

    let mut program = executable(format!(
        "A module calling {} imported functions of every shape",
        answers.len() - 1
    ));
    program.include("stdarg.h");
    program.top("struct pair {");
    program.top("    int lo;");
    program.top("    int hi;");
    program.top("};");
    program.top("");
    program.top("struct wide {");
    program.top("    long long a;");
    program.top("    long long b;");
    program.top("    long long c;");
    program.top("};");
    program.top("");
    program.top("PGDLLEXPORT long long add_wide(signed char a, short b, int c, long long d,");
    program.top(
        "                             unsigned char e, unsigned short f, unsigned g, unsigned long long h)",
    );
    program.top("{");
    program.top("    return a + b + c + d + e + f + (long long)g + (long long)h;");
    program.top("}");
    program.top("");
    program.top("PGDLLEXPORT double scale(double x, float y, int k)");
    program.top("{");
    program.top("    return x * y + k;");
    program.top("}");
    program.top("");
    program.top("PGDLLEXPORT struct pair split(long long x)");
    program.top("{");
    program.top("    struct pair p = { (int)(x & 0xffff), (int)(x >> 16) };");
    program.top("    return p;");
    program.top("}");
    program.top("");
    program.top("PGDLLEXPORT struct wide spread(int x)");
    program.top("{");
    program.top("    struct wide w = { x - 1, (long long)x * x, 3LL * x + 5 };");
    program.top("    return w;");
    program.top("}");
    program.top("");
    program.top("PGDLLEXPORT long long fmt_sum(const char *fmt, ...)");
    program.top("{");
    program.top("    va_list ap;");
    program.top("    long long sum = 0;");
    program.top("    va_start(ap, fmt);");
    program.top("    for (const char *at = fmt; *at; at++) {");
    program.top("        if (*at == 'f') {");
    program.top("            sum += (long long)(va_arg(ap, double) * 2);");
    program.top("        } else {");
    program.top("            sum += va_arg(ap, int);");
    program.top("        }");
    program.top("    }");
    program.top("    va_end(ap);");
    program.top("    return sum;");
    program.top("}");
    program.top("");
    program
        .top("PGDLLEXPORT long long ten(int a1, int a2, int a3, int a4, int a5, int a6, int a7,");
    program.top("                          int a8, int a9, int a10)");
    program.top("{");
    program.top(
        "    return a1 + 2LL * a2 + 3LL * a3 + 4LL * a4 + 5LL * a5 + 6LL * a6 + 7LL * a7 + 8LL * a8",
    );
    program.top("           + 9LL * a9 + 10LL * a10;");
    program.top("}");
    program.top("");
    open(&mut program, "m0");
    program.line(
        "void (*calls)(int, long long *) = (void (*)(int, long long *))find(m0, \"mod_calls\");",
    );
    program.input(Ty::I32, "s", i128::from(s));
    program.line(format!("long long out[{}];", answers.len()));
    program.line("calls(s, out);");
    for (at, &answer) in answers.iter().enumerate() {
        program.check(Ty::I64, &format!("out[{at}]"), i128::from(answer));
    }
    push(sink, "imported-calls", variant, program, vec![m0]);
}

/// The module calls the executable's functions without the attribute, through the import thunks.
///
/// Postgres declares its functions without `PGDLLIMPORT`, so a module that calls `palloc` calls a
/// thunk the import library put in the module, which jumps through the `__imp_` pointer. The
/// module also keeps a constant table of them, the way an extension fills in a table of
/// callbacks, and there the address is the thunk's, which a static initializer can name.
fn thunk_calls(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let add = 17 + v;
    let flip = 0x55 + v;
    let steps = 6 + 3 * variant;
    let fold = |x: i64, rounds: usize| {
        (0..rounds).fold(x, |x, i| match i % 3 {
            0 => x + add,
            1 => x * 3,
            _ => x ^ flip,
        })
    };
    let starts = [1i64, 20 + v, -5];

    let body = format!(
        "\
long long step_add(long long x);
long long step_mul(long long x);
long long step_xor(long long x);

static long long (*const steps[3])(long long) = {{ step_add, step_mul, step_xor }};

PGDLLEXPORT long long mod_fold(long long x, int rounds)
{{
    for (int i = 0; i < rounds; i++) {{
        x = steps[i % 3](x);
    }}
    return x;
}}

PGDLLEXPORT long long mod_direct(long long x)
{{
    return step_xor(step_mul(step_add(x)));
}}
"
    );
    let m0 = module("m0", "Calls the executable's functions through import thunks.", &body);

    let mut program =
        executable(format!("{steps} calls through import thunks and a table of them"));
    program.top(format!("PGDLLEXPORT long long step_add(long long x) {{ return x + {add}; }}"));
    program.top("PGDLLEXPORT long long step_mul(long long x) { return x * 3; }");
    program.top(format!("PGDLLEXPORT long long step_xor(long long x) {{ return x ^ {flip}; }}"));
    program.top("");
    open(&mut program, "m0");
    program.line(
        "long long (*fold)(long long, int) = (long long (*)(long long, int))find(m0, \"mod_fold\");",
    );
    program.line(
        "long long (*direct)(long long) = (long long (*)(long long))find(m0, \"mod_direct\");",
    );
    program.input(Ty::I32, "rounds", steps as i128);
    for (at, &start) in starts.iter().enumerate() {
        let name = format!("x{at}");
        program.input(Ty::I64, &name, i128::from(start));
        program.check(Ty::I64, &format!("fold({name}, rounds)"), i128::from(fold(start, steps)));
        program.check(Ty::I64, &format!("direct({name})"), i128::from(fold(start, 3)));
    }
    push(sink, "thunk-calls", variant, program, vec![m0]);
}

/// The address of an import, taken in the module, is the address the executable has for it.
///
/// A hook is compared against the standard function to see whether anybody installed one, and a
/// pointer into a server table is handed back to the server. Through `__imp_` both are the real
/// address in the executable. A compiler that took the address of the thunk instead, or of the
/// import pointer itself, gets a number that compares unequal and a write that lands nowhere.
fn addresses(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let n = 6 + variant;
    let at = 2 + variant;
    let table: Vec<i64> = (0..n as i64).map(|i| 11 * i + v).collect();
    let bump = |x: i64| x * 5 + 3 + v;
    let added = 40 + v;

    let body = format!(
        "\
extern PGDLLIMPORT int shared_count;
extern PGDLLIMPORT long long shared_table[{n}];
extern PGDLLIMPORT long long bump(long long);

PGDLLEXPORT int mod_same(int *count, long long *table, long long (*fn)(long long))
{{
    return (count == &shared_count) + 2 * (table == shared_table) + 4 * (fn == bump)
           + 8 * (&shared_table[{at}] == table + {at});
}}

PGDLLEXPORT long long *mod_where(void)
{{
    shared_count++;
    return &shared_table[{at}];
}}

PGDLLEXPORT long long (*mod_fn(void))(long long)
{{
    return bump;
}}
"
    );
    let m0 = module("m0", "Takes the addresses of imports and hands them back.", &body);

    let mut program =
        executable(format!("Addresses of imported data and functions, compared at {n} cells"));
    program.top("PGDLLEXPORT int shared_count;");
    program.top(format!(
        "PGDLLEXPORT long long shared_table[{n}] = {{ {} }};",
        table.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
    ));
    program.top(format!("PGDLLEXPORT long long bump(long long x) {{ return x * 5 + {}; }}", 3 + v));
    program.top("");
    open(&mut program, "m0");
    program.line(
        "int (*same)(int *, long long *, long long (*)(long long)) = (int (*)(int *, long long *, long long (*)(long long)))find(m0, \"mod_same\");",
    );
    program.line("long long *(*where)(void) = (long long *(*)(void))find(m0, \"mod_where\");");
    program.line(
        "long long (*(*fn)(void))(long long) = (long long (*(*)(void))(long long))find(m0, \"mod_fn\");",
    );
    program.check(Ty::I32, "same(&shared_count, shared_table, bump)", 15);
    program.check(Ty::I32, &format!("where() == &shared_table[{at}]"), 1);
    program.check(Ty::I32, "fn() == bump", 1);
    program.input(Ty::I64, "x", 9);
    program.check(Ty::I64, "fn()(x)", i128::from(bump(9)));
    program.line(format!("*where() += {added};"));
    program.check(Ty::I64, &format!("shared_table[{at}]"), i128::from(table[at] + added));
    program.check(Ty::I32, "shared_count", 2);
    push(sink, "addresses", variant, program, vec![m0]);
}

/// What a module adds to the query before it hands it down the chain.
fn hook_shift(m: usize) -> i64 {
    3 * m as i64 + 1
}

/// What a module multiplies the answer that comes back up the chain by.
fn hook_scale(m: usize) -> i64 {
    m as i64 + 2
}

/// What the planner answers with the first `installed` modules' hooks in place.
///
/// The last module installed runs first and hands its shifted query down to the one before, and
/// the bottom of the chain is the standard planner.
fn hook_plan(q: i64, installed: usize) -> i64 {
    if installed == 0 {
        return q * 4 - 1;
    }
    let m = installed - 1;
    hook_plan(q + hook_shift(m), m) * hook_scale(m) + m as i64
}

/// Each module saves an imported hook and installs its own, which calls the saved one.
///
/// `planner_hook` and the rest are `PGDLLIMPORT` function pointers. A module's `_PG_init` reads
/// one through the import, keeps it, and stores its own function through the same import, and the
/// server then calls whatever is there. With more than one module loaded the last one installed
/// runs first and calls down through the others to the standard function, which the modules
/// also reach through an import.
fn hooks(sink: &mut Sink<'_>, variant: usize) {
    let modules = variant + 1;
    let queries = [2i64, 5 + variant as i64, 11];
    let mut units = Vec::new();
    for m in 0..modules {
        let body = format!(
            "\
typedef long long (*planner_hook_type)(long long);

extern PGDLLIMPORT planner_hook_type planner_hook;
extern PGDLLIMPORT long long standard_planner(long long);

static planner_hook_type prev_planner_hook;

static long long my_planner(long long q)
{{
    long long r = prev_planner_hook ? prev_planner_hook(q + {shift}) : standard_planner(q + {shift});
    return r * {scale} + {m};
}}

PGDLLEXPORT void _PG_init(void)
{{
    prev_planner_hook = planner_hook;
    planner_hook = my_planner;
}}
",
            shift = hook_shift(m),
            scale = hook_scale(m),
        );
        units.push(module(&format!("m{m}"), "Chains onto an imported planner hook.", &body));
    }

    let mut program =
        executable(format!("{modules} modules chaining onto an imported planner hook"));
    program.top("typedef long long (*planner_hook_type)(long long);");
    program.top("");
    program.top("PGDLLEXPORT planner_hook_type planner_hook;");
    program.top("");
    program.top("PGDLLEXPORT long long standard_planner(long long q)");
    program.top("{");
    program.top("    return q * 4 - 1;");
    program.top("}");
    program.top("");
    program.top("static long long plan(long long q)");
    program.top("{");
    program.top("    return planner_hook ? planner_hook(q) : standard_planner(q);");
    program.top("}");
    program.top("");
    program.input(Ty::I64, "q", i128::from(queries[0]));
    program.check(Ty::I64, "plan(q)", i128::from(hook_plan(queries[0], 0)));
    for m in 0..modules {
        let name = format!("m{m}");
        open(&mut program, &name);
        program
            .line(format!("void (*init{m})(void) = (void (*)(void))find({name}, \"_PG_init\");"));
        program.line(format!("init{m}();"));
        program.check(Ty::I64, "plan(q)", i128::from(hook_plan(queries[0], m + 1)));
    }
    for &q in &queries {
        program.check(Ty::I64, &format!("plan({q})"), i128::from(hook_plan(q, modules)));
    }
    push(sink, "hooks", variant, program, units);
}

/// The executable finds a module's exported functions and data by name, and they call back in.
///
/// `PG_MODULE_MAGIC` defines `Pg_magic_func`, which hands back a constant struct, and
/// `PG_FUNCTION_INFO_V1` defines one exported function for each SQL function. Both are
/// `PGDLLEXPORT`, which on Windows is a directive the compiler leaves in the object for the
/// linker. The module here exports a function returning a constant table, a function that
/// updates exported data and imported data both, and the exported data itself, which the
/// executable reads and writes through the address it found.
fn exports(sink: &mut Sink<'_>, variant: usize) {
    let v = variant as i64;
    let magic = [180_000 + v, 100, 32 + v, 8];
    let total = 1000 + 10 * v;
    let start = 3 + v;
    let steps = 3 + variant;

    let mut exe_total = start;
    let mut mod_total = total;
    let mut seen = Vec::new();
    for i in 0..steps as i64 {
        mod_total += i * 3 + exe_total;
        exe_total += 1;
        seen.push(mod_total);
    }
    let after_reset = {
        let x = 100;
        x * 3 + exe_total
    };
    exe_total += 1;

    let body = format!(
        "\
extern PGDLLIMPORT long long exe_total;

static const int magic[4] = {{ {}, {}, {}, {} }};

PGDLLEXPORT long long mod_total = {total};

static long long helper(long long x)
{{
    return x * 3 + exe_total;
}}

long long mod_plain(long long x)
{{
    return helper(x) + 1;
}}

PGDLLEXPORT const int *Pg_magic_func(void)
{{
    return magic;
}}

PGDLLEXPORT long long mod_step(long long x)
{{
    mod_total += helper(x);
    exe_total += 1;
    return mod_total;
}}
",
        magic[0], magic[1], magic[2], magic[3]
    );
    let m0 = module("m0", "Exports functions and data for the executable to find.", &body);

    let mut program = executable(format!("{steps} calls into a module's exported functions"));
    program.top(format!("PGDLLEXPORT long long exe_total = {start};"));
    program.top("");
    open(&mut program, "m0");
    program.line(
        "const int *(*magic_func)(void) = (const int *(*)(void))find(m0, \"Pg_magic_func\");",
    );
    program.line("const int *magic = magic_func();");
    for (at, &value) in magic.iter().enumerate() {
        program.check(Ty::I32, &format!("magic[{at}]"), i128::from(value));
    }
    program.line("long long *total = (long long *)find(m0, \"mod_total\");");
    program.check(Ty::I64, "*total", i128::from(total));
    program
        .line("long long (*step)(long long) = (long long (*)(long long))find(m0, \"mod_step\");");
    for (i, &value) in seen.iter().enumerate() {
        program.check(Ty::I64, &format!("step({i})"), i128::from(value));
    }
    program.check(Ty::I64, "*total", i128::from(mod_total));
    program.line("*total = 0;");
    program.check(Ty::I64, "step(100)", i128::from(after_reset));
    program.check(Ty::I64, "exe_total", i128::from(exe_total));
    push(sink, "exports", variant, program, vec![m0]);
}
