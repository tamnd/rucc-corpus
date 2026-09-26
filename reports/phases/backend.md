# backend

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`selection`](../../programs/backend/selection/README.md) | 40 | 680 | 200 of 200 | 7% less | 44% less | 52% less |
| [`register-pressure`](../../programs/backend/register-pressure/README.md) | 60 | 3,120 | 300 of 300 | 34% more | 45% less | 52% less |
| [`register-alloc`](../../programs/backend/register-alloc/README.md) | 40 | 2,664 | 200 of 200 | 62% more | 45% less | 54% less |
| [`scheduling`](../../programs/backend/scheduling/README.md) | 20 | 1,046 | 100 of 100 | 42% more | 44% less | 64% less |
| [`block-layout`](../../programs/backend/block-layout/README.md) | 16 | 421 | 80 of 80 | 1% less | 49% less | 56% less |
| [`if-conversion`](../../programs/backend/if-conversion/README.md) | 38 | 945 | 190 of 190 | 8% less | 50% less | level |
| [`switch-lowering`](../../programs/backend/switch-lowering/README.md) | 27 | 1,521 | 135 of 135 | 3% more | 54% less | 62% less |
| [`switch-runs`](../../programs/backend/switch-runs/README.md) | 15 | 1,356 | 75 of 75 | 8% more | 45% less | 67% less |
| [`switch-dispatch`](../../programs/backend/switch-dispatch/README.md) | 21 | 942 | 105 of 105 | 14% less | 55% less | level |
| [`calling-convention`](../../programs/backend/calling-convention/README.md) | 10 | 216 | 50 of 50 | 17% more | 43% less | 53% less |
| [`machine-peephole`](../../programs/backend/machine-peephole/README.md) | 22 | 402 | 110 of 110 | 6% less | 45% less | 51% less |
| [`bit-liveness`](../../programs/backend/bit-liveness/README.md) | 28 | 628 | 140 of 140 | 3% less | 48% less | 53% less |
| [`compare-elim`](../../programs/backend/compare-elim/README.md) | 26 | 526 | 130 of 130 | 5% less | 47% less | 53% less |
| [`address-fold`](../../programs/backend/address-fold/README.md) | 28 | 584 | 140 of 140 | 8% more | 45% less | 54% less |
| [`load-fold`](../../programs/backend/load-fold/README.md) | 40 | 924 | 200 of 200 | 13% less | 47% less | 68% less |
| [`store-fold`](../../programs/backend/store-fold/README.md) | 56 | 1,416 | 280 of 280 | 13% less | 49% less | 67% less |
| [`store-fold-constant`](../../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 590 of 590 | 11% less | 49% less | 64% less |
| [`compare-fold`](../../programs/backend/compare-fold/README.md) | 118 | 2,742 | 590 of 590 | 9% less | 47% less | 64% less |
| [`frame-address`](../../programs/backend/frame-address/README.md) | 6 | 117 | 30 of 30 | 3% more | 46% less | 54% less |
| [`stack-slots`](../../programs/backend/stack-slots/README.md) | 6 | 201 | 30 of 30 | 5% less | 54% less | 66% less |
| [`bit-builtins`](../../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 110 of 110 | 83% more | 42% less | 60% less |
| [`float-conversion`](../../programs/backend/float-conversion/README.md) | 66 | 1,477 | 330 of 330 | 1% less | 45% less | 53% less |
| [`long-double`](../../programs/backend/long-double/README.md) | 10 | 226 | 50 of 50 | 47% more | 48% less | 68% less |

The reference compiler said it took 1913 transformations in this phase and wanted 9409 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

## Switches lowered differently

Each of these cases has a `switch` that a compiler under test lowered into a different shape from the one `gcc-16` used: a jump table, a bit test, a lookup table of answers, or compares when it was none of the three. It is a lead to read the assembly for rather than a failure. The run's `report.md` says how many cases agreed.

| compiler | level | case | it used | `gcc-16` used |
|---|---|---|---|---|
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
| `rucc` | O1 | `switch-dispatch.eight-labels.unpredictable.c17.9cb88f09` | compares | table |
| `rucc` | O1 | `switch-dispatch.five-labels.unpredictable.c17.912748f9` | compares | table |
| `rucc` | O1 | `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | compares | table |
| `rucc` | O1 | `switch-dispatch.seven-labels.unpredictable.c17.95b72a1c` | compares | table |
| `rucc` | O1 | `switch-dispatch.six-labels.unpredictable.c17.98c71a5e` | compares | table |
| `rucc` | O1 | `switch-lowering.dense.8.c17.35614fce` | compares | table |
| `rucc` | O1 | `switch-runs.look-alike-arms.five.c17.641be602` | compares | table |
| `rucc` | O2 | `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | compares | table |
| `rucc` | O2 | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | compares | table |
| `rucc` | O2 | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | compares | table |
| `rucc` | O2 | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | compares | table |
| `rucc` | O2 | `switch-dispatch.shared-default.unpredictable.c17.e5a3b685` | table | compares |
| `rucc` | O2 | `switch-runs.several-runs.31.c17.02b4c07b` | table | lookup |
| `rucc` | O2 | `switch-runs.several-runs.33.c17.9b43c0a4` | table | lookup |
| `rucc` | O2 | `switch-runs.several-runs.4.c17.09e48ba4` | compares | lookup |
| `rucc` | O3 | `switch-dispatch.interpreter.unpredictable.c17.2c27825f` | compares | table |
| `rucc` | O3 | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | compares | table |
| `rucc` | O3 | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | compares | table |
| `rucc` | O3 | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | compares | table |
| `rucc` | O3 | `switch-dispatch.shared-default.unpredictable.c17.e5a3b685` | table | compares |
| `rucc` | O3 | `switch-runs.several-runs.31.c17.02b4c07b` | table | lookup |
| `rucc` | O3 | `switch-runs.several-runs.33.c17.9b43c0a4` | table | lookup |
| `rucc` | O3 | `switch-runs.several-runs.4.c17.09e48ba4` | compares | lookup |
| `rucc` | Os | `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | compares | table |
| `rucc` | Os | `switch-dispatch.names-with-holes.unpredictable.c17.64a0db1b` | compares | table |
| `rucc` | Os | `switch-dispatch.names.unpredictable.c17.cd05ba1f` | compares | table |
| `rucc` | Os | `switch-dispatch.shared-default.unpredictable.c17.e5a3b685` | table | compares |
| `rucc` | Os | `switch-runs.several-runs.31.c17.02b4c07b` | compares | lookup |
| `rucc` | Os | `switch-runs.several-runs.33.c17.9b43c0a4` | compares | lookup |
| `rucc` | Os | `switch-runs.several-runs.4.c17.09e48ba4` | compares | lookup |

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
