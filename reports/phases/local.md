# local

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`constant-fold`](../../programs/local/constant-fold/README.md) | 112 | 6,438 | 560 of 560 | 2% less | 49% less | 32% less |
| [`strength`](../../programs/local/strength/README.md) | 24 | 2,656 | 120 of 120 | 3% more | 62% less | 46% less |
| [`narrowing`](../../programs/local/narrowing/README.md) | 52 | 2,204 | 260 of 260 | 2% less | 43% less | 35% less |
| [`simplify`](../../programs/local/simplify/README.md) | 114 | 6,416 | 570 of 570 | 2% less | 50% less | 36% less |
| [`short-circuit`](../../programs/local/short-circuit/README.md) | 26 | 704 | 130 of 130 | 40% less | 52% less | 35% less |
| [`conditional-store`](../../programs/local/conditional-store/README.md) | 20 | 705 | 100 of 100 | 38% less | 55% less | 47% less |
| [`value-settled`](../../programs/local/value-settled/README.md) | 13 | 372 | 65 of 65 | 27% less | 49% less | 30% less |
| [`reassociate`](../../programs/local/reassociate/README.md) | 20 | 440 | 100 of 100 | 3% more | 40% less | 34% less |
| [`dead-code`](../../programs/local/dead-code/README.md) | 12 | 264 | 60 of 60 | 8% less | 40% less | 37% less |
| [`dead-store`](../../programs/local/dead-store/README.md) | 12 | 220 | 60 of 60 | 8% less | 40% less | 16% less |
| [`unreachable-code`](../../programs/local/unreachable-code/README.md) | 8 | 162 | 40 of 40 | 7% less | 42% less | level |

The reference compiler said it took 445 transformations in this phase and wanted 2504 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
