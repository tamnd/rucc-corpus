# rucc-corpus

A C corpus for proving that an optimization in [rucc](https://github.com/tamnd/rucc) is correct and that it actually optimizes.

Every program here was written for exactly one named transformation. Every program prints an answer that this repository worked out in Rust before any C compiler was involved. Every program prints nothing that depends on the machine it runs on, so one expected answer is right everywhere.

1572 programs across 54 facets, grouped by the phases of the M4 plan.

## Where it stands

Everything between the two markers below is written by the last run. Everything else on this page is prose that a person wrote, and the generator leaves it alone.

<!-- corpus:begin -->

1831 programs, 1,855 files between them and 57,801 lines of C in all, built at `O0`, `O1`, `O2`, `O3` and `Os`, against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee]. Corpus digest `096ade4ec00e39a6`.

| compiler | passed | wrong | rejected | not built yet | crashed |
|---|---|---|---|---|---|
| `gcc-16` | 9123 of 9123 | 0 | 0 | 0 | 0 |
| `rucc` | 8883 of 9123 | 5 | 100 | 130 | 5 |

The full report is in [reports/README.md](reports/README.md). What each facet cost is in [reports/cost.md](reports/cost.md), what went wrong is in [reports/failures.md](reports/failures.md), and the breakdown by phase of the plan is in [reports/phases/README.md](reports/phases/README.md).

<!-- corpus:end -->

## Why the answers are computed rather than compared

The obvious way to test a compiler is to run the program through two compilers and check they agree. That is what [rucc-compat](https://github.com/tamnd/rucc-compat) does, and it is good at what it does, but it cannot catch a bug that both compilers have and it cannot tell you which optimization was responsible for anything.

Here the generator evaluates the program itself, in Rust, with its own integer semantics, and writes the answer into the manifest. GCC never gets asked. So if rucc and GCC 16 both fold a shift wrongly in the same direction, this corpus still goes red, and when it does the failing case is named `strength.u32.shift-by-31` rather than being one of a hundred thousand random programs.

The other half of the same decision: a case whose value the generator cannot work out without guessing is dropped rather than emitted. `eval` returns nothing when a program would be undefined, and a case with no answer is not a case. That is why there is no signed overflow anywhere in here and no shift past the width of a promoted type.

## Why the programs are systematic rather than random

Csmith and YARPGen are excellent and they answer a different question. A random program that miscompiles tells you the compiler is wrong somewhere. It does not tell you that loop unrolling is wrong at trip count seven, because a random program does not have a trip count of seven on purpose.

Every facet here walks named axes. `loop-unroll` walks four integer types by eight trip counts by known and unknown bounds, which is 64 programs, and the one that fails names the point it failed at. That is what makes a corpus run into evidence about a pass rather than evidence about a compiler.

## What is in here

```
programs/
  floor/             baseline, control flow shapes, branch probability, computed goto, run time sized objects and frontend, which have to be right before anything else means much
  local/             constant folding, strength reduction, width narrowing, dead code, the peephole rules
  global/            common subexpressions, alias analysis, memory SSA, propagation, scalar replacement
  loops/             loop shape, invariant motion, induction variables, unrolling, unswitching, idioms
  interprocedural/   inlining, tail calls, purity, specialization, reachability, and what none of them can do until the compiler has both files
  backend/           selection, register pressure and allocation, scheduling, layout, switch lowering, the widest floating type
  correctness/       programs where the compiler must not act, and acting is the bug, including atomics and the jump out of a frame
```

Each directory has a `README.md` listing every program in it, the axis point it was generated for, and the output it must produce. Read that first. The expected answer is the interesting half and it is deliberately not in the C file, because the file that is compiled has to be the file that is in the repository, byte for byte, and a comment carrying the answer would be a second copy that could drift.

The C is generated. Editing a file under `programs/` changes nothing, because the next `gen` writes it back and CI checks that nothing was hand-edited. The thing to edit is `crates/corpus-gen`. The output is kept in the repository anyway, so that a change to the generator shows up as a diff of the thousand programs it altered, which is the only practical way to review one.

## The reports

`reports/` is regenerated nightly, so `git log reports/report.md` is the history of the compiler getting better or worse.

| file | who reads it | committed |
|---|---|---|
| `report.md` | a person | yes |
| `report.json` | a tool, and `rucc-corpus diff` | no, a workflow artifact |
| `runs.jsonl` | one line per build, for when the summary is not enough | no, a workflow artifact |
| `findings.sarif` | GitHub code scanning, so a failure lands on the line | no, a workflow artifact |

`report.md` says what the compiler got wrong, how its code size compares to GCC 16 per facet, and which facets it is furthest behind on. Losses come before wins in that report on purpose.

The same numbers are also rendered as a small tree of pages, which is the version to read rather than to scroll.

| page | what is on it |
|---|---|
| [reports/README.md](reports/README.md) | The verdict, the compilers, how every case came out, and the way in to the rest |
| [reports/cost.md](reports/cost.md) | Code size, size on disk, initialized data, compile time, run time and compiler memory, per facet, each against the GCC 16 build of the same program, with how much C the facet is beside them |
| [reports/failures.md](reports/failures.md) | Every failure in full, and every case a compiler said it has not built yet, grouped |
| [reports/phases/README.md](reports/phases/README.md) | One page per phase of the plan, with its facets and links to the programs |

The block on the front page and every one of those files is regenerated by `run`. The front page is spliced between two markers rather than rewritten, so the prose around it survives.

**Every page says how much C it is talking about.** The corpus is generated, so nobody arrives with the feel for its size that they would have for a project they had checked out, and a compile time or a peak memory figure with no size beside it is a number a reader can do nothing with. Four hundred milliseconds is quick for ten thousand lines and slow for two hundred. So the front page and the hub say how many lines the whole corpus is, the cost page and both phase pages carry a `lines` column per facet, and `report.json` carries the same figures for a tool that wants to tell a compile time that moved because the compiler changed from one that moved because the corpus grew. Each is counted once per case however many compilers and levels the case was built with, since a sum over records would report a corpus of a thousand programs at ten times its size. It is a denominator and never a score, so there is no lines per second anywhere in the report.

The file count is kept alongside the line count and mentioned only where it is not the number of programs, which is to say only for the facet whose whole subject is the file boundary. Printing it everywhere would put a column of the same figure twice on every page.

**Only the markdown is committed.** The other three are regenerated on every run whether or not anything about the compiler moved, so committing them makes the history churn and makes the diff something nobody reads. They go up as artifacts on the nightly instead, kept for the default retention, and `rucc-corpus diff` on the nightly reads yesterday's from there rather than from the tree. The same rule covers `programs/manifest.json`, which is the machine readable copy of a program list that the `README.md` files already state in a form a person can read.

## Twenty of the programs are more than one file

Every case in here is a single translation unit apart from the `link-time-optimization` facet, and that default is worth defending. A program the compiler saw all of at once is a program where a wrong answer is about code generation and nothing else, which is most of what makes a failing case worth reading.

The exception exists because a whole class of transformations only lives at the boundary between two files. Inlining across one, propagating a constant across one, turning an indirect call into a direct one across one, and dropping a function nothing calls across one are all things a compiler can only do once somebody has handed it both files together. That is what `-flto` is for and it cannot be tested with one file.

So ten shapes are each generated twice. One half is compiled and linked the ordinary way and the other half is built with `-flto`, and both have to print the same answer. The pair is the point. An `-flto` case on its own that passes says the compiler accepted a flag, which is not the question. The pair says that turning whole program optimization on did not change the meaning of the program, which is the thing that actually breaks, and the two rows sit next to each other in the report with their code sizes beside them so what LTO bought is a subtraction rather than a claim.

`-flto` is a property of those cases and not a sixth optimization level. A level is something every program in the corpus is built at, and asking all 1572 of them for LTO would multiply the whole run to learn one facet's answer. The case carries the flag, the facet index says which cases carry it, and the axis on the case says which half of the pair it is.

## What `-Os` does

Every program is built at `-Os` alongside the other four levels, and the nightly report has a section saying what that bought. It is the one measurement in the report that compares a compiler against itself rather than against the reference, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites and the question worth asking is whether it picks any. The number is a compiler's own code size at `-Os` over its own code size at `-O2`, as a median, alongside a count of the cases that came out at exactly the same number of bytes.

That count is the point. A compiler whose `-Os` output is byte for byte its `-O2` output has accepted the flag and ignored it, and a median of one on its own does not tell those two apart from a compiler that genuinely had nothing left to save. The `size-model:<compiler>` target fails on either, and GCC 16's own row is printed beside it as the control, since it is a compiler with a size cost model that works.

The per-commit job builds `-O0` and `-O2` only, so it does not produce this section or that target. Nightly builds all five levels and does.

## What GCC says about these programs

Every reference build is run with `-fopt-info-all`, and what GCC said is parsed and kept. It is not a pass or fail signal. GCC missing something is not a bug in GCC and GCC taking something is not a requirement on rucc.

It is worth keeping because it answers a question nothing else does: whether the transformation a case was written for is available in that program at all. A case rucc leaves alone because the pass is missing and a case rucc leaves alone because there was nothing there to do look identical in a size number and completely different here. That is the input to deciding what to implement next.

Lines about `printf` are dropped. Every program in the corpus calls it and GCC says the same two things about it every time, which buries everything else.

## Running it

```sh
cargo run --release -p rucc-corpus -- list
cargo run --release -p rucc-corpus -- gen --clean
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 --toolchain rucc=../rucc/target/release/rucc \
    --reference gcc-16
cargo run --release -p rucc-corpus -- diff old-report.json reports/report.json
```

Narrow it down while working on one pass:

```sh
cargo run --release -p rucc-corpus -- run --facet loop-unroll --level O2 --keep
```

`--keep` leaves the source, the binary and what the compiler said in a directory per case, so the next question after a failure can be answered by looking rather than by running it again by hand.

The reference is GCC 16, which is the current release. Homebrew installs it as `gcc-16` and the Ubuntu toolchain archive installs it under the same name.

## A case that has not changed is not built again

A result whose every input hashes to what it hashed last time is read out of a cache instead of being built. The key covers the source, the expected answer, the dialect, the level, the compiler as bytes and as a version string, the extra flags, the repeat count, the operating system, the architecture and the version of the harness. Change any one of them and the entry misses.

That is worth having because the reference column almost never moves. GCC 16 built these programs yesterday and will build them the same way tomorrow, so on a normal day the only new work in a run is the compiler under test. A narrow run that took eighteen seconds cold takes under a second warm.

```sh
cargo run --release -p rucc-corpus -- run --refresh    # build everything, keep the results
cargo run --release -p rucc-corpus -- run --no-cache   # neither read nor write
```

Three things are true of it and all three matter.

Nothing partial is ever reused. A result is either built from nothing or handed back whole. There is no third case where a build starts from something an earlier run left behind, because that is the case whose numbers belong to neither run.

A reused record says it was reused. Every record carries the flag, `report.json` carries a `measured` block with the two counts in it, and every page that quotes a time says how many of its numbers were not taken today. An outcome keeps. A timing does not.

The nightly runs `--refresh`, so its numbers were all measured on one machine in one hour, and it still leaves the cache warm for whoever runs next.

The cache lives in `$CORPUS_CACHE`, or under `~/.cache/rucc-corpus` when that is not set. Deleting it loses nothing that cannot be measured again.

## rucc only has an x86-64 Linux back end

As of today that is the only target rucc generates code for, so the rucc column of the report can only be produced on x86-64 Linux. On any other machine the corpus still runs against GCC 16, which is worth doing on its own, because that is what checks 1572 expected answers against a compiler that has been wrong about very few things since 1987.

CI produces the rucc column. Locally, on anything that is not x86-64 Linux, leave `--toolchain rucc` off.

## No dependencies

The whole thing is `std` and nothing else. The JSON writer, the JSON parser, the SHA-256, the process runner with its timeouts, and the ELF and Mach-O section reader are all in here and all tested.

That is not minimalism for its own sake. `report.json` is a contract other tools read and the corpus is evidence about a compiler, so the number of things between the measurement and the claim should be small enough to read. It also means this builds anywhere a Rust toolchain does, with no network.

## The crates

| crate | what it holds |
|---|---|
| `corpus-model` | cases, facets, records, the JSON writer and parser, SHA-256 |
| `corpus-gen` | the generator, one module per phase, and the evaluator that computes every answer |
| `corpus-run` | building, running, timing, sizing the code, and reading what GCC said |
| `corpus-report` | the human report, the machine report, SARIF, and the terminal progress |
| `rucc-corpus` | the command line |

## Licence

Apache-2.0.
