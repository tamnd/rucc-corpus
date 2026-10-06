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
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 1% less | 62% less | 15% less | 37% less | not measured | inside the noise | 135% more |
| [`control-flow`](../programs/floor/control-flow/README.md) | 10 | 260 | level | 62% less | 16% less | 38% less | not measured | inside the noise | 39% more |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | 3% less | 63% less | 16% less | 31% less | not measured | inside the noise | 142% more |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | 22% less | 61% less | 19% less | 44% less | not measured | inside the noise | 84% more |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 23% less | 62% less | 20% less | 50% less | not measured | inside the noise | 86% more |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | level | 56% less | 14% less | 38% less | not measured | inside the noise | 95% more |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 4% more | 48% less | 16% less | 56% less | not measured | inside the noise | 8% more |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | level | 61% less | 15% less | 35% less | not measured | inside the noise | 169% more |
| [`simplify`](../programs/local/simplify/README.md) | 114 | 6,416 | 1% less | 59% less | 15% less | 44% less | not measured | level | 78% more |
| [`short-circuit`](../programs/local/short-circuit/README.md) | 24 | 650 | 39% less | 62% less | 22% less | 44% less | not measured | inside the noise | 80% more |
| [`conditional-store`](../programs/local/conditional-store/README.md) | 20 | 705 | 37% less | 62% less | 22% less | 47% less | not measured | inside the noise | 62% more |
| [`value-settled`](../programs/local/value-settled/README.md) | 13 | 372 | 27% less | 62% less | 22% less | 43% less | not measured | inside the noise | 83% more |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 4% more | 62% less | 13% less | 32% less | not measured | inside the noise | 160% more |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 5% less | 63% less | 16% less | 32% less | not measured | inside the noise | 184% more |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | 5% less | 63% less | 16% less | 31% less | not measured | inside the noise | 624% more |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 5% less | 63% less | 16% less | 32% less | not measured | inside the noise | 172% more |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 32 | 740 | 5% less | 62% less | 16% less | 36% less | not measured | inside the noise | 173% more |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | 5% less | 63% less | 16% less | 32% less | not measured | inside the noise | 165% more |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 2% less | 62% less | 15% less | 30% less | not measured | inside the noise | 177% more |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 6% less | 63% less | 16% less | 31% less | not measured | level | 174% more |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 5% less | 63% less | 16% less | 33% less | not measured | level | 87% more |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 7% more | 62% less | 15% less | 31% less | not measured | level | 169% more |
| [`prune`](../programs/global/prune/README.md) | 15 | 472 | 34% less | 62% less | 22% less | 43% less | not measured | inside the noise | 81% more |
| [`jump-threading`](../programs/global/jump-threading/README.md) | 5 | 224 | 14% less | 61% less | 22% less | 46% less | not measured | inside the noise | 61% more |
| [`value-replacement`](../programs/global/value-replacement/README.md) | 13 | 360 | 32% less | 62% less | 22% less | 42% less | not measured | inside the noise | 101% more |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 4% less | 63% less | 16% less | 33% less | not measured | inside the noise | level |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | 4% less | 63% less | 16% less | 32% less | not measured | inside the noise | 171% more |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 4% less | 63% less | 16% less | 29% less | not measured | inside the noise | 174% more |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 4% less | 63% less | 16% less | 33% less | not measured | inside the noise | 65% more |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 3% less | 62% less | 15% less | 35% less | not measured | inside the noise | 92% more |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 5% less | 63% less | 16% less | 32% less | not measured | inside the noise | 135% more |
| [`iv-selection`](../programs/loops/iv-selection/README.md) | 36 | 760 | 26% less | 62% less | 22% less | 44% less | not measured | inside the noise | 74% more |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 1% less | 63% less | 16% less | 38% less | not measured | level | 103% more |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 7% less | 63% less | 16% less | 38% less | not measured | inside the noise | 119% more |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 7% less | 63% less | 16% less | 32% less | not measured | inside the noise | 162% more |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 28% less | 63% less | 21% less | 36% less | not measured | inside the noise | 135% more |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 5% less | 63% less | 16% less | 33% less | not measured | inside the noise | 161% more |
| [`bit-loops`](../programs/loops/bit-loops/README.md) | 16 | 564 | 1% more | 50% less | 12% less | 50% less | not measured | inside the noise | level |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 7% less | 63% less | 16% less | 32% less | not measured | level | 163% more |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 6% less | 63% less | 16% less | 39% less | not measured | level | 99% more |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 7% less | 63% less | 16% less | 33% less | not measured | inside the noise | 109% more |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 3% less | 63% less | 15% less | 32% less | not measured | inside the noise | 173% more |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 10% more | 62% less | 14% less | 37% less | not measured | inside the noise | 152% more |
| [`tail-dispatch`](../programs/interprocedural/tail-dispatch/README.md) | 5 | 893 | 4% more | 58% less | 14% less | 54% less | not measured | inside the noise | 16% more |
| [`called-once`](../programs/interprocedural/called-once/README.md) | 5 | 413 | 1% more | 61% less | 15% less | 54% less | not measured | 15% more | 113% more |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 28 | 636 | 5% less | 63% less | 16% less | 34% less | not measured | inside the noise | 121% more |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 36 | 792 | 5% less | 63% less | 16% less | 33% less | not measured | inside the noise | 167% more |
| [`unused-params`](../programs/interprocedural/unused-params/README.md) | 44 | 948 | 5% less | 63% less | 16% less | 32% less | not measured | inside the noise | 147% more |
| [`unused-returns`](../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 4% less | 62% less | 15% less | 33% less | not measured | inside the noise | 146% more |
| [`declared-purity`](../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 12% less | 62% less | 18% less | 49% less | not measured | inside the noise | 133% more |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 5% less | 63% less | 16% less | 36% less | not measured | inside the noise | 8% more |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 3% less | 62% less | 15% less | 30% less | not measured | inside the noise | 135% more |
| [`memory-effects`](../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 4% less | 61% less | 15% less | 41% less | not measured | inside the noise | 69% more |
| [`call-motion`](../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | 4% less | 62% less | 14% less | 42% less | not measured | inside the noise | 100% more |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 4% less | 62% less | 14% less | 58% less | not measured | inside the noise | 927% more |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 5% less | 63% less | 16% less | 34% less | not measured | inside the noise | 180% more |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 60 | 3,120 | 2% less | 63% less | 16% less | 33% less | not measured | inside the noise | 164% more |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 1% less | 63% less | 16% less | 35% less | not measured | inside the noise | 133% more |
| [`scheduling`](../programs/backend/scheduling/README.md) | 20 | 1,046 | 42% more | 61% less | 15% less | 35% less | not measured | inside the noise | 83% more |
| [`block-layout`](../programs/backend/block-layout/README.md) | 16 | 421 | 2% less | 61% less | 14% less | 40% less | not measured | inside the noise | 124% more |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 38 | 945 | 9% less | 62% less | 15% less | 39% less | not measured | inside the noise | 91% more |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 27 | 1,521 | level | 61% less | 16% less | 38% less | not measured | inside the noise | level |
| [`switch-runs`](../programs/backend/switch-runs/README.md) | 15 | 1,356 | 4% less | 62% less | 15% less | 39% less | not measured | inside the noise | 2% more |
| [`switch-dispatch`](../programs/backend/switch-dispatch/README.md) | 21 | 942 | 22% less | 62% less | 21% less | 44% less | not measured | inside the noise | level |
| [`division`](../programs/backend/division/README.md) | 72 | 2,020 | 18% less | 62% less | 15% less | 49% less | not measured | inside the noise | 84% more |
| [`narrow-shift`](../programs/backend/narrow-shift/README.md) | 8 | 256 | 38% less | 61% less | 16% less | 54% less | not measured | inside the noise | 84% less |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 3% less | 63% less | 16% less | 32% less | not measured | level | 126% more |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 4% less | 63% less | 16% less | 34% less | not measured | inside the noise | 175% more |
| [`bit-liveness`](../programs/backend/bit-liveness/README.md) | 28 | 628 | 2% less | 63% less | 16% less | 36% less | not measured | level | 20% more |
| [`compare-elim`](../programs/backend/compare-elim/README.md) | 26 | 526 | 3% less | 63% less | 16% less | 35% less | not measured | inside the noise | 164% more |
| [`address-fold`](../programs/backend/address-fold/README.md) | 28 | 584 | 4% less | 63% less | 16% less | 31% less | not measured | level | 163% more |
| [`load-fold`](../programs/backend/load-fold/README.md) | 40 | 924 | 14% less | 62% less | 22% less | 37% less | not measured | inside the noise | 85% more |
| [`store-fold`](../programs/backend/store-fold/README.md) | 56 | 1,416 | 15% less | 62% less | 22% less | 37% less | not measured | level | 83% more |
| [`store-fold-constant`](../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 12% less | 62% less | 22% less | 37% less | not measured | level | 118% more |
| [`compare-fold`](../programs/backend/compare-fold/README.md) | 118 | 2,742 | 11% less | 62% less | 22% less | 37% less | not measured | inside the noise | 82% more |
| [`frame-address`](../programs/backend/frame-address/README.md) | 6 | 117 | 4% less | 63% less | 16% less | 32% less | not measured | inside the noise | 176% more |
| [`stack-slots`](../programs/backend/stack-slots/README.md) | 6 | 201 | 34% less | 63% less | 24% less | 43% less | not measured | inside the noise | 47% more |
| [`frame-size`](../programs/backend/frame-size/README.md) | 59 | 8,808 | 26% less | 55% less | 21% less | 65% less | not measured | inside the noise | 50% more |
| [`interpreter-dispatch`](../programs/backend/interpreter-dispatch/README.md) | 36 | 20,442 | 61% more | 24% less | 11% less | 58% less | not measured | inside the noise | 4% less |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 19% more | 55% less | 14% less | 44% less | not measured | inside the noise | 67% more |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | 6% more | 60% less | 15% less | 34% less | not measured | inside the noise | 97% more |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 49% more | 61% less | 16% less | 33% less | not measured | inside the noise | 163% more |
| [`barrier`](../programs/correctness/barrier/README.md) | 13 | 241 | 4% less | 63% less | 16% less | 29% less | not measured | inside the noise | 170% more |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 1% more | 59% less | 16% less | 41% less | not measured | inside the noise | 110% more |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 9% less | 61% less | 16% less | 36% less | not measured | inside the noise | 42% less |
| [`sigsetjmp`](../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 2% more | 54% less | 21% less | 50% less | not measured | inside the noise | 37% more |
| [`builtin-setjmp`](../programs/correctness/builtin-setjmp/README.md) | 48 | 5,660 | 8% more | 54% less | 23% less | 57% less | not measured | inside the noise | 19% more |
| [`overflow-builtins`](../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 56% more | 40% less | 15% less | 46% less | not measured | inside the noise | 65% more |
| [`target-attribute`](../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 13% more | 50% less | 12% less | 43% less | not measured | inside the noise | 35% more |
| [`crc32c`](../programs/correctness/crc32c/README.md) | 64 | 15,632 | 4% more | 40% less | 11% less | 17% less | not measured | inside the noise | level |
| [`simd-lfind`](../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 85% more | 51% less | 16% less | 45% less | not measured | inside the noise | 46% more |
| [`crc32c-armv8`](../programs/correctness/crc32c-armv8/README.md) | 72 | 25,314 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`simd-lfind-neon`](../programs/correctness/simd-lfind-neon/README.md) | 48 | 4,044 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`lkmm`](../programs/correctness/lkmm/README.md) | 24 | 1,164 | 14% less | 60% less | 16% less | 31% less | not measured | inside the noise | 39% more |
| [`mitigations`](../programs/correctness/mitigations/README.md) | 25 | 1,510 | level | 59% less | 13% less | 42% less | not measured | inside the noise | 85% more |
| [`asm-goto`](../programs/correctness/asm-goto/README.md) | 6 | 172 | 5% less | 62% less | 15% less | 35% less | not measured | inside the noise | 121% more |
| [`asm-local-labels`](../programs/correctness/asm-local-labels/README.md) | 5 | 135 | 1% more | 62% less | 15% less | 39% less | not measured | inside the noise | 88% more |
| [`gas-macros`](../programs/correctness/gas-macros/README.md) | 6 | 151 | 6% less | 62% less | 15% less | 38% less | not measured | inside the noise | 92% more |
| [`constant-p-after-inline`](../programs/correctness/constant-p-after-inline/README.md) | 5 | 168 | 4% less | 62% less | 15% less | 37% less | not measured | inside the noise | 145% more |
| [`mcmodel-kernel`](../programs/correctness/mcmodel-kernel/README.md) | 6 | 165 | 2% less | 63% less | 10% less | 35% less | not measured | inside the noise | 110% more |
| [`general-regs-only`](../programs/correctness/general-regs-only/README.md) | 5 | 463 | 4% less | 59% less | 17% less | 43% less | not measured | inside the noise | 38% more |
| [`objtool-shapes`](../programs/correctness/objtool-shapes/README.md) | 5 | 170 | level | 61% less | 15% less | 39% less | not measured | inside the noise | 82% more |
| [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md) | 6 | 302 | 1% more | 61% less | 15% less | 42% less | not measured | inside the noise | 93% more |
| [`const-ice`](../programs/correctness/const-ice/README.md) | 8 | 408 | 1% less | 61% less | 15% less | 43% less | not measured | level | 83% more |
| [`section-attr`](../programs/correctness/section-attr/README.md) | 6 | 320 | 3% less | 58% less | 14% less | 46% less | not measured | inside the noise | 81% more |
| [`bundle`](../programs/correctness/bundle/README.md) | 33 | 3,651 in 78 files | level | 58% less | 12% less | 41% less | not measured | inside the noise | level |
| [`dllimport`](../programs/correctness/dllimport/README.md) | 18 | 2,268 in 39 files | 4% less | 59% less | 10% less | 38% less | not measured | inside the noise | level |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | 5% less | 63% less | 16% less | 35% less | not measured | inside the noise | 174% more |

3497 cases had a size to compare and 3498 had a memory figure.

