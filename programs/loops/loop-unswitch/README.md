# loop-unswitch

removing a test the loop guard already decided. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-unswitch.i32.1.0.c17.30d4015b`](loop-unswitch.i32.1.0.c17.30d4015b.c) | type=i32, trips=1, flag=0 | c17 | `1` |
| [`loop-unswitch.i32.1.1.c17.0c45c187`](loop-unswitch.i32.1.1.c17.0c45c187.c) | type=i32, trips=1, flag=1 | c17 | `0` |
| [`loop-unswitch.i32.100.0.c17.fffb077f`](loop-unswitch.i32.100.0.c17.fffb077f.c) | type=i32, trips=100, flag=0 | c17 | `5050` |
| [`loop-unswitch.i32.100.1.c17.56b7b3fa`](loop-unswitch.i32.100.1.c17.56b7b3fa.c) | type=i32, trips=100, flag=1 | c17 | `9900` |
| [`loop-unswitch.i32.16.0.c17.bd55cb29`](loop-unswitch.i32.16.0.c17.bd55cb29.c) | type=i32, trips=16, flag=0 | c17 | `136` |
| [`loop-unswitch.i32.16.1.c17.0976100e`](loop-unswitch.i32.16.1.c17.0976100e.c) | type=i32, trips=16, flag=1 | c17 | `240` |
| [`loop-unswitch.i32.2.0.c17.54075f17`](loop-unswitch.i32.2.0.c17.54075f17.c) | type=i32, trips=2, flag=0 | c17 | `3` |
| [`loop-unswitch.i32.2.1.c17.fa1ae24f`](loop-unswitch.i32.2.1.c17.fa1ae24f.c) | type=i32, trips=2, flag=1 | c17 | `2` |
| [`loop-unswitch.i32.3.0.c17.351c6dc0`](loop-unswitch.i32.3.0.c17.351c6dc0.c) | type=i32, trips=3, flag=0 | c17 | `6` |
| [`loop-unswitch.i32.3.1.c17.d0dc676c`](loop-unswitch.i32.3.1.c17.d0dc676c.c) | type=i32, trips=3, flag=1 | c17 | `6` |
| [`loop-unswitch.i32.4.0.c17.b46797da`](loop-unswitch.i32.4.0.c17.b46797da.c) | type=i32, trips=4, flag=0 | c17 | `10` |
| [`loop-unswitch.i32.4.1.c17.0201031a`](loop-unswitch.i32.4.1.c17.0201031a.c) | type=i32, trips=4, flag=1 | c17 | `12` |
| [`loop-unswitch.i32.7.0.c17.6320c618`](loop-unswitch.i32.7.0.c17.6320c618.c) | type=i32, trips=7, flag=0 | c17 | `28` |
| [`loop-unswitch.i32.7.1.c17.19cb48dd`](loop-unswitch.i32.7.1.c17.19cb48dd.c) | type=i32, trips=7, flag=1 | c17 | `42` |
| [`loop-unswitch.i32.8.0.c17.e4129a1b`](loop-unswitch.i32.8.0.c17.e4129a1b.c) | type=i32, trips=8, flag=0 | c17 | `36` |
| [`loop-unswitch.i32.8.1.c17.640e1e7c`](loop-unswitch.i32.8.1.c17.640e1e7c.c) | type=i32, trips=8, flag=1 | c17 | `56` |
| [`loop-unswitch.i64.1.0.c17.ade82dec`](loop-unswitch.i64.1.0.c17.ade82dec.c) | type=i64, trips=1, flag=0 | c17 | `1` |
| [`loop-unswitch.i64.1.1.c17.435919aa`](loop-unswitch.i64.1.1.c17.435919aa.c) | type=i64, trips=1, flag=1 | c17 | `0` |
| [`loop-unswitch.i64.100.0.c17.200e770d`](loop-unswitch.i64.100.0.c17.200e770d.c) | type=i64, trips=100, flag=0 | c17 | `5050` |
| [`loop-unswitch.i64.100.1.c17.b430f8b0`](loop-unswitch.i64.100.1.c17.b430f8b0.c) | type=i64, trips=100, flag=1 | c17 | `9900` |
| [`loop-unswitch.i64.16.0.c17.0cbc7de0`](loop-unswitch.i64.16.0.c17.0cbc7de0.c) | type=i64, trips=16, flag=0 | c17 | `136` |
| [`loop-unswitch.i64.16.1.c17.bbd32216`](loop-unswitch.i64.16.1.c17.bbd32216.c) | type=i64, trips=16, flag=1 | c17 | `240` |
| [`loop-unswitch.i64.2.0.c17.bb8d5cc0`](loop-unswitch.i64.2.0.c17.bb8d5cc0.c) | type=i64, trips=2, flag=0 | c17 | `3` |
| [`loop-unswitch.i64.2.1.c17.6fa56d63`](loop-unswitch.i64.2.1.c17.6fa56d63.c) | type=i64, trips=2, flag=1 | c17 | `2` |
| [`loop-unswitch.i64.3.0.c17.d1e555f0`](loop-unswitch.i64.3.0.c17.d1e555f0.c) | type=i64, trips=3, flag=0 | c17 | `6` |
| [`loop-unswitch.i64.3.1.c17.70d04f08`](loop-unswitch.i64.3.1.c17.70d04f08.c) | type=i64, trips=3, flag=1 | c17 | `6` |
| [`loop-unswitch.i64.4.0.c17.afc5804c`](loop-unswitch.i64.4.0.c17.afc5804c.c) | type=i64, trips=4, flag=0 | c17 | `10` |
| [`loop-unswitch.i64.4.1.c17.32015eda`](loop-unswitch.i64.4.1.c17.32015eda.c) | type=i64, trips=4, flag=1 | c17 | `12` |
| [`loop-unswitch.i64.7.0.c17.cadb23c7`](loop-unswitch.i64.7.0.c17.cadb23c7.c) | type=i64, trips=7, flag=0 | c17 | `28` |
| [`loop-unswitch.i64.7.1.c17.efd13df4`](loop-unswitch.i64.7.1.c17.efd13df4.c) | type=i64, trips=7, flag=1 | c17 | `42` |
| [`loop-unswitch.i64.8.0.c17.c562753f`](loop-unswitch.i64.8.0.c17.c562753f.c) | type=i64, trips=8, flag=0 | c17 | `36` |
| [`loop-unswitch.i64.8.1.c17.ed9db24e`](loop-unswitch.i64.8.1.c17.ed9db24e.c) | type=i64, trips=8, flag=1 | c17 | `56` |
| [`loop-unswitch.u32.1.0.c17.bc1e17f6`](loop-unswitch.u32.1.0.c17.bc1e17f6.c) | type=u32, trips=1, flag=0 | c17 | `1` |
| [`loop-unswitch.u32.1.1.c17.5f6ba6e7`](loop-unswitch.u32.1.1.c17.5f6ba6e7.c) | type=u32, trips=1, flag=1 | c17 | `0` |
| [`loop-unswitch.u32.100.0.c17.22966542`](loop-unswitch.u32.100.0.c17.22966542.c) | type=u32, trips=100, flag=0 | c17 | `5050` |
| [`loop-unswitch.u32.100.1.c17.d7ae5d0b`](loop-unswitch.u32.100.1.c17.d7ae5d0b.c) | type=u32, trips=100, flag=1 | c17 | `9900` |
| [`loop-unswitch.u32.16.0.c17.a4a74ca2`](loop-unswitch.u32.16.0.c17.a4a74ca2.c) | type=u32, trips=16, flag=0 | c17 | `136` |
| [`loop-unswitch.u32.16.1.c17.eeaf02b1`](loop-unswitch.u32.16.1.c17.eeaf02b1.c) | type=u32, trips=16, flag=1 | c17 | `240` |
| [`loop-unswitch.u32.2.0.c17.92fb2ad7`](loop-unswitch.u32.2.0.c17.92fb2ad7.c) | type=u32, trips=2, flag=0 | c17 | `3` |
| [`loop-unswitch.u32.2.1.c17.bec187ef`](loop-unswitch.u32.2.1.c17.bec187ef.c) | type=u32, trips=2, flag=1 | c17 | `2` |
| [`loop-unswitch.u32.3.0.c17.2664402f`](loop-unswitch.u32.3.0.c17.2664402f.c) | type=u32, trips=3, flag=0 | c17 | `6` |
| [`loop-unswitch.u32.3.1.c17.3110ecf3`](loop-unswitch.u32.3.1.c17.3110ecf3.c) | type=u32, trips=3, flag=1 | c17 | `6` |
| [`loop-unswitch.u32.4.0.c17.c4f18617`](loop-unswitch.u32.4.0.c17.c4f18617.c) | type=u32, trips=4, flag=0 | c17 | `10` |
| [`loop-unswitch.u32.4.1.c17.6ed0970d`](loop-unswitch.u32.4.1.c17.6ed0970d.c) | type=u32, trips=4, flag=1 | c17 | `12` |
| [`loop-unswitch.u32.7.0.c17.cbfd3de8`](loop-unswitch.u32.7.0.c17.cbfd3de8.c) | type=u32, trips=7, flag=0 | c17 | `28` |
| [`loop-unswitch.u32.7.1.c17.de7f6a9c`](loop-unswitch.u32.7.1.c17.de7f6a9c.c) | type=u32, trips=7, flag=1 | c17 | `42` |
| [`loop-unswitch.u32.8.0.c17.35c815dc`](loop-unswitch.u32.8.0.c17.35c815dc.c) | type=u32, trips=8, flag=0 | c17 | `36` |
| [`loop-unswitch.u32.8.1.c17.06a29aec`](loop-unswitch.u32.8.1.c17.06a29aec.c) | type=u32, trips=8, flag=1 | c17 | `56` |
| [`loop-unswitch.u64.1.0.c17.195e26cf`](loop-unswitch.u64.1.0.c17.195e26cf.c) | type=u64, trips=1, flag=0 | c17 | `1` |
| [`loop-unswitch.u64.1.1.c17.3b1de547`](loop-unswitch.u64.1.1.c17.3b1de547.c) | type=u64, trips=1, flag=1 | c17 | `0` |
| [`loop-unswitch.u64.100.0.c17.ea290872`](loop-unswitch.u64.100.0.c17.ea290872.c) | type=u64, trips=100, flag=0 | c17 | `5050` |
| [`loop-unswitch.u64.100.1.c17.7d9c19e1`](loop-unswitch.u64.100.1.c17.7d9c19e1.c) | type=u64, trips=100, flag=1 | c17 | `9900` |
| [`loop-unswitch.u64.16.0.c17.507aeaf0`](loop-unswitch.u64.16.0.c17.507aeaf0.c) | type=u64, trips=16, flag=0 | c17 | `136` |
| [`loop-unswitch.u64.16.1.c17.fd712699`](loop-unswitch.u64.16.1.c17.fd712699.c) | type=u64, trips=16, flag=1 | c17 | `240` |
| [`loop-unswitch.u64.2.0.c17.2650aca4`](loop-unswitch.u64.2.0.c17.2650aca4.c) | type=u64, trips=2, flag=0 | c17 | `3` |
| [`loop-unswitch.u64.2.1.c17.a7daeda3`](loop-unswitch.u64.2.1.c17.a7daeda3.c) | type=u64, trips=2, flag=1 | c17 | `2` |
| [`loop-unswitch.u64.3.0.c17.ea3c7fdd`](loop-unswitch.u64.3.0.c17.ea3c7fdd.c) | type=u64, trips=3, flag=0 | c17 | `6` |
| [`loop-unswitch.u64.3.1.c17.e971a46e`](loop-unswitch.u64.3.1.c17.e971a46e.c) | type=u64, trips=3, flag=1 | c17 | `6` |
| [`loop-unswitch.u64.4.0.c17.229ffa23`](loop-unswitch.u64.4.0.c17.229ffa23.c) | type=u64, trips=4, flag=0 | c17 | `10` |
| [`loop-unswitch.u64.4.1.c17.752ada4e`](loop-unswitch.u64.4.1.c17.752ada4e.c) | type=u64, trips=4, flag=1 | c17 | `12` |
| [`loop-unswitch.u64.7.0.c17.4de101f2`](loop-unswitch.u64.7.0.c17.4de101f2.c) | type=u64, trips=7, flag=0 | c17 | `28` |
| [`loop-unswitch.u64.7.1.c17.e8f2ca40`](loop-unswitch.u64.7.1.c17.e8f2ca40.c) | type=u64, trips=7, flag=1 | c17 | `42` |
| [`loop-unswitch.u64.8.0.c17.11d9bf54`](loop-unswitch.u64.8.0.c17.11d9bf54.c) | type=u64, trips=8, flag=0 | c17 | `36` |
| [`loop-unswitch.u64.8.1.c17.3ce109ca`](loop-unswitch.u64.8.1.c17.3ce109ca.c) | type=u64, trips=8, flag=1 | c17 | `56` |

