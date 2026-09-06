//! What a case is about.
//!
//! A facet names the one transformation or analysis a program was written to exercise. It is
//! the join key of the whole system. The generator emits programs tagged with a facet, the
//! harness records results tagged with a facet, and the report answers the question the whole
//! corpus exists to answer, which is whether a given optimization is correct and whether it
//! pays for itself. Without that key you have a pile of timings and no way to attribute them.
//!
//! The list follows the optimization phases of the rucc M4 plan. Adding a facet is a normal
//! change. Renaming one is not, because the name appears in every stored record.

/// The one thing a case is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Facet {
    /// Nothing in particular. The baseline programs that every level should get right and
    /// that exist to catch a compiler that is broken before any optimization runs.
    Baseline,

    /// The control flow shapes the analyses underneath every pass have to get right.
    ///
    /// The graph, both dominance relations and the loop forest. Nothing here is about a
    /// transformation. It is about the shapes that break the analysis a transformation asked.
    ControlFlow,

    /// Folding an operation on constants into a constant.
    ConstantFold,
    /// Rewriting an operation into a cheaper one that computes the same value.
    Strength,
    /// Taking the width back off arithmetic that C promoted to `int`.
    Narrowing,
    /// Algebraic identities and the local peephole rules that follow from them.
    Simplify,
    /// Reassociating an associative chain to shorten its dependency height.
    Reassociate,
    /// Removing a computation whose result nothing reads.
    DeadCode,
    /// Removing a store that a later store to the same place makes invisible.
    DeadStore,
    /// Removing a branch whose condition is known, and the block it guarded.
    UnreachableCode,

    /// Replacing a redundant computation with the earlier one that already produced it.
    CommonSubexpr,
    /// Replacing a load with the value a dominating store or load already put there.
    LoadForwarding,
    /// Moving a computation to a point where it runs no more often than before.
    CodeMotion,
    /// Propagating a copy so the copy itself becomes dead.
    CopyPropagation,
    /// Propagating a value that is a constant on every path that reaches a use.
    ConstantPropagation,
    /// Narrowing an integer to the range its definition can actually produce.
    ValueRange,
    /// Proving that two memory references cannot be the same object.
    AliasAnalysis,
    /// Turning an address-taken local that never escapes back into a value.
    ScalarReplacement,

    /// Hoisting a loop invariant computation out of the loop.
    LoopInvariant,
    /// Rewriting the induction variables of a loop into a cheaper set.
    InductionVariable,
    /// Removing a bound check or a branch that the loop guard already decided.
    LoopUnswitch,
    /// Unrolling a loop body, whether the trip count is known or not.
    LoopUnroll,
    /// Recognizing a loop that is a memory operation the runtime already has.
    LoopIdiom,
    /// Deleting a loop whose body computes nothing anybody reads.
    LoopDeletion,
    /// Rotating a loop so the test lands at the bottom.
    LoopRotate,
    /// Exchanging or fusing loops for locality.
    LoopRestructure,

    /// Replacing a call with the body of what it called.
    Inline,
    /// Turning a call in tail position into a jump.
    TailCall,
    /// Proving a function pure, const or neither, and using that at its call sites.
    FunctionPurity,
    /// Specializing a function to a constant argument.
    ConstantArgs,
    /// Removing a function or a variable that nothing references.
    Reachability,
    /// Turning an indirect call into a direct one.
    Devirtualize,

    /// Choosing the machine instruction for an operation.
    Selection,
    /// Assigning values to registers and deciding what to spill.
    RegisterAlloc,
    /// Ordering instructions within a block.
    Scheduling,
    /// Laying out blocks so the common path falls through.
    BlockLayout,
    /// Turning a short branch into branchless code.
    IfConversion,
    /// Choosing how to lower a switch.
    SwitchLowering,
    /// Deciding what a call has to save and restore.
    CallingConvention,
    /// The peephole rules that only make sense on machine instructions.
    MachinePeephole,

    /// Programs whose point is that the compiler must not do something.
    ///
    /// Volatile, atomics, signal handlers, setjmp, inline assembly, and the flags that turn
    /// an optimization off. A pass firing here is a bug, and the case exists to catch it.
    Barrier,

    /// Programs whose point is the shape of the language rather than an optimization.
    ///
    /// The C23 constructs, the awkward corners of the type system, and everything that has
    /// to parse and typecheck before an optimization can be asked about it.
    Frontend,
}

impl Facet {
    /// Every facet, in declaration order.
    ///
    /// Declaration order is the order of the pipeline, which is the order a reader of the
    /// report expects to see the sections in.
    pub const ALL: &'static [Self] = &[
        Self::Baseline,
        Self::ControlFlow,
        Self::ConstantFold,
        Self::Strength,
        Self::Narrowing,
        Self::Simplify,
        Self::Reassociate,
        Self::DeadCode,
        Self::DeadStore,
        Self::UnreachableCode,
        Self::CommonSubexpr,
        Self::LoadForwarding,
        Self::CodeMotion,
        Self::CopyPropagation,
        Self::ConstantPropagation,
        Self::ValueRange,
        Self::AliasAnalysis,
        Self::ScalarReplacement,
        Self::LoopInvariant,
        Self::InductionVariable,
        Self::LoopUnswitch,
        Self::LoopUnroll,
        Self::LoopIdiom,
        Self::LoopDeletion,
        Self::LoopRotate,
        Self::LoopRestructure,
        Self::Inline,
        Self::TailCall,
        Self::FunctionPurity,
        Self::ConstantArgs,
        Self::Reachability,
        Self::Devirtualize,
        Self::Selection,
        Self::RegisterAlloc,
        Self::Scheduling,
        Self::BlockLayout,
        Self::IfConversion,
        Self::SwitchLowering,
        Self::CallingConvention,
        Self::MachinePeephole,
        Self::Barrier,
        Self::Frontend,
    ];

    /// The name that goes in a file name, a command line and a stored record.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::ControlFlow => "control-flow",
            Self::ConstantFold => "constant-fold",
            Self::Strength => "strength",
            Self::Narrowing => "narrowing",
            Self::Simplify => "simplify",
            Self::Reassociate => "reassociate",
            Self::DeadCode => "dead-code",
            Self::DeadStore => "dead-store",
            Self::UnreachableCode => "unreachable-code",
            Self::CommonSubexpr => "common-subexpr",
            Self::LoadForwarding => "load-forwarding",
            Self::CodeMotion => "code-motion",
            Self::CopyPropagation => "copy-propagation",
            Self::ConstantPropagation => "constant-propagation",
            Self::ValueRange => "value-range",
            Self::AliasAnalysis => "alias-analysis",
            Self::ScalarReplacement => "scalar-replacement",
            Self::LoopInvariant => "loop-invariant",
            Self::InductionVariable => "induction-variable",
            Self::LoopUnswitch => "loop-unswitch",
            Self::LoopUnroll => "loop-unroll",
            Self::LoopIdiom => "loop-idiom",
            Self::LoopDeletion => "loop-deletion",
            Self::LoopRotate => "loop-rotate",
            Self::LoopRestructure => "loop-restructure",
            Self::Inline => "inline",
            Self::TailCall => "tail-call",
            Self::FunctionPurity => "function-purity",
            Self::ConstantArgs => "constant-args",
            Self::Reachability => "reachability",
            Self::Devirtualize => "devirtualize",
            Self::Selection => "selection",
            Self::RegisterAlloc => "register-alloc",
            Self::Scheduling => "scheduling",
            Self::BlockLayout => "block-layout",
            Self::IfConversion => "if-conversion",
            Self::SwitchLowering => "switch-lowering",
            Self::CallingConvention => "calling-convention",
            Self::MachinePeephole => "machine-peephole",
            Self::Barrier => "barrier",
            Self::Frontend => "frontend",
        }
    }

    /// One line saying what the facet covers, for the human report and for `--help`.
    #[must_use]
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Baseline => {
                "programs with no optimization target, which every level must get right"
            }
            Self::ControlFlow => "the graph shapes the analyses under every pass have to get right",
            Self::ConstantFold => "folding an operation on constants into a constant",
            Self::Strength => "rewriting an operation into a cheaper one with the same value",
            Self::Narrowing => "taking the width back off arithmetic that C promoted",
            Self::Simplify => "algebraic identities and the local peephole rules",
            Self::Reassociate => "reassociating a chain to shorten its dependency height",
            Self::DeadCode => "removing a computation whose result nothing reads",
            Self::DeadStore => "removing a store a later store makes invisible",
            Self::UnreachableCode => "removing a branch with a known condition and its dead arm",
            Self::CommonSubexpr => "reusing an earlier computation instead of repeating it",
            Self::LoadForwarding => "replacing a load with the value already in that place",
            Self::CodeMotion => "moving a computation to where it runs no more often",
            Self::CopyPropagation => "propagating a copy so the copy becomes dead",
            Self::ConstantPropagation => "propagating a value constant on every reaching path",
            Self::ValueRange => "narrowing an integer to the range it can hold",
            Self::AliasAnalysis => "proving two references cannot name the same object",
            Self::ScalarReplacement => "turning a non-escaping local back into a value",
            Self::LoopInvariant => "hoisting an invariant computation out of a loop",
            Self::InductionVariable => "rewriting induction variables into a cheaper set",
            Self::LoopUnswitch => "removing a test the loop guard already decided",
            Self::LoopUnroll => "unrolling a loop body, known trip count or not",
            Self::LoopIdiom => "recognizing a loop the runtime already implements",
            Self::LoopDeletion => "deleting a loop whose body nobody reads",
            Self::LoopRotate => "rotating a loop so the test lands at the bottom",
            Self::LoopRestructure => "exchanging or fusing loops for locality",
            Self::Inline => "replacing a call with the body of what it called",
            Self::TailCall => "turning a call in tail position into a jump",
            Self::FunctionPurity => "proving purity and using it at the call sites",
            Self::ConstantArgs => "specializing a function to a constant argument",
            Self::Reachability => "removing what nothing references",
            Self::Devirtualize => "turning an indirect call into a direct one",
            Self::Selection => "choosing the machine instruction for an operation",
            Self::RegisterAlloc => "assigning registers and deciding what to spill",
            Self::Scheduling => "ordering instructions within a block",
            Self::BlockLayout => "laying out blocks so the common path falls through",
            Self::IfConversion => "turning a short branch into branchless code",
            Self::SwitchLowering => "choosing how to lower a switch",
            Self::CallingConvention => "deciding what a call saves and restores",
            Self::MachinePeephole => "the rules that only make sense on machine instructions",
            Self::Barrier => "programs where the compiler must not act, and a firing is a bug",
            Self::Frontend => "language shape rather than optimization, including C23",
        }
    }

    /// The phase of the M4 plan a facet belongs to.
    ///
    /// The report groups by this so that a run against a half finished compiler reads as
    /// progress through the plan rather than as a wall of unrelated failures.
    #[must_use]
    pub const fn phase(self) -> Phase {
        match self {
            Self::Baseline | Self::ControlFlow | Self::Frontend => Phase::Floor,
            Self::ConstantFold
            | Self::Strength
            | Self::Narrowing
            | Self::Simplify
            | Self::Reassociate
            | Self::DeadCode
            | Self::DeadStore
            | Self::UnreachableCode => Phase::Local,
            Self::CommonSubexpr
            | Self::LoadForwarding
            | Self::CodeMotion
            | Self::CopyPropagation
            | Self::ConstantPropagation
            | Self::ValueRange
            | Self::AliasAnalysis
            | Self::ScalarReplacement => Phase::Global,
            Self::LoopInvariant
            | Self::InductionVariable
            | Self::LoopUnswitch
            | Self::LoopUnroll
            | Self::LoopIdiom
            | Self::LoopDeletion
            | Self::LoopRotate
            | Self::LoopRestructure => Phase::Loops,
            Self::Inline
            | Self::TailCall
            | Self::FunctionPurity
            | Self::ConstantArgs
            | Self::Reachability
            | Self::Devirtualize => Phase::Interprocedural,
            Self::Selection
            | Self::RegisterAlloc
            | Self::Scheduling
            | Self::BlockLayout
            | Self::IfConversion
            | Self::SwitchLowering
            | Self::CallingConvention
            | Self::MachinePeephole => Phase::Backend,
            Self::Barrier => Phase::Correctness,
        }
    }

    /// The facet with this name.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|facet| facet.name() == name)
    }
}

impl std::fmt::Display for Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The part of the M4 plan a facet is evidence for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Phase {
    /// The infrastructure everything else stands on.
    Floor,
    /// Transformations inside one block.
    Local,
    /// Transformations across the blocks of one function.
    Global,
    /// Transformations that need loop structure.
    Loops,
    /// Transformations that need more than one function.
    Interprocedural,
    /// Everything below the machine independent IR.
    Backend,
    /// The cases whose job is to prove nothing happened.
    Correctness,
}

impl Phase {
    /// Every phase, in plan order.
    pub const ALL: &'static [Self] = &[
        Self::Floor,
        Self::Local,
        Self::Global,
        Self::Loops,
        Self::Interprocedural,
        Self::Backend,
        Self::Correctness,
    ];

    /// The name that goes in a record.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Floor => "floor",
            Self::Local => "local",
            Self::Global => "global",
            Self::Loops => "loops",
            Self::Interprocedural => "interprocedural",
            Self::Backend => "backend",
            Self::Correctness => "correctness",
        }
    }

    /// The phase with this name.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|phase| phase.name() == name)
    }

    /// What the phase is for, in one sentence, for the index pages and the report.
    #[must_use]
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Floor => {
                "The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on."
            }
            Self::Local => {
                "Transformations that need to see no further than one basic block, which is where the cheapest wins are."
            }
            Self::Global => {
                "Transformations across the blocks of one function, which need dataflow rather than a peephole."
            }
            Self::Loops => {
                "Transformations that need loop structure, which is where most of the remaining time in real programs goes."
            }
            Self::Interprocedural => {
                "Transformations that need to look at more than one function at a time."
            }
            Self::Backend => {
                "Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations."
            }
            Self::Correctness => {
                "The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation."
            }
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::{Facet, Phase};
    use std::collections::BTreeSet;

    #[test]
    fn no_two_facets_share_a_name() {
        let names: BTreeSet<&str> = Facet::ALL.iter().map(|f| f.name()).collect();
        assert_eq!(names.len(), Facet::ALL.len());
    }

    #[test]
    fn every_facet_name_is_spellable_on_a_command_line_and_in_a_file_name() {
        for facet in Facet::ALL {
            let name = facet.name();
            assert!(!name.is_empty());
            assert!(
                name.bytes().all(|b| b.is_ascii_lowercase() || b == b'-'),
                "{name} has a character that would need quoting"
            );
            assert!(!name.starts_with('-') && !name.ends_with('-'), "{name}");
        }
    }

    #[test]
    fn parsing_a_facet_name_gives_the_facet_back() {
        for facet in Facet::ALL {
            assert_eq!(Facet::parse(facet.name()), Some(*facet));
        }
        assert_eq!(Facet::parse("not-a-facet"), None);
        assert_eq!(Facet::parse(""), None);
    }

    #[test]
    fn every_facet_describes_itself_without_repeating_another() {
        let lines: BTreeSet<&str> = Facet::ALL.iter().map(|f| f.describe()).collect();
        assert_eq!(lines.len(), Facet::ALL.len());
        for facet in Facet::ALL {
            assert!(!facet.describe().is_empty(), "{facet}");
        }
    }

    #[test]
    fn every_phase_has_at_least_one_facet_so_no_section_of_the_report_is_empty() {
        for phase in Phase::ALL {
            assert!(Facet::ALL.iter().any(|f| f.phase() == *phase), "{phase} has no facet");
        }
    }

    #[test]
    fn parsing_a_phase_name_gives_the_phase_back() {
        for phase in Phase::ALL {
            assert_eq!(Phase::parse(phase.name()), Some(*phase));
        }
        assert_eq!(Phase::parse("middle"), None);
    }
}
