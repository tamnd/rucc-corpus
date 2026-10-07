# The pass off: each rucc pass turned off on its own

This is the default experiment of section 42.4 in rucc's [optimizer measurement plan](https://github.com/tamnd/rucc/blob/main/spec/optimizer/42-measurement.md#424-the-default-experiment-the-pass-off), tamnd/rucc#2968. Every pass rucc runs at `-O2` is turned off with `-fdisable-<pass>`, one at a time, and the cases whose assembly changes are built and run, which says what each pass is worth on this corpus with the rest of the pipeline still there.

**Of 43 passes, 24 pay, 7 trade, 3 cost, 2 cannot be turned off and 7 change no case.** No pass that can be turned off made a case print the wrong answer. Each of the three that cost does so through a shape it leaves for a later part of the compiler, and each of those shapes has an issue in tamnd/rucc.

The machine-readable numbers are in [results.json](results.json).

## How it was run

`rucc-corpus passoff` at commit `60c8fd1` (#143), on server2, an x86-64 Linux machine shared with other work, with 4 jobs, from 22:09 to 23:16 on 6 October 2026. rucc was 0.27.0, built from main at `a52b6c74`. gcc 16 was `16.0.1 20260315`. The corpus was its 3500 cases without the aarch64 ones, at `-O2`.

Every case was compiled to assembly once as rucc is and once with each pass off, and only the cases whose assembly changed were built and run, against rucc as it is and gcc 16. A pass pays when turning it off costs instructions retired or text by more than 0.1% of what the cases it changed had, and saves on neither. It trades when off saves on one and costs on the other, and it costs when off saves on one and costs on neither, which is a pass that makes the code worse on this corpus. It is flat when it changes the assembly and moves neither, and idle when it changes no case at all. Compile time is wall time on a shared machine and is the noisiest of the numbers, so no call is made on it.

**These numbers do not add up.** Passes enable each other, so what one pass is worth with the rest of the pipeline there is not its share of what the pipeline is worth, and a pass that looks idle here may be what makes another one fire. This is the warning of section 42.8, and the three passes below that cost are each an example of it.

## The passes that cost

| pass | what turning it off saves | why | issue |
|---|---|---|---|
| `widen` | 1.58% of the instructions of the 698 cases it changes, and 151 million in `division` alone, where each of the 72 cases saves 2.1 million, one instruction for each trip of the inner loop. | With a 64-bit index the loop takes the address of the `static` array again on every trip, where without `widen` ivopts steps a pointer through it as gcc 16 does. rucc 0.27.0 takes the address of a symbol inside a loop in 556 of the 3500 cases. | tamnd/rucc#3185 |
| `dce` | 5.14% of the instructions of the 195 cases it changes, and nearly all of it is 8.4 million in the four `narrow-shift` cases. | The compare that ends the inner loop goes through `setl` and `testb`, because the add of the pointer falls between the compare and the branch and takes the flags. Turning off `ivopts` saves the same on the same cases. | tamnd/rucc#3186 |
| `redundant-load` | 0.12% of the instructions and 0.57% of the text of the 350 cases it changes, and 98 thousand of the instructions in one case, `frame-size.8.16384.4.leaf`. | Keeping four loaded fields in registers raises the pressure enough that a sum loop's accumulator does not keep its register, and the loop ends in a copy and a jump on every trip. | tamnd/rucc#3187 |

None of the three is wrong in what it does. Fixing what comes after each should turn its call into pays or flat on the next run, which is how each issue says it is done.

## The passes that trade

`unroll`, `loop-idiom`, `vectorize`, `canon`, `header-copy` and `licm` save instructions and spend text, which is what each of them is for. `loop-idiom` saves the most, 35.48% of the instructions of 17 cases for 2.56% of their text, and `unroll` the most text, 5.81% of 1133 cases for 2.34% of their instructions.

`phiopt` is the other way round. It saves 2.73% of the text of 634 cases and costs 0.72% of their instructions, and 77 million of those are in `if-conversion`. That is the same finding as the `phiopt-arm-instructions` row of [experiment 60](../60-constant-sweep/README.md), tamnd/rucc#3157, and the two should be read together.

## The passes that cannot be turned off

`expect` and `constant-p` are the two passes rucc requires, because nothing after them lowers what they remove, and `-fno-<pass>` leaves them in. `-fdisable-<pass>` did not, which is tamnd/rucc#3189. With `expect` off the 28 cases that call `__builtin_expect` stopped compiling with E0653, and with `constant-p` off the five `constant-p-after-inline` cases failed to link, since a call the answer would have removed was left in.

The run called `expect` flat, because a case that stops on a construct the compiler says it has not been taught was counted as a gap and not as broken. That was wrong for a move of the compiler, since the program is the same, and `rucc-corpus sweep` and `rucc-corpus passoff` now count it as broken. Once tamnd/rucc#3189 is fixed both passes will change no case here, which is what a required pass should do.

## The passes that change nothing

`lanes`, `hoist`, `plane-sink`, `split`, `discharge`, `dead-plane` and `coalesce` change the assembly of no case at `-O2`. That is either a pass whose work another one already does on this corpus or a shape the corpus does not have yet, and which of the two it is goes to the firing counts of tamnd/rucc#404 to settle, along with `expect` and `constant-p` above. Until then none of them is evidence that the pass can go.

## Every pass

Each change is over the cases that pass changed and not over the whole corpus, and is off against on, so a positive change is what the pass saves. Instructions are over the cases that ran right for rucc both ways and for gcc 16, and the column against gcc 16 is rucc's instructions over gcc 16's on those cases, with the pass and then without.

| pass | cases changed | instructions | text | compile time | against gcc-16 | helps most | hurts most | call |
|---|---:|---:|---:|---:|---|---|---|---|
| `expect` | 28 |  |  |  |  |  |  | flat |
| `fold` | 1132 | +1.10% | +17.01% | +0.91% | 1.039 to 1.051 | simd-lfind +1282296, target-attribute +210145, crc32c +199774 |  | pays |
| `image` | 67 | +0.01% | +0.47% | -19.18% | 1.073 to 1.073 | bundle +344, interpreter-dispatch +205, constant-args +55 | section-attr -25 | pays |
| `sroa` | 487 | +0.92% | +2.47% | +2.58% | 1.150 to 1.160 | simd-lfind +604429, crc32c +92628, target-attribute +72058 | baseline -32, memory-ssa -4, frame-address -3 | pays |
| `simplify` | 1834 | +7.85% | +1.29% | -15.80% | 1.111 to 1.198 | if-conversion +900008395, called-once +29359907, lkmm +27162952 | frame-size -11969, call-motion -4003, prune -1993 | pays |
| `sccp` | 39 | +3.51% | +0.90% | -28.71% | 1.658 to 1.716 | division +18874368, prune +2563, target-attribute +685 | narrow-shift -8388610, bit-liveness -7 | pays |
| `narrow` | 242 | +5.29% | +0.13% | -38.13% | 1.148 to 1.208 | narrow-shift +4194304, lkmm +1382214, simd-lfind +243114 | compare-fold -285, value-settled -252, store-fold-constant -186 | pays |
| `switch-conv` | 37 | +24.69% | +42.04% | +30.07% | 1.098 to 1.369 | switch-dispatch +863680828, switch-lowering +399466137, switch-runs +2433 |  | pays |
| `short-circuit` | 33 | +81.97% | +2.16% | -12.19% | 0.595 to 1.083 | lkmm +15375039, simplify +16, baseline +2 | short-circuit -703, value-range -31 | pays |
| `thread` | 7 | +0.55% | +0.79% | -44.81% | 1.011 to 1.016 | short-circuit +4577 |  | pays |
| `phiopt` | 634 | -0.72% | +2.73% | -52.23% | 1.062 to 1.054 | value-replacement +26980395, lkmm +721388, block-layout +14993 | if-conversion -77265337, loop-idiom -667440, simplify -489160 | trades |
| `rangeswitch` | 1 | +1.12% | +0.78% | -22.74% | 1.006 to 1.018 | short-circuit +1354 |  | pays |
| `rangetest` | 15 | +1.49% | +2.68% | -27.15% | 0.564 to 0.572 | lkmm +239853, short-circuit +6656, value-range +15 |  | pays |
| `thread-copy` | 68 | +0.03% | +1.41% | -64.93% | 1.080 to 1.081 | jump-threading +3226, target-attribute +354, crc32c +222 | short-circuit -579 | pays |
| `prune` | 8 | +24.90% | +6.90% | -28.78% | 0.937 to 1.170 | switch-dispatch +61439998, prune +4486, objtool-shapes +193 |  | pays |
| `canon` | 1504 | +4.74% | -2.60% | -48.50% | 1.125 to 1.178 | if-conversion +360019905, switch-lowering +240000459, switch-dispatch +182886036 | interpreter-dispatch -183661, mitigations -243, computed-goto -164 | trades |
| `header-copy` | 1209 | +4.69% | -1.64% | -54.92% | 1.126 to 1.178 | if-conversion +360019838, switch-lowering +240000380, switch-dispatch +182973179 | lkmm -1238217, section-attr -809, mitigations -246 | trades |
| `licm` | 240 | +0.28% | -0.49% | -59.56% | 1.127 to 1.130 | switch-dispatch +2057508, call-motion +60001, memory-effects +35984 | crc32c -3998, simd-lfind -3362, vla-and-alloca -170 | trades |
| `unroll` | 1133 | +2.34% | -5.81% | -55.45% | 1.040 to 1.064 | simd-lfind +1777038, crc32c +1196029, target-attribute +619927 |  | trades |
| `simplify-cfg` | 2563 | +3.31% | +10.13% | -48.67% | 1.125 to 1.162 | switch-lowering +480000473, called-once +82837381, switch-dispatch +81919811 | division -3973218, lkmm -2728183, interpreter-dispatch -52257 | pays |
| `lanes` | 0 |  |  |  |  |  |  | idle |
| `number` | 1608 | +3.41% | +3.51% | -52.92% | 1.407 to 1.455 | switch-dispatch +30719760, lkmm +5180642, division +2097162 | narrow-shift -8372226, loop-idiom -1401370, memory-effects -16027 | pays |
| `load-forward` | 158 | +0.37% | +0.79% | -64.81% | 1.094 to 1.098 | crc32c +98365, loop-restructure +9476, section-attr +126 | jump-threading -1336, mcmodel-kernel -187, unused-returns -12 | pays |
| `redundant-load` | 350 | -0.12% | -0.57% | -57.99% | 1.012 to 1.011 | memory-effects +35964, dllimport +106, crc32c +36 | frame-size -101882, builtin-setjmp -1179, const-ice -321 | costs |
| `dse` | 263 | +0.01% | +2.46% | -44.08% | 1.001 to 1.001 | overflow-builtins +1201, builtin-setjmp +406, load-forwarding +263 |  | pays |
| `bswap` | 91 | +4.71% | +1.44% | -62.39% | 1.093 to 1.145 | crc32c +900654, target-attribute +35582, compare-fold +2 |  | pays |
| `constant-p` | 5 |  |  |  |  |  |  | flat |
| `subscript` | 38 | +0.01% | +0.97% | -43.99% | 1.006 to 1.006 | bundle +234, store-fold-constant +26, dllimport +19 |  | pays |
| `hoist` | 0 |  |  |  |  |  |  | idle |
| `plane-sink` | 0 |  |  |  |  |  |  | idle |
| `split` | 0 |  |  |  |  |  |  | idle |
| `reassoc` | 730 | +0.05% | +2.85% | -51.25% | 1.003 to 1.003 | simplify +49375, memory-effects +7989, inline +2474 | loop-restructure -3766, conditional-store -1017, frame-size -789 | pays |
| `loop-idiom` | 17 | +35.48% | -2.56% | +19.82% | 2.232 to 3.024 | loop-idiom +2530796, conditional-store +486 |  | trades |
| `vectorize` | 2 | +1.06% | -3.66% | -53.39% | 1.025 to 1.035 | short-circuit +2554 |  | trades |
| `widen` | 698 | -1.58% | +0.10% | -56.47% | 1.278 to 1.258 | value-replacement +13634828, frame-size +3781240, simplify +599033 | division -151068641, narrow-shift -8335362, called-once -20709 | costs |
| `ivopts` | 386 | +3.99% | +0.04% | -58.71% | 1.104 to 1.148 | if-conversion +359999906, switch-lowering +239999939, division +73720 | narrow-shift -16781314, lkmm -1311395, crc32c -49214 | pays |
| `discharge` | 0 |  |  |  |  |  |  | idle |
| `dead-plane` | 0 |  |  |  |  |  |  | idle |
| `coalesce` | 0 |  |  |  |  |  |  | idle |
| `dce` | 195 | -5.14% | -0.10% | -69.58% | 1.634 to 1.550 | simd-lfind +642, bundle +274, sigsetjmp +77 | narrow-shift -8396807, crc32c -16508, short-circuit -519 | costs |
| `gcm` | 195 | +0.25% | +0.82% | -55.95% | 1.059 to 1.062 | frame-size +101666, simd-lfind +27441, value-settled +5633 | conditional-store -2049, section-attr -1023, target-attribute -364 | pays |
| `loop-delete` | 105 | +227.57% | +1.71% | -33.70% | 1.235 to 4.047 | bit-loops +56242402, loop-deletion +40004436, loop-idiom +2402410 | loop-hoist -3 | pays |
| `adce` | 24 | +0.00% | +3.55% | -32.19% | 1.003 to 1.003 | function-purity +40, loop-deletion +30, branch-probability +10 |  | pays |
