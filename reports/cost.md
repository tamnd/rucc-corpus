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
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 15% more | level | 8% less | 52% less | level | 52% less |
| [`control-flow`](../programs/floor/control-flow/README.md) | 10 | 260 | 19% more | level | 9% less | 52% less | 6% more | 59% less |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | 5% more | level | 9% less | 49% less | 2% less | 57% less |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | not measured | not measured | not measured | 91% less | not measured | 83% less |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | not measured | not measured | not measured | 91% less | not measured | 91% less |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | 19% more | 13% more | 15% more | 52% less | 1% more | 58% less |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 12% more | 17% more | 19% more | 65% less | 3% more | 70% less |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | 7% more | 2% more | 6% less | 51% less | level | 57% less |
| [`simplify`](../programs/local/simplify/README.md) | 69 | 4,595 | 19% more | 7% more | 4% more | 53% less | level | 67% less |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 15% more | level | 8% less | 49% less | 1% less | 56% less |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 5% less | level | 8% less | 48% less | 2% more | 56% less |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | level | level | 8% less | 48% less | 3% less | 55% less |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 5% less | level | 9% less | 49% less | 2% more | 42% more |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 16 | 376 | level | level | 9% less | 50% less | 3% less | 57% less |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | 5% more | level | 9% less | 49% less | 3% less | 58% less |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 4% more | level | 8% less | 51% less | level | 55% less |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 6% less | level | 8% less | 47% less | 2% less | 54% less |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 4% less | level | 9% less | 48% less | 4% more | 57% less |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 18% more | level | 8% less | 50% less | 6% less | 59% less |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 9% more | level | 9% less | 49% less | 1% more | 57% less |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | 2% more | level | 9% less | 48% less | level | 58% less |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 1% less | level | 8% less | 48% less | level | 59% less |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 10% more | level | 8% less | 50% less | 2% more | 59% less |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 56 | 1,328 | 28% more | level | 8% less | 51% less | 3% less | 59% less |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 32% more | level | 8% less | 48% less | 1% less | 60% less |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 21% more | level | 9% less | 55% less | 2% more | 66% less |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 5% more | level | 9% less | 51% less | 3% more | 59% less |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 8% more | level | 9% less | 48% less | 1% more | 59% less |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 64 | 1,312 | 43% more | level | 12% less | 53% less | 2% more | 63% less |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 24 | 516 | 31% more | level | 9% less | 51% less | 1% less | 59% less |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 9% more | level | 9% less | 48% less | 1% less | 58% less |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 11% more | level | 9% less | 54% less | 4% more | 60% less |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 124% more | level | 9% less | 51% less | 1% less | 63% less |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 17% more | level | 8% less | 49% less | 1% less | 57% less |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 11% more | level | 9% less | 53% less | level | 60% less |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 20 | 440 | 17% more | level | 8% less | 51% less | 1% more | 60% less |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 12 | 244 | 11% more | level | 8% less | 49% less | level | 58% less |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 5% less | level | 9% less | 48% less | 1% more | 57% less |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 6% more | 1% more | 6% less | 51% less | 16% more | 28% less |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 2% more | level | 11% less | 83% less | 5% more | 55% less |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 2% less | level | 8% less | 47% less | level | 56% less |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 48 | 2,460 | 52% more | level | 9% less | 51% less | 2% more | 58% less |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 76% more | level | 8% less | 51% less | 4% less | 57% less |
| [`scheduling`](../programs/backend/scheduling/README.md) | 12 | 832 | 75% more | level | 8% less | 48% less | 8% more | 74% less |
| [`block-layout`](../programs/backend/block-layout/README.md) | 4 | 91 | 3% more | level | 9% less | 54% less | 5% less | 64% less |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 20 | 444 | 10% more | level | 8% less | 55% less | 3% more | 66% less |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 9 | 369 | 33% more | level | 9% less | 56% less | 4% more | 71% less |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 27% more | level | 8% less | 51% less | 3% more | 55% less |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 2% less | level | 8% less | 49% less | 1% less | 56% less |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 296% more | 33% more | 5% more | 48% less | 5% less | 63% less |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | 30% more | 2% more | 5% less | 51% less | 3% more | 57% less |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 110% more | level | 9% less | 52% less | 1% less | 58% less |
| [`barrier`](../programs/correctness/barrier/README.md) | 7 | 124 | 3% more | level | 8% less | 46% less | 2% more | 53% less |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 24% more | 4% more | 3% less | 57% less | 2% more | 67% less |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 6% less | level | 9% less | 46% less | 7% less | 65% less |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | level | level | 8% less | 48% less | 6% less | 58% less |

1648 cases had a size to compare and 1699 had a memory figure.

