# switch-dispatch

a switch dispatched often enough to time how it was lowered. Part of the backend phase of the M4 plan.

8 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`switch-dispatch.affine.in-order.c17.31fb5ba5`](switch-dispatch.affine.in-order.c17.31fb5ba5.c) | shape=affine, stream=in-order | c17 | `70160000` |
| [`switch-dispatch.affine.unpredictable.c17.50ca08a2`](switch-dispatch.affine.unpredictable.c17.50ca08a2.c) | shape=affine, stream=unpredictable | c17 | `68020000` |
| [`switch-dispatch.below-zero.unpredictable.c17.8a61fb10`](switch-dispatch.below-zero.unpredictable.c17.8a61fb10.c) | shape=below-zero, stream=unpredictable | c17 | `68020000` |
| [`switch-dispatch.constant-arms.unpredictable.c17.434e74d0`](switch-dispatch.constant-arms.unpredictable.c17.434e74d0.c) | shape=constant-arms, stream=unpredictable | c17 | `8340000` |
| [`switch-dispatch.interpreter.unpredictable.c17.403fa029`](switch-dispatch.interpreter.unpredictable.c17.403fa029.c) | shape=interpreter, stream=unpredictable | c17 | `1419164709399262` |
| [`switch-dispatch.near-the-edge.unpredictable.c17.c1ab3ce0`](switch-dispatch.near-the-edge.unpredictable.c17.c1ab3ce0.c) | shape=near-the-edge, stream=unpredictable | c17 | `187980000` |
| [`switch-dispatch.scattered.unpredictable.c17.7350b390`](switch-dispatch.scattered.unpredictable.c17.7350b390.c) | shape=scattered, stream=unpredictable | c17 | `204460000` |
| [`switch-dispatch.shared-default.unpredictable.c17.6c6c02c1`](switch-dispatch.shared-default.unpredictable.c17.6c6c02c1.c) | shape=shared-default, stream=unpredictable | c17 | `58420000` |

