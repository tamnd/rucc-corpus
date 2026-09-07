# common-subexpr

reusing an earlier computation instead of repeating it. Part of the global phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`common-subexpr.i32.across-call.c17.638e4dc0`](common-subexpr.i32.across-call.c17.638e4dc0.c) | type=i32, shape=across-call | c17 | `84` |
| [`common-subexpr.i32.across-if.c17.17efe941`](common-subexpr.i32.across-if.c17.17efe941.c) | type=i32, shape=across-if | c17 | `84` |
| [`common-subexpr.i32.partial.c17.dfd05a0f`](common-subexpr.i32.partial.c17.dfd05a0f.c) | type=i32, shape=partial | c17 | `84` |
| [`common-subexpr.i32.straight.c17.c2624911`](common-subexpr.i32.straight.c17.c2624911.c) | type=i32, shape=straight | c17 | `84` |
| [`common-subexpr.i64.across-call.c17.83d736f6`](common-subexpr.i64.across-call.c17.83d736f6.c) | type=i64, shape=across-call | c17 | `84` |
| [`common-subexpr.i64.across-if.c17.7fc8f94f`](common-subexpr.i64.across-if.c17.7fc8f94f.c) | type=i64, shape=across-if | c17 | `84` |
| [`common-subexpr.i64.partial.c17.d4a99d85`](common-subexpr.i64.partial.c17.d4a99d85.c) | type=i64, shape=partial | c17 | `84` |
| [`common-subexpr.i64.straight.c17.dd621c78`](common-subexpr.i64.straight.c17.dd621c78.c) | type=i64, shape=straight | c17 | `84` |
| [`common-subexpr.u32.across-call.c17.a5746aa6`](common-subexpr.u32.across-call.c17.a5746aa6.c) | type=u32, shape=across-call | c17 | `84` |
| [`common-subexpr.u32.across-if.c17.c7e317f0`](common-subexpr.u32.across-if.c17.c7e317f0.c) | type=u32, shape=across-if | c17 | `84` |
| [`common-subexpr.u32.partial.c17.f937d396`](common-subexpr.u32.partial.c17.f937d396.c) | type=u32, shape=partial | c17 | `84` |
| [`common-subexpr.u32.straight.c17.b10c00ab`](common-subexpr.u32.straight.c17.b10c00ab.c) | type=u32, shape=straight | c17 | `84` |
| [`common-subexpr.u64.across-call.c17.717980cc`](common-subexpr.u64.across-call.c17.717980cc.c) | type=u64, shape=across-call | c17 | `84` |
| [`common-subexpr.u64.across-if.c17.979c05b2`](common-subexpr.u64.across-if.c17.979c05b2.c) | type=u64, shape=across-if | c17 | `84` |
| [`common-subexpr.u64.partial.c17.1a6d4cd7`](common-subexpr.u64.partial.c17.1a6d4cd7.c) | type=u64, shape=partial | c17 | `84` |
| [`common-subexpr.u64.straight.c17.b5d96c1e`](common-subexpr.u64.straight.c17.b5d96c1e.c) | type=u64, shape=straight | c17 | `84` |

