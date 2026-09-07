# code-motion

moving a computation to where it runs no more often. Part of the global phase of the M4 plan.

12 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`code-motion.i32.hoist-from-both-arms.c17.4c43c680`](code-motion.i32.hoist-from-both-arms.c17.4c43c680.c) | type=i32, shape=hoist-from-both-arms | c17 | `37` |
| [`code-motion.i32.hoist-past-store.c17.5a983003`](code-motion.i32.hoist-past-store.c17.5a983003.c) | type=i32, shape=hoist-past-store | c17 | `37` |
| [`code-motion.i32.sink-into-one-arm.c17.a84ae089`](code-motion.i32.sink-into-one-arm.c17.a84ae089.c) | type=i32, shape=sink-into-one-arm | c17 | `37` |
| [`code-motion.i64.hoist-from-both-arms.c17.ca5b1f9a`](code-motion.i64.hoist-from-both-arms.c17.ca5b1f9a.c) | type=i64, shape=hoist-from-both-arms | c17 | `37` |
| [`code-motion.i64.hoist-past-store.c17.2cbe2e86`](code-motion.i64.hoist-past-store.c17.2cbe2e86.c) | type=i64, shape=hoist-past-store | c17 | `37` |
| [`code-motion.i64.sink-into-one-arm.c17.32ecc14b`](code-motion.i64.sink-into-one-arm.c17.32ecc14b.c) | type=i64, shape=sink-into-one-arm | c17 | `37` |
| [`code-motion.u32.hoist-from-both-arms.c17.61a8698b`](code-motion.u32.hoist-from-both-arms.c17.61a8698b.c) | type=u32, shape=hoist-from-both-arms | c17 | `37` |
| [`code-motion.u32.hoist-past-store.c17.ddce71cb`](code-motion.u32.hoist-past-store.c17.ddce71cb.c) | type=u32, shape=hoist-past-store | c17 | `37` |
| [`code-motion.u32.sink-into-one-arm.c17.83c701fd`](code-motion.u32.sink-into-one-arm.c17.83c701fd.c) | type=u32, shape=sink-into-one-arm | c17 | `37` |
| [`code-motion.u64.hoist-from-both-arms.c17.6b7f352c`](code-motion.u64.hoist-from-both-arms.c17.6b7f352c.c) | type=u64, shape=hoist-from-both-arms | c17 | `37` |
| [`code-motion.u64.hoist-past-store.c17.0b8056cb`](code-motion.u64.hoist-past-store.c17.0b8056cb.c) | type=u64, shape=hoist-past-store | c17 | `37` |
| [`code-motion.u64.sink-into-one-arm.c17.644c46ff`](code-motion.u64.sink-into-one-arm.c17.644c46ff.c) | type=u64, shape=sink-into-one-arm | c17 | `37` |

