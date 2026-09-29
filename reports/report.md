# rucc corpus report

Every case in the corpus produced the answer the generator computed, on every compiler, at every level.

The corpus holds 3241 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `e55eb59fd1fd2382`.

That is 142,580 lines of C, 4.6 MiB, in 3,309 files, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.14.1 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 0 failures | yes |
| `code-quality:rucc` | within 10 percent | 52 percent more | no |
| `compile-throughput:rucc` | no worse | 37 percent less | yes |
| `size-model:rucc` | no worse | level | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2, counted over the whole corpus by byte.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.
- `size-model:rucc`: the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 16173 | 16173 | 0 | 0 | 0 | 0 | 0 | 0 |
| `rucc` | 16173 | 16168 | 0 | 0 | 5 | 0 | 0 | 0 |

## What is not built yet

These are cases a compiler refused while saying itself that the construct is not implemented. They are valid C and they are counted, and they do not turn the build red, because a compiler that tells you what it has not written yet is doing the right thing. The count going up between two runs is a regression and `rucc-corpus diff` will say so.

### `rucc`

- error: `__atomic_signal_fence` is not implemented yet [E0686] (5 cases, first is `atomics.fence.c17.6b7c6e1f`)

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`. The per facet figures are medians over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the facet's number. The headline for each compiler is the corpus total instead, every byte counted once, because that is the number that does not move when somebody adds a facet.

The run time column is the same comparison over the fastest of the repetitions of each program, and it says `inside the noise` where the difference between the two compilers is smaller than the difference this machine produced running one of them several times. Size is exact and time is not, so on most of these facets the size column is the honest signal and the time column is there to be checked rather than quoted. Every repetition is in `runs.jsonl` under `execute.samples`, so any number here can be traced back to what was measured.

The instructions column is how many instructions this compiler's programs ran across the facet, less how many the reference's ran. A count rather than a clock, so it comes back the same to within one part in a hundred thousand on a machine where the wall clock moves by a factor of twelve. It is a difference rather than a ratio on purpose. A program in this corpus retires about a hundred and forty thousand instructions and about a hundred and eight thousand of those are the process starting up before `main`, which is the same code on both sides, so a ratio has all of that in its denominator and comes out at one however the compilers differed. A difference has none of it, because a constant on both sides subtracts away exactly. The ratio is still in `report.json` as `instruction_ratio` for anyone who wants to check that. The column says `not measured` where the machine would not count, which is any machine without `perf` and any machine that will not hand a counter to an unprivileged process.

### `rucc`

Over the whole corpus, `rucc` produces 52 percent more. The middle facet is 1 percent less, over 89 facets. The two differ when the larger facets are the ones going badly, and the total is the one to believe.

Furthest behind:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `simd-lfind` | correctness | 30 | 188 percent more | not measured | inside the noise | 46 percent less |
| `overflow-builtins` | correctness | 144 | 109 percent more | not measured | level | 42 percent less |
| `bit-builtins` | backend | 22 | 83 percent more | not measured | inside the noise | 39 percent less |
| `register-alloc` | backend | 40 | 64 percent more | not measured | inside the noise | 36 percent less |
| `long-double` | backend | 10 | 49 percent more | not measured | inside the noise | 38 percent less |
| `scheduling` | backend | 20 | 43 percent more | not measured | inside the noise | 37 percent less |
| `loop-restructure` | loops | 48 | 41 percent more | not measured | inside the noise | 33 percent less |
| `register-pressure` | backend | 60 | 30 percent more | not measured | inside the noise | 33 percent less |

Furthest ahead:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `narrow-shift` | backend | 8 | 33 percent less | not measured | inside the noise | 56 percent less |
| `short-circuit` | local | 22 | 32 percent less | not measured | inside the noise | 45 percent less |
| `conditional-store` | local | 20 | 31 percent less | not measured | inside the noise | 45 percent less |
| `prune` | global | 15 | 28 percent less | not measured | inside the noise | 42 percent less |
| `value-replacement` | global | 13 | 26 percent less | not measured | 96 percent more | 45 percent less |
| `value-settled` | local | 13 | 21 percent less | not measured | inside the noise | 44 percent less |
| `vla-and-alloca` | floor | 9 | 20 percent less | not measured | inside the noise | 49 percent less |
| `iv-selection` | loops | 36 | 18 percent less | not measured | level | 44 percent less |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 3233 | 4 percent less | 1138 |
| `rucc` | 3232 | level | 938 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

### `rucc` at `-Os`

Where `-Os` saves the most:

| facet | cases | code size at `-Os` |
|---|---|---|
| `loop-restructure` | 48 | 32 percent less |
| `scheduling` | 20 | 18 percent less |
| `stack-slots` | 6 | 17 percent less |
| `simd-lfind` | 30 | 16 percent less |
| `strength` | 24 | 7 percent less |
| `crc32c` | 64 | 6 percent less |

Where it saves the least, which is where it is spending size and getting nothing for it when the number is above level:

| facet | cases | code size at `-Os` |
|---|---|---|
| `loop-invariant` | 32 | 3 percent more |
| `function-purity` | 28 | 4 percent more |
| `tail-dispatch` | 5 | 8 percent more |
| `loop-unroll-shape` | 80 | 8 percent more |
| `loop-rotate` | 64 | 8 percent more |
| `loop-deletion` | 64 | 9 percent more |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines | `rucc` passed | `rucc` code size |
|---|---|---|---|---|---|
| floor | 6 | 118 | 3,458 | 558 of 558 | 3 percent less |
| local | 11 | 409 | 20,476 | 2045 of 2045 | 1 percent less |
| global | 12 | 242 | 5,794 | 1210 of 1210 | 3 percent less |
| loops | 12 | 700 | 14,151 | 3500 of 3500 | 1 percent less |
| interprocedural | 14 | 351 | 10,173 in 419 files | 1755 of 1755 | level |
| backend | 26 | 967 | 37,226 | 4835 of 4835 | 2 percent less |
| correctness | 8 | 454 | 51,302 | 2265 of 2270 | 16 percent more |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `crc32c` | correctness | 7840 | 61488 |
| `sigsetjmp` | correctness | 1128 | 17244 |
| `overflow-builtins` | correctness | 0 | 14400 |
| `target-attribute` | correctness | 2505 | 7220 |
| `frame-size` | backend | 2191 | 2163 |
| `atomics` | correctness | 0 | 4026 |
| `bit-builtins` | backend | 0 | 3352 |
| `simd-lfind` | correctness | 1290 | 1386 |
| `tail-dispatch` | interprocedural | 0 | 1792 |
| `simplify` | local | 161 | 1135 |
| `inline` | interprocedural | 900 | 372 |
| `store-fold-constant` | backend | 444 | 820 |
| `compare-fold` | backend | 472 | 584 |
| `division` | backend | 203 | 802 |
| `float-conversion` | backend | 0 | 850 |
| `tail-call` | interprocedural | 300 | 480 |
| `call-motion` | interprocedural | 10 | 720 |
| `computed-goto` | floor | 32 | 600 |
| `store-fold` | backend | 212 | 400 |
| `switch-dispatch` | backend | 67 | 526 |

## What each switch became

Every case with a `switch` in it is compiled once more with `-S` to see what its switches were lowered as. rucc says so under `-fopt-info`, and gcc-16's assembly is read for a jump table, a bit test or a lookup table of answers, with compares meaning none of the three. A case agrees when both compilers used the same of those. The two do not have to inline the same functions, so a disagreement is a lead to read the assembly for and not a verdict.

| compiler | level | cases | same | different |
|---|---|---|---|---|
| rucc | O0 | 77 | 51 | 26 |
| rucc | O1 | 75 | 68 | 7 |
| rucc | O2 | 75 | 68 | 7 |
| rucc | O3 | 75 | 68 | 7 |
| rucc | Os | 75 | 69 | 6 |

### rucc at O0 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| table | compares | 26 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `switch-dispatch.affine.in-order.c17.3de0f02c` | `switch-dispatch` | table | compares |
| `switch-dispatch.affine.unpredictable.c17.1095fab2` | `switch-dispatch` | table | compares |
| `switch-dispatch.below-zero.unpredictable.c17.0348e7c3` | `switch-dispatch` | table | compares |
| `switch-dispatch.holes.unpredictable.c17.343584c6` | `switch-dispatch` | table | compares |
| `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | `switch-dispatch` | table | compares |
| `switch-dispatch.masked.unpredictable.c17.fcb9805f` | `switch-dispatch` | table | compares |
| `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | `switch-dispatch` | table | compares |
| `switch-dispatch.names.unpredictable.c17.cd05ba1f` | `switch-dispatch` | table | compares |
| `switch-dispatch.near-the-edge.unpredictable.c17.f7a7f236` | `switch-dispatch` | table | compares |
| `switch-dispatch.negative-answers.unpredictable.c17.45c3408b` | `switch-dispatch` | table | compares |
| `switch-dispatch.scattered.in-order.c17.0eea8687` | `switch-dispatch` | table | compares |
| `switch-dispatch.scattered.unpredictable.c17.13757964` | `switch-dispatch` | table | compares |
| `switch-dispatch.shared-default.unpredictable.c17.e5a3b685` | `switch-dispatch` | table | compares |
| `switch-dispatch.wide-answers.unpredictable.c17.7659b1be` | `switch-dispatch` | table | compares |
| `switch-lowering.dense.31.c17.d18e4de0` | `switch-lowering` | table | compares |
| `switch-lowering.dense.33.c17.81ec76e2` | `switch-lowering` | table | compares |
| `switch-lowering.dense.40.c17.85e83015` | `switch-lowering` | table | compares |
| `switch-lowering.dense.hot-case.75.none.c17.57f4af9a` | `switch-lowering` | table | compares |
| `switch-lowering.dense.hot-case.75.right.c17.34253fd4` | `switch-lowering` | table | compares |
| `switch-lowering.dense.hot-case.75.wrong.c17.d50a50bd` | `switch-lowering` | table | compares |
| `switch-lowering.dense.hot-case.95.none.c17.126eab91` | `switch-lowering` | table | compares |
| `switch-lowering.dense.hot-case.95.right.c17.38920e87` | `switch-lowering` | table | compares |
| `switch-lowering.dense.hot-case.95.wrong.c17.22be52ce` | `switch-lowering` | table | compares |
| `switch-runs.several-runs.31.c17.02b4c07b` | `switch-runs` | table | compares |
| `switch-runs.several-runs.33.c17.9b43c0a4` | `switch-runs` | table | compares |

And 1 more, all in `reports/report.json`.

### rucc at O1 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | table | 7 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `switch-dispatch.eight-labels.unpredictable.c17.9cb88f09` | `switch-dispatch` | compares | table |
| `switch-dispatch.five-labels.unpredictable.c17.912748f9` | `switch-dispatch` | compares | table |
| `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | `switch-dispatch` | compares | table |
| `switch-dispatch.seven-labels.unpredictable.c17.95b72a1c` | `switch-dispatch` | compares | table |
| `switch-dispatch.six-labels.unpredictable.c17.98c71a5e` | `switch-dispatch` | compares | table |
| `switch-lowering.dense.8.c17.35614fce` | `switch-lowering` | compares | table |
| `switch-runs.look-alike-arms.five.c17.641be602` | `switch-runs` | compares | table |

### rucc at O2 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | table | 4 |
| lookup | compares | 3 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | `switch-dispatch` | compares | table |
| `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | `switch-dispatch` | compares | table |
| `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | `switch-dispatch` | compares | table |
| `switch-dispatch.names.unpredictable.c17.cd05ba1f` | `switch-dispatch` | compares | table |
| `switch-runs.classifier.char.c17.b072f937` | `switch-runs` | lookup | compares |
| `switch-runs.classifier.unsigned-char.c17.3ae9dc72` | `switch-runs` | lookup | compares |
| `switch-runs.several-runs.2.c17.c52ca973` | `switch-runs` | lookup | compares |

### rucc at O3 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | table | 4 |
| lookup | compares | 3 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | `switch-dispatch` | compares | table |
| `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | `switch-dispatch` | compares | table |
| `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | `switch-dispatch` | compares | table |
| `switch-dispatch.names.unpredictable.c17.cd05ba1f` | `switch-dispatch` | compares | table |
| `switch-runs.classifier.char.c17.b072f937` | `switch-runs` | lookup | compares |
| `switch-runs.classifier.unsigned-char.c17.3ae9dc72` | `switch-runs` | lookup | compares |
| `switch-runs.several-runs.2.c17.c52ca973` | `switch-runs` | lookup | compares |

### rucc at Os against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | table | 3 |
| lookup | compares | 3 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | `switch-dispatch` | compares | table |
| `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | `switch-dispatch` | compares | table |
| `switch-dispatch.names.unpredictable.c17.cd05ba1f` | `switch-dispatch` | compares | table |
| `switch-runs.classifier.char.c17.b072f937` | `switch-runs` | lookup | compares |
| `switch-runs.classifier.unsigned-char.c17.3ae9dc72` | `switch-runs` | lookup | compares |
| `switch-runs.several-runs.2.c17.c52ca973` | `switch-runs` | lookup | compares |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 3241 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
