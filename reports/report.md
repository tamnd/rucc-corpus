# rucc corpus report

Every case in the corpus produced the answer the generator computed, on every compiler, at every level.

The corpus holds 1461 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `dbfc0cafbb185c10`.

That is 47,898 lines of C, 1.8 MiB, one translation unit per program, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Homebrew GCC 16.2.0) 16.2.0 | reference |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 0 failures | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 7273 | 7273 | 0 | 0 | 0 | 0 | 0 | 0 |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 1453 | level | 1145 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines |
|---|---|---|---|
| floor | 5 | 109 | 3,222 |
| local | 8 | 309 | 16,979 |
| global | 9 | 193 | 4,374 |
| loops | 8 | 402 | 7,827 |
| interprocedural | 6 | 117 | 3,087 |
| backend | 11 | 293 | 10,951 |
| correctness | 2 | 38 | 1,458 |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `atomics` | correctness | 0 | 4026 |
| `bit-builtins` | backend | 0 | 3352 |
| `inline` | interprocedural | 900 | 372 |
| `float-conversion` | backend | 0 | 850 |
| `simplify` | local | 0 | 806 |
| `tail-call` | interprocedural | 324 | 456 |
| `computed-goto` | floor | 0 | 632 |
| `narrowing` | local | 0 | 528 |
| `loop-idiom` | loops | 382 | 86 |
| `loop-restructure` | loops | 275 | 72 |
| `loop-unswitch` | loops | 176 | 164 |
| `loop-unroll` | loops | 172 | 128 |
| `register-pressure` | backend | 75 | 216 |
| `strength` | local | 0 | 288 |
| `selection` | backend | 0 | 252 |
| `load-forwarding` | global | 60 | 184 |
| `loop-rotate` | loops | 224 | 0 |
| `alias-analysis` | global | 66 | 136 |
| `register-alloc` | backend | 104 | 92 |
| `loop-invariant` | loops | 60 | 128 |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1461 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
