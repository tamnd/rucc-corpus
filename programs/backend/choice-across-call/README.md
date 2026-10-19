# choice-across-call

the smaller or larger of two values, one from a call, read after more calls. Part of the backend phase of the M4 plan.

48 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`choice-across-call.double.ge.1.0.c17.f2d1dee7`](choice-across-call.double.ge.1.0.c17.f2d1dee7.c) | type=double, pick=ge, calls=1, kept=0 | c17 | `6 12 6 -2` |
| [`choice-across-call.double.ge.1.1.c17.c0f3a112`](choice-across-call.double.ge.1.1.c17.c0f3a112.c) | type=double, pick=ge, calls=1, kept=1 | c17 | `7 24 12 -4` |
| [`choice-across-call.double.ge.1.2.c17.bd59156e`](choice-across-call.double.ge.1.2.c17.bd59156e.c) | type=double, pick=ge, calls=1, kept=2 | c17 | `12 84 42 4` |
| [`choice-across-call.double.ge.2.0.c17.60a81125`](choice-across-call.double.ge.2.0.c17.60a81125.c) | type=double, pick=ge, calls=2, kept=0 | c17 | `12 24 12 -4` |
| [`choice-across-call.double.ge.2.1.c17.992a8ca9`](choice-across-call.double.ge.2.1.c17.992a8ca9.c) | type=double, pick=ge, calls=2, kept=1 | c17 | `13 36 18 -6` |
| [`choice-across-call.double.ge.2.2.c17.975dc1eb`](choice-across-call.double.ge.2.2.c17.975dc1eb.c) | type=double, pick=ge, calls=2, kept=2 | c17 | `18 96 48 2` |
| [`choice-across-call.double.lt.1.0.c17.01059226`](choice-across-call.double.lt.1.0.c17.01059226.c) | type=double, pick=lt, calls=1, kept=0 | c17 | `1 6 6 -3` |
| [`choice-across-call.double.lt.1.1.c17.95dca994`](choice-across-call.double.lt.1.1.c17.95dca994.c) | type=double, pick=lt, calls=1, kept=1 | c17 | `2 18 12 -5` |
| [`choice-across-call.double.lt.1.2.c17.6791267c`](choice-across-call.double.lt.1.2.c17.6791267c.c) | type=double, pick=lt, calls=1, kept=2 | c17 | `7 78 42 3` |
| [`choice-across-call.double.lt.2.0.c17.e3390b34`](choice-across-call.double.lt.2.0.c17.e3390b34.c) | type=double, pick=lt, calls=2, kept=0 | c17 | `2 12 12 -6` |
| [`choice-across-call.double.lt.2.1.c17.d97fb8e6`](choice-across-call.double.lt.2.1.c17.d97fb8e6.c) | type=double, pick=lt, calls=2, kept=1 | c17 | `3 24 18 -8` |
| [`choice-across-call.double.lt.2.2.c17.b5cd848e`](choice-across-call.double.lt.2.2.c17.b5cd848e.c) | type=double, pick=lt, calls=2, kept=2 | c17 | `8 84 48 0` |
| [`choice-across-call.float.ge.1.0.c17.163bce84`](choice-across-call.float.ge.1.0.c17.163bce84.c) | type=float, pick=ge, calls=1, kept=0 | c17 | `6 12 6 -2` |
| [`choice-across-call.float.ge.1.1.c17.40209e84`](choice-across-call.float.ge.1.1.c17.40209e84.c) | type=float, pick=ge, calls=1, kept=1 | c17 | `7 24 12 -4` |
| [`choice-across-call.float.ge.1.2.c17.ca846f95`](choice-across-call.float.ge.1.2.c17.ca846f95.c) | type=float, pick=ge, calls=1, kept=2 | c17 | `12 84 42 4` |
| [`choice-across-call.float.ge.2.0.c17.24e054ac`](choice-across-call.float.ge.2.0.c17.24e054ac.c) | type=float, pick=ge, calls=2, kept=0 | c17 | `12 24 12 -4` |
| [`choice-across-call.float.ge.2.1.c17.23bce0f2`](choice-across-call.float.ge.2.1.c17.23bce0f2.c) | type=float, pick=ge, calls=2, kept=1 | c17 | `13 36 18 -6` |
| [`choice-across-call.float.ge.2.2.c17.abdeddae`](choice-across-call.float.ge.2.2.c17.abdeddae.c) | type=float, pick=ge, calls=2, kept=2 | c17 | `18 96 48 2` |
| [`choice-across-call.float.lt.1.0.c17.c5c59a91`](choice-across-call.float.lt.1.0.c17.c5c59a91.c) | type=float, pick=lt, calls=1, kept=0 | c17 | `1 6 6 -3` |
| [`choice-across-call.float.lt.1.1.c17.3389f259`](choice-across-call.float.lt.1.1.c17.3389f259.c) | type=float, pick=lt, calls=1, kept=1 | c17 | `2 18 12 -5` |
| [`choice-across-call.float.lt.1.2.c17.bf1de397`](choice-across-call.float.lt.1.2.c17.bf1de397.c) | type=float, pick=lt, calls=1, kept=2 | c17 | `7 78 42 3` |
| [`choice-across-call.float.lt.2.0.c17.ecafd2f6`](choice-across-call.float.lt.2.0.c17.ecafd2f6.c) | type=float, pick=lt, calls=2, kept=0 | c17 | `2 12 12 -6` |
| [`choice-across-call.float.lt.2.1.c17.fe119e42`](choice-across-call.float.lt.2.1.c17.fe119e42.c) | type=float, pick=lt, calls=2, kept=1 | c17 | `3 24 18 -8` |
| [`choice-across-call.float.lt.2.2.c17.98b92cd7`](choice-across-call.float.lt.2.2.c17.98b92cd7.c) | type=float, pick=lt, calls=2, kept=2 | c17 | `8 84 48 0` |
| [`choice-across-call.int.ge.1.0.c17.5c4c9240`](choice-across-call.int.ge.1.0.c17.5c4c9240.c) | type=int, pick=ge, calls=1, kept=0 | c17 | `6 12 6 -2` |
| [`choice-across-call.int.ge.1.1.c17.6d6e4318`](choice-across-call.int.ge.1.1.c17.6d6e4318.c) | type=int, pick=ge, calls=1, kept=1 | c17 | `7 24 12 -4` |
| [`choice-across-call.int.ge.1.2.c17.e2fe19e9`](choice-across-call.int.ge.1.2.c17.e2fe19e9.c) | type=int, pick=ge, calls=1, kept=2 | c17 | `12 84 42 4` |
| [`choice-across-call.int.ge.2.0.c17.f15452f5`](choice-across-call.int.ge.2.0.c17.f15452f5.c) | type=int, pick=ge, calls=2, kept=0 | c17 | `12 24 12 -4` |
| [`choice-across-call.int.ge.2.1.c17.52be5715`](choice-across-call.int.ge.2.1.c17.52be5715.c) | type=int, pick=ge, calls=2, kept=1 | c17 | `13 36 18 -6` |
| [`choice-across-call.int.ge.2.2.c17.98563446`](choice-across-call.int.ge.2.2.c17.98563446.c) | type=int, pick=ge, calls=2, kept=2 | c17 | `18 96 48 2` |
| [`choice-across-call.int.lt.1.0.c17.5e2166fd`](choice-across-call.int.lt.1.0.c17.5e2166fd.c) | type=int, pick=lt, calls=1, kept=0 | c17 | `1 6 6 -3` |
| [`choice-across-call.int.lt.1.1.c17.31248647`](choice-across-call.int.lt.1.1.c17.31248647.c) | type=int, pick=lt, calls=1, kept=1 | c17 | `2 18 12 -5` |
| [`choice-across-call.int.lt.1.2.c17.31b19a07`](choice-across-call.int.lt.1.2.c17.31b19a07.c) | type=int, pick=lt, calls=1, kept=2 | c17 | `7 78 42 3` |
| [`choice-across-call.int.lt.2.0.c17.59d7b583`](choice-across-call.int.lt.2.0.c17.59d7b583.c) | type=int, pick=lt, calls=2, kept=0 | c17 | `2 12 12 -6` |
| [`choice-across-call.int.lt.2.1.c17.30d9ab98`](choice-across-call.int.lt.2.1.c17.30d9ab98.c) | type=int, pick=lt, calls=2, kept=1 | c17 | `3 24 18 -8` |
| [`choice-across-call.int.lt.2.2.c17.61b12464`](choice-across-call.int.lt.2.2.c17.61b12464.c) | type=int, pick=lt, calls=2, kept=2 | c17 | `8 84 48 0` |
| [`choice-across-call.long.ge.1.0.c17.1e7c77e9`](choice-across-call.long.ge.1.0.c17.1e7c77e9.c) | type=long, pick=ge, calls=1, kept=0 | c17 | `6 12 6 -2` |
| [`choice-across-call.long.ge.1.1.c17.29764258`](choice-across-call.long.ge.1.1.c17.29764258.c) | type=long, pick=ge, calls=1, kept=1 | c17 | `7 24 12 -4` |
| [`choice-across-call.long.ge.1.2.c17.c37b027c`](choice-across-call.long.ge.1.2.c17.c37b027c.c) | type=long, pick=ge, calls=1, kept=2 | c17 | `12 84 42 4` |
| [`choice-across-call.long.ge.2.0.c17.5643c560`](choice-across-call.long.ge.2.0.c17.5643c560.c) | type=long, pick=ge, calls=2, kept=0 | c17 | `12 24 12 -4` |
| [`choice-across-call.long.ge.2.1.c17.373dc4c1`](choice-across-call.long.ge.2.1.c17.373dc4c1.c) | type=long, pick=ge, calls=2, kept=1 | c17 | `13 36 18 -6` |
| [`choice-across-call.long.ge.2.2.c17.d05c8f8e`](choice-across-call.long.ge.2.2.c17.d05c8f8e.c) | type=long, pick=ge, calls=2, kept=2 | c17 | `18 96 48 2` |
| [`choice-across-call.long.lt.1.0.c17.ace4d1d0`](choice-across-call.long.lt.1.0.c17.ace4d1d0.c) | type=long, pick=lt, calls=1, kept=0 | c17 | `1 6 6 -3` |
| [`choice-across-call.long.lt.1.1.c17.cb7277a3`](choice-across-call.long.lt.1.1.c17.cb7277a3.c) | type=long, pick=lt, calls=1, kept=1 | c17 | `2 18 12 -5` |
| [`choice-across-call.long.lt.1.2.c17.52993b76`](choice-across-call.long.lt.1.2.c17.52993b76.c) | type=long, pick=lt, calls=1, kept=2 | c17 | `7 78 42 3` |
| [`choice-across-call.long.lt.2.0.c17.26730981`](choice-across-call.long.lt.2.0.c17.26730981.c) | type=long, pick=lt, calls=2, kept=0 | c17 | `2 12 12 -6` |
| [`choice-across-call.long.lt.2.1.c17.279ea39a`](choice-across-call.long.lt.2.1.c17.279ea39a.c) | type=long, pick=lt, calls=2, kept=1 | c17 | `3 24 18 -8` |
| [`choice-across-call.long.lt.2.2.c17.006b7553`](choice-across-call.long.lt.2.2.c17.006b7553.c) | type=long, pick=lt, calls=2, kept=2 | c17 | `8 84 48 0` |

