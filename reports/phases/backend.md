# backend

Everything below the machine independent IR, where the cost of a decision is measured in instructions rather than in operations.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases |
|---|---|
| [`selection`](../../programs/backend/selection/README.md) | 40 |
| [`register-pressure`](../../programs/backend/register-pressure/README.md) | 48 |
| [`register-alloc`](../../programs/backend/register-alloc/README.md) | 40 |
| [`scheduling`](../../programs/backend/scheduling/README.md) | 12 |
| [`block-layout`](../../programs/backend/block-layout/README.md) | 4 |
| [`if-conversion`](../../programs/backend/if-conversion/README.md) | 20 |
| [`switch-lowering`](../../programs/backend/switch-lowering/README.md) | 9 |
| [`calling-convention`](../../programs/backend/calling-convention/README.md) | 10 |
| [`machine-peephole`](../../programs/backend/machine-peephole/README.md) | 22 |
| [`bit-builtins`](../../programs/backend/bit-builtins/README.md) | 22 |
| [`float-conversion`](../../programs/backend/float-conversion/README.md) | 66 |

The reference compiler said it took 339 transformations in this phase and wanted 5184 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
