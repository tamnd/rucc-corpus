# dead-store

removing a store a later store makes invisible. Part of the local phase of the M4 plan.

12 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`dead-store.i32.address-taken.c17.3293918d`](dead-store.i32.address-taken.c17.3293918d.c) | type=i32, shape=address-taken | c17 | `8` |
| [`dead-store.i32.array.c17.43206e85`](dead-store.i32.array.c17.43206e85.c) | type=i32, shape=array | c17 | `8` |
| [`dead-store.i32.local.c17.a3d48227`](dead-store.i32.local.c17.a3d48227.c) | type=i32, shape=local | c17 | `8` |
| [`dead-store.i64.address-taken.c17.919c0997`](dead-store.i64.address-taken.c17.919c0997.c) | type=i64, shape=address-taken | c17 | `8` |
| [`dead-store.i64.array.c17.274caa82`](dead-store.i64.array.c17.274caa82.c) | type=i64, shape=array | c17 | `8` |
| [`dead-store.i64.local.c17.c8ee0de3`](dead-store.i64.local.c17.c8ee0de3.c) | type=i64, shape=local | c17 | `8` |
| [`dead-store.u32.address-taken.c17.f3d05568`](dead-store.u32.address-taken.c17.f3d05568.c) | type=u32, shape=address-taken | c17 | `8` |
| [`dead-store.u32.array.c17.7e62aca1`](dead-store.u32.array.c17.7e62aca1.c) | type=u32, shape=array | c17 | `8` |
| [`dead-store.u32.local.c17.c3a401d8`](dead-store.u32.local.c17.c3a401d8.c) | type=u32, shape=local | c17 | `8` |
| [`dead-store.u64.address-taken.c17.050ee6f8`](dead-store.u64.address-taken.c17.050ee6f8.c) | type=u64, shape=address-taken | c17 | `8` |
| [`dead-store.u64.array.c17.5914e2db`](dead-store.u64.array.c17.5914e2db.c) | type=u64, shape=array | c17 | `8` |
| [`dead-store.u64.local.c17.ea554db0`](dead-store.u64.local.c17.ea554db0.c) | type=u64, shape=local | c17 | `8` |

