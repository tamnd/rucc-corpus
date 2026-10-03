# bit-loops

loops that count bits, which some processors count in one instruction. Part of the loops phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`bit-loops.leading.u32.plain.c17.4c1df095`](bit-loops.leading.u32.plain.c17.4c1df095.c) | count=leading, type=u32, built=plain | c17 | `872619` |
| [`bit-loops.leading.u32.target.c17.9c1a2578`](bit-loops.leading.u32.target.c17.9c1a2578.c) | count=leading, type=u32, built=target | c17 | `872619` |
| [`bit-loops.leading.u64.plain.c17.f21658cd`](bit-loops.leading.u64.plain.c17.f21658cd.c) | count=leading, type=u64, built=plain | c17 | `3247623` |
| [`bit-loops.leading.u64.target.c17.a727a3ad`](bit-loops.leading.u64.target.c17.a727a3ad.c) | count=leading, type=u64, built=target | c17 | `3247623` |
| [`bit-loops.ones.u32.plain.c17.219f8a9a`](bit-loops.ones.u32.plain.c17.219f8a9a.c) | count=ones, type=u32, built=plain | c17 | `1214717` |
| [`bit-loops.ones.u32.target.c17.47943630`](bit-loops.ones.u32.target.c17.47943630.c) | count=ones, type=u32, built=target | c17 | `1214717` |
| [`bit-loops.ones.u64.plain.c17.f9e7bdcb`](bit-loops.ones.u64.plain.c17.f9e7bdcb.c) | count=ones, type=u64, built=plain | c17 | `1627007` |
| [`bit-loops.ones.u64.target.c17.4504a3d6`](bit-loops.ones.u64.target.c17.4504a3d6.c) | count=ones, type=u64, built=target | c17 | `1627007` |
| [`bit-loops.trailing.u32.plain.c17.4ea34673`](bit-loops.trailing.u32.plain.c17.4ea34673.c) | count=trailing, type=u32, built=plain | c17 | `144763` |
| [`bit-loops.trailing.u32.target.c17.662fbea6`](bit-loops.trailing.u32.target.c17.662fbea6.c) | count=trailing, type=u32, built=target | c17 | `144763` |
| [`bit-loops.trailing.u64.plain.c17.41b9759c`](bit-loops.trailing.u64.plain.c17.41b9759c.c) | count=trailing, type=u64, built=plain | c17 | `194299` |
| [`bit-loops.trailing.u64.target.c17.4654f37c`](bit-loops.trailing.u64.target.c17.4654f37c.c) | count=trailing, type=u64, built=target | c17 | `194299` |
| [`bit-loops.width.u32.plain.c17.40d8503b`](bit-loops.width.u32.plain.c17.40d8503b.c) | count=width, type=u32, built=plain | c17 | `2327381` |
| [`bit-loops.width.u32.target.c17.613a02db`](bit-loops.width.u32.target.c17.613a02db.c) | count=width, type=u32, built=target | c17 | `2327381` |
| [`bit-loops.width.u64.plain.c17.2aa94261`](bit-loops.width.u64.plain.c17.2aa94261.c) | count=width, type=u64, built=plain | c17 | `3152377` |
| [`bit-loops.width.u64.target.c17.e8fafc14`](bit-loops.width.u64.target.c17.e8fafc14.c) | count=width, type=u64, built=target | c17 | `3152377` |

