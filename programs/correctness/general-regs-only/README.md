# general-regs-only

programs built with -mgeneral-regs-only that check no vector register moved. Part of the correctness phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`general-regs-only.array-sum.c17.b6fb6e86`](general-regs-only.array-sum.c17.b6fb6e86.c) | `-mgeneral-regs-only` | shape=array-sum | c17 | `470974658` |
| [`general-regs-only.byte-mix.c17.4a4263ee`](general-regs-only.byte-mix.c17.4a4263ee.c) | `-mgeneral-regs-only` | shape=byte-mix | c17 | `9900455` |
| [`general-regs-only.struct-copy.c17.496266d3`](general-regs-only.struct-copy.c17.496266d3.c) | `-mgeneral-regs-only` | shape=struct-copy | c17 | `2188897103` |
| [`general-regs-only.u64-math.c17.754a8f5a`](general-regs-only.u64-math.c17.754a8f5a.c) | `-mgeneral-regs-only` | shape=u64-math | c17 | `188972664` |
| [`general-regs-only.zeroed.c17.e639f9d1`](general-regs-only.zeroed.c17.e639f9d1.c) | `-mgeneral-regs-only` | shape=zeroed | c17 | `3575246495` |

