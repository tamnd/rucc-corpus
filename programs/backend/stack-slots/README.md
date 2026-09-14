# stack-slots

two things in the frame that may be the same bytes. Part of the backend phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`stack-slots.address-escapes.c17.5d09e83c`](stack-slots.address-escapes.c17.5d09e83c.c) | shape=address-escapes | c17 | `216` |
| [`stack-slots.both-at-once.c17.b273e448`](stack-slots.both-at-once.c17.b273e448.c) | shape=both-at-once | c17 | `284` |
| [`stack-slots.scope-in-a-loop.c17.e1d7b2c2`](stack-slots.scope-in-a-loop.c17.e1d7b2c2.c) | shape=scope-in-a-loop | c17 | `468` |
| [`stack-slots.spilled-around-a-call.c17.222d05d7`](stack-slots.spilled-around-a-call.c17.222d05d7.c) | shape=spilled-around-a-call | c17 | `200` |
| [`stack-slots.two-scopes.c17.5a26f0da`](stack-slots.two-scopes.c17.5a26f0da.c) | shape=two-scopes | c17 | `216` |
| [`stack-slots.wide-alignment.c17.f32cea04`](stack-slots.wide-alignment.c17.f32cea04.c) | shape=wide-alignment | c17 | `216` |

