# interprocedural

Transformations that need to look at more than one function at a time.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`inline`](../../programs/interprocedural/inline/README.md) | 36 | 1,200 | 180 of 180 | 17% more | 49% less | 57% less |
| [`tail-call`](../../programs/interprocedural/tail-call/README.md) | 36 | 876 | 180 of 180 | 11% more | 53% less | 60% less |
| [`function-purity`](../../programs/interprocedural/function-purity/README.md) | 20 | 440 | 100 of 100 | 17% more | 51% less | 60% less |
| [`constant-args`](../../programs/interprocedural/constant-args/README.md) | 12 | 244 | 60 of 60 | 11% more | 49% less | 58% less |
| [`reachability`](../../programs/interprocedural/reachability/README.md) | 9 | 247 | 45 of 45 | 5% less | 48% less | 57% less |
| [`devirtualize`](../../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 20 of 20 | 6% more | 51% less | 28% less |
| [`link-time-optimization`](../../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 50 of 100 | 2% more | 83% less | 55% less |

The reference compiler said it took 1387 transformations in this phase and wanted 1210 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
