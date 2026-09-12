# The corpus

1984 programs. Every one of them was written for exactly one named transformation, prints an answer this repository computed in Rust before any C was compiled, and prints nothing that depends on the machine it runs on. That last part is what lets one expected output be right everywhere.

These files are generated. Editing one here changes nothing, because the next run of `rucc-corpus gen` writes it back. The thing to edit is the generator in `crates/corpus-gen`, and the reason the output is kept in the repository anyway is so that a change to the generator shows up as a diff of the programs it produces.

Corpus digest `6f8a28857eaf78402b1a95a673179a2c887aa6cd033418c19944ed2241109c88`.

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

## local (368 programs)

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | programs | what it is about |
|---|---|---|
| [`constant-fold`](local/constant-fold/README.md) | 112 | folding an operation on constants into a constant |
| [`strength`](local/strength/README.md) | 24 | rewriting an operation into a cheaper one with the same value |
| [`narrowing`](local/narrowing/README.md) | 52 | taking the width back off arithmetic that C promoted |
| [`simplify`](local/simplify/README.md) | 77 | algebraic identities and the local peephole rules |
| [`short-circuit`](local/short-circuit/README.md) | 22 | collapsing the two branches of a logical operator into one |
| [`conditional-store`](local/conditional-store/README.md) | 16 | moving a store below a branch whose arms wrote the same place |
| [`value-settled`](local/value-settled/README.md) | 13 | a condition that settles the value its two arms disagree about |
| [`reassociate`](local/reassociate/README.md) | 20 | reassociating a chain to shorten its dependency height |
| [`dead-code`](local/dead-code/README.md) | 12 | removing a computation whose result nothing reads |
| [`dead-store`](local/dead-store/README.md) | 12 | removing a store a later store makes invisible |
| [`unreachable-code`](local/unreachable-code/README.md) | 8 | removing a branch with a known condition and its dead arm |

## global (224 programs)

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
| [`alias-analysis`](global/alias-analysis/README.md) | 36 | proving two references cannot name the same object |
| [`memory-ssa`](global/memory-ssa/README.md) | 28 | walking back from a load to the store that answers it |
| [`scalar-replacement`](global/scalar-replacement/README.md) | 20 | turning a non-escaping local back into a value |

## loops (646 programs)

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
| [`loop-idiom`](loops/loop-idiom/README.md) | 64 | recognizing a loop the runtime already implements |
| [`loop-deletion`](loops/loop-deletion/README.md) | 24 | deleting a loop whose body nobody reads |
| [`loop-rotate`](loops/loop-rotate/README.md) | 64 | rotating a loop so the test lands at the bottom |
| [`loop-shape`](loops/loop-shape/README.md) | 64 | the shape a loop is left in, before any pass reads it |
| [`loop-restructure`](loops/loop-restructure/README.md) | 48 | exchanging or fusing loops for locality |

## interprocedural (137 programs)

Transformations that need to look at more than one function at a time.

| facet | programs | what it is about |
|---|---|---|
| [`inline`](interprocedural/inline/README.md) | 36 | replacing a call with the body of what it called |
| [`tail-call`](interprocedural/tail-call/README.md) | 36 | turning a call in tail position into a jump |
| [`function-purity`](interprocedural/function-purity/README.md) | 20 | proving purity and using it at the call sites |
| [`constant-args`](interprocedural/constant-args/README.md) | 12 | specializing a function to a constant argument |
| [`reachability`](interprocedural/reachability/README.md) | 9 | removing what nothing references |
| [`devirtualize`](interprocedural/devirtualize/README.md) | 4 | turning an indirect call into a direct one |
| [`link-time-optimization`](interprocedural/link-time-optimization/README.md) | 20 | optimizing across a translation unit boundary |

## backend (445 programs)

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | programs | what it is about |
|---|---|---|
| [`selection`](backend/selection/README.md) | 40 | choosing the machine instruction for an operation |
| [`register-pressure`](backend/register-pressure/README.md) | 60 | how many values are live at once, and what that costs |
| [`register-alloc`](backend/register-alloc/README.md) | 40 | assigning registers and deciding what to spill |
| [`scheduling`](backend/scheduling/README.md) | 12 | ordering instructions within a block |
| [`block-layout`](backend/block-layout/README.md) | 16 | laying out blocks so the common path falls through |
| [`if-conversion`](backend/if-conversion/README.md) | 20 | turning a short branch into branchless code |
| [`switch-lowering`](backend/switch-lowering/README.md) | 15 | choosing how to lower a switch |
| [`switch-runs`](backend/switch-runs/README.md) | 15 | stretches of consecutive labels that share one arm |
| [`switch-dispatch`](backend/switch-dispatch/README.md) | 9 | a switch dispatched often enough to time how it was lowered |
| [`calling-convention`](backend/calling-convention/README.md) | 10 | deciding what a call saves and restores |
| [`machine-peephole`](backend/machine-peephole/README.md) | 22 | the rules that only make sense on machine instructions |
| [`bit-liveness`](backend/bit-liveness/README.md) | 28 | a widening whose upper bits nothing reads, across a block boundary |
| [`compare-elim`](backend/compare-elim/README.md) | 26 | a comparison the instruction in front of it has already made |
| [`address-fold`](backend/address-fold/README.md) | 28 | whether an address is worked out once or carried by each reader |
| [`frame-address`](backend/frame-address/README.md) | 6 | one local, used at a counted number of offsets |
| [`bit-builtins`](backend/bit-builtins/README.md) | 22 | the bit counting builtins, over every position at both widths |
| [`float-conversion`](backend/float-conversion/README.md) | 66 | conversions between the floating types and the integer ones |
| [`long-double`](backend/long-double/README.md) | 10 | the widest floating type, whose shape the target rather than C decides |

## correctness (46 programs)

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | programs | what it is about |
|---|---|---|
| [`barrier`](correctness/barrier/README.md) | 7 | programs where the compiler must not act, and a firing is a bug |
| [`atomics`](correctness/atomics/README.md) | 31 | the atomic builtins at every ordering, and the header over them |
| [`setjmp-longjmp`](correctness/setjmp-longjmp/README.md) | 8 | the jump that leaves a function without returning from it |

