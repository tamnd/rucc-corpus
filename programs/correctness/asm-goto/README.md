# asm-goto

asm goto as static keys and user copies use it, with and without outputs. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`asm-goto.branch.c17.33690266`](asm-goto.branch.c17.33690266.c) | shape=branch | c17 | `3249` |
| [`asm-goto.in-loop.c17.340e5676`](asm-goto.in-loop.c17.340e5676.c) | shape=in-loop | c17 | `1736138884` |
| [`asm-goto.output.c17.2a15ca0e`](asm-goto.output.c17.2a15ca0e.c) | shape=output | c17 | `2840` |
| [`asm-goto.static-key-off.c17.c699bc6d`](asm-goto.static-key-off.c17.c699bc6d.c) | shape=static-key-off | c17 | `14119` |
| [`asm-goto.static-key-on.c17.7b3f5e16`](asm-goto.static-key-on.c17.7b3f5e16.c) | shape=static-key-on | c17 | `3854620103` |
| [`asm-goto.two-labels.c17.8f7cd34d`](asm-goto.two-labels.c17.8f7cd34d.c) | shape=two-labels | c17 | `5383` |

