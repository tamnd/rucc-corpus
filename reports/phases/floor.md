# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 15% more | 48% less | 58% less |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 10 | 260 | 45 of 50 | 18% more | 51% less | 39% less |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 5% more | 48% less | 55% less |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 0 of 125 | not measured | 92% less | 80% less |
| [`vla-and-alloca`](../../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 0 of 45 | not measured | 89% less | level |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | level | 47% less | 55% less |

The reference compiler said it took 192 transformations in this phase and wanted 1083 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
