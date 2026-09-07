# atomics

the atomic builtins at every ordering, and the header over them. Part of the correctness phase of the M4 plan.

31 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`atomics.exchange.i16.c17.b3725d59`](atomics.exchange.i16.c17.b3725d59.c) | shape=exchange, type=i16 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.i32.c17.56bac4d2`](atomics.exchange.i32.c17.56bac4d2.c) | shape=exchange, type=i32 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.i64.c17.ae722f49`](atomics.exchange.i64.c17.ae722f49.c) | shape=exchange, type=i64 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.i8.c17.718b6040`](atomics.exchange.i8.c17.718b6040.c) | shape=exchange, type=i8 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.u16.c17.c562a220`](atomics.exchange.u16.c17.c562a220.c) | shape=exchange, type=u16 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.u32.c17.6a366331`](atomics.exchange.u32.c17.6a366331.c) | shape=exchange, type=u32 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.u64.c17.72009905`](atomics.exchange.u64.c17.72009905.c) | shape=exchange, type=u64 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.exchange.u8.c17.07e6e337`](atomics.exchange.u8.c17.07e6e337.c) | shape=exchange, type=u8 | c17 | `1 10 11 ...` and 18 more lines |
| [`atomics.fence.c17.6b7c6e1f`](atomics.fence.c17.6b7c6e1f.c) | shape=fence | c17 | `3 2` |
| [`atomics.fetch-op.i16.c17.f2fddbd2`](atomics.fetch-op.i16.c17.f2fddbd2.c) | shape=fetch-op, type=i16 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.i32.c17.f8f5948f`](atomics.fetch-op.i32.c17.f8f5948f.c) | shape=fetch-op, type=i32 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.i64.c17.39b9037a`](atomics.fetch-op.i64.c17.39b9037a.c) | shape=fetch-op, type=i64 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.i8.c17.641779b5`](atomics.fetch-op.i8.c17.641779b5.c) | shape=fetch-op, type=i8 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.u16.c17.22d053de`](atomics.fetch-op.u16.c17.22d053de.c) | shape=fetch-op, type=u16 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.u32.c17.05b38601`](atomics.fetch-op.u32.c17.05b38601.c) | shape=fetch-op, type=u32 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.u64.c17.18d98da0`](atomics.fetch-op.u64.c17.18d98da0.c) | shape=fetch-op, type=u64 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.fetch-op.u8.c17.43c093fd`](atomics.fetch-op.u8.c17.43c093fd.c) | shape=fetch-op, type=u8 | c17 | `50 75 75 ...` and 22 more lines |
| [`atomics.flag.c17.c1e486c9`](atomics.flag.c17.c1e486c9.c) | shape=flag | c17 | `0 1 0 0` |
| [`atomics.load-store.i16.c17.74a1721f`](atomics.load-store.i16.c17.74a1721f.c) | shape=load-store, type=i16 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.i32.c17.c60384ba`](atomics.load-store.i32.c17.c60384ba.c) | shape=load-store, type=i32 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.i64.c17.b940f74d`](atomics.load-store.i64.c17.b940f74d.c) | shape=load-store, type=i64 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.i8.c17.359e146d`](atomics.load-store.i8.c17.359e146d.c) | shape=load-store, type=i8 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.u16.c17.0408b893`](atomics.load-store.u16.c17.0408b893.c) | shape=load-store, type=u16 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.u32.c17.153542fd`](atomics.load-store.u32.c17.153542fd.c) | shape=load-store, type=u32 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.u64.c17.66142599`](atomics.load-store.u64.c17.66142599.c) | shape=load-store, type=u64 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.load-store.u8.c17.36de058a`](atomics.load-store.u8.c17.36de058a.c) | shape=load-store, type=u8 | c17 | `3 3 3 ...` and 11 more lines |
| [`atomics.stdatomic-flag.c17.6551714c`](atomics.stdatomic-flag.c17.6551714c.c) | shape=stdatomic-flag | c17 | `0 1 0 0` |
| [`atomics.stdatomic.i32.c17.95fbc2ae`](atomics.stdatomic.i32.c17.95fbc2ae.c) | shape=stdatomic, type=i32 | c17 | `5 9 9 ...` and 13 more lines |
| [`atomics.stdatomic.i64.c17.19020add`](atomics.stdatomic.i64.c17.19020add.c) | shape=stdatomic, type=i64 | c17 | `5 9 9 ...` and 13 more lines |
| [`atomics.stdatomic.u32.c17.efd489a1`](atomics.stdatomic.u32.c17.efd489a1.c) | shape=stdatomic, type=u32 | c17 | `5 9 9 ...` and 13 more lines |
| [`atomics.stdatomic.u64.c17.53c8ec77`](atomics.stdatomic.u64.c17.53c8ec77.c) | shape=stdatomic, type=u64 | c17 | `5 9 9 ...` and 13 more lines |

