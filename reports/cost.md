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
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 3% more | 61% less | 15% less | 33% less | not measured | inside the noise | level |
| [`control-flow`](../programs/floor/control-flow/README.md) | 10 | 260 | 5% more | 62% less | 15% less | 37% less | not measured | inside the noise | 62% more |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | 2% less | 63% less | 16% less | 32% less | not measured | inside the noise | 166% more |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | 18% less | 60% less | 19% less | 41% less | not measured | inside the noise | 90% more |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 20% less | 61% less | 20% less | 49% less | not measured | inside the noise | 9% less |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | 2% more | 40% less | 9% more | 39% less | not measured | inside the noise | 99% more |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 7% more | 27% less | 13% more | 56% less | not measured | inside the noise | 77% more |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | level | 59% less | 13% less | 35% less | not measured | inside the noise | 172% more |
| [`simplify`](../programs/local/simplify/README.md) | 114 | 6,416 | 1% more | 54% less | 6% less | 44% less | not measured | inside the noise | 74% more |
| [`short-circuit`](../programs/local/short-circuit/README.md) | 22 | 599 | 32% less | 62% less | 20% less | 45% less | not measured | inside the noise | 75% more |
| [`conditional-store`](../programs/local/conditional-store/README.md) | 20 | 705 | 31% less | 61% less | 21% less | 45% less | not measured | inside the noise | 66% more |
| [`value-settled`](../programs/local/value-settled/README.md) | 13 | 372 | 21% less | 62% less | 20% less | 44% less | not measured | inside the noise | 46% more |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 4% more | 61% less | 13% less | 34% less | not measured | inside the noise | 121% more |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 5% less | 63% less | 16% less | 33% less | not measured | inside the noise | 148% more |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | 1% less | 63% less | 16% less | 29% less | not measured | level | 183% more |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 5% less | 63% less | 16% less | 33% less | not measured | inside the noise | 71% less |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 32 | 740 | 4% less | 62% less | 16% less | 34% less | not measured | inside the noise | 158% more |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | level | 62% less | 16% less | 31% less | not measured | inside the noise | 162% more |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 2% less | 62% less | 16% less | 33% less | not measured | inside the noise | 163% more |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 6% less | 63% less | 16% less | 36% less | not measured | level | 175% more |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 5% less | 63% less | 16% less | 34% less | not measured | level | 165% more |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 10% more | 62% less | 15% less | 30% less | not measured | inside the noise | 171% more |
| [`prune`](../programs/global/prune/README.md) | 15 | 472 | 28% less | 61% less | 20% less | 42% less | not measured | inside the noise | 77% more |
| [`jump-threading`](../programs/global/jump-threading/README.md) | 5 | 224 | 17% less | 60% less | 19% less | 47% less | not measured | inside the noise | 30% more |
| [`value-replacement`](../programs/global/value-replacement/README.md) | 13 | 360 | 26% less | 62% less | 21% less | 45% less | not measured | 96% more | 82% more |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 2% more | 62% less | 16% less | 34% less | not measured | inside the noise | 156% more |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | 1% less | 62% less | 16% less | 32% less | not measured | inside the noise | 155% more |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 2% less | 63% less | 16% less | 31% less | not measured | inside the noise | 147% more |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 1% more | 62% less | 15% less | 33% less | not measured | inside the noise | 153% more |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 8% more | 62% less | 15% less | 35% less | not measured | inside the noise | 151% more |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 4% more | 62% less | 15% less | 33% less | not measured | inside the noise | 154% more |
| [`iv-selection`](../programs/loops/iv-selection/README.md) | 36 | 760 | 18% less | 62% less | 20% less | 44% less | not measured | level | 79% more |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | level | 62% less | 16% less | 40% less | not measured | inside the noise | 102% more |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 7% less | 62% less | 15% less | 37% less | not measured | inside the noise | 105% more |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 7% less | 63% less | 16% less | 32% less | not measured | inside the noise | 153% more |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 11% more | 62% less | 20% less | 38% less | not measured | level | 86% more |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 4% less | 63% less | 16% less | 34% less | not measured | level | 149% more |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 7% less | 63% less | 16% less | 34% less | not measured | inside the noise | 161% more |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 2% less | 62% less | 14% less | 38% less | not measured | level | 99% more |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 41% more | 61% less | 16% less | 33% less | not measured | inside the noise | 93% more |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 17% more | 62% less | 12% less | 35% less | not measured | level | 174% more |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 10% more | 62% less | 12% less | 34% less | not measured | inside the noise | 103% more |
| [`tail-dispatch`](../programs/interprocedural/tail-dispatch/README.md) | 5 | 893 | 15% more | 57% less | 11% less | 52% less | not measured | inside the noise | 90% more |
| [`called-once`](../programs/interprocedural/called-once/README.md) | 5 | 413 | 4% more | 61% less | 15% less | 54% less | not measured | 22% more | 38% more |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 28 | 636 | level | 62% less | 15% less | 32% less | not measured | inside the noise | 153% more |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 36 | 792 | level | 62% less | 14% less | 35% less | not measured | level | 166% more |
| [`unused-params`](../programs/interprocedural/unused-params/README.md) | 44 | 948 | 1% more | 62% less | 14% less | 36% less | not measured | inside the noise | 170% more |
| [`unused-returns`](../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 2% less | 62% less | 13% less | 37% less | not measured | inside the noise | 168% more |
| [`declared-purity`](../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 10% less | 62% less | 18% less | 50% less | not measured | inside the noise | 157% more |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 5% less | 63% less | 16% less | 32% less | not measured | inside the noise | 150% more |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 1% more | 60% less | 9% less | 38% less | not measured | inside the noise | 84% more |
| [`memory-effects`](../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 1% less | 60% less | 13% less | 37% less | not measured | inside the noise | 85% more |
| [`call-motion`](../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | level | 61% less | 14% less | 43% less | not measured | inside the noise | 93% more |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | level | 61% less | 13% less | 60% less | not measured | inside the noise | 192% more |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 5% less | 62% less | 16% less | 34% less | not measured | inside the noise | 177% more |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 60 | 3,120 | 30% more | 61% less | 13% less | 33% less | not measured | inside the noise | 155% more |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 64% more | 61% less | 13% less | 36% less | not measured | inside the noise | 133% more |
| [`scheduling`](../programs/backend/scheduling/README.md) | 20 | 1,046 | 43% more | 61% less | 16% less | 37% less | not measured | inside the noise | 113% more |
| [`block-layout`](../programs/backend/block-layout/README.md) | 16 | 421 | 1% less | 61% less | 13% less | 40% less | not measured | inside the noise | 107% more |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 38 | 945 | 8% less | 62% less | 14% less | 39% less | not measured | inside the noise | 84% more |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 27 | 1,521 | 4% more | 60% less | 13% less | 43% less | not measured | inside the noise | level |
| [`switch-runs`](../programs/backend/switch-runs/README.md) | 15 | 1,356 | 4% more | 61% less | 13% less | 38% less | not measured | inside the noise | 87% more |
| [`switch-dispatch`](../programs/backend/switch-dispatch/README.md) | 21 | 942 | 15% less | 61% less | 19% less | 41% less | not measured | inside the noise | 63% more |
| [`division`](../programs/backend/division/README.md) | 72 | 2,020 | 15% less | 61% less | 15% less | 50% less | not measured | inside the noise | 81% more |
| [`narrow-shift`](../programs/backend/narrow-shift/README.md) | 8 | 256 | 33% less | 61% less | 16% less | 56% less | not measured | inside the noise | 68% more |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 11% more | 62% less | 15% less | 31% less | not measured | inside the noise | 165% more |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 4% less | 63% less | 16% less | 32% less | not measured | inside the noise | 172% more |
| [`bit-liveness`](../programs/backend/bit-liveness/README.md) | 28 | 628 | 1% less | 62% less | 16% less | 32% less | not measured | inside the noise | 163% more |
| [`compare-elim`](../programs/backend/compare-elim/README.md) | 26 | 526 | 3% less | 62% less | 16% less | 35% less | not measured | inside the noise | 162% more |
| [`address-fold`](../programs/backend/address-fold/README.md) | 28 | 584 | 10% more | 62% less | 15% less | 33% less | not measured | inside the noise | 169% more |
| [`load-fold`](../programs/backend/load-fold/README.md) | 40 | 924 | 11% less | 62% less | 22% less | 39% less | not measured | inside the noise | 159% more |
| [`store-fold`](../programs/backend/store-fold/README.md) | 56 | 1,416 | 11% less | 62% less | 22% less | 38% less | not measured | inside the noise | 88% more |
| [`store-fold-constant`](../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 9% less | 62% less | 22% less | 38% less | not measured | inside the noise | 113% more |
| [`compare-fold`](../programs/backend/compare-fold/README.md) | 118 | 2,742 | 8% less | 62% less | 22% less | 38% less | not measured | inside the noise | 127% more |
| [`frame-address`](../programs/backend/frame-address/README.md) | 6 | 117 | 5% more | 62% less | 15% less | 31% less | not measured | inside the noise | 183% more |
| [`stack-slots`](../programs/backend/stack-slots/README.md) | 6 | 201 | 4% less | 61% less | 23% less | 45% less | not measured | level | 2% less |
| [`frame-size`](../programs/backend/frame-size/README.md) | 54 | 8,520 | 6% less | 50% less | 20% less | 66% less | not measured | inside the noise | 26% more |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 83% more | 38% less | level | 39% less | not measured | inside the noise | 54% more |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | level | 58% less | 11% less | 34% less | not measured | inside the noise | 153% more |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 49% more | 59% less | 15% less | 38% less | not measured | inside the noise | 92% more |
| [`barrier`](../programs/correctness/barrier/README.md) | 13 | 241 | 1% less | 62% less | 15% less | 32% less | not measured | inside the noise | 172% more |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 3% more | 54% less | 7% less | 42% less | not measured | inside the noise | 94% more |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 7% less | 60% less | 15% less | 35% less | not measured | inside the noise | 128% more |
| [`sigsetjmp`](../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 26% more | 50% less | 16% less | 50% less | not measured | level | 50% more |
| [`overflow-builtins`](../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 109% more | 27% less | 4% more | 42% less | not measured | level | 66% more |
| [`target-attribute`](../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 24% more | 43% less | 6% less | 51% less | not measured | inside the noise | 37% more |
| [`crc32c`](../programs/correctness/crc32c/README.md) | 64 | 15,632 | 8% more | 33% less | 6% more | 59% less | not measured | inside the noise | 2% more |
| [`simd-lfind`](../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 188% more | 42% less | 11% less | 46% less | not measured | inside the noise | 27% more |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | 4% less | 62% less | 15% less | 36% less | not measured | inside the noise | 174% more |

3232 cases had a size to compare and 3233 had a memory figure.

