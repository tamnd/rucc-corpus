# long-double

the widest floating type, whose shape the target rather than C decides. Part of the backend phase of the M4 plan.

10 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`long-double.as-an-argument.c17.754d598a`](long-double.as-an-argument.c17.754d598a.c) | shape=as-an-argument | c17 | `204 42 8` |
| [`long-double.at-least-as-wide-as-double.c17.9a54be00`](long-double.at-least-as-wide-as-double.c17.9a54be00.c) | shape=at-least-as-wide-as-double | c17 | `1 1 1 ...` and 2 more lines |
| [`long-double.compared-against-integers.c17.290d26b6`](long-double.compared-against-integers.c17.290d26b6.c) | shape=compared-against-integers | c17 | `1 1 1 ...` and 3 more lines |
| [`long-double.converted-both-ways.c17.8ae376ce`](long-double.converted-both-ways.c17.8ae376ce.c) | shape=converted-both-ways | c17 | `1099511627776 4000000000 1073741824 ...` and 3 more lines |
| [`long-double.every-double-is-one.c17.d7ba5e4e`](long-double.every-double-is-one.c17.d7ba5e4e.c) | shape=every-double-is-one | c17 | `1` |
| [`long-double.exact-on-small-integers.c17.eb8e0b1d`](long-double.exact-on-small-integers.c17.eb8e0b1d.c) | shape=exact-on-small-integers | c17 | `130456 116456 864192000 123456` |
| [`long-double.in-a-struct.c17.b968b294`](long-double.in-a-struct.c17.b968b294.c) | shape=in-a-struct | c17 | `84 12 96` |
| [`long-double.mixed-with-integers.c17.460ceae2`](long-double.mixed-with-integers.c17.460ceae2.c) | shape=mixed-with-integers | c17 | `42 22` |
| [`long-double.through-varargs.c17.2f2ae387`](long-double.through-varargs.c17.2f2ae387.c) | shape=through-varargs | c17 | `31 4320` |
| [`long-double.written-as-a-literal.c17.eaccf38c`](long-double.written-as-a-literal.c17.eaccf38c.c) | shape=written-as-a-literal | c17 | `1500 1024 3 ...` and 3 more lines |

