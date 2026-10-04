# builtin-setjmp

locals live across a __builtin_setjmp the way MinGW builds of Postgres use it. Part of the correctness phase of the M4 plan.

48 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`builtin-setjmp.catch.1.busy.c17.e2ab7405`](builtin-setjmp.catch.1.busy.c17.e2ab7405.c) | shape=catch, live=1, first-arm=busy | c17 | `6 1 1 ...` and 2 more lines |
| [`builtin-setjmp.catch.1.quiet.c17.9d0b96e4`](builtin-setjmp.catch.1.quiet.c17.9d0b96e4.c) | shape=catch, live=1, first-arm=quiet | c17 | `6 1 1 1` |
| [`builtin-setjmp.catch.12.busy.c17.fc0d9866`](builtin-setjmp.catch.12.busy.c17.fc0d9866.c) | shape=catch, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`builtin-setjmp.catch.12.quiet.c17.a3796f40`](builtin-setjmp.catch.12.quiet.c17.a3796f40.c) | shape=catch, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`builtin-setjmp.catch.30.busy.c17.f5b51d2c`](builtin-setjmp.catch.30.busy.c17.f5b51d2c.c) | shape=catch, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`builtin-setjmp.catch.30.quiet.c17.9633b293`](builtin-setjmp.catch.30.quiet.c17.9633b293.c) | shape=catch, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`builtin-setjmp.catch.4.busy.c17.15817a96`](builtin-setjmp.catch.4.busy.c17.15817a96.c) | shape=catch, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`builtin-setjmp.catch.4.quiet.c17.959254e6`](builtin-setjmp.catch.4.quiet.c17.959254e6.c) | shape=catch, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`builtin-setjmp.deep.1.busy.c17.b052eb10`](builtin-setjmp.deep.1.busy.c17.b052eb10.c) | shape=deep, live=1, first-arm=busy | c17 | `6 1 1 ...` and 2 more lines |
| [`builtin-setjmp.deep.1.quiet.c17.99d8836a`](builtin-setjmp.deep.1.quiet.c17.99d8836a.c) | shape=deep, live=1, first-arm=quiet | c17 | `6 1 1 1` |
| [`builtin-setjmp.deep.12.busy.c17.4881dc71`](builtin-setjmp.deep.12.busy.c17.4881dc71.c) | shape=deep, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`builtin-setjmp.deep.12.quiet.c17.40a61a22`](builtin-setjmp.deep.12.quiet.c17.40a61a22.c) | shape=deep, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`builtin-setjmp.deep.30.busy.c17.0ca4c2db`](builtin-setjmp.deep.30.busy.c17.0ca4c2db.c) | shape=deep, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`builtin-setjmp.deep.30.quiet.c17.75d5f5bc`](builtin-setjmp.deep.30.quiet.c17.75d5f5bc.c) | shape=deep, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`builtin-setjmp.deep.4.busy.c17.80ed79cb`](builtin-setjmp.deep.4.busy.c17.80ed79cb.c) | shape=deep, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`builtin-setjmp.deep.4.quiet.c17.573808a3`](builtin-setjmp.deep.4.quiet.c17.573808a3.c) | shape=deep, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`builtin-setjmp.global.1.busy.c17.2104e799`](builtin-setjmp.global.1.busy.c17.2104e799.c) | shape=global, live=1, first-arm=busy | c17 | `6 1 1 ...` and 3 more lines |
| [`builtin-setjmp.global.1.quiet.c17.80ceb724`](builtin-setjmp.global.1.quiet.c17.80ceb724.c) | shape=global, live=1, first-arm=quiet | c17 | `6 1 1 ...` and 2 more lines |
| [`builtin-setjmp.global.12.busy.c17.509d6a8b`](builtin-setjmp.global.12.busy.c17.509d6a8b.c) | shape=global, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 14 more lines |
| [`builtin-setjmp.global.12.quiet.c17.4663539d`](builtin-setjmp.global.12.quiet.c17.4663539d.c) | shape=global, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 13 more lines |
| [`builtin-setjmp.global.30.busy.c17.010cca32`](builtin-setjmp.global.30.busy.c17.010cca32.c) | shape=global, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 32 more lines |
| [`builtin-setjmp.global.30.quiet.c17.d34c8679`](builtin-setjmp.global.30.quiet.c17.d34c8679.c) | shape=global, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 31 more lines |
| [`builtin-setjmp.global.4.busy.c17.b32b5697`](builtin-setjmp.global.4.busy.c17.b32b5697.c) | shape=global, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 6 more lines |
| [`builtin-setjmp.global.4.quiet.c17.b5bfe219`](builtin-setjmp.global.4.quiet.c17.b5bfe219.c) | shape=global, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 5 more lines |
| [`builtin-setjmp.loop.1.busy.c17.5666b8d2`](builtin-setjmp.loop.1.busy.c17.5666b8d2.c) | shape=loop, live=1, first-arm=busy | c17 | `12 2 4 ...` and 2 more lines |
| [`builtin-setjmp.loop.1.quiet.c17.b44e69b8`](builtin-setjmp.loop.1.quiet.c17.b44e69b8.c) | shape=loop, live=1, first-arm=quiet | c17 | `12 2 4 4` |
| [`builtin-setjmp.loop.12.busy.c17.1b01b183`](builtin-setjmp.loop.12.busy.c17.1b01b183.c) | shape=loop, live=12, first-arm=busy | c17 | `12 6004 66 ...` and 13 more lines |
| [`builtin-setjmp.loop.12.quiet.c17.6ee47e15`](builtin-setjmp.loop.12.quiet.c17.6ee47e15.c) | shape=loop, live=12, first-arm=quiet | c17 | `12 6004 66 ...` and 12 more lines |
| [`builtin-setjmp.loop.30.busy.c17.e93429aa`](builtin-setjmp.loop.30.busy.c17.e93429aa.c) | shape=loop, live=30, first-arm=busy | c17 | `12 6004 66 ...` and 31 more lines |
| [`builtin-setjmp.loop.30.quiet.c17.a778387c`](builtin-setjmp.loop.30.quiet.c17.a778387c.c) | shape=loop, live=30, first-arm=quiet | c17 | `12 6004 66 ...` and 30 more lines |
| [`builtin-setjmp.loop.4.busy.c17.e4c9b0b5`](builtin-setjmp.loop.4.busy.c17.e4c9b0b5.c) | shape=loop, live=4, first-arm=busy | c17 | `12 6004 66 ...` and 5 more lines |
| [`builtin-setjmp.loop.4.quiet.c17.8a499386`](builtin-setjmp.loop.4.quiet.c17.8a499386.c) | shape=loop, live=4, first-arm=quiet | c17 | `12 6004 66 ...` and 4 more lines |
| [`builtin-setjmp.no-error.1.busy.c17.7578707d`](builtin-setjmp.no-error.1.busy.c17.7578707d.c) | shape=no-error, live=1, first-arm=busy | c17 | `6 0 2 ...` and 2 more lines |
| [`builtin-setjmp.no-error.1.quiet.c17.9aa54d14`](builtin-setjmp.no-error.1.quiet.c17.9aa54d14.c) | shape=no-error, live=1, first-arm=quiet | c17 | `6 0 2 1` |
| [`builtin-setjmp.no-error.12.busy.c17.b9674e28`](builtin-setjmp.no-error.12.busy.c17.b9674e28.c) | shape=no-error, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`builtin-setjmp.no-error.12.quiet.c17.bfd4a239`](builtin-setjmp.no-error.12.quiet.c17.bfd4a239.c) | shape=no-error, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`builtin-setjmp.no-error.30.busy.c17.c02df153`](builtin-setjmp.no-error.30.busy.c17.c02df153.c) | shape=no-error, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`builtin-setjmp.no-error.30.quiet.c17.1c813a74`](builtin-setjmp.no-error.30.quiet.c17.1c813a74.c) | shape=no-error, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`builtin-setjmp.no-error.4.busy.c17.80cd5e90`](builtin-setjmp.no-error.4.busy.c17.80cd5e90.c) | shape=no-error, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`builtin-setjmp.no-error.4.quiet.c17.81450f81`](builtin-setjmp.no-error.4.quiet.c17.81450f81.c) | shape=no-error, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`builtin-setjmp.rethrow.1.busy.c17.3a411165`](builtin-setjmp.rethrow.1.busy.c17.3a411165.c) | shape=rethrow, live=1, first-arm=busy | c17 | `6 1 2 ...` and 3 more lines |
| [`builtin-setjmp.rethrow.1.quiet.c17.f34bce15`](builtin-setjmp.rethrow.1.quiet.c17.f34bce15.c) | shape=rethrow, live=1, first-arm=quiet | c17 | `6 1 2 ...` and 2 more lines |
| [`builtin-setjmp.rethrow.12.busy.c17.3b58b7c2`](builtin-setjmp.rethrow.12.busy.c17.3b58b7c2.c) | shape=rethrow, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 14 more lines |
| [`builtin-setjmp.rethrow.12.quiet.c17.16d6bcd6`](builtin-setjmp.rethrow.12.quiet.c17.16d6bcd6.c) | shape=rethrow, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 13 more lines |
| [`builtin-setjmp.rethrow.30.busy.c17.937f1fa6`](builtin-setjmp.rethrow.30.busy.c17.937f1fa6.c) | shape=rethrow, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 32 more lines |
| [`builtin-setjmp.rethrow.30.quiet.c17.a1c6407c`](builtin-setjmp.rethrow.30.quiet.c17.a1c6407c.c) | shape=rethrow, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 31 more lines |
| [`builtin-setjmp.rethrow.4.busy.c17.2683b95d`](builtin-setjmp.rethrow.4.busy.c17.2683b95d.c) | shape=rethrow, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 6 more lines |
| [`builtin-setjmp.rethrow.4.quiet.c17.6f4c9f59`](builtin-setjmp.rethrow.4.quiet.c17.6f4c9f59.c) | shape=rethrow, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 5 more lines |

