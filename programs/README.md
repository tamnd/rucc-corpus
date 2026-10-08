# The corpus

3646 programs. Every one of them was written for exactly one named transformation, prints an answer this repository computed in Rust before any C was compiled, and prints nothing that depends on the machine it runs on. That last part is what lets one expected output be right everywhere.

These files are generated. Editing one here changes nothing, because the next run of `rucc-corpus gen` writes it back. The thing to edit is the generator in `crates/corpus-gen`, and the reason the output is kept in the repository anyway is so that a change to the generator shows up as a diff of the programs it produces.

Corpus digest `fc0c2efa3c9c0b1e4d7fed5b03cf0b66814a9bd836f42df0d305a4daf2dbd00a`.

## floor (118 programs)

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | programs | what it is about |
|---|---|---|
| [`baseline`](floor/baseline/README.md) | 10 | programs with no optimization target, which every level must get right |
| [`control-flow`](floor/control-flow/README.md) | 10 | the graph shapes the analyses under every pass have to get right |
| [`branch-probability`](floor/branch-probability/README.md) | 34 | the odds put on an edge before the program has ever run |
| [`computed-goto`](floor/computed-goto/README.md) | 25 | the address of a label, and the indirect jump through it |
| [`vla-and-alloca`](floor/vla-and-alloca/README.md) | 9 | an object whose size is not known until the program runs |
| [`frontend`](floor/frontend/README.md) | 30 | language shape rather than optimization, including C23 |

## local (413 programs)

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | programs | what it is about |
|---|---|---|
| [`constant-fold`](local/constant-fold/README.md) | 112 | folding an operation on constants into a constant |
| [`strength`](local/strength/README.md) | 24 | rewriting an operation into a cheaper one with the same value |
| [`narrowing`](local/narrowing/README.md) | 52 | taking the width back off arithmetic that C promoted |
| [`simplify`](local/simplify/README.md) | 114 | algebraic identities and the local peephole rules |
| [`short-circuit`](local/short-circuit/README.md) | 26 | collapsing the two branches of a logical operator into one |
| [`conditional-store`](local/conditional-store/README.md) | 20 | moving a store below a branch whose arms wrote the same place |
| [`value-settled`](local/value-settled/README.md) | 13 | a condition that settles the value its two arms disagree about |
| [`reassociate`](local/reassociate/README.md) | 20 | reassociating a chain to shorten its dependency height |
| [`dead-code`](local/dead-code/README.md) | 12 | removing a computation whose result nothing reads |
| [`dead-store`](local/dead-store/README.md) | 12 | removing a store a later store makes invisible |
| [`unreachable-code`](local/unreachable-code/README.md) | 8 | removing a branch with a known condition and its dead arm |

## global (242 programs)

Transformations across the blocks of one function, which need dataflow rather than a peephole.

| facet | programs | what it is about |
|---|---|---|
| [`common-subexpr`](global/common-subexpr/README.md) | 32 | reusing an earlier computation instead of repeating it |
| [`load-forwarding`](global/load-forwarding/README.md) | 40 | replacing a load with the value already in that place |
| [`code-motion`](global/code-motion/README.md) | 12 | moving a computation to where it runs no more often |
| [`copy-propagation`](global/copy-propagation/README.md) | 12 | propagating a copy so the copy becomes dead |
| [`constant-propagation`](global/constant-propagation/README.md) | 16 | propagating a value constant on every reaching path |
| [`value-range`](global/value-range/README.md) | 13 | narrowing an integer to the range it can hold |
| [`prune`](global/prune/README.md) | 15 | a branch or a switch case a condition above it has settled |
| [`jump-threading`](global/jump-threading/README.md) | 5 | sending an edge that decides a branch below straight to its arm |
| [`value-replacement`](global/value-replacement/README.md) | 13 | a branch whose condition already settles its value |
| [`alias-analysis`](global/alias-analysis/README.md) | 36 | proving two references cannot name the same object |
| [`memory-ssa`](global/memory-ssa/README.md) | 28 | walking back from a load to the store that answers it |
| [`scalar-replacement`](global/scalar-replacement/README.md) | 20 | turning a non-escaping local back into a value |

## loops (716 programs)

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | programs | what it is about |
|---|---|---|
| [`loop-invariant`](loops/loop-invariant/README.md) | 32 | hoisting an invariant computation out of a loop |
| [`loop-hoist`](loops/loop-hoist/README.md) | 64 | whether an invariant computation is allowed out of its loop |
| [`induction-variable`](loops/induction-variable/README.md) | 42 | rewriting induction variables into a cheaper set |
| [`iv-selection`](loops/iv-selection/README.md) | 36 | which induction variables a loop is left with, and how many |
| [`loop-unswitch`](loops/loop-unswitch/README.md) | 64 | removing a test the loop guard already decided |
| [`loop-unroll`](loops/loop-unroll/README.md) | 64 | unrolling a loop body, known trip count or not |
| [`loop-unroll-shape`](loops/loop-unroll-shape/README.md) | 80 | the loop shapes an unroller has to count or refuse |
| [`loop-idiom`](loops/loop-idiom/README.md) | 78 | recognizing a loop the runtime already implements |
| [`loop-deletion`](loops/loop-deletion/README.md) | 64 | deleting a loop whose body nobody reads |
| [`bit-loops`](loops/bit-loops/README.md) | 16 | loops that count bits, which some processors count in one instruction |
| [`loop-rotate`](loops/loop-rotate/README.md) | 64 | rotating a loop so the test lands at the bottom |
| [`loop-shape`](loops/loop-shape/README.md) | 64 | the shape a loop is left in, before any pass reads it |
| [`loop-restructure`](loops/loop-restructure/README.md) | 48 | exchanging or fusing loops for locality |

## interprocedural (351 programs)

Transformations that need to look at more than one function at a time.

| facet | programs | what it is about |
|---|---|---|
| [`inline`](interprocedural/inline/README.md) | 36 | replacing a call with the body of what it called |
| [`tail-call`](interprocedural/tail-call/README.md) | 36 | turning a call in tail position into a jump |
| [`tail-dispatch`](interprocedural/tail-dispatch/README.md) | 5 | calls in tail position made often enough to time whether they became jumps |
| [`called-once`](interprocedural/called-once/README.md) | 5 | a static function called from one place, timed in a hot loop |
| [`function-purity`](interprocedural/function-purity/README.md) | 28 | proving purity and using it at the call sites |
| [`constant-args`](interprocedural/constant-args/README.md) | 36 | specializing a function to a constant argument |
| [`unused-params`](interprocedural/unused-params/README.md) | 44 | taking out a parameter nothing reads, and the argument with it |
| [`unused-returns`](interprocedural/unused-returns/README.md) | 36 | taking out a return value no call reads, and the result with it |
| [`declared-purity`](interprocedural/declared-purity/README.md) | 32 | believing a const or pure promise on a callee in another file |
| [`reachability`](interprocedural/reachability/README.md) | 9 | removing what nothing references |
| [`devirtualize`](interprocedural/devirtualize/README.md) | 4 | turning an indirect call into a direct one |
| [`memory-effects`](interprocedural/memory-effects/README.md) | 24 | what a callee reads and writes, seen from its call sites |
| [`call-motion`](interprocedural/call-motion/README.md) | 36 | whether a call came out of a loop |
| [`link-time-optimization`](interprocedural/link-time-optimization/README.md) | 20 | optimizing across a translation unit boundary |

## backend (1026 programs)

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | programs | what it is about |
|---|---|---|
| [`selection`](backend/selection/README.md) | 40 | choosing the machine instruction for an operation |
| [`register-pressure`](backend/register-pressure/README.md) | 60 | how many values are live at once, and what that costs |
| [`register-alloc`](backend/register-alloc/README.md) | 40 | assigning registers and deciding what to spill |
| [`scheduling`](backend/scheduling/README.md) | 20 | ordering instructions within a block |
| [`block-layout`](backend/block-layout/README.md) | 16 | laying out blocks so the common path falls through |
| [`if-conversion`](backend/if-conversion/README.md) | 38 | turning a short branch into branchless code |
| [`switch-lowering`](backend/switch-lowering/README.md) | 27 | choosing how to lower a switch |
| [`switch-runs`](backend/switch-runs/README.md) | 15 | stretches of consecutive labels that share one arm |
| [`switch-dispatch`](backend/switch-dispatch/README.md) | 21 | a switch dispatched often enough to time how it was lowered |
| [`division`](backend/division/README.md) | 72 | a division by a constant done often enough to time how it was lowered |
| [`narrow-shift`](backend/narrow-shift/README.md) | 8 | a shift or rotate of a byte or a short by a count below its width |
| [`calling-convention`](backend/calling-convention/README.md) | 10 | deciding what a call saves and restores |
| [`machine-peephole`](backend/machine-peephole/README.md) | 22 | the rules that only make sense on machine instructions |
| [`bit-liveness`](backend/bit-liveness/README.md) | 28 | a widening whose upper bits nothing reads, across a block boundary |
| [`compare-elim`](backend/compare-elim/README.md) | 26 | a comparison the instruction in front of it has already made |
| [`address-fold`](backend/address-fold/README.md) | 28 | whether an address is worked out once or carried by each reader |
| [`load-fold`](backend/load-fold/README.md) | 40 | a load one arithmetic instruction reads, and what stops it moving |
| [`store-fold`](backend/store-fold/README.md) | 56 | a load, arithmetic on it, and a store back to the same place |
| [`store-fold-constant`](backend/store-fold-constant/README.md) | 118 | a load, arithmetic against a constant, and a store back to the same place |
| [`compare-fold`](backend/compare-fold/README.md) | 118 | a load and a comparison that reads it, on either side and against a constant |
| [`frame-address`](backend/frame-address/README.md) | 6 | one local, used at a counted number of offsets |
| [`stack-slots`](backend/stack-slots/README.md) | 6 | two things in the frame that may be the same bytes |
| [`frame-size`](backend/frame-size/README.md) | 59 | large frames with a simple answer, to hold against gcc -fstack-usage |
| [`interpreter-dispatch`](backend/interpreter-dispatch/README.md) | 54 | a threaded bytecode interpreter with many values alive across every dispatch |
| [`bit-builtins`](backend/bit-builtins/README.md) | 22 | the bit counting builtins, over every position at both widths |
| [`float-conversion`](backend/float-conversion/README.md) | 66 | conversions between the floating types and the integer ones |
| [`long-double`](backend/long-double/README.md) | 10 | the widest floating type, whose shape the target rather than C decides |

## correctness (780 programs)

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | programs | what it is about |
|---|---|---|
| [`barrier`](correctness/barrier/README.md) | 13 | programs where the compiler must not act, and a firing is a bug |
| [`atomics`](correctness/atomics/README.md) | 31 | the atomic builtins at every ordering, and the header over them |
| [`setjmp-longjmp`](correctness/setjmp-longjmp/README.md) | 8 | the jump that leaves a function without returning from it |
| [`sigsetjmp`](correctness/sigsetjmp/README.md) | 112 | locals live across a sigsetjmp the way PG_TRY and PG_CATCH use it |
| [`builtin-setjmp`](correctness/builtin-setjmp/README.md) | 48 | locals live across a __builtin_setjmp the way MinGW builds of Postgres use it |
| [`overflow-builtins`](correctness/overflow-builtins/README.md) | 144 | the checked add, subtract and multiply builtins over mixed integer types |
| [`target-attribute`](correctness/target-attribute/README.md) | 52 | functions built for an extension and called only after asking the processor |
| [`crc32c`](correctness/crc32c/README.md) | 64 | CRC-32C through the SSE4.2 instructions against slicing by eight |
| [`simd-lfind`](correctness/simd-lfind/README.md) | 30 | the SSE2 search loops from port/simd.h and pg_lfind.h |
| [`crc32c-armv8`](correctness/crc32c-armv8/README.md) | 72 | CRC-32C through the ARMv8 CRC instructions against slicing by eight |
| [`simd-lfind-neon`](correctness/simd-lfind-neon/README.md) | 48 | the NEON search loops from port/simd.h and pg_lfind.h |
| [`lkmm`](correctness/lkmm/README.md) | 24 | the access and store rules the Linux kernel memory model assumes |
| [`mitigations`](correctness/mitigations/README.md) | 25 | programs built with the kernel's thunk, trap and landing pad flags |
| [`asm-goto`](correctness/asm-goto/README.md) | 6 | asm goto as static keys and user copies use it, with and without outputs |
| [`asm-local-labels`](correctness/asm-local-labels/README.md) | 5 | numeric labels, the %= number and a label another section names |
| [`gas-macros`](correctness/gas-macros/README.md) | 6 | assembler macros, .rept, .irp and .if used from inline asm |
| [`constant-p-after-inline`](correctness/constant-p-after-inline/README.md) | 5 | __builtin_constant_p answered after inlining, with the other branch unlinkable |
| [`mcmodel-kernel`](correctness/mcmodel-kernel/README.md) | 6 | programs built for the kernel code model and linked without PIE |
| [`general-regs-only`](correctness/general-regs-only/README.md) | 5 | programs built with -mgeneral-regs-only that check no vector register moved |
| [`objtool-shapes`](correctness/objtool-shapes/README.md) | 5 | the code shapes objtool follows through a kernel object |
| [`null-pointer-constant`](correctness/null-pointer-constant/README.md) | 6 | __is_constexpr, is_const and the null pointer constants they rest on |
| [`const-ice`](correctness/const-ice/README.md) | 8 | sizeof, offsetof and const objects in integer constant expressions |
| [`section-attr`](correctness/section-attr/README.md) | 6 | functions and data in named sections, walked from __start_ to __stop_ |
| [`bundle`](correctness/bundle/README.md) | 33 | modules loaded with dlopen that call back into the executable |
| [`dllimport`](correctness/dllimport/README.md) | 18 | modules importing the executable's functions and data the way Windows Postgres does |

