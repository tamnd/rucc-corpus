# local

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases | lines |
|---|---|---|
| [`constant-fold`](../../programs/local/constant-fold/README.md) | 112 | 6,438 |
| [`strength`](../../programs/local/strength/README.md) | 24 | 2,656 |
| [`narrowing`](../../programs/local/narrowing/README.md) | 52 | 2,204 |
| [`simplify`](../../programs/local/simplify/README.md) | 69 | 4,595 |
| [`reassociate`](../../programs/local/reassociate/README.md) | 20 | 440 |
| [`dead-code`](../../programs/local/dead-code/README.md) | 12 | 264 |
| [`dead-store`](../../programs/local/dead-store/README.md) | 12 | 220 |
| [`unreachable-code`](../../programs/local/unreachable-code/README.md) | 8 | 162 |

The reference compiler said it took 3 transformations in this phase and wanted 1847 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
