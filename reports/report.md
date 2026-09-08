# rucc corpus report

75 case results did not come out as expected. They are listed below, worst first.

The corpus holds 1525 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `690da57bab01fe09`.

That is 49,314 lines of C, 1.8 MiB, one translation unit per program, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.8.2 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 75 failures | no |
| `code-quality:rucc` | within 10 percent | 10 percent more | no |
| `compile-throughput:rucc` | no worse | 51 percent less | yes |
| `size-model:rucc` | no worse | level | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.
- `size-model:rucc`: the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 7593 | 7593 | 0 | 0 | 0 | 0 | 0 | 0 |
| `rucc` | 7593 | 7308 | 0 | 75 | 210 | 0 | 0 | 0 |

## What went wrong

### `atomics.stdatomic-flag.c17.6551714c`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `gate` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic-flag.c17.6551714c.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic-flag.c17.6551714c`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `gate` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic-flag.c17.6551714c.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic-flag.c17.6551714c`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `gate` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic-flag.c17.6551714c.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic-flag.c17.6551714c`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `gate` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic-flag.c17.6551714c.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic-flag.c17.6551714c`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `gate` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic-flag.c17.6551714c.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i32.c17.95fbc2ae`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:18: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i32.c17.95fbc2ae.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i32.c17.95fbc2ae`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:18: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i32.c17.95fbc2ae.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i32.c17.95fbc2ae`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:18: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i32.c17.95fbc2ae.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i32.c17.95fbc2ae`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:18: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i32.c17.95fbc2ae.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i32.c17.95fbc2ae`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:18: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i32.c17.95fbc2ae.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i64.c17.19020add`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:20: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i64.c17.19020add.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i64.c17.19020add`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:20: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i64.c17.19020add.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i64.c17.19020add`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:20: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i64.c17.19020add.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i64.c17.19020add`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:20: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i64.c17.19020add.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.i64.c17.19020add`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:20: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.i64.c17.19020add.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u32.c17.efd489a1`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u32.c17.efd489a1.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u32.c17.efd489a1`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u32.c17.efd489a1.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u32.c17.efd489a1`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u32.c17.efd489a1.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u32.c17.efd489a1`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u32.c17.efd489a1.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u32.c17.efd489a1`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:19: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u32.c17.efd489a1.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u64.c17.53c8ec77`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:21: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u64.c17.53c8ec77.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u64.c17.53c8ec77`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:21: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u64.c17.53c8ec77.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u64.c17.53c8ec77`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:21: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u64.c17.53c8ec77.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u64.c17.53c8ec77`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:21: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u64.c17.53c8ec77.c` and the working directory of the failing build was kept under the run directory.

### `atomics.stdatomic.u64.c17.53c8ec77`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them. This is a case about the atomic builtins at every ordering, and the header over them, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:4:1: error: `stdatomic.h` file not found [E0341]
case.c:4:1: note: searched: <builtin>, /usr/local/include, /usr/include/x86_64-linux-gnu, /usr/include
case.c:8:21: error: expected `;`, found `counter` [E0400]
```

The program is `programs/correctness/atomics/atomics.stdatomic.u64.c17.53c8ec77.c` and the working directory of the failing build was kept under the run directory.

And 50 more, which are all in `findings.sarif` and in `report.json`.

## What is not built yet

These are cases a compiler refused while saying itself that the construct is not implemented. They are valid C and they are counted, and they do not turn the build red, because a compiler that tells you what it has not written yet is doing the right thing. The count going up between two runs is a regression and `rucc-corpus diff` will say so.

### `rucc`

- error: `__atomic_exchange_n` is not implemented yet [E0686] (40 cases, first is `atomics.exchange.i16.c17.b3725d59`)
- error: `__atomic_fetch_add` is not implemented yet [E0686] (40 cases, first is `atomics.fetch-op.i16.c17.f2fddbd2`)
- error: `__atomic_load` is not implemented yet [E0686] (40 cases, first is `atomics.load-store.i16.c17.74a1721f`)
- error: `__atomic_signal_fence` is not implemented yet [E0686] (5 cases, first is `atomics.fence.c17.6b7c6e1f`)
- error: `__atomic_test_and_set` is not implemented yet [E0686] (5 cases, first is `atomics.flag.c17.c1e486c9`)
- error: cannot generate code for 'main': no rule lowers a `block_addr` producing a `ptr` [E0653] (80 cases, first is `computed-goto.address-in-variable.16.c17.4419a599`)

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`, and every number is a median over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the headline.

### `rucc`

Furthest behind:

| facet | phase | cases | code size | run time | compile time |
|---|---|---|---|---|---|
| `bit-builtins` | backend | 22 | 296 percent more | 2 percent less | 46 percent less |
| `register-alloc` | backend | 40 | 63 percent more | 2 percent more | 53 percent less |
| `scheduling` | backend | 12 | 60 percent more | 2 percent less | 52 percent less |
| `register-pressure` | backend | 48 | 55 percent more | 2 percent less | 52 percent less |
| `loop-restructure` | loops | 48 | 32 percent more | 2 percent less | 54 percent less |
| `float-conversion` | backend | 66 | 30 percent more | 1 percent more | 52 percent less |
| `loop-deletion` | loops | 24 | 29 percent more | 5 percent less | 51 percent less |
| `calling-convention` | backend | 10 | 27 percent more | 1 percent more | 50 percent less |

Furthest ahead:

| facet | phase | cases | code size | run time | compile time |
|---|---|---|---|---|---|
| `copy-propagation` | global | 12 | 6 percent less | 8 percent less | 48 percent less |
| `unreachable-code` | local | 8 | 5 percent less | level | 49 percent less |
| `reachability` | interprocedural | 9 | 5 percent less | 15 percent more | 50 percent less |
| `dead-code` | local | 12 | 5 percent less | 2 percent more | 48 percent less |
| `constant-propagation` | global | 16 | 4 percent less | 3 percent more | 50 percent less |
| `selection` | backend | 40 | 2 percent less | 2 percent less | 49 percent less |
| `machine-peephole` | backend | 22 | 2 percent less | 2 percent less | 51 percent less |
| `scalar-replacement` | global | 20 | 1 percent less | 4 percent less | 51 percent less |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 1517 | level | 722 |
| `rucc` | 1460 | level | 1446 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

### `rucc` at `-Os`

Where `-Os` saves the most:

| facet | cases | code size at `-Os` |
|---|---|---|
| `baseline` | 10 | level |
| `control-flow` | 9 | level |
| `branch-probability` | 34 | level |
| `constant-fold` | 112 | level |
| `strength` | 24 | level |
| `narrowing` | 52 | level |

Where it saves the least, which is where it is spending size and getting nothing for it when the number is above level:

| facet | cases | code size at `-Os` |
|---|---|---|
| `calling-convention` | 10 | level |
| `machine-peephole` | 22 | level |
| `bit-builtins` | 22 | level |
| `float-conversion` | 66 | level |
| `barrier` | 7 | level |
| `frontend` | 22 | level |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines | `rucc` passed | `rucc` code size |
|---|---|---|---|---|---|
| floor | 5 | 109 | 3,222 | 383 of 513 | 9 percent more |
| local | 8 | 309 | 16,979 | 1545 of 1545 | 10 percent more |
| global | 9 | 193 | 4,374 | 965 of 965 | 4 percent more |
| loops | 9 | 466 | 9,243 | 2330 of 2330 | 14 percent more |
| interprocedural | 6 | 117 | 3,087 | 585 of 585 | 11 percent more |
| backend | 11 | 293 | 10,951 | 1465 of 1465 | 27 percent more |
| correctness | 2 | 38 | 1,458 | 35 of 190 | 3 percent more |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `atomics` | correctness | 0 | 4026 |
| `bit-builtins` | backend | 0 | 3352 |
| `inline` | interprocedural | 900 | 372 |
| `float-conversion` | backend | 0 | 850 |
| `simplify` | local | 0 | 806 |
| `tail-call` | interprocedural | 300 | 480 |
| `computed-goto` | floor | 32 | 600 |
| `narrowing` | local | 0 | 528 |
| `loop-idiom` | loops | 395 | 76 |
| `loop-shape` | loops | 112 | 304 |
| `loop-unswitch` | loops | 134 | 212 |
| `loop-unroll` | loops | 196 | 128 |
| `loop-restructure` | loops | 262 | 60 |
| `strength` | local | 0 | 288 |
| `register-pressure` | backend | 75 | 210 |
| `load-forwarding` | global | 60 | 184 |
| `selection` | backend | 0 | 244 |
| `loop-rotate` | loops | 224 | 0 |
| `alias-analysis` | global | 66 | 136 |
| `loop-invariant` | loops | 60 | 128 |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1525 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
