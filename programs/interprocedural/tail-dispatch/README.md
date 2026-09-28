# tail-dispatch

calls in tail position made often enough to time whether they became jumps. Part of the interprocedural phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`tail-dispatch.alternate.2.c17.ede7610d`](tail-dispatch.alternate.2.c17.ede7610d.c) | shape=alternate, states=2 | c17 | `3510186909672803840` |
| [`tail-dispatch.rotate.1.c17.b2d99408`](tail-dispatch.rotate.1.c17.b2d99408.c) | shape=rotate, states=1 | c17 | `1077868032` |
| [`tail-dispatch.switch.16.c17.e756d443`](tail-dispatch.switch.16.c17.e756d443.c) | shape=switch, states=16 | c17 | `13391797043948228096` |
| [`tail-dispatch.switch.4.c17.f2c81c43`](tail-dispatch.switch.4.c17.f2c81c43.c) | shape=switch, states=4 | c17 | `8311899052146404864` |
| [`tail-dispatch.table.4.c17.eb9bdb34`](tail-dispatch.table.4.c17.eb9bdb34.c) | shape=table, states=4 | c17 | `14034240900470701568` |

