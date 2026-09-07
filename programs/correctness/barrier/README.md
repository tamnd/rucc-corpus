# barrier

programs where the compiler must not act, and a firing is a bug. Part of the correctness phase of the M4 plan.

7 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`barrier.char-aliasing.c17.c3cc419e`](barrier.char-aliasing.c17.c3cc419e.c) | shape=char-aliasing | c17 | `1 1` |
| [`barrier.pointer-escapes.c17.15f81214`](barrier.pointer-escapes.c17.15f81214.c) | shape=pointer-escapes | c17 | `8` |
| [`barrier.union-punning.c17.03ac9ae3`](barrier.union-punning.c17.03ac9ae3.c) | shape=union-punning | c17 | `255 1` |
| [`barrier.volatile-array.c17.666fb1b5`](barrier.volatile-array.c17.666fb1b5.c) | shape=volatile-array | c17 | `4` |
| [`barrier.volatile-in-loop.c17.5e0a5e33`](barrier.volatile-in-loop.c17.5e0a5e33.c) | shape=volatile-in-loop | c17 | `100` |
| [`barrier.volatile-order.c17.b53dec9a`](barrier.volatile-order.c17.b53dec9a.c) | shape=volatile-order | c17 | `321` |
| [`barrier.volatile-scalar.c17.06479072`](barrier.volatile-scalar.c17.06479072.c) | shape=volatile-scalar | c17 | `10 10` |

