# dead-code

removing a computation whose result nothing reads. Part of the local phase of the M4 plan.

12 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`dead-code.i32.1.c17.0761ce7f`](dead-code.i32.1.c17.0761ce7f.c) | type=i32, depth=1 | c17 | `4` |
| [`dead-code.i32.16.c17.702d0e50`](dead-code.i32.16.c17.702d0e50.c) | type=i32, depth=16 | c17 | `4` |
| [`dead-code.i32.4.c17.55d70279`](dead-code.i32.4.c17.55d70279.c) | type=i32, depth=4 | c17 | `4` |
| [`dead-code.i64.1.c17.888a3032`](dead-code.i64.1.c17.888a3032.c) | type=i64, depth=1 | c17 | `4` |
| [`dead-code.i64.16.c17.f4839860`](dead-code.i64.16.c17.f4839860.c) | type=i64, depth=16 | c17 | `4` |
| [`dead-code.i64.4.c17.505654b2`](dead-code.i64.4.c17.505654b2.c) | type=i64, depth=4 | c17 | `4` |
| [`dead-code.u32.1.c17.d89da94c`](dead-code.u32.1.c17.d89da94c.c) | type=u32, depth=1 | c17 | `4` |
| [`dead-code.u32.16.c17.ff743718`](dead-code.u32.16.c17.ff743718.c) | type=u32, depth=16 | c17 | `4` |
| [`dead-code.u32.4.c17.8cd6a73f`](dead-code.u32.4.c17.8cd6a73f.c) | type=u32, depth=4 | c17 | `4` |
| [`dead-code.u64.1.c17.64b81074`](dead-code.u64.1.c17.64b81074.c) | type=u64, depth=1 | c17 | `4` |
| [`dead-code.u64.16.c17.686dca36`](dead-code.u64.16.c17.686dca36.c) | type=u64, depth=16 | c17 | `4` |
| [`dead-code.u64.4.c17.6445e743`](dead-code.u64.4.c17.6445e743.c) | type=u64, depth=4 | c17 | `4` |

