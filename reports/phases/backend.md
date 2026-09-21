# backend

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`selection`](../../programs/backend/selection/README.md) | 40 | 680 | 200 of 200 | 6% less | 48% less | 56% less |
| [`register-pressure`](../../programs/backend/register-pressure/README.md) | 60 | 3,120 | 300 of 300 | 36% more | 44% less | 57% less |
| [`register-alloc`](../../programs/backend/register-alloc/README.md) | 40 | 2,664 | 200 of 200 | 62% more | 45% less | 64% less |
| [`scheduling`](../../programs/backend/scheduling/README.md) | 20 | 1,046 | 100 of 100 | 44% more | 48% less | 66% less |
| [`block-layout`](../../programs/backend/block-layout/README.md) | 16 | 421 | 80 of 80 | level | 48% less | 68% less |
| [`if-conversion`](../../programs/backend/if-conversion/README.md) | 20 | 444 | 100 of 100 | 3% less | 53% less | 70% less |
| [`switch-lowering`](../../programs/backend/switch-lowering/README.md) | 15 | 705 | 75 of 75 | 8% more | 56% less | 66% less |
| [`switch-runs`](../../programs/backend/switch-runs/README.md) | 15 | 1,356 | 75 of 75 | 10% more | 50% less | 68% less |
| [`switch-dispatch`](../../programs/backend/switch-dispatch/README.md) | 9 | 415 | 45 of 45 | 10% less | 55% less | level |
| [`calling-convention`](../../programs/backend/calling-convention/README.md) | 10 | 216 | 50 of 50 | 18% more | 47% less | 58% less |
| [`machine-peephole`](../../programs/backend/machine-peephole/README.md) | 22 | 402 | 110 of 110 | 6% less | 48% less | 57% less |
| [`bit-liveness`](../../programs/backend/bit-liveness/README.md) | 28 | 628 | 140 of 140 | 2% less | 48% less | 57% less |
| [`compare-elim`](../../programs/backend/compare-elim/README.md) | 26 | 526 | 130 of 130 | 5% less | 51% less | 57% less |
| [`address-fold`](../../programs/backend/address-fold/README.md) | 28 | 584 | 140 of 140 | 8% more | 47% less | 58% less |
| [`load-fold`](../../programs/backend/load-fold/README.md) | 40 | 924 | 200 of 200 | 13% less | 49% less | 64% less |
| [`store-fold`](../../programs/backend/store-fold/README.md) | 56 | 1,416 | 280 of 280 | 13% less | 50% less | 64% less |
| [`store-fold-constant`](../../programs/backend/store-fold-constant/README.md) | 118 | 2,960 | 590 of 590 | 9% less | 49% less | 64% less |
| [`compare-fold`](../../programs/backend/compare-fold/README.md) | 118 | 2,742 | 590 of 590 | 8% less | 48% less | 65% less |
| [`frame-address`](../../programs/backend/frame-address/README.md) | 6 | 117 | 30 of 30 | 3% more | 43% less | 59% less |
| [`stack-slots`](../../programs/backend/stack-slots/README.md) | 6 | 201 | 30 of 30 | 5% less | 51% less | 71% less |
| [`bit-builtins`](../../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 110 of 110 | 83% more | 47% less | 64% less |
| [`float-conversion`](../../programs/backend/float-conversion/README.md) | 66 | 1,477 | 330 of 330 | 1% less | 45% less | 58% less |
| [`long-double`](../../programs/backend/long-double/README.md) | 10 | 226 | 50 of 50 | 58% more | 44% less | 60% less |

The reference compiler said it took 1847 transformations in this phase and wanted 8677 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
