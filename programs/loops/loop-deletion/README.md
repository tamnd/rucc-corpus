# loop-deletion

deleting a loop whose body nobody reads. Part of the loops phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-deletion.i32.100.known.c17.0e4c8bec`](loop-deletion.i32.100.known.c17.0e4c8bec.c) | type=i32, trips=100, bound=known | c17 | `4950` |
| [`loop-deletion.i32.100.unknown.c17.f6cb9d95`](loop-deletion.i32.100.unknown.c17.f6cb9d95.c) | type=i32, trips=100, bound=unknown | c17 | `4950` |
| [`loop-deletion.i32.16.known.c17.3f27ffaf`](loop-deletion.i32.16.known.c17.3f27ffaf.c) | type=i32, trips=16, bound=known | c17 | `120` |
| [`loop-deletion.i32.16.unknown.c17.8eb8566e`](loop-deletion.i32.16.unknown.c17.8eb8566e.c) | type=i32, trips=16, bound=unknown | c17 | `120` |
| [`loop-deletion.i32.4.known.c17.378b41ff`](loop-deletion.i32.4.known.c17.378b41ff.c) | type=i32, trips=4, bound=known | c17 | `6` |
| [`loop-deletion.i32.4.unknown.c17.77449fb3`](loop-deletion.i32.4.unknown.c17.77449fb3.c) | type=i32, trips=4, bound=unknown | c17 | `6` |
| [`loop-deletion.i64.100.known.c17.91752697`](loop-deletion.i64.100.known.c17.91752697.c) | type=i64, trips=100, bound=known | c17 | `4950` |
| [`loop-deletion.i64.100.unknown.c17.9cfe1c50`](loop-deletion.i64.100.unknown.c17.9cfe1c50.c) | type=i64, trips=100, bound=unknown | c17 | `4950` |
| [`loop-deletion.i64.16.known.c17.455cec31`](loop-deletion.i64.16.known.c17.455cec31.c) | type=i64, trips=16, bound=known | c17 | `120` |
| [`loop-deletion.i64.16.unknown.c17.7018734c`](loop-deletion.i64.16.unknown.c17.7018734c.c) | type=i64, trips=16, bound=unknown | c17 | `120` |
| [`loop-deletion.i64.4.known.c17.8cb4eff1`](loop-deletion.i64.4.known.c17.8cb4eff1.c) | type=i64, trips=4, bound=known | c17 | `6` |
| [`loop-deletion.i64.4.unknown.c17.c79da8fd`](loop-deletion.i64.4.unknown.c17.c79da8fd.c) | type=i64, trips=4, bound=unknown | c17 | `6` |
| [`loop-deletion.u32.100.known.c17.90e2d607`](loop-deletion.u32.100.known.c17.90e2d607.c) | type=u32, trips=100, bound=known | c17 | `4950` |
| [`loop-deletion.u32.100.unknown.c17.458bd5db`](loop-deletion.u32.100.unknown.c17.458bd5db.c) | type=u32, trips=100, bound=unknown | c17 | `4950` |
| [`loop-deletion.u32.16.known.c17.f5f77199`](loop-deletion.u32.16.known.c17.f5f77199.c) | type=u32, trips=16, bound=known | c17 | `120` |
| [`loop-deletion.u32.16.unknown.c17.67926070`](loop-deletion.u32.16.unknown.c17.67926070.c) | type=u32, trips=16, bound=unknown | c17 | `120` |
| [`loop-deletion.u32.4.known.c17.ec3198ff`](loop-deletion.u32.4.known.c17.ec3198ff.c) | type=u32, trips=4, bound=known | c17 | `6` |
| [`loop-deletion.u32.4.unknown.c17.03da902d`](loop-deletion.u32.4.unknown.c17.03da902d.c) | type=u32, trips=4, bound=unknown | c17 | `6` |
| [`loop-deletion.u64.100.known.c17.5906e9c9`](loop-deletion.u64.100.known.c17.5906e9c9.c) | type=u64, trips=100, bound=known | c17 | `4950` |
| [`loop-deletion.u64.100.unknown.c17.13f17cfb`](loop-deletion.u64.100.unknown.c17.13f17cfb.c) | type=u64, trips=100, bound=unknown | c17 | `4950` |
| [`loop-deletion.u64.16.known.c17.0c8114d6`](loop-deletion.u64.16.known.c17.0c8114d6.c) | type=u64, trips=16, bound=known | c17 | `120` |
| [`loop-deletion.u64.16.unknown.c17.30ab6b72`](loop-deletion.u64.16.unknown.c17.30ab6b72.c) | type=u64, trips=16, bound=unknown | c17 | `120` |
| [`loop-deletion.u64.4.known.c17.768d4433`](loop-deletion.u64.4.known.c17.768d4433.c) | type=u64, trips=4, bound=known | c17 | `6` |
| [`loop-deletion.u64.4.unknown.c17.a70c9d0b`](loop-deletion.u64.4.unknown.c17.a70c9d0b.c) | type=u64, trips=4, bound=unknown | c17 | `6` |

