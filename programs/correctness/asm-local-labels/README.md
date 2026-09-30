# asm-local-labels

numeric labels, the %= number and a label another section names. Part of the correctness phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`asm-local-labels.backward.c17.dce84780`](asm-local-labels.backward.c17.dce84780.c) | shape=backward | c17 | `9543` |
| [`asm-local-labels.forward.c17.657cf2e4`](asm-local-labels.forward.c17.657cf2e4.c) | shape=forward | c17 | `1863996935` |
| [`asm-local-labels.inlined-copies.c17.d7c43c59`](asm-local-labels.inlined-copies.c17.d7c43c59.c) | shape=inlined-copies | c17 | `2426` |
| [`asm-local-labels.pushsection.c17.d8176bc4`](asm-local-labels.pushsection.c17.d8176bc4.c) | shape=pushsection | c17 | `3367` |
| [`asm-local-labels.unique-number.c17.7f861ad3`](asm-local-labels.unique-number.c17.7f861ad3.c) | shape=unique-number | c17 | `1768705308` |

