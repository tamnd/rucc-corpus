# simd-lfind

the SSE2 search loops from port/simd.h and pg_lfind.h. Part of the correctness phase of the M4 plan.

30 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`simd-lfind.lfind32-x1.high.0.c17.87bc3b6a`](simd-lfind.lfind32-x1.high.0.c17.87bc3b6a.c) | search=lfind32-x1, values=high, offset=0 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x1.high.1.c17.9b80b718`](simd-lfind.lfind32-x1.high.1.c17.9b80b718.c) | search=lfind32-x1, values=high, offset=1 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x1.high.3.c17.7ac09c5a`](simd-lfind.lfind32-x1.high.3.c17.7ac09c5a.c) | search=lfind32-x1, values=high, offset=3 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x1.low.0.c17.2109efe8`](simd-lfind.lfind32-x1.low.0.c17.2109efe8.c) | search=lfind32-x1, values=low, offset=0 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x1.low.1.c17.bef9c67f`](simd-lfind.lfind32-x1.low.1.c17.bef9c67f.c) | search=lfind32-x1, values=low, offset=1 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x1.low.3.c17.5b529775`](simd-lfind.lfind32-x1.low.3.c17.5b529775.c) | search=lfind32-x1, values=low, offset=3 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x4.high.0.c17.3235d5eb`](simd-lfind.lfind32-x4.high.0.c17.3235d5eb.c) | search=lfind32-x4, values=high, offset=0 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x4.high.1.c17.6225ad6e`](simd-lfind.lfind32-x4.high.1.c17.6225ad6e.c) | search=lfind32-x4, values=high, offset=1 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x4.high.3.c17.ac8fd4d8`](simd-lfind.lfind32-x4.high.3.c17.ac8fd4d8.c) | search=lfind32-x4, values=high, offset=3 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x4.low.0.c17.a78568a3`](simd-lfind.lfind32-x4.low.0.c17.a78568a3.c) | search=lfind32-x4, values=low, offset=0 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x4.low.1.c17.a5382b9e`](simd-lfind.lfind32-x4.low.1.c17.a5382b9e.c) | search=lfind32-x4, values=low, offset=1 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind32-x4.low.3.c17.d0635d29`](simd-lfind.lfind32-x4.low.3.c17.d0635d29.c) | search=lfind32-x4, values=low, offset=3 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-max.high.0.c17.c7952ddb`](simd-lfind.lfind8-le-max.high.0.c17.c7952ddb.c) | search=lfind8-le-max, values=high, offset=0 | c17 | `47 50 50 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-max.high.1.c17.4ed85cec`](simd-lfind.lfind8-le-max.high.1.c17.4ed85cec.c) | search=lfind8-le-max, values=high, offset=1 | c17 | `52 48 48 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-max.high.3.c17.9625de19`](simd-lfind.lfind8-le-max.high.3.c17.9625de19.c) | search=lfind8-le-max, values=high, offset=3 | c17 | `50 50 49 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-max.low.0.c17.7bf6bc7e`](simd-lfind.lfind8-le-max.low.0.c17.7bf6bc7e.c) | search=lfind8-le-max, values=low, offset=0 | c17 | `52 45 45 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-max.low.1.c17.50965dfc`](simd-lfind.lfind8-le-max.low.1.c17.50965dfc.c) | search=lfind8-le-max, values=low, offset=1 | c17 | `52 46 46 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-max.low.3.c17.ed4e7ddb`](simd-lfind.lfind8-le-max.low.3.c17.ed4e7ddb.c) | search=lfind8-le-max, values=low, offset=3 | c17 | `50 48 46 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-subs.high.0.c17.6ba6a00c`](simd-lfind.lfind8-le-subs.high.0.c17.6ba6a00c.c) | search=lfind8-le-subs, values=high, offset=0 | c17 | `47 50 50 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-subs.high.1.c17.f2d2820f`](simd-lfind.lfind8-le-subs.high.1.c17.f2d2820f.c) | search=lfind8-le-subs, values=high, offset=1 | c17 | `52 48 48 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-subs.high.3.c17.01b29770`](simd-lfind.lfind8-le-subs.high.3.c17.01b29770.c) | search=lfind8-le-subs, values=high, offset=3 | c17 | `50 50 49 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-subs.low.0.c17.148bb4cf`](simd-lfind.lfind8-le-subs.low.0.c17.148bb4cf.c) | search=lfind8-le-subs, values=low, offset=0 | c17 | `52 45 45 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-subs.low.1.c17.61e1e169`](simd-lfind.lfind8-le-subs.low.1.c17.61e1e169.c) | search=lfind8-le-subs, values=low, offset=1 | c17 | `52 46 46 ...` and 3 more lines |
| [`simd-lfind.lfind8-le-subs.low.3.c17.ca4c54a2`](simd-lfind.lfind8-le-subs.low.3.c17.ca4c54a2.c) | search=lfind8-le-subs, values=low, offset=3 | c17 | `50 48 46 ...` and 3 more lines |
| [`simd-lfind.lfind8.high.0.c17.6fc71636`](simd-lfind.lfind8.high.0.c17.6fc71636.c) | search=lfind8, values=high, offset=0 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind8.high.1.c17.675f197b`](simd-lfind.lfind8.high.1.c17.675f197b.c) | search=lfind8, values=high, offset=1 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind8.high.3.c17.7ba429c7`](simd-lfind.lfind8.high.3.c17.7ba429c7.c) | search=lfind8, values=high, offset=3 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind8.low.0.c17.ad5854e3`](simd-lfind.lfind8.low.0.c17.ad5854e3.c) | search=lfind8, values=low, offset=0 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind8.low.1.c17.a9a74dd6`](simd-lfind.lfind8.low.1.c17.a9a74dd6.c) | search=lfind8, values=low, offset=1 | c17 | `53 53 53 ...` and 3 more lines |
| [`simd-lfind.lfind8.low.3.c17.f02c28d2`](simd-lfind.lfind8.low.3.c17.f02c28d2.c) | search=lfind8, values=low, offset=3 | c17 | `53 53 53 ...` and 3 more lines |

