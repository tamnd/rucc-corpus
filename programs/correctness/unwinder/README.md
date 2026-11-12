# unwinder

_Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel. Part of the correctness phase of the M4 plan.

7 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`unwinder.alloca.c17.55d0563c`](unwinder.alloca.c17.55d0563c.c) | nothing extra | shape=alloca | c17 | `1 218` |
| [`unwinder.callee-saved.c17.07383ad2`](unwinder.callee-saved.c17.07383ad2.c) | nothing extra | shape=callee-saved | c17 | `1 2318` |
| [`unwinder.cleanup-cancel.c17.78497b80`](unwinder.cleanup-cancel.c17.78497b80.c) | `-fexceptions` `-pthread` | shape=cleanup-cancel | c17 | `1 321` |
| [`unwinder.cleanup-exit.c17.dee38085`](unwinder.cleanup-exit.c17.dee38085.c) | `-fexceptions` `-pthread` | shape=cleanup-exit | c17 | `46 321` |
| [`unwinder.plain.c17.c17dd73a`](unwinder.plain.c17.c17dd73a.c) | nothing extra | shape=plain | c17 | `1 218` |
| [`unwinder.realign.c17.be399eb8`](unwinder.realign.c17.be399eb8.c) | nothing extra | shape=realign | c17 | `1 218` |
| [`unwinder.signal.c17.9bec8b4e`](unwinder.signal.c17.9bec8b4e.c) | nothing extra | shape=signal | c17 | `1 188 1` |

