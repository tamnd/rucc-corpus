# mcmodel-kernel

programs built for the kernel code model and linked without PIE. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`mcmodel-kernel.address-table.c17.ee3f2a1f`](mcmodel-kernel.address-table.c17.ee3f2a1f.c) | `-mcmodel=kernel` `-mno-red-zone` `-fno-pie` `-no-pie` | shape=address-table | c17 | `266691827` |
| [`mcmodel-kernel.function-table.c17.3cfb4a24`](mcmodel-kernel.function-table.c17.3cfb4a24.c) | `-mcmodel=kernel` `-mno-red-zone` `-fno-pie` `-no-pie` | shape=function-table | c17 | `3407135943` |
| [`mcmodel-kernel.global-index.c17.67b3d739`](mcmodel-kernel.global-index.c17.67b3d739.c) | `-mcmodel=kernel` `-mno-red-zone` `-fno-pie` `-no-pie` | shape=global-index | c17 | `1607223923` |
| [`mcmodel-kernel.large-array.c17.bd42b4cd`](mcmodel-kernel.large-array.c17.bd42b4cd.c) | `-mcmodel=kernel` `-mno-red-zone` `-fno-pie` `-no-pie` | shape=large-array | c17 | `3082289831` |
| [`mcmodel-kernel.strings.c17.3e1c70f9`](mcmodel-kernel.strings.c17.3e1c70f9.c) | `-mcmodel=kernel` `-mno-red-zone` `-fno-pie` `-no-pie` | shape=strings | c17 | `1780605601` |
| [`mcmodel-kernel.switch.c17.1218421f`](mcmodel-kernel.switch.c17.1218421f.c) | `-mcmodel=kernel` `-mno-red-zone` `-fno-pie` `-no-pie` | shape=switch | c17 | `3963163919` |

