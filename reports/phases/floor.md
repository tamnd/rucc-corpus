# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 | 241 | 50 of 50 | 5% less | 39% less | 2% less |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 11 | 291 | 55 of 55 | 6% less | 43% less | 23% less |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 | 704 | 170 of 170 | 6% less | 37% less | level |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 | 1,614 | 125 of 125 | 27% less | 41% less | 33% less |
| [`vla-and-alloca`](../../programs/floor/vla-and-alloca/README.md) | 9 | 236 | 45 of 45 | 27% less | 54% less | 35% less |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 | 403 | 118 of 118 | 6% less | 37% less | 1% more |
| [`brace-elision`](../../programs/floor/brace-elision/README.md) | 20 | 329 | 100 of 100 | 6% less | 36% less | 1% less |
| [`kencc-members`](../../programs/floor/kencc-members/README.md) | 17 | 325 | 81 of 81 | 6% less | 28% less | level |
| [`kencc-varargck`](../../programs/floor/kencc-varargck/README.md) | 89 | 7,614 | 261 of 261 | not measured | not measured | not measured |
| [`kencc-conversions`](../../programs/floor/kencc-conversions/README.md) | 64 | 2,560 | 320 of 320 | not measured | not measured | not measured |
| [`kencc-layout`](../../programs/floor/kencc-layout/README.md) | 15 | 288 | 75 of 75 | not measured | not measured | not measured |
| [`kencc-regs`](../../programs/floor/kencc-regs/README.md) | 6 | 140 | 30 of 30 | not measured | not measured | not measured |

The reference compiler said it took 357 transformations in this phase and wanted 1148 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
