# gas-macros

assembler macros, .rept, .irp and .if used from inline asm. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`gas-macros.conditional.c17.974a52c7`](gas-macros.conditional.c17.974a52c7.c) | shape=conditional | c17 | `3767271015` |
| [`gas-macros.default-arg.c17.ba349e82`](gas-macros.default-arg.c17.ba349e82.c) | shape=default-arg | c17 | `2729695975` |
| [`gas-macros.irp.c17.21aebcee`](gas-macros.irp.c17.21aebcee.c) | shape=irp | c17 | `3743793127` |
| [`gas-macros.macro-args.c17.97ca40b2`](gas-macros.macro-args.c17.97ca40b2.c) | shape=macro-args | c17 | `1524020594` |
| [`gas-macros.rept.c17.5cdf162c`](gas-macros.rept.c17.5cdf162c.c) | shape=rept | c17 | `6343` |
| [`gas-macros.set.c17.5b04ce68`](gas-macros.set.c17.5b04ce68.c) | shape=set | c17 | `4026400710` |

