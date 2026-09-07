# value-range

narrowing an integer to the range it can hold. Part of the global phase of the M4 plan.

13 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`value-range.both-edges.c17.cc26b3d1`](value-range.both-edges.c17.cc26b3d1.c) | shape=both-edges | c17 | `1` |
| [`value-range.carried-round-a-loop.c17.d3e1bc04`](value-range.carried-round-a-loop.c17.d3e1bc04.c) | shape=carried-round-a-loop | c17 | `1 1` |
| [`value-range.guard-keeps-the-divide.c17.ad0655ca`](value-range.guard-keeps-the-divide.c17.ad0655ca.c) | shape=guard-keeps-the-divide | c17 | `1` |
| [`value-range.guard-keeps-the-index.c17.b4461bd1`](value-range.guard-keeps-the-index.c17.b4461bd1.c) | shape=guard-keeps-the-index | c17 | `1` |
| [`value-range.guard-keeps-the-shift.c17.10b611ce`](value-range.guard-keeps-the-shift.c17.10b611ce.c) | shape=guard-keeps-the-shift | c17 | `1` |
| [`value-range.i32.c17.62b45c41`](value-range.i32.c17.62b45c41.c) | type=i32 | c17 | `1 1 1 ...` and 2 more lines |
| [`value-range.i64.c17.3659dfd3`](value-range.i64.c17.3659dfd3.c) | type=i64 | c17 | `1 1 1 ...` and 2 more lines |
| [`value-range.inverted-condition.c17.c6ce6f75`](value-range.inverted-condition.c17.c6ce6f75.c) | shape=inverted-condition | c17 | `1` |
| [`value-range.relation-between-two.c17.16363e85`](value-range.relation-between-two.c17.16363e85.c) | shape=relation-between-two | c17 | `1` |
| [`value-range.switch-arms.c17.149995c6`](value-range.switch-arms.c17.149995c6.c) | shape=switch-arms | c17 | `1` |
| [`value-range.through-a-cast.c17.09f330f1`](value-range.through-a-cast.c17.09f330f1.c) | shape=through-a-cast | c17 | `1 1` |
| [`value-range.u32.c17.c8bfc2a4`](value-range.u32.c17.c8bfc2a4.c) | type=u32 | c17 | `1 1 1 1` |
| [`value-range.u64.c17.a9f9c33a`](value-range.u64.c17.a9f9c33a.c) | type=u64 | c17 | `1 1 1 1` |

