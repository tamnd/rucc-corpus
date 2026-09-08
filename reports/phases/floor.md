# floor

The pass infrastructure, the verifiers and the cost model, which is what every later phase is built on.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases |
|---|---|
| [`baseline`](../../programs/floor/baseline/README.md) | 10 |
| [`control-flow`](../../programs/floor/control-flow/README.md) | 10 |
| [`branch-probability`](../../programs/floor/branch-probability/README.md) | 34 |
| [`computed-goto`](../../programs/floor/computed-goto/README.md) | 25 |
| [`frontend`](../../programs/floor/frontend/README.md) | 30 |

The reference compiler said it took 125 transformations in this phase and wanted 919 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
