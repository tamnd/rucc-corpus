# interprocedural

Transformations that need to look at more than one function at a time.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases | lines |
|---|---|---|
| [`inline`](../../programs/interprocedural/inline/README.md) | 36 | 1,200 |
| [`tail-call`](../../programs/interprocedural/tail-call/README.md) | 36 | 876 |
| [`function-purity`](../../programs/interprocedural/function-purity/README.md) | 20 | 440 |
| [`constant-args`](../../programs/interprocedural/constant-args/README.md) | 12 | 244 |
| [`reachability`](../../programs/interprocedural/reachability/README.md) | 9 | 247 |
| [`devirtualize`](../../programs/interprocedural/devirtualize/README.md) | 4 | 80 |

The reference compiler said it took 1394 transformations in this phase and wanted 980 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
