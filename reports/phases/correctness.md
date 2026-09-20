# correctness

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`barrier`](../../programs/correctness/barrier/README.md) | 13 | 241 | 65 of 65 | 2% less | 51% less | 65% less |
| [`atomics`](../../programs/correctness/atomics/README.md) | 31 | 1,334 | 150 of 155 | 8% more | 50% less | 61% less |
| [`setjmp-longjmp`](../../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 30 of 40 | 11% less | 44% less | 61% less |

The reference compiler said it took 36 transformations in this phase and wanted 4454 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
