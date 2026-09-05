//! Building one C program, and knowing what it prints.
//!
//! A generated case is a whole translation unit with a `main` that computes things and prints
//! them. The printed text is the oracle. Two rules make that work.
//!
//! The first is that the generator records the expected text as it builds the program, so a
//! case ships with its answer and never needs a reference compiler to acquire one. If the
//! generator cannot say what a statement prints, it does not emit that statement.
//!
//! The second is that nothing about the output may depend on the machine. No pointer is ever
//! printed, no `sizeof` is ever printed, no address is compared for order, and every value
//! goes out through `long long` or `unsigned long long` so the width of the type under test
//! does not change the text. A case that printed a pointer would differ between two correct
//! compilers and the corpus would call that a bug.

use crate::lang::Ty;
use std::fmt::Write as _;

/// A C program under construction.
#[derive(Debug, Clone)]
pub struct Program {
    /// A short comment at the top saying what the program is for.
    purpose: String,
    /// System headers, in the order they were asked for.
    includes: Vec<String>,
    /// Declarations and function definitions that come before `main`.
    top: Vec<String>,
    /// Statements inside `main`, already indented.
    body: Vec<String>,
    /// What the program is expected to print, in order.
    expected: String,
    /// Whether anything asked for the `printf` declaration.
    prints: bool,
    /// Whether anything asked for the volatile sink.
    wants_sink: bool,
}

impl Program {
    /// A new program with a one line statement of what it is for.
    #[must_use]
    pub fn new(purpose: impl Into<String>) -> Self {
        Self {
            purpose: purpose.into(),
            includes: Vec::new(),
            top: Vec::new(),
            body: Vec::new(),
            expected: String::new(),
            prints: false,
            wants_sink: false,
        }
    }

    /// Adds a line before `main`, such as a global or a whole function.
    pub fn top(&mut self, text: impl Into<String>) {
        self.top.push(text.into());
    }

    /// Asks for a system header.
    ///
    /// Almost no case needs one, and that is on purpose: a corpus that includes headers
    /// everywhere is a corpus that stops telling you about code generation the moment the
    /// header search path is wrong. The exceptions are the few cases whose subject matter is
    /// in a header, such as the variable argument macros, and those carry a `headers` tag so
    /// they can be left out of a run that has not got that far.
    pub fn include(&mut self, header: &str) {
        let line = format!("#include <{header}>");
        if !self.includes.contains(&line) {
            self.includes.push(line);
        }
    }

    /// Adds a statement to `main`, at one level of indentation.
    pub fn line(&mut self, text: impl Into<String>) {
        self.body.push(format!("    {}", text.into()));
    }

    /// Adds a statement at a deeper indentation, for the inside of a loop or an `if`.
    pub fn line_at(&mut self, depth: usize, text: impl Into<String>) {
        self.body.push(format!("{}{}", "    ".repeat(depth + 1), text.into()));
    }

    /// Adds a blank line, to keep the generated source readable by a person.
    ///
    /// Generated code that nobody can read is generated code nobody debugs. When a case
    /// fails, somebody has to open it, and the two seconds spent emitting a blank line
    /// between the setup and the checks is repaid the first time that happens.
    pub fn blank(&mut self) {
        self.body.push(String::new());
    }

    /// Prints an expression of a given type and records what it will print.
    ///
    /// The caller has already worked out the value with [`crate::lang::eval`], so this is
    /// where the program and its expected output are tied together. They cannot drift,
    /// because there is no other way to add a line that prints.
    pub fn check(&mut self, ty: Ty, expr: &str, value: i128) {
        self.prints = true;
        self.body.push(format!(
            "    printf(\"{}\\n\", {}({expr}));",
            ty.print_spec(),
            ty.print_cast()
        ));
        let _ = writeln!(self.expected, "{value}");
    }

    /// The same, at a deeper indentation.
    pub fn check_at(&mut self, depth: usize, ty: Ty, expr: &str, value: i128) {
        self.prints = true;
        self.body.push(format!(
            "{}printf(\"{}\\n\", {}({expr}));",
            "    ".repeat(depth + 1),
            ty.print_spec(),
            ty.print_cast()
        ));
        let _ = writeln!(self.expected, "{value}");
    }

    /// Records that a line already added will print a value.
    ///
    /// For the few cases that need to write the `printf` themselves, such as one inside a
    /// loop body that runs a known number of times. The caller is on the hook for keeping
    /// the two in step, so this is used sparingly and never where [`Self::check`] would do.
    pub fn expects(&mut self, value: i128) {
        self.prints = true;
        let _ = writeln!(self.expected, "{value}");
    }

    /// Whether anything has been printed yet.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !self.prints
    }

    /// How many values the program prints.
    #[must_use]
    pub fn checks(&self) -> usize {
        self.expected.lines().count()
    }

    /// Declares a local whose value the compiler is not allowed to know.
    ///
    /// Many cases need an operand that is a known number to the generator and an unknown one
    /// to the optimizer, because the point of the case is what happens when the value is not
    /// a constant. Reading it once out of a `volatile` global says exactly that, in plain C
    /// that every compiler has to honour.
    ///
    /// Inline assembly would be cheaper and is what a hand written benchmark usually reaches
    /// for. It is the wrong tool here. A corpus that leans on `asm` is a corpus that stops
    /// working the moment it is pointed at a compiler whose assembler support is not
    /// finished, and finding that out is not what these cases are for.
    pub fn input(&mut self, ty: Ty, name: &str, value: i128) {
        self.top.push(format!(
            "static volatile {} {name}_in = {};",
            ty.c_name(),
            ty.literal(value)
        ));
        self.body.push(format!("    {} {name} = {name}_in;", ty.c_name()));
    }

    /// Keeps a value alive without telling the compiler anything about it.
    ///
    /// For the cases where the computation is the point and the result is not. Storing to a
    /// `volatile` global means the store cannot be removed, so the computation feeding it
    /// cannot be removed either, and the case measures the work it was written to measure.
    pub fn sink(&mut self, expr: &str) {
        self.wants_sink = true;
        self.body.push(format!("    corpus_sink = (long long)({expr});"));
    }

    /// The same, at a deeper indentation.
    pub fn sink_at(&mut self, depth: usize, expr: &str) {
        self.wants_sink = true;
        self.body
            .push(format!("{}corpus_sink = (long long)({expr});", "    ".repeat(depth + 1)));
    }

    /// The finished source and the text it prints.
    #[must_use]
    pub fn finish(self) -> (String, String) {
        let mut out = String::new();
        let _ = writeln!(out, "// {}", self.purpose);
        let _ = writeln!(
            out,
            "// Generated by rucc-corpus. Edit the generator, not this file."
        );
        out.push('\n');
        if !self.includes.is_empty() {
            for header in &self.includes {
                let _ = writeln!(out, "{header}");
            }
            out.push('\n');
        }
        if self.prints {
            // Declared here rather than included from stdio.h. The corpus is about code
            // generation, and pulling in a system header makes every case depend on which
            // headers the compiler under test can find, which is a different question.
            let _ = writeln!(out, "int printf(const char *, ...);");
            out.push('\n');
        }
        if self.wants_sink {
            let _ = writeln!(out, "static volatile long long corpus_sink;");
            out.push('\n');
        }
        for item in &self.top {
            let _ = writeln!(out, "{item}");
        }
        if !self.top.is_empty() {
            out.push('\n');
        }
        let _ = writeln!(out, "int main(void) {{");
        for line in &self.body {
            let _ = writeln!(out, "{line}");
        }
        let _ = writeln!(out, "    return 0;");
        let _ = writeln!(out, "}}");
        (out, self.expected)
    }
}

/// A name that is a valid C identifier, built from parts.
///
/// Case ids contain dots and dashes, so a name that comes from one has to be cleaned before
/// it can be a variable. Every character that is not a letter, a digit or an underscore
/// becomes an underscore, and a leading digit gets a prefix.
#[must_use]
pub fn ident(parts: &[&str]) -> String {
    let mut out = String::new();
    for part in parts {
        if !out.is_empty() {
            out.push('_');
        }
        for ch in part.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                out.push(ch);
            } else {
                out.push('_');
            }
        }
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, 'v');
    }
    if out.is_empty() {
        out.push('v');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{Program, ident};
    use crate::lang::Ty;

    #[test]
    fn a_program_with_nothing_in_it_still_compiles_as_a_translation_unit() {
        let (source, expected) = Program::new("empty").finish();
        assert!(source.contains("int main(void) {"));
        assert!(source.contains("return 0;"));
        assert!(expected.is_empty());
    }

    #[test]
    fn printf_is_declared_only_when_something_prints() {
        let (quiet, _) = Program::new("quiet").finish();
        assert!(!quiet.contains("printf"));

        let mut loud = Program::new("loud");
        loud.check(Ty::I32, "1 + 1", 2);
        let (source, _) = loud.finish();
        assert!(source.contains("int printf(const char *, ...);"));
    }

    #[test]
    fn a_check_writes_the_line_and_the_expected_value_together() {
        let mut program = Program::new("one check");
        program.check(Ty::U64, "a + b", 42);
        assert_eq!(program.checks(), 1);
        let (source, expected) = program.finish();
        assert!(source.contains(r#"printf("%llu\n", (unsigned long long)(a + b));"#));
        assert_eq!(expected, "42\n");
    }

    #[test]
    fn a_signed_check_goes_out_through_long_long_and_a_negative_value_survives() {
        let mut program = Program::new("negative");
        program.check(Ty::I8, "x", -128);
        let (source, expected) = program.finish();
        assert!(source.contains(r#"printf("%lld\n", (long long)(x));"#));
        assert_eq!(expected, "-128\n");
    }

    #[test]
    fn expected_output_comes_out_in_the_order_the_checks_were_added() {
        let mut program = Program::new("order");
        program.check(Ty::I32, "a", 1);
        program.check(Ty::I32, "b", 2);
        program.check(Ty::I32, "c", 3);
        let (_, expected) = program.finish();
        assert_eq!(expected, "1\n2\n3\n");
    }

    #[test]
    fn top_level_items_land_before_main() {
        let mut program = Program::new("with a helper");
        program.top("static int helper(int x) { return x + 1; }");
        program.check(Ty::I32, "helper(1)", 2);
        let (source, _) = program.finish();
        let helper_at = source.find("helper").unwrap();
        let main_at = source.find("int main").unwrap();
        assert!(helper_at < main_at);
    }

    #[test]
    fn indentation_is_four_spaces_per_level() {
        let mut program = Program::new("nested");
        program.line("for (int i = 0; i < 2; i++) {");
        program.line_at(1, "int x = i;");
        program.check_at(2, Ty::I32, "x", 0);
        program.line("}");
        let (source, _) = program.finish();
        assert!(source.contains("\n    for (int i = 0"));
        assert!(source.contains("\n        int x = i;"));
        assert!(source.contains("\n            printf"));
    }

    #[test]
    fn an_identifier_is_always_something_c_will_accept() {
        assert_eq!(ident(&["constant-fold", "i32.add"]), "constant_fold_i32_add");
        assert_eq!(ident(&["8bit"]), "v8bit");
        assert_eq!(ident(&[""]), "v");
        assert_eq!(ident(&["a", "b"]), "a_b");
    }

    #[test]
    fn an_opaque_input_reads_a_volatile_global_once_and_uses_no_assembly() {
        let mut program = Program::new("opaque");
        program.input(Ty::U32, "x", 7);
        program.check(Ty::U32, "x", 7);
        let (source, _) = program.finish();
        assert!(source.contains("static volatile unsigned int x_in = 7u;"));
        assert!(source.contains("unsigned int x = x_in;"));
        assert!(!source.contains("asm"), "the corpus must not need assembler support");
    }

    #[test]
    fn a_sink_is_declared_once_however_many_values_go_into_it() {
        let mut program = Program::new("sink");
        program.check(Ty::I32, "1", 1);
        program.sink("a");
        program.sink("b");
        let (source, _) = program.finish();
        assert_eq!(source.matches("static volatile long long corpus_sink;").count(), 1);
        assert!(source.contains("corpus_sink = (long long)(a);"));
    }

    #[test]
    fn a_program_that_never_sinks_anything_does_not_declare_the_sink() {
        let mut program = Program::new("quiet");
        program.check(Ty::I32, "1", 1);
        let (source, _) = program.finish();
        assert!(!source.contains("corpus_sink"));
    }
}
