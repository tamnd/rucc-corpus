# correctness

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`barrier`](../../programs/correctness/barrier/README.md) | 7 | 124 | 35 of 35 | 2% more | 48% less | 54% less |
| [`atomics`](../../programs/correctness/atomics/README.md) | 31 | 1,334 | 150 of 155 | 15% more | 56% less | 65% less |
| [`setjmp-longjmp`](../../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 30 of 40 | 7% less | 47% less | 58% less |

The reference compiler said it took 21 transformations in this phase and wanted 4379 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
