# asm-goto

asm goto as static keys and user copies use it, with and without outputs. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`asm-goto.branch.c17.bd8abef0`](asm-goto.branch.c17.bd8abef0.c) | shape=branch | c17 | `3249` |
| [`asm-goto.in-loop.c17.d3655c03`](asm-goto.in-loop.c17.d3655c03.c) | shape=in-loop | c17 | `1736138884` |
| [`asm-goto.output.c17.b939b5fe`](asm-goto.output.c17.b939b5fe.c) | shape=output | c17 | `2840` |
| [`asm-goto.static-key-off.c17.429621de`](asm-goto.static-key-off.c17.429621de.c) | shape=static-key-off | c17 | `14119` |
| [`asm-goto.static-key-on.c17.afdb5d04`](asm-goto.static-key-on.c17.afdb5d04.c) | shape=static-key-on | c17 | `3854620103` |
| [`asm-goto.two-labels.c17.c9d186cc`](asm-goto.two-labels.c17.c9d186cc.c) | shape=two-labels | c17 | `5383` |

