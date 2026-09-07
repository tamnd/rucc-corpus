# loop-rotate

rotating a loop so the test lands at the bottom. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-rotate.i32.1.do-while.c17.ad1cf337`](loop-rotate.i32.1.do-while.c17.ad1cf337.c) | type=i32, trips=1, shape=do-while | c17 | `1` |
| [`loop-rotate.i32.1.for.c17.2fd27a50`](loop-rotate.i32.1.for.c17.2fd27a50.c) | type=i32, trips=1, shape=for | c17 | `1` |
| [`loop-rotate.i32.1.goto.c17.3d61431c`](loop-rotate.i32.1.goto.c17.3d61431c.c) | type=i32, trips=1, shape=goto | c17 | `1` |
| [`loop-rotate.i32.1.while.c17.78bcf441`](loop-rotate.i32.1.while.c17.78bcf441.c) | type=i32, trips=1, shape=while | c17 | `1` |
| [`loop-rotate.i32.16.do-while.c17.13fffd00`](loop-rotate.i32.16.do-while.c17.13fffd00.c) | type=i32, trips=16, shape=do-while | c17 | `136` |
| [`loop-rotate.i32.16.for.c17.063fd33e`](loop-rotate.i32.16.for.c17.063fd33e.c) | type=i32, trips=16, shape=for | c17 | `136` |
| [`loop-rotate.i32.16.goto.c17.f279953a`](loop-rotate.i32.16.goto.c17.f279953a.c) | type=i32, trips=16, shape=goto | c17 | `136` |
| [`loop-rotate.i32.16.while.c17.b3760170`](loop-rotate.i32.16.while.c17.b3760170.c) | type=i32, trips=16, shape=while | c17 | `136` |
| [`loop-rotate.i32.2.do-while.c17.4be34935`](loop-rotate.i32.2.do-while.c17.4be34935.c) | type=i32, trips=2, shape=do-while | c17 | `3` |
| [`loop-rotate.i32.2.for.c17.49a2b6c2`](loop-rotate.i32.2.for.c17.49a2b6c2.c) | type=i32, trips=2, shape=for | c17 | `3` |
| [`loop-rotate.i32.2.goto.c17.cacf6a68`](loop-rotate.i32.2.goto.c17.cacf6a68.c) | type=i32, trips=2, shape=goto | c17 | `3` |
| [`loop-rotate.i32.2.while.c17.787b895d`](loop-rotate.i32.2.while.c17.787b895d.c) | type=i32, trips=2, shape=while | c17 | `3` |
| [`loop-rotate.i32.7.do-while.c17.6583c31b`](loop-rotate.i32.7.do-while.c17.6583c31b.c) | type=i32, trips=7, shape=do-while | c17 | `28` |
| [`loop-rotate.i32.7.for.c17.5d9553e8`](loop-rotate.i32.7.for.c17.5d9553e8.c) | type=i32, trips=7, shape=for | c17 | `28` |
| [`loop-rotate.i32.7.goto.c17.ca804951`](loop-rotate.i32.7.goto.c17.ca804951.c) | type=i32, trips=7, shape=goto | c17 | `28` |
| [`loop-rotate.i32.7.while.c17.6dc27bd6`](loop-rotate.i32.7.while.c17.6dc27bd6.c) | type=i32, trips=7, shape=while | c17 | `28` |
| [`loop-rotate.i64.1.do-while.c17.a6149d99`](loop-rotate.i64.1.do-while.c17.a6149d99.c) | type=i64, trips=1, shape=do-while | c17 | `1` |
| [`loop-rotate.i64.1.for.c17.c9ac4ffd`](loop-rotate.i64.1.for.c17.c9ac4ffd.c) | type=i64, trips=1, shape=for | c17 | `1` |
| [`loop-rotate.i64.1.goto.c17.f5fc06bb`](loop-rotate.i64.1.goto.c17.f5fc06bb.c) | type=i64, trips=1, shape=goto | c17 | `1` |
| [`loop-rotate.i64.1.while.c17.f754bc31`](loop-rotate.i64.1.while.c17.f754bc31.c) | type=i64, trips=1, shape=while | c17 | `1` |
| [`loop-rotate.i64.16.do-while.c17.c5a270d8`](loop-rotate.i64.16.do-while.c17.c5a270d8.c) | type=i64, trips=16, shape=do-while | c17 | `136` |
| [`loop-rotate.i64.16.for.c17.ed4ee6e3`](loop-rotate.i64.16.for.c17.ed4ee6e3.c) | type=i64, trips=16, shape=for | c17 | `136` |
| [`loop-rotate.i64.16.goto.c17.49edc607`](loop-rotate.i64.16.goto.c17.49edc607.c) | type=i64, trips=16, shape=goto | c17 | `136` |
| [`loop-rotate.i64.16.while.c17.85a9a509`](loop-rotate.i64.16.while.c17.85a9a509.c) | type=i64, trips=16, shape=while | c17 | `136` |
| [`loop-rotate.i64.2.do-while.c17.7295802e`](loop-rotate.i64.2.do-while.c17.7295802e.c) | type=i64, trips=2, shape=do-while | c17 | `3` |
| [`loop-rotate.i64.2.for.c17.11cfb25d`](loop-rotate.i64.2.for.c17.11cfb25d.c) | type=i64, trips=2, shape=for | c17 | `3` |
| [`loop-rotate.i64.2.goto.c17.7f4e6b05`](loop-rotate.i64.2.goto.c17.7f4e6b05.c) | type=i64, trips=2, shape=goto | c17 | `3` |
| [`loop-rotate.i64.2.while.c17.25cbc990`](loop-rotate.i64.2.while.c17.25cbc990.c) | type=i64, trips=2, shape=while | c17 | `3` |
| [`loop-rotate.i64.7.do-while.c17.a1ea7c09`](loop-rotate.i64.7.do-while.c17.a1ea7c09.c) | type=i64, trips=7, shape=do-while | c17 | `28` |
| [`loop-rotate.i64.7.for.c17.ebe515ec`](loop-rotate.i64.7.for.c17.ebe515ec.c) | type=i64, trips=7, shape=for | c17 | `28` |
| [`loop-rotate.i64.7.goto.c17.018d206d`](loop-rotate.i64.7.goto.c17.018d206d.c) | type=i64, trips=7, shape=goto | c17 | `28` |
| [`loop-rotate.i64.7.while.c17.91809517`](loop-rotate.i64.7.while.c17.91809517.c) | type=i64, trips=7, shape=while | c17 | `28` |
| [`loop-rotate.u32.1.do-while.c17.1939660c`](loop-rotate.u32.1.do-while.c17.1939660c.c) | type=u32, trips=1, shape=do-while | c17 | `1` |
| [`loop-rotate.u32.1.for.c17.49c0d525`](loop-rotate.u32.1.for.c17.49c0d525.c) | type=u32, trips=1, shape=for | c17 | `1` |
| [`loop-rotate.u32.1.goto.c17.44b99ab2`](loop-rotate.u32.1.goto.c17.44b99ab2.c) | type=u32, trips=1, shape=goto | c17 | `1` |
| [`loop-rotate.u32.1.while.c17.9eb2f2ba`](loop-rotate.u32.1.while.c17.9eb2f2ba.c) | type=u32, trips=1, shape=while | c17 | `1` |
| [`loop-rotate.u32.16.do-while.c17.64dc276f`](loop-rotate.u32.16.do-while.c17.64dc276f.c) | type=u32, trips=16, shape=do-while | c17 | `136` |
| [`loop-rotate.u32.16.for.c17.2f2dd1d6`](loop-rotate.u32.16.for.c17.2f2dd1d6.c) | type=u32, trips=16, shape=for | c17 | `136` |
| [`loop-rotate.u32.16.goto.c17.76b63870`](loop-rotate.u32.16.goto.c17.76b63870.c) | type=u32, trips=16, shape=goto | c17 | `136` |
| [`loop-rotate.u32.16.while.c17.610c3e80`](loop-rotate.u32.16.while.c17.610c3e80.c) | type=u32, trips=16, shape=while | c17 | `136` |
| [`loop-rotate.u32.2.do-while.c17.b9206ead`](loop-rotate.u32.2.do-while.c17.b9206ead.c) | type=u32, trips=2, shape=do-while | c17 | `3` |
| [`loop-rotate.u32.2.for.c17.faadeb20`](loop-rotate.u32.2.for.c17.faadeb20.c) | type=u32, trips=2, shape=for | c17 | `3` |
| [`loop-rotate.u32.2.goto.c17.235d51fa`](loop-rotate.u32.2.goto.c17.235d51fa.c) | type=u32, trips=2, shape=goto | c17 | `3` |
| [`loop-rotate.u32.2.while.c17.cbdbe2fc`](loop-rotate.u32.2.while.c17.cbdbe2fc.c) | type=u32, trips=2, shape=while | c17 | `3` |
| [`loop-rotate.u32.7.do-while.c17.959e7281`](loop-rotate.u32.7.do-while.c17.959e7281.c) | type=u32, trips=7, shape=do-while | c17 | `28` |
| [`loop-rotate.u32.7.for.c17.b50662a7`](loop-rotate.u32.7.for.c17.b50662a7.c) | type=u32, trips=7, shape=for | c17 | `28` |
| [`loop-rotate.u32.7.goto.c17.4f92abec`](loop-rotate.u32.7.goto.c17.4f92abec.c) | type=u32, trips=7, shape=goto | c17 | `28` |
| [`loop-rotate.u32.7.while.c17.a9192bbb`](loop-rotate.u32.7.while.c17.a9192bbb.c) | type=u32, trips=7, shape=while | c17 | `28` |
| [`loop-rotate.u64.1.do-while.c17.1cb53f1d`](loop-rotate.u64.1.do-while.c17.1cb53f1d.c) | type=u64, trips=1, shape=do-while | c17 | `1` |
| [`loop-rotate.u64.1.for.c17.84b3013f`](loop-rotate.u64.1.for.c17.84b3013f.c) | type=u64, trips=1, shape=for | c17 | `1` |
| [`loop-rotate.u64.1.goto.c17.6ae8b1e0`](loop-rotate.u64.1.goto.c17.6ae8b1e0.c) | type=u64, trips=1, shape=goto | c17 | `1` |
| [`loop-rotate.u64.1.while.c17.b35bb7e9`](loop-rotate.u64.1.while.c17.b35bb7e9.c) | type=u64, trips=1, shape=while | c17 | `1` |
| [`loop-rotate.u64.16.do-while.c17.42d3ee75`](loop-rotate.u64.16.do-while.c17.42d3ee75.c) | type=u64, trips=16, shape=do-while | c17 | `136` |
| [`loop-rotate.u64.16.for.c17.6df2ad96`](loop-rotate.u64.16.for.c17.6df2ad96.c) | type=u64, trips=16, shape=for | c17 | `136` |
| [`loop-rotate.u64.16.goto.c17.c4609863`](loop-rotate.u64.16.goto.c17.c4609863.c) | type=u64, trips=16, shape=goto | c17 | `136` |
| [`loop-rotate.u64.16.while.c17.4bc8f2bc`](loop-rotate.u64.16.while.c17.4bc8f2bc.c) | type=u64, trips=16, shape=while | c17 | `136` |
| [`loop-rotate.u64.2.do-while.c17.634033e9`](loop-rotate.u64.2.do-while.c17.634033e9.c) | type=u64, trips=2, shape=do-while | c17 | `3` |
| [`loop-rotate.u64.2.for.c17.1fa5b247`](loop-rotate.u64.2.for.c17.1fa5b247.c) | type=u64, trips=2, shape=for | c17 | `3` |
| [`loop-rotate.u64.2.goto.c17.d0299782`](loop-rotate.u64.2.goto.c17.d0299782.c) | type=u64, trips=2, shape=goto | c17 | `3` |
| [`loop-rotate.u64.2.while.c17.d78fa15f`](loop-rotate.u64.2.while.c17.d78fa15f.c) | type=u64, trips=2, shape=while | c17 | `3` |
| [`loop-rotate.u64.7.do-while.c17.01397751`](loop-rotate.u64.7.do-while.c17.01397751.c) | type=u64, trips=7, shape=do-while | c17 | `28` |
| [`loop-rotate.u64.7.for.c17.11ab6c9a`](loop-rotate.u64.7.for.c17.11ab6c9a.c) | type=u64, trips=7, shape=for | c17 | `28` |
| [`loop-rotate.u64.7.goto.c17.7682923f`](loop-rotate.u64.7.goto.c17.7682923f.c) | type=u64, trips=7, shape=goto | c17 | `28` |
| [`loop-rotate.u64.7.while.c17.77973010`](loop-rotate.u64.7.while.c17.77973010.c) | type=u64, trips=7, shape=while | c17 | `28` |

