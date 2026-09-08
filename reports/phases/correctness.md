# correctness

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases |
|---|---|
| [`barrier`](../../programs/correctness/barrier/README.md) | 7 |
| [`atomics`](../../programs/correctness/atomics/README.md) | 31 |

The reference compiler said it took 6 transformations in this phase and wanted 4077 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
