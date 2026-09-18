# barrier

programs where the compiler must not act, and a firing is a bug. Part of the correctness phase of the M4 plan.

13 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`barrier.char-aliasing.c17.c3cc419e`](barrier.char-aliasing.c17.c3cc419e.c) | shape=char-aliasing | c17 | `1 1` |
| [`barrier.pointer-escapes.c17.15f81214`](barrier.pointer-escapes.c17.15f81214.c) | shape=pointer-escapes | c17 | `8` |
| [`barrier.union-punning.c17.03ac9ae3`](barrier.union-punning.c17.03ac9ae3.c) | shape=union-punning | c17 | `255 1` |
| [`barrier.volatile-array.c17.666fb1b5`](barrier.volatile-array.c17.666fb1b5.c) | shape=volatile-array | c17 | `4` |
| [`barrier.volatile-compare.c17.6808636c`](barrier.volatile-compare.c17.6808636c.c) | shape=volatile-compare | c17 | `101` |
| [`barrier.volatile-field.c17.5ea86f98`](barrier.volatile-field.c17.5ea86f98.c) | shape=volatile-field | c17 | `44` |
| [`barrier.volatile-in-loop.c17.5e0a5e33`](barrier.volatile-in-loop.c17.5e0a5e33.c) | shape=volatile-in-loop | c17 | `100` |
| [`barrier.volatile-order.c17.b53dec9a`](barrier.volatile-order.c17.b53dec9a.c) | shape=volatile-order | c17 | `321` |
| [`barrier.volatile-pointer-itself.c17.96737246`](barrier.volatile-pointer-itself.c17.96737246.c) | shape=volatile-pointer-itself | c17 | `42` |
| [`barrier.volatile-read-and-add.c17.03b3be1e`](barrier.volatile-read-and-add.c17.03b3be1e.c) | shape=volatile-read-and-add | c17 | `42 80` |
| [`barrier.volatile-scalar.c17.06479072`](barrier.volatile-scalar.c17.06479072.c) | shape=volatile-scalar | c17 | `10 10` |
| [`barrier.volatile-update.c17.c8cfbaf4`](barrier.volatile-update.c17.c8cfbaf4.c) | shape=volatile-update | c17 | `17` |
| [`barrier.volatile-widths.c17.02cee234`](barrier.volatile-widths.c17.02cee234.c) | shape=volatile-widths | c17 | `4 301 70001 5000000001` |

