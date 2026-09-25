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
    /// Sending an edge that decides a branch below it straight to the arm it takes.
    JumpThreading,
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
    /// Removing a parameter nothing reads, and the argument at every call with it.
    UnusedParams,
    /// Removing a return value no call reads, and the result at every call with it.
    UnusedReturns,
    /// What a declaration of a callee in another file promised about memory, and what
    /// believing it is worth.
    DeclaredPurity,
    /// Removing a function or a variable that nothing references.
    Reachability,
    /// Turning an indirect call into a direct one.
    Devirtualize,
    /// What a callee reads and writes, as seen from its call sites.
    ///
    /// Purity is the coarse version of this question, and it only has three answers. This one
    /// is the fine version: a callee that writes through one of its two pointer arguments and
    /// not the other, or that reads a local it was lent without keeping the address. Both are
    /// things a caller can act on, and neither shows up in the output, so the evidence is the
    /// instruction count.
    MemoryEffects,
    /// Whether a call came out of a loop.
    ///
    /// The loop facets can say whether a load was hoisted. This one asks the same question
    /// about the call itself, which is only answerable once the compiler knows what the
    /// callee does to memory. A call to a callee that reads a table nothing in the loop
    /// writes may come out. The same call may not, once the loop writes that table.
    CallMotion,
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
    /// A switch dispatched enough times that how it was lowered shows up in the clock.
    SwitchDispatch,
    /// Deciding what a call has to save and restore.
    CallingConvention,
    /// The peephole rules that only make sense on machine instructions.
    MachinePeephole,
    /// A widening whose upper bits nothing ever reads.
    ///
    /// A facet of its own rather than a shape of `machine-peephole`, because what decides it
    /// is not the instruction on the next line. C promotes every narrow operand to `int`
    /// before doing anything with it, so the conversions are everywhere, and whether the bits
    /// a widening worked out are bits anybody looks at is a question about every reader of the
    /// result. The pairs that sit together in one block are a peephole rule. These are the
    /// rest.
    BitLiveness,
    /// A comparison the machine has already made.
    ///
    /// A comparison sets a few bits nobody named and the instruction behind it reads them, so
    /// a comparison that sets the bits already sitting there is one nothing could tell had
    /// run. Two shapes put them there: the same comparison made twice, and a comparison
    /// against zero of a value arithmetic has just worked out. It is a facet of its own rather
    /// than a shape of `machine-peephole` because half of what decides it is which conditions
    /// read what the arithmetic left, which is a fact about the machine rather than about the
    /// pair of instructions.
    CompareElim,
    /// The address shapes that decide whether an address is worked out once into a register
    /// or carried by every instruction that reads it.
    AddressFold,
    /// A load whose value one arithmetic instruction reads, and everything that stops it moving.
    ///
    /// A back end facet of its own rather than a shape of `machine-peephole`, because what
    /// decides it is not the pair of instructions. The load stops being where it was and
    /// becomes part of an instruction further down, so what settles it is everything between
    /// the two and everybody else who wanted the value. So the shapes are the reasons rather
    /// than the arithmetic: one reader against two, a store in the way, a call in the way,
    /// another load in the way, a write of a register the address reads, a reader in another
    /// block, and the distance between the two.
    ///
    /// The load in the way is the one worth having. A back end that moves a read past a read
    /// reorders two accesses, and by the time it can see them it cannot tell a `volatile` one
    /// from an ordinary one, so the case exists to say what the order has to be.
    LoadFold,
    /// A load, arithmetic on what came back, and a store of the answer to the same place.
    ///
    /// A back end facet of its own rather than a shape of `load-fold`, because it is not the
    /// same question with the operands the other way about. Three instructions become one here
    /// rather than two becoming one, and the three have to be read exactly as instruction
    /// selection wrote them: `*p -= x` and `x - *p` are the same pair of operands once a load
    /// has been folded into the subtraction, and only one of them is the instruction that
    /// writes memory. So the shapes are both arrangements of the subtraction, each of the five
    /// operations that have a memory destination, and then every reason the three are not one:
    /// the loaded word read twice, the answer read by something besides the store, an access
    /// in between, a call in between, a store to somewhere else, a store at another
    /// displacement off the same address, and an index written in between.
    ///
    /// The two frame slots are the shape worth having. Two locals the layout has not placed
    /// yet are both written down as the same distance from the stack pointer, so a back end
    /// that compares the addresses it can see says they are one place, and they are two.
    StoreFold,
    /// A load, arithmetic against a constant, and a store of the answer to the same place.
    ///
    /// The same three instructions as `store-fold` with the second operand written down rather
    /// than in a register, which is the commoner half of the shape and a different instruction
    /// on the machine: it carries an addressing mode and an immediate at once and no register
    /// of its own, so a back end that has the register form still has work to do to reach it.
    /// `*p += 1` is here and `*p += x` is there, and neither one is evidence about the other.
    ///
    /// Eight types rather than the four at least as wide as `int`, because the widths are the
    /// point. C promotes a narrow operand to `int` before the arithmetic and narrows the answer
    /// back on the way into memory, so reaching the byte and the word form at all means a
    /// compiler worked out that the wide arithmetic in the middle can be thrown away. A
    /// compiler that never narrows is correct here and one instruction longer in every one of
    /// these programs, which is what the size column is for.
    ///
    /// The shapes are the five operations that have a memory destination, both arrangements of
    /// the subtraction, since a constant can only be the right hand one, a constant too big for
    /// the short form of the encoding, and then every reason the three are not one, which are
    /// the same reasons the register form has.
    StoreFoldConstant,
    /// A load and a comparison that reads it, against a register and against a constant.
    ///
    /// A facet of its own rather than a shape of `load-fold`, because a comparison is not
    /// arithmetic with a different name. It writes no value, only the answer to a question, and
    /// which side of it the memory is on decides the question rather than the operand order. On
    /// a machine that reads its right hand side out of memory, `x < *p` folds and keeps the
    /// condition it was written with, and `*p < x` folds and comes out as `x > *p`, so a back
    /// end that folds the left hand side without turning the condition over is the bug this
    /// facet is for. Equality is the pair that turns over into itself, which is why both
    /// arrangements of it are here: the wrong table and the right one agree on the name and
    /// disagree on nothing else.
    ///
    /// The comparison against a constant is the other half, and it is a different instruction
    /// again: an addressing mode and an immediate at once, no register on either side, and
    /// nothing to arrange either way round because the constant has nowhere else to be. The
    /// constant on the left is here too, since `10 < *p` is the same one instruction with the
    /// condition turned over and a back end that only looks at the right hand side misses it.
    ///
    /// Eight types rather than the four at least as wide as `int`. C promotes a narrow operand
    /// before comparing it, so reaching the byte and the word form means a compiler worked out
    /// that the comparison can be asked at the width the memory has. Against a constant that
    /// is always available, because the constant is narrowed with it; against a register it is
    /// only available when both sides came from the same width, which is why the two halves of
    /// this facet do not report the same numbers at the narrow types.
    ///
    /// The rest of the shapes are the two ways the answer is used, which are a byte and a
    /// branch and are two different instructions once the block layout has been through, and
    /// then the reasons the two are not one: the loaded word read twice, a store in the way, a
    /// call in the way, an index written in between, and the comparison in another block.
    CompareFold,
    /// One local, used at a counted number of offsets.
    ///
    /// A back end facet of its own rather than a shape of `address-fold`, because the question
    /// is a number rather than a yes or a no. An address into the frame is a distance from the
    /// stack pointer, so a reader that takes one costs more than a reader that takes an
    /// address in an ordinary register, and there is a count past which handing it to all of
    /// them is worse than working it out once. This is one program per count, so the count can
    /// be read off a family rather than inferred from a total.
    FrameAddress,
    /// Two things in the frame that are never both wanted, which may be the same bytes.
    ///
    /// A local whose last use is behind it and a value the allocator had to write out are both
    /// runs of bytes off the stack pointer, and a back end that lays every one of them out
    /// end to end takes a frame as large as all of them at once. One that shares takes a frame
    /// as large as the most it needs at any point, which is what these count. The shapes are
    /// the ones where the answer is yes and the ones where it is no, and the no shapes matter
    /// more: a local whose address got out is one that may be read through a pointer long
    /// after the name went out of use, and giving its bytes to something else there is a
    /// miscompilation rather than a frame that came out too big.
    StackSlots,
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
        Self::JumpThreading,
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
        Self::UnusedParams,
        Self::UnusedReturns,
        Self::DeclaredPurity,
        Self::Reachability,
        Self::Devirtualize,
        Self::MemoryEffects,
        Self::CallMotion,
        Self::LinkTimeOptimization,
        Self::Selection,
        Self::RegisterPressure,
        Self::RegisterAlloc,
        Self::Scheduling,
        Self::BlockLayout,
        Self::IfConversion,
        Self::SwitchLowering,
        Self::SwitchRuns,
        Self::SwitchDispatch,
        Self::CallingConvention,
        Self::MachinePeephole,
        Self::BitLiveness,
        Self::CompareElim,
        Self::AddressFold,
        Self::LoadFold,
        Self::StoreFold,
        Self::StoreFoldConstant,
        Self::CompareFold,
        Self::FrameAddress,
        Self::StackSlots,
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
            Self::JumpThreading => "jump-threading",
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
            Self::UnusedParams => "unused-params",
            Self::UnusedReturns => "unused-returns",
            Self::DeclaredPurity => "declared-purity",
            Self::Reachability => "reachability",
            Self::Devirtualize => "devirtualize",
            Self::MemoryEffects => "memory-effects",
            Self::CallMotion => "call-motion",
            Self::LinkTimeOptimization => "link-time-optimization",
            Self::Selection => "selection",
            Self::RegisterPressure => "register-pressure",
            Self::RegisterAlloc => "register-alloc",
            Self::Scheduling => "scheduling",
            Self::BlockLayout => "block-layout",
            Self::IfConversion => "if-conversion",
            Self::SwitchLowering => "switch-lowering",
            Self::SwitchRuns => "switch-runs",
            Self::SwitchDispatch => "switch-dispatch",
            Self::CallingConvention => "calling-convention",
            Self::MachinePeephole => "machine-peephole",
            Self::BitLiveness => "bit-liveness",
            Self::CompareElim => "compare-elim",
            Self::AddressFold => "address-fold",
            Self::LoadFold => "load-fold",
            Self::StoreFold => "store-fold",
            Self::StoreFoldConstant => "store-fold-constant",
            Self::CompareFold => "compare-fold",
            Self::FrameAddress => "frame-address",
            Self::StackSlots => "stack-slots",
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
            Self::JumpThreading => {
                "sending an edge that decides a branch below straight to its arm"
            }
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
            Self::UnusedParams => "taking out a parameter nothing reads, and the argument with it",
            Self::UnusedReturns => {
                "taking out a return value no call reads, and the result with it"
            }
            Self::DeclaredPurity => "believing a const or pure promise on a callee in another file",
            Self::Reachability => "removing what nothing references",
            Self::Devirtualize => "turning an indirect call into a direct one",
            Self::MemoryEffects => "what a callee reads and writes, seen from its call sites",
            Self::CallMotion => "whether a call came out of a loop",
            Self::LinkTimeOptimization => "optimizing across a translation unit boundary",
            Self::Selection => "choosing the machine instruction for an operation",
            Self::RegisterPressure => "how many values are live at once, and what that costs",
            Self::RegisterAlloc => "assigning registers and deciding what to spill",
            Self::Scheduling => "ordering instructions within a block",
            Self::BlockLayout => "laying out blocks so the common path falls through",
            Self::IfConversion => "turning a short branch into branchless code",
            Self::SwitchLowering => "choosing how to lower a switch",
            Self::SwitchRuns => "stretches of consecutive labels that share one arm",
            Self::SwitchDispatch => "a switch dispatched often enough to time how it was lowered",
            Self::CallingConvention => "deciding what a call saves and restores",
            Self::MachinePeephole => "the rules that only make sense on machine instructions",
            Self::BitLiveness => {
                "a widening whose upper bits nothing reads, across a block boundary"
            }
            Self::CompareElim => "a comparison the instruction in front of it has already made",
            Self::AddressFold => "whether an address is worked out once or carried by each reader",
            Self::LoadFold => "a load one arithmetic instruction reads, and what stops it moving",
            Self::StoreFold => "a load, arithmetic on it, and a store back to the same place",
            Self::StoreFoldConstant => {
                "a load, arithmetic against a constant, and a store back to the same place"
            }
            Self::CompareFold => {
                "a load and a comparison that reads it, on either side and against a constant"
            }
            Self::FrameAddress => "one local, used at a counted number of offsets",
            Self::StackSlots => "two things in the frame that may be the same bytes",
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
            | Self::JumpThreading
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
            | Self::UnusedParams
            | Self::UnusedReturns
            | Self::DeclaredPurity
            | Self::Reachability
            | Self::Devirtualize
            | Self::MemoryEffects
            | Self::CallMotion
            | Self::LinkTimeOptimization => Phase::Interprocedural,
            Self::Selection
            | Self::RegisterPressure
            | Self::RegisterAlloc
            | Self::Scheduling
            | Self::BlockLayout
            | Self::IfConversion
            | Self::SwitchLowering
            | Self::SwitchRuns
            | Self::SwitchDispatch
            | Self::CallingConvention
            | Self::MachinePeephole
            | Self::BitLiveness
            | Self::CompareElim
            | Self::AddressFold
            | Self::LoadFold
            | Self::StoreFold
            | Self::StoreFoldConstant
            | Self::CompareFold
            | Self::FrameAddress
            | Self::StackSlots
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
