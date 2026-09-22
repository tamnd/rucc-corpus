# loop-deletion

deleting a loop whose body nobody reads. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-deletion.i32.100.known.read-back.c17.b25952a0`](loop-deletion.i32.100.known.read-back.c17.b25952a0.c) | type=i32, trips=100, bound=known, shape=read-back | c17 | `28 300` |
| [`loop-deletion.i32.100.known.unread.c17.72e62533`](loop-deletion.i32.100.known.unread.c17.72e62533.c) | type=i32, trips=100, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i32.100.unknown.read-back.c17.110b6776`](loop-deletion.i32.100.unknown.read-back.c17.110b6776.c) | type=i32, trips=100, bound=unknown, shape=read-back | c17 | `28 300` |
| [`loop-deletion.i32.100.unknown.unread.c17.4b1a0cd0`](loop-deletion.i32.100.unknown.unread.c17.4b1a0cd0.c) | type=i32, trips=100, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i32.1000000.known.read-back.c17.361ade6c`](loop-deletion.i32.1000000.known.read-back.c17.361ade6c.c) | type=i32, trips=1000000, bound=known, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.i32.1000000.known.unread.c17.c47e5b78`](loop-deletion.i32.1000000.known.unread.c17.c47e5b78.c) | type=i32, trips=1000000, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i32.1000000.unknown.read-back.c17.33a42d92`](loop-deletion.i32.1000000.unknown.read-back.c17.33a42d92.c) | type=i32, trips=1000000, bound=unknown, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.i32.1000000.unknown.unread.c17.45431027`](loop-deletion.i32.1000000.unknown.unread.c17.45431027.c) | type=i32, trips=1000000, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i32.16.known.read-back.c17.a4e99d4f`](loop-deletion.i32.16.known.read-back.c17.a4e99d4f.c) | type=i32, trips=16, bound=known, shape=read-back | c17 | `28 48` |
| [`loop-deletion.i32.16.known.unread.c17.f3d691df`](loop-deletion.i32.16.known.unread.c17.f3d691df.c) | type=i32, trips=16, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i32.16.unknown.read-back.c17.aaae9f1c`](loop-deletion.i32.16.unknown.read-back.c17.aaae9f1c.c) | type=i32, trips=16, bound=unknown, shape=read-back | c17 | `28 48` |
| [`loop-deletion.i32.16.unknown.unread.c17.2076453b`](loop-deletion.i32.16.unknown.unread.c17.2076453b.c) | type=i32, trips=16, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i32.4.known.read-back.c17.f2ac8312`](loop-deletion.i32.4.known.read-back.c17.f2ac8312.c) | type=i32, trips=4, bound=known, shape=read-back | c17 | `28 12` |
| [`loop-deletion.i32.4.known.unread.c17.e372096b`](loop-deletion.i32.4.known.unread.c17.e372096b.c) | type=i32, trips=4, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i32.4.unknown.read-back.c17.bde3fd15`](loop-deletion.i32.4.unknown.read-back.c17.bde3fd15.c) | type=i32, trips=4, bound=unknown, shape=read-back | c17 | `28 12` |
| [`loop-deletion.i32.4.unknown.unread.c17.16fc8955`](loop-deletion.i32.4.unknown.unread.c17.16fc8955.c) | type=i32, trips=4, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i64.100.known.read-back.c17.a2c80345`](loop-deletion.i64.100.known.read-back.c17.a2c80345.c) | type=i64, trips=100, bound=known, shape=read-back | c17 | `28 300` |
| [`loop-deletion.i64.100.known.unread.c17.50819306`](loop-deletion.i64.100.known.unread.c17.50819306.c) | type=i64, trips=100, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i64.100.unknown.read-back.c17.1da93d7d`](loop-deletion.i64.100.unknown.read-back.c17.1da93d7d.c) | type=i64, trips=100, bound=unknown, shape=read-back | c17 | `28 300` |
| [`loop-deletion.i64.100.unknown.unread.c17.77c40732`](loop-deletion.i64.100.unknown.unread.c17.77c40732.c) | type=i64, trips=100, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i64.1000000.known.read-back.c17.3a85447c`](loop-deletion.i64.1000000.known.read-back.c17.3a85447c.c) | type=i64, trips=1000000, bound=known, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.i64.1000000.known.unread.c17.5d2e4c6f`](loop-deletion.i64.1000000.known.unread.c17.5d2e4c6f.c) | type=i64, trips=1000000, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i64.1000000.unknown.read-back.c17.0e2520f2`](loop-deletion.i64.1000000.unknown.read-back.c17.0e2520f2.c) | type=i64, trips=1000000, bound=unknown, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.i64.1000000.unknown.unread.c17.7660f445`](loop-deletion.i64.1000000.unknown.unread.c17.7660f445.c) | type=i64, trips=1000000, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i64.16.known.read-back.c17.f65543f0`](loop-deletion.i64.16.known.read-back.c17.f65543f0.c) | type=i64, trips=16, bound=known, shape=read-back | c17 | `28 48` |
| [`loop-deletion.i64.16.known.unread.c17.cdafce61`](loop-deletion.i64.16.known.unread.c17.cdafce61.c) | type=i64, trips=16, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i64.16.unknown.read-back.c17.654312da`](loop-deletion.i64.16.unknown.read-back.c17.654312da.c) | type=i64, trips=16, bound=unknown, shape=read-back | c17 | `28 48` |
| [`loop-deletion.i64.16.unknown.unread.c17.faf51579`](loop-deletion.i64.16.unknown.unread.c17.faf51579.c) | type=i64, trips=16, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.i64.4.known.read-back.c17.93deaddb`](loop-deletion.i64.4.known.read-back.c17.93deaddb.c) | type=i64, trips=4, bound=known, shape=read-back | c17 | `28 12` |
| [`loop-deletion.i64.4.known.unread.c17.cf400407`](loop-deletion.i64.4.known.unread.c17.cf400407.c) | type=i64, trips=4, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.i64.4.unknown.read-back.c17.c26687a8`](loop-deletion.i64.4.unknown.read-back.c17.c26687a8.c) | type=i64, trips=4, bound=unknown, shape=read-back | c17 | `28 12` |
| [`loop-deletion.i64.4.unknown.unread.c17.12e94094`](loop-deletion.i64.4.unknown.unread.c17.12e94094.c) | type=i64, trips=4, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u32.100.known.read-back.c17.42509324`](loop-deletion.u32.100.known.read-back.c17.42509324.c) | type=u32, trips=100, bound=known, shape=read-back | c17 | `28 300` |
| [`loop-deletion.u32.100.known.unread.c17.d302ff75`](loop-deletion.u32.100.known.unread.c17.d302ff75.c) | type=u32, trips=100, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u32.100.unknown.read-back.c17.05811723`](loop-deletion.u32.100.unknown.read-back.c17.05811723.c) | type=u32, trips=100, bound=unknown, shape=read-back | c17 | `28 300` |
| [`loop-deletion.u32.100.unknown.unread.c17.e93d50fa`](loop-deletion.u32.100.unknown.unread.c17.e93d50fa.c) | type=u32, trips=100, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u32.1000000.known.read-back.c17.48d3211f`](loop-deletion.u32.1000000.known.read-back.c17.48d3211f.c) | type=u32, trips=1000000, bound=known, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.u32.1000000.known.unread.c17.946128ab`](loop-deletion.u32.1000000.known.unread.c17.946128ab.c) | type=u32, trips=1000000, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u32.1000000.unknown.read-back.c17.c24d44d3`](loop-deletion.u32.1000000.unknown.read-back.c17.c24d44d3.c) | type=u32, trips=1000000, bound=unknown, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.u32.1000000.unknown.unread.c17.23d5f6fa`](loop-deletion.u32.1000000.unknown.unread.c17.23d5f6fa.c) | type=u32, trips=1000000, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u32.16.known.read-back.c17.26e50329`](loop-deletion.u32.16.known.read-back.c17.26e50329.c) | type=u32, trips=16, bound=known, shape=read-back | c17 | `28 48` |
| [`loop-deletion.u32.16.known.unread.c17.17486948`](loop-deletion.u32.16.known.unread.c17.17486948.c) | type=u32, trips=16, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u32.16.unknown.read-back.c17.3b7babf3`](loop-deletion.u32.16.unknown.read-back.c17.3b7babf3.c) | type=u32, trips=16, bound=unknown, shape=read-back | c17 | `28 48` |
| [`loop-deletion.u32.16.unknown.unread.c17.a103e57f`](loop-deletion.u32.16.unknown.unread.c17.a103e57f.c) | type=u32, trips=16, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u32.4.known.read-back.c17.49659428`](loop-deletion.u32.4.known.read-back.c17.49659428.c) | type=u32, trips=4, bound=known, shape=read-back | c17 | `28 12` |
| [`loop-deletion.u32.4.known.unread.c17.8d0a6c44`](loop-deletion.u32.4.known.unread.c17.8d0a6c44.c) | type=u32, trips=4, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u32.4.unknown.read-back.c17.9f1a6e71`](loop-deletion.u32.4.unknown.read-back.c17.9f1a6e71.c) | type=u32, trips=4, bound=unknown, shape=read-back | c17 | `28 12` |
| [`loop-deletion.u32.4.unknown.unread.c17.b69c975b`](loop-deletion.u32.4.unknown.unread.c17.b69c975b.c) | type=u32, trips=4, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u64.100.known.read-back.c17.9f30cdd1`](loop-deletion.u64.100.known.read-back.c17.9f30cdd1.c) | type=u64, trips=100, bound=known, shape=read-back | c17 | `28 300` |
| [`loop-deletion.u64.100.known.unread.c17.7eecf82f`](loop-deletion.u64.100.known.unread.c17.7eecf82f.c) | type=u64, trips=100, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u64.100.unknown.read-back.c17.39db1cb0`](loop-deletion.u64.100.unknown.read-back.c17.39db1cb0.c) | type=u64, trips=100, bound=unknown, shape=read-back | c17 | `28 300` |
| [`loop-deletion.u64.100.unknown.unread.c17.dc11fb63`](loop-deletion.u64.100.unknown.unread.c17.dc11fb63.c) | type=u64, trips=100, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u64.1000000.known.read-back.c17.694e9aec`](loop-deletion.u64.1000000.known.read-back.c17.694e9aec.c) | type=u64, trips=1000000, bound=known, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.u64.1000000.known.unread.c17.a5558af2`](loop-deletion.u64.1000000.known.unread.c17.a5558af2.c) | type=u64, trips=1000000, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u64.1000000.unknown.read-back.c17.167c7443`](loop-deletion.u64.1000000.unknown.read-back.c17.167c7443.c) | type=u64, trips=1000000, bound=unknown, shape=read-back | c17 | `28 3000000` |
| [`loop-deletion.u64.1000000.unknown.unread.c17.43255c12`](loop-deletion.u64.1000000.unknown.unread.c17.43255c12.c) | type=u64, trips=1000000, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u64.16.known.read-back.c17.8691c529`](loop-deletion.u64.16.known.read-back.c17.8691c529.c) | type=u64, trips=16, bound=known, shape=read-back | c17 | `28 48` |
| [`loop-deletion.u64.16.known.unread.c17.3a2f2f81`](loop-deletion.u64.16.known.unread.c17.3a2f2f81.c) | type=u64, trips=16, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u64.16.unknown.read-back.c17.85ada9d5`](loop-deletion.u64.16.unknown.read-back.c17.85ada9d5.c) | type=u64, trips=16, bound=unknown, shape=read-back | c17 | `28 48` |
| [`loop-deletion.u64.16.unknown.unread.c17.187de5cb`](loop-deletion.u64.16.unknown.unread.c17.187de5cb.c) | type=u64, trips=16, bound=unknown, shape=unread | c17 | `28` |
| [`loop-deletion.u64.4.known.read-back.c17.bd42f62f`](loop-deletion.u64.4.known.read-back.c17.bd42f62f.c) | type=u64, trips=4, bound=known, shape=read-back | c17 | `28 12` |
| [`loop-deletion.u64.4.known.unread.c17.9920ba4f`](loop-deletion.u64.4.known.unread.c17.9920ba4f.c) | type=u64, trips=4, bound=known, shape=unread | c17 | `28` |
| [`loop-deletion.u64.4.unknown.read-back.c17.9f899d1f`](loop-deletion.u64.4.unknown.read-back.c17.9f899d1f.c) | type=u64, trips=4, bound=unknown, shape=read-back | c17 | `28 12` |
| [`loop-deletion.u64.4.unknown.unread.c17.5bdc1072`](loop-deletion.u64.4.unknown.unread.c17.5bdc1072.c) | type=u64, trips=4, bound=unknown, shape=unread | c17 | `28` |

