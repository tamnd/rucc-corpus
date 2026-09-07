# alias-analysis

proving two references cannot name the same object. Part of the global phase of the M4 plan.

36 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`alias-analysis.i32.char-pointer.c17.2c550cb8`](alias-analysis.i32.char-pointer.c17.2c550cb8.c) | type=i32, shape=char-pointer | c17 | `7` |
| [`alias-analysis.i32.computed-same-index.c17.56d27958`](alias-analysis.i32.computed-same-index.c17.56d27958.c) | type=i32, shape=computed-same-index | c17 | `14` |
| [`alias-analysis.i32.distinct-indices.c17.3cd5cae7`](alias-analysis.i32.distinct-indices.c17.3cd5cae7.c) | type=i32, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.i32.distinct-locals.c17.9f9f3d08`](alias-analysis.i32.distinct-locals.c17.9f9f3d08.c) | type=i32, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.i32.distinct-types.c17.84b3fa35`](alias-analysis.i32.distinct-types.c17.84b3fa35.c) | type=i32, shape=distinct-types | c17 | `8` |
| [`alias-analysis.i32.escaped-local.c17.051bcf33`](alias-analysis.i32.escaped-local.c17.051bcf33.c) | type=i32, shape=escaped-local | c17 | `8` |
| [`alias-analysis.i32.may-alias.c17.600624a4`](alias-analysis.i32.may-alias.c17.600624a4.c) | type=i32, shape=may-alias | c17 | `14` |
| [`alias-analysis.i32.restrict.c17.04c007e2`](alias-analysis.i32.restrict.c17.04c007e2.c) | type=i32, shape=restrict | c17 | `13` |
| [`alias-analysis.i32.union-pun.c17.fe8474a9`](alias-analysis.i32.union-pun.c17.fe8474a9.c) | type=i32, shape=union-pun | c17 | `7` |
| [`alias-analysis.i64.char-pointer.c17.4be55c4f`](alias-analysis.i64.char-pointer.c17.4be55c4f.c) | type=i64, shape=char-pointer | c17 | `7` |
| [`alias-analysis.i64.computed-same-index.c17.eb7c5c73`](alias-analysis.i64.computed-same-index.c17.eb7c5c73.c) | type=i64, shape=computed-same-index | c17 | `14` |
| [`alias-analysis.i64.distinct-indices.c17.d56c27c7`](alias-analysis.i64.distinct-indices.c17.d56c27c7.c) | type=i64, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.i64.distinct-locals.c17.62d1e29a`](alias-analysis.i64.distinct-locals.c17.62d1e29a.c) | type=i64, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.i64.distinct-types.c17.2d1b2e60`](alias-analysis.i64.distinct-types.c17.2d1b2e60.c) | type=i64, shape=distinct-types | c17 | `8` |
| [`alias-analysis.i64.escaped-local.c17.380f753c`](alias-analysis.i64.escaped-local.c17.380f753c.c) | type=i64, shape=escaped-local | c17 | `8` |
| [`alias-analysis.i64.may-alias.c17.d8ec6558`](alias-analysis.i64.may-alias.c17.d8ec6558.c) | type=i64, shape=may-alias | c17 | `14` |
| [`alias-analysis.i64.restrict.c17.561a4990`](alias-analysis.i64.restrict.c17.561a4990.c) | type=i64, shape=restrict | c17 | `13` |
| [`alias-analysis.i64.union-pun.c17.baac9db3`](alias-analysis.i64.union-pun.c17.baac9db3.c) | type=i64, shape=union-pun | c17 | `7` |
| [`alias-analysis.u32.char-pointer.c17.356a7067`](alias-analysis.u32.char-pointer.c17.356a7067.c) | type=u32, shape=char-pointer | c17 | `7` |
| [`alias-analysis.u32.computed-same-index.c17.35e0a5f1`](alias-analysis.u32.computed-same-index.c17.35e0a5f1.c) | type=u32, shape=computed-same-index | c17 | `14` |
| [`alias-analysis.u32.distinct-indices.c17.ac08e171`](alias-analysis.u32.distinct-indices.c17.ac08e171.c) | type=u32, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.u32.distinct-locals.c17.b793c74a`](alias-analysis.u32.distinct-locals.c17.b793c74a.c) | type=u32, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.u32.distinct-types.c17.cd887661`](alias-analysis.u32.distinct-types.c17.cd887661.c) | type=u32, shape=distinct-types | c17 | `8` |
| [`alias-analysis.u32.escaped-local.c17.b71abb7f`](alias-analysis.u32.escaped-local.c17.b71abb7f.c) | type=u32, shape=escaped-local | c17 | `8` |
| [`alias-analysis.u32.may-alias.c17.8b2aa55b`](alias-analysis.u32.may-alias.c17.8b2aa55b.c) | type=u32, shape=may-alias | c17 | `14` |
| [`alias-analysis.u32.restrict.c17.d67e0d70`](alias-analysis.u32.restrict.c17.d67e0d70.c) | type=u32, shape=restrict | c17 | `13` |
| [`alias-analysis.u32.union-pun.c17.a8cfccbd`](alias-analysis.u32.union-pun.c17.a8cfccbd.c) | type=u32, shape=union-pun | c17 | `7` |
| [`alias-analysis.u64.char-pointer.c17.3f74031b`](alias-analysis.u64.char-pointer.c17.3f74031b.c) | type=u64, shape=char-pointer | c17 | `7` |
| [`alias-analysis.u64.computed-same-index.c17.70d5a244`](alias-analysis.u64.computed-same-index.c17.70d5a244.c) | type=u64, shape=computed-same-index | c17 | `14` |
| [`alias-analysis.u64.distinct-indices.c17.233c0752`](alias-analysis.u64.distinct-indices.c17.233c0752.c) | type=u64, shape=distinct-indices | c17 | `13` |
| [`alias-analysis.u64.distinct-locals.c17.ad8bd0d3`](alias-analysis.u64.distinct-locals.c17.ad8bd0d3.c) | type=u64, shape=distinct-locals | c17 | `13` |
| [`alias-analysis.u64.distinct-types.c17.53904136`](alias-analysis.u64.distinct-types.c17.53904136.c) | type=u64, shape=distinct-types | c17 | `8` |
| [`alias-analysis.u64.escaped-local.c17.8f66aa4a`](alias-analysis.u64.escaped-local.c17.8f66aa4a.c) | type=u64, shape=escaped-local | c17 | `8` |
| [`alias-analysis.u64.may-alias.c17.6134a36b`](alias-analysis.u64.may-alias.c17.6134a36b.c) | type=u64, shape=may-alias | c17 | `14` |
| [`alias-analysis.u64.restrict.c17.66f42ddf`](alias-analysis.u64.restrict.c17.66f42ddf.c) | type=u64, shape=restrict | c17 | `13` |
| [`alias-analysis.u64.union-pun.c17.cdff97f0`](alias-analysis.u64.union-pun.c17.cdff97f0.c) | type=u64, shape=union-pun | c17 | `7` |

