# machine-peephole

the rules that only make sense on machine instructions. Part of the backend phase of the M4 plan.

22 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`machine-peephole.i32.compare-after-subtract.c17.be14ce45`](machine-peephole.i32.compare-after-subtract.c17.be14ce45.c) | type=i32, shape=compare-after-subtract | c17 | `26` |
| [`machine-peephole.i32.double-negate.c17.ca087384`](machine-peephole.i32.double-negate.c17.ca087384.c) | type=i32, shape=double-negate | c17 | `25` |
| [`machine-peephole.i32.load-then-mask.c17.987c2fa3`](machine-peephole.i32.load-then-mask.c17.987c2fa3.c) | type=i32, shape=load-then-mask | c17 | `40` |
| [`machine-peephole.i32.redundant-move.c17.e57333e7`](machine-peephole.i32.redundant-move.c17.e57333e7.c) | type=i32, shape=redundant-move | c17 | `25` |
| [`machine-peephole.i32.shift-pair.c17.4b129f95`](machine-peephole.i32.shift-pair.c17.4b129f95.c) | type=i32, shape=shift-pair | c17 | `40` |
| [`machine-peephole.i32.test-against-zero.c17.df64fe7d`](machine-peephole.i32.test-against-zero.c17.df64fe7d.c) | type=i32, shape=test-against-zero | c17 | `1` |
| [`machine-peephole.i64.compare-after-subtract.c17.a20aaccb`](machine-peephole.i64.compare-after-subtract.c17.a20aaccb.c) | type=i64, shape=compare-after-subtract | c17 | `26` |
| [`machine-peephole.i64.double-negate.c17.dcd49368`](machine-peephole.i64.double-negate.c17.dcd49368.c) | type=i64, shape=double-negate | c17 | `25` |
| [`machine-peephole.i64.load-then-mask.c17.d79b2832`](machine-peephole.i64.load-then-mask.c17.d79b2832.c) | type=i64, shape=load-then-mask | c17 | `40` |
| [`machine-peephole.i64.redundant-move.c17.fad36a63`](machine-peephole.i64.redundant-move.c17.fad36a63.c) | type=i64, shape=redundant-move | c17 | `25` |
| [`machine-peephole.i64.shift-pair.c17.26b0bf8d`](machine-peephole.i64.shift-pair.c17.26b0bf8d.c) | type=i64, shape=shift-pair | c17 | `40` |
| [`machine-peephole.i64.test-against-zero.c17.095b9f17`](machine-peephole.i64.test-against-zero.c17.095b9f17.c) | type=i64, shape=test-against-zero | c17 | `1` |
| [`machine-peephole.u32.compare-after-subtract.c17.6c3762c5`](machine-peephole.u32.compare-after-subtract.c17.6c3762c5.c) | type=u32, shape=compare-after-subtract | c17 | `26` |
| [`machine-peephole.u32.load-then-mask.c17.c1870775`](machine-peephole.u32.load-then-mask.c17.c1870775.c) | type=u32, shape=load-then-mask | c17 | `40` |
| [`machine-peephole.u32.redundant-move.c17.a3415b25`](machine-peephole.u32.redundant-move.c17.a3415b25.c) | type=u32, shape=redundant-move | c17 | `25` |
| [`machine-peephole.u32.shift-pair.c17.ecbe7d91`](machine-peephole.u32.shift-pair.c17.ecbe7d91.c) | type=u32, shape=shift-pair | c17 | `40` |
| [`machine-peephole.u32.test-against-zero.c17.033e7071`](machine-peephole.u32.test-against-zero.c17.033e7071.c) | type=u32, shape=test-against-zero | c17 | `1` |
| [`machine-peephole.u64.compare-after-subtract.c17.d305bfc5`](machine-peephole.u64.compare-after-subtract.c17.d305bfc5.c) | type=u64, shape=compare-after-subtract | c17 | `26` |
| [`machine-peephole.u64.load-then-mask.c17.318059c8`](machine-peephole.u64.load-then-mask.c17.318059c8.c) | type=u64, shape=load-then-mask | c17 | `40` |
| [`machine-peephole.u64.redundant-move.c17.802a6cf0`](machine-peephole.u64.redundant-move.c17.802a6cf0.c) | type=u64, shape=redundant-move | c17 | `25` |
| [`machine-peephole.u64.shift-pair.c17.c6f5287c`](machine-peephole.u64.shift-pair.c17.c6f5287c.c) | type=u64, shape=shift-pair | c17 | `40` |
| [`machine-peephole.u64.test-against-zero.c17.d77b123e`](machine-peephole.u64.test-against-zero.c17.d77b123e.c) | type=u64, shape=test-against-zero | c17 | `1` |

