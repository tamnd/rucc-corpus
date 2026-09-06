# memory-ssa

walking back from a load to the store that answers it. Part of the global phase of the M4 plan.

28 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`memory-ssa.i32.clobbered-in-a-loop.c17.b891115b`](memory-ssa.i32.clobbered-in-a-loop.c17.b891115b.c) | type=i32, shape=clobbered-in-a-loop | c17 | `42` |
| [`memory-ssa.i32.merge-at-join.c17.01aa091d`](memory-ssa.i32.merge-at-join.c17.01aa091d.c) | type=i32, shape=merge-at-join | c17 | `42` |
| [`memory-ssa.i32.over-a-loop.c17.5aa64f4b`](memory-ssa.i32.over-a-loop.c17.5aa64f4b.c) | type=i32, shape=over-a-loop | c17 | `41` |
| [`memory-ssa.i32.skip-other-index.c17.da2ca7c2`](memory-ssa.i32.skip-other-index.c17.da2ca7c2.c) | type=i32, shape=skip-other-index | c17 | `41` |
| [`memory-ssa.i32.skip-other-object.c17.7a9dd78c`](memory-ssa.i32.skip-other-object.c17.7a9dd78c.c) | type=i32, shape=skip-other-object | c17 | `41` |
| [`memory-ssa.i32.stopped-by-call.c17.f46bd4cd`](memory-ssa.i32.stopped-by-call.c17.f46bd4cd.c) | type=i32, shape=stopped-by-call | c17 | `42` |
| [`memory-ssa.i32.stopped-by-may-alias.c17.44c9b0da`](memory-ssa.i32.stopped-by-may-alias.c17.44c9b0da.c) | type=i32, shape=stopped-by-may-alias | c17 | `42` |
| [`memory-ssa.i64.clobbered-in-a-loop.c17.51558e7c`](memory-ssa.i64.clobbered-in-a-loop.c17.51558e7c.c) | type=i64, shape=clobbered-in-a-loop | c17 | `42` |
| [`memory-ssa.i64.merge-at-join.c17.092b1e40`](memory-ssa.i64.merge-at-join.c17.092b1e40.c) | type=i64, shape=merge-at-join | c17 | `42` |
| [`memory-ssa.i64.over-a-loop.c17.d6ddd73f`](memory-ssa.i64.over-a-loop.c17.d6ddd73f.c) | type=i64, shape=over-a-loop | c17 | `41` |
| [`memory-ssa.i64.skip-other-index.c17.82f1c9da`](memory-ssa.i64.skip-other-index.c17.82f1c9da.c) | type=i64, shape=skip-other-index | c17 | `41` |
| [`memory-ssa.i64.skip-other-object.c17.c65b1f40`](memory-ssa.i64.skip-other-object.c17.c65b1f40.c) | type=i64, shape=skip-other-object | c17 | `41` |
| [`memory-ssa.i64.stopped-by-call.c17.dd9cc063`](memory-ssa.i64.stopped-by-call.c17.dd9cc063.c) | type=i64, shape=stopped-by-call | c17 | `42` |
| [`memory-ssa.i64.stopped-by-may-alias.c17.b7be52fb`](memory-ssa.i64.stopped-by-may-alias.c17.b7be52fb.c) | type=i64, shape=stopped-by-may-alias | c17 | `42` |
| [`memory-ssa.u32.clobbered-in-a-loop.c17.3bae317e`](memory-ssa.u32.clobbered-in-a-loop.c17.3bae317e.c) | type=u32, shape=clobbered-in-a-loop | c17 | `42` |
| [`memory-ssa.u32.merge-at-join.c17.e0edd57d`](memory-ssa.u32.merge-at-join.c17.e0edd57d.c) | type=u32, shape=merge-at-join | c17 | `42` |
| [`memory-ssa.u32.over-a-loop.c17.94398e47`](memory-ssa.u32.over-a-loop.c17.94398e47.c) | type=u32, shape=over-a-loop | c17 | `41` |
| [`memory-ssa.u32.skip-other-index.c17.6f3767f4`](memory-ssa.u32.skip-other-index.c17.6f3767f4.c) | type=u32, shape=skip-other-index | c17 | `41` |
| [`memory-ssa.u32.skip-other-object.c17.0ef7701e`](memory-ssa.u32.skip-other-object.c17.0ef7701e.c) | type=u32, shape=skip-other-object | c17 | `41` |
| [`memory-ssa.u32.stopped-by-call.c17.7aac3263`](memory-ssa.u32.stopped-by-call.c17.7aac3263.c) | type=u32, shape=stopped-by-call | c17 | `42` |
| [`memory-ssa.u32.stopped-by-may-alias.c17.f602c454`](memory-ssa.u32.stopped-by-may-alias.c17.f602c454.c) | type=u32, shape=stopped-by-may-alias | c17 | `42` |
| [`memory-ssa.u64.clobbered-in-a-loop.c17.5e040def`](memory-ssa.u64.clobbered-in-a-loop.c17.5e040def.c) | type=u64, shape=clobbered-in-a-loop | c17 | `42` |
| [`memory-ssa.u64.merge-at-join.c17.e3aeb1f1`](memory-ssa.u64.merge-at-join.c17.e3aeb1f1.c) | type=u64, shape=merge-at-join | c17 | `42` |
| [`memory-ssa.u64.over-a-loop.c17.e817f93d`](memory-ssa.u64.over-a-loop.c17.e817f93d.c) | type=u64, shape=over-a-loop | c17 | `41` |
| [`memory-ssa.u64.skip-other-index.c17.60f5823e`](memory-ssa.u64.skip-other-index.c17.60f5823e.c) | type=u64, shape=skip-other-index | c17 | `41` |
| [`memory-ssa.u64.skip-other-object.c17.b5c9b6ff`](memory-ssa.u64.skip-other-object.c17.b5c9b6ff.c) | type=u64, shape=skip-other-object | c17 | `41` |
| [`memory-ssa.u64.stopped-by-call.c17.4806211b`](memory-ssa.u64.stopped-by-call.c17.4806211b.c) | type=u64, shape=stopped-by-call | c17 | `42` |
| [`memory-ssa.u64.stopped-by-may-alias.c17.deee39b9`](memory-ssa.u64.stopped-by-may-alias.c17.deee39b9.c) | type=u64, shape=stopped-by-may-alias | c17 | `42` |

