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

    /// The probability the compiler puts on an edge before it has ever run the program.
    ///
    /// The static predictors and the block frequencies worked out from them. A wrong
    /// probability is not a wrong answer on its own, which is exactly why it needs its own
    /// facet: the damage shows up later, as a transformation that fired on the cold path.
    BranchProbability,

    /// The address of a label, and the indirect jump that goes through it.
    ///
    /// A floor facet and not a back end one, because an indirect jump is the one edge whose
    /// target the compiler cannot see, and every analysis under every pass has to survive
    /// that. It is also what every bytecode interpreter is written as.
    ComputedGoto,

    /// An object whose size is not known until the program runs.
    ///
    /// A floor facet for the same reason `computed-goto` is one. A variable length array is
    /// the one object whose size the compiler cannot see, so the frame it lives in has to be
    /// laid out at run time and every analysis above that has to survive not knowing how big
    /// it is. `alloca` is here too, because it asks the same question in a rougher way.
    VlaAndAlloca,

    /// Folding an operation on constants into a constant.
    ConstantFold,
    /// Rewriting an operation into a cheaper one that computes the same value.
    Strength,
    /// Taking the width back off arithmetic that C promoted to `int`.
    Narrowing,
    /// Algebraic identities and the local peephole rules that follow from them.
    Simplify,
    /// Collapsing the two branches of `&&` or `||` into one, where that is allowed.
    ShortCircuit,
    /// Moving a store below a branch whose two arms both wrote to the same place.
    ConditionalStore,
    /// A branch whose condition already settles the value its two arms disagree about.
    ValueSettled,
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
    /// A branch or a switch case that a condition above it has already settled.
    Prune,
    /// Proving that two memory references cannot be the same object.
    AliasAnalysis,
    /// Walking back from a load to the store that last wrote what it reads.
    MemorySsa,
    /// Turning an address-taken local that never escapes back into a value.
    ScalarReplacement,

    /// Hoisting a loop invariant computation out of the loop.
    LoopInvariant,
    /// The shapes that decide whether an invariant computation is allowed out of its loop.
    LoopHoist,
    /// Rewriting the induction variables of a loop into a cheaper set.
    InductionVariable,
    /// The address shapes that decide which induction variables a loop is left with.
    IvSelection,
    /// Removing a bound check or a branch that the loop guard already decided.
    LoopUnswitch,
    /// Unrolling a loop body, whether the trip count is known or not.
    LoopUnroll,
    /// The loop shapes complete unrolling has to count correctly or leave alone.
    LoopUnrollShape,
    /// Recognizing a loop that is a memory operation the runtime already has.
    LoopIdiom,
    /// Deleting a loop whose body computes nothing anybody reads.
    LoopDeletion,
    /// Rotating a loop so the test lands at the bottom.
    LoopRotate,
    /// The shape a loop is left in by canonicalization, rather than what it computes.
    LoopShape,
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
    /// An optimization that cannot happen until the compiler has seen more than one file.
    ///
    /// Every other facet is one translation unit, and that is the right default, because a
    /// failure in a single file is a failure about code generation rather than about linking.
    /// This one is the exception on purpose. Inlining across a file boundary, propagating a
    /// constant across one, and throwing away a function nothing calls across one are all
    /// things a compiler can only do when it has been handed the other file, so a case about
    /// them has to be more than one file. Each shape is generated twice, once compiled and
    /// linked the ordinary way and once with `-flto`, and the pair has to print the same
    /// answer. That is what makes the second half evidence rather than an assertion.
    LinkTimeOptimization,

    /// Choosing the machine instruction for an operation.
    Selection,
    /// How many values are live at once, which is how many registers the code needs.
    RegisterPressure,
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
    /// Stretches of consecutive labels that share one arm.
    SwitchRuns,
    /// Deciding what a call has to save and restore.
    CallingConvention,
    /// The peephole rules that only make sense on machine instructions.
    MachinePeephole,
    /// The bit counting builtins, which a back end has to have an instruction for.
    BitBuiltins,
    /// Conversions between the floating types and the integer ones, in both directions.
    FloatConversion,

    /// The widest floating type, which is the one whose shape the target decides.
    ///
    /// A back end facet, because almost everything that is hard about `long double` is below
    /// the machine independent IR. It is eighty bits on x86, a hundred and twenty eight on
    /// aarch64 and the same as `double` on some targets, so where it is passed, what it is
    /// padded to and how it comes back out of a call are all things the back end decides.
    LongDouble,

    /// Programs whose point is that the compiler must not do something.
    ///
    /// Volatile, type punning, escaping pointers, and the flags that turn an optimization
    /// off. A pass firing here is a bug, and the case exists to catch it. The two barriers
    /// big enough to have grown their own facets, atomics and `setjmp`, are not here.
    Barrier,

    /// The atomic builtins and the header that wraps them.
    ///
    /// A correctness facet for the same reason `barrier` is one. Most of what an atomic asks
    /// of a compiler is restraint, and a pass that fires across one is a bug even when the
    /// single threaded answer stays right.
    Atomics,

    /// The jump that leaves a function without returning from it.
    ///
    /// A correctness facet, because most of what `setjmp` asks of a compiler is restraint. It
    /// puts an edge into the graph that the source does not show, so a store the optimizer
    /// wanted to sink past the call, or a local it wanted to keep in a register that the jump
    /// will not restore, is a wrong answer rather than a slow one. Every interpreter on the
    /// ladder unwinds its errors this way, so it is the frame rule those projects lean on.
    SetjmpLongjmp,

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
        Self::BranchProbability,
        Self::ComputedGoto,
        Self::VlaAndAlloca,
        Self::ConstantFold,
        Self::Strength,
        Self::Narrowing,
        Self::Simplify,
        Self::ShortCircuit,
        Self::ConditionalStore,
        Self::ValueSettled,
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
        Self::Prune,
        Self::AliasAnalysis,
        Self::MemorySsa,
        Self::ScalarReplacement,
        Self::LoopInvariant,
        Self::LoopHoist,
        Self::InductionVariable,
        Self::IvSelection,
        Self::LoopUnswitch,
        Self::LoopUnroll,
        Self::LoopUnrollShape,
        Self::LoopIdiom,
        Self::LoopDeletion,
        Self::LoopRotate,
        Self::LoopShape,
        Self::LoopRestructure,
        Self::Inline,
        Self::TailCall,
        Self::FunctionPurity,
        Self::ConstantArgs,
        Self::Reachability,
        Self::Devirtualize,
        Self::LinkTimeOptimization,
        Self::Selection,
        Self::RegisterPressure,
        Self::RegisterAlloc,
        Self::Scheduling,
        Self::BlockLayout,
        Self::IfConversion,
        Self::SwitchLowering,
        Self::SwitchRuns,
        Self::CallingConvention,
        Self::MachinePeephole,
        Self::BitBuiltins,
        Self::FloatConversion,
        Self::LongDouble,
        Self::Barrier,
        Self::Atomics,
        Self::SetjmpLongjmp,
        Self::Frontend,
    ];

    /// The name that goes in a file name, a command line and a stored record.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::ControlFlow => "control-flow",
            Self::BranchProbability => "branch-probability",
            Self::ComputedGoto => "computed-goto",
            Self::VlaAndAlloca => "vla-and-alloca",
            Self::ConstantFold => "constant-fold",
            Self::Strength => "strength",
            Self::Narrowing => "narrowing",
            Self::Simplify => "simplify",
            Self::ShortCircuit => "short-circuit",
            Self::ConditionalStore => "conditional-store",
            Self::ValueSettled => "value-settled",
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
            Self::Prune => "prune",
            Self::AliasAnalysis => "alias-analysis",
            Self::MemorySsa => "memory-ssa",
            Self::ScalarReplacement => "scalar-replacement",
            Self::LoopInvariant => "loop-invariant",
            Self::LoopHoist => "loop-hoist",
            Self::InductionVariable => "induction-variable",
            Self::IvSelection => "iv-selection",
            Self::LoopUnswitch => "loop-unswitch",
            Self::LoopUnroll => "loop-unroll",
            Self::LoopUnrollShape => "loop-unroll-shape",
            Self::LoopIdiom => "loop-idiom",
            Self::LoopDeletion => "loop-deletion",
            Self::LoopRotate => "loop-rotate",
            Self::LoopShape => "loop-shape",
            Self::LoopRestructure => "loop-restructure",
            Self::Inline => "inline",
            Self::TailCall => "tail-call",
            Self::FunctionPurity => "function-purity",
            Self::ConstantArgs => "constant-args",
            Self::Reachability => "reachability",
            Self::Devirtualize => "devirtualize",
            Self::LinkTimeOptimization => "link-time-optimization",
            Self::Selection => "selection",
            Self::RegisterPressure => "register-pressure",
            Self::RegisterAlloc => "register-alloc",
            Self::Scheduling => "scheduling",
            Self::BlockLayout => "block-layout",
            Self::IfConversion => "if-conversion",
            Self::SwitchLowering => "switch-lowering",
            Self::SwitchRuns => "switch-runs",
            Self::CallingConvention => "calling-convention",
            Self::MachinePeephole => "machine-peephole",
            Self::BitBuiltins => "bit-builtins",
            Self::FloatConversion => "float-conversion",
            Self::LongDouble => "long-double",
            Self::Barrier => "barrier",
            Self::Atomics => "atomics",
            Self::SetjmpLongjmp => "setjmp-longjmp",
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
            Self::BranchProbability => "the odds put on an edge before the program has ever run",
            Self::ComputedGoto => "the address of a label, and the indirect jump through it",
            Self::VlaAndAlloca => "an object whose size is not known until the program runs",
            Self::ConstantFold => "folding an operation on constants into a constant",
            Self::Strength => "rewriting an operation into a cheaper one with the same value",
            Self::Narrowing => "taking the width back off arithmetic that C promoted",
            Self::Simplify => "algebraic identities and the local peephole rules",
            Self::ShortCircuit => "collapsing the two branches of a logical operator into one",
            Self::ConditionalStore => {
                "moving a store below a branch whose arms wrote the same place"
            }
            Self::ValueSettled => "a condition that settles the value its two arms disagree about",
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
            Self::Prune => "a branch or a switch case a condition above it has settled",
            Self::AliasAnalysis => "proving two references cannot name the same object",
            Self::MemorySsa => "walking back from a load to the store that answers it",
            Self::ScalarReplacement => "turning a non-escaping local back into a value",
            Self::LoopInvariant => "hoisting an invariant computation out of a loop",
            Self::LoopHoist => "whether an invariant computation is allowed out of its loop",
            Self::InductionVariable => "rewriting induction variables into a cheaper set",
            Self::IvSelection => "which induction variables a loop is left with, and how many",
            Self::LoopUnswitch => "removing a test the loop guard already decided",
            Self::LoopUnroll => "unrolling a loop body, known trip count or not",
            Self::LoopUnrollShape => "the loop shapes an unroller has to count or refuse",
            Self::LoopIdiom => "recognizing a loop the runtime already implements",
            Self::LoopDeletion => "deleting a loop whose body nobody reads",
            Self::LoopRotate => "rotating a loop so the test lands at the bottom",
            Self::LoopShape => "the shape a loop is left in, before any pass reads it",
            Self::LoopRestructure => "exchanging or fusing loops for locality",
            Self::Inline => "replacing a call with the body of what it called",
            Self::TailCall => "turning a call in tail position into a jump",
            Self::FunctionPurity => "proving purity and using it at the call sites",
            Self::ConstantArgs => "specializing a function to a constant argument",
            Self::Reachability => "removing what nothing references",
            Self::Devirtualize => "turning an indirect call into a direct one",
            Self::LinkTimeOptimization => "optimizing across a translation unit boundary",
            Self::Selection => "choosing the machine instruction for an operation",
            Self::RegisterPressure => "how many values are live at once, and what that costs",
            Self::RegisterAlloc => "assigning registers and deciding what to spill",
            Self::Scheduling => "ordering instructions within a block",
            Self::BlockLayout => "laying out blocks so the common path falls through",
            Self::IfConversion => "turning a short branch into branchless code",
            Self::SwitchLowering => "choosing how to lower a switch",
            Self::SwitchRuns => "stretches of consecutive labels that share one arm",
            Self::CallingConvention => "deciding what a call saves and restores",
            Self::MachinePeephole => "the rules that only make sense on machine instructions",
            Self::BitBuiltins => "the bit counting builtins, over every position at both widths",
            Self::FloatConversion => "conversions between the floating types and the integer ones",
            Self::LongDouble => {
                "the widest floating type, whose shape the target rather than C decides"
            }
            Self::Barrier => "programs where the compiler must not act, and a firing is a bug",
            Self::Atomics => "the atomic builtins at every ordering, and the header over them",
            Self::SetjmpLongjmp => "the jump that leaves a function without returning from it",
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
            Self::Baseline
            | Self::ControlFlow
            | Self::BranchProbability
            | Self::ComputedGoto
            | Self::VlaAndAlloca
            | Self::Frontend => Phase::Floor,
            Self::ConstantFold
            | Self::Strength
            | Self::Narrowing
            | Self::Simplify
            | Self::ShortCircuit
            | Self::ConditionalStore
            | Self::ValueSettled
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
            | Self::Prune
            | Self::AliasAnalysis
            | Self::MemorySsa
            | Self::ScalarReplacement => Phase::Global,
            Self::LoopInvariant
            | Self::InductionVariable
            | Self::IvSelection
            | Self::LoopUnswitch
            | Self::LoopUnroll
            | Self::LoopUnrollShape
            | Self::LoopIdiom
            | Self::LoopDeletion
            | Self::LoopRotate
            | Self::LoopShape
            | Self::LoopHoist
            | Self::LoopRestructure => Phase::Loops,
            Self::Inline
            | Self::TailCall
            | Self::FunctionPurity
            | Self::ConstantArgs
            | Self::Reachability
            | Self::Devirtualize
            | Self::LinkTimeOptimization => Phase::Interprocedural,
            Self::Selection
            | Self::RegisterPressure
            | Self::RegisterAlloc
            | Self::Scheduling
            | Self::BlockLayout
            | Self::IfConversion
            | Self::SwitchLowering
            | Self::SwitchRuns
            | Self::CallingConvention
            | Self::MachinePeephole
            | Self::BitBuiltins
            | Self::FloatConversion
            | Self::LongDouble => Phase::Backend,
            Self::Barrier | Self::Atomics | Self::SetjmpLongjmp => Phase::Correctness,
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
