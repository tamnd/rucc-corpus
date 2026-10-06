# asm-local-labels

numeric labels, the %= number and a label another section names. Part of the correctness phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`asm-local-labels.backward.c17.03e01722`](asm-local-labels.backward.c17.03e01722.c) | shape=backward | c17 | `9543` |
| [`asm-local-labels.forward.c17.c7069b9d`](asm-local-labels.forward.c17.c7069b9d.c) | shape=forward | c17 | `1863996935` |
| [`asm-local-labels.inlined-copies.c17.4c7d5942`](asm-local-labels.inlined-copies.c17.4c7d5942.c) | shape=inlined-copies | c17 | `2426` |
| [`asm-local-labels.pushsection.c17.718d8389`](asm-local-labels.pushsection.c17.718d8389.c) | shape=pushsection | c17 | `3367` |
| [`asm-local-labels.unique-number.c17.b824349c`](asm-local-labels.unique-number.c17.b824349c.c) | shape=unique-number | c17 | `1768705308` |

