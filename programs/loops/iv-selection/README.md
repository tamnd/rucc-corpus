# iv-selection

which induction variables a loop is left with, and how many. Part of the loops phase of the M4 plan.

28 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`iv-selection.i32.awkward-stride.c17.2a907fc2`](iv-selection.i32.awkward-stride.c17.2a907fc2.c) | type=i32, shape=awkward-stride | c17 | `45` |
| [`iv-selection.i32.counter-dead-after.c17.25fbec4f`](iv-selection.i32.counter-dead-after.c17.25fbec4f.c) | type=i32, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.i32.counter-live-after.c17.db656536`](iv-selection.i32.counter-live-after.c17.db656536.c) | type=i32, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.i32.grouped-offsets.c17.2a327766`](iv-selection.i32.grouped-offsets.c17.2a327766.c) | type=i32, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.i32.legal-scale.c17.373c76b8`](iv-selection.i32.legal-scale.c17.373c76b8.c) | type=i32, shape=legal-scale | c17 | `24` |
| [`iv-selection.i32.many-walks.c17.64127297`](iv-selection.i32.many-walks.c17.64127297.c) | type=i32, shape=many-walks | c17 | `2520` |
| [`iv-selection.i32.two-arrays.c17.0de541c2`](iv-selection.i32.two-arrays.c17.0de541c2.c) | type=i32, shape=two-arrays | c17 | `360` |
| [`iv-selection.i64.awkward-stride.c17.50d5eee6`](iv-selection.i64.awkward-stride.c17.50d5eee6.c) | type=i64, shape=awkward-stride | c17 | `45` |
| [`iv-selection.i64.counter-dead-after.c17.3a0f9645`](iv-selection.i64.counter-dead-after.c17.3a0f9645.c) | type=i64, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.i64.counter-live-after.c17.0337bebd`](iv-selection.i64.counter-live-after.c17.0337bebd.c) | type=i64, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.i64.grouped-offsets.c17.033ec82f`](iv-selection.i64.grouped-offsets.c17.033ec82f.c) | type=i64, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.i64.legal-scale.c17.55ff6a0d`](iv-selection.i64.legal-scale.c17.55ff6a0d.c) | type=i64, shape=legal-scale | c17 | `24` |
| [`iv-selection.i64.many-walks.c17.df982404`](iv-selection.i64.many-walks.c17.df982404.c) | type=i64, shape=many-walks | c17 | `2520` |
| [`iv-selection.i64.two-arrays.c17.47a40674`](iv-selection.i64.two-arrays.c17.47a40674.c) | type=i64, shape=two-arrays | c17 | `360` |
| [`iv-selection.u32.awkward-stride.c17.d8eefe43`](iv-selection.u32.awkward-stride.c17.d8eefe43.c) | type=u32, shape=awkward-stride | c17 | `45` |
| [`iv-selection.u32.counter-dead-after.c17.d50db25b`](iv-selection.u32.counter-dead-after.c17.d50db25b.c) | type=u32, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.u32.counter-live-after.c17.f2477c5e`](iv-selection.u32.counter-live-after.c17.f2477c5e.c) | type=u32, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.u32.grouped-offsets.c17.b9a6ddd1`](iv-selection.u32.grouped-offsets.c17.b9a6ddd1.c) | type=u32, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.u32.legal-scale.c17.58c0ae30`](iv-selection.u32.legal-scale.c17.58c0ae30.c) | type=u32, shape=legal-scale | c17 | `24` |
| [`iv-selection.u32.many-walks.c17.e9f60216`](iv-selection.u32.many-walks.c17.e9f60216.c) | type=u32, shape=many-walks | c17 | `2520` |
| [`iv-selection.u32.two-arrays.c17.61149f0a`](iv-selection.u32.two-arrays.c17.61149f0a.c) | type=u32, shape=two-arrays | c17 | `360` |
| [`iv-selection.u64.awkward-stride.c17.1d354524`](iv-selection.u64.awkward-stride.c17.1d354524.c) | type=u64, shape=awkward-stride | c17 | `45` |
| [`iv-selection.u64.counter-dead-after.c17.461e67f9`](iv-selection.u64.counter-dead-after.c17.461e67f9.c) | type=u64, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.u64.counter-live-after.c17.22dd66be`](iv-selection.u64.counter-live-after.c17.22dd66be.c) | type=u64, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.u64.grouped-offsets.c17.6b3f5911`](iv-selection.u64.grouped-offsets.c17.6b3f5911.c) | type=u64, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.u64.legal-scale.c17.d2f9c3c1`](iv-selection.u64.legal-scale.c17.d2f9c3c1.c) | type=u64, shape=legal-scale | c17 | `24` |
| [`iv-selection.u64.many-walks.c17.624b2f10`](iv-selection.u64.many-walks.c17.624b2f10.c) | type=u64, shape=many-walks | c17 | `2520` |
| [`iv-selection.u64.two-arrays.c17.b66cc989`](iv-selection.u64.two-arrays.c17.b66cc989.c) | type=u64, shape=two-arrays | c17 | `360` |

