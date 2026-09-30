# simd-lfind-neon

the NEON search loops from port/simd.h and pg_lfind.h. Part of the correctness phase of the M4 plan.

48 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`simd-lfind-neon.index8-mask.high.0.c17.0a8950bd`](simd-lfind-neon.index8-mask.high.0.c17.0a8950bd.c) | search=index8-mask, values=high, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-mask.high.1.c17.69c46999`](simd-lfind-neon.index8-mask.high.1.c17.69c46999.c) | search=index8-mask, values=high, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-mask.high.3.c17.414d874f`](simd-lfind-neon.index8-mask.high.3.c17.414d874f.c) | search=index8-mask, values=high, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-mask.low.0.c17.bfb35007`](simd-lfind-neon.index8-mask.low.0.c17.bfb35007.c) | search=index8-mask, values=low, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-mask.low.1.c17.b1645558`](simd-lfind-neon.index8-mask.low.1.c17.b1645558.c) | search=index8-mask, values=low, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-mask.low.3.c17.eefbe16f`](simd-lfind-neon.index8-mask.low.3.c17.eefbe16f.c) | search=index8-mask, values=low, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-shrn.high.0.c17.5dfadc59`](simd-lfind-neon.index8-shrn.high.0.c17.5dfadc59.c) | search=index8-shrn, values=high, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-shrn.high.1.c17.5bca8816`](simd-lfind-neon.index8-shrn.high.1.c17.5bca8816.c) | search=index8-shrn, values=high, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-shrn.high.3.c17.337f0c82`](simd-lfind-neon.index8-shrn.high.3.c17.337f0c82.c) | search=index8-shrn, values=high, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-shrn.low.0.c17.1efa74f7`](simd-lfind-neon.index8-shrn.low.0.c17.1efa74f7.c) | search=index8-shrn, values=low, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-shrn.low.1.c17.0641ab19`](simd-lfind-neon.index8-shrn.low.1.c17.0641ab19.c) | search=index8-shrn, values=low, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.index8-shrn.low.3.c17.198a5e1f`](simd-lfind-neon.index8-shrn.low.3.c17.198a5e1f.c) | search=index8-shrn, values=low, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x1.high.0.c17.a56a728b`](simd-lfind-neon.lfind32-x1.high.0.c17.a56a728b.c) | search=lfind32-x1, values=high, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x1.high.1.c17.b7ac0257`](simd-lfind-neon.lfind32-x1.high.1.c17.b7ac0257.c) | search=lfind32-x1, values=high, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x1.high.3.c17.8158d507`](simd-lfind-neon.lfind32-x1.high.3.c17.8158d507.c) | search=lfind32-x1, values=high, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x1.low.0.c17.d01d5d70`](simd-lfind-neon.lfind32-x1.low.0.c17.d01d5d70.c) | search=lfind32-x1, values=low, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x1.low.1.c17.e84a75da`](simd-lfind-neon.lfind32-x1.low.1.c17.e84a75da.c) | search=lfind32-x1, values=low, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x1.low.3.c17.a342769d`](simd-lfind-neon.lfind32-x1.low.3.c17.a342769d.c) | search=lfind32-x1, values=low, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x4.high.0.c17.04e4c31f`](simd-lfind-neon.lfind32-x4.high.0.c17.04e4c31f.c) | search=lfind32-x4, values=high, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x4.high.1.c17.6bb8bfa1`](simd-lfind-neon.lfind32-x4.high.1.c17.6bb8bfa1.c) | search=lfind32-x4, values=high, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x4.high.3.c17.ea7f1775`](simd-lfind-neon.lfind32-x4.high.3.c17.ea7f1775.c) | search=lfind32-x4, values=high, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x4.low.0.c17.9df2ce15`](simd-lfind-neon.lfind32-x4.low.0.c17.9df2ce15.c) | search=lfind32-x4, values=low, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x4.low.1.c17.72dfb11e`](simd-lfind-neon.lfind32-x4.low.1.c17.72dfb11e.c) | search=lfind32-x4, values=low, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind32-x4.low.3.c17.2f4e07cb`](simd-lfind-neon.lfind32-x4.low.3.c17.2f4e07cb.c) | search=lfind32-x4, values=low, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-minv.high.0.c17.fd1aee13`](simd-lfind-neon.lfind8-le-minv.high.0.c17.fd1aee13.c) | search=lfind8-le-minv, values=high, offset=0 | c17 | `47 50 50 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-minv.high.1.c17.0ca63707`](simd-lfind-neon.lfind8-le-minv.high.1.c17.0ca63707.c) | search=lfind8-le-minv, values=high, offset=1 | c17 | `52 48 48 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-minv.high.3.c17.d36a637d`](simd-lfind-neon.lfind8-le-minv.high.3.c17.d36a637d.c) | search=lfind8-le-minv, values=high, offset=3 | c17 | `50 50 49 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-minv.low.0.c17.c9827a8e`](simd-lfind-neon.lfind8-le-minv.low.0.c17.c9827a8e.c) | search=lfind8-le-minv, values=low, offset=0 | c17 | `52 45 45 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-minv.low.1.c17.c8c91741`](simd-lfind-neon.lfind8-le-minv.low.1.c17.c8c91741.c) | search=lfind8-le-minv, values=low, offset=1 | c17 | `52 46 46 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-minv.low.3.c17.48fbeddf`](simd-lfind-neon.lfind8-le-minv.low.3.c17.48fbeddf.c) | search=lfind8-le-minv, values=low, offset=3 | c17 | `50 48 46 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-subs.high.0.c17.379e1c85`](simd-lfind-neon.lfind8-le-subs.high.0.c17.379e1c85.c) | search=lfind8-le-subs, values=high, offset=0 | c17 | `47 50 50 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-subs.high.1.c17.a61f172e`](simd-lfind-neon.lfind8-le-subs.high.1.c17.a61f172e.c) | search=lfind8-le-subs, values=high, offset=1 | c17 | `52 48 48 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-subs.high.3.c17.a062c8db`](simd-lfind-neon.lfind8-le-subs.high.3.c17.a062c8db.c) | search=lfind8-le-subs, values=high, offset=3 | c17 | `50 50 49 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-subs.low.0.c17.71a0fa2d`](simd-lfind-neon.lfind8-le-subs.low.0.c17.71a0fa2d.c) | search=lfind8-le-subs, values=low, offset=0 | c17 | `52 45 45 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-subs.low.1.c17.d0f77c64`](simd-lfind-neon.lfind8-le-subs.low.1.c17.d0f77c64.c) | search=lfind8-le-subs, values=low, offset=1 | c17 | `52 46 46 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-le-subs.low.3.c17.2eb23ed7`](simd-lfind-neon.lfind8-le-subs.low.3.c17.2eb23ed7.c) | search=lfind8-le-subs, values=low, offset=3 | c17 | `50 48 46 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-x2.high.0.c17.56d655ae`](simd-lfind-neon.lfind8-x2.high.0.c17.56d655ae.c) | search=lfind8-x2, values=high, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-x2.high.1.c17.acef8d61`](simd-lfind-neon.lfind8-x2.high.1.c17.acef8d61.c) | search=lfind8-x2, values=high, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-x2.high.3.c17.826ec98d`](simd-lfind-neon.lfind8-x2.high.3.c17.826ec98d.c) | search=lfind8-x2, values=high, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-x2.low.0.c17.48c51a05`](simd-lfind-neon.lfind8-x2.low.0.c17.48c51a05.c) | search=lfind8-x2, values=low, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-x2.low.1.c17.0ef0d540`](simd-lfind-neon.lfind8-x2.low.1.c17.0ef0d540.c) | search=lfind8-x2, values=low, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8-x2.low.3.c17.74c1332e`](simd-lfind-neon.lfind8-x2.low.3.c17.74c1332e.c) | search=lfind8-x2, values=low, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8.high.0.c17.64d93cba`](simd-lfind-neon.lfind8.high.0.c17.64d93cba.c) | search=lfind8, values=high, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8.high.1.c17.19ded0eb`](simd-lfind-neon.lfind8.high.1.c17.19ded0eb.c) | search=lfind8, values=high, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8.high.3.c17.f36a153a`](simd-lfind-neon.lfind8.high.3.c17.f36a153a.c) | search=lfind8, values=high, offset=3 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8.low.0.c17.9dbc4b80`](simd-lfind-neon.lfind8.low.0.c17.9dbc4b80.c) | search=lfind8, values=low, offset=0 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8.low.1.c17.50f746b1`](simd-lfind-neon.lfind8.low.1.c17.50f746b1.c) | search=lfind8, values=low, offset=1 | c17 | `53 53 53 ...` and 4 more lines |
| [`simd-lfind-neon.lfind8.low.3.c17.045d0173`](simd-lfind-neon.lfind8.low.3.c17.045d0173.c) | search=lfind8, values=low, offset=3 | c17 | `53 53 53 ...` and 4 more lines |

