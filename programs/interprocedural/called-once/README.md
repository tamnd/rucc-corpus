# called-once

a static function called from one place, timed in a hot loop. Part of the interprocedural phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`called-once.chain.8.c17.ab122fc0`](called-once.chain.8.c17.ab122fc0.c) | shape=chain, steps=8 | c17 | `8981144779908096` |
| [`called-once.loop.4.c17.6d565be3`](called-once.loop.4.c17.6d565be3.c) | shape=loop, steps=4 | c17 | `8951970647040000` |
| [`called-once.mix.2.c17.808bd61e`](called-once.mix.2.c17.808bd61e.c) | shape=mix, steps=2 | c17 | `9079324724409344` |
| [`called-once.mix.8.c17.0fb4c875`](called-once.mix.8.c17.0fb4c875.c) | shape=mix, steps=8 | c17 | `9046951916363776` |
| [`called-once.mix.96.c17.9c02be0f`](called-once.mix.96.c17.9c02be0f.c) | shape=mix, steps=96 | c17 | `9012661833986048` |

