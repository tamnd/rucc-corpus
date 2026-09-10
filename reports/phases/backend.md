# backend

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`selection`](../../programs/backend/selection/README.md) | 40 | 680 | 200 of 200 | 5% less | 48% less | 55% less |
| [`register-pressure`](../../programs/backend/register-pressure/README.md) | 48 | 2,460 | 240 of 240 | 51% more | 47% less | 58% less |
| [`register-alloc`](../../programs/backend/register-alloc/README.md) | 40 | 2,664 | 200 of 200 | 75% more | 49% less | 57% less |
| [`scheduling`](../../programs/backend/scheduling/README.md) | 12 | 832 | 60 of 60 | 74% more | 50% less | 62% less |
| [`block-layout`](../../programs/backend/block-layout/README.md) | 4 | 91 | 20 of 20 | 2% more | 49% less | 64% less |
| [`if-conversion`](../../programs/backend/if-conversion/README.md) | 20 | 444 | 100 of 100 | 8% more | 55% less | 60% less |
| [`switch-lowering`](../../programs/backend/switch-lowering/README.md) | 15 | 705 | 75 of 75 | 23% more | 55% less | 60% less |
| [`switch-runs`](../../programs/backend/switch-runs/README.md) | 15 | 1,356 | 75 of 75 | 20% more | 53% less | 68% less |
| [`calling-convention`](../../programs/backend/calling-convention/README.md) | 10 | 216 | 50 of 50 | 27% more | 47% less | 33% less |
| [`machine-peephole`](../../programs/backend/machine-peephole/README.md) | 22 | 402 | 110 of 110 | 4% less | 46% less | 55% less |
| [`bit-builtins`](../../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 110 of 110 | 282% more | 50% less | 65% less |
| [`float-conversion`](../../programs/backend/float-conversion/README.md) | 66 | 1,477 | 330 of 330 | 29% more | 48% less | 57% less |
| [`long-double`](../../programs/backend/long-double/README.md) | 10 | 226 | 50 of 50 | 105% more | 47% less | 30% less |

The reference compiler said it took 358 transformations in this phase and wanted 5519 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
