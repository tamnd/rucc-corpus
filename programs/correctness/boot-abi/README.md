# boot-abi

the i386 boot code convention: -mrtd and -mregparm=3 with assembly. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`boot-abi.asm-callee-pops.c17.2c1bb800`](boot-abi.asm-callee-pops.c17.2c1bb800.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=asm-callee-pops | c17 | `68226 0 0` |
| [`boot-abi.asm-callee-registers.c17.d1ba49bc`](boot-abi.asm-callee-registers.c17.d1ba49bc.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=asm-callee-registers | c17 | `2136543891 0 0` |
| [`boot-abi.asm-caller.c17.e8cbb0e2`](boot-abi.asm-caller.c17.e8cbb0e2.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=asm-caller | c17 | `531264132 0 0` |
| [`boot-abi.cdecl-override.c17.09fafae6`](boot-abi.cdecl-override.c17.09fafae6.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=cdecl-override | c17 | `440401 0 0` |
| [`boot-abi.varargs.c17.2bb6149b`](boot-abi.varargs.c17.2bb6149b.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=varargs | c17 | `2165108654 0 0` |
| [`boot-abi.wide-argument.c17.73591d8b`](boot-abi.wide-argument.c17.73591d8b.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=wide-argument | c17 | `4278171134 0 0` |

