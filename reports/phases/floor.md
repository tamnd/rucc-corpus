# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 1% less | 35% less | 13% more |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 10 | 260 | 50 of 50 | 4% more | 40% less | 115% more |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 2% less | 34% less | 159% more |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 125 of 125 | 19% less | 41% less | 73% more |
| [`vla-and-alloca`](../../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 45 of 45 | 22% less | 52% less | 61% more |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | 5% less | 33% less | 163% more |

The reference compiler said it took 192 transformations in this phase and wanted 1083 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
