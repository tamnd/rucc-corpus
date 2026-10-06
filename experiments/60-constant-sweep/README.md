# Experiment 60: every constant at half and at double

This is experiment 60 of section 42.5 in rucc's [optimizer measurement plan](https://github.com/tamnd/rucc/blob/main/spec/optimizer/42-measurement.md), tamnd/rucc#2970. Section 40.14 of the [cost model specification](https://github.com/tamnd/rucc/blob/main/spec/optimizer/40-cost-models.md#4014-what-to-measure) says a constant whose sensitivity is flat is one that does not need tuning, and that knowing which those are is worth more than tuning the rest. This run says which they are.

**Of 88 rows, 63 are flat, 16 are worth tuning and 9 are wrong.** No move made any case print the wrong answer. Every wrong row has an issue of its own, and most of them are gcc's own numbers, so the question each one asks is whether rucc's code is what differs rather than whether the number should change.

The machine-readable numbers are in [results.json](results.json).

## How it was run

`rucc-corpus sweep` at commit `8da8508` (#141), on server2, an x86-64 Linux machine shared with other work, with 4 jobs, from 17:27 to 20:31 on 6 October 2026. rucc was 0.25.0, built from the branch of tamnd/rucc#3109, which is that afternoon's main with `cargo xtask sweep` added. gcc 16 was `16.0.1 20260315`. The corpus was its 3500 cases without the aarch64 ones, at `-O2`.

Each of the 88 rows of `rucc --print-params` was moved to half and to double its default through `--param`, one row at a time, which is 175 moves, since a row whose default is 0 has no half. Every case was compiled to assembly once as rucc is and once per move, and only the cases whose assembly changed were built and run, against rucc as it is and gcc 16. 46 of the moves changed at least one case.

A row is flat when neither end moves the instructions retired or the text by more than 0.1% of what the cases it changed had before. It is wrong when an end saves on one of the two and does not spend on the other, so a value the sweep tried beats the default. Every other row that moves something is worth tuning. Compile time is wall time on a shared machine and is the noisiest of the numbers, so no call is made on it.

## The wrong rows

| row | default | what the sweep found | issue |
|---|---:|---|---|
| `phiopt-arm-instructions` | 2 | rucc's own value. At 1, 116 cases retire 6.34% fewer instructions, nearly all in `if-conversion`. At 4, 45 cases retire 24% more, nearly all in `switch-lowering`. | tamnd/rucc#3157 |
| `inline-hint-percent` | 200 | gcc's value. Only 8 and 12 `target-attribute` cases move, by 18% less text at 100 and 24% more at 400, with instructions flat. | tamnd/rucc#3158 |
| `predict-expect` | 90 | gcc's value. At 180 `if-conversion` and `switch-lowering` move by about half a billion instructions each, in opposite directions, and 0.24% is left. | tamnd/rucc#3159 |
| `predict-pointer-not-null` | 70 | gcc's value. At 35, 74 cases have 0.48% less text and 0.09% more instructions. | tamnd/rucc#3160 |
| `predict-call-not-taken` | 67 | gcc's value. At 33, 198 cases retire 0.32% fewer instructions, nearly all in `bit-loops`, but section 11.2 drops predictors under 65 percent. | tamnd/rucc#3161 |
| `loop-reserved-regs` | 2 | gcc's value. Both ends change 71 cases and both save a little, mostly in `crc32c`. | tamnd/rucc#3162 |
| `switch-conversion-max-growth` | 8 | gcc's value. At 16 two `switch-lowering` cases lose 15.64% of their text. | tamnd/rucc#3163 |
| `jump-table-min-targets` | 11 | rucc's own value, set by timing in tamnd/rucc#1759. At 5, 29 cases retire 14.88% fewer instructions, nearly all in `switch-dispatch`, and rucc goes from 1.587 to 1.351 times gcc 16 on them. | tamnd/rucc#3164 |
| `dse-walk-limit` | 256 | gcc's value. At 512, 21 `setjmp` cases have 0.36% less text. | tamnd/rucc#3165 |

## The rows worth tuning

`phiopt-unpredictable-margin-percent`, `short-circuit-instructions`, `sra-max-bytes`, `sra-max-pieces`, `sra-max-word-pieces`, `loop-header-insns-for-speed`, `inline-insns-auto`, `inline-frame-growth`, `inline-large-frame`, `predict-loop-exit-not-taken`, `predict-loop-guard-taken`, `unroll-max-times`, `unroll-max-insns`, `ivopts-new-variable-bias`, `switch-peel-percent`, `range-test-bit-intervals`. Each of these saves on one of instructions and text at one end and spends on the other, so a better value is a trade and not a free win, and none of them gets an issue until a program the corpus does not have yet asks for one.

## Every move

Each change is over the cases that move changed and not over the whole corpus. Instructions are over the cases that ran right for rucc both ways and for gcc 16, and the column against gcc 16 is rucc's instructions over gcc 16's on those cases, as it is and then moved.

| row | default | end | value | cases changed | instructions | text | compile time | against gcc-16 | instructions by facet | call |
|---|---:|---|---:|---:|---:|---:|---:|---|---|---|
| `branch-cost-for-size` | 2 | half | 1 | 0 |  |  |  |  |  | flat |
| `branch-cost-for-size` | 2 | double | 4 | 0 |  |  |  |  |  | flat |
| `branch-cost-predictable` | 0 | double | 1 | 0 |  |  |  |  |  | flat |
| `predictable-branch-percent` | 2 | half | 1 | 0 |  |  |  |  |  | flat |
| `predictable-branch-percent` | 2 | double | 4 | 0 |  |  |  |  |  | flat |
| `if-conversion-budget-predictable` | 20 | half | 10 | 0 |  |  |  |  |  | flat |
| `if-conversion-budget-predictable` | 20 | double | 40 | 0 |  |  |  |  |  | flat |
| `if-conversion-budget-unpredictable` | 40 | half | 20 | 0 |  |  |  |  |  | flat |
| `if-conversion-budget-unpredictable` | 40 | double | 80 | 0 |  |  |  |  |  | flat |
| `if-conversion-block-limit` | 10 | half | 5 | 0 |  |  |  |  |  | flat |
| `if-conversion-block-limit` | 10 | double | 20 | 0 |  |  |  |  |  | flat |
| `phiopt-arm-instructions` | 2 | half | 1 | 116 | -6.34% | +0.04% | +2.50% | 1.195 to 1.119 | if-conversion -60014541, value-replacement +1703936, crc32c -393221 | wrong |
| `phiopt-arm-instructions` | 2 | double | 4 | 45 | +24.07% | +0.57% | -20.95% | 1.009 to 1.251 | switch-lowering +1297469211, simd-lfind +84460, value-settled -6352 | wrong |
| `phiopt-factor-depth` | 4 | half | 2 | 0 |  |  |  |  |  | flat |
| `phiopt-factor-depth` | 4 | double | 8 | 0 |  |  |  |  |  | flat |
| `phiopt-arm-scan-instructions` | 16 | half | 8 | 0 |  |  |  |  |  | flat |
| `phiopt-arm-scan-instructions` | 16 | double | 32 | 0 |  |  |  |  |  | flat |
| `phiopt-unpredictable-margin-percent` | 3 | half | 1 | 2 | +0.28% | -0.77% | +4.30% | 1.023 to 1.026 | short-circuit +667 | worth tuning |
| `phiopt-unpredictable-margin-percent` | 3 | double | 6 | 0 |  |  |  |  |  | worth tuning |
| `short-circuit-instructions` | 2 | half | 1 | 8 | -0.53% | +0.19% | -1.34% | 1.028 to 1.022 | short-circuit -5120 | worth tuning |
| `short-circuit-instructions` | 2 | double | 4 | 65 | +0.25% | +0.67% | +106.14% | 1.160 to 1.163 | crc32c +40182, section-attr -192 | worth tuning |
| `sra-max-bytes` | 128 | half | 64 | 11 | +0.00% | +10.29% | +5.19% | 0.999 to 0.999 | calling-convention +36, loop-idiom +12, induction-variable +1 | worth tuning |
| `sra-max-bytes` | 128 | double | 256 | 0 |  |  |  |  |  | worth tuning |
| `sra-max-pieces` | 8 | half | 4 | 4 | +0.01% | +11.32% | -22.11% | 0.999 to 0.999 | alias-analysis +36 | worth tuning |
| `sra-max-pieces` | 8 | double | 16 | 26 | +3.16% | +8.38% | -26.90% | 1.507 to 1.554 | simd-lfind +198734, target-attribute -2750 | worth tuning |
| `sra-max-word-pieces` | 16 | half | 8 | 22 | +0.01% | +5.72% | -41.60% | 0.998 to 0.998 | general-regs-only +320, calling-convention +37, induction-variable +2 | worth tuning |
| `sra-max-word-pieces` | 16 | double | 32 | 0 |  |  |  |  |  | worth tuning |
| `block-copy-moves-for-speed` | 8 | half | 4 | 0 |  |  |  |  |  | flat |
| `block-copy-moves-for-speed` | 8 | double | 16 | 0 |  |  |  |  |  | flat |
| `block-copy-moves-for-size` | 4 | half | 2 | 0 |  |  |  |  |  | flat |
| `block-copy-moves-for-size` | 4 | double | 8 | 0 |  |  |  |  |  | flat |
| `loop-header-insns-for-speed` | 20 | half | 10 | 0 |  |  |  |  |  | worth tuning |
| `loop-header-insns-for-speed` | 20 | double | 40 | 4 | -0.03% | +1.59% | -1.97% | 1.003 to 1.003 | loop-shape -139 | worth tuning |
| `loop-header-insns-for-size` | 5 | half | 2 | 0 |  |  |  |  |  | flat |
| `loop-header-insns-for-size` | 5 | double | 10 | 0 |  |  |  |  |  | flat |
| `jump-thread-duplication-insns` | 15 | half | 7 | 0 |  |  |  |  |  | flat |
| `jump-thread-duplication-insns` | 15 | double | 30 | 0 |  |  |  |  |  | flat |
| `jump-thread-paths` | 64 | half | 32 | 0 |  |  |  |  |  | flat |
| `jump-thread-paths` | 64 | double | 128 | 0 |  |  |  |  |  | flat |
| `jump-thread-path-insns` | 100 | half | 50 | 0 |  |  |  |  |  | flat |
| `jump-thread-path-insns` | 100 | double | 200 | 0 |  |  |  |  |  | flat |
| `jump-thread-back-edge-scale` | 2 | half | 1 | 0 |  |  |  |  |  | flat |
| `jump-thread-back-edge-scale` | 2 | double | 4 | 0 |  |  |  |  |  | flat |
| `reassoc-width-untuned` | 1 | half | 0 | 0 |  |  |  |  |  | flat |
| `reassoc-width-untuned` | 1 | double | 2 | 0 |  |  |  |  |  | flat |
| `inline-frequency-clamp` | 100 | half | 50 | 0 |  |  |  |  |  | flat |
| `inline-frequency-clamp` | 100 | double | 200 | 0 |  |  |  |  |  | flat |
| `inline-growth-squaring-bound` | 256 | half | 128 | 0 |  |  |  |  |  | flat |
| `inline-growth-squaring-bound` | 256 | double | 512 | 0 |  |  |  |  |  | flat |
| `inline-insns-single` | 70 | half | 35 | 0 |  |  |  |  |  | flat |
| `inline-insns-single` | 70 | double | 140 | 0 |  |  |  |  |  | flat |
| `inline-early-insns` | 6 | half | 3 | 0 |  |  |  |  |  | flat |
| `inline-early-insns` | 6 | double | 12 | 0 |  |  |  |  |  | flat |
| `inline-insns-single-o3` | 200 | half | 100 | 0 |  |  |  |  |  | flat |
| `inline-insns-single-o3` | 200 | double | 400 | 0 |  |  |  |  |  | flat |
| `inline-insns-auto` | 15 | half | 7 | 169 | +1.26% | -4.92% | -32.13% | 1.106 to 1.120 | crc32c +413839, interpreter-dispatch +2769, bundle +1770 | worth tuning |
| `inline-insns-auto` | 15 | double | 30 | 83 | -0.14% | +16.49% | -50.05% | 1.114 to 1.113 | crc32c -25431, target-attribute +953, bundle -55 | worth tuning |
| `inline-called-once-insns` | 4000 | half | 2000 | 0 |  |  |  |  |  | flat |
| `inline-called-once-insns` | 4000 | double | 8000 | 0 |  |  |  |  |  | flat |
| `inline-called-once-loop-depth` | 6 | half | 3 | 0 |  |  |  |  |  | flat |
| `inline-called-once-loop-depth` | 6 | double | 12 | 0 |  |  |  |  |  | flat |
| `inline-frame-growth` | 1000 | half | 500 | 0 |  |  |  |  |  | worth tuning |
| `inline-frame-growth` | 1000 | double | 2000 | 6 | +0.10% | -6.72% | -9.28% | 1.706 to 1.707 | simd-lfind +1971 | worth tuning |
| `inline-large-frame` | 256 | half | 128 | 6 | -0.19% | +13.00% | -58.34% | 1.883 to 1.879 | simd-lfind -3678 | worth tuning |
| `inline-large-frame` | 256 | double | 512 | 6 | +0.10% | -6.72% | +96.25% | 1.706 to 1.707 | simd-lfind +1972 | worth tuning |
| `inline-frame-growth-conserve` | 40 | half | 20 | 0 |  |  |  |  |  | flat |
| `inline-frame-growth-conserve` | 40 | double | 80 | 0 |  |  |  |  |  | flat |
| `inline-large-frame-conserve` | 100 | half | 50 | 0 |  |  |  |  |  | flat |
| `inline-large-frame-conserve` | 100 | double | 200 | 0 |  |  |  |  |  | flat |
| `inline-hint-percent` | 200 | half | 100 | 8 | -0.05% | -18.11% | +326.14% | 1.030 to 1.030 | target-attribute -533 | wrong |
| `inline-hint-percent` | 200 | double | 400 | 12 | -0.01% | +24.23% | +35.73% | 1.010 to 1.010 | target-attribute -183 | wrong |
| `inline-hint-percent-o3` | 600 | half | 300 | 0 |  |  |  |  |  | flat |
| `inline-hint-percent-o3` | 600 | double | 1200 | 0 |  |  |  |  |  | flat |
| `inline-min-speedup` | 30 | half | 15 | 0 |  |  |  |  |  | flat |
| `inline-min-speedup` | 30 | double | 60 | 0 |  |  |  |  |  | flat |
| `inline-min-speedup-o3` | 15 | half | 7 | 0 |  |  |  |  |  | flat |
| `inline-min-speedup-o3` | 15 | double | 30 | 0 |  |  |  |  |  | flat |
| `inline-unit-growth` | 40 | half | 20 | 0 |  |  |  |  |  | flat |
| `inline-unit-growth` | 40 | double | 80 | 0 |  |  |  |  |  | flat |
| `large-unit-insns` | 10000 | half | 5000 | 0 |  |  |  |  |  | flat |
| `large-unit-insns` | 10000 | double | 20000 | 0 |  |  |  |  |  | flat |
| `large-function-insns` | 2700 | half | 1350 | 0 |  |  |  |  |  | flat |
| `large-function-insns` | 2700 | double | 5400 | 0 |  |  |  |  |  | flat |
| `large-function-growth` | 100 | half | 50 | 0 |  |  |  |  |  | flat |
| `large-function-growth` | 100 | double | 200 | 0 |  |  |  |  |  | flat |
| `inline-call-time` | 10 | half | 5 | 0 |  |  |  |  |  | flat |
| `inline-call-time` | 10 | double | 20 | 0 |  |  |  |  |  | flat |
| `hot-block-fraction` | 1000 | half | 500 | 0 |  |  |  |  |  | flat |
| `hot-block-fraction` | 1000 | double | 2000 | 0 |  |  |  |  |  | flat |
| `predict-expect` | 90 | half | 45 | 8 | +11.04% | -0.14% | -11.10% | 1.109 to 1.231 | switch-lowering +226778956, block-layout +2001 | wrong |
| `predict-expect` | 90 | double | 180 | 17 | -0.24% | +0.09% | -9.40% | 1.147 to 1.144 | if-conversion -527248604, switch-lowering +512094696, block-layout +1000 | wrong |
| `predict-never-returns` | 99 | half | 49 | 96 | -0.00% | -0.02% | -0.10% | 0.999 to 0.999 | setjmp-longjmp -166, sigsetjmp +36, branch-probability -1 | flat |
| `predict-never-returns` | 99 | double | 198 | 0 |  |  |  |  |  | flat |
| `predict-cold-call` | 99 | half | 49 | 0 |  |  |  |  |  | flat |
| `predict-cold-call` | 99 | double | 198 | 0 |  |  |  |  |  | flat |
| `predict-loop-exit-not-taken` | 89 | half | 44 | 448 | -0.06% | -0.03% | +13.59% | 1.079 to 1.079 | lkmm -2924755, bit-loops +679289, frame-size +99805 | worth tuning |
| `predict-loop-exit-not-taken` | 89 | double | 178 | 482 | +5.83% | +0.42% | +8.95% | 1.216 to 1.287 | division +289406668, called-once +8388589, bit-loops -400005 | worth tuning |
| `predict-loop-guard-taken` | 73 | half | 36 | 335 | +0.12% | +0.29% | +31.82% | 1.270 to 1.272 | called-once +3670025, bit-loops +375235, loop-unroll -40989 | worth tuning |
| `predict-loop-guard-taken` | 73 | double | 146 | 87 | -0.08% | +0.19% | -37.88% | 1.232 to 1.231 | crc32c -21697, simd-lfind +3240, frame-size -112 | worth tuning |
| `predict-pointer-not-null` | 70 | half | 35 | 74 | +0.09% | -0.48% | +0.13% | 1.050 to 1.051 | interpreter-dispatch +10221, short-circuit +286, dllimport +139 | wrong |
| `predict-pointer-not-null` | 70 | double | 140 | 24 | +0.00% | -0.02% | +10.35% | 1.094 to 1.094 | bundle +152, interpreter-dispatch +4 | wrong |
| `predict-negative-return` | 98 | half | 49 | 0 |  |  |  |  |  | flat |
| `predict-negative-return` | 98 | double | 196 | 0 |  |  |  |  |  | flat |
| `predict-null-return` | 71 | half | 35 | 0 |  |  |  |  |  | flat |
| `predict-null-return` | 71 | double | 142 | 0 |  |  |  |  |  | flat |
| `predict-call-not-taken` | 67 | half | 33 | 198 | -0.32% | -0.03% | -8.50% | 1.251 to 1.247 | bit-loops -800004, interpreter-dispatch -4913, block-layout +2004 | wrong |
| `predict-call-not-taken` | 67 | double | 134 | 46 | -0.00% | -0.07% | +19.20% | 1.353 to 1.353 | interpreter-dispatch -2419, simplify -54, tail-dispatch -7 | wrong |
| `predict-continue-taken` | 67 | half | 33 | 0 |  |  |  |  |  | flat |
| `predict-continue-taken` | 67 | double | 134 | 0 |  |  |  |  |  | flat |
| `max-predicted-iterations` | 100 | half | 50 | 4 | -0.00% | +0.00% | +33.39% | 1.143 to 1.143 | interpreter-dispatch -4 | flat |
| `max-predicted-iterations` | 100 | double | 200 | 6 | -0.00% | +0.00% | +40.00% | 1.131 to 1.131 | interpreter-dispatch -3 | flat |
| `loop-reserved-regs` | 2 | half | 1 | 71 | -0.05% | +0.07% | -22.64% | 1.154 to 1.154 | crc32c -8558, target-attribute +30, sigsetjmp -3 | wrong |
| `loop-reserved-regs` | 2 | double | 4 | 71 | -0.03% | -0.26% | -38.74% | 1.122 to 1.122 | crc32c -10622, bundle +15, dllimport +10 | wrong |
| `licm-expensive` | 20 | half | 10 | 0 |  |  |  |  |  | flat |
| `licm-expensive` | 20 | double | 40 | 0 |  |  |  |  |  | flat |
| `unroll-max-times` | 16 | half | 8 | 164 | +9.92% | -8.10% | -11.39% | 1.196 to 1.315 | simd-lfind +2119237, target-attribute +462267, loop-restructure +20066 | worth tuning |
| `unroll-max-times` | 16 | double | 32 | 76 | -1.34% | +37.23% | -7.60% | 1.075 to 1.061 | frame-size -267476, target-attribute -45396, iv-selection -5023 | worth tuning |
| `unroll-max-insns` | 200 | half | 100 | 58 | +2.99% | -7.49% | -1.35% | 1.125 to 1.158 | target-attribute +218227, loop-restructure +12175, simd-lfind +10405 | worth tuning |
| `unroll-max-insns` | 200 | double | 400 | 48 | -5.20% | +33.02% | -8.07% | 1.271 to 1.205 | simd-lfind -411626, loop-restructure -1867, register-alloc -895 | worth tuning |
| `split-max-insns` | 200 | half | 100 | 0 |  |  |  |  |  | flat |
| `split-max-insns` | 200 | double | 400 | 0 |  |  |  |  |  | flat |
| `split-remade-insns` | 8 | half | 4 | 0 |  |  |  |  |  | flat |
| `split-remade-insns` | 8 | double | 16 | 0 |  |  |  |  |  | flat |
| `unroll-max-depth` | 8 | half | 4 | 0 |  |  |  |  |  | flat |
| `unroll-max-depth` | 8 | double | 16 | 0 |  |  |  |  |  | flat |
| `profile-sum-tolerance-percent` | 1 | half | 0 | 0 |  |  |  |  |  | flat |
| `profile-sum-tolerance-percent` | 1 | double | 2 | 0 |  |  |  |  |  | flat |
| `predict-return-blocks` | 8 | half | 4 | 0 |  |  |  |  |  | flat |
| `predict-return-blocks` | 8 | double | 16 | 0 |  |  |  |  |  | flat |
| `align-frequency-fraction` | 100 | half | 50 | 0 |  |  |  |  |  | flat |
| `align-frequency-fraction` | 100 | double | 200 | 0 |  |  |  |  |  | flat |
| `loop-align-min-iterations` | 4 | half | 2 | 0 |  |  |  |  |  | flat |
| `loop-align-min-iterations` | 4 | double | 8 | 0 |  |  |  |  |  | flat |
| `iv-max-considered-uses` | 250 | half | 125 | 0 |  |  |  |  |  | flat |
| `iv-max-considered-uses` | 250 | double | 500 | 0 |  |  |  |  |  | flat |
| `iv-consider-all-candidates-bound` | 40 | half | 20 | 0 |  |  |  |  |  | flat |
| `iv-consider-all-candidates-bound` | 40 | double | 80 | 0 |  |  |  |  |  | flat |
| `iv-always-prune-cand-set-bound` | 10 | half | 5 | 0 |  |  |  |  |  | flat |
| `iv-always-prune-cand-set-bound` | 10 | double | 20 | 0 |  |  |  |  |  | flat |
| `ivopts-new-variable-bias` | 3 | half | 1 | 484 | -0.38% | +1.19% | +10.30% | 1.304 to 1.299 | value-replacement -13545422, loop-idiom +600846, simplify -585138 | worth tuning |
| `ivopts-new-variable-bias` | 3 | double | 6 | 203 | +5.70% | +0.12% | +21.04% | 1.238 to 1.309 | division +302211177, narrow-shift +8380422, frame-size +157729 | worth tuning |
| `ivopts-set-penalty` | 10 | half | 5 | 0 |  |  |  |  |  | flat |
| `ivopts-set-penalty` | 10 | double | 20 | 0 |  |  |  |  |  | flat |
| `scheduler-ready-list-bound` | 100 | half | 50 | 0 |  |  |  |  |  | flat |
| `scheduler-ready-list-bound` | 100 | double | 200 | 0 |  |  |  |  |  | flat |
| `switch-conversion-max-growth` | 8 | half | 4 | 0 |  |  |  |  |  | wrong |
| `switch-conversion-max-growth` | 8 | double | 16 | 2 | -0.02% | -15.64% | +0.03% | 1.003 to 1.003 | switch-lowering -50 | wrong |
| `jump-table-min-targets` | 11 | half | 5 | 29 | -14.88% | -1.53% | -10.14% | 1.587 to 1.351 | switch-dispatch -33842428, interpreter-dispatch -94053, mitigations -309 | wrong |
| `jump-table-min-targets` | 11 | double | 22 | 1 | +35.76% | +35.31% | -67.66% | 1.488 to 2.020 | tail-dispatch +19651206 | wrong |
| `jump-table-min-targets-for-size` | 6 | half | 3 | 0 |  |  |  |  |  | flat |
| `jump-table-min-targets-for-size` | 6 | double | 12 | 0 |  |  |  |  |  | flat |
| `switch-peel-percent` | 66 | half | 33 | 0 |  |  |  |  |  | worth tuning |
| `switch-peel-percent` | 66 | double | 132 | 4 | +11.04% | +0.00% | +219.19% | 1.109 to 1.231 | switch-lowering +226779150 | worth tuning |
| `wasm-jump-table-min-targets` | 24 | half | 12 | 0 |  |  |  |  |  | flat |
| `wasm-jump-table-min-targets` | 24 | double | 48 | 0 |  |  |  |  |  | flat |
| `wasm-jump-table-min-targets-for-size` | 4 | half | 2 | 0 |  |  |  |  |  | flat |
| `wasm-jump-table-min-targets-for-size` | 4 | double | 8 | 0 |  |  |  |  |  | flat |
| `allocator-degradation-percent` | 10 | half | 5 | 0 |  |  |  |  |  | flat |
| `allocator-degradation-percent` | 10 | double | 20 | 0 |  |  |  |  |  | flat |
| `dse-walk-limit` | 256 | half | 128 | 34 | +0.00% | +0.64% | +42.65% | 1.002 to 1.002 | load-forwarding +83, sigsetjmp +56, builtin-setjmp +13 | wrong |
| `dse-walk-limit` | 256 | double | 512 | 21 | -0.00% | -0.36% | -16.48% | 1.001 to 1.001 | builtin-setjmp -29, sigsetjmp -21 | wrong |
| `range-test-bit-intervals` | 3 | half | 1 | 63 | +0.01% | +1.35% | -35.32% | 1.107 to 1.107 | target-attribute +703, crc32c +319, value-range +8 | worth tuning |
| `range-test-bit-intervals` | 3 | double | 6 | 1 | +1.12% | +0.78% | +35.48% | 1.011 to 1.022 | short-circuit +1355 | worth tuning |
| `range-switch-cases` | 64 | half | 32 | 0 |  |  |  |  |  | flat |
| `range-switch-cases` | 64 | double | 128 | 0 |  |  |  |  |  | flat |
| `constant-p-load-depth` | 8 | half | 4 | 0 |  |  |  |  |  | flat |
| `constant-p-load-depth` | 8 | double | 16 | 0 |  |  |  |  |  | flat |
| `cons-cascade` | 8 | half | 4 | 0 |  |  |  |  |  | flat |
| `cons-cascade` | 8 | double | 16 | 0 |  |  |  |  |  | flat |
| `egraph-nodes` | 65536 | half | 32768 | 0 |  |  |  |  |  | flat |
| `egraph-nodes` | 65536 | double | 131072 | 0 |  |  |  |  |  | flat |
| `select-forms` | 2 | half | 1 | 0 |  |  |  |  |  | flat |
| `select-forms` | 2 | double | 4 | 0 |  |  |  |  |  | flat |
| `simplify-rounds` | 4 | half | 2 | 0 |  |  |  |  |  | flat |
| `simplify-rounds` | 4 | double | 8 | 0 |  |  |  |  |  | flat |

## Running it again

```sh
cargo run --release -p rucc-corpus -- sweep \
  --rucc path/to/rucc --reference gcc-16=/usr/bin/gcc-16 \
  --exclude-tag aarch64 --out out --work work --jobs 4
```

`cargo xtask sweep` in tamnd/rucc runs the same thing against the corpus commit rucc pins. A sweep that stops picks up where it was from `sweep.jsonl`, as long as the compiler and the corpus are the same.
