# switch-dispatch

a switch dispatched often enough to time how it was lowered. Part of the backend phase of the M4 plan.

13 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`switch-dispatch.affine.in-order.c17.3de0f02c`](switch-dispatch.affine.in-order.c17.3de0f02c.c) | shape=affine, stream=in-order | c17 | `69645000` |
| [`switch-dispatch.affine.unpredictable.c17.1095fab2`](switch-dispatch.affine.unpredictable.c17.1095fab2.c) | shape=affine, stream=unpredictable | c17 | `69480000` |
| [`switch-dispatch.below-zero.unpredictable.c17.0348e7c3`](switch-dispatch.below-zero.unpredictable.c17.0348e7c3.c) | shape=below-zero, stream=unpredictable | c17 | `69480000` |
| [`switch-dispatch.constant-arms.unpredictable.c17.1075a13f`](switch-dispatch.constant-arms.unpredictable.c17.1075a13f.c) | shape=constant-arms, stream=unpredictable | c17 | `8215000` |
| [`switch-dispatch.eight-labels.unpredictable.c17.9cb88f09`](switch-dispatch.eight-labels.unpredictable.c17.9cb88f09.c) | shape=eight-labels, stream=unpredictable | c17 | `250725000` |
| [`switch-dispatch.five-labels.unpredictable.c17.912748f9`](switch-dispatch.five-labels.unpredictable.c17.912748f9.c) | shape=five-labels, stream=unpredictable | c17 | `222220000` |
| [`switch-dispatch.interpreter.unpredictable.c17.2c27825f`](switch-dispatch.interpreter.unpredictable.c17.2c27825f.c) | shape=interpreter, stream=unpredictable | c17 | `5598638839280509662` |
| [`switch-dispatch.near-the-edge.unpredictable.c17.f7a7f236`](switch-dispatch.near-the-edge.unpredictable.c17.f7a7f236.c) | shape=near-the-edge, stream=unpredictable | c17 | `192075000` |
| [`switch-dispatch.scattered.in-order.c17.0eea8687`](switch-dispatch.scattered.in-order.c17.0eea8687.c) | shape=scattered, stream=in-order | c17 | `219695000` |
| [`switch-dispatch.scattered.unpredictable.c17.13757964`](switch-dispatch.scattered.unpredictable.c17.13757964.c) | shape=scattered, stream=unpredictable | c17 | `218070000` |
| [`switch-dispatch.seven-labels.unpredictable.c17.95b72a1c`](switch-dispatch.seven-labels.unpredictable.c17.95b72a1c.c) | shape=seven-labels, stream=unpredictable | c17 | `188890000` |
| [`switch-dispatch.shared-default.unpredictable.c17.e5a3b685`](switch-dispatch.shared-default.unpredictable.c17.e5a3b685.c) | shape=shared-default, stream=unpredictable | c17 | `60120000` |
| [`switch-dispatch.six-labels.unpredictable.c17.98c71a5e`](switch-dispatch.six-labels.unpredictable.c17.98c71a5e.c) | shape=six-labels, stream=unpredictable | c17 | `215635000` |

