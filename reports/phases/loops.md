# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 | 640 | 160 of 160 | 14% more | 52% less | 59% less |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 | 863 | 210 of 210 | 25% more | 51% less | 59% less |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 | 1,408 | 320 of 320 | 7% more | 54% less | 60% less |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 | 1,056 | 320 of 320 | 9% more | 52% less | 60% less |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 64 | 1,312 | 320 of 320 | 17% more | 54% less | 64% less |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 24 | 516 | 120 of 120 | 29% more | 51% less | 61% less |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 | 1,072 | 320 of 320 | 9% more | 49% less | 59% less |
| [`loop-shape`](../../programs/loops/loop-shape/README.md) | 64 | 1,416 | 320 of 320 | 11% more | 56% less | 67% less |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 | 960 | 240 of 240 | 32% more | 54% less | 61% less |

The reference compiler said it took 1590 transformations in this phase and wanted 980 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
