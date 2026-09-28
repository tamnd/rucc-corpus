# crc32c

CRC-32C through the SSE4.2 instructions against slicing by eight. Part of the correctness phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`crc32c.aligned.0.pieces.builtin.c17.74a53861`](crc32c.aligned.0.pieces.builtin.c17.74a53861.c) | step=aligned, offset=0, calls=pieces, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.aligned.0.pieces.cpuid.c17.d6c23cf3`](crc32c.aligned.0.pieces.cpuid.c17.d6c23cf3.c) | step=aligned, offset=0, calls=pieces, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.aligned.0.whole.builtin.c17.b86b85b8`](crc32c.aligned.0.whole.builtin.c17.b86b85b8.c) | step=aligned, offset=0, calls=whole, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.aligned.0.whole.cpuid.c17.63ddb811`](crc32c.aligned.0.whole.cpuid.c17.63ddb811.c) | step=aligned, offset=0, calls=whole, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.aligned.1.pieces.builtin.c17.cd247f37`](crc32c.aligned.1.pieces.builtin.c17.cd247f37.c) | step=aligned, offset=1, calls=pieces, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.aligned.1.pieces.cpuid.c17.e6f1bf8b`](crc32c.aligned.1.pieces.cpuid.c17.e6f1bf8b.c) | step=aligned, offset=1, calls=pieces, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.aligned.1.whole.builtin.c17.7f9dff1b`](crc32c.aligned.1.whole.builtin.c17.7f9dff1b.c) | step=aligned, offset=1, calls=whole, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.aligned.1.whole.cpuid.c17.9f87a391`](crc32c.aligned.1.whole.cpuid.c17.9f87a391.c) | step=aligned, offset=1, calls=whole, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.aligned.3.pieces.builtin.c17.2d516ebb`](crc32c.aligned.3.pieces.builtin.c17.2d516ebb.c) | step=aligned, offset=3, calls=pieces, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.aligned.3.pieces.cpuid.c17.b4a07488`](crc32c.aligned.3.pieces.cpuid.c17.b4a07488.c) | step=aligned, offset=3, calls=pieces, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.aligned.3.whole.builtin.c17.457878d8`](crc32c.aligned.3.whole.builtin.c17.457878d8.c) | step=aligned, offset=3, calls=whole, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.aligned.3.whole.cpuid.c17.6555d854`](crc32c.aligned.3.whole.cpuid.c17.6555d854.c) | step=aligned, offset=3, calls=whole, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.aligned.7.pieces.builtin.c17.8372d8a8`](crc32c.aligned.7.pieces.builtin.c17.8372d8a8.c) | step=aligned, offset=7, calls=pieces, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.aligned.7.pieces.cpuid.c17.21f86f66`](crc32c.aligned.7.pieces.cpuid.c17.21f86f66.c) | step=aligned, offset=7, calls=pieces, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.aligned.7.whole.builtin.c17.0a8b5762`](crc32c.aligned.7.whole.builtin.c17.0a8b5762.c) | step=aligned, offset=7, calls=whole, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.aligned.7.whole.cpuid.c17.76fb7b47`](crc32c.aligned.7.whole.cpuid.c17.76fb7b47.c) | step=aligned, offset=7, calls=whole, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u32.0.pieces.builtin.c17.ce4f78c4`](crc32c.u32.0.pieces.builtin.c17.ce4f78c4.c) | step=u32, offset=0, calls=pieces, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u32.0.pieces.cpuid.c17.4541b9b2`](crc32c.u32.0.pieces.cpuid.c17.4541b9b2.c) | step=u32, offset=0, calls=pieces, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u32.0.whole.builtin.c17.3ae5ea06`](crc32c.u32.0.whole.builtin.c17.3ae5ea06.c) | step=u32, offset=0, calls=whole, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u32.0.whole.cpuid.c17.0a583a75`](crc32c.u32.0.whole.cpuid.c17.0a583a75.c) | step=u32, offset=0, calls=whole, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u32.1.pieces.builtin.c17.76fceddd`](crc32c.u32.1.pieces.builtin.c17.76fceddd.c) | step=u32, offset=1, calls=pieces, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u32.1.pieces.cpuid.c17.d8347d47`](crc32c.u32.1.pieces.cpuid.c17.d8347d47.c) | step=u32, offset=1, calls=pieces, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u32.1.whole.builtin.c17.0bbe1047`](crc32c.u32.1.whole.builtin.c17.0bbe1047.c) | step=u32, offset=1, calls=whole, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u32.1.whole.cpuid.c17.aa64e2e2`](crc32c.u32.1.whole.cpuid.c17.aa64e2e2.c) | step=u32, offset=1, calls=whole, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u32.3.pieces.builtin.c17.4f4d1121`](crc32c.u32.3.pieces.builtin.c17.4f4d1121.c) | step=u32, offset=3, calls=pieces, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u32.3.pieces.cpuid.c17.6deec706`](crc32c.u32.3.pieces.cpuid.c17.6deec706.c) | step=u32, offset=3, calls=pieces, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u32.3.whole.builtin.c17.8fe4638a`](crc32c.u32.3.whole.builtin.c17.8fe4638a.c) | step=u32, offset=3, calls=whole, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u32.3.whole.cpuid.c17.984adc50`](crc32c.u32.3.whole.cpuid.c17.984adc50.c) | step=u32, offset=3, calls=whole, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u32.7.pieces.builtin.c17.f2bac6dc`](crc32c.u32.7.pieces.builtin.c17.f2bac6dc.c) | step=u32, offset=7, calls=pieces, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u32.7.pieces.cpuid.c17.fe6eb470`](crc32c.u32.7.pieces.cpuid.c17.fe6eb470.c) | step=u32, offset=7, calls=pieces, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u32.7.whole.builtin.c17.264a4101`](crc32c.u32.7.whole.builtin.c17.264a4101.c) | step=u32, offset=7, calls=whole, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u32.7.whole.cpuid.c17.a7e47d7c`](crc32c.u32.7.whole.cpuid.c17.a7e47d7c.c) | step=u32, offset=7, calls=whole, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u64.0.pieces.builtin.c17.4a2abd22`](crc32c.u64.0.pieces.builtin.c17.4a2abd22.c) | step=u64, offset=0, calls=pieces, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u64.0.pieces.cpuid.c17.31cc1662`](crc32c.u64.0.pieces.cpuid.c17.31cc1662.c) | step=u64, offset=0, calls=pieces, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u64.0.whole.builtin.c17.bbb5b889`](crc32c.u64.0.whole.builtin.c17.bbb5b889.c) | step=u64, offset=0, calls=whole, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u64.0.whole.cpuid.c17.765aca7c`](crc32c.u64.0.whole.cpuid.c17.765aca7c.c) | step=u64, offset=0, calls=whole, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u64.1.pieces.builtin.c17.c050abf8`](crc32c.u64.1.pieces.builtin.c17.c050abf8.c) | step=u64, offset=1, calls=pieces, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u64.1.pieces.cpuid.c17.28e459ac`](crc32c.u64.1.pieces.cpuid.c17.28e459ac.c) | step=u64, offset=1, calls=pieces, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u64.1.whole.builtin.c17.286887fc`](crc32c.u64.1.whole.builtin.c17.286887fc.c) | step=u64, offset=1, calls=whole, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u64.1.whole.cpuid.c17.8ba889ee`](crc32c.u64.1.whole.cpuid.c17.8ba889ee.c) | step=u64, offset=1, calls=whole, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u64.3.pieces.builtin.c17.37c832c7`](crc32c.u64.3.pieces.builtin.c17.37c832c7.c) | step=u64, offset=3, calls=pieces, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u64.3.pieces.cpuid.c17.653ec6d1`](crc32c.u64.3.pieces.cpuid.c17.653ec6d1.c) | step=u64, offset=3, calls=pieces, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u64.3.whole.builtin.c17.cdd5830a`](crc32c.u64.3.whole.builtin.c17.cdd5830a.c) | step=u64, offset=3, calls=whole, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u64.3.whole.cpuid.c17.cfdbeffb`](crc32c.u64.3.whole.cpuid.c17.cfdbeffb.c) | step=u64, offset=3, calls=whole, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u64.7.pieces.builtin.c17.ec28a09c`](crc32c.u64.7.pieces.builtin.c17.ec28a09c.c) | step=u64, offset=7, calls=pieces, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u64.7.pieces.cpuid.c17.5af590f7`](crc32c.u64.7.pieces.cpuid.c17.5af590f7.c) | step=u64, offset=7, calls=pieces, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u64.7.whole.builtin.c17.755c1bdf`](crc32c.u64.7.whole.builtin.c17.755c1bdf.c) | step=u64, offset=7, calls=whole, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u64.7.whole.cpuid.c17.67bb62d5`](crc32c.u64.7.whole.cpuid.c17.67bb62d5.c) | step=u64, offset=7, calls=whole, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u8.0.pieces.builtin.c17.79b8f1f1`](crc32c.u8.0.pieces.builtin.c17.79b8f1f1.c) | step=u8, offset=0, calls=pieces, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u8.0.pieces.cpuid.c17.1f790a8d`](crc32c.u8.0.pieces.cpuid.c17.1f790a8d.c) | step=u8, offset=0, calls=pieces, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u8.0.whole.builtin.c17.8d42f91d`](crc32c.u8.0.whole.builtin.c17.8d42f91d.c) | step=u8, offset=0, calls=whole, check=builtin | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u8.0.whole.cpuid.c17.762286d5`](crc32c.u8.0.whole.cpuid.c17.762286d5.c) | step=u8, offset=0, calls=whole, check=cpuid | c17 | `3808858755 0 2751553929 ...` and 23 more lines |
| [`crc32c.u8.1.pieces.builtin.c17.ed777190`](crc32c.u8.1.pieces.builtin.c17.ed777190.c) | step=u8, offset=1, calls=pieces, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u8.1.pieces.cpuid.c17.6b3a77d0`](crc32c.u8.1.pieces.cpuid.c17.6b3a77d0.c) | step=u8, offset=1, calls=pieces, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u8.1.whole.builtin.c17.bfc8723d`](crc32c.u8.1.whole.builtin.c17.bfc8723d.c) | step=u8, offset=1, calls=whole, check=builtin | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u8.1.whole.cpuid.c17.757e3a78`](crc32c.u8.1.whole.cpuid.c17.757e3a78.c) | step=u8, offset=1, calls=whole, check=cpuid | c17 | `3808858755 0 917085598 ...` and 23 more lines |
| [`crc32c.u8.3.pieces.builtin.c17.469cf04c`](crc32c.u8.3.pieces.builtin.c17.469cf04c.c) | step=u8, offset=3, calls=pieces, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u8.3.pieces.cpuid.c17.2bd00456`](crc32c.u8.3.pieces.cpuid.c17.2bd00456.c) | step=u8, offset=3, calls=pieces, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u8.3.whole.builtin.c17.8938f817`](crc32c.u8.3.whole.builtin.c17.8938f817.c) | step=u8, offset=3, calls=whole, check=builtin | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u8.3.whole.cpuid.c17.883edec9`](crc32c.u8.3.whole.cpuid.c17.883edec9.c) | step=u8, offset=3, calls=whole, check=cpuid | c17 | `3808858755 0 1049034281 ...` and 23 more lines |
| [`crc32c.u8.7.pieces.builtin.c17.1f090881`](crc32c.u8.7.pieces.builtin.c17.1f090881.c) | step=u8, offset=7, calls=pieces, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u8.7.pieces.cpuid.c17.ab74f876`](crc32c.u8.7.pieces.cpuid.c17.ab74f876.c) | step=u8, offset=7, calls=pieces, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u8.7.whole.builtin.c17.2b54411e`](crc32c.u8.7.whole.builtin.c17.2b54411e.c) | step=u8, offset=7, calls=whole, check=builtin | c17 | `3808858755 0 1977178319 ...` and 23 more lines |
| [`crc32c.u8.7.whole.cpuid.c17.560da95d`](crc32c.u8.7.whole.cpuid.c17.560da95d.c) | step=u8, offset=7, calls=whole, check=cpuid | c17 | `3808858755 0 1977178319 ...` and 23 more lines |

