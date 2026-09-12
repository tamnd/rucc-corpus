# block-layout

laying out blocks so the common path falls through. Part of the backend phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`block-layout.chain-of-tests.c17.b179ceb6`](block-layout.chain-of-tests.c17.b179ceb6.c) | shape=chain-of-tests | c17 | `1000` |
| [`block-layout.early-exit.c17.7cb667e4`](block-layout.early-exit.c17.7cb667e4.c) | shape=early-exit | c17 | `1000` |
| [`block-layout.hot-then.builtin.c17.f752e355`](block-layout.hot-then.builtin.c17.f752e355.c) | shape=hot-then, hint=builtin | c17 | `1000` |
| [`block-layout.hot-then.none.c17.a7cd56d7`](block-layout.hot-then.none.c17.a7cd56d7.c) | shape=hot-then, hint=none | c17 | `1000` |
| [`block-layout.hot-then.probability.c17.b47a2ec9`](block-layout.hot-then.probability.c17.b47a2ec9.c) | shape=hot-then, hint=probability | c17 | `1000` |
| [`block-layout.rare-call.builtin.c17.4da46270`](block-layout.rare-call.builtin.c17.4da46270.c) | shape=rare-call, hint=builtin | c17 | `1000` |
| [`block-layout.rare-call.none.c17.57431f1a`](block-layout.rare-call.none.c17.57431f1a.c) | shape=rare-call, hint=none | c17 | `1000` |
| [`block-layout.rare-call.probability.c17.2aecc6fa`](block-layout.rare-call.probability.c17.2aecc6fa.c) | shape=rare-call, hint=probability | c17 | `1000` |
| [`block-layout.rare-chain.builtin.c17.429c3e0e`](block-layout.rare-chain.builtin.c17.429c3e0e.c) | shape=rare-chain, hint=builtin | c17 | `1000` |
| [`block-layout.rare-chain.none.c17.7be4511f`](block-layout.rare-chain.none.c17.7be4511f.c) | shape=rare-chain, hint=none | c17 | `1000` |
| [`block-layout.rare-chain.probability.c17.56a2606e`](block-layout.rare-chain.probability.c17.56a2606e.c) | shape=rare-chain, hint=probability | c17 | `1000` |
| [`block-layout.rare-else.c17.50301dcb`](block-layout.rare-else.c17.50301dcb.c) | shape=rare-else | c17 | `1000` |
| [`block-layout.rare-then.c17.d0366994`](block-layout.rare-then.c17.d0366994.c) | shape=rare-then | c17 | `1000` |
| [`block-layout.widened-condition.builtin.c17.086ee23a`](block-layout.widened-condition.builtin.c17.086ee23a.c) | shape=widened-condition, hint=builtin | c17 | `1000` |
| [`block-layout.widened-condition.none.c17.2fdc96a4`](block-layout.widened-condition.none.c17.2fdc96a4.c) | shape=widened-condition, hint=none | c17 | `1000` |
| [`block-layout.widened-condition.probability.c17.4af40292`](block-layout.widened-condition.probability.c17.4af40292.c) | shape=widened-condition, hint=probability | c17 | `1000` |

