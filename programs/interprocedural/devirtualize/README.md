# devirtualize

turning an indirect call into a direct one. Part of the interprocedural phase of the M4 plan.

4 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`devirtualize.constant-table.c17.ef6af85b`](devirtualize.constant-table.c17.ef6af85b.c) | shape=constant-table | c17 | `21` |
| [`devirtualize.direct-assign.c17.d0042aa1`](devirtualize.direct-assign.c17.d0042aa1.c) | shape=direct-assign | c17 | `14` |
| [`devirtualize.two-targets.c17.1f473aa2`](devirtualize.two-targets.c17.1f473aa2.c) | shape=two-targets | c17 | `21` |
| [`devirtualize.unknown.c17.1b073f50`](devirtualize.unknown.c17.1b073f50.c) | shape=unknown | c17 | `21` |

