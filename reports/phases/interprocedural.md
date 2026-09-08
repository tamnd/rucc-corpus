# interprocedural

Transformations that need to look at more than one function at a time.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`inline`](../../programs/interprocedural/inline/README.md) | 36 | 1,200 | 180 of 180 | 17% more | 50% less | 57% less |
| [`tail-call`](../../programs/interprocedural/tail-call/README.md) | 36 | 876 | 180 of 180 | 11% more | 53% less | 61% less |
| [`function-purity`](../../programs/interprocedural/function-purity/README.md) | 20 | 440 | 100 of 100 | 14% more | 52% less | 59% less |
| [`constant-args`](../../programs/interprocedural/constant-args/README.md) | 12 | 244 | 60 of 60 | 11% more | 48% less | 15% less |
| [`reachability`](../../programs/interprocedural/reachability/README.md) | 9 | 247 | 45 of 45 | 5% less | 50% less | 56% less |
| [`devirtualize`](../../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 20 of 20 | 6% more | 53% less | 57% less |

The reference compiler said it took 1370 transformations in this phase and wanted 1000 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
