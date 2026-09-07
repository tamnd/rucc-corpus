# reassociate

reassociating a chain to shorten its dependency height. Part of the local phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`reassociate.i32.add.c17.31f011dc`](reassociate.i32.add.c17.31f011dc.c) | type=i32, op=add | c17 | `11 11 11 11` |
| [`reassociate.i32.and.c17.274faff7`](reassociate.i32.and.c17.274faff7.c) | type=i32, op=and | c17 | `0 0 0 0` |
| [`reassociate.i32.mul.c17.611c7d80`](reassociate.i32.mul.c17.611c7d80.c) | type=i32, op=mul | c17 | `30 30 30 30` |
| [`reassociate.i32.or.c17.1b46a9f0`](reassociate.i32.or.c17.1b46a9f0.c) | type=i32, op=or | c17 | `255 255 255 255` |
| [`reassociate.i32.xor.c17.9775d414`](reassociate.i32.xor.c17.9775d414.c) | type=i32, op=xor | c17 | `195 195 195 195` |
| [`reassociate.i64.add.c17.3dd61c59`](reassociate.i64.add.c17.3dd61c59.c) | type=i64, op=add | c17 | `11 11 11 11` |
| [`reassociate.i64.and.c17.6d479c84`](reassociate.i64.and.c17.6d479c84.c) | type=i64, op=and | c17 | `0 0 0 0` |
| [`reassociate.i64.mul.c17.9e2dc697`](reassociate.i64.mul.c17.9e2dc697.c) | type=i64, op=mul | c17 | `30 30 30 30` |
| [`reassociate.i64.or.c17.d8050763`](reassociate.i64.or.c17.d8050763.c) | type=i64, op=or | c17 | `255 255 255 255` |
| [`reassociate.i64.xor.c17.5854816b`](reassociate.i64.xor.c17.5854816b.c) | type=i64, op=xor | c17 | `195 195 195 195` |
| [`reassociate.u32.add.c17.93fc0519`](reassociate.u32.add.c17.93fc0519.c) | type=u32, op=add | c17 | `11 11 11 11` |
| [`reassociate.u32.and.c17.1f1b6b7f`](reassociate.u32.and.c17.1f1b6b7f.c) | type=u32, op=and | c17 | `0 0 0 0` |
| [`reassociate.u32.mul.c17.3721a532`](reassociate.u32.mul.c17.3721a532.c) | type=u32, op=mul | c17 | `30 30 30 30` |
| [`reassociate.u32.or.c17.14309a2e`](reassociate.u32.or.c17.14309a2e.c) | type=u32, op=or | c17 | `255 255 255 255` |
| [`reassociate.u32.xor.c17.31620b80`](reassociate.u32.xor.c17.31620b80.c) | type=u32, op=xor | c17 | `195 195 195 195` |
| [`reassociate.u64.add.c17.bace659d`](reassociate.u64.add.c17.bace659d.c) | type=u64, op=add | c17 | `11 11 11 11` |
| [`reassociate.u64.and.c17.7761a76d`](reassociate.u64.and.c17.7761a76d.c) | type=u64, op=and | c17 | `0 0 0 0` |
| [`reassociate.u64.mul.c17.787f7f4d`](reassociate.u64.mul.c17.787f7f4d.c) | type=u64, op=mul | c17 | `30 30 30 30` |
| [`reassociate.u64.or.c17.ffde2efe`](reassociate.u64.or.c17.ffde2efe.c) | type=u64, op=or | c17 | `255 255 255 255` |
| [`reassociate.u64.xor.c17.4ca878fa`](reassociate.u64.xor.c17.4ca878fa.c) | type=u64, op=xor | c17 | `195 195 195 195` |

