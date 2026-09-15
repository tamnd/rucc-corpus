# scheduling

ordering instructions within a block. Part of the backend phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`scheduling.u32.1.32.c17.59a89e05`](scheduling.u32.1.32.c17.59a89e05.c) | type=u32, chains=1, length=32 | c17 | `1338619649` |
| [`scheduling.u32.1.8.c17.1cd5934b`](scheduling.u32.1.8.c17.1cd5934b.c) | type=u32, chains=1, length=8 | c17 | `3542965697` |
| [`scheduling.u32.2.32.c17.74c2c8c6`](scheduling.u32.2.32.c17.74c2c8c6.c) | type=u32, chains=2, length=32 | c17 | `706340867` |
| [`scheduling.u32.2.8.c17.e9558143`](scheduling.u32.2.8.c17.e9558143.c) | type=u32, chains=2, length=8 | c17 | `2289629699` |
| [`scheduling.u32.4.32.c17.7aceedd4`](scheduling.u32.4.32.c17.7aceedd4.c) | type=u32, chains=4, length=32 | c17 | `2119022602` |
| [`scheduling.u32.4.8.c17.9cfb012f`](scheduling.u32.4.8.c17.9cfb012f.c) | type=u32, chains=4, length=8 | c17 | `2573921802` |
| [`scheduling.u32.read-after-write.c17.60401d16`](scheduling.u32.read-after-write.c17.60401d16.c) | type=u32, access=read-after-write | c17 | `1599746848` |
| [`scheduling.u32.read-only.c17.158d22d7`](scheduling.u32.read-only.c17.158d22d7.c) | type=u32, access=read-only | c17 | `1356217536` |
| [`scheduling.u32.two-writes.c17.a5471f0c`](scheduling.u32.two-writes.c17.a5471f0c.c) | type=u32, access=two-writes | c17 | `1717986918` |
| [`scheduling.u32.write-after-read.c17.654d6715`](scheduling.u32.write-after-read.c17.654d6715.c) | type=u32, access=write-after-read | c17 | `265632032` |
| [`scheduling.u64.1.32.c17.a72a5443`](scheduling.u64.1.32.c17.a72a5443.c) | type=u64, chains=1, length=32 | c17 | `13862957897887627009` |
| [`scheduling.u64.1.8.c17.7919aad2`](scheduling.u64.1.8.c17.7919aad2.c) | type=u64, chains=1, length=8 | c17 | `2779530283277761` |
| [`scheduling.u64.2.32.c17.92fc0765`](scheduling.u64.2.32.c17.92fc0765.c) | type=u64, chains=2, length=32 | c17 | `74399580281235459` |
| [`scheduling.u64.2.8.c17.bfb6f11c`](scheduling.u64.2.8.c17.bfb6f11c.c) | type=u64, chains=2, length=8 | c17 | `7412080755407363` |
| [`scheduling.u64.4.32.c17.7ebc5efd`](scheduling.u64.4.32.c17.7ebc5efd.c) | type=u64, chains=4, length=32 | c17 | `223198740843706378` |
| [`scheduling.u64.4.8.c17.addb3b11`](scheduling.u64.4.8.c17.addb3b11.c) | type=u64, chains=4, length=8 | c17 | `22236242266222090` |
| [`scheduling.u64.read-after-write.c17.48180b09`](scheduling.u64.read-after-write.c17.48180b09.c) | type=u64, access=read-after-write | c17 | `3641665938238616352` |
| [`scheduling.u64.read-only.c17.8ffd8e21`](scheduling.u64.read-only.c17.8ffd8e21.c) | type=u64, access=read-only | c17 | `1396478424030526656` |
| [`scheduling.u64.two-writes.c17.247d823b`](scheduling.u64.two-writes.c17.247d823b.c) | type=u64, access=two-writes | c17 | `7378697629483820646` |
| [`scheduling.u64.write-after-read.c17.13a7e8e5`](scheduling.u64.write-after-read.c17.13a7e8e5.c) | type=u64, access=write-after-read | c17 | `5163711417854277920` |

