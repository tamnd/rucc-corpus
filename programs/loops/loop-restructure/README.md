# loop-restructure

exchanging or fusing loops for locality. Part of the loops phase of the M4 plan.

48 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-restructure.i32.16.column-major.c17.90b98b55`](loop-restructure.i32.16.column-major.c17.90b98b55.c) | type=i32, side=16, shape=column-major | c17 | `32640` |
| [`loop-restructure.i32.16.fused.c17.66492403`](loop-restructure.i32.16.fused.c17.66492403.c) | type=i32, side=16, shape=fused | c17 | `32640` |
| [`loop-restructure.i32.16.row-major.c17.2fd231b3`](loop-restructure.i32.16.row-major.c17.2fd231b3.c) | type=i32, side=16, shape=row-major | c17 | `32640` |
| [`loop-restructure.i32.16.two-loops.c17.e48c8b72`](loop-restructure.i32.16.two-loops.c17.e48c8b72.c) | type=i32, side=16, shape=two-loops | c17 | `32640` |
| [`loop-restructure.i32.4.column-major.c17.86c6938d`](loop-restructure.i32.4.column-major.c17.86c6938d.c) | type=i32, side=4, shape=column-major | c17 | `120` |
| [`loop-restructure.i32.4.fused.c17.fcc51859`](loop-restructure.i32.4.fused.c17.fcc51859.c) | type=i32, side=4, shape=fused | c17 | `120` |
| [`loop-restructure.i32.4.row-major.c17.3ff2b0cb`](loop-restructure.i32.4.row-major.c17.3ff2b0cb.c) | type=i32, side=4, shape=row-major | c17 | `120` |
| [`loop-restructure.i32.4.two-loops.c17.d862da5d`](loop-restructure.i32.4.two-loops.c17.d862da5d.c) | type=i32, side=4, shape=two-loops | c17 | `120` |
| [`loop-restructure.i32.8.column-major.c17.86fb4778`](loop-restructure.i32.8.column-major.c17.86fb4778.c) | type=i32, side=8, shape=column-major | c17 | `2016` |
| [`loop-restructure.i32.8.fused.c17.7338d6ce`](loop-restructure.i32.8.fused.c17.7338d6ce.c) | type=i32, side=8, shape=fused | c17 | `2016` |
| [`loop-restructure.i32.8.row-major.c17.441e9a72`](loop-restructure.i32.8.row-major.c17.441e9a72.c) | type=i32, side=8, shape=row-major | c17 | `2016` |
| [`loop-restructure.i32.8.two-loops.c17.3440d036`](loop-restructure.i32.8.two-loops.c17.3440d036.c) | type=i32, side=8, shape=two-loops | c17 | `2016` |
| [`loop-restructure.i64.16.column-major.c17.c69aae9a`](loop-restructure.i64.16.column-major.c17.c69aae9a.c) | type=i64, side=16, shape=column-major | c17 | `32640` |
| [`loop-restructure.i64.16.fused.c17.0d28cc75`](loop-restructure.i64.16.fused.c17.0d28cc75.c) | type=i64, side=16, shape=fused | c17 | `32640` |
| [`loop-restructure.i64.16.row-major.c17.05614365`](loop-restructure.i64.16.row-major.c17.05614365.c) | type=i64, side=16, shape=row-major | c17 | `32640` |
| [`loop-restructure.i64.16.two-loops.c17.83a03eab`](loop-restructure.i64.16.two-loops.c17.83a03eab.c) | type=i64, side=16, shape=two-loops | c17 | `32640` |
| [`loop-restructure.i64.4.column-major.c17.eb03e066`](loop-restructure.i64.4.column-major.c17.eb03e066.c) | type=i64, side=4, shape=column-major | c17 | `120` |
| [`loop-restructure.i64.4.fused.c17.d62494ab`](loop-restructure.i64.4.fused.c17.d62494ab.c) | type=i64, side=4, shape=fused | c17 | `120` |
| [`loop-restructure.i64.4.row-major.c17.8227251b`](loop-restructure.i64.4.row-major.c17.8227251b.c) | type=i64, side=4, shape=row-major | c17 | `120` |
| [`loop-restructure.i64.4.two-loops.c17.0075a0df`](loop-restructure.i64.4.two-loops.c17.0075a0df.c) | type=i64, side=4, shape=two-loops | c17 | `120` |
| [`loop-restructure.i64.8.column-major.c17.faeae353`](loop-restructure.i64.8.column-major.c17.faeae353.c) | type=i64, side=8, shape=column-major | c17 | `2016` |
| [`loop-restructure.i64.8.fused.c17.71638d3f`](loop-restructure.i64.8.fused.c17.71638d3f.c) | type=i64, side=8, shape=fused | c17 | `2016` |
| [`loop-restructure.i64.8.row-major.c17.ab3c9a3b`](loop-restructure.i64.8.row-major.c17.ab3c9a3b.c) | type=i64, side=8, shape=row-major | c17 | `2016` |
| [`loop-restructure.i64.8.two-loops.c17.7a02d0c5`](loop-restructure.i64.8.two-loops.c17.7a02d0c5.c) | type=i64, side=8, shape=two-loops | c17 | `2016` |
| [`loop-restructure.u32.16.column-major.c17.0675892c`](loop-restructure.u32.16.column-major.c17.0675892c.c) | type=u32, side=16, shape=column-major | c17 | `32640` |
| [`loop-restructure.u32.16.fused.c17.860b2d7c`](loop-restructure.u32.16.fused.c17.860b2d7c.c) | type=u32, side=16, shape=fused | c17 | `32640` |
| [`loop-restructure.u32.16.row-major.c17.949a9ccb`](loop-restructure.u32.16.row-major.c17.949a9ccb.c) | type=u32, side=16, shape=row-major | c17 | `32640` |
| [`loop-restructure.u32.16.two-loops.c17.5f558e1e`](loop-restructure.u32.16.two-loops.c17.5f558e1e.c) | type=u32, side=16, shape=two-loops | c17 | `32640` |
| [`loop-restructure.u32.4.column-major.c17.356c42e1`](loop-restructure.u32.4.column-major.c17.356c42e1.c) | type=u32, side=4, shape=column-major | c17 | `120` |
| [`loop-restructure.u32.4.fused.c17.d5405f60`](loop-restructure.u32.4.fused.c17.d5405f60.c) | type=u32, side=4, shape=fused | c17 | `120` |
| [`loop-restructure.u32.4.row-major.c17.f3d92905`](loop-restructure.u32.4.row-major.c17.f3d92905.c) | type=u32, side=4, shape=row-major | c17 | `120` |
| [`loop-restructure.u32.4.two-loops.c17.ce3a22f6`](loop-restructure.u32.4.two-loops.c17.ce3a22f6.c) | type=u32, side=4, shape=two-loops | c17 | `120` |
| [`loop-restructure.u32.8.column-major.c17.1d8091f3`](loop-restructure.u32.8.column-major.c17.1d8091f3.c) | type=u32, side=8, shape=column-major | c17 | `2016` |
| [`loop-restructure.u32.8.fused.c17.dca69115`](loop-restructure.u32.8.fused.c17.dca69115.c) | type=u32, side=8, shape=fused | c17 | `2016` |
| [`loop-restructure.u32.8.row-major.c17.486a2098`](loop-restructure.u32.8.row-major.c17.486a2098.c) | type=u32, side=8, shape=row-major | c17 | `2016` |
| [`loop-restructure.u32.8.two-loops.c17.ead7a0cb`](loop-restructure.u32.8.two-loops.c17.ead7a0cb.c) | type=u32, side=8, shape=two-loops | c17 | `2016` |
| [`loop-restructure.u64.16.column-major.c17.37db052a`](loop-restructure.u64.16.column-major.c17.37db052a.c) | type=u64, side=16, shape=column-major | c17 | `32640` |
| [`loop-restructure.u64.16.fused.c17.1f2cbd40`](loop-restructure.u64.16.fused.c17.1f2cbd40.c) | type=u64, side=16, shape=fused | c17 | `32640` |
| [`loop-restructure.u64.16.row-major.c17.eab72dd0`](loop-restructure.u64.16.row-major.c17.eab72dd0.c) | type=u64, side=16, shape=row-major | c17 | `32640` |
| [`loop-restructure.u64.16.two-loops.c17.7570052e`](loop-restructure.u64.16.two-loops.c17.7570052e.c) | type=u64, side=16, shape=two-loops | c17 | `32640` |
| [`loop-restructure.u64.4.column-major.c17.bf75c9fc`](loop-restructure.u64.4.column-major.c17.bf75c9fc.c) | type=u64, side=4, shape=column-major | c17 | `120` |
| [`loop-restructure.u64.4.fused.c17.7c342a3e`](loop-restructure.u64.4.fused.c17.7c342a3e.c) | type=u64, side=4, shape=fused | c17 | `120` |
| [`loop-restructure.u64.4.row-major.c17.1534eed3`](loop-restructure.u64.4.row-major.c17.1534eed3.c) | type=u64, side=4, shape=row-major | c17 | `120` |
| [`loop-restructure.u64.4.two-loops.c17.6a131873`](loop-restructure.u64.4.two-loops.c17.6a131873.c) | type=u64, side=4, shape=two-loops | c17 | `120` |
| [`loop-restructure.u64.8.column-major.c17.e311f4f3`](loop-restructure.u64.8.column-major.c17.e311f4f3.c) | type=u64, side=8, shape=column-major | c17 | `2016` |
| [`loop-restructure.u64.8.fused.c17.7a786a53`](loop-restructure.u64.8.fused.c17.7a786a53.c) | type=u64, side=8, shape=fused | c17 | `2016` |
| [`loop-restructure.u64.8.row-major.c17.68dbb963`](loop-restructure.u64.8.row-major.c17.68dbb963.c) | type=u64, side=8, shape=row-major | c17 | `2016` |
| [`loop-restructure.u64.8.two-loops.c17.95399389`](loop-restructure.u64.8.two-loops.c17.95399389.c) | type=u64, side=8, shape=two-loops | c17 | `2016` |

