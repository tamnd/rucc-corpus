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
| [`baseline`](../programs/floor/baseline/README.md) | 10 | 241 | 3% less | level | 4% less | 41% less | not measured | inside the noise | 19% less |
| [`control-flow`](../programs/floor/control-flow/README.md) | 11 | 291 | 6% less | level | 4% less | 43% less | not measured | level | 18% less |
| [`branch-probability`](../programs/floor/branch-probability/README.md) | 34 | 704 | 6% less | level | 4% less | 40% less | not measured | level | 19% less |
| [`computed-goto`](../programs/floor/computed-goto/README.md) | 25 | 1,614 | 27% less | level | 9% less | 49% less | not measured | inside the noise | 22% less |
| [`vla-and-alloca`](../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 27% less | level | 9% less | 60% less | not measured | inside the noise | 46% less |
| [`constant-fold`](../programs/local/constant-fold/README.md) | 112 | 6,438 | 2% less | level | 4% less | 46% less | not measured | level | 32% less |
| [`strength`](../programs/local/strength/README.md) | 24 | 2,656 | 3% more | level | 5% less | 61% less | not measured | inside the noise | 40% less |
| [`narrowing`](../programs/local/narrowing/README.md) | 52 | 2,204 | 1% less | level | 4% less | 43% less | not measured | inside the noise | 15% less |
| [`simplify`](../programs/local/simplify/README.md) | 114 | 6,416 | 2% less | level | 4% less | 50% less | not measured | level | 36% less |
| [`short-circuit`](../programs/local/short-circuit/README.md) | 26 | 704 | 41% less | level | 11% less | 52% less | not measured | inside the noise | 42% less |
| [`conditional-store`](../programs/local/conditional-store/README.md) | 20 | 705 | 42% less | level | 11% less | 55% less | not measured | inside the noise | 45% less |
| [`value-settled`](../programs/local/value-settled/README.md) | 13 | 372 | 32% less | level | 11% less | 50% less | not measured | inside the noise | 44% less |
| [`reassociate`](../programs/local/reassociate/README.md) | 20 | 440 | 2% more | level | 2% less | 41% less | not measured | inside the noise | 15% less |
| [`dead-code`](../programs/local/dead-code/README.md) | 12 | 264 | 6% less | level | 4% less | 40% less | not measured | inside the noise | 17% less |
| [`dead-store`](../programs/local/dead-store/README.md) | 12 | 220 | 6% less | level | 4% less | 40% less | not measured | inside the noise | 31% less |
| [`unreachable-code`](../programs/local/unreachable-code/README.md) | 8 | 162 | 6% less | level | 4% less | 43% less | not measured | inside the noise | 19% less |
| [`libcall-fold`](../programs/local/libcall-fold/README.md) | 20 | 488 | 6% less | level | 4% less | 46% less | not measured | inside the noise | 16% less |
| [`common-subexpr`](../programs/global/common-subexpr/README.md) | 32 | 740 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 17% less |
| [`load-forwarding`](../programs/global/load-forwarding/README.md) | 40 | 1,008 | 6% less | level | 4% less | 38% less | not measured | inside the noise | 16% less |
| [`code-motion`](../programs/global/code-motion/README.md) | 12 | 288 | 6% less | level | 3% less | 39% less | not measured | inside the noise | 16% less |
| [`copy-propagation`](../programs/global/copy-propagation/README.md) | 12 | 336 | 8% less | level | 4% less | 36% less | not measured | inside the noise | 15% less |
| [`constant-propagation`](../programs/global/constant-propagation/README.md) | 16 | 268 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 17% less |
| [`value-range`](../programs/global/value-range/README.md) | 13 | 310 | 2% more | level | 4% less | 39% less | not measured | inside the noise | 16% less |
| [`prune`](../programs/global/prune/README.md) | 15 | 472 | 37% less | level | 11% less | 48% less | not measured | inside the noise | 44% less |
| [`jump-threading`](../programs/global/jump-threading/README.md) | 5 | 224 | 17% less | level | 11% less | 56% less | not measured | inside the noise | 38% less |
| [`value-replacement`](../programs/global/value-replacement/README.md) | 13 | 360 | 35% less | level | 11% less | 52% less | not measured | inside the noise | 47% less |
| [`alias-analysis`](../programs/global/alias-analysis/README.md) | 36 | 792 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 17% less |
| [`memory-ssa`](../programs/global/memory-ssa/README.md) | 28 | 620 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 19% less |
| [`scalar-replacement`](../programs/global/scalar-replacement/README.md) | 20 | 376 | 6% less | level | 4% less | 38% less | not measured | inside the noise | 16% less |
| [`loop-invariant`](../programs/loops/loop-invariant/README.md) | 32 | 640 | 6% less | level | 4% less | 41% less | not measured | inside the noise | 20% less |
| [`loop-hoist`](../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 6% less | level | 4% less | 43% less | not measured | inside the noise | 20% less |
| [`induction-variable`](../programs/loops/induction-variable/README.md) | 42 | 863 | 6% less | level | 4% less | 40% less | not measured | inside the noise | 22% less |
| [`iv-selection`](../programs/loops/iv-selection/README.md) | 36 | 760 | 29% less | level | 11% less | 51% less | not measured | inside the noise | 38% less |
| [`loop-unswitch`](../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 2% less | level | 4% less | 47% less | not measured | inside the noise | 31% less |
| [`loop-unroll`](../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 10% less | level | 4% less | 43% less | not measured | inside the noise | 20% less |
| [`loop-unroll-shape`](../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 10% less | level | 4% less | 40% less | not measured | inside the noise | 19% less |
| [`loop-idiom`](../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 29% less | level | 10% less | 45% less | not measured | level | 20% less |
| [`loop-deletion`](../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 6% less | level | 4% less | 40% less | not measured | inside the noise | 19% less |
| [`bit-loops`](../programs/loops/bit-loops/README.md) | 16 | 564 | 1% less | level | 2% less | 51% less | not measured | inside the noise | 38% less |
| [`loop-rotate`](../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 10% less | level | 4% less | 39% less | not measured | inside the noise | 19% less |
| [`loop-shape`](../programs/loops/loop-shape/README.md) | 64 | 1,416 | 9% less | level | 4% less | 46% less | not measured | inside the noise | 32% less |
| [`loop-restructure`](../programs/loops/loop-restructure/README.md) | 48 | 960 | 10% less | level | 4% less | 42% less | not measured | inside the noise | 26% less |
| [`inline`](../programs/interprocedural/inline/README.md) | 36 | 1,200 | 6% less | level | 3% less | 39% less | not measured | inside the noise | 15% less |
| [`tail-call`](../programs/interprocedural/tail-call/README.md) | 36 | 876 | 6% more | level | 2% less | 44% less | not measured | inside the noise | 21% less |
| [`tail-dispatch`](../programs/interprocedural/tail-dispatch/README.md) | 5 | 893 | 7% more | level | 3% less | 57% less | not measured | inside the noise | 41% less |
| [`called-once`](../programs/interprocedural/called-once/README.md) | 5 | 413 | 2% less | level | 4% less | 59% less | not measured | inside the noise | 1037% more |
| [`function-purity`](../programs/interprocedural/function-purity/README.md) | 28 | 636 | 6% less | level | 4% less | 42% less | not measured | inside the noise | 21% less |
| [`constant-args`](../programs/interprocedural/constant-args/README.md) | 36 | 792 | 6% less | level | 4% less | 43% less | not measured | level | 17% less |
| [`unused-params`](../programs/interprocedural/unused-params/README.md) | 44 | 948 | 6% less | level | 4% less | 41% less | not measured | inside the noise | 17% less |
| [`unused-returns`](../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 6% less | level | 3% less | 41% less | not measured | inside the noise | 19% less |
| [`declared-purity`](../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 14% less | level | 6% less | 56% less | not measured | inside the noise | 17% less |
| [`reachability`](../programs/interprocedural/reachability/README.md) | 9 | 247 | 6% less | level | 4% less | 44% less | not measured | inside the noise | 14% less |
| [`devirtualize`](../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 6% less | level | 4% less | 45% less | not measured | inside the noise | level |
| [`memory-effects`](../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 6% less | level | 3% less | 48% less | not measured | inside the noise | 41% less |
| [`call-motion`](../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | 6% less | level | 3% less | 48% less | not measured | inside the noise | 38% less |
| [`link-time-optimization`](../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 6% less | level | 3% less | 64% less | not measured | inside the noise | 17% less |
| [`selection`](../programs/backend/selection/README.md) | 40 | 680 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 15% less |
| [`register-pressure`](../programs/backend/register-pressure/README.md) | 60 | 3,120 | 6% less | level | 4% less | 42% less | not measured | inside the noise | 17% less |
| [`register-alloc`](../programs/backend/register-alloc/README.md) | 40 | 2,664 | 2% less | level | 4% less | 42% less | not measured | inside the noise | 20% less |
| [`scheduling`](../programs/backend/scheduling/README.md) | 20 | 1,046 | 42% more | level | 4% less | 45% less | not measured | level | 37% less |
| [`block-layout`](../programs/backend/block-layout/README.md) | 16 | 421 | 8% less | 1% more | 2% less | 44% less | not measured | inside the noise | 30% less |
| [`if-conversion`](../programs/backend/if-conversion/README.md) | 38 | 945 | 12% less | level | 4% less | 46% less | not measured | inside the noise | 36% less |
| [`switch-lowering`](../programs/backend/switch-lowering/README.md) | 27 | 1,521 | 8% less | level | 4% less | 52% less | not measured | inside the noise | level |
| [`switch-runs`](../programs/backend/switch-runs/README.md) | 15 | 1,356 | 9% less | level | 4% less | 48% less | not measured | inside the noise | 97% less |
| [`switch-dispatch`](../programs/backend/switch-dispatch/README.md) | 21 | 942 | 28% less | level | 10% less | 51% less | not measured | inside the noise | level |
| [`division`](../programs/backend/division/README.md) | 72 | 2,020 | 23% less | level | 4% less | 56% less | not measured | inside the noise | 37% less |
| [`narrow-shift`](../programs/backend/narrow-shift/README.md) | 8 | 256 | 40% less | level | 5% less | 61% less | not measured | inside the noise | 48% less |
| [`calling-convention`](../programs/backend/calling-convention/README.md) | 10 | 216 | 6% less | level | 4% less | 44% less | not measured | inside the noise | 10% less |
| [`machine-peephole`](../programs/backend/machine-peephole/README.md) | 22 | 402 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 16% less |
| [`bit-liveness`](../programs/backend/bit-liveness/README.md) | 28 | 628 | 2% less | level | 4% less | 38% less | not measured | inside the noise | 17% less |
| [`compare-elim`](../programs/backend/compare-elim/README.md) | 26 | 526 | 6% less | level | 4% less | 39% less | not measured | inside the noise | 17% less |
| [`address-fold`](../programs/backend/address-fold/README.md) | 28 | 584 | 6% less | level | 4% less | 40% less | not measured | inside the noise | 17% less |
| [`load-fold`](../programs/backend/load-fold/README.md) | 40 | 924 | 14% less | level | 12% less | 44% less | not measured | inside the noise | 21% less |
| [`store-fold`](../programs/backend/store-fold/README.md) | 56 | 1,416 | 16% less | level | 11% less | 44% less | not measured | inside the noise | 21% less |
| [`store-fold-constant`](../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 14% less | level | 11% less | 45% less | not measured | level | 20% less |
| [`compare-fold`](../programs/backend/compare-fold/README.md) | 118 | 2,742 | 12% less | level | 11% less | 45% less | not measured | level | 21% less |
| [`frame-address`](../programs/backend/frame-address/README.md) | 6 | 117 | 6% less | level | 4% less | 40% less | not measured | inside the noise | 16% less |
| [`stack-slots`](../programs/backend/stack-slots/README.md) | 6 | 201 | 35% less | level | 13% less | 55% less | not measured | inside the noise | 38% less |
| [`block-copy`](../programs/backend/block-copy/README.md) | 44 | 1,320 | 23% less | level | 3% less | 53% less | not measured | inside the noise | 41% less |
| [`frame-size`](../programs/backend/frame-size/README.md) | 59 | 8,808 | 27% less | level | 10% less | 71% less | not measured | inside the noise | 46% less |
| [`interpreter-dispatch`](../programs/backend/interpreter-dispatch/README.md) | 54 | 34,509 | 60% more | level | 4% less | 67% less | not measured | inside the noise | 39% less |
| [`choice-across-call`](../programs/backend/choice-across-call/README.md) | 48 | 3,024 | 8% less | 1% less | 8% less | 50% less | not measured | inside the noise | 42% less |
| [`bit-builtins`](../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 14% more | level | 4% less | 49% less | not measured | inside the noise | 32% less |
| [`float-conversion`](../programs/backend/float-conversion/README.md) | 66 | 1,477 | 4% less | level | 4% less | 41% less | not measured | inside the noise | 17% less |
| [`long-double`](../programs/backend/long-double/README.md) | 10 | 226 | 38% more | level | 4% less | 43% less | not measured | inside the noise | 22% less |
| [`barrier`](../programs/correctness/barrier/README.md) | 13 | 241 | 6% less | level | 4% less | 37% less | not measured | inside the noise | 17% less |
| [`atomics`](../programs/correctness/atomics/README.md) | 31 | 1,334 | 4% less | level | 4% less | 48% less | not measured | inside the noise | 39% less |
| [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 12% less | level | 5% less | 42% less | not measured | inside the noise | 11% less |
| [`sigsetjmp`](../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 1% less | level | 12% less | 57% less | not measured | inside the noise | 48% less |
| [`builtin-setjmp`](../programs/correctness/builtin-setjmp/README.md) | 48 | 5,660 | 5% more | level | 13% less | 60% less | not measured | inside the noise | 48% less |
| [`overflow-builtins`](../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 26% more | level | 3% less | 56% less | not measured | inside the noise | 36% less |
| [`target-attribute`](../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 5% more | level | 2% less | 62% less | not measured | inside the noise | 43% less |
| [`crc32c`](../programs/correctness/crc32c/README.md) | 64 | 15,632 | level | level | 3% less | 60% less | not measured | inside the noise | 46% less |
| [`simd-lfind`](../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 76% more | level | 6% less | 58% less | not measured | inside the noise | 40% less |
| [`crc32c-armv8`](../programs/correctness/crc32c-armv8/README.md) | 72 | 25,314 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`simd-lfind-neon`](../programs/correctness/simd-lfind-neon/README.md) | 54 | 4,590 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`lkmm`](../programs/correctness/lkmm/README.md) | 24 | 1,164 | 16% less | level | 6% less | 47% less | not measured | inside the noise | 36% less |
| [`mitigations`](../programs/correctness/mitigations/README.md) | 25 | 1,510 | 3% less | level | 3% less | 50% less | not measured | inside the noise | 40% less |
| [`asm-goto`](../programs/correctness/asm-goto/README.md) | 6 | 172 | 10% less | level | 3% less | 44% less | not measured | inside the noise | 1783% more |
| [`asm-local-labels`](../programs/correctness/asm-local-labels/README.md) | 5 | 135 | 9% less | level | 3% less | 47% less | not measured | inside the noise | 22% less |
| [`gas-macros`](../programs/correctness/gas-macros/README.md) | 6 | 151 | 9% less | level | 3% less | 44% less | not measured | inside the noise | 10% less |
| [`constant-p-after-inline`](../programs/correctness/constant-p-after-inline/README.md) | 5 | 168 | 10% less | level | 3% less | 42% less | not measured | inside the noise | 40% less |
| [`mcmodel-kernel`](../programs/correctness/mcmodel-kernel/README.md) | 6 | 165 | 7% less | level | 3% less | 48% less | not measured | inside the noise | 899% more |
| [`general-regs-only`](../programs/correctness/general-regs-only/README.md) | 5 | 463 | 5% less | level | 6% less | 54% less | not measured | inside the noise | 40% less |
| [`i386-kernel`](../programs/correctness/i386-kernel/README.md) | 9 | 624 | not measured | not measured | not measured | not measured | not measured | not measured | not measured |
| [`objtool-shapes`](../programs/correctness/objtool-shapes/README.md) | 5 | 170 | 9% less | level | 3% less | 46% less | not measured | inside the noise | level |
| [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md) | 6 | 302 | 7% less | level | 3% less | 49% less | not measured | inside the noise | 41% less |
| [`const-ice`](../programs/correctness/const-ice/README.md) | 8 | 408 | 7% less | level | 3% less | 49% less | not measured | level | 39% less |
| [`section-attr`](../programs/correctness/section-attr/README.md) | 6 | 320 | 9% less | level | 3% less | 54% less | not measured | inside the noise | 46% less |
| [`bundle`](../programs/correctness/bundle/README.md) | 33 | 3,651 in 78 files | 8% less | level | 1% more | 50% less | not measured | level | level |
| [`dllimport`](../programs/correctness/dllimport/README.md) | 18 | 2,268 in 39 files | 9% less | level | 2% more | 46% less | not measured | level | level |
| [`frontend`](../programs/floor/frontend/README.md) | 30 | 403 | 6% less | level | 4% less | 43% less | not measured | inside the noise | 14% less |

3631 cases had a size to compare and 3631 had a memory figure.

