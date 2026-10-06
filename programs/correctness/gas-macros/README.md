# gas-macros

assembler macros, .rept, .irp and .if used from inline asm. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`gas-macros.conditional.c17.387f82c2`](gas-macros.conditional.c17.387f82c2.c) | shape=conditional | c17 | `3767271015` |
| [`gas-macros.default-arg.c17.2b5e7cc2`](gas-macros.default-arg.c17.2b5e7cc2.c) | shape=default-arg | c17 | `2729695975` |
| [`gas-macros.irp.c17.ce0f445b`](gas-macros.irp.c17.ce0f445b.c) | shape=irp | c17 | `3743793127` |
| [`gas-macros.macro-args.c17.d6703822`](gas-macros.macro-args.c17.d6703822.c) | shape=macro-args | c17 | `1524020594` |
| [`gas-macros.rept.c17.08b98c8f`](gas-macros.rept.c17.08b98c8f.c) | shape=rept | c17 | `6343` |
| [`gas-macros.set.c17.f7ccc461`](gas-macros.set.c17.f7ccc461.c) | shape=set | c17 | `4026400710` |

