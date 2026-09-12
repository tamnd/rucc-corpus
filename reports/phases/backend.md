# backend

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`selection`](../../programs/backend/selection/README.md) | 40 | 680 | 200 of 200 | 5% less | 45% less | level |
| [`register-pressure`](../../programs/backend/register-pressure/README.md) | 60 | 3,120 | 300 of 300 | 47% more | 47% less | 58% less |
| [`register-alloc`](../../programs/backend/register-alloc/README.md) | 40 | 2,664 | 200 of 200 | 75% more | 50% less | 59% less |
| [`scheduling`](../../programs/backend/scheduling/README.md) | 12 | 832 | 60 of 60 | 71% more | 50% less | 29% less |
| [`block-layout`](../../programs/backend/block-layout/README.md) | 4 | 91 | 20 of 20 | 4% less | 46% less | 53% less |
| [`if-conversion`](../../programs/backend/if-conversion/README.md) | 20 | 444 | 100 of 100 | 1% more | 54% less | 63% less |
| [`switch-lowering`](../../programs/backend/switch-lowering/README.md) | 15 | 705 | 75 of 75 | 11% more | 59% less | level |
| [`switch-runs`](../../programs/backend/switch-runs/README.md) | 15 | 1,356 | 75 of 75 | 14% more | 52% less | 68% less |
| [`switch-dispatch`](../../programs/backend/switch-dispatch/README.md) | 9 | 415 | 45 of 45 | 7% less | 59% less | level |
| [`calling-convention`](../../programs/backend/calling-convention/README.md) | 10 | 216 | 50 of 50 | 24% more | 48% less | 48% less |
| [`machine-peephole`](../../programs/backend/machine-peephole/README.md) | 22 | 402 | 110 of 110 | 4% less | 47% less | 54% less |
| [`address-fold`](../../programs/backend/address-fold/README.md) | 28 | 584 | 140 of 140 | 16% more | 48% less | 56% less |
| [`frame-address`](../../programs/backend/frame-address/README.md) | 6 | 117 | 30 of 30 | 6% more | 42% less | 58% less |
| [`bit-builtins`](../../programs/backend/bit-builtins/README.md) | 22 | 1,316 | 110 of 110 | 88% more | 54% less | 55% less |
| [`float-conversion`](../../programs/backend/float-conversion/README.md) | 66 | 1,477 | 330 of 330 | 27% more | 48% less | 58% less |
| [`long-double`](../../programs/backend/long-double/README.md) | 10 | 226 | 50 of 50 | 76% more | 49% less | 57% less |

The reference compiler said it took 459 transformations in this phase and wanted 5875 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
