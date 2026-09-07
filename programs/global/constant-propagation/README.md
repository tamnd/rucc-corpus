# constant-propagation

propagating a value constant on every reaching path. Part of the global phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`constant-propagation.i32.direct.c17.692e2d95`](constant-propagation.i32.direct.c17.692e2d95.c) | type=i32, shape=direct | c17 | `42` |
| [`constant-propagation.i32.same-on-both-arms.c17.04b1d528`](constant-propagation.i32.same-on-both-arms.c17.04b1d528.c) | type=i32, shape=same-on-both-arms | c17 | `42` |
| [`constant-propagation.i32.through-array.c17.8b73ffb9`](constant-propagation.i32.through-array.c17.8b73ffb9.c) | type=i32, shape=through-array | c17 | `42` |
| [`constant-propagation.i32.through-copy.c17.52b6e624`](constant-propagation.i32.through-copy.c17.52b6e624.c) | type=i32, shape=through-copy | c17 | `42` |
| [`constant-propagation.i64.direct.c17.2dd3491f`](constant-propagation.i64.direct.c17.2dd3491f.c) | type=i64, shape=direct | c17 | `42` |
| [`constant-propagation.i64.same-on-both-arms.c17.c991f777`](constant-propagation.i64.same-on-both-arms.c17.c991f777.c) | type=i64, shape=same-on-both-arms | c17 | `42` |
| [`constant-propagation.i64.through-array.c17.60001ed6`](constant-propagation.i64.through-array.c17.60001ed6.c) | type=i64, shape=through-array | c17 | `42` |
| [`constant-propagation.i64.through-copy.c17.e4a34895`](constant-propagation.i64.through-copy.c17.e4a34895.c) | type=i64, shape=through-copy | c17 | `42` |
| [`constant-propagation.u32.direct.c17.f29aa22b`](constant-propagation.u32.direct.c17.f29aa22b.c) | type=u32, shape=direct | c17 | `42` |
| [`constant-propagation.u32.same-on-both-arms.c17.ee53c438`](constant-propagation.u32.same-on-both-arms.c17.ee53c438.c) | type=u32, shape=same-on-both-arms | c17 | `42` |
| [`constant-propagation.u32.through-array.c17.12eb3fea`](constant-propagation.u32.through-array.c17.12eb3fea.c) | type=u32, shape=through-array | c17 | `42` |
| [`constant-propagation.u32.through-copy.c17.f56d8115`](constant-propagation.u32.through-copy.c17.f56d8115.c) | type=u32, shape=through-copy | c17 | `42` |
| [`constant-propagation.u64.direct.c17.22401aa9`](constant-propagation.u64.direct.c17.22401aa9.c) | type=u64, shape=direct | c17 | `42` |
| [`constant-propagation.u64.same-on-both-arms.c17.de6213c5`](constant-propagation.u64.same-on-both-arms.c17.de6213c5.c) | type=u64, shape=same-on-both-arms | c17 | `42` |
| [`constant-propagation.u64.through-array.c17.fcdb067f`](constant-propagation.u64.through-array.c17.fcdb067f.c) | type=u64, shape=through-array | c17 | `42` |
| [`constant-propagation.u64.through-copy.c17.8178ff62`](constant-propagation.u64.through-copy.c17.8178ff62.c) | type=u64, shape=through-copy | c17 | `42` |

