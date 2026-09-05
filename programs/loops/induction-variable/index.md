# induction-variable

rewriting induction variables into a cheaper set. Part of the loops phase of the M4 plan.

32 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`induction-variable.i32.1.c17.e53c9d8e`](induction-variable.i32.1.c17.e53c9d8e.c) | type=i32, trips=1 | c17 | `0 0 1` |
| [`induction-variable.i32.100.c17.cc8b9216`](induction-variable.i32.100.c17.cc8b9216.c) | type=i32, trips=100 | c17 | `19800 4950 100` |
| [`induction-variable.i32.16.c17.695799a0`](induction-variable.i32.16.c17.695799a0.c) | type=i32, trips=16 | c17 | `480 120 16` |
| [`induction-variable.i32.2.c17.ceafdf83`](induction-variable.i32.2.c17.ceafdf83.c) | type=i32, trips=2 | c17 | `4 1 2` |
| [`induction-variable.i32.3.c17.bd525d2a`](induction-variable.i32.3.c17.bd525d2a.c) | type=i32, trips=3 | c17 | `12 3 3` |
| [`induction-variable.i32.4.c17.ac2f8aa1`](induction-variable.i32.4.c17.ac2f8aa1.c) | type=i32, trips=4 | c17 | `24 6 4` |
| [`induction-variable.i32.7.c17.e42ada4b`](induction-variable.i32.7.c17.e42ada4b.c) | type=i32, trips=7 | c17 | `84 21 7` |
| [`induction-variable.i32.8.c17.bf63f564`](induction-variable.i32.8.c17.bf63f564.c) | type=i32, trips=8 | c17 | `112 28 8` |
| [`induction-variable.i64.1.c17.9e04d9b8`](induction-variable.i64.1.c17.9e04d9b8.c) | type=i64, trips=1 | c17 | `0 0 1` |
| [`induction-variable.i64.100.c17.df80e156`](induction-variable.i64.100.c17.df80e156.c) | type=i64, trips=100 | c17 | `19800 4950 100` |
| [`induction-variable.i64.16.c17.dc75190a`](induction-variable.i64.16.c17.dc75190a.c) | type=i64, trips=16 | c17 | `480 120 16` |
| [`induction-variable.i64.2.c17.0f38c525`](induction-variable.i64.2.c17.0f38c525.c) | type=i64, trips=2 | c17 | `4 1 2` |
| [`induction-variable.i64.3.c17.71e7f3f7`](induction-variable.i64.3.c17.71e7f3f7.c) | type=i64, trips=3 | c17 | `12 3 3` |
| [`induction-variable.i64.4.c17.72bd990b`](induction-variable.i64.4.c17.72bd990b.c) | type=i64, trips=4 | c17 | `24 6 4` |
| [`induction-variable.i64.7.c17.3e9f7e85`](induction-variable.i64.7.c17.3e9f7e85.c) | type=i64, trips=7 | c17 | `84 21 7` |
| [`induction-variable.i64.8.c17.2dac0580`](induction-variable.i64.8.c17.2dac0580.c) | type=i64, trips=8 | c17 | `112 28 8` |
| [`induction-variable.u32.1.c17.97aa82f0`](induction-variable.u32.1.c17.97aa82f0.c) | type=u32, trips=1 | c17 | `0 0 1` |
| [`induction-variable.u32.100.c17.833e1a24`](induction-variable.u32.100.c17.833e1a24.c) | type=u32, trips=100 | c17 | `19800 4950 100` |
| [`induction-variable.u32.16.c17.157bc3d7`](induction-variable.u32.16.c17.157bc3d7.c) | type=u32, trips=16 | c17 | `480 120 16` |
| [`induction-variable.u32.2.c17.a7847c0e`](induction-variable.u32.2.c17.a7847c0e.c) | type=u32, trips=2 | c17 | `4 1 2` |
| [`induction-variable.u32.3.c17.8d2bc103`](induction-variable.u32.3.c17.8d2bc103.c) | type=u32, trips=3 | c17 | `12 3 3` |
| [`induction-variable.u32.4.c17.5c571a3e`](induction-variable.u32.4.c17.5c571a3e.c) | type=u32, trips=4 | c17 | `24 6 4` |
| [`induction-variable.u32.7.c17.1f5b4141`](induction-variable.u32.7.c17.1f5b4141.c) | type=u32, trips=7 | c17 | `84 21 7` |
| [`induction-variable.u32.8.c17.05844359`](induction-variable.u32.8.c17.05844359.c) | type=u32, trips=8 | c17 | `112 28 8` |
| [`induction-variable.u64.1.c17.774c9f14`](induction-variable.u64.1.c17.774c9f14.c) | type=u64, trips=1 | c17 | `0 0 1` |
| [`induction-variable.u64.100.c17.853cde8c`](induction-variable.u64.100.c17.853cde8c.c) | type=u64, trips=100 | c17 | `19800 4950 100` |
| [`induction-variable.u64.16.c17.093f5188`](induction-variable.u64.16.c17.093f5188.c) | type=u64, trips=16 | c17 | `480 120 16` |
| [`induction-variable.u64.2.c17.51d87b8b`](induction-variable.u64.2.c17.51d87b8b.c) | type=u64, trips=2 | c17 | `4 1 2` |
| [`induction-variable.u64.3.c17.48add17e`](induction-variable.u64.3.c17.48add17e.c) | type=u64, trips=3 | c17 | `12 3 3` |
| [`induction-variable.u64.4.c17.6a5b4eb6`](induction-variable.u64.4.c17.6a5b4eb6.c) | type=u64, trips=4 | c17 | `24 6 4` |
| [`induction-variable.u64.7.c17.770c9de3`](induction-variable.u64.7.c17.770c9de3.c) | type=u64, trips=7 | c17 | `84 21 7` |
| [`induction-variable.u64.8.c17.12eab8b5`](induction-variable.u64.8.c17.12eab8b5.c) | type=u64, trips=8 | c17 | `112 28 8` |

