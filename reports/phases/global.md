# global

Transformations across the blocks of one function, which need dataflow rather than a peephole.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`common-subexpr`](../../programs/global/common-subexpr/README.md) | 16 | 376 | 80 of 80 | 2% less | 46% less | 57% less |
| [`load-forwarding`](../../programs/global/load-forwarding/README.md) | 40 | 1,008 | 200 of 200 | 4% more | 46% less | 55% less |
| [`code-motion`](../../programs/global/code-motion/README.md) | 12 | 288 | 60 of 60 | 1% more | 47% less | 56% less |
| [`copy-propagation`](../../programs/global/copy-propagation/README.md) | 12 | 336 | 60 of 60 | 7% less | 52% less | 58% less |
| [`constant-propagation`](../../programs/global/constant-propagation/README.md) | 16 | 268 | 80 of 80 | 5% less | 43% less | 54% less |
| [`value-range`](../../programs/global/value-range/README.md) | 13 | 310 | 65 of 65 | 18% more | 47% less | 57% less |
| [`prune`](../../programs/global/prune/README.md) | 15 | 472 | 75 of 75 | 5% less | 55% less | 71% less |
| [`alias-analysis`](../../programs/global/alias-analysis/README.md) | 36 | 792 | 180 of 180 | 8% more | 46% less | 55% less |
| [`memory-ssa`](../../programs/global/memory-ssa/README.md) | 28 | 620 | 140 of 140 | level | 46% less | 56% less |
| [`scalar-replacement`](../../programs/global/scalar-replacement/README.md) | 20 | 376 | 100 of 100 | 1% less | 48% less | 56% less |

The reference compiler said it took 221 transformations in this phase and wanted 775 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
