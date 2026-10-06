# objtool-shapes

the code shapes objtool follows through a kernel object. Part of the correctness phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`objtool-shapes.jump-table.c17.9da06433`](objtool-shapes.jump-table.c17.9da06433.c) | `-O2` | shape=jump-table | c17 | `2432668743` |
| [`objtool-shapes.noreturn.c17.430b0997`](objtool-shapes.noreturn.c17.430b0997.c) | `-O2` | shape=noreturn | c17 | `1311591431` |
| [`objtool-shapes.sibling-call.c17.df0c9a19`](objtool-shapes.sibling-call.c17.df0c9a19.c) | `-O2` | shape=sibling-call | c17 | `2702887196` |
| [`objtool-shapes.stack-realign.c17.ebaabdef`](objtool-shapes.stack-realign.c17.ebaabdef.c) | `-O2` | shape=stack-realign | c17 | `1579880263` |
| [`objtool-shapes.unreachable-end.c17.d8425c49`](objtool-shapes.unreachable-end.c17.d8425c49.c) | `-O2` | shape=unreachable-end | c17 | `1188` |

