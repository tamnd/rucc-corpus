# local

Transformations that need to see no further than one basic block, which is where the cheapest wins are.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`constant-fold`](../../programs/local/constant-fold/README.md) | 112 | 6,438 | 560 of 560 | 2% less | 44% less | 27% less |
| [`strength`](../../programs/local/strength/README.md) | 24 | 2,656 | 120 of 120 | 3% more | 59% less | 35% less |
| [`narrowing`](../../programs/local/narrowing/README.md) | 52 | 2,204 | 260 of 260 | 1% less | 38% less | 10% less |
| [`simplify`](../../programs/local/simplify/README.md) | 114 | 6,416 | 570 of 570 | 2% less | 47% less | 33% less |
| [`short-circuit`](../../programs/local/short-circuit/README.md) | 26 | 704 | 130 of 130 | 42% less | 50% less | 36% less |
| [`conditional-store`](../../programs/local/conditional-store/README.md) | 20 | 705 | 100 of 100 | 42% less | 52% less | 42% less |
| [`value-settled`](../../programs/local/value-settled/README.md) | 13 | 372 | 65 of 65 | 32% less | 45% less | 35% less |
| [`reassociate`](../../programs/local/reassociate/README.md) | 20 | 440 | 100 of 100 | 2% more | 34% less | 9% less |
| [`dead-code`](../../programs/local/dead-code/README.md) | 12 | 264 | 60 of 60 | 6% less | 35% less | 10% less |
| [`dead-store`](../../programs/local/dead-store/README.md) | 12 | 220 | 60 of 60 | 6% less | 35% less | 10% less |
| [`unreachable-code`](../../programs/local/unreachable-code/README.md) | 8 | 162 | 40 of 40 | 6% less | 33% less | 9% less |
| [`libcall-fold`](../../programs/local/libcall-fold/README.md) | 20 | 488 | 100 of 100 | 7% less | 41% less | 18% less |

The reference compiler said it took 608 transformations in this phase and wanted 2795 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
