# call-motion

whether a call came out of a loop. Part of the interprocedural phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`call-motion.i32.a-call-under-a-false-test.c17.7e6b183c`](call-motion.i32.a-call-under-a-false-test.c17.7e6b183c.c) | type=i32, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.i32.an-argument-the-loop-changes.c17.f9ed94b7`](call-motion.i32.an-argument-the-loop-changes.c17.f9ed94b7.c) | type=i32, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.i32.reads-a-table-nothing-writes.c17.83ca5fff`](call-motion.i32.reads-a-table-nothing-writes.c17.83ca5fff.c) | type=i32, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.i32.reads-a-table-the-loop-writes.c17.f0823b43`](call-motion.i32.reads-a-table-the-loop-writes.c17.f0823b43.c) | type=i32, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.i32.touches-nothing.c17.42d98c2e`](call-motion.i32.touches-nothing.c17.42d98c2e.c) | type=i32, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.i32.writes-through-an-argument.c17.ea12d1e2`](call-motion.i32.writes-through-an-argument.c17.ea12d1e2.c) | type=i32, shape=writes-through-an-argument | c17 | `6000 1000 6` |
| [`call-motion.i64.a-call-under-a-false-test.c17.6f504db2`](call-motion.i64.a-call-under-a-false-test.c17.6f504db2.c) | type=i64, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.i64.an-argument-the-loop-changes.c17.435cfa83`](call-motion.i64.an-argument-the-loop-changes.c17.435cfa83.c) | type=i64, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.i64.reads-a-table-nothing-writes.c17.d5fdf297`](call-motion.i64.reads-a-table-nothing-writes.c17.d5fdf297.c) | type=i64, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.i64.reads-a-table-the-loop-writes.c17.59711f20`](call-motion.i64.reads-a-table-the-loop-writes.c17.59711f20.c) | type=i64, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.i64.touches-nothing.c17.80734786`](call-motion.i64.touches-nothing.c17.80734786.c) | type=i64, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.i64.writes-through-an-argument.c17.bd57a061`](call-motion.i64.writes-through-an-argument.c17.bd57a061.c) | type=i64, shape=writes-through-an-argument | c17 | `6000 1000 6` |
| [`call-motion.u32.a-call-under-a-false-test.c17.139be797`](call-motion.u32.a-call-under-a-false-test.c17.139be797.c) | type=u32, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.u32.an-argument-the-loop-changes.c17.ff53c5ab`](call-motion.u32.an-argument-the-loop-changes.c17.ff53c5ab.c) | type=u32, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.u32.reads-a-table-nothing-writes.c17.7f6921be`](call-motion.u32.reads-a-table-nothing-writes.c17.7f6921be.c) | type=u32, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.u32.reads-a-table-the-loop-writes.c17.3b2720c3`](call-motion.u32.reads-a-table-the-loop-writes.c17.3b2720c3.c) | type=u32, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.u32.touches-nothing.c17.44576a40`](call-motion.u32.touches-nothing.c17.44576a40.c) | type=u32, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.u32.writes-through-an-argument.c17.eafdb22f`](call-motion.u32.writes-through-an-argument.c17.eafdb22f.c) | type=u32, shape=writes-through-an-argument | c17 | `6000 1000 6` |
| [`call-motion.u64.a-call-under-a-false-test.c17.edfd5743`](call-motion.u64.a-call-under-a-false-test.c17.edfd5743.c) | type=u64, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.u64.an-argument-the-loop-changes.c17.17f01555`](call-motion.u64.an-argument-the-loop-changes.c17.17f01555.c) | type=u64, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.u64.reads-a-table-nothing-writes.c17.c65c20af`](call-motion.u64.reads-a-table-nothing-writes.c17.c65c20af.c) | type=u64, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.u64.reads-a-table-the-loop-writes.c17.26133a93`](call-motion.u64.reads-a-table-the-loop-writes.c17.26133a93.c) | type=u64, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.u64.touches-nothing.c17.80dcc81c`](call-motion.u64.touches-nothing.c17.80dcc81c.c) | type=u64, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.u64.writes-through-an-argument.c17.eaf7f1fb`](call-motion.u64.writes-through-an-argument.c17.eaf7f1fb.c) | type=u64, shape=writes-through-an-argument | c17 | `6000 1000 6` |

