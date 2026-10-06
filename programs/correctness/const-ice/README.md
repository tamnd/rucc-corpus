# const-ice

sizeof, offsetof and const objects in integer constant expressions. Part of the correctness phase of the M4 plan.

8 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`const-ice.array-size.c17.a7f9ec33`](const-ice.array-size.c17.a7f9ec33.c) | nothing extra | shape=array-size | c17 | `2506149349` |
| [`const-ice.build-bug-on-zero.c17.93c378f1`](const-ice.build-bug-on-zero.c17.93c378f1.c) | nothing extra | shape=build-bug-on-zero | c17 | `4030112491` |
| [`const-ice.case-label.c17.9a8f6d41`](const-ice.case-label.c17.9a8f6d41.c) | nothing extra | shape=case-label | c17 | `2924` |
| [`const-ice.cast-conditional.c17.564af6db`](const-ice.cast-conditional.c17.564af6db.c) | nothing extra | shape=cast-conditional | c17 | `809268103` |
| [`const-ice.const-local.c17.d372496d`](const-ice.const-local.c17.d372496d.c) | `-O2` | shape=const-local | c17 | `16992` |
| [`const-ice.constant-p.c17.f4183ad8`](const-ice.constant-p.c17.f4183ad8.c) | nothing extra | shape=constant-p | c17 | `2288824994` |
| [`const-ice.enum.c17.8749f01c`](const-ice.enum.c17.8749f01c.c) | nothing extra | shape=enum | c17 | `1451` |
| [`const-ice.static-assert.c17.94f00f5f`](const-ice.static-assert.c17.94f00f5f.c) | nothing extra | shape=static-assert | c17 | `555299760` |

