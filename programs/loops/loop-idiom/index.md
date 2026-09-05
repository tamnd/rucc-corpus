# loop-idiom

recognizing a loop the runtime already implements. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-idiom.i32.100.copy.c17.f4b0255e`](loop-idiom.i32.100.copy.c17.f4b0255e.c) | type=i32, trips=100, kind=copy | c17 | `1 100` |
| [`loop-idiom.i32.100.count.c17.e22bdfce`](loop-idiom.i32.100.count.c17.e22bdfce.c) | type=i32, trips=100, kind=count | c17 | `33` |
| [`loop-idiom.i32.100.fill.c17.6854d658`](loop-idiom.i32.100.fill.c17.6854d658.c) | type=i32, trips=100, kind=fill | c17 | `0 0` |
| [`loop-idiom.i32.100.sum.c17.3d9f15d0`](loop-idiom.i32.100.sum.c17.3d9f15d0.c) | type=i32, trips=100, kind=sum | c17 | `5050` |
| [`loop-idiom.i32.16.copy.c17.da9d0b3b`](loop-idiom.i32.16.copy.c17.da9d0b3b.c) | type=i32, trips=16, kind=copy | c17 | `1 16` |
| [`loop-idiom.i32.16.count.c17.9867715b`](loop-idiom.i32.16.count.c17.9867715b.c) | type=i32, trips=16, kind=count | c17 | `5` |
| [`loop-idiom.i32.16.fill.c17.9fc9e5b6`](loop-idiom.i32.16.fill.c17.9fc9e5b6.c) | type=i32, trips=16, kind=fill | c17 | `0 0` |
| [`loop-idiom.i32.16.sum.c17.f7027c53`](loop-idiom.i32.16.sum.c17.f7027c53.c) | type=i32, trips=16, kind=sum | c17 | `136` |
| [`loop-idiom.i32.4.copy.c17.b2616640`](loop-idiom.i32.4.copy.c17.b2616640.c) | type=i32, trips=4, kind=copy | c17 | `1 4` |
| [`loop-idiom.i32.4.count.c17.02fe05bf`](loop-idiom.i32.4.count.c17.02fe05bf.c) | type=i32, trips=4, kind=count | c17 | `1` |
| [`loop-idiom.i32.4.fill.c17.2a2f4205`](loop-idiom.i32.4.fill.c17.2a2f4205.c) | type=i32, trips=4, kind=fill | c17 | `0 0` |
| [`loop-idiom.i32.4.sum.c17.23c54309`](loop-idiom.i32.4.sum.c17.23c54309.c) | type=i32, trips=4, kind=sum | c17 | `10` |
| [`loop-idiom.i32.7.copy.c17.7dc042b0`](loop-idiom.i32.7.copy.c17.7dc042b0.c) | type=i32, trips=7, kind=copy | c17 | `1 7` |
| [`loop-idiom.i32.7.count.c17.0b7af878`](loop-idiom.i32.7.count.c17.0b7af878.c) | type=i32, trips=7, kind=count | c17 | `2` |
| [`loop-idiom.i32.7.fill.c17.24242fb8`](loop-idiom.i32.7.fill.c17.24242fb8.c) | type=i32, trips=7, kind=fill | c17 | `0 0` |
| [`loop-idiom.i32.7.sum.c17.efbd793d`](loop-idiom.i32.7.sum.c17.efbd793d.c) | type=i32, trips=7, kind=sum | c17 | `28` |
| [`loop-idiom.i64.100.copy.c17.ff52efe6`](loop-idiom.i64.100.copy.c17.ff52efe6.c) | type=i64, trips=100, kind=copy | c17 | `1 100` |
| [`loop-idiom.i64.100.count.c17.a39ccb2a`](loop-idiom.i64.100.count.c17.a39ccb2a.c) | type=i64, trips=100, kind=count | c17 | `33` |
| [`loop-idiom.i64.100.fill.c17.25464313`](loop-idiom.i64.100.fill.c17.25464313.c) | type=i64, trips=100, kind=fill | c17 | `0 0` |
| [`loop-idiom.i64.100.sum.c17.bc61948f`](loop-idiom.i64.100.sum.c17.bc61948f.c) | type=i64, trips=100, kind=sum | c17 | `5050` |
| [`loop-idiom.i64.16.copy.c17.ce6a9275`](loop-idiom.i64.16.copy.c17.ce6a9275.c) | type=i64, trips=16, kind=copy | c17 | `1 16` |
| [`loop-idiom.i64.16.count.c17.23823c22`](loop-idiom.i64.16.count.c17.23823c22.c) | type=i64, trips=16, kind=count | c17 | `5` |
| [`loop-idiom.i64.16.fill.c17.07d6c15c`](loop-idiom.i64.16.fill.c17.07d6c15c.c) | type=i64, trips=16, kind=fill | c17 | `0 0` |
| [`loop-idiom.i64.16.sum.c17.6118f1a5`](loop-idiom.i64.16.sum.c17.6118f1a5.c) | type=i64, trips=16, kind=sum | c17 | `136` |
| [`loop-idiom.i64.4.copy.c17.301566f5`](loop-idiom.i64.4.copy.c17.301566f5.c) | type=i64, trips=4, kind=copy | c17 | `1 4` |
| [`loop-idiom.i64.4.count.c17.fb1a136c`](loop-idiom.i64.4.count.c17.fb1a136c.c) | type=i64, trips=4, kind=count | c17 | `1` |
| [`loop-idiom.i64.4.fill.c17.f619a13b`](loop-idiom.i64.4.fill.c17.f619a13b.c) | type=i64, trips=4, kind=fill | c17 | `0 0` |
| [`loop-idiom.i64.4.sum.c17.8787016c`](loop-idiom.i64.4.sum.c17.8787016c.c) | type=i64, trips=4, kind=sum | c17 | `10` |
| [`loop-idiom.i64.7.copy.c17.91972074`](loop-idiom.i64.7.copy.c17.91972074.c) | type=i64, trips=7, kind=copy | c17 | `1 7` |
| [`loop-idiom.i64.7.count.c17.27388b98`](loop-idiom.i64.7.count.c17.27388b98.c) | type=i64, trips=7, kind=count | c17 | `2` |
| [`loop-idiom.i64.7.fill.c17.d0d972d9`](loop-idiom.i64.7.fill.c17.d0d972d9.c) | type=i64, trips=7, kind=fill | c17 | `0 0` |
| [`loop-idiom.i64.7.sum.c17.b6bec5f1`](loop-idiom.i64.7.sum.c17.b6bec5f1.c) | type=i64, trips=7, kind=sum | c17 | `28` |
| [`loop-idiom.u32.100.copy.c17.83859b83`](loop-idiom.u32.100.copy.c17.83859b83.c) | type=u32, trips=100, kind=copy | c17 | `1 100` |
| [`loop-idiom.u32.100.count.c17.5023b0a0`](loop-idiom.u32.100.count.c17.5023b0a0.c) | type=u32, trips=100, kind=count | c17 | `33` |
| [`loop-idiom.u32.100.fill.c17.880ac645`](loop-idiom.u32.100.fill.c17.880ac645.c) | type=u32, trips=100, kind=fill | c17 | `0 0` |
| [`loop-idiom.u32.100.sum.c17.e1362fe4`](loop-idiom.u32.100.sum.c17.e1362fe4.c) | type=u32, trips=100, kind=sum | c17 | `5050` |
| [`loop-idiom.u32.16.copy.c17.31de058d`](loop-idiom.u32.16.copy.c17.31de058d.c) | type=u32, trips=16, kind=copy | c17 | `1 16` |
| [`loop-idiom.u32.16.count.c17.757f9fe9`](loop-idiom.u32.16.count.c17.757f9fe9.c) | type=u32, trips=16, kind=count | c17 | `5` |
| [`loop-idiom.u32.16.fill.c17.6b0980af`](loop-idiom.u32.16.fill.c17.6b0980af.c) | type=u32, trips=16, kind=fill | c17 | `0 0` |
| [`loop-idiom.u32.16.sum.c17.f41d02a7`](loop-idiom.u32.16.sum.c17.f41d02a7.c) | type=u32, trips=16, kind=sum | c17 | `136` |
| [`loop-idiom.u32.4.copy.c17.a0251924`](loop-idiom.u32.4.copy.c17.a0251924.c) | type=u32, trips=4, kind=copy | c17 | `1 4` |
| [`loop-idiom.u32.4.count.c17.b6658d03`](loop-idiom.u32.4.count.c17.b6658d03.c) | type=u32, trips=4, kind=count | c17 | `1` |
| [`loop-idiom.u32.4.fill.c17.583ee289`](loop-idiom.u32.4.fill.c17.583ee289.c) | type=u32, trips=4, kind=fill | c17 | `0 0` |
| [`loop-idiom.u32.4.sum.c17.ad41f055`](loop-idiom.u32.4.sum.c17.ad41f055.c) | type=u32, trips=4, kind=sum | c17 | `10` |
| [`loop-idiom.u32.7.copy.c17.e1fd4810`](loop-idiom.u32.7.copy.c17.e1fd4810.c) | type=u32, trips=7, kind=copy | c17 | `1 7` |
| [`loop-idiom.u32.7.count.c17.9894752b`](loop-idiom.u32.7.count.c17.9894752b.c) | type=u32, trips=7, kind=count | c17 | `2` |
| [`loop-idiom.u32.7.fill.c17.dbe8fff9`](loop-idiom.u32.7.fill.c17.dbe8fff9.c) | type=u32, trips=7, kind=fill | c17 | `0 0` |
| [`loop-idiom.u32.7.sum.c17.db164951`](loop-idiom.u32.7.sum.c17.db164951.c) | type=u32, trips=7, kind=sum | c17 | `28` |
| [`loop-idiom.u64.100.copy.c17.7b9f9cf8`](loop-idiom.u64.100.copy.c17.7b9f9cf8.c) | type=u64, trips=100, kind=copy | c17 | `1 100` |
| [`loop-idiom.u64.100.count.c17.f4bef02f`](loop-idiom.u64.100.count.c17.f4bef02f.c) | type=u64, trips=100, kind=count | c17 | `33` |
| [`loop-idiom.u64.100.fill.c17.8c61c692`](loop-idiom.u64.100.fill.c17.8c61c692.c) | type=u64, trips=100, kind=fill | c17 | `0 0` |
| [`loop-idiom.u64.100.sum.c17.1f553fc5`](loop-idiom.u64.100.sum.c17.1f553fc5.c) | type=u64, trips=100, kind=sum | c17 | `5050` |
| [`loop-idiom.u64.16.copy.c17.b31e8e9f`](loop-idiom.u64.16.copy.c17.b31e8e9f.c) | type=u64, trips=16, kind=copy | c17 | `1 16` |
| [`loop-idiom.u64.16.count.c17.e0496fc0`](loop-idiom.u64.16.count.c17.e0496fc0.c) | type=u64, trips=16, kind=count | c17 | `5` |
| [`loop-idiom.u64.16.fill.c17.1ec8b0a3`](loop-idiom.u64.16.fill.c17.1ec8b0a3.c) | type=u64, trips=16, kind=fill | c17 | `0 0` |
| [`loop-idiom.u64.16.sum.c17.5a54da6a`](loop-idiom.u64.16.sum.c17.5a54da6a.c) | type=u64, trips=16, kind=sum | c17 | `136` |
| [`loop-idiom.u64.4.copy.c17.22f57385`](loop-idiom.u64.4.copy.c17.22f57385.c) | type=u64, trips=4, kind=copy | c17 | `1 4` |
| [`loop-idiom.u64.4.count.c17.61fe8f95`](loop-idiom.u64.4.count.c17.61fe8f95.c) | type=u64, trips=4, kind=count | c17 | `1` |
| [`loop-idiom.u64.4.fill.c17.daaf6943`](loop-idiom.u64.4.fill.c17.daaf6943.c) | type=u64, trips=4, kind=fill | c17 | `0 0` |
| [`loop-idiom.u64.4.sum.c17.24fa3e06`](loop-idiom.u64.4.sum.c17.24fa3e06.c) | type=u64, trips=4, kind=sum | c17 | `10` |
| [`loop-idiom.u64.7.copy.c17.6ee44c17`](loop-idiom.u64.7.copy.c17.6ee44c17.c) | type=u64, trips=7, kind=copy | c17 | `1 7` |
| [`loop-idiom.u64.7.count.c17.0e53ceed`](loop-idiom.u64.7.count.c17.0e53ceed.c) | type=u64, trips=7, kind=count | c17 | `2` |
| [`loop-idiom.u64.7.fill.c17.41b6eba8`](loop-idiom.u64.7.fill.c17.41b6eba8.c) | type=u64, trips=7, kind=fill | c17 | `0 0` |
| [`loop-idiom.u64.7.sum.c17.d62365d5`](loop-idiom.u64.7.sum.c17.d62365d5.c) | type=u64, trips=7, kind=sum | c17 | `28` |

