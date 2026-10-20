# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 3% less | 41% less | 27% less |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 11 | 291 | 55 of 55 | 3% less | 42% less | 30% less |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 5% less | 39% less | 73% more |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 125 of 125 | 24% less | 49% less | 38% less |
| [`vla-and-alloca`](../../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 45 of 45 | 26% less | 54% less | 22% less |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | 7% less | 39% less | 34% less |

The reference compiler said it took 201 transformations in this phase and wanted 1098 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
