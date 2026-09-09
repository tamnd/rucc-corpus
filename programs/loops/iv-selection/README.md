# iv-selection

which induction variables a loop is left with, and how many. Part of the loops phase of the M4 plan.

28 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`iv-selection.i32.awkward-stride.c17.a0d5f4ad`](iv-selection.i32.awkward-stride.c17.a0d5f4ad.c) | type=i32, shape=awkward-stride | c17 | `45` |
| [`iv-selection.i32.counter-dead-after.c17.4272b8f5`](iv-selection.i32.counter-dead-after.c17.4272b8f5.c) | type=i32, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.i32.counter-live-after.c17.b87c9109`](iv-selection.i32.counter-live-after.c17.b87c9109.c) | type=i32, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.i32.grouped-offsets.c17.ce171156`](iv-selection.i32.grouped-offsets.c17.ce171156.c) | type=i32, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.i32.legal-scale.c17.c4efc1a2`](iv-selection.i32.legal-scale.c17.c4efc1a2.c) | type=i32, shape=legal-scale | c17 | `24` |
| [`iv-selection.i32.many-walks.c17.d786560b`](iv-selection.i32.many-walks.c17.d786560b.c) | type=i32, shape=many-walks | c17 | `2520` |
| [`iv-selection.i32.two-arrays.c17.007fff5b`](iv-selection.i32.two-arrays.c17.007fff5b.c) | type=i32, shape=two-arrays | c17 | `360` |
| [`iv-selection.i64.awkward-stride.c17.a8e426fa`](iv-selection.i64.awkward-stride.c17.a8e426fa.c) | type=i64, shape=awkward-stride | c17 | `45` |
| [`iv-selection.i64.counter-dead-after.c17.21aa6e97`](iv-selection.i64.counter-dead-after.c17.21aa6e97.c) | type=i64, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.i64.counter-live-after.c17.d4a6c896`](iv-selection.i64.counter-live-after.c17.d4a6c896.c) | type=i64, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.i64.grouped-offsets.c17.ff7b78a3`](iv-selection.i64.grouped-offsets.c17.ff7b78a3.c) | type=i64, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.i64.legal-scale.c17.cbafa08d`](iv-selection.i64.legal-scale.c17.cbafa08d.c) | type=i64, shape=legal-scale | c17 | `24` |
| [`iv-selection.i64.many-walks.c17.7e3501d6`](iv-selection.i64.many-walks.c17.7e3501d6.c) | type=i64, shape=many-walks | c17 | `2520` |
| [`iv-selection.i64.two-arrays.c17.447bd5a3`](iv-selection.i64.two-arrays.c17.447bd5a3.c) | type=i64, shape=two-arrays | c17 | `360` |
| [`iv-selection.u32.awkward-stride.c17.e5b28f50`](iv-selection.u32.awkward-stride.c17.e5b28f50.c) | type=u32, shape=awkward-stride | c17 | `45` |
| [`iv-selection.u32.counter-dead-after.c17.54a36c5c`](iv-selection.u32.counter-dead-after.c17.54a36c5c.c) | type=u32, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.u32.counter-live-after.c17.aac97e3d`](iv-selection.u32.counter-live-after.c17.aac97e3d.c) | type=u32, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.u32.grouped-offsets.c17.4619be7a`](iv-selection.u32.grouped-offsets.c17.4619be7a.c) | type=u32, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.u32.legal-scale.c17.a0c240f6`](iv-selection.u32.legal-scale.c17.a0c240f6.c) | type=u32, shape=legal-scale | c17 | `24` |
| [`iv-selection.u32.many-walks.c17.b419fecd`](iv-selection.u32.many-walks.c17.b419fecd.c) | type=u32, shape=many-walks | c17 | `2520` |
| [`iv-selection.u32.two-arrays.c17.d0394a02`](iv-selection.u32.two-arrays.c17.d0394a02.c) | type=u32, shape=two-arrays | c17 | `360` |
| [`iv-selection.u64.awkward-stride.c17.e36c48e0`](iv-selection.u64.awkward-stride.c17.e36c48e0.c) | type=u64, shape=awkward-stride | c17 | `45` |
| [`iv-selection.u64.counter-dead-after.c17.25b78515`](iv-selection.u64.counter-dead-after.c17.25b78515.c) | type=u64, shape=counter-dead-after | c17 | `120` |
| [`iv-selection.u64.counter-live-after.c17.6d3c81d7`](iv-selection.u64.counter-live-after.c17.6d3c81d7.c) | type=u64, shape=counter-live-after | c17 | `120 16` |
| [`iv-selection.u64.grouped-offsets.c17.ff78bff9`](iv-selection.u64.grouped-offsets.c17.ff78bff9.c) | type=u64, shape=grouped-offsets | c17 | `408` |
| [`iv-selection.u64.legal-scale.c17.8d70ffcb`](iv-selection.u64.legal-scale.c17.8d70ffcb.c) | type=u64, shape=legal-scale | c17 | `24` |
| [`iv-selection.u64.many-walks.c17.c706ba99`](iv-selection.u64.many-walks.c17.c706ba99.c) | type=u64, shape=many-walks | c17 | `2520` |
| [`iv-selection.u64.two-arrays.c17.585e4b2e`](iv-selection.u64.two-arrays.c17.585e4b2e.c) | type=u64, shape=two-arrays | c17 | `360` |

