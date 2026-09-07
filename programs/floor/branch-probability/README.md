# branch-probability

the odds put on an edge before the program has ever run. Part of the floor phase of the M4 plan.

34 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`branch-probability.i32.arm-that-jumps-out.against.c17.4a832962`](branch-probability.i32.arm-that-jumps-out.against.c17.4a832962.c) | type=i32, shape=arm-that-jumps-out, direction=against | c17 | `7` |
| [`branch-probability.i32.arm-that-jumps-out.predicted.c17.4d6c76d7`](branch-probability.i32.arm-that-jumps-out.predicted.c17.4d6c76d7.c) | type=i32, shape=arm-that-jumps-out, direction=predicted | c17 | `7` |
| [`branch-probability.i32.arm-with-a-call.against.c17.0538fc81`](branch-probability.i32.arm-with-a-call.against.c17.0538fc81.c) | type=i32, shape=arm-with-a-call, direction=against | c17 | `7` |
| [`branch-probability.i32.arm-with-a-call.predicted.c17.f22d441f`](branch-probability.i32.arm-with-a-call.predicted.c17.f22d441f.c) | type=i32, shape=arm-with-a-call, direction=predicted | c17 | `7` |
| [`branch-probability.i32.builtin-expect.against.c17.8d44cde8`](branch-probability.i32.builtin-expect.against.c17.8d44cde8.c) | type=i32, shape=builtin-expect, direction=against | c17 | `7` |
| [`branch-probability.i32.builtin-expect.predicted.c17.63ce429f`](branch-probability.i32.builtin-expect.predicted.c17.63ce429f.c) | type=i32, shape=builtin-expect, direction=predicted | c17 | `7` |
| [`branch-probability.i32.loop-back-edge.against.c17.d33f8e5f`](branch-probability.i32.loop-back-edge.against.c17.d33f8e5f.c) | type=i32, shape=loop-back-edge, direction=against | c17 | `1` |
| [`branch-probability.i32.loop-back-edge.predicted.c17.668cc518`](branch-probability.i32.loop-back-edge.predicted.c17.668cc518.c) | type=i32, shape=loop-back-edge, direction=predicted | c17 | `16` |
| [`branch-probability.i32.loop-early-exit.against.c17.7f09f623`](branch-probability.i32.loop-early-exit.against.c17.7f09f623.c) | type=i32, shape=loop-early-exit, direction=against | c17 | `3` |
| [`branch-probability.i32.loop-early-exit.predicted.c17.eebb60a4`](branch-probability.i32.loop-early-exit.predicted.c17.eebb60a4.c) | type=i32, shape=loop-early-exit, direction=predicted | c17 | `16` |
| [`branch-probability.i32.never-returns.predicted.c17.e27bf33c`](branch-probability.i32.never-returns.predicted.c17.e27bf33c.c) | type=i32, shape=never-returns, direction=predicted | c17 | `7` |
| [`branch-probability.i32.not-equal.against.c17.57f79cbc`](branch-probability.i32.not-equal.against.c17.57f79cbc.c) | type=i32, shape=not-equal, direction=against | c17 | `7` |
| [`branch-probability.i32.not-equal.predicted.c17.f185aa4e`](branch-probability.i32.not-equal.predicted.c17.f185aa4e.c) | type=i32, shape=not-equal, direction=predicted | c17 | `7` |
| [`branch-probability.i32.not-negative.against.c17.52e9d844`](branch-probability.i32.not-negative.against.c17.52e9d844.c) | type=i32, shape=not-negative, direction=against | c17 | `7` |
| [`branch-probability.i32.not-negative.predicted.c17.644d5207`](branch-probability.i32.not-negative.predicted.c17.644d5207.c) | type=i32, shape=not-negative, direction=predicted | c17 | `7` |
| [`branch-probability.i32.not-null.against.c17.50dd9656`](branch-probability.i32.not-null.against.c17.50dd9656.c) | type=i32, shape=not-null, direction=against | c17 | `7` |
| [`branch-probability.i32.not-null.predicted.c17.d6cebef3`](branch-probability.i32.not-null.predicted.c17.d6cebef3.c) | type=i32, shape=not-null, direction=predicted | c17 | `7` |
| [`branch-probability.i64.arm-that-jumps-out.against.c17.ff80149f`](branch-probability.i64.arm-that-jumps-out.against.c17.ff80149f.c) | type=i64, shape=arm-that-jumps-out, direction=against | c17 | `7` |
| [`branch-probability.i64.arm-that-jumps-out.predicted.c17.651adbde`](branch-probability.i64.arm-that-jumps-out.predicted.c17.651adbde.c) | type=i64, shape=arm-that-jumps-out, direction=predicted | c17 | `7` |
| [`branch-probability.i64.arm-with-a-call.against.c17.b215e93a`](branch-probability.i64.arm-with-a-call.against.c17.b215e93a.c) | type=i64, shape=arm-with-a-call, direction=against | c17 | `7` |
| [`branch-probability.i64.arm-with-a-call.predicted.c17.fd688d9b`](branch-probability.i64.arm-with-a-call.predicted.c17.fd688d9b.c) | type=i64, shape=arm-with-a-call, direction=predicted | c17 | `7` |
| [`branch-probability.i64.builtin-expect.against.c17.87da6eae`](branch-probability.i64.builtin-expect.against.c17.87da6eae.c) | type=i64, shape=builtin-expect, direction=against | c17 | `7` |
| [`branch-probability.i64.builtin-expect.predicted.c17.33420faa`](branch-probability.i64.builtin-expect.predicted.c17.33420faa.c) | type=i64, shape=builtin-expect, direction=predicted | c17 | `7` |
| [`branch-probability.i64.loop-back-edge.against.c17.833a2c4a`](branch-probability.i64.loop-back-edge.against.c17.833a2c4a.c) | type=i64, shape=loop-back-edge, direction=against | c17 | `1` |
| [`branch-probability.i64.loop-back-edge.predicted.c17.3ea03ad1`](branch-probability.i64.loop-back-edge.predicted.c17.3ea03ad1.c) | type=i64, shape=loop-back-edge, direction=predicted | c17 | `16` |
| [`branch-probability.i64.loop-early-exit.against.c17.3cb62dc5`](branch-probability.i64.loop-early-exit.against.c17.3cb62dc5.c) | type=i64, shape=loop-early-exit, direction=against | c17 | `3` |
| [`branch-probability.i64.loop-early-exit.predicted.c17.401b4a25`](branch-probability.i64.loop-early-exit.predicted.c17.401b4a25.c) | type=i64, shape=loop-early-exit, direction=predicted | c17 | `16` |
| [`branch-probability.i64.never-returns.predicted.c17.37357b7f`](branch-probability.i64.never-returns.predicted.c17.37357b7f.c) | type=i64, shape=never-returns, direction=predicted | c17 | `7` |
| [`branch-probability.i64.not-equal.against.c17.abac9401`](branch-probability.i64.not-equal.against.c17.abac9401.c) | type=i64, shape=not-equal, direction=against | c17 | `7` |
| [`branch-probability.i64.not-equal.predicted.c17.d34f0e63`](branch-probability.i64.not-equal.predicted.c17.d34f0e63.c) | type=i64, shape=not-equal, direction=predicted | c17 | `7` |
| [`branch-probability.i64.not-negative.against.c17.665bf4e4`](branch-probability.i64.not-negative.against.c17.665bf4e4.c) | type=i64, shape=not-negative, direction=against | c17 | `7` |
| [`branch-probability.i64.not-negative.predicted.c17.2ad68dca`](branch-probability.i64.not-negative.predicted.c17.2ad68dca.c) | type=i64, shape=not-negative, direction=predicted | c17 | `7` |
| [`branch-probability.i64.not-null.against.c17.a08aa465`](branch-probability.i64.not-null.against.c17.a08aa465.c) | type=i64, shape=not-null, direction=against | c17 | `7` |
| [`branch-probability.i64.not-null.predicted.c17.00d40305`](branch-probability.i64.not-null.predicted.c17.00d40305.c) | type=i64, shape=not-null, direction=predicted | c17 | `7` |

