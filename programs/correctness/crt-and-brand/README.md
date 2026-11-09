# crt-and-brand

the BSD start files: the brand notes, init order and a bare _start. Part of the correctness phase of the M4 plan.

7 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`crt-and-brand.init-fragment.c17.2c66596e`](crt-and-brand.init-fragment.c17.2c66596e.c) | nothing, it is one file | nothing extra | shape=init-fragment | c17 | `12` |
| [`crt-and-brand.init-priority-units.c17.17c47dac`](crt-and-brand.init-priority-units.c17.17c47dac.c) | [`early`](crt-and-brand.init-priority-units.c17.17c47dac.early.c) | nothing extra | shape=init-priority-units | c17 | `1234` |
| [`crt-and-brand.init-priority.c17.d14e9203`](crt-and-brand.init-priority.c17.d14e9203.c) | nothing, it is one file | nothing extra | shape=init-priority | c17 | `1234` |
| [`crt-and-brand.note-pax.c17.c1e1fc5d`](crt-and-brand.note-pax.c17.c1e1fc5d.c) | nothing, it is one file | nothing extra | shape=note-pax | c17 | `2 28930` |
| [`crt-and-brand.note-tag.c17.906d41b8`](crt-and-brand.note-tag.c17.906d41b8.c) | nothing, it is one file | nothing extra | shape=note-tag | c17 | `1501000 28930` |
| [`crt-and-brand.preinit-array.c17.bb828f49`](crt-and-brand.preinit-array.c17.bb828f49.c) | nothing, it is one file | nothing extra | shape=preinit-array | c17 | `1 1` |
| [`crt-and-brand.start-nostdlib.c17.3574a3f5`](crt-and-brand.start-nostdlib.c17.3574a3f5.c) | nothing, it is one file | `-nostdlib` `-static` | shape=start-nostdlib | c17 | `42` |

