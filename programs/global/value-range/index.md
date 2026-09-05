# value-range

narrowing an integer to the range it can hold. Part of the global phase of the M4 plan.

4 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`value-range.i32.c17.62b45c41`](value-range.i32.c17.62b45c41.c) | type=i32 | c17 | `1 1 1 ...` and 2 more lines |
| [`value-range.i64.c17.3659dfd3`](value-range.i64.c17.3659dfd3.c) | type=i64 | c17 | `1 1 1 ...` and 2 more lines |
| [`value-range.u32.c17.c8bfc2a4`](value-range.u32.c17.c8bfc2a4.c) | type=u32 | c17 | `1 1 1 1` |
| [`value-range.u64.c17.a9f9c33a`](value-range.u64.c17.a9f9c33a.c) | type=u64 | c17 | `1 1 1 1` |

