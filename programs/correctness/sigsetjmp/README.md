# sigsetjmp

locals live across a sigsetjmp the way PG_TRY and PG_CATCH use it. Part of the correctness phase of the M4 plan.

112 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`sigsetjmp.setjmp.catch.1.busy.c17.bbe38ae0`](sigsetjmp.setjmp.catch.1.busy.c17.bbe38ae0.c) | api=setjmp, shape=catch, live=1, first-arm=busy | c17 | `6 1 1 ...` and 2 more lines |
| [`sigsetjmp.setjmp.catch.1.quiet.c17.284140a5`](sigsetjmp.setjmp.catch.1.quiet.c17.284140a5.c) | api=setjmp, shape=catch, live=1, first-arm=quiet | c17 | `6 1 1 1` |
| [`sigsetjmp.setjmp.catch.12.busy.c17.0889f535`](sigsetjmp.setjmp.catch.12.busy.c17.0889f535.c) | api=setjmp, shape=catch, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`sigsetjmp.setjmp.catch.12.quiet.c17.232e2755`](sigsetjmp.setjmp.catch.12.quiet.c17.232e2755.c) | api=setjmp, shape=catch, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`sigsetjmp.setjmp.catch.2.busy.c17.a20c3e98`](sigsetjmp.setjmp.catch.2.busy.c17.a20c3e98.c) | api=setjmp, shape=catch, live=2, first-arm=busy | c17 | `6 3002 1 ...` and 3 more lines |
| [`sigsetjmp.setjmp.catch.2.quiet.c17.2ace0f5c`](sigsetjmp.setjmp.catch.2.quiet.c17.2ace0f5c.c) | api=setjmp, shape=catch, live=2, first-arm=quiet | c17 | `6 3002 1 ...` and 2 more lines |
| [`sigsetjmp.setjmp.catch.20.busy.c17.393a340b`](sigsetjmp.setjmp.catch.20.busy.c17.393a340b.c) | api=setjmp, shape=catch, live=20, first-arm=busy | c17 | `6 3002 21 ...` and 21 more lines |
| [`sigsetjmp.setjmp.catch.20.quiet.c17.70307c6d`](sigsetjmp.setjmp.catch.20.quiet.c17.70307c6d.c) | api=setjmp, shape=catch, live=20, first-arm=quiet | c17 | `6 3002 21 ...` and 20 more lines |
| [`sigsetjmp.setjmp.catch.30.busy.c17.c480031a`](sigsetjmp.setjmp.catch.30.busy.c17.c480031a.c) | api=setjmp, shape=catch, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`sigsetjmp.setjmp.catch.30.quiet.c17.b935186a`](sigsetjmp.setjmp.catch.30.quiet.c17.b935186a.c) | api=setjmp, shape=catch, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`sigsetjmp.setjmp.catch.4.busy.c17.a5790970`](sigsetjmp.setjmp.catch.4.busy.c17.a5790970.c) | api=setjmp, shape=catch, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`sigsetjmp.setjmp.catch.4.quiet.c17.353ea94f`](sigsetjmp.setjmp.catch.4.quiet.c17.353ea94f.c) | api=setjmp, shape=catch, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`sigsetjmp.setjmp.catch.7.busy.c17.443ead50`](sigsetjmp.setjmp.catch.7.busy.c17.443ead50.c) | api=setjmp, shape=catch, live=7, first-arm=busy | c17 | `6 3002 21 ...` and 8 more lines |
| [`sigsetjmp.setjmp.catch.7.quiet.c17.3ca53bfd`](sigsetjmp.setjmp.catch.7.quiet.c17.3ca53bfd.c) | api=setjmp, shape=catch, live=7, first-arm=quiet | c17 | `6 3002 21 ...` and 7 more lines |
| [`sigsetjmp.setjmp.loop.1.busy.c17.fa13f5db`](sigsetjmp.setjmp.loop.1.busy.c17.fa13f5db.c) | api=setjmp, shape=loop, live=1, first-arm=busy | c17 | `12 2 4 ...` and 2 more lines |
| [`sigsetjmp.setjmp.loop.1.quiet.c17.f994dbd1`](sigsetjmp.setjmp.loop.1.quiet.c17.f994dbd1.c) | api=setjmp, shape=loop, live=1, first-arm=quiet | c17 | `12 2 4 4` |
| [`sigsetjmp.setjmp.loop.12.busy.c17.f49970e2`](sigsetjmp.setjmp.loop.12.busy.c17.f49970e2.c) | api=setjmp, shape=loop, live=12, first-arm=busy | c17 | `12 6004 66 ...` and 13 more lines |
| [`sigsetjmp.setjmp.loop.12.quiet.c17.46bfad91`](sigsetjmp.setjmp.loop.12.quiet.c17.46bfad91.c) | api=setjmp, shape=loop, live=12, first-arm=quiet | c17 | `12 6004 66 ...` and 12 more lines |
| [`sigsetjmp.setjmp.loop.2.busy.c17.bce88194`](sigsetjmp.setjmp.loop.2.busy.c17.bce88194.c) | api=setjmp, shape=loop, live=2, first-arm=busy | c17 | `12 6004 2 ...` and 3 more lines |
| [`sigsetjmp.setjmp.loop.2.quiet.c17.355803a5`](sigsetjmp.setjmp.loop.2.quiet.c17.355803a5.c) | api=setjmp, shape=loop, live=2, first-arm=quiet | c17 | `12 6004 2 ...` and 2 more lines |
| [`sigsetjmp.setjmp.loop.20.busy.c17.62492158`](sigsetjmp.setjmp.loop.20.busy.c17.62492158.c) | api=setjmp, shape=loop, live=20, first-arm=busy | c17 | `12 6004 66 ...` and 21 more lines |
| [`sigsetjmp.setjmp.loop.20.quiet.c17.68445b52`](sigsetjmp.setjmp.loop.20.quiet.c17.68445b52.c) | api=setjmp, shape=loop, live=20, first-arm=quiet | c17 | `12 6004 66 ...` and 20 more lines |
| [`sigsetjmp.setjmp.loop.30.busy.c17.98cf3b45`](sigsetjmp.setjmp.loop.30.busy.c17.98cf3b45.c) | api=setjmp, shape=loop, live=30, first-arm=busy | c17 | `12 6004 66 ...` and 31 more lines |
| [`sigsetjmp.setjmp.loop.30.quiet.c17.0be2c43d`](sigsetjmp.setjmp.loop.30.quiet.c17.0be2c43d.c) | api=setjmp, shape=loop, live=30, first-arm=quiet | c17 | `12 6004 66 ...` and 30 more lines |
| [`sigsetjmp.setjmp.loop.4.busy.c17.ac5ba9f8`](sigsetjmp.setjmp.loop.4.busy.c17.ac5ba9f8.c) | api=setjmp, shape=loop, live=4, first-arm=busy | c17 | `12 6004 66 ...` and 5 more lines |
| [`sigsetjmp.setjmp.loop.4.quiet.c17.9b6a448b`](sigsetjmp.setjmp.loop.4.quiet.c17.9b6a448b.c) | api=setjmp, shape=loop, live=4, first-arm=quiet | c17 | `12 6004 66 ...` and 4 more lines |
| [`sigsetjmp.setjmp.loop.7.busy.c17.83ce930d`](sigsetjmp.setjmp.loop.7.busy.c17.83ce930d.c) | api=setjmp, shape=loop, live=7, first-arm=busy | c17 | `12 6004 66 ...` and 8 more lines |
| [`sigsetjmp.setjmp.loop.7.quiet.c17.2a214862`](sigsetjmp.setjmp.loop.7.quiet.c17.2a214862.c) | api=setjmp, shape=loop, live=7, first-arm=quiet | c17 | `12 6004 66 ...` and 7 more lines |
| [`sigsetjmp.setjmp.no-error.1.busy.c17.015be86c`](sigsetjmp.setjmp.no-error.1.busy.c17.015be86c.c) | api=setjmp, shape=no-error, live=1, first-arm=busy | c17 | `6 0 2 ...` and 2 more lines |
| [`sigsetjmp.setjmp.no-error.1.quiet.c17.4132792f`](sigsetjmp.setjmp.no-error.1.quiet.c17.4132792f.c) | api=setjmp, shape=no-error, live=1, first-arm=quiet | c17 | `6 0 2 1` |
| [`sigsetjmp.setjmp.no-error.12.busy.c17.154646fd`](sigsetjmp.setjmp.no-error.12.busy.c17.154646fd.c) | api=setjmp, shape=no-error, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`sigsetjmp.setjmp.no-error.12.quiet.c17.43b68c0b`](sigsetjmp.setjmp.no-error.12.quiet.c17.43b68c0b.c) | api=setjmp, shape=no-error, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`sigsetjmp.setjmp.no-error.2.busy.c17.cbd39a67`](sigsetjmp.setjmp.no-error.2.busy.c17.cbd39a67.c) | api=setjmp, shape=no-error, live=2, first-arm=busy | c17 | `6 3002 0 ...` and 3 more lines |
| [`sigsetjmp.setjmp.no-error.2.quiet.c17.049f0d25`](sigsetjmp.setjmp.no-error.2.quiet.c17.049f0d25.c) | api=setjmp, shape=no-error, live=2, first-arm=quiet | c17 | `6 3002 0 ...` and 2 more lines |
| [`sigsetjmp.setjmp.no-error.20.busy.c17.4764b4a8`](sigsetjmp.setjmp.no-error.20.busy.c17.4764b4a8.c) | api=setjmp, shape=no-error, live=20, first-arm=busy | c17 | `6 3002 21 ...` and 21 more lines |
| [`sigsetjmp.setjmp.no-error.20.quiet.c17.89b33e28`](sigsetjmp.setjmp.no-error.20.quiet.c17.89b33e28.c) | api=setjmp, shape=no-error, live=20, first-arm=quiet | c17 | `6 3002 21 ...` and 20 more lines |
| [`sigsetjmp.setjmp.no-error.30.busy.c17.8f4b254b`](sigsetjmp.setjmp.no-error.30.busy.c17.8f4b254b.c) | api=setjmp, shape=no-error, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`sigsetjmp.setjmp.no-error.30.quiet.c17.96844d69`](sigsetjmp.setjmp.no-error.30.quiet.c17.96844d69.c) | api=setjmp, shape=no-error, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`sigsetjmp.setjmp.no-error.4.busy.c17.c06ac6eb`](sigsetjmp.setjmp.no-error.4.busy.c17.c06ac6eb.c) | api=setjmp, shape=no-error, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`sigsetjmp.setjmp.no-error.4.quiet.c17.ee7081a8`](sigsetjmp.setjmp.no-error.4.quiet.c17.ee7081a8.c) | api=setjmp, shape=no-error, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`sigsetjmp.setjmp.no-error.7.busy.c17.c26a491a`](sigsetjmp.setjmp.no-error.7.busy.c17.c26a491a.c) | api=setjmp, shape=no-error, live=7, first-arm=busy | c17 | `6 3002 21 ...` and 8 more lines |
| [`sigsetjmp.setjmp.no-error.7.quiet.c17.9e95ce60`](sigsetjmp.setjmp.no-error.7.quiet.c17.9e95ce60.c) | api=setjmp, shape=no-error, live=7, first-arm=quiet | c17 | `6 3002 21 ...` and 7 more lines |
| [`sigsetjmp.setjmp.rethrow.1.busy.c17.a3786bab`](sigsetjmp.setjmp.rethrow.1.busy.c17.a3786bab.c) | api=setjmp, shape=rethrow, live=1, first-arm=busy | c17 | `6 1 2 ...` and 3 more lines |
| [`sigsetjmp.setjmp.rethrow.1.quiet.c17.79101501`](sigsetjmp.setjmp.rethrow.1.quiet.c17.79101501.c) | api=setjmp, shape=rethrow, live=1, first-arm=quiet | c17 | `6 1 2 ...` and 2 more lines |
| [`sigsetjmp.setjmp.rethrow.12.busy.c17.80e1b78c`](sigsetjmp.setjmp.rethrow.12.busy.c17.80e1b78c.c) | api=setjmp, shape=rethrow, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 14 more lines |
| [`sigsetjmp.setjmp.rethrow.12.quiet.c17.be64804f`](sigsetjmp.setjmp.rethrow.12.quiet.c17.be64804f.c) | api=setjmp, shape=rethrow, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 13 more lines |
| [`sigsetjmp.setjmp.rethrow.2.busy.c17.55d757a8`](sigsetjmp.setjmp.rethrow.2.busy.c17.55d757a8.c) | api=setjmp, shape=rethrow, live=2, first-arm=busy | c17 | `6 3002 1 ...` and 4 more lines |
| [`sigsetjmp.setjmp.rethrow.2.quiet.c17.5d80f409`](sigsetjmp.setjmp.rethrow.2.quiet.c17.5d80f409.c) | api=setjmp, shape=rethrow, live=2, first-arm=quiet | c17 | `6 3002 1 ...` and 3 more lines |
| [`sigsetjmp.setjmp.rethrow.20.busy.c17.430473d6`](sigsetjmp.setjmp.rethrow.20.busy.c17.430473d6.c) | api=setjmp, shape=rethrow, live=20, first-arm=busy | c17 | `6 3002 21 ...` and 22 more lines |
| [`sigsetjmp.setjmp.rethrow.20.quiet.c17.97e3c469`](sigsetjmp.setjmp.rethrow.20.quiet.c17.97e3c469.c) | api=setjmp, shape=rethrow, live=20, first-arm=quiet | c17 | `6 3002 21 ...` and 21 more lines |
| [`sigsetjmp.setjmp.rethrow.30.busy.c17.2e7c7e69`](sigsetjmp.setjmp.rethrow.30.busy.c17.2e7c7e69.c) | api=setjmp, shape=rethrow, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 32 more lines |
| [`sigsetjmp.setjmp.rethrow.30.quiet.c17.dfdc8108`](sigsetjmp.setjmp.rethrow.30.quiet.c17.dfdc8108.c) | api=setjmp, shape=rethrow, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 31 more lines |
| [`sigsetjmp.setjmp.rethrow.4.busy.c17.58d24482`](sigsetjmp.setjmp.rethrow.4.busy.c17.58d24482.c) | api=setjmp, shape=rethrow, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 6 more lines |
| [`sigsetjmp.setjmp.rethrow.4.quiet.c17.f675917a`](sigsetjmp.setjmp.rethrow.4.quiet.c17.f675917a.c) | api=setjmp, shape=rethrow, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 5 more lines |
| [`sigsetjmp.setjmp.rethrow.7.busy.c17.7e72e9a9`](sigsetjmp.setjmp.rethrow.7.busy.c17.7e72e9a9.c) | api=setjmp, shape=rethrow, live=7, first-arm=busy | c17 | `6 3002 21 ...` and 9 more lines |
| [`sigsetjmp.setjmp.rethrow.7.quiet.c17.a265ec98`](sigsetjmp.setjmp.rethrow.7.quiet.c17.a265ec98.c) | api=setjmp, shape=rethrow, live=7, first-arm=quiet | c17 | `6 3002 21 ...` and 8 more lines |
| [`sigsetjmp.sigsetjmp.catch.1.busy.c17.57fa2af4`](sigsetjmp.sigsetjmp.catch.1.busy.c17.57fa2af4.c) | api=sigsetjmp, shape=catch, live=1, first-arm=busy | c17 | `6 1 1 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.catch.1.quiet.c17.2f3e6b29`](sigsetjmp.sigsetjmp.catch.1.quiet.c17.2f3e6b29.c) | api=sigsetjmp, shape=catch, live=1, first-arm=quiet | c17 | `6 1 1 1` |
| [`sigsetjmp.sigsetjmp.catch.12.busy.c17.16a3f096`](sigsetjmp.sigsetjmp.catch.12.busy.c17.16a3f096.c) | api=sigsetjmp, shape=catch, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`sigsetjmp.sigsetjmp.catch.12.quiet.c17.978fa12c`](sigsetjmp.sigsetjmp.catch.12.quiet.c17.978fa12c.c) | api=sigsetjmp, shape=catch, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`sigsetjmp.sigsetjmp.catch.2.busy.c17.0aece6fc`](sigsetjmp.sigsetjmp.catch.2.busy.c17.0aece6fc.c) | api=sigsetjmp, shape=catch, live=2, first-arm=busy | c17 | `6 3002 1 ...` and 3 more lines |
| [`sigsetjmp.sigsetjmp.catch.2.quiet.c17.1b145230`](sigsetjmp.sigsetjmp.catch.2.quiet.c17.1b145230.c) | api=sigsetjmp, shape=catch, live=2, first-arm=quiet | c17 | `6 3002 1 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.catch.20.busy.c17.6bcdf2ed`](sigsetjmp.sigsetjmp.catch.20.busy.c17.6bcdf2ed.c) | api=sigsetjmp, shape=catch, live=20, first-arm=busy | c17 | `6 3002 21 ...` and 21 more lines |
| [`sigsetjmp.sigsetjmp.catch.20.quiet.c17.8a9b7f66`](sigsetjmp.sigsetjmp.catch.20.quiet.c17.8a9b7f66.c) | api=sigsetjmp, shape=catch, live=20, first-arm=quiet | c17 | `6 3002 21 ...` and 20 more lines |
| [`sigsetjmp.sigsetjmp.catch.30.busy.c17.a5cb4d76`](sigsetjmp.sigsetjmp.catch.30.busy.c17.a5cb4d76.c) | api=sigsetjmp, shape=catch, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`sigsetjmp.sigsetjmp.catch.30.quiet.c17.098e6bc3`](sigsetjmp.sigsetjmp.catch.30.quiet.c17.098e6bc3.c) | api=sigsetjmp, shape=catch, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`sigsetjmp.sigsetjmp.catch.4.busy.c17.fa960555`](sigsetjmp.sigsetjmp.catch.4.busy.c17.fa960555.c) | api=sigsetjmp, shape=catch, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`sigsetjmp.sigsetjmp.catch.4.quiet.c17.ef8109e2`](sigsetjmp.sigsetjmp.catch.4.quiet.c17.ef8109e2.c) | api=sigsetjmp, shape=catch, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`sigsetjmp.sigsetjmp.catch.7.busy.c17.07f79525`](sigsetjmp.sigsetjmp.catch.7.busy.c17.07f79525.c) | api=sigsetjmp, shape=catch, live=7, first-arm=busy | c17 | `6 3002 21 ...` and 8 more lines |
| [`sigsetjmp.sigsetjmp.catch.7.quiet.c17.e9a68784`](sigsetjmp.sigsetjmp.catch.7.quiet.c17.e9a68784.c) | api=sigsetjmp, shape=catch, live=7, first-arm=quiet | c17 | `6 3002 21 ...` and 7 more lines |
| [`sigsetjmp.sigsetjmp.loop.1.busy.c17.e4e07b85`](sigsetjmp.sigsetjmp.loop.1.busy.c17.e4e07b85.c) | api=sigsetjmp, shape=loop, live=1, first-arm=busy | c17 | `12 2 4 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.loop.1.quiet.c17.b2f9f7d2`](sigsetjmp.sigsetjmp.loop.1.quiet.c17.b2f9f7d2.c) | api=sigsetjmp, shape=loop, live=1, first-arm=quiet | c17 | `12 2 4 4` |
| [`sigsetjmp.sigsetjmp.loop.12.busy.c17.6040fe67`](sigsetjmp.sigsetjmp.loop.12.busy.c17.6040fe67.c) | api=sigsetjmp, shape=loop, live=12, first-arm=busy | c17 | `12 6004 66 ...` and 13 more lines |
| [`sigsetjmp.sigsetjmp.loop.12.quiet.c17.9370192c`](sigsetjmp.sigsetjmp.loop.12.quiet.c17.9370192c.c) | api=sigsetjmp, shape=loop, live=12, first-arm=quiet | c17 | `12 6004 66 ...` and 12 more lines |
| [`sigsetjmp.sigsetjmp.loop.2.busy.c17.ce7d6704`](sigsetjmp.sigsetjmp.loop.2.busy.c17.ce7d6704.c) | api=sigsetjmp, shape=loop, live=2, first-arm=busy | c17 | `12 6004 2 ...` and 3 more lines |
| [`sigsetjmp.sigsetjmp.loop.2.quiet.c17.e19bd5ad`](sigsetjmp.sigsetjmp.loop.2.quiet.c17.e19bd5ad.c) | api=sigsetjmp, shape=loop, live=2, first-arm=quiet | c17 | `12 6004 2 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.loop.20.busy.c17.f01d4856`](sigsetjmp.sigsetjmp.loop.20.busy.c17.f01d4856.c) | api=sigsetjmp, shape=loop, live=20, first-arm=busy | c17 | `12 6004 66 ...` and 21 more lines |
| [`sigsetjmp.sigsetjmp.loop.20.quiet.c17.dd45adf7`](sigsetjmp.sigsetjmp.loop.20.quiet.c17.dd45adf7.c) | api=sigsetjmp, shape=loop, live=20, first-arm=quiet | c17 | `12 6004 66 ...` and 20 more lines |
| [`sigsetjmp.sigsetjmp.loop.30.busy.c17.0c9d660a`](sigsetjmp.sigsetjmp.loop.30.busy.c17.0c9d660a.c) | api=sigsetjmp, shape=loop, live=30, first-arm=busy | c17 | `12 6004 66 ...` and 31 more lines |
| [`sigsetjmp.sigsetjmp.loop.30.quiet.c17.bb8973c1`](sigsetjmp.sigsetjmp.loop.30.quiet.c17.bb8973c1.c) | api=sigsetjmp, shape=loop, live=30, first-arm=quiet | c17 | `12 6004 66 ...` and 30 more lines |
| [`sigsetjmp.sigsetjmp.loop.4.busy.c17.7cd5689f`](sigsetjmp.sigsetjmp.loop.4.busy.c17.7cd5689f.c) | api=sigsetjmp, shape=loop, live=4, first-arm=busy | c17 | `12 6004 66 ...` and 5 more lines |
| [`sigsetjmp.sigsetjmp.loop.4.quiet.c17.bc08d613`](sigsetjmp.sigsetjmp.loop.4.quiet.c17.bc08d613.c) | api=sigsetjmp, shape=loop, live=4, first-arm=quiet | c17 | `12 6004 66 ...` and 4 more lines |
| [`sigsetjmp.sigsetjmp.loop.7.busy.c17.46036cfa`](sigsetjmp.sigsetjmp.loop.7.busy.c17.46036cfa.c) | api=sigsetjmp, shape=loop, live=7, first-arm=busy | c17 | `12 6004 66 ...` and 8 more lines |
| [`sigsetjmp.sigsetjmp.loop.7.quiet.c17.5bf002b2`](sigsetjmp.sigsetjmp.loop.7.quiet.c17.5bf002b2.c) | api=sigsetjmp, shape=loop, live=7, first-arm=quiet | c17 | `12 6004 66 ...` and 7 more lines |
| [`sigsetjmp.sigsetjmp.no-error.1.busy.c17.92d08477`](sigsetjmp.sigsetjmp.no-error.1.busy.c17.92d08477.c) | api=sigsetjmp, shape=no-error, live=1, first-arm=busy | c17 | `6 0 2 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.no-error.1.quiet.c17.3e6d3e9c`](sigsetjmp.sigsetjmp.no-error.1.quiet.c17.3e6d3e9c.c) | api=sigsetjmp, shape=no-error, live=1, first-arm=quiet | c17 | `6 0 2 1` |
| [`sigsetjmp.sigsetjmp.no-error.12.busy.c17.dc495b25`](sigsetjmp.sigsetjmp.no-error.12.busy.c17.dc495b25.c) | api=sigsetjmp, shape=no-error, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 13 more lines |
| [`sigsetjmp.sigsetjmp.no-error.12.quiet.c17.ffa3ade7`](sigsetjmp.sigsetjmp.no-error.12.quiet.c17.ffa3ade7.c) | api=sigsetjmp, shape=no-error, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 12 more lines |
| [`sigsetjmp.sigsetjmp.no-error.2.busy.c17.7c894985`](sigsetjmp.sigsetjmp.no-error.2.busy.c17.7c894985.c) | api=sigsetjmp, shape=no-error, live=2, first-arm=busy | c17 | `6 3002 0 ...` and 3 more lines |
| [`sigsetjmp.sigsetjmp.no-error.2.quiet.c17.2b595f10`](sigsetjmp.sigsetjmp.no-error.2.quiet.c17.2b595f10.c) | api=sigsetjmp, shape=no-error, live=2, first-arm=quiet | c17 | `6 3002 0 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.no-error.20.busy.c17.b33e576d`](sigsetjmp.sigsetjmp.no-error.20.busy.c17.b33e576d.c) | api=sigsetjmp, shape=no-error, live=20, first-arm=busy | c17 | `6 3002 21 ...` and 21 more lines |
| [`sigsetjmp.sigsetjmp.no-error.20.quiet.c17.cc148278`](sigsetjmp.sigsetjmp.no-error.20.quiet.c17.cc148278.c) | api=sigsetjmp, shape=no-error, live=20, first-arm=quiet | c17 | `6 3002 21 ...` and 20 more lines |
| [`sigsetjmp.sigsetjmp.no-error.30.busy.c17.0675fcc2`](sigsetjmp.sigsetjmp.no-error.30.busy.c17.0675fcc2.c) | api=sigsetjmp, shape=no-error, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 31 more lines |
| [`sigsetjmp.sigsetjmp.no-error.30.quiet.c17.0319ee67`](sigsetjmp.sigsetjmp.no-error.30.quiet.c17.0319ee67.c) | api=sigsetjmp, shape=no-error, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 30 more lines |
| [`sigsetjmp.sigsetjmp.no-error.4.busy.c17.1a374e3a`](sigsetjmp.sigsetjmp.no-error.4.busy.c17.1a374e3a.c) | api=sigsetjmp, shape=no-error, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 5 more lines |
| [`sigsetjmp.sigsetjmp.no-error.4.quiet.c17.6b500872`](sigsetjmp.sigsetjmp.no-error.4.quiet.c17.6b500872.c) | api=sigsetjmp, shape=no-error, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 4 more lines |
| [`sigsetjmp.sigsetjmp.no-error.7.busy.c17.4005f58c`](sigsetjmp.sigsetjmp.no-error.7.busy.c17.4005f58c.c) | api=sigsetjmp, shape=no-error, live=7, first-arm=busy | c17 | `6 3002 21 ...` and 8 more lines |
| [`sigsetjmp.sigsetjmp.no-error.7.quiet.c17.fe0e78c5`](sigsetjmp.sigsetjmp.no-error.7.quiet.c17.fe0e78c5.c) | api=sigsetjmp, shape=no-error, live=7, first-arm=quiet | c17 | `6 3002 21 ...` and 7 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.1.busy.c17.6bc91bf5`](sigsetjmp.sigsetjmp.rethrow.1.busy.c17.6bc91bf5.c) | api=sigsetjmp, shape=rethrow, live=1, first-arm=busy | c17 | `6 1 2 ...` and 3 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.1.quiet.c17.ddfc1229`](sigsetjmp.sigsetjmp.rethrow.1.quiet.c17.ddfc1229.c) | api=sigsetjmp, shape=rethrow, live=1, first-arm=quiet | c17 | `6 1 2 ...` and 2 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.12.busy.c17.7cae6c46`](sigsetjmp.sigsetjmp.rethrow.12.busy.c17.7cae6c46.c) | api=sigsetjmp, shape=rethrow, live=12, first-arm=busy | c17 | `6 3002 21 ...` and 14 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.12.quiet.c17.1c31ff15`](sigsetjmp.sigsetjmp.rethrow.12.quiet.c17.1c31ff15.c) | api=sigsetjmp, shape=rethrow, live=12, first-arm=quiet | c17 | `6 3002 21 ...` and 13 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.2.busy.c17.c16d1a68`](sigsetjmp.sigsetjmp.rethrow.2.busy.c17.c16d1a68.c) | api=sigsetjmp, shape=rethrow, live=2, first-arm=busy | c17 | `6 3002 1 ...` and 4 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.2.quiet.c17.1dee4608`](sigsetjmp.sigsetjmp.rethrow.2.quiet.c17.1dee4608.c) | api=sigsetjmp, shape=rethrow, live=2, first-arm=quiet | c17 | `6 3002 1 ...` and 3 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.20.busy.c17.0c0653be`](sigsetjmp.sigsetjmp.rethrow.20.busy.c17.0c0653be.c) | api=sigsetjmp, shape=rethrow, live=20, first-arm=busy | c17 | `6 3002 21 ...` and 22 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.20.quiet.c17.1b616961`](sigsetjmp.sigsetjmp.rethrow.20.quiet.c17.1b616961.c) | api=sigsetjmp, shape=rethrow, live=20, first-arm=quiet | c17 | `6 3002 21 ...` and 21 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.30.busy.c17.fde25a6e`](sigsetjmp.sigsetjmp.rethrow.30.busy.c17.fde25a6e.c) | api=sigsetjmp, shape=rethrow, live=30, first-arm=busy | c17 | `6 3002 21 ...` and 32 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.30.quiet.c17.f53b6f4d`](sigsetjmp.sigsetjmp.rethrow.30.quiet.c17.f53b6f4d.c) | api=sigsetjmp, shape=rethrow, live=30, first-arm=quiet | c17 | `6 3002 21 ...` and 31 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.4.busy.c17.c3c39820`](sigsetjmp.sigsetjmp.rethrow.4.busy.c17.c3c39820.c) | api=sigsetjmp, shape=rethrow, live=4, first-arm=busy | c17 | `6 3002 21 ...` and 6 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.4.quiet.c17.3bfc0648`](sigsetjmp.sigsetjmp.rethrow.4.quiet.c17.3bfc0648.c) | api=sigsetjmp, shape=rethrow, live=4, first-arm=quiet | c17 | `6 3002 21 ...` and 5 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.7.busy.c17.b3dcc1ea`](sigsetjmp.sigsetjmp.rethrow.7.busy.c17.b3dcc1ea.c) | api=sigsetjmp, shape=rethrow, live=7, first-arm=busy | c17 | `6 3002 21 ...` and 9 more lines |
| [`sigsetjmp.sigsetjmp.rethrow.7.quiet.c17.91c021ac`](sigsetjmp.sigsetjmp.rethrow.7.quiet.c17.91c021ac.c) | api=sigsetjmp, shape=rethrow, live=7, first-arm=quiet | c17 | `6 3002 21 ...` and 8 more lines |

