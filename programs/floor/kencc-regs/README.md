# kencc-regs

the extern register variables of kencc across calls and longjmp. Part of the floor phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`kencc-regs.across-calls.kencc.c17.1a8b17b9`](kencc-regs.across-calls.kencc.c17.1a8b17b9.c) | `-fkencc` | shape=across-calls, dialect=kencc | c17 | `10 1 2 ...` and 2 more lines |
| [`kencc-regs.longjmp.kencc.c17.d454d85e`](kencc-regs.longjmp.kencc.c17.d454d85e.c) | `-fkencc` | shape=longjmp, dialect=kencc | c17 | `3 77 80` |
| [`kencc-regs.many-live-values.kencc.c17.5a613b27`](kencc-regs.many-live-values.kencc.c17.5a613b27.c) | `-fkencc` | shape=many-live-values, dialect=kencc | c17 | `1283 1000 1566` |
| [`kencc-regs.recursion.kencc.c17.4e8e2174`](kencc-regs.recursion.kencc.c17.4e8e2174.c) | `-fkencc` | shape=recursion, dialect=kencc | c17 | `5 405` |
| [`kencc-regs.two-registers.kencc.c17.37d2b459`](kencc-regs.two-registers.kencc.c17.37d2b459.c) | `-fkencc` | shape=two-registers, dialect=kencc | c17 | `8 9 920 2` |
| [`kencc-regs.write-in-callee.kencc.c17.b451109a`](kencc-regs.write-in-callee.kencc.c17.b451109a.c) | `-fkencc` | shape=write-in-callee, dialect=kencc | c17 | `42 42 88` |

