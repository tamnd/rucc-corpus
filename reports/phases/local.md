# local

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`constant-fold`](../../programs/local/constant-fold/README.md) | 112 | 6,438 | 560 of 560 | level | 42% less | level |
| [`strength`](../../programs/local/strength/README.md) | 24 | 2,656 | 120 of 120 | 5% more | 57% less | 36% more |
| [`narrowing`](../../programs/local/narrowing/README.md) | 52 | 2,204 | 260 of 260 | level | 38% less | 76% more |
| [`simplify`](../../programs/local/simplify/README.md) | 114 | 6,416 | 570 of 570 | level | 47% less | 16% more |
| [`short-circuit`](../../programs/local/short-circuit/README.md) | 24 | 650 | 120 of 120 | 37% less | 49% less | 45% more |
| [`conditional-store`](../../programs/local/conditional-store/README.md) | 20 | 705 | 100 of 100 | 34% less | 50% less | level |
| [`value-settled`](../../programs/local/value-settled/README.md) | 13 | 372 | 65 of 65 | 26% less | 48% less | 1% more |
| [`reassociate`](../../programs/local/reassociate/README.md) | 20 | 440 | 100 of 100 | 4% more | 34% less | 67% more |
| [`dead-code`](../../programs/local/dead-code/README.md) | 12 | 264 | 60 of 60 | 5% less | 37% less | 414% more |
| [`dead-store`](../../programs/local/dead-store/README.md) | 12 | 220 | 60 of 60 | 5% less | 39% less | 350% more |
| [`unreachable-code`](../../programs/local/unreachable-code/README.md) | 8 | 162 | 40 of 40 | 5% less | 31% less | level |

The reference compiler said it took 438 transformations in this phase and wanted 2494 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
