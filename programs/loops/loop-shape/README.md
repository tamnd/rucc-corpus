# loop-shape

the shape a loop is left in, before any pass reads it. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-shape.i32.already-canonical.c17.e490e164`](loop-shape.i32.already-canonical.c17.e490e164.c) | type=i32, shape=already-canonical | c17 | `28` |
| [`loop-shape.i32.entry-disprovable.c17.673f133f`](loop-shape.i32.entry-disprovable.c17.673f133f.c) | type=i32, shape=entry-disprovable | c17 | `0` |
| [`loop-shape.i32.entry-provable.c17.5ef14537`](loop-shape.i32.entry-provable.c17.5ef14537.c) | type=i32, shape=entry-provable | c17 | `15` |
| [`loop-shape.i32.entry-unknown.c17.56b262d0`](loop-shape.i32.entry-unknown.c17.56b262d0.c) | type=i32, shape=entry-unknown | c17 | `36` |
| [`loop-shape.i32.header-many-preds.c17.de675690`](loop-shape.i32.header-many-preds.c17.de675690.c) | type=i32, shape=header-many-preds | c17 | `28` |
| [`loop-shape.i32.header-over-size-limit.c17.dc7ccd09`](loop-shape.i32.header-over-size-limit.c17.dc7ccd09.c) | type=i32, shape=header-over-size-limit | c17 | `6` |
| [`loop-shape.i32.header-over-speed-limit.c17.ae0df454`](loop-shape.i32.header-over-speed-limit.c17.ae0df454.c) | type=i32, shape=header-over-speed-limit | c17 | `6` |
| [`loop-shape.i32.header-store.c17.bba84fbf`](loop-shape.i32.header-store.c17.bba84fbf.c) | type=i32, shape=header-store | c17 | `28` |
| [`loop-shape.i32.irreducible.c17.444bd879`](loop-shape.i32.irreducible.c17.444bd879.c) | type=i32, shape=irreducible | c17 | `44` |
| [`loop-shape.i32.live-out-from-header.c17.26831ec2`](loop-shape.i32.live-out-from-header.c17.26831ec2.c) | type=i32, shape=live-out-from-header | c17 | `21 63` |
| [`loop-shape.i32.live-out-multi-exit.c17.33dd9092`](loop-shape.i32.live-out-multi-exit.c17.33dd9092.c) | type=i32, shape=live-out-multi-exit | c17 | `6 15` |
| [`loop-shape.i32.live-out-once.c17.bfabfdb4`](loop-shape.i32.live-out-once.c17.bfabfdb4.c) | type=i32, shape=live-out-once | c17 | `10 45` |
| [`loop-shape.i32.live-out-twice.c17.46f8892a`](loop-shape.i32.live-out-twice.c17.46f8892a.c) | type=i32, shape=live-out-twice | c17 | `10 20 45` |
| [`loop-shape.i32.two-exits-one-join.c17.6538affe`](loop-shape.i32.two-exits-one-join.c17.6538affe.c) | type=i32, shape=two-exits-one-join | c17 | `10` |
| [`loop-shape.i32.two-latches.c17.7d984fbb`](loop-shape.i32.two-latches.c17.7d984fbb.c) | type=i32, shape=two-latches | c17 | `104` |
| [`loop-shape.i32.while-to-do-while.c17.36a0a503`](loop-shape.i32.while-to-do-while.c17.36a0a503.c) | type=i32, shape=while-to-do-while | c17 | `45` |
| [`loop-shape.i64.already-canonical.c17.10713e32`](loop-shape.i64.already-canonical.c17.10713e32.c) | type=i64, shape=already-canonical | c17 | `28` |
| [`loop-shape.i64.entry-disprovable.c17.847fcc60`](loop-shape.i64.entry-disprovable.c17.847fcc60.c) | type=i64, shape=entry-disprovable | c17 | `0` |
| [`loop-shape.i64.entry-provable.c17.2a15ed8a`](loop-shape.i64.entry-provable.c17.2a15ed8a.c) | type=i64, shape=entry-provable | c17 | `15` |
| [`loop-shape.i64.entry-unknown.c17.59051e97`](loop-shape.i64.entry-unknown.c17.59051e97.c) | type=i64, shape=entry-unknown | c17 | `36` |
| [`loop-shape.i64.header-many-preds.c17.8fc71270`](loop-shape.i64.header-many-preds.c17.8fc71270.c) | type=i64, shape=header-many-preds | c17 | `28` |
| [`loop-shape.i64.header-over-size-limit.c17.82b93ca2`](loop-shape.i64.header-over-size-limit.c17.82b93ca2.c) | type=i64, shape=header-over-size-limit | c17 | `6` |
| [`loop-shape.i64.header-over-speed-limit.c17.baedae1e`](loop-shape.i64.header-over-speed-limit.c17.baedae1e.c) | type=i64, shape=header-over-speed-limit | c17 | `6` |
| [`loop-shape.i64.header-store.c17.d66a4170`](loop-shape.i64.header-store.c17.d66a4170.c) | type=i64, shape=header-store | c17 | `28` |
| [`loop-shape.i64.irreducible.c17.748100bf`](loop-shape.i64.irreducible.c17.748100bf.c) | type=i64, shape=irreducible | c17 | `44` |
| [`loop-shape.i64.live-out-from-header.c17.ae65a6a4`](loop-shape.i64.live-out-from-header.c17.ae65a6a4.c) | type=i64, shape=live-out-from-header | c17 | `21 63` |
| [`loop-shape.i64.live-out-multi-exit.c17.25638a10`](loop-shape.i64.live-out-multi-exit.c17.25638a10.c) | type=i64, shape=live-out-multi-exit | c17 | `6 15` |
| [`loop-shape.i64.live-out-once.c17.77855dab`](loop-shape.i64.live-out-once.c17.77855dab.c) | type=i64, shape=live-out-once | c17 | `10 45` |
| [`loop-shape.i64.live-out-twice.c17.3c7c1661`](loop-shape.i64.live-out-twice.c17.3c7c1661.c) | type=i64, shape=live-out-twice | c17 | `10 20 45` |
| [`loop-shape.i64.two-exits-one-join.c17.a2b7d0c5`](loop-shape.i64.two-exits-one-join.c17.a2b7d0c5.c) | type=i64, shape=two-exits-one-join | c17 | `10` |
| [`loop-shape.i64.two-latches.c17.0fefc807`](loop-shape.i64.two-latches.c17.0fefc807.c) | type=i64, shape=two-latches | c17 | `104` |
| [`loop-shape.i64.while-to-do-while.c17.c893cabc`](loop-shape.i64.while-to-do-while.c17.c893cabc.c) | type=i64, shape=while-to-do-while | c17 | `45` |
| [`loop-shape.u32.already-canonical.c17.75958acf`](loop-shape.u32.already-canonical.c17.75958acf.c) | type=u32, shape=already-canonical | c17 | `28` |
| [`loop-shape.u32.entry-disprovable.c17.b5fd9626`](loop-shape.u32.entry-disprovable.c17.b5fd9626.c) | type=u32, shape=entry-disprovable | c17 | `0` |
| [`loop-shape.u32.entry-provable.c17.fdb820d1`](loop-shape.u32.entry-provable.c17.fdb820d1.c) | type=u32, shape=entry-provable | c17 | `15` |
| [`loop-shape.u32.entry-unknown.c17.5af246ed`](loop-shape.u32.entry-unknown.c17.5af246ed.c) | type=u32, shape=entry-unknown | c17 | `36` |
| [`loop-shape.u32.header-many-preds.c17.39884cd7`](loop-shape.u32.header-many-preds.c17.39884cd7.c) | type=u32, shape=header-many-preds | c17 | `28` |
| [`loop-shape.u32.header-over-size-limit.c17.48e4214d`](loop-shape.u32.header-over-size-limit.c17.48e4214d.c) | type=u32, shape=header-over-size-limit | c17 | `6` |
| [`loop-shape.u32.header-over-speed-limit.c17.344a0080`](loop-shape.u32.header-over-speed-limit.c17.344a0080.c) | type=u32, shape=header-over-speed-limit | c17 | `6` |
| [`loop-shape.u32.header-store.c17.d269d472`](loop-shape.u32.header-store.c17.d269d472.c) | type=u32, shape=header-store | c17 | `28` |
| [`loop-shape.u32.irreducible.c17.cfcf791c`](loop-shape.u32.irreducible.c17.cfcf791c.c) | type=u32, shape=irreducible | c17 | `44` |
| [`loop-shape.u32.live-out-from-header.c17.2490070f`](loop-shape.u32.live-out-from-header.c17.2490070f.c) | type=u32, shape=live-out-from-header | c17 | `21 63` |
| [`loop-shape.u32.live-out-multi-exit.c17.a2d0c485`](loop-shape.u32.live-out-multi-exit.c17.a2d0c485.c) | type=u32, shape=live-out-multi-exit | c17 | `6 15` |
| [`loop-shape.u32.live-out-once.c17.f9d29978`](loop-shape.u32.live-out-once.c17.f9d29978.c) | type=u32, shape=live-out-once | c17 | `10 45` |
| [`loop-shape.u32.live-out-twice.c17.69f53dd9`](loop-shape.u32.live-out-twice.c17.69f53dd9.c) | type=u32, shape=live-out-twice | c17 | `10 20 45` |
| [`loop-shape.u32.two-exits-one-join.c17.bb3455ad`](loop-shape.u32.two-exits-one-join.c17.bb3455ad.c) | type=u32, shape=two-exits-one-join | c17 | `10` |
| [`loop-shape.u32.two-latches.c17.5679484a`](loop-shape.u32.two-latches.c17.5679484a.c) | type=u32, shape=two-latches | c17 | `104` |
| [`loop-shape.u32.while-to-do-while.c17.829eda3a`](loop-shape.u32.while-to-do-while.c17.829eda3a.c) | type=u32, shape=while-to-do-while | c17 | `45` |
| [`loop-shape.u64.already-canonical.c17.dae5f228`](loop-shape.u64.already-canonical.c17.dae5f228.c) | type=u64, shape=already-canonical | c17 | `28` |
| [`loop-shape.u64.entry-disprovable.c17.a79811d4`](loop-shape.u64.entry-disprovable.c17.a79811d4.c) | type=u64, shape=entry-disprovable | c17 | `0` |
| [`loop-shape.u64.entry-provable.c17.5ce12e21`](loop-shape.u64.entry-provable.c17.5ce12e21.c) | type=u64, shape=entry-provable | c17 | `15` |
| [`loop-shape.u64.entry-unknown.c17.3143bc20`](loop-shape.u64.entry-unknown.c17.3143bc20.c) | type=u64, shape=entry-unknown | c17 | `36` |
| [`loop-shape.u64.header-many-preds.c17.e2c12550`](loop-shape.u64.header-many-preds.c17.e2c12550.c) | type=u64, shape=header-many-preds | c17 | `28` |
| [`loop-shape.u64.header-over-size-limit.c17.e6ec80c8`](loop-shape.u64.header-over-size-limit.c17.e6ec80c8.c) | type=u64, shape=header-over-size-limit | c17 | `6` |
| [`loop-shape.u64.header-over-speed-limit.c17.6c8f22eb`](loop-shape.u64.header-over-speed-limit.c17.6c8f22eb.c) | type=u64, shape=header-over-speed-limit | c17 | `6` |
| [`loop-shape.u64.header-store.c17.9435c95c`](loop-shape.u64.header-store.c17.9435c95c.c) | type=u64, shape=header-store | c17 | `28` |
| [`loop-shape.u64.irreducible.c17.64547fc7`](loop-shape.u64.irreducible.c17.64547fc7.c) | type=u64, shape=irreducible | c17 | `44` |
| [`loop-shape.u64.live-out-from-header.c17.2f91dffa`](loop-shape.u64.live-out-from-header.c17.2f91dffa.c) | type=u64, shape=live-out-from-header | c17 | `21 63` |
| [`loop-shape.u64.live-out-multi-exit.c17.aebf375a`](loop-shape.u64.live-out-multi-exit.c17.aebf375a.c) | type=u64, shape=live-out-multi-exit | c17 | `6 15` |
| [`loop-shape.u64.live-out-once.c17.689d057c`](loop-shape.u64.live-out-once.c17.689d057c.c) | type=u64, shape=live-out-once | c17 | `10 45` |
| [`loop-shape.u64.live-out-twice.c17.9f885849`](loop-shape.u64.live-out-twice.c17.9f885849.c) | type=u64, shape=live-out-twice | c17 | `10 20 45` |
| [`loop-shape.u64.two-exits-one-join.c17.2ed92b4f`](loop-shape.u64.two-exits-one-join.c17.2ed92b4f.c) | type=u64, shape=two-exits-one-join | c17 | `10` |
| [`loop-shape.u64.two-latches.c17.3692323e`](loop-shape.u64.two-latches.c17.3692323e.c) | type=u64, shape=two-latches | c17 | `104` |
| [`loop-shape.u64.while-to-do-while.c17.628a6bf9`](loop-shape.u64.while-to-do-while.c17.628a6bf9.c) | type=u64, shape=while-to-do-while | c17 | `45` |

