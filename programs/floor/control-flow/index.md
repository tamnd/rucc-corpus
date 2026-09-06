# control-flow

the graph shapes the analyses under every pass have to get right. Part of the floor phase of the M4 plan.

10 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`control-flow.computed-goto.c17.7715e4c1`](control-flow.computed-goto.c17.7715e4c1.c) | shape=computed-goto | c17 | `104` |
| [`control-flow.continue-and-break.c17.d30b2e3d`](control-flow.continue-and-break.c17.d30b2e3d.c) | shape=continue-and-break | c17 | `18` |
| [`control-flow.irreducible.c17.662a0a0f`](control-flow.irreducible.c17.662a0a0f.c) | shape=irreducible | c17 | `21` |
| [`control-flow.jumped-over-block.c17.f15166df`](control-flow.jumped-over-block.c17.f15166df.c) | shape=jumped-over-block | c17 | `5` |
| [`control-flow.loop-with-no-exit.c17.c65b95a8`](control-flow.loop-with-no-exit.c17.c65b95a8.c) | shape=loop-with-no-exit | c17 | `1` |
| [`control-flow.many-returns.c17.bc0f4b9a`](control-flow.many-returns.c17.bc0f4b9a.c) | shape=many-returns | c17 | `1 2 3 ...` and 2 more lines |
| [`control-flow.nested-five-deep.c17.32b2c0c2`](control-flow.nested-five-deep.c17.32b2c0c2.c) | shape=nested-five-deep | c17 | `1215` |
| [`control-flow.switch-shared-arms.c17.fbfcdf3d`](control-flow.switch-shared-arms.c17.fbfcdf3d.c) | shape=switch-shared-arms | c17 | `23` |
| [`control-flow.two-exit-loop.c17.23b51e46`](control-flow.two-exit-loop.c17.23b51e46.c) | shape=two-exit-loop | c17 | `15 1` |
| [`control-flow.wide-if-chain.c17.4cc10cde`](control-flow.wide-if-chain.c17.4cc10cde.c) | shape=wide-if-chain | c17 | `15` |

