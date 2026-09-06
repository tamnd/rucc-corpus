# The corpus

1119 programs. Every one of them was written for exactly one named transformation, prints an answer this repository computed in Rust before any C was compiled, and prints nothing that depends on the machine it runs on. That last part is what lets one expected output be right everywhere.

These files are generated. Editing one here changes nothing, because the next run of `rucc-corpus gen` writes it back. The thing to edit is the generator in `crates/corpus-gen`, and the reason the output is kept in the repository anyway is so that a change to the generator shows up as a diff of the programs it produces.

Corpus digest `c3a615a2df67001d46d2e9b342cd8d034c093993a40f825220573a835b9d3ff1`.

## floor (50 programs)

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | programs | what it is about |
|---|---|---|
| [`baseline`](floor/baseline/index.md) | 10 | programs with no optimization target, which every level must get right |
| [`control-flow`](floor/control-flow/index.md) | 10 | the graph shapes the analyses under every pass have to get right |
| [`frontend`](floor/frontend/index.md) | 30 | language shape rather than optimization, including C23 |

## local (221 programs)

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | programs | what it is about |
|---|---|---|
| [`constant-fold`](local/constant-fold/index.md) | 112 | folding an operation on constants into a constant |
| [`strength`](local/strength/index.md) | 24 | rewriting an operation into a cheaper one with the same value |
| [`narrowing`](local/narrowing/index.md) | 28 | taking the width back off arithmetic that C promoted |
| [`simplify`](local/simplify/index.md) | 8 | algebraic identities and the local peephole rules |
| [`reassociate`](local/reassociate/index.md) | 20 | reassociating a chain to shorten its dependency height |
| [`dead-code`](local/dead-code/index.md) | 12 | removing a computation whose result nothing reads |
| [`dead-store`](local/dead-store/index.md) | 12 | removing a store a later store makes invisible |
| [`unreachable-code`](local/unreachable-code/index.md) | 5 | removing a branch with a known condition and its dead arm |

## global (165 programs)

Transformations across the blocks of one function, which need dataflow rather than a peephole.

| facet | programs | what it is about |
|---|---|---|
| [`common-subexpr`](global/common-subexpr/index.md) | 16 | reusing an earlier computation instead of repeating it |
| [`load-forwarding`](global/load-forwarding/index.md) | 40 | replacing a load with the value already in that place |
| [`code-motion`](global/code-motion/index.md) | 12 | moving a computation to where it runs no more often |
| [`copy-propagation`](global/copy-propagation/index.md) | 12 | propagating a copy so the copy becomes dead |
| [`constant-propagation`](global/constant-propagation/index.md) | 16 | propagating a value constant on every reaching path |
| [`value-range`](global/value-range/index.md) | 13 | narrowing an integer to the range it can hold |
| [`alias-analysis`](global/alias-analysis/index.md) | 36 | proving two references cannot name the same object |
| [`scalar-replacement`](global/scalar-replacement/index.md) | 20 | turning a non-escaping local back into a value |

## loops (402 programs)

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | programs | what it is about |
|---|---|---|
| [`loop-invariant`](loops/loop-invariant/index.md) | 32 | hoisting an invariant computation out of a loop |
| [`induction-variable`](loops/induction-variable/index.md) | 42 | rewriting induction variables into a cheaper set |
| [`loop-unswitch`](loops/loop-unswitch/index.md) | 64 | removing a test the loop guard already decided |
| [`loop-unroll`](loops/loop-unroll/index.md) | 64 | unrolling a loop body, known trip count or not |
| [`loop-idiom`](loops/loop-idiom/index.md) | 64 | recognizing a loop the runtime already implements |
| [`loop-deletion`](loops/loop-deletion/index.md) | 24 | deleting a loop whose body nobody reads |
| [`loop-rotate`](loops/loop-rotate/index.md) | 64 | rotating a loop so the test lands at the bottom |
| [`loop-restructure`](loops/loop-restructure/index.md) | 48 | exchanging or fusing loops for locality |

## interprocedural (117 programs)

Transformations that need to look at more than one function at a time.

| facet | programs | what it is about |
|---|---|---|
| [`inline`](interprocedural/inline/index.md) | 36 | replacing a call with the body of what it called |
| [`tail-call`](interprocedural/tail-call/index.md) | 36 | turning a call in tail position into a jump |
| [`function-purity`](interprocedural/function-purity/index.md) | 20 | proving purity and using it at the call sites |
| [`constant-args`](interprocedural/constant-args/index.md) | 12 | specializing a function to a constant argument |
| [`reachability`](interprocedural/reachability/index.md) | 9 | removing what nothing references |
| [`devirtualize`](interprocedural/devirtualize/index.md) | 4 | turning an indirect call into a direct one |

## backend (157 programs)

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | programs | what it is about |
|---|---|---|
| [`selection`](backend/selection/index.md) | 40 | choosing the machine instruction for an operation |
| [`register-alloc`](backend/register-alloc/index.md) | 40 | assigning registers and deciding what to spill |
| [`scheduling`](backend/scheduling/index.md) | 12 | ordering instructions within a block |
| [`block-layout`](backend/block-layout/index.md) | 4 | laying out blocks so the common path falls through |
| [`if-conversion`](backend/if-conversion/index.md) | 20 | turning a short branch into branchless code |
| [`switch-lowering`](backend/switch-lowering/index.md) | 9 | choosing how to lower a switch |
| [`calling-convention`](backend/calling-convention/index.md) | 10 | deciding what a call saves and restores |
| [`machine-peephole`](backend/machine-peephole/index.md) | 22 | the rules that only make sense on machine instructions |

## correctness (7 programs)

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | programs | what it is about |
|---|---|---|
| [`barrier`](correctness/barrier/index.md) | 7 | programs where the compiler must not act, and a firing is a bug |

