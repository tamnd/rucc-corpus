# What it cost

Seven numbers per facet, each a median over the cases in that facet at the headline level, and each a ratio against the reference compiler building the same program on the same machine in the same run. A ratio below one is the compiler under test doing better.

| number | what it is | why it is separate |
|---|---|---|
| code | Allocated executable sections in the image | This is the code quality number. It is what the optimizer decided and nothing else. |
| on disk | The whole executable file | What a build costs somebody. Includes the runtime, the symbol table and whatever the linker padded with, none of which the optimizer chose. |
| data | Allocated initialized sections in the image | A compiler that unrolls by materializing a table and one that folds the loop away move the code column the same way and this column the opposite way. |
| compile | Wall clock for the build | Noisy and machine dependent. Worth watching between two commits of the compiler, not worth quoting on its own. |
| instructions | How many instructions this compiler's programs ran across the facet, less how many the reference's ran | The same question the run column asks, answered by a counter that repeats to within one part in a hundred thousand instead of by a clock that moves by a factor of twelve. A difference and not a ratio, because four fifths of every count is the process starting up and a constant on both sides subtracts away rather than sitting in a denominator. Missing on any machine that will not count. |
| run | Wall clock for the program, fastest of the repetitions | The fastest rather than the mean, because every slower measurement has somebody else's work in it and there is no way to subtract that. |
| memory | Largest high water mark in the compiler's process tree | Sampled from outside the process, so it is a floor rather than an exact peak, and it is missing entirely on any platform that is not Linux. |

None of these are averaged into a single figure for the corpus. A mean over fifty facets of wildly different shapes is a number with no referent.

The `lines` column is the odd one out, because it is not a ratio and not a cost. It is how much C the facet is, counted once per case however many compilers and levels the case was built with, and it is here because none of the six columns beside it can be read without it. Four hundred milliseconds is quick for ten thousand lines and slow for two hundred. It is a denominator and never a score, so there is deliberately no lines per second anywhere on this page.

## `rucc` against `gcc-16`

| facet | cases | lines | code | on disk | data | compile | instructions | run | memory |
|---|---|---|---|---|---|---|---|---|---|
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 2% less | level | 5% less | 54% less | not measured | inside the noise | 64% less |
| [`control-flow`](../programs/floor/control-flow/README.md) | 10 | 260 | 5% more | level | 5% less | 55% less | not measured | inside the noise | 60% less |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | level | level | 6% less | 45% less | not measured | inside the noise | 58% less |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | 19% more | level | 9% less | 48% less | not measured | inside the noise | 72% less |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 21% less | level | 11% less | 58% less | not measured | inside the noise | 74% less |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | 1% more | 13% more | 18% more | 48% less | not measured | inside the noise | 63% less |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 2% less | 17% more | 24% more | 58% less | not measured | inside the noise | 70% less |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | 2% less | 2% more | 3% less | 45% less | not measured | inside the noise | 62% less |
| [`simplify`](../programs/local/simplify/README.md) | 78 | 5,444 | 1% more | 5% more | 4% more | 48% less | not measured | inside the noise | 66% less |
| [`short-circuit`](../programs/local/short-circuit/README.md) | 22 | 599 | 31% less | 1% less | 11% less | 54% less | not measured | level | 68% less |
| [`conditional-store`](../programs/local/conditional-store/README.md) | 16 | 563 | 35% less | 1% less | 12% less | 54% less | not measured | level | 72% less |
| [`value-settled`](../programs/local/value-settled/README.md) | 13 | 372 | 22% less | 1% less | 11% less | 51% less | not measured | inside the noise | 65% less |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 3% more | level | 3% less | 46% less | not measured | inside the noise | 65% less |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 7% less | level | 6% less | 44% less | not measured | inside the noise | 30% less |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | 4% less | level | 6% less | 47% less | not measured | inside the noise | 35% less |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 7% less | level | 6% less | 50% less | not measured | inside the noise | 140% more |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 32 | 740 | 5% less | level | 6% less | 46% less | not measured | inside the noise | 57% less |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | level | level | 6% less | 42% less | not measured | inside the noise | 63% less |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 2% less | level | 6% less | 50% less | not measured | inside the noise | 64% less |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 8% less | level | 6% less | 49% less | not measured | inside the noise | 20% more |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 8% less | level | 6% less | 44% less | not measured | inside the noise | 63% less |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 7% more | level | 6% less | 49% less | not measured | inside the noise | 8% less |
| [`prune`](../programs/global/prune/README.md) | 15 | 472 | 26% less | level | 11% less | 49% less | not measured | inside the noise | 61% less |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 1% more | level | 6% less | 43% less | not measured | level | 61% less |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | 3% less | level | 6% less | 47% less | not measured | level | 65% less |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 4% less | level | 6% less | 48% less | not measured | inside the noise | 34% less |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 2% less | level | 6% less | 44% less | not measured | inside the noise | 63% less |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 9% more | level | 6% less | 49% less | not measured | inside the noise | 65% less |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 3% more | level | 5% less | 42% less | not measured | inside the noise | 65% less |
| [`iv-selection`](../programs/loops/iv-selection/README.md) | 36 | 760 | 20% less | 1% less | 11% less | 54% less | not measured | inside the noise | 65% less |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 5% more | level | 6% less | 47% less | not measured | inside the noise | 65% less |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 8% less | level | 6% less | 49% less | not measured | inside the noise | 64% less |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 9% less | level | 6% less | 50% less | not measured | inside the noise | 60% less |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 64 | 1,312 | 12% more | level | 9% less | 47% less | not measured | inside the noise | 64% less |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 24 | 516 | level | level | 6% less | 45% less | not measured | inside the noise | 60% less |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 9% less | level | 6% less | 46% less | not measured | inside the noise | 60% less |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 4% less | level | 5% less | 50% less | not measured | level | 64% less |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 34% more | level | 6% less | 44% less | not measured | inside the noise | 62% less |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 15% more | level | 3% less | 43% less | not measured | inside the noise | 60% less |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 9% more | level | level | 51% less | not measured | inside the noise | 61% less |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 20 | 440 | 12% more | level | 2% less | 43% less | not measured | inside the noise | 63% less |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 12 | 244 | 7% more | level | 3% less | 47% less | not measured | inside the noise | 60% less |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 7% less | level | 6% less | 48% less | not measured | inside the noise | level |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 1% less | 1% more | level | 49% less | not measured | inside the noise | 66% less |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 2% less | level | 4% less | 65% less | not measured | inside the noise | 49% less |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 6% less | level | 6% less | 50% less | not measured | inside the noise | 61% less |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 60 | 3,120 | 36% more | level | 3% less | 44% less | not measured | inside the noise | 62% less |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 62% more | level | 3% less | 46% less | not measured | inside the noise | 63% less |
| [`scheduling`](../programs/backend/scheduling/README.md) | 20 | 1,046 | 44% more | level | 6% less | 47% less | not measured | inside the noise | 65% less |
| [`block-layout`](../programs/backend/block-layout/README.md) | 16 | 421 | level | level | 4% less | 52% less | not measured | inside the noise | 63% less |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 20 | 444 | 3% less | level | 5% less | 48% less | not measured | inside the noise | 61% less |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 15 | 705 | 8% more | level | 3% less | 53% less | not measured | inside the noise | 65% less |
| [`switch-runs`](../programs/backend/switch-runs/README.md) | 15 | 1,356 | 10% more | level | 2% less | 49% less | not measured | inside the noise | 63% less |
| [`switch-dispatch`](../programs/backend/switch-dispatch/README.md) | 9 | 415 | 10% less | level | 8% less | 53% less | not measured | 116% more | level |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 18% more | level | 2% less | 43% less | not measured | inside the noise | 58% less |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 6% less | level | 6% less | 45% less | not measured | inside the noise | 63% less |
| [`bit-liveness`](../programs/backend/bit-liveness/README.md) | 28 | 628 | 2% less | level | 6% less | 46% less | not measured | inside the noise | 64% less |
| [`compare-elim`](../programs/backend/compare-elim/README.md) | 26 | 526 | 5% less | level | 6% less | 48% less | not measured | inside the noise | 64% less |
| [`address-fold`](../programs/backend/address-fold/README.md) | 28 | 584 | 8% more | level | 6% less | 49% less | not measured | level | 66% less |
| [`load-fold`](../programs/backend/load-fold/README.md) | 40 | 924 | 13% less | 1% less | 14% less | 46% less | not measured | inside the noise | 66% less |
| [`store-fold`](../programs/backend/store-fold/README.md) | 56 | 1,416 | 13% less | 1% less | 14% less | 47% less | not measured | level | 65% less |
| [`store-fold-constant`](../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 9% less | 1% less | 13% less | 48% less | not measured | level | 65% less |
| [`compare-fold`](../programs/backend/compare-fold/README.md) | 118 | 2,742 | 8% less | 1% less | 13% less | 47% less | not measured | inside the noise | 64% less |
| [`frame-address`](../programs/backend/frame-address/README.md) | 6 | 117 | 3% more | level | 6% less | 38% less | not measured | inside the noise | 66% less |
| [`stack-slots`](../programs/backend/stack-slots/README.md) | 6 | 201 | 5% less | 1% less | 14% less | 50% less | not measured | inside the noise | 64% less |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 83% more | 14% more | 9% more | 42% less | not measured | inside the noise | 60% less |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | 1% less | 2% more | 2% less | 44% less | not measured | inside the noise | 63% less |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 58% more | level | 4% less | 42% less | not measured | inside the noise | 33% less |
| [`barrier`](../programs/correctness/barrier/README.md) | 13 | 241 | 2% less | level | 5% less | 51% less | not measured | inside the noise | 65% less |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 8% more | 4% more | 3% more | 50% less | not measured | inside the noise | 61% less |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 11% less | level | 7% less | 44% less | not measured | inside the noise | 61% less |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | 4% less | level | 5% less | 49% less | not measured | inside the noise | 59% less |

2323 cases had a size to compare and 2329 had a memory figure.

