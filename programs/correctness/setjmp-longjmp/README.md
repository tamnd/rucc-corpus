# setjmp-longjmp

the jump that leaves a function without returning from it. Part of the correctness phase of the M4 plan.

8 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`setjmp-longjmp.a-retry-loop.c17.916756fb`](setjmp-longjmp.a-retry-loop.c17.916756fb.c) | shape=a-retry-loop | c17 | `4 3` |
| [`setjmp-longjmp.out-of-nested-frames.c17.de169568`](setjmp-longjmp.out-of-nested-frames.c17.de169568.c) | shape=out-of-nested-frames | c17 | `123 5` |
| [`setjmp-longjmp.past-an-inner-handler.c17.dcc0d2b8`](setjmp-longjmp.past-an-inner-handler.c17.dcc0d2b8.c) | shape=past-an-inner-handler | c17 | `1 2` |
| [`setjmp-longjmp.stores-are-not-sunk.c17.7dfbf268`](setjmp-longjmp.stores-are-not-sunk.c17.7dfbf268.c) | shape=stores-are-not-sunk | c17 | `11 11` |
| [`setjmp-longjmp.the-value-comes-back.c17.1e1b77b1`](setjmp-longjmp.the-value-comes-back.c17.1e1b77b1.c) | shape=the-value-comes-back | c17 | `1 70` |
| [`setjmp-longjmp.unchanged-locals-survive.c17.10d360e8`](setjmp-longjmp.unchanged-locals-survive.c17.10d360e8.c) | shape=unchanged-locals-survive | c17 | `42 40` |
| [`setjmp-longjmp.volatile-survives.c17.59577dbb`](setjmp-longjmp.volatile-survives.c17.59577dbb.c) | shape=volatile-survives | c17 | `15` |
| [`setjmp-longjmp.zero-becomes-one.c17.6542db8f`](setjmp-longjmp.zero-becomes-one.c17.6542db8f.c) | shape=zero-becomes-one | c17 | `1` |

