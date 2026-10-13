# block-copy

a copy or a clear of a block of known size, as moves or as a call. Part of the backend phase of the M4 plan.

44 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`block-copy.assign.100.1.c17.425d3c41`](block-copy.assign.100.1.c17.425d3c41.c) | shape=assign, bytes=100, align=1 | c17 | `254786416` |
| [`block-copy.assign.128.8.c17.ad1cfa84`](block-copy.assign.128.8.c17.ad1cfa84.c) | shape=assign, bytes=128, align=8 | c17 | `255170775` |
| [`block-copy.assign.13.1.c17.36db9ae4`](block-copy.assign.13.1.c17.36db9ae4.c) | shape=assign, bytes=13, align=1 | c17 | `254818574` |
| [`block-copy.assign.16.8.c17.9ef12f95`](block-copy.assign.16.8.c17.9ef12f95.c) | shape=assign, bytes=16, align=8 | c17 | `254886276` |
| [`block-copy.assign.256.8.c17.f5d90569`](block-copy.assign.256.8.c17.f5d90569.c) | shape=assign, bytes=256, align=8 | c17 | `255004425` |
| [`block-copy.assign.32.8.c17.c15d06c3`](block-copy.assign.32.8.c17.c15d06c3.c) | shape=assign, bytes=32, align=8 | c17 | `254885106` |
| [`block-copy.assign.48.8.c17.ff65624a`](block-copy.assign.48.8.c17.ff65624a.c) | shape=assign, bytes=48, align=8 | c17 | `254513312` |
| [`block-copy.assign.61.1.c17.cfa4889c`](block-copy.assign.61.1.c17.cfa4889c.c) | shape=assign, bytes=61, align=1 | c17 | `254896416` |
| [`block-copy.assign.64.8.c17.5aa0763f`](block-copy.assign.64.8.c17.5aa0763f.c) | shape=assign, bytes=64, align=8 | c17 | `254898345` |
| [`block-copy.assign.80.8.c17.da077e7c`](block-copy.assign.80.8.c17.da077e7c.c) | shape=assign, bytes=80, align=8 | c17 | `255328820` |
| [`block-copy.assign.96.8.c17.88d6e753`](block-copy.assign.96.8.c17.88d6e753.c) | shape=assign, bytes=96, align=8 | c17 | `255296265` |
| [`block-copy.literal.100.1.c17.d651483c`](block-copy.literal.100.1.c17.d651483c.c) | shape=literal, bytes=100, align=1 | c17 | `127393600` |
| [`block-copy.literal.128.8.c17.c873df2c`](block-copy.literal.128.8.c17.c873df2c.c) | shape=literal, bytes=128, align=8 | c17 | `127588320` |
| [`block-copy.literal.13.1.c17.938440e9`](block-copy.literal.13.1.c17.938440e9.c) | shape=literal, bytes=13, align=1 | c17 | `127409656` |
| [`block-copy.literal.16.8.c17.946ab7c3`](block-copy.literal.16.8.c17.946ab7c3.c) | shape=literal, bytes=16, align=8 | c17 | `127443264` |
| [`block-copy.literal.256.8.c17.2e40629a`](block-copy.literal.256.8.c17.2e40629a.c) | shape=literal, bytes=256, align=8 | c17 | `127498336` |
| [`block-copy.literal.32.8.c17.cfc9d078`](block-copy.literal.32.8.c17.cfc9d078.c) | shape=literal, bytes=32, align=8 | c17 | `127442688` |
| [`block-copy.literal.48.8.c17.820f1dd9`](block-copy.literal.48.8.c17.820f1dd9.c) | shape=literal, bytes=48, align=8 | c17 | `127257584` |
| [`block-copy.literal.61.1.c17.6e38af59`](block-copy.literal.61.1.c17.6e38af59.c) | shape=literal, bytes=61, align=1 | c17 | `127448124` |
| [`block-copy.literal.64.8.c17.6edc7334`](block-copy.literal.64.8.c17.6edc7334.c) | shape=literal, bytes=64, align=8 | c17 | `127449312` |
| [`block-copy.literal.80.8.c17.59aa7ab3`](block-copy.literal.80.8.c17.59aa7ab3.c) | shape=literal, bytes=80, align=8 | c17 | `127664960` |
| [`block-copy.literal.96.8.c17.ebd3b996`](block-copy.literal.96.8.c17.ebd3b996.c) | shape=literal, bytes=96, align=8 | c17 | `127651040` |
| [`block-copy.memcpy.100.1.c17.0b9c7117`](block-copy.memcpy.100.1.c17.0b9c7117.c) | shape=memcpy, bytes=100, align=1 | c17 | `254786416` |
| [`block-copy.memcpy.128.8.c17.df28a755`](block-copy.memcpy.128.8.c17.df28a755.c) | shape=memcpy, bytes=128, align=8 | c17 | `255170775` |
| [`block-copy.memcpy.13.1.c17.77d180a5`](block-copy.memcpy.13.1.c17.77d180a5.c) | shape=memcpy, bytes=13, align=1 | c17 | `254818574` |
| [`block-copy.memcpy.16.8.c17.34414c38`](block-copy.memcpy.16.8.c17.34414c38.c) | shape=memcpy, bytes=16, align=8 | c17 | `254886276` |
| [`block-copy.memcpy.256.8.c17.8d9f0b91`](block-copy.memcpy.256.8.c17.8d9f0b91.c) | shape=memcpy, bytes=256, align=8 | c17 | `255004425` |
| [`block-copy.memcpy.32.8.c17.65949956`](block-copy.memcpy.32.8.c17.65949956.c) | shape=memcpy, bytes=32, align=8 | c17 | `254885106` |
| [`block-copy.memcpy.48.8.c17.43917aa5`](block-copy.memcpy.48.8.c17.43917aa5.c) | shape=memcpy, bytes=48, align=8 | c17 | `254513312` |
| [`block-copy.memcpy.61.1.c17.7210e579`](block-copy.memcpy.61.1.c17.7210e579.c) | shape=memcpy, bytes=61, align=1 | c17 | `254896416` |
| [`block-copy.memcpy.64.8.c17.d1e5c184`](block-copy.memcpy.64.8.c17.d1e5c184.c) | shape=memcpy, bytes=64, align=8 | c17 | `254898345` |
| [`block-copy.memcpy.80.8.c17.e5dea351`](block-copy.memcpy.80.8.c17.e5dea351.c) | shape=memcpy, bytes=80, align=8 | c17 | `255328820` |
| [`block-copy.memcpy.96.8.c17.87620694`](block-copy.memcpy.96.8.c17.87620694.c) | shape=memcpy, bytes=96, align=8 | c17 | `255296265` |
| [`block-copy.memset.100.1.c17.ae896e65`](block-copy.memset.100.1.c17.ae896e65.c) | shape=memset, bytes=100, align=1 | c17 | `127393600` |
| [`block-copy.memset.128.8.c17.b2aa4ee6`](block-copy.memset.128.8.c17.b2aa4ee6.c) | shape=memset, bytes=128, align=8 | c17 | `127588320` |
| [`block-copy.memset.13.1.c17.aa6b39ed`](block-copy.memset.13.1.c17.aa6b39ed.c) | shape=memset, bytes=13, align=1 | c17 | `127409656` |
| [`block-copy.memset.16.8.c17.37decee6`](block-copy.memset.16.8.c17.37decee6.c) | shape=memset, bytes=16, align=8 | c17 | `127443264` |
| [`block-copy.memset.256.8.c17.37de022a`](block-copy.memset.256.8.c17.37de022a.c) | shape=memset, bytes=256, align=8 | c17 | `127498336` |
| [`block-copy.memset.32.8.c17.dfaa0290`](block-copy.memset.32.8.c17.dfaa0290.c) | shape=memset, bytes=32, align=8 | c17 | `127442688` |
| [`block-copy.memset.48.8.c17.01d8e880`](block-copy.memset.48.8.c17.01d8e880.c) | shape=memset, bytes=48, align=8 | c17 | `127257584` |
| [`block-copy.memset.61.1.c17.17cd2e03`](block-copy.memset.61.1.c17.17cd2e03.c) | shape=memset, bytes=61, align=1 | c17 | `127448124` |
| [`block-copy.memset.64.8.c17.b6607543`](block-copy.memset.64.8.c17.b6607543.c) | shape=memset, bytes=64, align=8 | c17 | `127449312` |
| [`block-copy.memset.80.8.c17.dacbec64`](block-copy.memset.80.8.c17.dacbec64.c) | shape=memset, bytes=80, align=8 | c17 | `127664960` |
| [`block-copy.memset.96.8.c17.dc44e3c6`](block-copy.memset.96.8.c17.dc44e3c6.c) | shape=memset, bytes=96, align=8 | c17 | `127651040` |

