# address-fold

whether an address is worked out once or carried by each reader. Part of the backend phase of the M4 plan.

28 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`address-fold.i32.base-written-between.c17.76f85ef2`](address-fold.i32.base-written-between.c17.76f85ef2.c) | type=i32, shape=base-written-between | c17 | `11` |
| [`address-fold.i32.computed-index.c17.0c1e3514`](address-fold.i32.computed-index.c17.0c1e3514.c) | type=i32, shape=computed-index | c17 | `11` |
| [`address-fold.i32.filled-then-read.c17.527eac26`](address-fold.i32.filled-then-read.c17.527eac26.c) | type=i32, shape=filled-then-read | c17 | `68` |
| [`address-fold.i32.global-many-readers.c17.032196ae`](address-fold.i32.global-many-readers.c17.032196ae.c) | type=i32, shape=global-many-readers | c17 | `26` |
| [`address-fold.i32.reader-in-another-block.c17.089d2963`](address-fold.i32.reader-in-another-block.c17.089d2963.c) | type=i32, shape=reader-in-another-block | c17 | `11` |
| [`address-fold.i32.several-offsets.c17.bd125f42`](address-fold.i32.several-offsets.c17.bd125f42.c) | type=i32, shape=several-offsets | c17 | `18` |
| [`address-fold.i32.store-then-load.c17.d8f132e6`](address-fold.i32.store-then-load.c17.d8f132e6.c) | type=i32, shape=store-then-load | c17 | `6` |
| [`address-fold.i64.base-written-between.c17.37ec7c9e`](address-fold.i64.base-written-between.c17.37ec7c9e.c) | type=i64, shape=base-written-between | c17 | `11` |
| [`address-fold.i64.computed-index.c17.d1c5ce74`](address-fold.i64.computed-index.c17.d1c5ce74.c) | type=i64, shape=computed-index | c17 | `11` |
| [`address-fold.i64.filled-then-read.c17.eb1e66a4`](address-fold.i64.filled-then-read.c17.eb1e66a4.c) | type=i64, shape=filled-then-read | c17 | `68` |
| [`address-fold.i64.global-many-readers.c17.0bf45492`](address-fold.i64.global-many-readers.c17.0bf45492.c) | type=i64, shape=global-many-readers | c17 | `26` |
| [`address-fold.i64.reader-in-another-block.c17.df421e38`](address-fold.i64.reader-in-another-block.c17.df421e38.c) | type=i64, shape=reader-in-another-block | c17 | `11` |
| [`address-fold.i64.several-offsets.c17.05a18e62`](address-fold.i64.several-offsets.c17.05a18e62.c) | type=i64, shape=several-offsets | c17 | `18` |
| [`address-fold.i64.store-then-load.c17.d7b3aef3`](address-fold.i64.store-then-load.c17.d7b3aef3.c) | type=i64, shape=store-then-load | c17 | `6` |
| [`address-fold.u32.base-written-between.c17.6b474351`](address-fold.u32.base-written-between.c17.6b474351.c) | type=u32, shape=base-written-between | c17 | `11` |
| [`address-fold.u32.computed-index.c17.eb454765`](address-fold.u32.computed-index.c17.eb454765.c) | type=u32, shape=computed-index | c17 | `11` |
| [`address-fold.u32.filled-then-read.c17.1dbd237e`](address-fold.u32.filled-then-read.c17.1dbd237e.c) | type=u32, shape=filled-then-read | c17 | `68` |
| [`address-fold.u32.global-many-readers.c17.dde9e3d0`](address-fold.u32.global-many-readers.c17.dde9e3d0.c) | type=u32, shape=global-many-readers | c17 | `26` |
| [`address-fold.u32.reader-in-another-block.c17.d8ac5657`](address-fold.u32.reader-in-another-block.c17.d8ac5657.c) | type=u32, shape=reader-in-another-block | c17 | `11` |
| [`address-fold.u32.several-offsets.c17.37c178b1`](address-fold.u32.several-offsets.c17.37c178b1.c) | type=u32, shape=several-offsets | c17 | `18` |
| [`address-fold.u32.store-then-load.c17.f2c136ed`](address-fold.u32.store-then-load.c17.f2c136ed.c) | type=u32, shape=store-then-load | c17 | `6` |
| [`address-fold.u64.base-written-between.c17.37632378`](address-fold.u64.base-written-between.c17.37632378.c) | type=u64, shape=base-written-between | c17 | `11` |
| [`address-fold.u64.computed-index.c17.ea176739`](address-fold.u64.computed-index.c17.ea176739.c) | type=u64, shape=computed-index | c17 | `11` |
| [`address-fold.u64.filled-then-read.c17.21189118`](address-fold.u64.filled-then-read.c17.21189118.c) | type=u64, shape=filled-then-read | c17 | `68` |
| [`address-fold.u64.global-many-readers.c17.3eef5891`](address-fold.u64.global-many-readers.c17.3eef5891.c) | type=u64, shape=global-many-readers | c17 | `26` |
| [`address-fold.u64.reader-in-another-block.c17.4a52ab12`](address-fold.u64.reader-in-another-block.c17.4a52ab12.c) | type=u64, shape=reader-in-another-block | c17 | `11` |
| [`address-fold.u64.several-offsets.c17.0c9f7ed8`](address-fold.u64.several-offsets.c17.0c9f7ed8.c) | type=u64, shape=several-offsets | c17 | `18` |
| [`address-fold.u64.store-then-load.c17.681cd0e0`](address-fold.u64.store-then-load.c17.681cd0e0.c) | type=u64, shape=store-then-load | c17 | `6` |

