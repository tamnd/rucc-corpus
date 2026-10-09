# interprocedural

Transformations that need to look at more than one function at a time.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`inline`](../../programs/interprocedural/inline/README.md) | 36 | 1,200 | 180 of 180 | 5% less | 41% less | 25% less |
| [`tail-call`](../../programs/interprocedural/tail-call/README.md) | 36 | 876 | 180 of 180 | 12% more | 45% less | 30% less |
| [`tail-dispatch`](../../programs/interprocedural/tail-dispatch/README.md) | 5 | 893 | 25 of 25 | 10% more | 64% less | 1620% more |
| [`called-once`](../../programs/interprocedural/called-once/README.md) | 5 | 413 | 25 of 25 | 1% less | 61% less | 96% less |
| [`function-purity`](../../programs/interprocedural/function-purity/README.md) | 28 | 636 | 140 of 140 | 7% less | 44% less | 28% less |
| [`constant-args`](../../programs/interprocedural/constant-args/README.md) | 36 | 792 | 180 of 180 | 6% less | 43% less | 25% less |
| [`unused-params`](../../programs/interprocedural/unused-params/README.md) | 44 | 948 | 220 of 220 | 7% less | 43% less | 24% less |
| [`unused-returns`](../../programs/interprocedural/unused-returns/README.md) | 36 | 912 | 180 of 180 | 6% less | 42% less | 26% less |
| [`declared-purity`](../../programs/interprocedural/declared-purity/README.md) | 32 | 856 in 64 files | 160 of 160 | 14% less | 57% less | 25% less |
| [`reachability`](../../programs/interprocedural/reachability/README.md) | 9 | 247 | 45 of 45 | 7% less | 42% less | 26% less |
| [`devirtualize`](../../programs/interprocedural/devirtualize/README.md) | 4 | 80 | 20 of 20 | 6% less | 44% less | 96% less |
| [`memory-effects`](../../programs/interprocedural/memory-effects/README.md) | 24 | 760 | 120 of 120 | 6% less | 50% less | 38% less |
| [`call-motion`](../../programs/interprocedural/call-motion/README.md) | 36 | 1,004 in 48 files | 180 of 180 | 5% less | 49% less | 30% less |
| [`link-time-optimization`](../../programs/interprocedural/link-time-optimization/README.md) | 20 | 556 in 44 files | 100 of 100 | 6% less | 65% less | 22% less |

The reference compiler said it took 2121 transformations in this phase and wanted 5369 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

## Switches lowered differently

Each of these cases has a `switch` that a compiler under test lowered into a different shape from the one `gcc-16` used: a jump table, a bit test, a lookup table of answers, or compares when it was none of the three. It is a lead to read the assembly for rather than a failure. The run's `report.md` says how many cases agreed.

| compiler | level | case | it used | `gcc-16` used |
|---|---|---|---|---|
| `rucc` | O0 | `tail-dispatch.switch.16.c17.e756d443` | table | compares |

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
