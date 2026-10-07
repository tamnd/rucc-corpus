# local

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`constant-fold`](../../programs/local/constant-fold/README.md) | 112 | 6,438 | 560 of 560 | 2% less | 40% less | 94% more |
| [`strength`](../../programs/local/strength/README.md) | 24 | 2,656 | 120 of 120 | 3% more | 59% less | 7% more |
| [`narrowing`](../../programs/local/narrowing/README.md) | 52 | 2,204 | 260 of 260 | level | 35% less | 168% more |
| [`simplify`](../../programs/local/simplify/README.md) | 114 | 6,416 | 570 of 570 | 2% less | 43% less | 79% more |
| [`short-circuit`](../../programs/local/short-circuit/README.md) | 26 | 704 | 130 of 130 | 39% less | 46% less | 76% more |
| [`conditional-store`](../../programs/local/conditional-store/README.md) | 20 | 705 | 100 of 100 | 37% less | 48% less | 67% more |
| [`value-settled`](../../programs/local/value-settled/README.md) | 13 | 372 | 65 of 65 | 26% less | 42% less | 81% more |
| [`reassociate`](../../programs/local/reassociate/README.md) | 20 | 440 | 100 of 100 | 4% more | 30% less | 166% more |
| [`dead-code`](../../programs/local/dead-code/README.md) | 12 | 264 | 60 of 60 | 5% less | 31% less | 179% more |
| [`dead-store`](../../programs/local/dead-store/README.md) | 12 | 220 | 60 of 60 | 5% less | 32% less | 181% more |
| [`unreachable-code`](../../programs/local/unreachable-code/README.md) | 8 | 162 | 40 of 40 | 5% less | 31% less | 960% more |

The reference compiler said it took 445 transformations in this phase and wanted 2504 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
