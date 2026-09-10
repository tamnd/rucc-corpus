# What it cost

Six numbers per facet, each a median over the cases in that facet at the headline level, and each a ratio against the reference compiler building the same program on the same machine in the same run. A ratio below one is the compiler under test doing better.

| number | what it is | why it is separate |
|---|---|---|
| code | Allocated executable sections in the image | This is the code quality number. It is what the optimizer decided and nothing else. |
| on disk | The whole executable file | What a build costs somebody. Includes the runtime, the symbol table and whatever the linker padded with, none of which the optimizer chose. |
| data | Allocated initialized sections in the image | A compiler that unrolls by materializing a table and one that folds the loop away move the code column the same way and this column the opposite way. |
| compile | Wall clock for the build | Noisy and machine dependent. Worth watching between two commits of the compiler, not worth quoting on its own. |
| run | Wall clock for the program, fastest of the repetitions | The fastest rather than the mean, because every slower measurement has somebody else's work in it and there is no way to subtract that. |
| memory | Largest high water mark in the compiler's process tree | Sampled from outside the process, so it is a floor rather than an exact peak, and it is missing entirely on any platform that is not Linux. |

None of these are averaged into a single figure for the corpus. A mean over fifty facets of wildly different shapes is a number with no referent.

The `lines` column is the odd one out, because it is not a ratio and not a cost. It is how much C the facet is, counted once per case however many compilers and levels the case was built with, and it is here because none of the six columns beside it can be read without it. Four hundred milliseconds is quick for ten thousand lines and slow for two hundred. It is a denominator and never a score, so there is deliberately no lines per second anywhere on this page.

## `rucc` against `gcc-16`

| facet | cases | lines | code | on disk | data | compile | run | memory |
|---|---|---|---|---|---|---|---|---|
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 15% more | level | 5% less | 48% less | 9% more | 58% less |
| [`control-flow`](../programs/floor/control-flow/README.md) | 10 | 260 | 18% more | level | 5% less | 51% less | 4% more | 39% less |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | 5% more | level | 6% less | 48% less | 2% more | 55% less |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | not measured | not measured | not measured | 92% less | not measured | 80% less |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | not measured | not measured | not measured | 89% less | not measured | level |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | 19% more | 13% more | 17% more | 53% less | 2% more | 64% less |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 11% more | 17% more | 22% more | 65% less | level | 71% less |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | 5% more | 2% more | 3% less | 48% less | level | 58% less |
| [`simplify`](../programs/local/simplify/README.md) | 69 | 4,595 | 17% more | 7% more | 7% more | 53% less | level | 56% less |
| [`short-circuit`](../programs/local/short-circuit/README.md) | 22 | 599 | 17% less | 1% less | 11% less | 57% less | 6% less | 71% less |
| [`conditional-store`](../programs/local/conditional-store/README.md) | 16 | 563 | 16% less | 1% less | 11% less | 60% less | 2% less | 72% less |
| [`value-settled`](../programs/local/value-settled/README.md) | 13 | 372 | 8% less | 1% less | 11% less | 59% less | 2% less | 68% less |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 13% more | level | 3% less | 48% less | 1% less | 54% less |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 5% less | level | 6% less | 47% less | 4% less | 57% less |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | 1% less | level | 6% less | 43% less | 3% more | 55% less |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 6% less | level | 6% less | 45% less | 16% less | 6% more |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 16 | 376 | 2% less | level | 6% less | 46% less | 11% less | 57% less |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | 4% more | level | 6% less | 46% less | 7% less | 55% less |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 1% more | level | 6% less | 47% less | 2% more | 56% less |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 7% less | level | 6% less | 52% less | 11% less | 58% less |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 5% less | level | 6% less | 43% less | 8% more | 54% less |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 18% more | level | 6% less | 47% less | 4% less | 57% less |
| [`prune`](../programs/global/prune/README.md) | 15 | 472 | 5% less | level | 11% less | 55% less | 13% less | 71% less |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 8% more | level | 6% less | 46% less | 2% less | 55% less |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | level | level | 6% less | 46% less | 2% less | 56% less |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 1% less | level | 6% less | 48% less | 2% more | 56% less |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 8% more | level | 6% less | 47% less | 3% more | 59% less |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 25% more | level | 5% less | 50% less | 1% more | 58% less |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 29% more | level | 4% less | 47% less | level | 60% less |
| [`iv-selection`](../programs/loops/iv-selection/README.md) | 28 | 584 | 5% less | 1% less | 12% less | 58% less | 6% more | 67% less |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 21% more | level | 6% less | 52% less | 5% less | 60% less |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 4% more | level | 5% less | 51% less | 2% less | 57% less |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 8% more | level | 6% less | 46% less | 1% more | 58% less |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 64 | 1,312 | 36% more | level | 8% less | 51% less | 3% less | 60% less |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 24 | 516 | 17% more | level | 4% less | 47% less | level | 58% less |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 9% more | level | 6% less | 47% less | 3% less | 57% less |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 10% more | level | 4% less | 53% less | 1% less | 60% less |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 118% more | level | 6% less | 49% less | 5% more | 61% less |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 17% more | level | 3% less | 46% less | level | 55% less |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 10% more | level | level | 53% less | 7% more | 59% less |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 20 | 440 | 16% more | level | 2% less | 48% less | 6% less | 60% less |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 12 | 244 | 10% more | level | 3% less | 47% less | level | 56% less |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 6% less | level | 6% less | 47% less | 8% less | 53% more |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 4% more | 1% more | 1% more | 53% less | 15% less | 27% less |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 1% more | level | 5% less | 82% less | 7% more | 55% less |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 5% less | level | 6% less | 48% less | 1% less | 55% less |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 48 | 2,460 | 51% more | level | 3% less | 47% less | 2% less | 58% less |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 75% more | level | 3% less | 49% less | 1% less | 57% less |
| [`scheduling`](../programs/backend/scheduling/README.md) | 12 | 832 | 74% more | level | 5% less | 50% less | 1% less | 62% less |
| [`block-layout`](../programs/backend/block-layout/README.md) | 4 | 91 | 2% more | level | 5% less | 49% less | 14% more | 64% less |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 20 | 444 | 8% more | level | 4% less | 55% less | level | 60% less |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 15 | 705 | 23% more | level | 3% less | 55% less | 2% more | 60% less |
| [`switch-runs`](../programs/backend/switch-runs/README.md) | 15 | 1,356 | 20% more | level | 2% less | 53% less | 12% less | 68% less |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 27% more | level | 2% less | 47% less | level | 33% less |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 4% less | level | 6% less | 46% less | 4% less | 55% less |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 282% more | 33% more | 8% more | 50% less | 2% more | 65% less |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | 29% more | 2% more | 2% less | 48% less | 1% less | 57% less |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 105% more | level | 5% less | 47% less | level | 30% less |
| [`barrier`](../programs/correctness/barrier/README.md) | 7 | 124 | 2% more | level | 5% less | 48% less | 12% less | 54% less |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 15% more | 4% more | 2% more | 56% less | 4% less | 65% less |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 7% less | level | 7% less | 47% less | 3% more | 58% less |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | level | level | 5% less | 47% less | 4% less | 55% less |

1777 cases had a size to compare and 1821 had a memory figure.

