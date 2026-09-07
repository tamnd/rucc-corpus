# loops

Transformations that need loop structure, which is where most of the remaining time in real programs goes.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases |
|---|---|
| [`loop-invariant`](../../programs/loops/loop-invariant/README.md) | 32 |
| [`induction-variable`](../../programs/loops/induction-variable/README.md) | 42 |
| [`loop-unswitch`](../../programs/loops/loop-unswitch/README.md) | 64 |
| [`loop-unroll`](../../programs/loops/loop-unroll/README.md) | 64 |
| [`loop-idiom`](../../programs/loops/loop-idiom/README.md) | 64 |
| [`loop-deletion`](../../programs/loops/loop-deletion/README.md) | 24 |
| [`loop-rotate`](../../programs/loops/loop-rotate/README.md) | 64 |
| [`loop-restructure`](../../programs/loops/loop-restructure/README.md) | 48 |

The reference compiler said it took 1487 transformations in this phase and wanted 656 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
