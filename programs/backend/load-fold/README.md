# load-fold

a load one arithmetic instruction reads, and what stops it moving. Part of the backend phase of the M4 plan.

40 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`load-fold.i32.call-between.c17.d757bbc7`](load-fold.i32.call-between.c17.d757bbc7.c) | type=i32, shape=call-between | c17 | `13` |
| [`load-fold.i32.index-written-between.c17.0e6808dc`](load-fold.i32.index-written-between.c17.0e6808dc.c) | type=i32, shape=index-written-between | c17 | `19` |
| [`load-fold.i32.load-between.c17.e2947d0e`](load-fold.i32.load-between.c17.e2947d0e.c) | type=i32, shape=load-between | c17 | `5` |
| [`load-fold.i32.narrower-load.c17.639b0d51`](load-fold.i32.narrower-load.c17.639b0d51.c) | type=i32, shape=narrower-load | c17 | `13` |
| [`load-fold.i32.one-reader.c17.d1491255`](load-fold.i32.one-reader.c17.d1491255.c) | type=i32, shape=one-reader | c17 | `13` |
| [`load-fold.i32.reader-in-another-block.c17.c0b17313`](load-fold.i32.reader-in-another-block.c17.c0b17313.c) | type=i32, shape=reader-in-another-block | c17 | `13` |
| [`load-fold.i32.store-between.c17.51477e5a`](load-fold.i32.store-between.c17.51477e5a.c) | type=i32, shape=store-between | c17 | `13` |
| [`load-fold.i32.subtract-left.c17.7d3a1ddd`](load-fold.i32.subtract-left.c17.7d3a1ddd.c) | type=i32, shape=subtract-left | c17 | `3` |
| [`load-fold.i32.subtract-right.c17.0e5f9c54`](load-fold.i32.subtract-right.c17.0e5f9c54.c) | type=i32, shape=subtract-right | c17 | `6` |
| [`load-fold.i32.two-readers.c17.408590cc`](load-fold.i32.two-readers.c17.408590cc.c) | type=i32, shape=two-readers | c17 | `21` |
| [`load-fold.i64.call-between.c17.5c1c52f7`](load-fold.i64.call-between.c17.5c1c52f7.c) | type=i64, shape=call-between | c17 | `13` |
| [`load-fold.i64.index-written-between.c17.05bbe56f`](load-fold.i64.index-written-between.c17.05bbe56f.c) | type=i64, shape=index-written-between | c17 | `19` |
| [`load-fold.i64.load-between.c17.9f9c8ea8`](load-fold.i64.load-between.c17.9f9c8ea8.c) | type=i64, shape=load-between | c17 | `5` |
| [`load-fold.i64.narrower-load.c17.78be368e`](load-fold.i64.narrower-load.c17.78be368e.c) | type=i64, shape=narrower-load | c17 | `13` |
| [`load-fold.i64.one-reader.c17.d309d91a`](load-fold.i64.one-reader.c17.d309d91a.c) | type=i64, shape=one-reader | c17 | `13` |
| [`load-fold.i64.reader-in-another-block.c17.126861b5`](load-fold.i64.reader-in-another-block.c17.126861b5.c) | type=i64, shape=reader-in-another-block | c17 | `13` |
| [`load-fold.i64.store-between.c17.e81fc3de`](load-fold.i64.store-between.c17.e81fc3de.c) | type=i64, shape=store-between | c17 | `13` |
| [`load-fold.i64.subtract-left.c17.94a12cb2`](load-fold.i64.subtract-left.c17.94a12cb2.c) | type=i64, shape=subtract-left | c17 | `3` |
| [`load-fold.i64.subtract-right.c17.4d5ac5a1`](load-fold.i64.subtract-right.c17.4d5ac5a1.c) | type=i64, shape=subtract-right | c17 | `6` |
| [`load-fold.i64.two-readers.c17.26fb3330`](load-fold.i64.two-readers.c17.26fb3330.c) | type=i64, shape=two-readers | c17 | `21` |
| [`load-fold.u32.call-between.c17.dedfbfd7`](load-fold.u32.call-between.c17.dedfbfd7.c) | type=u32, shape=call-between | c17 | `13` |
| [`load-fold.u32.index-written-between.c17.07a86437`](load-fold.u32.index-written-between.c17.07a86437.c) | type=u32, shape=index-written-between | c17 | `19` |
| [`load-fold.u32.load-between.c17.95fe2881`](load-fold.u32.load-between.c17.95fe2881.c) | type=u32, shape=load-between | c17 | `5` |
| [`load-fold.u32.narrower-load.c17.a75f8e17`](load-fold.u32.narrower-load.c17.a75f8e17.c) | type=u32, shape=narrower-load | c17 | `13` |
| [`load-fold.u32.one-reader.c17.90841c85`](load-fold.u32.one-reader.c17.90841c85.c) | type=u32, shape=one-reader | c17 | `13` |
| [`load-fold.u32.reader-in-another-block.c17.d668ded7`](load-fold.u32.reader-in-another-block.c17.d668ded7.c) | type=u32, shape=reader-in-another-block | c17 | `13` |
| [`load-fold.u32.store-between.c17.2df4eaf7`](load-fold.u32.store-between.c17.2df4eaf7.c) | type=u32, shape=store-between | c17 | `13` |
| [`load-fold.u32.subtract-left.c17.9c8c68e8`](load-fold.u32.subtract-left.c17.9c8c68e8.c) | type=u32, shape=subtract-left | c17 | `3` |
| [`load-fold.u32.subtract-right.c17.40654139`](load-fold.u32.subtract-right.c17.40654139.c) | type=u32, shape=subtract-right | c17 | `6` |
| [`load-fold.u32.two-readers.c17.0d29606e`](load-fold.u32.two-readers.c17.0d29606e.c) | type=u32, shape=two-readers | c17 | `21` |
| [`load-fold.u64.call-between.c17.64c57c8d`](load-fold.u64.call-between.c17.64c57c8d.c) | type=u64, shape=call-between | c17 | `13` |
| [`load-fold.u64.index-written-between.c17.573a1881`](load-fold.u64.index-written-between.c17.573a1881.c) | type=u64, shape=index-written-between | c17 | `19` |
| [`load-fold.u64.load-between.c17.faa4482f`](load-fold.u64.load-between.c17.faa4482f.c) | type=u64, shape=load-between | c17 | `5` |
| [`load-fold.u64.narrower-load.c17.6d59a70f`](load-fold.u64.narrower-load.c17.6d59a70f.c) | type=u64, shape=narrower-load | c17 | `13` |
| [`load-fold.u64.one-reader.c17.f8a58e9a`](load-fold.u64.one-reader.c17.f8a58e9a.c) | type=u64, shape=one-reader | c17 | `13` |
| [`load-fold.u64.reader-in-another-block.c17.d8e7ebcd`](load-fold.u64.reader-in-another-block.c17.d8e7ebcd.c) | type=u64, shape=reader-in-another-block | c17 | `13` |
| [`load-fold.u64.store-between.c17.5b1d57a5`](load-fold.u64.store-between.c17.5b1d57a5.c) | type=u64, shape=store-between | c17 | `13` |
| [`load-fold.u64.subtract-left.c17.c00ce31f`](load-fold.u64.subtract-left.c17.c00ce31f.c) | type=u64, shape=subtract-left | c17 | `3` |
| [`load-fold.u64.subtract-right.c17.9f6f5e35`](load-fold.u64.subtract-right.c17.9f6f5e35.c) | type=u64, shape=subtract-right | c17 | `6` |
| [`load-fold.u64.two-readers.c17.e4f1d488`](load-fold.u64.two-readers.c17.e4f1d488.c) | type=u64, shape=two-readers | c17 | `21` |

