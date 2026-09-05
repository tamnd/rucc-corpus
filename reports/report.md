# rucc corpus report

Every case in the corpus produced the answer the generator computed, on every compiler, at every level.

The corpus holds 1018 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `7b4209805ef6afad`.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Homebrew GCC 16.2.0) 16.2.0 | reference |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 0 failures | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|
| `gcc-16` | 5058 | 5058 | 0 | 0 | 0 | 0 | 0 |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases |
|---|---|---|
| floor | 2 | 40 |
| local | 7 | 193 |
| global | 8 | 112 |
| loops | 8 | 392 |
| interprocedural | 6 | 117 |
| backend | 8 | 157 |
| correctness | 1 | 7 |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `inline` | interprocedural | 900 | 372 |
| `tail-call` | interprocedural | 324 | 456 |
| `loop-idiom` | loops | 382 | 86 |
| `loop-restructure` | loops | 275 | 72 |
| `loop-unswitch` | loops | 176 | 164 |
| `loop-unroll` | loops | 172 | 128 |
| `strength` | local | 0 | 288 |
| `selection` | backend | 0 | 252 |
| `loop-rotate` | loops | 224 | 0 |
| `register-alloc` | backend | 104 | 92 |
| `loop-invariant` | loops | 60 | 128 |
| `baseline` | floor | 32 | 131 |
| `reassociate` | local | 0 | 160 |
| `switch-lowering` | backend | 56 | 94 |
| `function-purity` | interprocedural | 80 | 64 |
| `constant-args` | interprocedural | 84 | 48 |
| `if-conversion` | backend | 31 | 98 |
| `loop-deletion` | loops | 64 | 54 |
| `common-subexpr` | global | 12 | 100 |
| `calling-convention` | backend | 44 | 57 |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1018 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
