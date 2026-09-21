# call-motion

whether a call came out of a loop. Part of the interprocedural phase of the M4 plan.

36 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | axes | dialect | must print |
|---|---|---|---|---|
| [`call-motion.i32.a-call-under-a-false-test.c17.7e6b183c`](call-motion.i32.a-call-under-a-false-test.c17.7e6b183c.c) | nothing, it is one file | type=i32, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.i32.an-argument-the-loop-changes.c17.f9ed94b7`](call-motion.i32.an-argument-the-loop-changes.c17.f9ed94b7.c) | nothing, it is one file | type=i32, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.i32.another-unit-is-silent.c17.6652d4fc`](call-motion.i32.another-unit-is-silent.c17.6652d4fc.c) | [`helper`](call-motion.i32.another-unit-is-silent.c17.6652d4fc.helper.c) | type=i32, shape=another-unit-is-silent | c17 | `19000 250` |
| [`call-motion.i32.another-unit-promises-const.c17.d4a2cbaa`](call-motion.i32.another-unit-promises-const.c17.d4a2cbaa.c) | [`helper`](call-motion.i32.another-unit-promises-const.c17.d4a2cbaa.helper.c) | type=i32, shape=another-unit-promises-const | c17 | `19000 250` |
| [`call-motion.i32.another-unit-writes-a-counter.c17.c5bfb439`](call-motion.i32.another-unit-writes-a-counter.c17.c5bfb439.c) | [`helper`](call-motion.i32.another-unit-writes-a-counter.c17.c5bfb439.helper.c) | type=i32, shape=another-unit-writes-a-counter | c17 | `12000 1000` |
| [`call-motion.i32.reads-a-table-nothing-writes.c17.83ca5fff`](call-motion.i32.reads-a-table-nothing-writes.c17.83ca5fff.c) | nothing, it is one file | type=i32, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.i32.reads-a-table-the-loop-writes.c17.f0823b43`](call-motion.i32.reads-a-table-the-loop-writes.c17.f0823b43.c) | nothing, it is one file | type=i32, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.i32.touches-nothing.c17.42d98c2e`](call-motion.i32.touches-nothing.c17.42d98c2e.c) | nothing, it is one file | type=i32, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.i32.writes-through-an-argument.c17.ea12d1e2`](call-motion.i32.writes-through-an-argument.c17.ea12d1e2.c) | nothing, it is one file | type=i32, shape=writes-through-an-argument | c17 | `6000 1000 6` |
| [`call-motion.i64.a-call-under-a-false-test.c17.6f504db2`](call-motion.i64.a-call-under-a-false-test.c17.6f504db2.c) | nothing, it is one file | type=i64, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.i64.an-argument-the-loop-changes.c17.435cfa83`](call-motion.i64.an-argument-the-loop-changes.c17.435cfa83.c) | nothing, it is one file | type=i64, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.i64.another-unit-is-silent.c17.0c6b902a`](call-motion.i64.another-unit-is-silent.c17.0c6b902a.c) | [`helper`](call-motion.i64.another-unit-is-silent.c17.0c6b902a.helper.c) | type=i64, shape=another-unit-is-silent | c17 | `19000 250` |
| [`call-motion.i64.another-unit-promises-const.c17.260d2ca5`](call-motion.i64.another-unit-promises-const.c17.260d2ca5.c) | [`helper`](call-motion.i64.another-unit-promises-const.c17.260d2ca5.helper.c) | type=i64, shape=another-unit-promises-const | c17 | `19000 250` |
| [`call-motion.i64.another-unit-writes-a-counter.c17.7b3c9687`](call-motion.i64.another-unit-writes-a-counter.c17.7b3c9687.c) | [`helper`](call-motion.i64.another-unit-writes-a-counter.c17.7b3c9687.helper.c) | type=i64, shape=another-unit-writes-a-counter | c17 | `12000 1000` |
| [`call-motion.i64.reads-a-table-nothing-writes.c17.d5fdf297`](call-motion.i64.reads-a-table-nothing-writes.c17.d5fdf297.c) | nothing, it is one file | type=i64, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.i64.reads-a-table-the-loop-writes.c17.59711f20`](call-motion.i64.reads-a-table-the-loop-writes.c17.59711f20.c) | nothing, it is one file | type=i64, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.i64.touches-nothing.c17.80734786`](call-motion.i64.touches-nothing.c17.80734786.c) | nothing, it is one file | type=i64, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.i64.writes-through-an-argument.c17.bd57a061`](call-motion.i64.writes-through-an-argument.c17.bd57a061.c) | nothing, it is one file | type=i64, shape=writes-through-an-argument | c17 | `6000 1000 6` |
| [`call-motion.u32.a-call-under-a-false-test.c17.139be797`](call-motion.u32.a-call-under-a-false-test.c17.139be797.c) | nothing, it is one file | type=u32, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.u32.an-argument-the-loop-changes.c17.ff53c5ab`](call-motion.u32.an-argument-the-loop-changes.c17.ff53c5ab.c) | nothing, it is one file | type=u32, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.u32.another-unit-is-silent.c17.d4631d9f`](call-motion.u32.another-unit-is-silent.c17.d4631d9f.c) | [`helper`](call-motion.u32.another-unit-is-silent.c17.d4631d9f.helper.c) | type=u32, shape=another-unit-is-silent | c17 | `19000 250` |
| [`call-motion.u32.another-unit-promises-const.c17.8e9e33c0`](call-motion.u32.another-unit-promises-const.c17.8e9e33c0.c) | [`helper`](call-motion.u32.another-unit-promises-const.c17.8e9e33c0.helper.c) | type=u32, shape=another-unit-promises-const | c17 | `19000 250` |
| [`call-motion.u32.another-unit-writes-a-counter.c17.fd89e44c`](call-motion.u32.another-unit-writes-a-counter.c17.fd89e44c.c) | [`helper`](call-motion.u32.another-unit-writes-a-counter.c17.fd89e44c.helper.c) | type=u32, shape=another-unit-writes-a-counter | c17 | `12000 1000` |
| [`call-motion.u32.reads-a-table-nothing-writes.c17.7f6921be`](call-motion.u32.reads-a-table-nothing-writes.c17.7f6921be.c) | nothing, it is one file | type=u32, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.u32.reads-a-table-the-loop-writes.c17.3b2720c3`](call-motion.u32.reads-a-table-the-loop-writes.c17.3b2720c3.c) | nothing, it is one file | type=u32, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.u32.touches-nothing.c17.44576a40`](call-motion.u32.touches-nothing.c17.44576a40.c) | nothing, it is one file | type=u32, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.u32.writes-through-an-argument.c17.eafdb22f`](call-motion.u32.writes-through-an-argument.c17.eafdb22f.c) | nothing, it is one file | type=u32, shape=writes-through-an-argument | c17 | `6000 1000 6` |
| [`call-motion.u64.a-call-under-a-false-test.c17.edfd5743`](call-motion.u64.a-call-under-a-false-test.c17.edfd5743.c) | nothing, it is one file | type=u64, shape=a-call-under-a-false-test | c17 | `1000` |
| [`call-motion.u64.an-argument-the-loop-changes.c17.17f01555`](call-motion.u64.an-argument-the-loop-changes.c17.17f01555.c) | nothing, it is one file | type=u64, shape=an-argument-the-loop-changes | c17 | `338863500 250` |
| [`call-motion.u64.another-unit-is-silent.c17.13ad90a5`](call-motion.u64.another-unit-is-silent.c17.13ad90a5.c) | [`helper`](call-motion.u64.another-unit-is-silent.c17.13ad90a5.helper.c) | type=u64, shape=another-unit-is-silent | c17 | `19000 250` |
| [`call-motion.u64.another-unit-promises-const.c17.bf930a84`](call-motion.u64.another-unit-promises-const.c17.bf930a84.c) | [`helper`](call-motion.u64.another-unit-promises-const.c17.bf930a84.helper.c) | type=u64, shape=another-unit-promises-const | c17 | `19000 250` |
| [`call-motion.u64.another-unit-writes-a-counter.c17.f2020137`](call-motion.u64.another-unit-writes-a-counter.c17.f2020137.c) | [`helper`](call-motion.u64.another-unit-writes-a-counter.c17.f2020137.helper.c) | type=u64, shape=another-unit-writes-a-counter | c17 | `12000 1000` |
| [`call-motion.u64.reads-a-table-nothing-writes.c17.c65c20af`](call-motion.u64.reads-a-table-nothing-writes.c17.c65c20af.c) | nothing, it is one file | type=u64, shape=reads-a-table-nothing-writes | c17 | `13000 7 250` |
| [`call-motion.u64.reads-a-table-the-loop-writes.c17.26133a93`](call-motion.u64.reads-a-table-the-loop-writes.c17.26133a93.c) | nothing, it is one file | type=u64, shape=reads-a-table-the-loop-writes | c17 | `512500 6 1007` |
| [`call-motion.u64.touches-nothing.c17.80dcc81c`](call-motion.u64.touches-nothing.c17.80dcc81c.c) | nothing, it is one file | type=u64, shape=touches-nothing | c17 | `36000 250` |
| [`call-motion.u64.writes-through-an-argument.c17.eaf7f1fb`](call-motion.u64.writes-through-an-argument.c17.eaf7f1fb.c) | nothing, it is one file | type=u64, shape=writes-through-an-argument | c17 | `6000 1000 6` |

