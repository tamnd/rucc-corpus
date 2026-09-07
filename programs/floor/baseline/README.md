# baseline

programs with no optimization target, which every level must get right. Part of the floor phase of the M4 plan.

10 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`baseline.arithmetic.c17.f5bb5b8a`](baseline.arithmetic.c17.f5bb5b8a.c) | program=arithmetic | c17 | `42 32 185 ...` and 2 more lines |
| [`baseline.arrays.c17.b15a82bb`](baseline.arrays.c17.b15a82bb.c) | program=arrays | c17 | `0 49 140` |
| [`baseline.branches.c17.d7ab3943`](baseline.branches.c17.d7ab3943.c) | program=branches | c17 | `2 1 0 ...` and 2 more lines |
| [`baseline.locals.c17.8667b687`](baseline.locals.c17.8667b687.c) | program=locals | c17 | `30 6 36` |
| [`baseline.loops.c17.20315a7c`](baseline.loops.c17.20315a7c.c) | program=loops | c17 | `55 0 1` |
| [`baseline.pointers.c17.ba6f44ef`](baseline.pointers.c17.ba6f44ef.c) | program=pointers | c17 | `9 27 2` |
| [`baseline.recursion.c17.1239ed5e`](baseline.recursion.c17.1239ed5e.c) | program=recursion | c17 | `3628800 55` |
| [`baseline.sort.c17.9e5d80ce`](baseline.sort.c17.9e5d80ce.c) | program=sort | c17 | `0 1 2 ...` and 5 more lines |
| [`baseline.strings.c17.3e788c5b`](baseline.strings.c17.3e788c5b.c) | program=strings | c17 | `5 114 33` |
| [`baseline.structs.c17.6ce43b43`](baseline.structs.c17.6ce43b43.c) | program=structs | c17 | `6 14` |

