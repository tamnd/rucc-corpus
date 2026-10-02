# interprocedural

Transformations that need to look at more than one function at a time.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`inline`](../../programs/interprocedural/inline/README.md) | 36 | 1,200 | 180 of 180 | 2% less | 35% less | 175% more |
| [`tail-call`](../../programs/interprocedural/tail-call/README.md) | 36 | 876 | 180 of 180 | 9% more | 39% less | 102% more |
| [`tail-dispatch`](../../programs/interprocedural/tail-dispatch/README.md) | 5 | 893 | 25 of 25 | 6% more | 58% less | 89% more |
| [`called-once`](../../programs/interprocedural/called-once/README.md) | 5 | 413 | 25 of 25 | 4% more | 54% less | 45% less |
| [`function-purity`](../../programs/interprocedural/function-purity/README.md) | 28 | 636 | 140 of 140 | 4% less | 39% less | 160% more |
| [`constant-args`](../../programs/interprocedural/constant-args/README.md) | 36 | 792 | 180 of 180 | 5% less | 37% less | 179% more |
| [`unused-params`](../../programs/interprocedural/unused-params/README.md) | 44 | 948 | 220 of 220 | 5% less | 33% less | 176% more |
| [`unused-returns`](../../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 180 of 180 | 3% less | 34% less | 162% more |
| [`declared-purity`](../../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 160 of 160 | 10% less | 51% less | 162% more |
| [`reachability`](../../programs/interprocedural/reachability/README.md) | 9 | 247 | 45 of 45 | 5% less | 37% less | 6% more |
| [`devirtualize`](../../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 20 of 20 | level | 31% less | 39% more |
| [`memory-effects`](../../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 120 of 120 | 2% less | 41% less | 93% more |
| [`call-motion`](../../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | 180 of 180 | 3% less | 44% less | 100% more |
| [`link-time-optimization`](../../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 100 of 100 | 2% less | 61% less | 174% more |

The reference compiler said it took 2121 transformations in this phase and wanted 5369 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

## Switches lowered differently

Each of these cases has a `switch` that a compiler under test lowered into a different shape from the one `gcc-16` used: a jump table, a bit test, a lookup table of answers, or compares when it was none of the three. It is a lead to read the assembly for rather than a failure. The run's `report.md` says how many cases agreed.

| compiler | level | case | it used | `gcc-16` used |
|---|---|---|---|---|
| `rucc` | O0 | `tail-dispatch.switch.16.c17.e756d443` | table | compares |

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
