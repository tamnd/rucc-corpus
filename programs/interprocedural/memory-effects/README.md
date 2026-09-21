# memory-effects

what a callee reads and writes, seen from its call sites. Part of the interprocedural phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`memory-effects.i32.keeps-a-local.c17.0571b041`](memory-effects.i32.keeps-a-local.c17.0571b041.c) | type=i32, shape=keeps-a-local | c17 | `1027000 1006 8` |
| [`memory-effects.i32.lends-a-local.c17.aa08ad0f`](memory-effects.i32.lends-a-local.c17.aa08ad0f.c) | type=i32, shape=lends-a-local | c17 | `27000 6 1000` |
| [`memory-effects.i32.reads-only.c17.23608b80`](memory-effects.i32.reads-only.c17.23608b80.c) | type=i32, shape=reads-only | c17 | `30000 6 8` |
| [`memory-effects.i32.writes-both-arrays.c17.49d6e581`](memory-effects.i32.writes-both-arrays.c17.49d6e581.c) | type=i32, shape=writes-both-arrays | c17 | `517500 7006 1008` |
| [`memory-effects.i32.writes-one-array.c17.60b66c90`](memory-effects.i32.writes-one-array.c17.60b66c90.c) | type=i32, shape=writes-one-array | c17 | `17000 7006 1000` |
| [`memory-effects.i32.writes-what-is-read.c17.17fb4385`](memory-effects.i32.writes-what-is-read.c17.17fb4385.c) | type=i32, shape=writes-what-is-read | c17 | `530500 6 1008` |
| [`memory-effects.i64.keeps-a-local.c17.cc9386a3`](memory-effects.i64.keeps-a-local.c17.cc9386a3.c) | type=i64, shape=keeps-a-local | c17 | `1027000 1006 8` |
| [`memory-effects.i64.lends-a-local.c17.bd9d48e1`](memory-effects.i64.lends-a-local.c17.bd9d48e1.c) | type=i64, shape=lends-a-local | c17 | `27000 6 1000` |
| [`memory-effects.i64.reads-only.c17.54ee7a18`](memory-effects.i64.reads-only.c17.54ee7a18.c) | type=i64, shape=reads-only | c17 | `30000 6 8` |
| [`memory-effects.i64.writes-both-arrays.c17.971b39c6`](memory-effects.i64.writes-both-arrays.c17.971b39c6.c) | type=i64, shape=writes-both-arrays | c17 | `517500 7006 1008` |
| [`memory-effects.i64.writes-one-array.c17.b31c7b80`](memory-effects.i64.writes-one-array.c17.b31c7b80.c) | type=i64, shape=writes-one-array | c17 | `17000 7006 1000` |
| [`memory-effects.i64.writes-what-is-read.c17.59afcd24`](memory-effects.i64.writes-what-is-read.c17.59afcd24.c) | type=i64, shape=writes-what-is-read | c17 | `530500 6 1008` |
| [`memory-effects.u32.keeps-a-local.c17.4ac3ed88`](memory-effects.u32.keeps-a-local.c17.4ac3ed88.c) | type=u32, shape=keeps-a-local | c17 | `1027000 1006 8` |
| [`memory-effects.u32.lends-a-local.c17.7b7f7ea2`](memory-effects.u32.lends-a-local.c17.7b7f7ea2.c) | type=u32, shape=lends-a-local | c17 | `27000 6 1000` |
| [`memory-effects.u32.reads-only.c17.d586027c`](memory-effects.u32.reads-only.c17.d586027c.c) | type=u32, shape=reads-only | c17 | `30000 6 8` |
| [`memory-effects.u32.writes-both-arrays.c17.dada1e8a`](memory-effects.u32.writes-both-arrays.c17.dada1e8a.c) | type=u32, shape=writes-both-arrays | c17 | `517500 7006 1008` |
| [`memory-effects.u32.writes-one-array.c17.ca03303f`](memory-effects.u32.writes-one-array.c17.ca03303f.c) | type=u32, shape=writes-one-array | c17 | `17000 7006 1000` |
| [`memory-effects.u32.writes-what-is-read.c17.96a35b1c`](memory-effects.u32.writes-what-is-read.c17.96a35b1c.c) | type=u32, shape=writes-what-is-read | c17 | `530500 6 1008` |
| [`memory-effects.u64.keeps-a-local.c17.28ae6fbf`](memory-effects.u64.keeps-a-local.c17.28ae6fbf.c) | type=u64, shape=keeps-a-local | c17 | `1027000 1006 8` |
| [`memory-effects.u64.lends-a-local.c17.9903be70`](memory-effects.u64.lends-a-local.c17.9903be70.c) | type=u64, shape=lends-a-local | c17 | `27000 6 1000` |
| [`memory-effects.u64.reads-only.c17.5236752c`](memory-effects.u64.reads-only.c17.5236752c.c) | type=u64, shape=reads-only | c17 | `30000 6 8` |
| [`memory-effects.u64.writes-both-arrays.c17.1bbd8db8`](memory-effects.u64.writes-both-arrays.c17.1bbd8db8.c) | type=u64, shape=writes-both-arrays | c17 | `517500 7006 1008` |
| [`memory-effects.u64.writes-one-array.c17.4c45d294`](memory-effects.u64.writes-one-array.c17.4c45d294.c) | type=u64, shape=writes-one-array | c17 | `17000 7006 1000` |
| [`memory-effects.u64.writes-what-is-read.c17.1e691520`](memory-effects.u64.writes-what-is-read.c17.1e691520.c) | type=u64, shape=writes-what-is-read | c17 | `530500 6 1008` |

