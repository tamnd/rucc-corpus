# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 1% less | 32% less | 139% more |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 10 | 260 | 50 of 50 | 3% more | 39% less | level |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 2% less | 34% less | 168% more |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 125 of 125 | 19% less | 40% less | 86% more |
| [`vla-and-alloca`](../../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 45 of 45 | 22% less | 52% less | 77% more |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | 5% less | 37% less | 160% more |

The reference compiler said it took 192 transformations in this phase and wanted 1083 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
