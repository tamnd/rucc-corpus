# common-subexpr

reusing an earlier computation instead of repeating it. Part of the global phase of the M4 plan.

32 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`common-subexpr.i32.across-call.c17.638e4dc0`](common-subexpr.i32.across-call.c17.638e4dc0.c) | type=i32, shape=across-call | c17 | `84` |
| [`common-subexpr.i32.across-if.c17.17efe941`](common-subexpr.i32.across-if.c17.17efe941.c) | type=i32, shape=across-if | c17 | `84` |
| [`common-subexpr.i32.across-store.c17.a78d38d5`](common-subexpr.i32.across-store.c17.a78d38d5.c) | type=i32, shape=across-store | c17 | `84` |
| [`common-subexpr.i32.chained.c17.cae5f36c`](common-subexpr.i32.chained.c17.cae5f36c.c) | type=i32, shape=chained | c17 | `96` |
| [`common-subexpr.i32.commutative.c17.b9f0c465`](common-subexpr.i32.commutative.c17.b9f0c465.c) | type=i32, shape=commutative | c17 | `84` |
| [`common-subexpr.i32.partial.c17.dfd05a0f`](common-subexpr.i32.partial.c17.dfd05a0f.c) | type=i32, shape=partial | c17 | `84` |
| [`common-subexpr.i32.straight.c17.c2624911`](common-subexpr.i32.straight.c17.c2624911.c) | type=i32, shape=straight | c17 | `84` |
| [`common-subexpr.i32.subscript.c17.4bf0107a`](common-subexpr.i32.subscript.c17.4bf0107a.c) | type=i32, shape=subscript | c17 | `84` |
| [`common-subexpr.i64.across-call.c17.83d736f6`](common-subexpr.i64.across-call.c17.83d736f6.c) | type=i64, shape=across-call | c17 | `84` |
| [`common-subexpr.i64.across-if.c17.7fc8f94f`](common-subexpr.i64.across-if.c17.7fc8f94f.c) | type=i64, shape=across-if | c17 | `84` |
| [`common-subexpr.i64.across-store.c17.cc90865f`](common-subexpr.i64.across-store.c17.cc90865f.c) | type=i64, shape=across-store | c17 | `84` |
| [`common-subexpr.i64.chained.c17.21d9e3b8`](common-subexpr.i64.chained.c17.21d9e3b8.c) | type=i64, shape=chained | c17 | `96` |
| [`common-subexpr.i64.commutative.c17.968ad21f`](common-subexpr.i64.commutative.c17.968ad21f.c) | type=i64, shape=commutative | c17 | `84` |
| [`common-subexpr.i64.partial.c17.d4a99d85`](common-subexpr.i64.partial.c17.d4a99d85.c) | type=i64, shape=partial | c17 | `84` |
| [`common-subexpr.i64.straight.c17.dd621c78`](common-subexpr.i64.straight.c17.dd621c78.c) | type=i64, shape=straight | c17 | `84` |
| [`common-subexpr.i64.subscript.c17.253b1f8e`](common-subexpr.i64.subscript.c17.253b1f8e.c) | type=i64, shape=subscript | c17 | `84` |
| [`common-subexpr.u32.across-call.c17.a5746aa6`](common-subexpr.u32.across-call.c17.a5746aa6.c) | type=u32, shape=across-call | c17 | `84` |
| [`common-subexpr.u32.across-if.c17.c7e317f0`](common-subexpr.u32.across-if.c17.c7e317f0.c) | type=u32, shape=across-if | c17 | `84` |
| [`common-subexpr.u32.across-store.c17.67d026e7`](common-subexpr.u32.across-store.c17.67d026e7.c) | type=u32, shape=across-store | c17 | `84` |
| [`common-subexpr.u32.chained.c17.42342882`](common-subexpr.u32.chained.c17.42342882.c) | type=u32, shape=chained | c17 | `96` |
| [`common-subexpr.u32.commutative.c17.abf5b712`](common-subexpr.u32.commutative.c17.abf5b712.c) | type=u32, shape=commutative | c17 | `84` |
| [`common-subexpr.u32.partial.c17.f937d396`](common-subexpr.u32.partial.c17.f937d396.c) | type=u32, shape=partial | c17 | `84` |
| [`common-subexpr.u32.straight.c17.b10c00ab`](common-subexpr.u32.straight.c17.b10c00ab.c) | type=u32, shape=straight | c17 | `84` |
| [`common-subexpr.u32.subscript.c17.11dd578b`](common-subexpr.u32.subscript.c17.11dd578b.c) | type=u32, shape=subscript | c17 | `84` |
| [`common-subexpr.u64.across-call.c17.717980cc`](common-subexpr.u64.across-call.c17.717980cc.c) | type=u64, shape=across-call | c17 | `84` |
| [`common-subexpr.u64.across-if.c17.979c05b2`](common-subexpr.u64.across-if.c17.979c05b2.c) | type=u64, shape=across-if | c17 | `84` |
| [`common-subexpr.u64.across-store.c17.343034a4`](common-subexpr.u64.across-store.c17.343034a4.c) | type=u64, shape=across-store | c17 | `84` |
| [`common-subexpr.u64.chained.c17.67dbf8d5`](common-subexpr.u64.chained.c17.67dbf8d5.c) | type=u64, shape=chained | c17 | `96` |
| [`common-subexpr.u64.commutative.c17.b1e80a8b`](common-subexpr.u64.commutative.c17.b1e80a8b.c) | type=u64, shape=commutative | c17 | `84` |
| [`common-subexpr.u64.partial.c17.1a6d4cd7`](common-subexpr.u64.partial.c17.1a6d4cd7.c) | type=u64, shape=partial | c17 | `84` |
| [`common-subexpr.u64.straight.c17.b5d96c1e`](common-subexpr.u64.straight.c17.b5d96c1e.c) | type=u64, shape=straight | c17 | `84` |
| [`common-subexpr.u64.subscript.c17.878e74a6`](common-subexpr.u64.subscript.c17.878e74a6.c) | type=u64, shape=subscript | c17 | `84` |

