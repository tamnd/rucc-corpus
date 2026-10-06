# rucc-corpus

A C corpus for proving that an optimization in [rucc](https://github.com/tamnd/rucc) is correct and that it actually optimizes.

Every program here was written for exactly one named transformation. Every program prints an answer that this repository worked out in Rust before any C compiler was involved. Every program prints nothing that depends on the machine it runs on, so one expected answer is right everywhere.

1572 programs across 54 facets, grouped by the phases of the M4 plan.

## Where it stands

Everything between the two markers below is written by the last run. Everything else on this page is prose that a person wrote, and the generator leaves it alone.

<!-- corpus:begin -->

3626 programs, 3,760 files between them and 209,990 lines of C in all, built at `O0`, `O1`, `O2`, `O3` and `Os`, against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee]. Corpus digest `8e2e474ec8da0fa1`.

| compiler | passed | wrong | rejected | not built yet | crashed |
|---|---|---|---|---|---|
| `gcc-16` | 17498 of 17498 | 0 | 0 | 0 | 0 |
| `rucc` | 17478 of 17498 | 15 | 5 | 0 | 0 |

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
| [reports/cost.md](reports/cost.md) | Code size, size on disk, initialized data, compile time, instructions, run time and compiler memory, per facet, each against the GCC 16 build of the same program, with how much C the facet is beside them |
| [reports/failures.md](reports/failures.md) | Every failure in full, and every case a compiler said it has not built yet, grouped |
| [reports/phases/README.md](reports/phases/README.md) | One page per phase of the plan, with its facets and links to the programs |

The block on the front page and every one of those files is regenerated by `run`. The front page is spliced between two markers rather than rewritten, so the prose around it survives.

**Every page says how much C it is talking about.** The corpus is generated, so nobody arrives with the feel for its size that they would have for a project they had checked out, and a compile time or a peak memory figure with no size beside it is a number a reader can do nothing with. Four hundred milliseconds is quick for ten thousand lines and slow for two hundred. So the front page and the hub say how many lines the whole corpus is, the cost page and both phase pages carry a `lines` column per facet, and `report.json` carries the same figures for a tool that wants to tell a compile time that moved because the compiler changed from one that moved because the corpus grew. Each is counted once per case however many compilers and levels the case was built with, since a sum over records would report a corpus of a thousand programs at ten times its size. It is a denominator and never a score, so there is no lines per second anywhere in the report.

The file count is kept alongside the line count and mentioned only where it is not the number of programs, which is to say only for the facet whose whole subject is the file boundary. Printing it everywhere would put a column of the same figure twice on every page.

**Only the markdown is committed.** The other three are regenerated on every run whether or not anything about the compiler moved, so committing them makes the history churn and makes the diff something nobody reads. They go up as artifacts on the nightly instead, kept for the default retention, and `rucc-corpus diff` on the nightly reads yesterday's from there rather than from the tree. The same rule covers `programs/manifest.json`, which is the machine readable copy of a program list that the `README.md` files already state in a form a person can read.

## Experiments

`experiments/` holds the measurements the plan asks for once, to settle a question, rather than every night. Each is a page for a person and a `results.json` for a tool, and both are committed, because unlike the nightly reports they only change when the experiment is run again, and then the diff is the point.

| experiment | what it settled |
|---|---|
| [07-rewriters](experiments/07-rewriters/README.md) | Experiment 7 of the optimizer plan: the conventional pipeline against the classical, hash-consed and e-graph rewriters, which answers open question one. The conventional pipeline ships. |
| [60-constant-sweep](experiments/60-constant-sweep/README.md) | Experiment 60 of the optimizer plan: every row of `rucc --print-params` at half and at double its value. 63 rows are flat, 16 are worth tuning and 9 are wrong, each with an issue in tamnd/rucc. |

## Some of the programs are more than one file

Every case in here is a single translation unit apart from the `link-time-optimization`, `bundle` and `dllimport` facets, and that default is worth defending. A program the compiler saw all of at once is a program where a wrong answer is about code generation and nothing else, which is most of what makes a failing case worth reading.

The exception exists because a whole class of transformations only lives at the boundary between two files. Inlining across one, propagating a constant across one, turning an indirect call into a direct one across one, and dropping a function nothing calls across one are all things a compiler can only do once somebody has handed it both files together. That is what `-flto` is for and it cannot be tested with one file.

So ten shapes are each generated twice. One half is compiled and linked the ordinary way and the other half is built with `-flto`, and both have to print the same answer. The pair is the point. An `-flto` case on its own that passes says the compiler accepted a flag, which is not the question. The pair says that turning whole program optimization on did not change the meaning of the program, which is the thing that actually breaks, and the two rows sit next to each other in the report with their code sizes beside them so what LTO bought is a subtraction rather than a claim.

`-flto` is a property of those cases and not a sixth optimization level. A level is something every program in the corpus is built at, and asking all 1572 of them for LTO would multiply the whole run to learn one facet's answer. The case carries the flag, the facet index says which cases carry it, and the axis on the case says which half of the pair it is.

The `bundle` facet is the other exception, and its files are not all linked into one program. Each case is an executable and one or more modules it opens with `dlopen`, which is the shape of every Postgres extension: the module calls functions and reads globals that the executable defines and leaves them undefined in itself, and the executable calls the hooks and callbacks the module registers. A unit in a case says which it is, linked or a module, and the harness knows the link for each target. On Linux the executable is built with `-rdynamic` and `-ldl`, and each module with `-fPIC -shared`. On macOS each module is a bundle built with `-fPIC -bundle -bundle_loader` naming the executable, so the executable is built first. Every module is written beside the executable as its name with `.so` on the end, which is the path the program opens. On Windows the executable is linked with `-Wl,--export-all-symbols -Wl,--out-implib,libcase.a`, which is how MinGW builds of Postgres link the server, and each module with `-shared` against that import library. There is no `dlopen` on Windows, so every one of these cases carries the `dlopen` tag and a run there leaves them out with `--exclude-tag dlopen`.

The `dllimport` facet is the same boundary as Windows has it. Everything the executable shares is declared `PGDLLIMPORT` in the module, which is `__declspec(dllimport)` on Windows and nothing anywhere else, and everything the executable looks up in a module is declared `PGDLLEXPORT`. A case loads its modules with `LoadLibraryA` on Windows and with `dlopen` everywhere else, so it carries no `dlopen` tag and the same expected answers are checked by GCC 16 on Linux and by MinGW GCC on Windows. A job on GitHub's `windows-2022` runner builds it, and `builtin-setjmp`, with MSYS2's UCRT64 GCC and with rucc, which is told its target and its sysroot with `--flag rucc=--target=x86_64-windows-gnu`.

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

## Moving the thresholds

Every number a rucc pass decides by is a row of `rucc --print-params`, and `--param=NAME=VALUE` moves one for a single run. `rucc-corpus sweep` moves each row to half and to double its value, one at a time, and measures what that did:

```sh
cargo run --release -p rucc-corpus -- sweep \
    --rucc ../rucc/target/release/rucc --reference gcc-16=gcc-16
```

It compiles every case to assembly once as rucc is and once per move, so only the cases whose assembly changed are built and run, against rucc as it is and gcc 16 at the same level. `reports/sweep/index.md` has a line per move with the cases it changed, the change in instructions retired, text and compile time, and the facets that moved most, and calls each row flat, worth tuning, or wrong, which is when a value the sweep tried saves on instructions or text and spends on neither. A move that makes a case print the wrong answer is listed on its own. `sweep.json` is the same for a tool, and `sweep.jsonl` keeps each move as it finishes, so a sweep that stops picks up where it was with the same compiler and corpus. `--row` and `--facet` narrow it down, and `--level` picks a level other than `-O2`.

## Turning each pass off

`rucc-corpus passoff` is the same machinery with a pass in place of a threshold. It reads the passes from `rucc -O2 --print-pipeline`, turns each off on its own with `-fdisable-NAME`, and builds and runs only the cases whose assembly changed:

```sh
cargo run --release -p rucc-corpus -- passoff \
    --rucc ../rucc/target/release/rucc --reference gcc-16=gcc-16
```

`reports/passoff/index.md` has a line per pass with the cases it changed, what turning it off did to instructions retired, text and compile time, the facets the pass helps and hurts most, and a call: it pays, trades, costs, is flat, or is idle when turning it off changes no case at all. A case that prints the wrong answer with a pass off is a bug in a later pass and is listed on its own. The numbers for one pass are not its share of the pipeline, because passes enable each other, and the page says so. `--pass` narrows it down, and it resumes from `passoff.jsonl` the way the sweep does.

## What a run time in here is worth

Every case is run five times by default and every one of those times is kept, in `runs.jsonl` under `execute.samples`. The number the reports compare is the fastest of them, because on a machine that is doing anything else the slower measurements contain somebody else's work and there is no way to subtract it. The rest are kept so the floor can be argued with.

The reason they are kept is that the floor on its own hides how much the machine moved while it was being measured. Two runs of this corpus over byte for byte identical programs moved the total wall time by nearly nine percent, with more than fifteen hundred programs moving by more than five percent each. Nothing about those programs changed. So a report that prints a one percent speedup out of samples that spread by nine is not reporting a speedup, and both the cost page and the human report say `inside the noise` instead of printing the number when the difference between two compilers is smaller than the spread within one of them. `report.json` carries `speed_spread` and `speed_is_real` per facet for anyone who wants the arithmetic.

Size is exact and time is not. For most facets in here the size columns are the honest signal, and the time columns are there to be checked rather than quoted. A claim about how fast something got wants a benchmark built for it, on a quiet machine, counting instructions rather than seconds.

So where the machine will count, the corpus counts. On Linux with `perf` available to an unprivileged process, every program that runs is also measured five times under `perf stat -e instructions:u`, the smallest is kept, and all five go in `runs.jsonl` under `execute.instructions` and `execute.instruction_samples`. That number is what the program did rather than how long the machine took to let it do it. On the machine where five repetitions of one unchanged binary spread from 1665 to 21147 microseconds, twenty five readings of its instruction count came back 112164 nineteen times and 112183 or above six times. The counts are not a bell curve, they are a floor and a tail of work the program did not ask for, which is why the smallest is the one kept and why five are taken rather than two. Two would land on the floor about ninety four times in a hundred, and the six percent that missed showed up as a couple of hundred instructions of drift in a facet total between two runs of the same binaries.

The reports carry it as a difference rather than as a ratio, and that is the part worth explaining. A program in this corpus retires about 139,000 instructions. A program whose whole body is `int main(void){return 0;}` retires 107,702 on the same machine, so about four fifths of every count is the process starting up before `main`, and that is the same code whichever compiler built the program. A ratio has all of it in the denominator, so the worst case in the `loop-unroll` facet, where rucc runs 1,103 more instructions than GCC 16, comes out at 1.008 and rounds to parity. The difference has none of it, because a constant on both sides subtracts away exactly. So the `instructions` column is the signed count of instructions this compiler ran across the facet less what the reference ran, `report.json` carries it as `instruction_delta` with `counted` beside it, and the ratio is still there as `instruction_ratio` for anyone who wants to check that arithmetic.

Nothing in the corpus depends on the counter working. A machine without `perf`, a container without the permission and a virtual machine with no counters are all ordinary places to run this, and on all of them the column reads `not measured` and every verdict is exactly what it would have been. Whether the machine counted is part of the cache key, so a result cached on a machine that could not count is not handed back on one that can.

## What the compiler is known not to do

`known-failures.json` names every family rucc is currently known to get wrong, with the issue that will close it and one line saying what is missing. A run fails on any failure that file does not name, and it also fails on any line in the file whose case did not fail this time.

Both directions matter and the second one is the one that is usually left out. A corpus that fails on everything a compiler under development has not reached yet is a corpus nobody runs, so its job gets marked advisory and then stays red for weeks and catches nothing. A corpus that tolerates a list of failures and never checks the list is a corpus whose list rots. Failing in both directions is what keeps the file honest: a new failure is red because it is not in the file, and a fixed one is red because it is in the file and did not happen, and the message says which line to take out.

The key is the family rather than the case. A case id ends in a digest of the program text, so it moves whenever the generator changes what the program says, and a file keyed on ids would churn on every generator edit while quietly ceasing to cover the case it was written for. The family is the id without that digest, which is the facet, the axis point and the dialect. The level is not in the key either, because a construct a compiler cannot lower cannot be lowered at any level. What the level does change is whether a case that works at `-O0` breaks at `-O2`, and that is not a gap, it is a miscompilation, and it arrives as a `wrong` verdict on a family that is either absent from the file or present with a different verdict. Both are red.

Regenerate it after a run:

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 --toolchain rucc=../rucc/target/release/rucc \
    --reference gcc-16 --accept
```

That keeps the issue and the prose on every line that is still failing, and gives a new line an empty issue and a reason taken from the compiler's own words, which is a starting point for somebody to write over rather than an answer. A run only rewrites the part of the file it exercised. A run narrowed with `--facet` leaves the other facets alone, and a run whose command line did not name a compiler leaves that compiler's lines alone, which is why the reference job can run GCC 16 on its own without deleting everything the file says about rucc.

As of the run that created it the file has eight lines, all of them rucc, across three things: the address of a label and the indirect jump through it, `__atomic_signal_fence`, and the library `setjmp` and `longjmp`. GCC 16 compiles and runs all of them. Since rucc 0.24.2 every one of them works and the file names nothing, so any failure at all turns the job red.

## What each rucc pass did

When the compiler under test is rucc, every build asks it for a trace, and the trace says for each pass that ran what the pass did and how many times. The corpus adds those up across a run and writes `reports/firing.md`: for every level, which passes ran, which of them fired on how many cases, how many rewrites they made, what they reported as missed, and the facets they fired on most. The few facets that pin `-O2` themselves, because what they check needs the optimizer, are only counted at `-O2`, since at any other level they still run the `-O2` pipeline.

A pass that runs on every case and never fires is either a pass the corpus has no program for or a pass that is broken, and neither shows up as a wrong answer. `quiet-passes.json` names every pass that is allowed to be quiet at a level, with the issue that explains it and one line saying why. It is checked the same way as `known-failures.json`, in both directions. A run fails when a pass fired on nothing and the file does not name it, when a pass the file names fired, and when a pass the file names no longer runs at that level. Each message says which line to add or take out.

It has 30 lines now, all of them rucc at `-O1` and above. The run that created it found 35. The three for `ipa-vrp` came out once rucc made the pass say what it wrote (tamnd/rucc#3016), and the two for `rangetest` at `-O1` and `-Os` came out once it merged a chain of branches at those levels too (tamnd/rucc#3017). Six of the passes are the safety check passes, which have nothing to do while the corpus builds with `-fsafety=off`. The rest are a shape no facet generates yet or a pipeline where the pass cannot see the form it reads, and each line names the issue that will take it out.

Only a whole run is checked, since a run narrowed with `--facet` or `--limit` leaves out the very programs a pass might have fired on. A run that leaves out a tag is still checked, which is what keeps the check on in CI, whose runs leave out the programs that only an aarch64 machine can run. Regenerate the file the same way, by adding `--accept` to a run, which keeps the issue and the reason on every line that still holds.

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

Two facets are the exception in the other direction. `crc32c-armv8` and `simd-lfind-neon` are the AArch64 halves of `crc32c` and `simd-lfind`, written against `arm_acle.h` and `arm_neon.h`, so they only build on AArch64 and every case in them carries the `aarch64` tag. The x86-64 jobs leave them out with `--exclude-tag aarch64`, and a job of their own on GitHub's `ubuntu-24.04-arm` runner builds them with GCC 16 and with rucc. The x86-64 cases carry `x86-64` the same way, for a run that goes the other way round.

`lkmm` needs a POSIX system rather than a particular processor. Its cases check the promises the Linux kernel memory model takes from a compiler, and most of those cannot change what a single threaded program prints, so a case puts the location a store must not reach on a page it made read only with `mprotect`, or shares a location with a second thread through `pthread_create`. A broken promise is a fault or a hang rather than a wrong number. The threaded cases carry the `threads` tag, for a run on a system without POSIX threads.

`mitigations` is x86-64 only, like the Postgres SSE cases, and carries the `x86-64` tag. Its cases are built with the flags the kernel's speculation mitigations add, one mitigation at a time and then all of them together, and each program links the return and indirect branch thunks from a few lines of top level assembly in its own source.

`asm-goto`, `asm-local-labels` and `gas-macros` are the inline assembly the kernel writes, and are x86-64 only too, in the AT&T syntax. The one case that puts data in another section with `.pushsection .rodata` also carries the `elf` tag, since that section name means nothing to a Mach-O assembler. Every one of them writes `__asm__` rather than `asm`, because the corpus is built with `-std=c17` and in ISO C `asm` is an ordinary name, so GCC 16 reads `asm goto(...)` as a call to an undeclared function and refuses it.

`constant-p-after-inline`, `mcmodel-kernel` and `general-regs-only` are the way the kernel is built. The first pins `-O2`, since the promise it checks only holds once the optimizer has run, and the branch a wrong answer would take calls a function defined nowhere, so a wrong answer fails to link. `mcmodel-kernel` is linked without PIE, which puts the program where a sign extended 32 bit address means what it says, so its cases carry the `elf` tag as well. `general-regs-only` puts a marker in every vector register before the work and checks them all before it prints.

`objtool-shapes` is the code objtool has to follow through a kernel object: a jump table, a call that does not return, a realigned stack, a function that ends in `__builtin_unreachable` and a sibling call. It is x86-64 only and pins `-O2`. The program only proves the code runs, and objtool from the pinned kernel is what checks the object. `frame-size` has five kernel cases next to the Postgres ones, with a `shape` axis, each with four 640 byte buffers whose lifetimes never overlap. GCC puts them in the same bytes and stays under the kernel's `FRAME_WARN` of 2048, and a compiler that gives each buffer its own slot goes over it. They are plain C and run anywhere.

`null-pointer-constant`, `const-ice` and `section-attr` are the front end the kernel leans on. `null-pointer-constant` is `__is_constexpr`, `is_const` and `__builtin_choose_expr`, which all decide what is a constant by asking whether something is a null pointer constant, and the conditionals whose type that choice settles. `const-ice` is `sizeof`, `offsetof` and const objects in array sizes, enumerators, case labels and static assertions. These two are the only facets allowed `sizeof`, and a test beside them checks that every size they take is the same on every target and that none is printed. The `const-local` case pins `-O2`, because GCC only folds a const local in a static assertion once the optimizer is on, and the kernel is never built without it. `section-attr` puts functions and data in named sections and walks them from `__start_` to `__stop_`, the way initcalls and module parameters are found, so its cases carry the `elf` tag. They are plain C otherwise and run on any ELF target.

## No dependencies

The whole thing is `std` and nothing else. The JSON writer, the JSON parser, the SHA-256, the process runner with its timeouts, and the ELF, Mach-O and COFF section reader are all in here and all tested. The one exception to `std` alone is six Win32 calls in `memory.rs` for the peak working set of a process tree on Windows, declared by hand rather than through a crate.

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
