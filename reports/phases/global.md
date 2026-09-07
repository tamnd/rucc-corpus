# global

Transformations across the blocks of one function, which need dataflow rather than a peephole.

This run had no compiler under test in it, so the only thing this page can say about each facet is how many programs are in it and where they are.

| facet | cases |
|---|---|
| [`common-subexpr`](../../programs/global/common-subexpr/README.md) | 16 |
| [`load-forwarding`](../../programs/global/load-forwarding/README.md) | 40 |
| [`code-motion`](../../programs/global/code-motion/README.md) | 12 |
| [`copy-propagation`](../../programs/global/copy-propagation/README.md) | 12 |
| [`constant-propagation`](../../programs/global/constant-propagation/README.md) | 16 |
| [`value-range`](../../programs/global/value-range/README.md) | 13 |
| [`alias-analysis`](../../programs/global/alias-analysis/README.md) | 36 |
| [`memory-ssa`](../../programs/global/memory-ssa/README.md) | 28 |
| [`scalar-replacement`](../../programs/global/scalar-replacement/README.md) | 20 |

The reference compiler said it took 168 transformations in this phase and wanted 740 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
