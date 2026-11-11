# kencc-conversions

the unsigned preserving arithmetic conversions of kencc for each pair of types. Part of the floor phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`kencc-conversions.i16.i16.kencc.c17.2a61629a`](kencc-conversions.i16.i16.kencc.c17.2a61629a.c) | `-fkencc` | left=i16, right=i16, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i16.i32.kencc.c17.37c99d11`](kencc-conversions.i16.i32.kencc.c17.37c99d11.c) | `-fkencc` | left=i16, right=i32, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i16.i64.kencc.c17.d5eee151`](kencc-conversions.i16.i64.kencc.c17.d5eee151.c) | `-fkencc` | left=i16, right=i64, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i16.i8.kencc.c17.0009fd04`](kencc-conversions.i16.i8.kencc.c17.0009fd04.c) | `-fkencc` | left=i16, right=i8, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i16.u16.kencc.c17.77f74eb5`](kencc-conversions.i16.u16.kencc.c17.77f74eb5.c) | `-fkencc` | left=i16, right=u16, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i16.u32.kencc.c17.9ff1557c`](kencc-conversions.i16.u32.kencc.c17.9ff1557c.c) | `-fkencc` | left=i16, right=u32, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i16.u64.kencc.c17.b5b96320`](kencc-conversions.i16.u64.kencc.c17.b5b96320.c) | `-fkencc` | left=i16, right=u64, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i16.u8.kencc.c17.c5be31e8`](kencc-conversions.i16.u8.kencc.c17.c5be31e8.c) | `-fkencc` | left=i16, right=u8, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i32.i16.kencc.c17.e3f5e720`](kencc-conversions.i32.i16.kencc.c17.e3f5e720.c) | `-fkencc` | left=i32, right=i16, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i32.i32.kencc.c17.4af0741e`](kencc-conversions.i32.i32.kencc.c17.4af0741e.c) | `-fkencc` | left=i32, right=i32, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i32.i64.kencc.c17.0c7154be`](kencc-conversions.i32.i64.kencc.c17.0c7154be.c) | `-fkencc` | left=i32, right=i64, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i32.i8.kencc.c17.e70acbd1`](kencc-conversions.i32.i8.kencc.c17.e70acbd1.c) | `-fkencc` | left=i32, right=i8, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i32.u16.kencc.c17.cf58556d`](kencc-conversions.i32.u16.kencc.c17.cf58556d.c) | `-fkencc` | left=i32, right=u16, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i32.u32.kencc.c17.9174cc4e`](kencc-conversions.i32.u32.kencc.c17.9174cc4e.c) | `-fkencc` | left=i32, right=u32, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i32.u64.kencc.c17.60064b2b`](kencc-conversions.i32.u64.kencc.c17.60064b2b.c) | `-fkencc` | left=i32, right=u64, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i32.u8.kencc.c17.c6aca2f1`](kencc-conversions.i32.u8.kencc.c17.c6aca2f1.c) | `-fkencc` | left=i32, right=u8, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i64.i16.kencc.c17.2a9bf774`](kencc-conversions.i64.i16.kencc.c17.2a9bf774.c) | `-fkencc` | left=i64, right=i16, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i64.i32.kencc.c17.11ed1742`](kencc-conversions.i64.i32.kencc.c17.11ed1742.c) | `-fkencc` | left=i64, right=i32, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i64.i64.kencc.c17.3523900c`](kencc-conversions.i64.i64.kencc.c17.3523900c.c) | `-fkencc` | left=i64, right=i64, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i64.i8.kencc.c17.a279ed29`](kencc-conversions.i64.i8.kencc.c17.a279ed29.c) | `-fkencc` | left=i64, right=i8, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i64.u16.kencc.c17.bd08c9a1`](kencc-conversions.i64.u16.kencc.c17.bd08c9a1.c) | `-fkencc` | left=i64, right=u16, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i64.u32.kencc.c17.052cbedb`](kencc-conversions.i64.u32.kencc.c17.052cbedb.c) | `-fkencc` | left=i64, right=u32, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i64.u64.kencc.c17.26a090d8`](kencc-conversions.i64.u64.kencc.c17.26a090d8.c) | `-fkencc` | left=i64, right=u64, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i64.u8.kencc.c17.75ff6bad`](kencc-conversions.i64.u8.kencc.c17.75ff6bad.c) | `-fkencc` | left=i64, right=u8, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i8.i16.kencc.c17.926e63aa`](kencc-conversions.i8.i16.kencc.c17.926e63aa.c) | `-fkencc` | left=i8, right=i16, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i8.i32.kencc.c17.b4782a1b`](kencc-conversions.i8.i32.kencc.c17.b4782a1b.c) | `-fkencc` | left=i8, right=i32, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i8.i64.kencc.c17.ef85204d`](kencc-conversions.i8.i64.kencc.c17.ef85204d.c) | `-fkencc` | left=i8, right=i64, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i8.i8.kencc.c17.2a85390d`](kencc-conversions.i8.i8.kencc.c17.2a85390d.c) | `-fkencc` | left=i8, right=i8, dialect=kencc | c17 | `-9 -5 14 ...` and 13 more lines |
| [`kencc-conversions.i8.u16.kencc.c17.e55f2cbf`](kencc-conversions.i8.u16.kencc.c17.e55f2cbf.c) | `-fkencc` | left=i8, right=u16, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i8.u32.kencc.c17.45f5f16a`](kencc-conversions.i8.u32.kencc.c17.45f5f16a.c) | `-fkencc` | left=i8, right=u32, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.i8.u64.kencc.c17.cf6aec5e`](kencc-conversions.i8.u64.kencc.c17.cf6aec5e.c) | `-fkencc` | left=i8, right=u64, dialect=kencc | c17 | `18446744073709551612 18446744073709551606 18446744073709551595 ...` and 13 more lines |
| [`kencc-conversions.i8.u8.kencc.c17.07a68058`](kencc-conversions.i8.u8.kencc.c17.07a68058.c) | `-fkencc` | left=i8, right=u8, dialect=kencc | c17 | `4294967292 4294967286 4294967275 ...` and 13 more lines |
| [`kencc-conversions.u16.i16.kencc.c17.a54a1b05`](kencc-conversions.u16.i16.kencc.c17.a54a1b05.c) | `-fkencc` | left=u16, right=i16, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u16.i32.kencc.c17.24cbf295`](kencc-conversions.u16.i32.kencc.c17.24cbf295.c) | `-fkencc` | left=u16, right=i32, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u16.i64.kencc.c17.12b4bd94`](kencc-conversions.u16.i64.kencc.c17.12b4bd94.c) | `-fkencc` | left=u16, right=i64, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u16.i8.kencc.c17.7c5f71d0`](kencc-conversions.u16.i8.kencc.c17.7c5f71d0.c) | `-fkencc` | left=u16, right=i8, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u16.u16.kencc.c17.d8f417a8`](kencc-conversions.u16.u16.kencc.c17.d8f417a8.c) | `-fkencc` | left=u16, right=u16, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u16.u32.kencc.c17.5d4bfcdd`](kencc-conversions.u16.u32.kencc.c17.5d4bfcdd.c) | `-fkencc` | left=u16, right=u32, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u16.u64.kencc.c17.c0df932b`](kencc-conversions.u16.u64.kencc.c17.c0df932b.c) | `-fkencc` | left=u16, right=u64, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u16.u8.kencc.c17.2a8c59ae`](kencc-conversions.u16.u8.kencc.c17.2a8c59ae.c) | `-fkencc` | left=u16, right=u8, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u32.i16.kencc.c17.17b802e9`](kencc-conversions.u32.i16.kencc.c17.17b802e9.c) | `-fkencc` | left=u32, right=i16, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u32.i32.kencc.c17.22c4e1c5`](kencc-conversions.u32.i32.kencc.c17.22c4e1c5.c) | `-fkencc` | left=u32, right=i32, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u32.i64.kencc.c17.61b24988`](kencc-conversions.u32.i64.kencc.c17.61b24988.c) | `-fkencc` | left=u32, right=i64, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u32.i8.kencc.c17.c5a85e43`](kencc-conversions.u32.i8.kencc.c17.c5a85e43.c) | `-fkencc` | left=u32, right=i8, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u32.u16.kencc.c17.779dc539`](kencc-conversions.u32.u16.kencc.c17.779dc539.c) | `-fkencc` | left=u32, right=u16, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u32.u32.kencc.c17.d4a8860c`](kencc-conversions.u32.u32.kencc.c17.d4a8860c.c) | `-fkencc` | left=u32, right=u32, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u32.u64.kencc.c17.ab90663d`](kencc-conversions.u32.u64.kencc.c17.ab90663d.c) | `-fkencc` | left=u32, right=u64, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u32.u8.kencc.c17.c4798822`](kencc-conversions.u32.u8.kencc.c17.c4798822.c) | `-fkencc` | left=u32, right=u8, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u64.i16.kencc.c17.2436f282`](kencc-conversions.u64.i16.kencc.c17.2436f282.c) | `-fkencc` | left=u64, right=i16, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u64.i32.kencc.c17.b5a3ddc4`](kencc-conversions.u64.i32.kencc.c17.b5a3ddc4.c) | `-fkencc` | left=u64, right=i32, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u64.i64.kencc.c17.3fe43961`](kencc-conversions.u64.i64.kencc.c17.3fe43961.c) | `-fkencc` | left=u64, right=i64, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u64.i8.kencc.c17.8324a9e9`](kencc-conversions.u64.i8.kencc.c17.8324a9e9.c) | `-fkencc` | left=u64, right=i8, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u64.u16.kencc.c17.f0f2d56e`](kencc-conversions.u64.u16.kencc.c17.f0f2d56e.c) | `-fkencc` | left=u64, right=u16, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u64.u32.kencc.c17.3734e1a5`](kencc-conversions.u64.u32.kencc.c17.3734e1a5.c) | `-fkencc` | left=u64, right=u32, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u64.u64.kencc.c17.b53e294f`](kencc-conversions.u64.u64.kencc.c17.b53e294f.c) | `-fkencc` | left=u64, right=u64, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u64.u8.kencc.c17.2ca9d369`](kencc-conversions.u64.u8.kencc.c17.2ca9d369.c) | `-fkencc` | left=u64, right=u8, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u8.i16.kencc.c17.d4161fd4`](kencc-conversions.u8.i16.kencc.c17.d4161fd4.c) | `-fkencc` | left=u8, right=i16, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u8.i32.kencc.c17.87c87c19`](kencc-conversions.u8.i32.kencc.c17.87c87c19.c) | `-fkencc` | left=u8, right=i32, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u8.i64.kencc.c17.fde99e7e`](kencc-conversions.u8.i64.kencc.c17.fde99e7e.c) | `-fkencc` | left=u8, right=i64, dialect=kencc | c17 | `3 7 18446744073709551606 ...` and 13 more lines |
| [`kencc-conversions.u8.i8.kencc.c17.afdd102b`](kencc-conversions.u8.i8.kencc.c17.afdd102b.c) | `-fkencc` | left=u8, right=i8, dialect=kencc | c17 | `3 7 4294967286 ...` and 13 more lines |
| [`kencc-conversions.u8.u16.kencc.c17.a89ea869`](kencc-conversions.u8.u16.kencc.c17.a89ea869.c) | `-fkencc` | left=u8, right=u16, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u8.u32.kencc.c17.5f845979`](kencc-conversions.u8.u32.kencc.c17.5f845979.c) | `-fkencc` | left=u8, right=u32, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u8.u64.kencc.c17.d782d0a3`](kencc-conversions.u8.u64.kencc.c17.d782d0a3.c) | `-fkencc` | left=u8, right=u64, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |
| [`kencc-conversions.u8.u8.kencc.c17.a880f0d5`](kencc-conversions.u8.u8.kencc.c17.a880f0d5.c) | `-fkencc` | left=u8, right=u8, dialect=kencc | c17 | `8 2 15 ...` and 13 more lines |

