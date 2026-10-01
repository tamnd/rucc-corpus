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
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 1% less | 62% less | 15% less | 33% less | not measured | inside the noise | 146% more |
| [`control-flow`](../programs/floor/control-flow/README.md) | 10 | 260 | 3% more | 62% less | 16% less | 35% less | not measured | inside the noise | 4% more |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | 2% less | 63% less | 16% less | 33% less | not measured | inside the noise | 102% more |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | 19% less | 60% less | 20% less | 42% less | not measured | inside the noise | 77% more |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 22% less | 62% less | 20% less | 47% less | not measured | inside the noise | 78% more |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | level | 56% less | 14% less | 42% less | not measured | inside the noise | 109% more |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 6% more | 48% less | 16% less | 57% less | not measured | inside the noise | 10% more |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | level | 61% less | 15% less | 37% less | not measured | inside the noise | 157% more |
| [`simplify`](../programs/local/simplify/README.md) | 114 | 6,416 | 1% more | 59% less | 16% less | 43% less | not measured | inside the noise | 88% more |
| [`short-circuit`](../programs/local/short-circuit/README.md) | 24 | 650 | 36% less | 62% less | 21% less | 45% less | not measured | inside the noise | 73% more |
| [`conditional-store`](../programs/local/conditional-store/README.md) | 20 | 705 | 34% less | 62% less | 21% less | 50% less | not measured | inside the noise | 64% more |
| [`value-settled`](../programs/local/value-settled/README.md) | 13 | 372 | 25% less | 62% less | 21% less | 47% less | not measured | inside the noise | 79% more |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 4% more | 62% less | 14% less | 36% less | not measured | inside the noise | 70% more |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 5% less | 63% less | 16% less | 34% less | not measured | inside the noise | 179% more |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | 5% less | 63% less | 16% less | 34% less | not measured | inside the noise | 181% more |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 5% less | 63% less | 16% less | 31% less | not measured | inside the noise | 166% more |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 32 | 740 | 4% less | 62% less | 16% less | 37% less | not measured | inside the noise | 72% more |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | 5% less | 63% less | 16% less | 32% less | not measured | level | 79% more |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 2% less | 62% less | 16% less | 32% less | not measured | inside the noise | 26% more |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 6% less | 63% less | 16% less | 32% less | not measured | inside the noise | 188% more |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 5% less | 63% less | 16% less | 34% less | not measured | inside the noise | 106% more |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 7% more | 62% less | 15% less | 32% less | not measured | inside the noise | 4% more |
| [`prune`](../programs/global/prune/README.md) | 15 | 472 | 31% less | 62% less | 21% less | 41% less | not measured | level | 81% more |
| [`jump-threading`](../programs/global/jump-threading/README.md) | 5 | 224 | 13% less | 61% less | 20% less | 48% less | not measured | inside the noise | 494% more |
| [`value-replacement`](../programs/global/value-replacement/README.md) | 13 | 360 | 28% less | 62% less | 21% less | 44% less | not measured | 122% more | 22% less |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 3% less | 63% less | 16% less | 34% less | not measured | inside the noise | level |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | 4% less | 63% less | 16% less | 35% less | not measured | inside the noise | 2% more |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 4% less | 63% less | 16% less | 32% less | not measured | inside the noise | 60% more |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 1% more | 62% less | 15% less | 35% less | not measured | inside the noise | 150% more |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 7% more | 62% less | 15% less | 38% less | not measured | inside the noise | 146% more |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 5% less | 63% less | 16% less | 37% less | not measured | level | 145% more |
| [`iv-selection`](../programs/loops/iv-selection/README.md) | 36 | 760 | 21% less | 62% less | 21% less | 45% less | not measured | inside the noise | 68% more |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 1% less | 63% less | 16% less | 37% less | not measured | level | 95% more |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 7% less | 63% less | 16% less | 37% less | not measured | inside the noise | 128% more |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 7% less | 63% less | 16% less | 33% less | not measured | level | 159% more |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 23% less | 63% less | 20% less | 40% less | not measured | inside the noise | 89% more |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 4% less | 63% less | 16% less | 35% less | not measured | inside the noise | 154% more |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 7% less | 63% less | 16% less | 34% less | not measured | inside the noise | 152% more |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 3% less | 62% less | 15% less | 40% less | not measured | inside the noise | 121% more |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 35% more | 61% less | 16% less | 37% less | not measured | inside the noise | 94% more |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 17% more | 62% less | 12% less | 35% less | not measured | inside the noise | 172% more |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 3% more | 62% less | 14% less | 40% less | not measured | inside the noise | 52% more |
| [`tail-dispatch`](../programs/interprocedural/tail-dispatch/README.md) | 5 | 893 | 6% more | 58% less | 13% less | 55% less | not measured | inside the noise | 662% more |
| [`called-once`](../programs/interprocedural/called-once/README.md) | 5 | 413 | 4% more | 61% less | 15% less | 61% less | not measured | 54% more | level |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 28 | 636 | level | 62% less | 15% less | 35% less | not measured | level | 64% more |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 36 | 792 | 1% more | 62% less | 14% less | 39% less | not measured | inside the noise | 154% more |
| [`unused-params`](../programs/interprocedural/unused-params/README.md) | 44 | 948 | 1% more | 62% less | 14% less | 34% less | not measured | level | 143% more |
| [`unused-returns`](../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 2% less | 62% less | 13% less | 38% less | not measured | inside the noise | 130% more |
| [`declared-purity`](../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 10% less | 62% less | 18% less | 49% less | not measured | inside the noise | level |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 5% less | 63% less | 16% less | 27% less | not measured | inside the noise | 174% more |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | level | 61% less | 14% less | 37% less | not measured | inside the noise | 18% more |
| [`memory-effects`](../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 1% less | 61% less | 14% less | 41% less | not measured | inside the noise | 91% more |
| [`call-motion`](../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | 3% less | 62% less | 14% less | 45% less | not measured | inside the noise | 123% more |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 2% less | 62% less | 14% less | 62% less | not measured | inside the noise | 234% more |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 5% less | 63% less | 16% less | 35% less | not measured | inside the noise | 174% more |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 60 | 3,120 | 30% more | 61% less | 13% less | 33% less | not measured | inside the noise | 166% more |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 64% more | 61% less | 13% less | 35% less | not measured | level | 119% more |
| [`scheduling`](../programs/backend/scheduling/README.md) | 20 | 1,046 | 43% more | 61% less | 16% less | 37% less | not measured | inside the noise | 139% more |
| [`block-layout`](../programs/backend/block-layout/README.md) | 16 | 421 | 1% less | 61% less | 13% less | 39% less | not measured | inside the noise | 96% more |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 38 | 945 | 9% less | 62% less | 15% less | 37% less | not measured | inside the noise | level |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 27 | 1,521 | 1% more | 61% less | 13% less | 45% less | not measured | inside the noise | 76% more |
| [`switch-runs`](../programs/backend/switch-runs/README.md) | 15 | 1,356 | 3% more | 61% less | 14% less | 38% less | not measured | inside the noise | level |
| [`switch-dispatch`](../programs/backend/switch-dispatch/README.md) | 21 | 942 | 18% less | 61% less | 20% less | 40% less | not measured | 40% more | 86% more |
| [`division`](../programs/backend/division/README.md) | 72 | 2,020 | 16% less | 62% less | 15% less | 52% less | not measured | inside the noise | level |
| [`narrow-shift`](../programs/backend/narrow-shift/README.md) | 8 | 256 | 34% less | 61% less | 16% less | 54% less | not measured | inside the noise | 55% more |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 9% more | 62% less | 16% less | 32% less | not measured | inside the noise | 16% less |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 4% less | 63% less | 16% less | 32% less | not measured | level | 157% more |
| [`bit-liveness`](../programs/backend/bit-liveness/README.md) | 28 | 628 | 2% less | 63% less | 16% less | 37% less | not measured | inside the noise | 120% more |
| [`compare-elim`](../programs/backend/compare-elim/README.md) | 26 | 526 | 3% less | 63% less | 16% less | 35% less | not measured | inside the noise | 149% more |
| [`address-fold`](../programs/backend/address-fold/README.md) | 28 | 584 | 2% less | 63% less | 16% less | 35% less | not measured | inside the noise | 106% more |
| [`load-fold`](../programs/backend/load-fold/README.md) | 40 | 924 | 10% less | 62% less | 22% less | 39% less | not measured | inside the noise | 157% more |
| [`store-fold`](../programs/backend/store-fold/README.md) | 56 | 1,416 | 11% less | 62% less | 22% less | 37% less | not measured | inside the noise | 145% more |
| [`store-fold-constant`](../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 8% less | 62% less | 22% less | 37% less | not measured | level | 145% more |
| [`compare-fold`](../programs/backend/compare-fold/README.md) | 118 | 2,742 | 7% less | 62% less | 22% less | 39% less | not measured | inside the noise | 146% more |
| [`frame-address`](../programs/backend/frame-address/README.md) | 6 | 117 | level | 63% less | 16% less | 35% less | not measured | inside the noise | 8% less |
| [`stack-slots`](../programs/backend/stack-slots/README.md) | 6 | 201 | 9% less | 61% less | 22% less | 44% less | not measured | inside the noise | 92% more |
| [`frame-size`](../programs/backend/frame-size/README.md) | 59 | 8,808 | 8% less | 51% less | 20% less | 69% less | not measured | inside the noise | 42% more |
| [`interpreter-dispatch`](../programs/backend/interpreter-dispatch/README.md) | 36 | 20,442 | 186% more | 5% more | 11% less | 52% less | not measured | inside the noise | level |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 87% more | 55% less | 15% less | 43% less | not measured | inside the noise | 59% more |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | 6% more | 61% less | 16% less | 36% less | not measured | inside the noise | 157% more |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 49% more | 61% less | 16% less | 36% less | not measured | inside the noise | 30% more |
| [`barrier`](../programs/correctness/barrier/README.md) | 13 | 241 | 2% less | 63% less | 15% less | 36% less | not measured | inside the noise | 137% more |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 2% more | 59% less | 15% less | 42% less | not measured | inside the noise | 90% more |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 9% less | 61% less | 16% less | 32% less | not measured | inside the noise | 84% more |
| [`sigsetjmp`](../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 18% more | 53% less | 20% less | 52% less | not measured | level | 25% more |
| [`overflow-builtins`](../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 108% more | 39% less | 16% less | 45% less | not measured | inside the noise | 56% more |
| [`target-attribute`](../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 9% more | 53% less | 10% less | 51% less | not measured | inside the noise | 29% more |
| [`crc32c`](../programs/correctness/crc32c/README.md) | 64 | 15,632 | 3% more | 39% less | 7% less | 57% less | not measured | inside the noise | 11% more |
| [`simd-lfind`](../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 107% more | 49% less | 19% less | 49% less | not measured | inside the noise | 11% more |
| [`crc32c-armv8`](../programs/correctness/crc32c-armv8/README.md) | 72 | 25,314 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`simd-lfind-neon`](../programs/correctness/simd-lfind-neon/README.md) | 48 | 4,044 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`lkmm`](../programs/correctness/lkmm/README.md) | 24 | 1,164 | 13% less | 60% less | 15% less | 34% less | not measured | inside the noise | 292% more |
| [`mitigations`](../programs/correctness/mitigations/README.md) | 25 | 1,510 | 1% more | 59% less | 13% less | 44% less | not measured | inside the noise | 62% more |
| [`asm-goto`](../programs/correctness/asm-goto/README.md) | 6 | 172 | not measured | not measured | not measured | 50% less | not measured | not measured | 77% less |
| [`asm-local-labels`](../programs/correctness/asm-local-labels/README.md) | 5 | 135 | not measured | not measured | not measured | 59% less | not measured | not measured | 94% less |
| [`gas-macros`](../programs/correctness/gas-macros/README.md) | 6 | 151 | not measured | not measured | not measured | 60% less | not measured | not measured | 6056% more |
| [`constant-p-after-inline`](../programs/correctness/constant-p-after-inline/README.md) | 5 | 168 | 4% less | 62% less | 15% less | 41% less | not measured | inside the noise | 157% more |
| [`mcmodel-kernel`](../programs/correctness/mcmodel-kernel/README.md) | 6 | 165 | 2% less | 63% less | 11% less | 40% less | not measured | inside the noise | 95% more |
| [`general-regs-only`](../programs/correctness/general-regs-only/README.md) | 5 | 463 | not measured | not measured | not measured | 41% less | not measured | not measured | 1622% more |
| [`objtool-shapes`](../programs/correctness/objtool-shapes/README.md) | 5 | 170 | 3% more | 61% less | 15% less | 48% less | not measured | inside the noise | 634% more |
| [`bundle`](../programs/correctness/bundle/README.md) | 33 | 3,651 in 78 files | 6% less | 58% less | 8% less | 43% less | not measured | inside the noise | level |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | 5% less | 63% less | 16% less | 33% less | not measured | inside the noise | level |

3373 cases had a size to compare and 3396 had a memory figure.

