# global

Transformations across the blocks of one function, which need dataflow rather than a peephole.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`common-subexpr`](../../programs/global/common-subexpr/README.md) | 32 | 740 | 160 of 160 | 4% less | 32% less | 164% more |
| [`load-forwarding`](../../programs/global/load-forwarding/README.md) | 40 | 1,008 | 200 of 200 | 5% less | 32% less | 168% more |
| [`code-motion`](../../programs/global/code-motion/README.md) | 12 | 288 | 60 of 60 | 2% less | 34% less | 145% more |
| [`copy-propagation`](../../programs/global/copy-propagation/README.md) | 12 | 336 | 60 of 60 | 6% less | 31% less | 149% more |
| [`constant-propagation`](../../programs/global/constant-propagation/README.md) | 16 | 268 | 80 of 80 | 5% less | 29% less | 153% more |
| [`value-range`](../../programs/global/value-range/README.md) | 13 | 310 | 65 of 65 | 7% more | 37% less | 127% more |
| [`prune`](../../programs/global/prune/README.md) | 15 | 472 | 75 of 75 | 32% less | 43% less | 79% more |
| [`jump-threading`](../../programs/global/jump-threading/README.md) | 5 | 224 | 25 of 25 | 14% less | 45% less | 64% more |
| [`value-replacement`](../../programs/global/value-replacement/README.md) | 13 | 360 | 65 of 65 | 30% less | 47% less | 92% more |
| [`alias-analysis`](../../programs/global/alias-analysis/README.md) | 36 | 792 | 180 of 180 | 4% less | 35% less | 128% more |
| [`memory-ssa`](../../programs/global/memory-ssa/README.md) | 28 | 620 | 140 of 140 | 4% less | 35% less | 170% more |
| [`scalar-replacement`](../../programs/global/scalar-replacement/README.md) | 20 | 376 | 100 of 100 | 4% less | 35% less | 120% more |

The reference compiler said it took 326 transformations in this phase and wanted 1174 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
