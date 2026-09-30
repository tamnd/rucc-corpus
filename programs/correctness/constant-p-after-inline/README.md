# constant-p-after-inline

__builtin_constant_p answered after inlining, with the other branch unlinkable. Part of the correctness phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`constant-p-after-inline.argument.c17.3809006f`](constant-p-after-inline.argument.c17.3809006f.c) | `-O2` | shape=argument | c17 | `55399` |
| [`constant-p-after-inline.expression.c17.a637af06`](constant-p-after-inline.expression.c17.a637af06.c) | `-O2` | shape=expression | c17 | `2010161447` |
| [`constant-p-after-inline.local.c17.29ba077c`](constant-p-after-inline.local.c17.29ba077c.c) | `-O2` | shape=local | c17 | `1063` |
| [`constant-p-after-inline.runtime-argument.c17.6772093a`](constant-p-after-inline.runtime-argument.c17.6772093a.c) | `-O2` | shape=runtime-argument | c17 | `2861933977` |
| [`constant-p-after-inline.two-levels.c17.03722f1c`](constant-p-after-inline.two-levels.c17.03722f1c.c) | `-O2` | shape=two-levels | c17 | `3111` |

