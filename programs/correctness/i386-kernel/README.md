# i386-kernel

programs built as an i386 kernel is, with regparm and no libgcc helper. Part of the correctness phase of the M4 plan.

9 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`i386-kernel.bit-scan-64.c17.350374d3`](i386-kernel.bit-scan-64.c17.350374d3.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=bit-scan-64 | c17 | `4128831` |
| [`i386-kernel.compare-64.c17.c18d1ed5`](i386-kernel.compare-64.c17.c18d1ed5.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=compare-64 | c17 | `2413717921` |
| [`i386-kernel.divide-narrow.c17.d883bac8`](i386-kernel.divide-narrow.c17.d883bac8.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=divide-narrow | c17 | `9434885` |
| [`i386-kernel.divide-power-of-two.c17.b214b5e7`](i386-kernel.divide-power-of-two.c17.b214b5e7.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=divide-power-of-two | c17 | `822601647` |
| [`i386-kernel.multiply-64.c17.973d73f9`](i386-kernel.multiply-64.c17.973d73f9.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=multiply-64 | c17 | `591044811` |
| [`i386-kernel.regparm-calls.c17.b683f0b0`](i386-kernel.regparm-calls.c17.b683f0b0.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=regparm-calls | c17 | `4177888425` |
| [`i386-kernel.regparm-pointer.c17.feff67d8`](i386-kernel.regparm-pointer.c17.feff67d8.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=regparm-pointer | c17 | `1778779173` |
| [`i386-kernel.shift-64.c17.5d3de3b9`](i386-kernel.shift-64.c17.5d3de3b9.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=shift-64 | c17 | `4218729230` |
| [`i386-kernel.struct-return.c17.b59db972`](i386-kernel.struct-return.c17.b59db972.c) | `-m32` `-march=i686` `-mregparm=3` `-freg-struct-return` `-fno-pie` `-no-pie` `-O2` | shape=struct-return | c17 | `3506494268` |

