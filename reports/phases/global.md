# global

Transformations across the blocks of one function, which need dataflow rather than a peephole.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`common-subexpr`](../../programs/global/common-subexpr/README.md) | 32 | 740 | 160 of 160 | 5% less | 33% less | 163% more |
| [`load-forwarding`](../../programs/global/load-forwarding/README.md) | 40 | 1,008 | 200 of 200 | 5% less | 34% less | 159% more |
| [`code-motion`](../../programs/global/code-motion/README.md) | 12 | 288 | 60 of 60 | 3% less | 31% less | 167% more |
| [`copy-propagation`](../../programs/global/copy-propagation/README.md) | 12 | 336 | 60 of 60 | 6% less | 29% less | 181% more |
| [`constant-propagation`](../../programs/global/constant-propagation/README.md) | 16 | 268 | 80 of 80 | 5% less | 31% less | 176% more |
| [`value-range`](../../programs/global/value-range/README.md) | 13 | 310 | 65 of 65 | 5% more | 37% less | 173% more |
| [`prune`](../../programs/global/prune/README.md) | 15 | 472 | 75 of 75 | 35% less | 41% less | 79% more |
| [`jump-threading`](../../programs/global/jump-threading/README.md) | 5 | 224 | 25 of 25 | 13% less | 48% less | 63% more |
| [`value-replacement`](../../programs/global/value-replacement/README.md) | 13 | 360 | 65 of 65 | 33% less | 48% less | 1392% more |
| [`alias-analysis`](../../programs/global/alias-analysis/README.md) | 36 | 792 | 180 of 180 | 4% less | 34% less | 114% more |
| [`memory-ssa`](../../programs/global/memory-ssa/README.md) | 28 | 620 | 140 of 140 | 4% less | 32% less | 162% more |
| [`scalar-replacement`](../../programs/global/scalar-replacement/README.md) | 20 | 376 | 100 of 100 | 4% less | 34% less | 163% more |

The reference compiler said it took 326 transformations in this phase and wanted 1174 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
