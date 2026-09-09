# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 | 640 | 160 of 160 | 10% more | 50% less | 59% less |
| [`loop-hoist`](../../programs/loops/loop-hoist/README.md) | 56 | 1,328 | 280 of 280 | 28% more | 51% less | 59% less |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 | 863 | 210 of 210 | 32% more | 48% less | 60% less |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 320 of 320 | 21% more | 55% less | 66% less |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 320 of 320 | 5% more | 51% less | 59% less |
| [`loop-unroll-shape`](../../programs/loops/loop-unroll-shape/README.md) | 80 | 1,348 | 400 of 400 | 8% more | 48% less | 59% less |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 64 | 1,312 | 320 of 320 | 43% more | 53% less | 63% less |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 24 | 516 | 120 of 120 | 31% more | 51% less | 59% less |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 320 of 320 | 9% more | 48% less | 58% less |
| [`loop-shape`](../../programs/loops/loop-shape/README.md) | 64 | 1,416 | 320 of 320 | 11% more | 54% less | 60% less |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 | 960 | 240 of 240 | 124% more | 51% less | 63% less |

The reference compiler said it took 1998 transformations in this phase and wanted 1385 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
