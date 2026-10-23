# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 | 640 | 160 of 160 | 6% less | 43% less | 38% more |
| [`loop-hoist`](../../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 320 of 320 | 6% less | 42% less | 36% less |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 | 863 | 210 of 210 | 6% less | 43% less | 72% more |
| [`iv-selection`](../../programs/loops/iv-selection/README.md) | 36 | 760 | 180 of 180 | 29% less | 48% less | 36% less |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 320 of 320 | 2% less | 45% less | 34% less |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 320 of 320 | 10% less | 42% less | 30% less |
| [`loop-unroll-shape`](../../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 400 of 400 | 10% less | 42% less | 73% more |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 390 of 390 | 29% less | 43% less | 35% less |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 320 of 320 | 6% less | 36% less | 77% more |
| [`bit-loops`](../../programs/loops/bit-loops/README.md) | 16 | 564 | 80 of 80 | 1% less | 49% less | 34% less |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 320 of 320 | 10% less | 35% less | 75% more |
| [`loop-shape`](../../programs/loops/loop-shape/README.md) | 64 | 1,416 | 320 of 320 | 9% less | 46% less | 35% less |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 | 960 | 240 of 240 | 10% less | 39% less | 78% more |

The reference compiler said it took 2639 transformations in this phase and wanted 1857 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
