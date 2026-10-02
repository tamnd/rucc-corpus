# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 | 640 | 160 of 160 | 4% less | 37% less | 60% more |
| [`loop-hoist`](../../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 320 of 320 | 2% less | 39% less | 117% more |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 | 863 | 210 of 210 | 5% less | 35% less | 158% more |
| [`iv-selection`](../../programs/loops/iv-selection/README.md) | 36 | 760 | 180 of 180 | 21% less | 45% less | 88% more |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 320 of 320 | 1% less | 41% less | 98% more |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 320 of 320 | 7% less | 39% less | 111% more |
| [`loop-unroll-shape`](../../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 400 of 400 | 7% less | 36% less | 134% more |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 390 of 390 | 23% less | 39% less | 131% more |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 320 of 320 | 5% less | 35% less | 158% more |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 320 of 320 | 7% less | 33% less | 164% more |
| [`loop-shape`](../../programs/loops/loop-shape/README.md) | 64 | 1,416 | 320 of 320 | 3% less | 40% less | 101% more |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 | 960 | 240 of 240 | 38% more | 35% less | 152% more |

The reference compiler said it took 2591 transformations in this phase and wanted 1657 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
