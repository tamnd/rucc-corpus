# block-layout

laying out blocks so the common path falls through. Part of the backend phase of the M4 plan.

4 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`block-layout.chain-of-tests.c17.b179ceb6`](block-layout.chain-of-tests.c17.b179ceb6.c) | shape=chain-of-tests | c17 | `1000` |
| [`block-layout.early-exit.c17.7cb667e4`](block-layout.early-exit.c17.7cb667e4.c) | shape=early-exit | c17 | `1000` |
| [`block-layout.rare-else.c17.50301dcb`](block-layout.rare-else.c17.50301dcb.c) | shape=rare-else | c17 | `1000` |
| [`block-layout.rare-then.c17.d0366994`](block-layout.rare-then.c17.d0366994.c) | shape=rare-then | c17 | `1000` |

