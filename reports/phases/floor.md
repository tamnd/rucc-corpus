# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 3% less | 42% less | 16% less |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 11 | 291 | 55 of 55 | 6% less | 47% less | 31% less |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 6% less | 39% less | 15% less |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 125 of 125 | 27% less | 45% less | 42% less |
| [`vla-and-alloca`](../../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 45 of 45 | 27% less | 57% less | 45% less |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | 6% less | 41% less | 13% less |

The reference compiler said it took 201 transformations in this phase and wanted 1098 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
