# copy-propagation

propagating a copy so the copy becomes dead. Part of the global phase of the M4 plan.

12 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`copy-propagation.i32.2.c17.d1db2169`](copy-propagation.i32.2.c17.d1db2169.c) | type=i32, depth=2 | c17 | `12` |
| [`copy-propagation.i32.32.c17.0ee6382e`](copy-propagation.i32.32.c17.0ee6382e.c) | type=i32, depth=32 | c17 | `12` |
| [`copy-propagation.i32.8.c17.b24108a7`](copy-propagation.i32.8.c17.b24108a7.c) | type=i32, depth=8 | c17 | `12` |
| [`copy-propagation.i64.2.c17.ee55573a`](copy-propagation.i64.2.c17.ee55573a.c) | type=i64, depth=2 | c17 | `12` |
| [`copy-propagation.i64.32.c17.3e2c286f`](copy-propagation.i64.32.c17.3e2c286f.c) | type=i64, depth=32 | c17 | `12` |
| [`copy-propagation.i64.8.c17.6da93ae1`](copy-propagation.i64.8.c17.6da93ae1.c) | type=i64, depth=8 | c17 | `12` |
| [`copy-propagation.u32.2.c17.b224cec8`](copy-propagation.u32.2.c17.b224cec8.c) | type=u32, depth=2 | c17 | `12` |
| [`copy-propagation.u32.32.c17.02065834`](copy-propagation.u32.32.c17.02065834.c) | type=u32, depth=32 | c17 | `12` |
| [`copy-propagation.u32.8.c17.fdab69ee`](copy-propagation.u32.8.c17.fdab69ee.c) | type=u32, depth=8 | c17 | `12` |
| [`copy-propagation.u64.2.c17.9f562f8a`](copy-propagation.u64.2.c17.9f562f8a.c) | type=u64, depth=2 | c17 | `12` |
| [`copy-propagation.u64.32.c17.650798f6`](copy-propagation.u64.32.c17.650798f6.c) | type=u64, depth=32 | c17 | `12` |
| [`copy-propagation.u64.8.c17.36d57db0`](copy-propagation.u64.8.c17.36d57db0.c) | type=u64, depth=8 | c17 | `12` |

