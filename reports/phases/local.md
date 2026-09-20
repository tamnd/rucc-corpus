# local

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`constant-fold`](../../programs/local/constant-fold/README.md) | 112 | 6,438 | 560 of 560 | 1% more | 48% less | 63% less |
| [`strength`](../../programs/local/strength/README.md) | 24 | 2,656 | 120 of 120 | 2% less | 58% less | 70% less |
| [`narrowing`](../../programs/local/narrowing/README.md) | 52 | 2,204 | 260 of 260 | 2% less | 45% less | 62% less |
| [`simplify`](../../programs/local/simplify/README.md) | 78 | 5,444 | 390 of 390 | 1% more | 48% less | 66% less |
| [`short-circuit`](../../programs/local/short-circuit/README.md) | 22 | 599 | 110 of 110 | 31% less | 54% less | 68% less |
| [`conditional-store`](../../programs/local/conditional-store/README.md) | 16 | 563 | 80 of 80 | 35% less | 54% less | 72% less |
| [`value-settled`](../../programs/local/value-settled/README.md) | 13 | 372 | 65 of 65 | 22% less | 51% less | 65% less |
| [`reassociate`](../../programs/local/reassociate/README.md) | 20 | 440 | 100 of 100 | 3% more | 46% less | 65% less |
| [`dead-code`](../../programs/local/dead-code/README.md) | 12 | 264 | 60 of 60 | 7% less | 44% less | 30% less |
| [`dead-store`](../../programs/local/dead-store/README.md) | 12 | 220 | 60 of 60 | 4% less | 47% less | 35% less |
| [`unreachable-code`](../../programs/local/unreachable-code/README.md) | 8 | 162 | 40 of 40 | 7% less | 50% less | 140% more |

The reference compiler said it took 257 transformations in this phase and wanted 2100 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
