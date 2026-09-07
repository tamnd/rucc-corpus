# loop-invariant

hoisting an invariant computation out of a loop. Part of the loops phase of the M4 plan.

32 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-invariant.i32.1.c17.6199aa14`](loop-invariant.i32.1.c17.6199aa14.c) | type=i32, trips=1 | c17 | `15` |
| [`loop-invariant.i32.100.c17.fa812b0c`](loop-invariant.i32.100.c17.fa812b0c.c) | type=i32, trips=100 | c17 | `6450` |
| [`loop-invariant.i32.16.c17.9b75f4da`](loop-invariant.i32.16.c17.9b75f4da.c) | type=i32, trips=16 | c17 | `360` |
| [`loop-invariant.i32.2.c17.affd15b6`](loop-invariant.i32.2.c17.affd15b6.c) | type=i32, trips=2 | c17 | `31` |
| [`loop-invariant.i32.3.c17.9f862b8d`](loop-invariant.i32.3.c17.9f862b8d.c) | type=i32, trips=3 | c17 | `48` |
| [`loop-invariant.i32.4.c17.f87d04cb`](loop-invariant.i32.4.c17.f87d04cb.c) | type=i32, trips=4 | c17 | `66` |
| [`loop-invariant.i32.7.c17.43b3c3f0`](loop-invariant.i32.7.c17.43b3c3f0.c) | type=i32, trips=7 | c17 | `126` |
| [`loop-invariant.i32.8.c17.b41d0977`](loop-invariant.i32.8.c17.b41d0977.c) | type=i32, trips=8 | c17 | `148` |
| [`loop-invariant.i64.1.c17.6d1c5422`](loop-invariant.i64.1.c17.6d1c5422.c) | type=i64, trips=1 | c17 | `15` |
| [`loop-invariant.i64.100.c17.b52f94dd`](loop-invariant.i64.100.c17.b52f94dd.c) | type=i64, trips=100 | c17 | `6450` |
| [`loop-invariant.i64.16.c17.30615a48`](loop-invariant.i64.16.c17.30615a48.c) | type=i64, trips=16 | c17 | `360` |
| [`loop-invariant.i64.2.c17.5c95e7d4`](loop-invariant.i64.2.c17.5c95e7d4.c) | type=i64, trips=2 | c17 | `31` |
| [`loop-invariant.i64.3.c17.01e8810d`](loop-invariant.i64.3.c17.01e8810d.c) | type=i64, trips=3 | c17 | `48` |
| [`loop-invariant.i64.4.c17.b28d651b`](loop-invariant.i64.4.c17.b28d651b.c) | type=i64, trips=4 | c17 | `66` |
| [`loop-invariant.i64.7.c17.058c2072`](loop-invariant.i64.7.c17.058c2072.c) | type=i64, trips=7 | c17 | `126` |
| [`loop-invariant.i64.8.c17.a5626463`](loop-invariant.i64.8.c17.a5626463.c) | type=i64, trips=8 | c17 | `148` |
| [`loop-invariant.u32.1.c17.72cb604c`](loop-invariant.u32.1.c17.72cb604c.c) | type=u32, trips=1 | c17 | `15` |
| [`loop-invariant.u32.100.c17.bd57e57f`](loop-invariant.u32.100.c17.bd57e57f.c) | type=u32, trips=100 | c17 | `6450` |
| [`loop-invariant.u32.16.c17.00c31b71`](loop-invariant.u32.16.c17.00c31b71.c) | type=u32, trips=16 | c17 | `360` |
| [`loop-invariant.u32.2.c17.b2e5e02a`](loop-invariant.u32.2.c17.b2e5e02a.c) | type=u32, trips=2 | c17 | `31` |
| [`loop-invariant.u32.3.c17.2b894d95`](loop-invariant.u32.3.c17.2b894d95.c) | type=u32, trips=3 | c17 | `48` |
| [`loop-invariant.u32.4.c17.e72c53c3`](loop-invariant.u32.4.c17.e72c53c3.c) | type=u32, trips=4 | c17 | `66` |
| [`loop-invariant.u32.7.c17.59e53faf`](loop-invariant.u32.7.c17.59e53faf.c) | type=u32, trips=7 | c17 | `126` |
| [`loop-invariant.u32.8.c17.7da835c5`](loop-invariant.u32.8.c17.7da835c5.c) | type=u32, trips=8 | c17 | `148` |
| [`loop-invariant.u64.1.c17.c453901c`](loop-invariant.u64.1.c17.c453901c.c) | type=u64, trips=1 | c17 | `15` |
| [`loop-invariant.u64.100.c17.b40c8188`](loop-invariant.u64.100.c17.b40c8188.c) | type=u64, trips=100 | c17 | `6450` |
| [`loop-invariant.u64.16.c17.fc18d57f`](loop-invariant.u64.16.c17.fc18d57f.c) | type=u64, trips=16 | c17 | `360` |
| [`loop-invariant.u64.2.c17.ab1b4f3c`](loop-invariant.u64.2.c17.ab1b4f3c.c) | type=u64, trips=2 | c17 | `31` |
| [`loop-invariant.u64.3.c17.c3978669`](loop-invariant.u64.3.c17.c3978669.c) | type=u64, trips=3 | c17 | `48` |
| [`loop-invariant.u64.4.c17.16c3d70e`](loop-invariant.u64.4.c17.16c3d70e.c) | type=u64, trips=4 | c17 | `66` |
| [`loop-invariant.u64.7.c17.ab7ee622`](loop-invariant.u64.7.c17.ab7ee622.c) | type=u64, trips=7 | c17 | `126` |
| [`loop-invariant.u64.8.c17.c9e3c086`](loop-invariant.u64.8.c17.c9e3c086.c) | type=u64, trips=8 | c17 | `148` |

