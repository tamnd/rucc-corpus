# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 | 640 | 160 of 160 | 6% less | 42% less | 28% less |
| [`loop-hoist`](../../programs/loops/loop-hoist/README.md) | 64 | 1,620 | 320 of 320 | 4% less | 47% less | 29% less |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 | 863 | 210 of 210 | 7% less | 42% less | 29% less |
| [`iv-selection`](../../programs/loops/iv-selection/README.md) | 36 | 760 | 180 of 180 | 27% less | 53% less | 49% less |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 320 of 320 | 3% less | 48% less | 31% less |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 320 of 320 | 9% less | 46% less | 29% less |
| [`loop-unroll-shape`](../../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 400 of 400 | 9% less | 42% less | 28% less |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 78 | 1,600 | 390 of 390 | 29% less | 48% less | 31% less |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 64 | 1,408 | 320 of 320 | 8% less | 43% less | 27% less |
| [`bit-loops`](../../programs/loops/bit-loops/README.md) | 16 | 564 | 80 of 80 | level | 56% less | 49% less |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 320 of 320 | 9% less | 43% less | 27% less |
| [`loop-shape`](../../programs/loops/loop-shape/README.md) | 64 | 1,416 | 320 of 320 | 8% less | 47% less | 32% less |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 | 960 | 240 of 240 | 9% less | 44% less | 29% less |

The reference compiler said it took 2639 transformations in this phase and wanted 1857 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
