# The corpus report

20 case results did not come out as expected. They are on the failures page, worst first.

3628 programs, 3,762 files between them and 210,044 lines of C in all, built at `O0`, `O1`, `O2`, `O3` and `Os`, against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee]. Corpus digest `d11e3ac475f4648e`.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.29.1 | under test |

## How the cases came out

Every count is per case per level, so a corpus of a thousand programs built at five levels has five thousand results in it. The seven verdicts are always all seven and never collapsed into a pass rate, because `unimplemented` is not a failure and `skipped` is not a pass, and a single percentage hides which of those it counted.

| compiler | pass | wrong | rejected | unimplemented | accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|
| `gcc-16` | 17508 | 0 | 0 | 0 | 0 | 0 | 600 |
| `rucc` | 17488 | 15 | 5 | 0 | 0 | 0 | 600 |

| verdict | what it means |
|---|---|
| `pass` | It did what the generator worked out that it should do. |
| `wrong` | It compiled, it ran, and it printed something else. This is the one verdict that is unambiguously a bug in the compiler under test. |
| `rejected` | It is valid C and the compiler would not compile it. |
| `unimplemented` | The compiler read it, recognised it, and said in its own words that it has not been taught to lower it. A line on a to-do list rather than a bug. |
| `accepted` | It is not valid C and the compiler compiled it anyway. |
| `crashed` | The compiler failed in a way that was not a diagnostic, which is a crash or a timeout. |
| `skipped` | It was never run, because a tag excluded it or an earlier step failed. |

## The rest of the report

| page | what is on it |
|---|---|
| [What it cost](cost.md) | Code size, size on disk, initialized data, compile time, instructions, run time and compiler memory, per facet, each against the reference build of the same program |
| [What went wrong](failures.md) | 20 failures in full, and nothing a compiler admitted to |
| [By phase of the plan](phases/README.md) | One page per phase, its facets, how they came out, and links to the programs themselves |
| [What each rucc pass did](firing.md) | Every optimizer pass rucc ran, how many cases it fired on and what it did, with the passes that never fired first |

## The claims this run checks

| target | wanted | this run | met |
|---|---|---|---|
| `correctness` | every case prints the answer the generator computed, on every compiler, at every level | 20.000 | no |
| `code-quality:rucc` | the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2, counted over the whole corpus by byte | 1.068 | yes |
| `compile-throughput:rucc` | rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00 | 0.528 | yes |
| `size-model:rucc` | the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2 | 1.000 | yes |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 3628 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
