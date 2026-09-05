# alias-analysis

proving two references cannot name the same object. Part of the global phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`alias-analysis.i32.distinct-indices.c17.3cd5cae7`](alias-analysis.i32.distinct-indices.c17.3cd5cae7.c) | type=i32, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.i32.distinct-locals.c17.9f9f3d08`](alias-analysis.i32.distinct-locals.c17.9f9f3d08.c) | type=i32, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.i32.may-alias.c17.600624a4`](alias-analysis.i32.may-alias.c17.600624a4.c) | type=i32, shape=may-alias | c17 | `14` |
| [`alias-analysis.i32.restrict.c17.04c007e2`](alias-analysis.i32.restrict.c17.04c007e2.c) | type=i32, shape=restrict | c17 | `13` |
| [`alias-analysis.i64.distinct-indices.c17.d56c27c7`](alias-analysis.i64.distinct-indices.c17.d56c27c7.c) | type=i64, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.i64.distinct-locals.c17.62d1e29a`](alias-analysis.i64.distinct-locals.c17.62d1e29a.c) | type=i64, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.i64.may-alias.c17.d8ec6558`](alias-analysis.i64.may-alias.c17.d8ec6558.c) | type=i64, shape=may-alias | c17 | `14` |
| [`alias-analysis.i64.restrict.c17.561a4990`](alias-analysis.i64.restrict.c17.561a4990.c) | type=i64, shape=restrict | c17 | `13` |
| [`alias-analysis.u32.distinct-indices.c17.ac08e171`](alias-analysis.u32.distinct-indices.c17.ac08e171.c) | type=u32, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.u32.distinct-locals.c17.b793c74a`](alias-analysis.u32.distinct-locals.c17.b793c74a.c) | type=u32, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.u32.may-alias.c17.8b2aa55b`](alias-analysis.u32.may-alias.c17.8b2aa55b.c) | type=u32, shape=may-alias | c17 | `14` |
| [`alias-analysis.u32.restrict.c17.d67e0d70`](alias-analysis.u32.restrict.c17.d67e0d70.c) | type=u32, shape=restrict | c17 | `13` |
| [`alias-analysis.u64.distinct-indices.c17.233c0752`](alias-analysis.u64.distinct-indices.c17.233c0752.c) | type=u64, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.u64.distinct-locals.c17.ad8bd0d3`](alias-analysis.u64.distinct-locals.c17.ad8bd0d3.c) | type=u64, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.u64.may-alias.c17.6134a36b`](alias-analysis.u64.may-alias.c17.6134a36b.c) | type=u64, shape=may-alias | c17 | `14` |
| [`alias-analysis.u64.restrict.c17.66f42ddf`](alias-analysis.u64.restrict.c17.66f42ddf.c) | type=u64, shape=restrict | c17 | `13` |

