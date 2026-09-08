# vla-and-alloca

an object whose size is not known until the program runs. Part of the floor phase of the M4 plan.

9 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`vla-and-alloca.alloca-in-a-loop.c17.c08d97f0`](vla-and-alloca.alloca-in-a-loop.c17.c08d97f0.c) | shape=alloca-in-a-loop | c17 | `10` |
| [`vla-and-alloca.alloca.c17.435d9869`](vla-and-alloca.alloca.c17.435d9869.c) | shape=alloca | c17 | `84 21` |
| [`vla-and-alloca.as-a-parameter.c17.798b835c`](vla-and-alloca.as-a-parameter.c17.798b835c.c) | shape=as-a-parameter | c17 | `138 23` |
| [`vla-and-alloca.both-in-one-frame.c17.b9295dc4`](vla-and-alloca.both-in-one-frame.c17.b9295dc4.c) | shape=both-in-one-frame | c17 | `15 20 35` |
| [`vla-and-alloca.jumping-out-of-the-scope.c17.a6581def`](vla-and-alloca.jumping-out-of-the-scope.c17.a6581def.c) | shape=jumping-out-of-the-scope | c17 | `4` |
| [`vla-and-alloca.one-dimension.c17.0ea83c35`](vla-and-alloca.one-dimension.c17.0ea83c35.c) | shape=one-dimension | c17 | `55 25` |
| [`vla-and-alloca.pointer-to-a-row.c17.cf78484a`](vla-and-alloca.pointer-to-a-row.c17.cf78484a.c) | shape=pointer-to-a-row | c17 | `33` |
| [`vla-and-alloca.size-changes-each-time.c17.c157711d`](vla-and-alloca.size-changes-each-time.c17.c157711d.c) | shape=size-changes-each-time | c17 | `20` |
| [`vla-and-alloca.two-dimensions.c17.acdfaea1`](vla-and-alloca.two-dimensions.c17.acdfaea1.c) | shape=two-dimensions | c17 | `138 23 0` |

