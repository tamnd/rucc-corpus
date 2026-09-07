# loop-unroll

unrolling a loop body, known trip count or not. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-unroll.i32.1.known.c17.ad11c666`](loop-unroll.i32.1.known.c17.ad11c666.c) | type=i32, trips=1, bound=known | c17 | `1` |
| [`loop-unroll.i32.1.unknown.c17.2e11099c`](loop-unroll.i32.1.unknown.c17.2e11099c.c) | type=i32, trips=1, bound=unknown | c17 | `1` |
| [`loop-unroll.i32.100.known.c17.d0c619e9`](loop-unroll.i32.100.known.c17.d0c619e9.c) | type=i32, trips=100, bound=known | c17 | `14950` |
| [`loop-unroll.i32.100.unknown.c17.b39b0c10`](loop-unroll.i32.100.unknown.c17.b39b0c10.c) | type=i32, trips=100, bound=unknown | c17 | `14950` |
| [`loop-unroll.i32.16.known.c17.3a299dd0`](loop-unroll.i32.16.known.c17.3a299dd0.c) | type=i32, trips=16, bound=known | c17 | `376` |
| [`loop-unroll.i32.16.unknown.c17.681f698a`](loop-unroll.i32.16.unknown.c17.681f698a.c) | type=i32, trips=16, bound=unknown | c17 | `376` |
| [`loop-unroll.i32.2.known.c17.9163a5aa`](loop-unroll.i32.2.known.c17.9163a5aa.c) | type=i32, trips=2, bound=known | c17 | `5` |
| [`loop-unroll.i32.2.unknown.c17.f318381a`](loop-unroll.i32.2.unknown.c17.f318381a.c) | type=i32, trips=2, bound=unknown | c17 | `5` |
| [`loop-unroll.i32.3.known.c17.0408e1d2`](loop-unroll.i32.3.known.c17.0408e1d2.c) | type=i32, trips=3, bound=known | c17 | `12` |
| [`loop-unroll.i32.3.unknown.c17.2dde946e`](loop-unroll.i32.3.unknown.c17.2dde946e.c) | type=i32, trips=3, bound=unknown | c17 | `12` |
| [`loop-unroll.i32.4.known.c17.a1097e48`](loop-unroll.i32.4.known.c17.a1097e48.c) | type=i32, trips=4, bound=known | c17 | `22` |
| [`loop-unroll.i32.4.unknown.c17.b064f05b`](loop-unroll.i32.4.unknown.c17.b064f05b.c) | type=i32, trips=4, bound=unknown | c17 | `22` |
| [`loop-unroll.i32.7.known.c17.708339d9`](loop-unroll.i32.7.known.c17.708339d9.c) | type=i32, trips=7, bound=known | c17 | `70` |
| [`loop-unroll.i32.7.unknown.c17.bcc9d8bd`](loop-unroll.i32.7.unknown.c17.bcc9d8bd.c) | type=i32, trips=7, bound=unknown | c17 | `70` |
| [`loop-unroll.i32.8.known.c17.bb5429b2`](loop-unroll.i32.8.known.c17.bb5429b2.c) | type=i32, trips=8, bound=known | c17 | `92` |
| [`loop-unroll.i32.8.unknown.c17.675ec89d`](loop-unroll.i32.8.unknown.c17.675ec89d.c) | type=i32, trips=8, bound=unknown | c17 | `92` |
| [`loop-unroll.i64.1.known.c17.110551b5`](loop-unroll.i64.1.known.c17.110551b5.c) | type=i64, trips=1, bound=known | c17 | `1` |
| [`loop-unroll.i64.1.unknown.c17.f9d399f6`](loop-unroll.i64.1.unknown.c17.f9d399f6.c) | type=i64, trips=1, bound=unknown | c17 | `1` |
| [`loop-unroll.i64.100.known.c17.a7e59913`](loop-unroll.i64.100.known.c17.a7e59913.c) | type=i64, trips=100, bound=known | c17 | `14950` |
| [`loop-unroll.i64.100.unknown.c17.094f6e71`](loop-unroll.i64.100.unknown.c17.094f6e71.c) | type=i64, trips=100, bound=unknown | c17 | `14950` |
| [`loop-unroll.i64.16.known.c17.c31739c6`](loop-unroll.i64.16.known.c17.c31739c6.c) | type=i64, trips=16, bound=known | c17 | `376` |
| [`loop-unroll.i64.16.unknown.c17.deae2799`](loop-unroll.i64.16.unknown.c17.deae2799.c) | type=i64, trips=16, bound=unknown | c17 | `376` |
| [`loop-unroll.i64.2.known.c17.1cdf5855`](loop-unroll.i64.2.known.c17.1cdf5855.c) | type=i64, trips=2, bound=known | c17 | `5` |
| [`loop-unroll.i64.2.unknown.c17.5a004789`](loop-unroll.i64.2.unknown.c17.5a004789.c) | type=i64, trips=2, bound=unknown | c17 | `5` |
| [`loop-unroll.i64.3.known.c17.e30f7254`](loop-unroll.i64.3.known.c17.e30f7254.c) | type=i64, trips=3, bound=known | c17 | `12` |
| [`loop-unroll.i64.3.unknown.c17.c5a267ca`](loop-unroll.i64.3.unknown.c17.c5a267ca.c) | type=i64, trips=3, bound=unknown | c17 | `12` |
| [`loop-unroll.i64.4.known.c17.22d5a427`](loop-unroll.i64.4.known.c17.22d5a427.c) | type=i64, trips=4, bound=known | c17 | `22` |
| [`loop-unroll.i64.4.unknown.c17.5ab0bbd4`](loop-unroll.i64.4.unknown.c17.5ab0bbd4.c) | type=i64, trips=4, bound=unknown | c17 | `22` |
| [`loop-unroll.i64.7.known.c17.38f119c8`](loop-unroll.i64.7.known.c17.38f119c8.c) | type=i64, trips=7, bound=known | c17 | `70` |
| [`loop-unroll.i64.7.unknown.c17.9b7b3485`](loop-unroll.i64.7.unknown.c17.9b7b3485.c) | type=i64, trips=7, bound=unknown | c17 | `70` |
| [`loop-unroll.i64.8.known.c17.7d2aa1ba`](loop-unroll.i64.8.known.c17.7d2aa1ba.c) | type=i64, trips=8, bound=known | c17 | `92` |
| [`loop-unroll.i64.8.unknown.c17.9dcf01a1`](loop-unroll.i64.8.unknown.c17.9dcf01a1.c) | type=i64, trips=8, bound=unknown | c17 | `92` |
| [`loop-unroll.u32.1.known.c17.b03a7a70`](loop-unroll.u32.1.known.c17.b03a7a70.c) | type=u32, trips=1, bound=known | c17 | `1` |
| [`loop-unroll.u32.1.unknown.c17.c726c97d`](loop-unroll.u32.1.unknown.c17.c726c97d.c) | type=u32, trips=1, bound=unknown | c17 | `1` |
| [`loop-unroll.u32.100.known.c17.c0a7a1cd`](loop-unroll.u32.100.known.c17.c0a7a1cd.c) | type=u32, trips=100, bound=known | c17 | `14950` |
| [`loop-unroll.u32.100.unknown.c17.063208ff`](loop-unroll.u32.100.unknown.c17.063208ff.c) | type=u32, trips=100, bound=unknown | c17 | `14950` |
| [`loop-unroll.u32.16.known.c17.83a27490`](loop-unroll.u32.16.known.c17.83a27490.c) | type=u32, trips=16, bound=known | c17 | `376` |
| [`loop-unroll.u32.16.unknown.c17.30f45815`](loop-unroll.u32.16.unknown.c17.30f45815.c) | type=u32, trips=16, bound=unknown | c17 | `376` |
| [`loop-unroll.u32.2.known.c17.0f04d7ae`](loop-unroll.u32.2.known.c17.0f04d7ae.c) | type=u32, trips=2, bound=known | c17 | `5` |
| [`loop-unroll.u32.2.unknown.c17.fb052323`](loop-unroll.u32.2.unknown.c17.fb052323.c) | type=u32, trips=2, bound=unknown | c17 | `5` |
| [`loop-unroll.u32.3.known.c17.ec52cd7c`](loop-unroll.u32.3.known.c17.ec52cd7c.c) | type=u32, trips=3, bound=known | c17 | `12` |
| [`loop-unroll.u32.3.unknown.c17.8e78d20e`](loop-unroll.u32.3.unknown.c17.8e78d20e.c) | type=u32, trips=3, bound=unknown | c17 | `12` |
| [`loop-unroll.u32.4.known.c17.e8ca4e41`](loop-unroll.u32.4.known.c17.e8ca4e41.c) | type=u32, trips=4, bound=known | c17 | `22` |
| [`loop-unroll.u32.4.unknown.c17.09bd85f6`](loop-unroll.u32.4.unknown.c17.09bd85f6.c) | type=u32, trips=4, bound=unknown | c17 | `22` |
| [`loop-unroll.u32.7.known.c17.c6cab6b5`](loop-unroll.u32.7.known.c17.c6cab6b5.c) | type=u32, trips=7, bound=known | c17 | `70` |
| [`loop-unroll.u32.7.unknown.c17.63087fbe`](loop-unroll.u32.7.unknown.c17.63087fbe.c) | type=u32, trips=7, bound=unknown | c17 | `70` |
| [`loop-unroll.u32.8.known.c17.e472696a`](loop-unroll.u32.8.known.c17.e472696a.c) | type=u32, trips=8, bound=known | c17 | `92` |
| [`loop-unroll.u32.8.unknown.c17.8305e0fb`](loop-unroll.u32.8.unknown.c17.8305e0fb.c) | type=u32, trips=8, bound=unknown | c17 | `92` |
| [`loop-unroll.u64.1.known.c17.8232db25`](loop-unroll.u64.1.known.c17.8232db25.c) | type=u64, trips=1, bound=known | c17 | `1` |
| [`loop-unroll.u64.1.unknown.c17.10af952a`](loop-unroll.u64.1.unknown.c17.10af952a.c) | type=u64, trips=1, bound=unknown | c17 | `1` |
| [`loop-unroll.u64.100.known.c17.bca5ed98`](loop-unroll.u64.100.known.c17.bca5ed98.c) | type=u64, trips=100, bound=known | c17 | `14950` |
| [`loop-unroll.u64.100.unknown.c17.87e80cab`](loop-unroll.u64.100.unknown.c17.87e80cab.c) | type=u64, trips=100, bound=unknown | c17 | `14950` |
| [`loop-unroll.u64.16.known.c17.73dbc8f2`](loop-unroll.u64.16.known.c17.73dbc8f2.c) | type=u64, trips=16, bound=known | c17 | `376` |
| [`loop-unroll.u64.16.unknown.c17.9363930d`](loop-unroll.u64.16.unknown.c17.9363930d.c) | type=u64, trips=16, bound=unknown | c17 | `376` |
| [`loop-unroll.u64.2.known.c17.0576d0aa`](loop-unroll.u64.2.known.c17.0576d0aa.c) | type=u64, trips=2, bound=known | c17 | `5` |
| [`loop-unroll.u64.2.unknown.c17.4912f27f`](loop-unroll.u64.2.unknown.c17.4912f27f.c) | type=u64, trips=2, bound=unknown | c17 | `5` |
| [`loop-unroll.u64.3.known.c17.395c3e44`](loop-unroll.u64.3.known.c17.395c3e44.c) | type=u64, trips=3, bound=known | c17 | `12` |
| [`loop-unroll.u64.3.unknown.c17.6fad2d1f`](loop-unroll.u64.3.unknown.c17.6fad2d1f.c) | type=u64, trips=3, bound=unknown | c17 | `12` |
| [`loop-unroll.u64.4.known.c17.ecb7ca8d`](loop-unroll.u64.4.known.c17.ecb7ca8d.c) | type=u64, trips=4, bound=known | c17 | `22` |
| [`loop-unroll.u64.4.unknown.c17.5ec8766a`](loop-unroll.u64.4.unknown.c17.5ec8766a.c) | type=u64, trips=4, bound=unknown | c17 | `22` |
| [`loop-unroll.u64.7.known.c17.93ded352`](loop-unroll.u64.7.known.c17.93ded352.c) | type=u64, trips=7, bound=known | c17 | `70` |
| [`loop-unroll.u64.7.unknown.c17.9e2b45e4`](loop-unroll.u64.7.unknown.c17.9e2b45e4.c) | type=u64, trips=7, bound=unknown | c17 | `70` |
| [`loop-unroll.u64.8.known.c17.a16f5f35`](loop-unroll.u64.8.known.c17.a16f5f35.c) | type=u64, trips=8, bound=known | c17 | `92` |
| [`loop-unroll.u64.8.unknown.c17.daf6a16a`](loop-unroll.u64.8.unknown.c17.daf6a16a.c) | type=u64, trips=8, bound=unknown | c17 | `92` |

