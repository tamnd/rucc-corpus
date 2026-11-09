# linker-sets

the BSD linker sets: pointers in a named section, walked start to stop. Part of the correctness phase of the M4 plan.

9 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`linker-sets.data-set-empty.c17.25d3cf17`](linker-sets.data-set-empty.c17.25d3cf17.c) | nothing, it is one file | nothing extra | shape=data-set-empty | c17 | `0 0` |
| [`linker-sets.data-set-gc.c17.7998a4a1`](linker-sets.data-set-gc.c17.7998a4a1.c) | [`more`](linker-sets.data-set-gc.c17.7998a4a1.more.c) | `-ffunction-sections` `-fdata-sections` `-Wl,--gc-sections` | shape=data-set-gc | c17 | `5 76543` |
| [`linker-sets.data-set-structs.c17.60dcd205`](linker-sets.data-set-structs.c17.60dcd205.c) | nothing, it is one file | nothing extra | shape=data-set-structs | c17 | `2 16` |
| [`linker-sets.data-set-units.c17.f9cbc4f3`](linker-sets.data-set-units.c17.f9cbc4f3.c) | [`more`](linker-sets.data-set-units.c17.f9cbc4f3.more.c) | nothing extra | shape=data-set-units | c17 | `5 76543` |
| [`linker-sets.data-set.c17.80be220e`](linker-sets.data-set.c17.80be220e.c) | nothing, it is one file | nothing extra | shape=data-set | c17 | `3 543` |
| [`linker-sets.link-set-gc.c17.4db31cb4`](linker-sets.link-set-gc.c17.4db31cb4.c) | [`more`](linker-sets.link-set-gc.c17.4db31cb4.more.c) | `-ffunction-sections` `-fdata-sections` `-Wl,--gc-sections` | shape=link-set-gc | c17 | `4 4333` |
| [`linker-sets.link-set-text.c17.c1a6378b`](linker-sets.link-set-text.c17.c1a6378b.c) | nothing, it is one file | nothing extra | shape=link-set-text | c17 | `21 21` |
| [`linker-sets.link-set-units.c17.e852276b`](linker-sets.link-set-units.c17.e852276b.c) | [`more`](linker-sets.link-set-units.c17.e852276b.more.c) | nothing extra | shape=link-set-units | c17 | `4 4333` |
| [`linker-sets.link-set.c17.3ca4315b`](linker-sets.link-set.c17.3ca4315b.c) | nothing, it is one file | nothing extra | shape=link-set | c17 | `3 333` |

