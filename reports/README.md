# The corpus report

Every case in the corpus produced the answer the generator computed, on every compiler, at every level.

1461 programs built at `O0`, `O1`, `O2`, `O3` and `Os`, against gcc-16 (Homebrew GCC 16.2.0) 16.2.0. Corpus digest `dbfc0cafbb185c10`.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Homebrew GCC 16.2.0) 16.2.0 | reference |

## How the cases came out

Every count is per case per level, so a corpus of a thousand programs built at five levels has five thousand results in it. The seven verdicts are always all seven and never collapsed into a pass rate, because `unimplemented` is not a failure and `skipped` is not a pass, and a single percentage hides which of those it counted.

| compiler | pass | wrong | rejected | unimplemented | accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|
| `gcc-16` | 7273 | 0 | 0 | 0 | 0 | 0 | 0 |

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
| [What it cost](cost.md) | Code size, size on disk, initialized data, compile time, run time and compiler memory, per facet, each against the reference build of the same program |
| [What went wrong](failures.md) | Nothing this time. Both lists on that page are empty. |
| [By phase of the plan](phases/README.md) | One page per phase, its facets, how they came out, and links to the programs themselves |

## The claims this run checks

| target | wanted | this run | met |
|---|---|---|---|
| `correctness` | every case prints the answer the generator computed, on every compiler, at every level | 0.000 | yes |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1461 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
