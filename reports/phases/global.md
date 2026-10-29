# global

Transformations across the blocks of one function, which need dataflow rather than a peephole.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`common-subexpr`](../../programs/global/common-subexpr/README.md) | 32 | 740 | 160 of 160 | 6% less | 39% less | 10% less |
| [`load-forwarding`](../../programs/global/load-forwarding/README.md) | 40 | 1,008 | 200 of 200 | 6% less | 38% less | 11% less |
| [`code-motion`](../../programs/global/code-motion/README.md) | 12 | 288 | 60 of 60 | 6% less | 37% less | 11% less |
| [`copy-propagation`](../../programs/global/copy-propagation/README.md) | 12 | 336 | 60 of 60 | 8% less | 36% less | 8% less |
| [`constant-propagation`](../../programs/global/constant-propagation/README.md) | 16 | 268 | 80 of 80 | 6% less | 36% less | 10% less |
| [`value-range`](../../programs/global/value-range/README.md) | 13 | 310 | 65 of 65 | 2% more | 33% less | 10% less |
| [`prune`](../../programs/global/prune/README.md) | 15 | 472 | 75 of 75 | 37% less | 45% less | 35% less |
| [`jump-threading`](../../programs/global/jump-threading/README.md) | 5 | 224 | 25 of 25 | 19% less | 51% less | 43% less |
| [`value-replacement`](../../programs/global/value-replacement/README.md) | 13 | 360 | 65 of 65 | 35% less | 47% less | 35% less |
| [`alias-analysis`](../../programs/global/alias-analysis/README.md) | 36 | 792 | 180 of 180 | 6% less | 34% less | 11% less |
| [`memory-ssa`](../../programs/global/memory-ssa/README.md) | 28 | 620 | 140 of 140 | 6% less | 37% less | 12% less |
| [`scalar-replacement`](../../programs/global/scalar-replacement/README.md) | 20 | 376 | 100 of 100 | 6% less | 36% less | 10% less |

The reference compiler said it took 326 transformations in this phase and wanted 1174 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
