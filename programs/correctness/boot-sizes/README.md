# boot-sizes

boot block shapes under the boot2 flag line, within a size budget. Part of the correctness phase of the M4 plan.

4 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`boot-sizes.fsread.c17.7e69ebd1`](boot-sizes.fsread.c17.7e69ebd1.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=fsread | c17 | `595363102 1267 33 1` |
| [`boot-sizes.lookup.c17.5b6bc83b`](boot-sizes.lookup.c17.5b6bc83b.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=lookup | c17 | `1057685349 1` |
| [`boot-sizes.parse.c17.2eb472d8`](boot-sizes.parse.c17.2eb472d8.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=parse | c17 | `2072072992 1` |
| [`boot-sizes.printf.c17.71af6914`](boot-sizes.printf.c17.71af6914.c) | `-m32` `-march=i386` `-ffreestanding` `-mrtd` `-mregparm=3` `-fomit-frame-pointer` `-mno-mmx` `-mno-sse` `-fno-pie` `-no-pie` `-nostdlib` `-static` `-Os` | shape=printf | c17 | `4253945320 200 1` |

