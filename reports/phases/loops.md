# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 | 640 | 160 of 160 | 2% less | 45% less | 54% less |
| [`loop-hoist`](../../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 320 of 320 | 9% more | 47% less | 55% less |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 | 863 | 210 of 210 | 3% more | 46% less | 55% less |
| [`iv-selection`](../../programs/loops/iv-selection/README.md) | 36 | 760 | 180 of 180 | 20% less | 55% less | 64% less |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 320 of 320 | 2% less | 49% less | 64% less |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 320 of 320 | 8% less | 48% less | 55% less |
| [`loop-unroll-shape`](../../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 400 of 400 | 9% less | 46% less | 54% less |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 390 of 390 | 8% more | 48% less | 55% less |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 320 of 320 | 6% less | 45% less | 55% less |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 320 of 320 | 9% less | 44% less | 54% less |
| [`loop-shape`](../../programs/loops/loop-shape/README.md) | 64 | 1,416 | 320 of 320 | 4% less | 51% less | 66% less |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 | 960 | 240 of 240 | 39% more | 43% less | 56% less |

The reference compiler said it took 2591 transformations in this phase and wanted 1657 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
