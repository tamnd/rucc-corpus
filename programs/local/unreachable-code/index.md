# unreachable-code

removing a branch with a known condition and its dead arm. Part of the local phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`unreachable-code.early-return.c17.e088336e`](unreachable-code.early-return.c17.e088336e.c) | shape=early-return | c17 | `11` |
| [`unreachable-code.if-false.c17.8c3596cd`](unreachable-code.if-false.c17.8c3596cd.c) | shape=if-false | c17 | `11` |
| [`unreachable-code.if-true.c17.fe92dba7`](unreachable-code.if-true.c17.fe92dba7.c) | shape=if-true | c17 | `11` |
| [`unreachable-code.switch-constant.c17.7ad55f72`](unreachable-code.switch-constant.c17.7ad55f72.c) | shape=switch-constant | c17 | `11` |
| [`unreachable-code.while-false.c17.815f8255`](unreachable-code.while-false.c17.815f8255.c) | shape=while-false | c17 | `11` |

