# store-fold

a load, arithmetic on it, and a store back to the same place. Part of the backend phase of the M4 plan.

56 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`store-fold.i32.add.c17.40ccda32`](store-fold.i32.add.c17.40ccda32.c) | type=i32, shape=add | c17 | `13` |
| [`store-fold.i32.answer-read.c17.1d95657f`](store-fold.i32.answer-read.c17.1d95657f.c) | type=i32, shape=answer-read | c17 | `26` |
| [`store-fold.i32.bitwise-and.c17.b26ccc70`](store-fold.i32.bitwise-and.c17.b26ccc70.c) | type=i32, shape=bitwise-and | c17 | `8` |
| [`store-fold.i32.bitwise-or.c17.3e9fc184`](store-fold.i32.bitwise-or.c17.3e9fc184.c) | type=i32, shape=bitwise-or | c17 | `13` |
| [`store-fold.i32.call-between.c17.8c6e6625`](store-fold.i32.call-between.c17.8c6e6625.c) | type=i32, shape=call-between | c17 | `13` |
| [`store-fold.i32.exclusive-or.c17.da23d84b`](store-fold.i32.exclusive-or.c17.da23d84b.c) | type=i32, shape=exclusive-or | c17 | `13` |
| [`store-fold.i32.frame-slots.c17.51558cd9`](store-fold.i32.frame-slots.c17.51558cd9.c) | type=i32, shape=frame-slots | c17 | `15` |
| [`store-fold.i32.index-written-between.c17.d1a8cfa8`](store-fold.i32.index-written-between.c17.d1a8cfa8.c) | type=i32, shape=index-written-between | c17 | `21` |
| [`store-fold.i32.other-offset.c17.0b216378`](store-fold.i32.other-offset.c17.0b216378.c) | type=i32, shape=other-offset | c17 | `21` |
| [`store-fold.i32.other-place.c17.01626995`](store-fold.i32.other-place.c17.01626995.c) | type=i32, shape=other-place | c17 | `21` |
| [`store-fold.i32.store-between.c17.bb6b99f4`](store-fold.i32.store-between.c17.bb6b99f4.c) | type=i32, shape=store-between | c17 | `18` |
| [`store-fold.i32.subtract-reversed.c17.7c808f68`](store-fold.i32.subtract-reversed.c17.7c808f68.c) | type=i32, shape=subtract-reversed | c17 | `17` |
| [`store-fold.i32.subtract.c17.9a5eb606`](store-fold.i32.subtract.c17.9a5eb606.c) | type=i32, shape=subtract | c17 | `3` |
| [`store-fold.i32.two-readers.c17.fb47cd08`](store-fold.i32.two-readers.c17.fb47cd08.c) | type=i32, shape=two-readers | c17 | `21` |
| [`store-fold.i64.add.c17.c0c3f62c`](store-fold.i64.add.c17.c0c3f62c.c) | type=i64, shape=add | c17 | `13` |
| [`store-fold.i64.answer-read.c17.bd8f7e50`](store-fold.i64.answer-read.c17.bd8f7e50.c) | type=i64, shape=answer-read | c17 | `26` |
| [`store-fold.i64.bitwise-and.c17.27671264`](store-fold.i64.bitwise-and.c17.27671264.c) | type=i64, shape=bitwise-and | c17 | `8` |
| [`store-fold.i64.bitwise-or.c17.54080256`](store-fold.i64.bitwise-or.c17.54080256.c) | type=i64, shape=bitwise-or | c17 | `13` |
| [`store-fold.i64.call-between.c17.67d07731`](store-fold.i64.call-between.c17.67d07731.c) | type=i64, shape=call-between | c17 | `13` |
| [`store-fold.i64.exclusive-or.c17.ac01aa2c`](store-fold.i64.exclusive-or.c17.ac01aa2c.c) | type=i64, shape=exclusive-or | c17 | `13` |
| [`store-fold.i64.frame-slots.c17.3ece2629`](store-fold.i64.frame-slots.c17.3ece2629.c) | type=i64, shape=frame-slots | c17 | `15` |
| [`store-fold.i64.index-written-between.c17.19d77698`](store-fold.i64.index-written-between.c17.19d77698.c) | type=i64, shape=index-written-between | c17 | `21` |
| [`store-fold.i64.other-offset.c17.5dfb73ce`](store-fold.i64.other-offset.c17.5dfb73ce.c) | type=i64, shape=other-offset | c17 | `21` |
| [`store-fold.i64.other-place.c17.b24add1d`](store-fold.i64.other-place.c17.b24add1d.c) | type=i64, shape=other-place | c17 | `21` |
| [`store-fold.i64.store-between.c17.89970ff3`](store-fold.i64.store-between.c17.89970ff3.c) | type=i64, shape=store-between | c17 | `18` |
| [`store-fold.i64.subtract-reversed.c17.24191aac`](store-fold.i64.subtract-reversed.c17.24191aac.c) | type=i64, shape=subtract-reversed | c17 | `17` |
| [`store-fold.i64.subtract.c17.2763da9d`](store-fold.i64.subtract.c17.2763da9d.c) | type=i64, shape=subtract | c17 | `3` |
| [`store-fold.i64.two-readers.c17.62d16394`](store-fold.i64.two-readers.c17.62d16394.c) | type=i64, shape=two-readers | c17 | `21` |
| [`store-fold.u32.add.c17.039e9502`](store-fold.u32.add.c17.039e9502.c) | type=u32, shape=add | c17 | `13` |
| [`store-fold.u32.answer-read.c17.abd48062`](store-fold.u32.answer-read.c17.abd48062.c) | type=u32, shape=answer-read | c17 | `26` |
| [`store-fold.u32.bitwise-and.c17.e2a9b2d3`](store-fold.u32.bitwise-and.c17.e2a9b2d3.c) | type=u32, shape=bitwise-and | c17 | `8` |
| [`store-fold.u32.bitwise-or.c17.29d34dbc`](store-fold.u32.bitwise-or.c17.29d34dbc.c) | type=u32, shape=bitwise-or | c17 | `13` |
| [`store-fold.u32.call-between.c17.43e29d12`](store-fold.u32.call-between.c17.43e29d12.c) | type=u32, shape=call-between | c17 | `13` |
| [`store-fold.u32.exclusive-or.c17.ea6e97e5`](store-fold.u32.exclusive-or.c17.ea6e97e5.c) | type=u32, shape=exclusive-or | c17 | `13` |
| [`store-fold.u32.frame-slots.c17.ffb3b24a`](store-fold.u32.frame-slots.c17.ffb3b24a.c) | type=u32, shape=frame-slots | c17 | `15` |
| [`store-fold.u32.index-written-between.c17.f4133044`](store-fold.u32.index-written-between.c17.f4133044.c) | type=u32, shape=index-written-between | c17 | `21` |
| [`store-fold.u32.other-offset.c17.070baa6e`](store-fold.u32.other-offset.c17.070baa6e.c) | type=u32, shape=other-offset | c17 | `21` |
| [`store-fold.u32.other-place.c17.7e2218e6`](store-fold.u32.other-place.c17.7e2218e6.c) | type=u32, shape=other-place | c17 | `21` |
| [`store-fold.u32.store-between.c17.e9ee5c40`](store-fold.u32.store-between.c17.e9ee5c40.c) | type=u32, shape=store-between | c17 | `18` |
| [`store-fold.u32.subtract-reversed.c17.b753d59d`](store-fold.u32.subtract-reversed.c17.b753d59d.c) | type=u32, shape=subtract-reversed | c17 | `17` |
| [`store-fold.u32.subtract.c17.a11a55d5`](store-fold.u32.subtract.c17.a11a55d5.c) | type=u32, shape=subtract | c17 | `3` |
| [`store-fold.u32.two-readers.c17.36962be4`](store-fold.u32.two-readers.c17.36962be4.c) | type=u32, shape=two-readers | c17 | `21` |
| [`store-fold.u64.add.c17.9768e8b4`](store-fold.u64.add.c17.9768e8b4.c) | type=u64, shape=add | c17 | `13` |
| [`store-fold.u64.answer-read.c17.5b9d5ba4`](store-fold.u64.answer-read.c17.5b9d5ba4.c) | type=u64, shape=answer-read | c17 | `26` |
| [`store-fold.u64.bitwise-and.c17.b074990a`](store-fold.u64.bitwise-and.c17.b074990a.c) | type=u64, shape=bitwise-and | c17 | `8` |
| [`store-fold.u64.bitwise-or.c17.51c7b04f`](store-fold.u64.bitwise-or.c17.51c7b04f.c) | type=u64, shape=bitwise-or | c17 | `13` |
| [`store-fold.u64.call-between.c17.f7ee5cd9`](store-fold.u64.call-between.c17.f7ee5cd9.c) | type=u64, shape=call-between | c17 | `13` |
| [`store-fold.u64.exclusive-or.c17.fd6a562e`](store-fold.u64.exclusive-or.c17.fd6a562e.c) | type=u64, shape=exclusive-or | c17 | `13` |
| [`store-fold.u64.frame-slots.c17.91a2890b`](store-fold.u64.frame-slots.c17.91a2890b.c) | type=u64, shape=frame-slots | c17 | `15` |
| [`store-fold.u64.index-written-between.c17.bf196b2a`](store-fold.u64.index-written-between.c17.bf196b2a.c) | type=u64, shape=index-written-between | c17 | `21` |
| [`store-fold.u64.other-offset.c17.9b897c7f`](store-fold.u64.other-offset.c17.9b897c7f.c) | type=u64, shape=other-offset | c17 | `21` |
| [`store-fold.u64.other-place.c17.24019817`](store-fold.u64.other-place.c17.24019817.c) | type=u64, shape=other-place | c17 | `21` |
| [`store-fold.u64.store-between.c17.a9aec524`](store-fold.u64.store-between.c17.a9aec524.c) | type=u64, shape=store-between | c17 | `18` |
| [`store-fold.u64.subtract-reversed.c17.826150d8`](store-fold.u64.subtract-reversed.c17.826150d8.c) | type=u64, shape=subtract-reversed | c17 | `17` |
| [`store-fold.u64.subtract.c17.1befa639`](store-fold.u64.subtract.c17.1befa639.c) | type=u64, shape=subtract | c17 | `3` |
| [`store-fold.u64.two-readers.c17.1bb3f6f6`](store-fold.u64.two-readers.c17.1bb3f6f6.c) | type=u64, shape=two-readers | c17 | `21` |

