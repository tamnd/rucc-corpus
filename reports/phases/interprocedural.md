# interprocedural

Transformations that need to look at more than one function at a time.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`inline`](../../programs/interprocedural/inline/README.md) | 36 | 1,200 | 180 of 180 | 15% more | 46% less | 57% less |
| [`tail-call`](../../programs/interprocedural/tail-call/README.md) | 36 | 876 | 180 of 180 | 9% more | 52% less | 60% less |
| [`function-purity`](../../programs/interprocedural/function-purity/README.md) | 28 | 636 | 140 of 140 | 2% more | 46% less | 62% less |
| [`constant-args`](../../programs/interprocedural/constant-args/README.md) | 36 | 792 | 180 of 180 | 2% more | 50% less | 58% less |
| [`unused-params`](../../programs/interprocedural/unused-params/README.md) | 44 | 948 | 220 of 220 | 2% more | 49% less | 58% less |
| [`unused-returns`](../../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 180 of 180 | 2% less | 48% less | 60% less |
| [`declared-purity`](../../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 160 of 160 | 12% less | 59% less | 53% less |
| [`reachability`](../../programs/interprocedural/reachability/README.md) | 9 | 247 | 45 of 45 | 7% less | 48% less | 56% less |
| [`devirtualize`](../../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 20 of 20 | 1% less | 50% less | 60% less |
| [`memory-effects`](../../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 120 of 120 | 1% less | 51% less | 69% less |
| [`call-motion`](../../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | 180 of 180 | 3% less | 54% less | 68% less |
| [`link-time-optimization`](../../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 100 of 100 | 2% less | 67% less | 55% less |

The reference compiler said it took 2097 transformations in this phase and wanted 3461 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
