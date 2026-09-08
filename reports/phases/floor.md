# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 12% more | 50% less | 57% less |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 10 | 260 | 45 of 50 | 19% more | 55% less | 58% less |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 5% more | 48% less | 57% less |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 0 of 125 | not measured | 91% less | 86% less |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | level | 50% less | 56% less |

The reference compiler said it took 142 transformations in this phase and wanted 871 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
