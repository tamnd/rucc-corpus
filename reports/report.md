# rucc corpus report

Every case in the corpus produced the answer the generator computed, on every compiler, at every level.

The corpus holds 1119 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `13e5bf2f336cab8f`.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.4.2 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 0 failures | yes |
| `code-quality:rucc` | within 10 percent | 12 percent more | no |
| `compile-throughput:rucc` | no worse | 57 percent less | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 5563 | 5563 | 0 | 0 | 0 | 0 | 0 | 0 |
| `rucc` | 5563 | 5558 | 0 | 0 | 5 | 0 | 0 | 0 |

## What is not built yet

These are cases a compiler refused while saying itself that the construct is not implemented. They are valid C and they are counted, and they do not turn the build red, because a compiler that tells you what it has not written yet is doing the right thing. The count going up between two runs is a regression and `rucc-corpus diff` will say so.

### `rucc`

- error: cannot generate code for 'main': no rule lowers a `block_addr` producing a `ptr` [E0653] (5 cases, first is `control-flow.computed-goto.c17.7715e4c1`)

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`, and every number is a median over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the headline.

### `rucc`

Furthest behind:

| facet | phase | cases | code size | run time | compile time |
|---|---|---|---|---|---|
| `scheduling` | backend | 12 | 68 percent more | 5 percent less | 58 percent less |
| `register-alloc` | backend | 40 | 61 percent more | 18 percent less | 52 percent less |
| `simplify` | local | 8 | 54 percent more | 1 percent less | 61 percent less |
| `induction-variable` | loops | 42 | 33 percent more | 2 percent more | 58 percent less |
| `loop-restructure` | loops | 48 | 31 percent more | 5 percent less | 54 percent less |
| `switch-lowering` | backend | 9 | 30 percent more | 26 percent more | 62 percent less |
| `value-range` | global | 13 | 26 percent more | 15 percent more | 41 percent less |
| `calling-convention` | backend | 10 | 24 percent more | 5 percent more | 57 percent less |

Furthest ahead:

| facet | phase | cases | code size | run time | compile time |
|---|---|---|---|---|---|
| `copy-propagation` | global | 12 | 6 percent less | 13 percent more | 38 percent less |
| `reachability` | interprocedural | 9 | 4 percent less | 17 percent more | 51 percent less |
| `dead-code` | local | 12 | 4 percent less | 21 percent more | 57 percent less |
| `selection` | backend | 40 | 2 percent less | 2 percent less | 53 percent less |
| `machine-peephole` | backend | 22 | 1 percent less | 12 percent less | 58 percent less |
| `constant-propagation` | global | 16 | level | 6 percent less | 48 percent less |
| `scalar-replacement` | global | 20 | level | 20 percent less | 52 percent less |
| `dead-store` | local | 12 | level | 1 percent more | 58 percent less |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | `rucc` passed | `rucc` code size |
|---|---|---|---|---|
| floor | 3 | 50 | 213 of 218 | 14 percent more |
| local | 8 | 221 | 1105 of 1105 | 14 percent more |
| global | 8 | 165 | 825 of 825 | 6 percent more |
| loops | 8 | 402 | 2010 of 2010 | 14 percent more |
| interprocedural | 6 | 117 | 585 of 585 | 13 percent more |
| backend | 8 | 157 | 785 of 785 | 19 percent more |
| correctness | 1 | 7 | 35 of 35 | 4 percent more |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `inline` | interprocedural | 900 | 372 |
| `tail-call` | interprocedural | 300 | 480 |
| `loop-idiom` | loops | 395 | 76 |
| `loop-unswitch` | loops | 134 | 212 |
| `loop-unroll` | loops | 196 | 128 |
| `loop-restructure` | loops | 262 | 60 |
| `strength` | local | 0 | 288 |
| `narrowing` | local | 0 | 288 |
| `load-forwarding` | global | 60 | 184 |
| `selection` | backend | 0 | 244 |
| `loop-rotate` | loops | 224 | 0 |
| `alias-analysis` | global | 66 | 136 |
| `loop-invariant` | loops | 60 | 128 |
| `reassociate` | local | 0 | 160 |
| `induction-variable` | loops | 131 | 24 |
| `switch-lowering` | backend | 52 | 100 |
| `function-purity` | interprocedural | 80 | 64 |
| `if-conversion` | backend | 17 | 126 |
| `baseline` | floor | 34 | 99 |
| `constant-args` | interprocedural | 84 | 44 |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1119 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
