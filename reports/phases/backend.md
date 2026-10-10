# backend

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`selection`](../../programs/backend/selection/README.md) | 40 | 680 | 200 of 200 | 7% less | 41% less | 13% less |
| [`register-pressure`](../../programs/backend/register-pressure/README.md) | 60 | 3,120 | 300 of 300 | 6% less | 42% less | 29% less |
| [`register-alloc`](../../programs/backend/register-alloc/README.md) | 40 | 2,664 | 200 of 200 | 3% less | 43% less | 27% less |
| [`scheduling`](../../programs/backend/scheduling/README.md) | 20 | 1,046 | 100 of 100 | 40% more | 46% less | 38% less |
| [`block-layout`](../../programs/backend/block-layout/README.md) | 16 | 421 | 80 of 80 | 5% less | 46% less | 38% less |
| [`if-conversion`](../../programs/backend/if-conversion/README.md) | 38 | 945 | 190 of 190 | 11% less | 48% less | 28% less |
| [`switch-lowering`](../../programs/backend/switch-lowering/README.md) | 27 | 1,521 | 135 of 135 | 1% less | 52% less | 34% less |
| [`switch-runs`](../../programs/backend/switch-runs/README.md) | 15 | 1,356 | 75 of 75 | 5% less | 47% less | 29% less |
| [`switch-dispatch`](../../programs/backend/switch-dispatch/README.md) | 21 | 942 | 105 of 105 | 26% less | 53% less | 47% less |
| [`division`](../../programs/backend/division/README.md) | 72 | 2,020 | 360 of 360 | 21% less | 56% less | 45% less |
| [`narrow-shift`](../../programs/backend/narrow-shift/README.md) | 8 | 256 | 40 of 40 | 39% less | 61% less | 51% less |
| [`calling-convention`](../../programs/backend/calling-convention/README.md) | 10 | 216 | 50 of 50 | 5% less | 45% less | 35% more |
| [`machine-peephole`](../../programs/backend/machine-peephole/README.md) | 22 | 402 | 110 of 110 | 6% less | 40% less | 32% less |
| [`bit-liveness`](../../programs/backend/bit-liveness/README.md) | 28 | 628 | 140 of 140 | 4% less | 39% less | 32% less |
| [`compare-elim`](../../programs/backend/compare-elim/README.md) | 26 | 526 | 130 of 130 | 5% less | 41% less | 31% less |
| [`address-fold`](../../programs/backend/address-fold/README.md) | 28 | 584 | 140 of 140 | 6% less | 41% less | 29% less |
| [`load-fold`](../../programs/backend/load-fold/README.md) | 40 | 924 | 200 of 200 | 16% less | 46% less | 30% less |
| [`store-fold`](../../programs/backend/store-fold/README.md) | 56 | 1,416 | 280 of 280 | 16% less | 44% less | 39% less |
| [`store-fold-constant`](../../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 590 of 590 | 14% less | 45% less | 36% less |
| [`compare-fold`](../../programs/backend/compare-fold/README.md) | 118 | 2,742 | 590 of 590 | 13% less | 47% less | 33% less |
| [`frame-address`](../../programs/backend/frame-address/README.md) | 6 | 117 | 30 of 30 | 6% less | 44% less | 38% less |
| [`stack-slots`](../../programs/backend/stack-slots/README.md) | 6 | 201 | 30 of 30 | 35% less | 48% less | 43% less |
| [`frame-size`](../../programs/backend/frame-size/README.md) | 59 | 8,808 | 255 of 295 | 26% less | 71% less | 49% less |
| [`interpreter-dispatch`](../../programs/backend/interpreter-dispatch/README.md) | 54 | 34,509 | 154 of 270 | 72% more | 69% less | 45% less |
| [`bit-builtins`](../../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 110 of 110 | 16% more | 53% less | 38% less |
| [`float-conversion`](../../programs/backend/float-conversion/README.md) | 66 | 1,477 | 330 of 330 | 4% less | 43% less | 30% less |
| [`long-double`](../../programs/backend/long-double/README.md) | 10 | 226 | 50 of 50 | 37% more | 44% less | 49% less |

The reference compiler said it took 4910 transformations in this phase and wanted 25757 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

## Switches lowered differently

Each of these cases has a `switch` that a compiler under test lowered into a different shape from the one `gcc-16` used: a jump table, a bit test, a lookup table of answers, or compares when it was none of the three. It is a lead to read the assembly for rather than a failure. The run's `report.md` says how many cases agreed.

| compiler | level | case | it used | `gcc-16` used |
|---|---|---|---|---|
| `rucc` | O0 | `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.24.table.none.8.c17.dd956486` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.4.table.direct.8.c17.ad381896` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.4.table.none.8.c17.66690ed6` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.4.table.pointer.8.c17.f16c570b` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.4.threaded.direct.8.c17.81bbc59c` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.4.threaded.none.8.c17.47a678c0` | compares | table |
| `rucc` | O0 | `interpreter-dispatch.4.threaded.pointer.8.c17.77dc9ec0` | compares | table |
| `rucc` | O0 | `switch-dispatch.affine.in-order.c17.3de0f02c` | table | compares |
| `rucc` | O0 | `switch-dispatch.affine.unpredictable.c17.1095fab2` | table | compares |
| `rucc` | O0 | `switch-dispatch.below-zero.unpredictable.c17.0348e7c3` | table | compares |
| `rucc` | O0 | `switch-dispatch.holes.unpredictable.c17.343584c6` | table | compares |
| `rucc` | O0 | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | table | compares |
| `rucc` | O0 | `switch-dispatch.masked.unpredictable.c17.fcb9805f` | table | compares |
| `rucc` | O0 | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | table | compares |
| `rucc` | O0 | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | table | compares |
| `rucc` | O0 | `switch-dispatch.near-the-edge.unpredictable.c17.f7a7f236` | table | compares |
| `rucc` | O0 | `switch-dispatch.negative-answers.unpredictable.c17.45c3408b` | table | compares |
| `rucc` | O0 | `switch-dispatch.scattered.in-order.c17.0eea8687` | table | compares |
| `rucc` | O0 | `switch-dispatch.scattered.unpredictable.c17.13757964` | table | compares |
| `rucc` | O0 | `switch-dispatch.shared-default.unpredictable.c17.e5a3b685` | table | compares |
| `rucc` | O0 | `switch-dispatch.wide-answers.unpredictable.c17.7659b1be` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.31.c17.d18e4de0` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.33.c17.81ec76e2` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.40.c17.85e83015` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.hot-case.75.none.c17.57f4af9a` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.hot-case.75.right.c17.34253fd4` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.hot-case.75.wrong.c17.d50a50bd` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.hot-case.95.none.c17.126eab91` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.hot-case.95.right.c17.38920e87` | table | compares |
| `rucc` | O0 | `switch-lowering.dense.hot-case.95.wrong.c17.22be52ce` | table | compares |
| `rucc` | O0 | `switch-runs.several-runs.31.c17.02b4c07b` | table | compares |
| `rucc` | O0 | `switch-runs.several-runs.33.c17.9b43c0a4` | table | compares |
| `rucc` | O1 | `interpreter-dispatch.12.table.direct.32.c17.49df9385` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.direct.32.state.c17.4727b18d` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.none.32.state.c17.d33e4ecc` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.pointer.32.state.c17.cb2bc2cb` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.direct.32.state.c17.689aedcb` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.none.32.state.c17.f6894c53` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.pointer.32.state.c17.3f2749d0` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.direct.32.state.c17.f68a39e0` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.none.32.state.c17.fd6231dd` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.none.8.c17.dd956486` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.pointer.32.state.c17.157ff818` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.direct.32.state.c17.8d3ba933` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.none.32.state.c17.0efc8547` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.pointer.32.state.c17.d03543dc` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.direct.32.state.c17.790ceed9` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.direct.8.c17.ad381896` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.none.32.c17.55d82006` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.none.32.state.c17.4d0fc2e5` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.none.8.c17.66690ed6` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.pointer.32.c17.b8148020` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.pointer.32.state.c17.38484dbf` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.table.pointer.8.c17.f16c570b` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.direct.32.c17.d368856f` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.direct.32.state.c17.4694b8a9` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.direct.8.c17.81bbc59c` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.none.32.c17.25ff2287` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.none.32.state.c17.a4bc0c15` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.none.8.c17.47a678c0` | compares | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.pointer.32.c17.03cdd965` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.pointer.32.state.c17.87128b2d` | table | bit-test, table |
| `rucc` | O1 | `interpreter-dispatch.4.threaded.pointer.8.c17.77dc9ec0` | compares | bit-test, table |
| `rucc` | O1 | `switch-dispatch.eight-labels.unpredictable.c17.9cb88f09` | compares | table |
| `rucc` | O1 | `switch-dispatch.five-labels.unpredictable.c17.912748f9` | compares | table |
| `rucc` | O1 | `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | compares | table |
| `rucc` | O1 | `switch-dispatch.seven-labels.unpredictable.c17.95b72a1c` | compares | table |
| `rucc` | O1 | `switch-dispatch.six-labels.unpredictable.c17.98c71a5e` | compares | table |
| `rucc` | O1 | `switch-lowering.dense.8.c17.35614fce` | compares | table |
| `rucc` | O1 | `switch-runs.look-alike-arms.five.c17.641be602` | compares | table |
| `rucc` | O2 | `interpreter-dispatch.12.table.direct.32.c17.49df9385` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.direct.32.state.c17.4727b18d` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.none.32.state.c17.d33e4ecc` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.pointer.32.state.c17.cb2bc2cb` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.direct.32.state.c17.689aedcb` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.none.32.state.c17.f6894c53` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.pointer.32.state.c17.3f2749d0` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.direct.32.state.c17.f68a39e0` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.none.32.state.c17.fd6231dd` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.none.8.c17.dd956486` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.pointer.32.state.c17.157ff818` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.direct.32.state.c17.8d3ba933` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.none.32.state.c17.0efc8547` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.pointer.32.state.c17.d03543dc` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.direct.32.state.c17.790ceed9` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.direct.8.c17.ad381896` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.none.32.c17.55d82006` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.none.32.state.c17.4d0fc2e5` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.none.8.c17.66690ed6` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.pointer.32.c17.b8148020` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.pointer.32.state.c17.38484dbf` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.table.pointer.8.c17.f16c570b` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.direct.32.c17.d368856f` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.direct.32.state.c17.4694b8a9` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.direct.8.c17.81bbc59c` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.none.32.c17.25ff2287` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.none.32.state.c17.a4bc0c15` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.none.8.c17.47a678c0` | compares | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.pointer.32.c17.03cdd965` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.pointer.32.state.c17.87128b2d` | table | bit-test, table |
| `rucc` | O2 | `interpreter-dispatch.4.threaded.pointer.8.c17.77dc9ec0` | compares | bit-test, table |
| `rucc` | O2 | `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | compares | table |
| `rucc` | O2 | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | compares | table |
| `rucc` | O2 | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | compares | table |
| `rucc` | O2 | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | compares | table |
| `rucc` | O2 | `switch-runs.classifier.char.c17.b072f937` | lookup | compares |
| `rucc` | O2 | `switch-runs.classifier.unsigned-char.c17.3ae9dc72` | lookup | compares |
| `rucc` | O2 | `switch-runs.several-runs.2.c17.c52ca973` | lookup | compares |
| `rucc` | O3 | `interpreter-dispatch.12.table.direct.32.c17.49df9385` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.direct.32.state.c17.4727b18d` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.none.32.state.c17.d33e4ecc` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.pointer.32.state.c17.cb2bc2cb` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.direct.32.state.c17.689aedcb` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.none.32.state.c17.f6894c53` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.pointer.32.state.c17.3f2749d0` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.direct.32.state.c17.f68a39e0` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.none.32.state.c17.fd6231dd` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.none.8.c17.dd956486` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.pointer.32.state.c17.157ff818` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.direct.32.state.c17.8d3ba933` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.none.32.state.c17.0efc8547` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.pointer.32.state.c17.d03543dc` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.direct.32.state.c17.790ceed9` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.direct.8.c17.ad381896` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.none.32.c17.55d82006` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.none.32.state.c17.4d0fc2e5` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.none.8.c17.66690ed6` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.pointer.32.c17.b8148020` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.pointer.32.state.c17.38484dbf` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.table.pointer.8.c17.f16c570b` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.direct.32.c17.d368856f` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.direct.32.state.c17.4694b8a9` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.direct.8.c17.81bbc59c` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.none.32.c17.25ff2287` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.none.32.state.c17.a4bc0c15` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.none.8.c17.47a678c0` | compares | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.pointer.32.c17.03cdd965` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.pointer.32.state.c17.87128b2d` | table | bit-test, table |
| `rucc` | O3 | `interpreter-dispatch.4.threaded.pointer.8.c17.77dc9ec0` | compares | bit-test, table |
| `rucc` | O3 | `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | compares | table |
| `rucc` | O3 | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | compares | table |
| `rucc` | O3 | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | compares | table |
| `rucc` | O3 | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | compares | table |
| `rucc` | O3 | `switch-runs.classifier.char.c17.b072f937` | lookup | compares |
| `rucc` | O3 | `switch-runs.classifier.unsigned-char.c17.3ae9dc72` | lookup | compares |
| `rucc` | O3 | `switch-runs.several-runs.2.c17.c52ca973` | lookup | compares |
| `rucc` | Os | `interpreter-dispatch.12.table.direct.32.c17.49df9385` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.direct.32.state.c17.4727b18d` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.none.32.state.c17.d33e4ecc` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.pointer.32.state.c17.cb2bc2cb` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.direct.32.state.c17.689aedcb` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.none.32.state.c17.f6894c53` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.pointer.32.state.c17.3f2749d0` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.direct.32.state.c17.f68a39e0` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.none.32.state.c17.fd6231dd` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.none.8.c17.dd956486` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.pointer.32.state.c17.157ff818` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.direct.32.state.c17.8d3ba933` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.none.32.state.c17.0efc8547` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.pointer.32.state.c17.d03543dc` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.direct.32.state.c17.790ceed9` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.direct.8.c17.ad381896` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.none.32.c17.55d82006` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.none.32.state.c17.4d0fc2e5` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.none.8.c17.66690ed6` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.pointer.32.c17.b8148020` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.pointer.32.state.c17.38484dbf` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.table.pointer.8.c17.f16c570b` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.direct.32.c17.d368856f` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.direct.32.state.c17.4694b8a9` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.direct.8.c17.81bbc59c` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.none.32.c17.25ff2287` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.none.32.state.c17.a4bc0c15` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.none.8.c17.47a678c0` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.pointer.32.c17.03cdd965` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.pointer.32.state.c17.87128b2d` | table | bit-test, table |
| `rucc` | Os | `interpreter-dispatch.4.threaded.pointer.8.c17.77dc9ec0` | table | bit-test, table |
| `rucc` | Os | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | compares | table |
| `rucc` | Os | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | compares | table |
| `rucc` | Os | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | compares | table |
| `rucc` | Os | `switch-runs.classifier.char.c17.b072f937` | lookup | compares |
| `rucc` | Os | `switch-runs.classifier.unsigned-char.c17.3ae9dc72` | lookup | compares |
| `rucc` | Os | `switch-runs.several-runs.2.c17.c52ca973` | lookup | compares |

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
