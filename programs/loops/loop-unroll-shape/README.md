# loop-unroll-shape

the loop shapes an unroller has to count or refuse. Part of the loops phase of the M4 plan.

80 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-unroll-shape.i32.a-branch-inside-the-body.c17.48508510`](loop-unroll-shape.i32.a-branch-inside-the-body.c17.48508510.c) | type=i32, shape=a-branch-inside-the-body | c17 | `6` |
| [`loop-unroll-shape.i32.a-call-in-the-body.c17.a69a38e9`](loop-unroll-shape.i32.a-call-in-the-body.c17.a69a38e9.c) | type=i32, shape=a-call-in-the-body | c17 | `5 5` |
| [`loop-unroll-shape.i32.a-counter-narrower-than-int.c17.b691bad6`](loop-unroll-shape.i32.a-counter-narrower-than-int.c17.b691bad6.c) | type=i32, shape=a-counter-narrower-than-int | c17 | `200` |
| [`loop-unroll-shape.i32.a-counter-read-after-the-loop.c17.89f6b4d5`](loop-unroll-shape.i32.a-counter-read-after-the-loop.c17.89f6b4d5.c) | type=i32, shape=a-counter-read-after-the-loop | c17 | `4 10` |
| [`loop-unroll-shape.i32.a-nest-both-counted.c17.c72e6d09`](loop-unroll-shape.i32.a-nest-both-counted.c17.c72e6d09.c) | type=i32, shape=a-nest-both-counted | c17 | `18` |
| [`loop-unroll-shape.i32.a-nest-the-inner-one-unknown.c17.0fcbda6b`](loop-unroll-shape.i32.a-nest-the-inner-one-unknown.c17.0fcbda6b.c) | type=i32, shape=a-nest-the-inner-one-unknown | c17 | `18` |
| [`loop-unroll-shape.i32.a-second-way-out.c17.46fb3ab2`](loop-unroll-shape.i32.a-second-way-out.c17.46fb3ab2.c) | type=i32, shape=a-second-way-out | c17 | `6` |
| [`loop-unroll-shape.i32.a-step-that-does-not-divide.c17.7e8cc829`](loop-unroll-shape.i32.a-step-that-does-not-divide.c17.7e8cc829.c) | type=i32, shape=a-step-that-does-not-divide | c17 | `18` |
| [`loop-unroll-shape.i32.an-exclusive-bound.c17.2163ee14`](loop-unroll-shape.i32.an-exclusive-bound.c17.2163ee14.c) | type=i32, shape=an-exclusive-bound | c17 | `21` |
| [`loop-unroll-shape.i32.an-inclusive-bound.c17.e545b410`](loop-unroll-shape.i32.an-inclusive-bound.c17.e545b410.c) | type=i32, shape=an-inclusive-bound | c17 | `28` |
| [`loop-unroll-shape.i32.an-unsigned-counter.c17.15edd792`](loop-unroll-shape.i32.an-unsigned-counter.c17.15edd792.c) | type=i32, shape=an-unsigned-counter | c17 | `36` |
| [`loop-unroll-shape.i32.counting-down-inclusive.c17.b232bb1a`](loop-unroll-shape.i32.counting-down-inclusive.c17.b232bb1a.c) | type=i32, shape=counting-down-inclusive | c17 | `21` |
| [`loop-unroll-shape.i32.counting-down.c17.cb5d4a58`](loop-unroll-shape.i32.counting-down.c17.cb5d4a58.c) | type=i32, shape=counting-down | c17 | `21` |
| [`loop-unroll-shape.i32.landing-on-the-bound-exactly.c17.d7ce3de7`](loop-unroll-shape.i32.landing-on-the-bound-exactly.c17.d7ce3de7.c) | type=i32, shape=landing-on-the-bound-exactly | c17 | `12` |
| [`loop-unroll-shape.i32.no-iterations.c17.5388dfdd`](loop-unroll-shape.i32.no-iterations.c17.5388dfdd.c) | type=i32, shape=no-iterations | c17 | `99` |
| [`loop-unroll-shape.i32.one-iteration.c17.e5573f59`](loop-unroll-shape.i32.one-iteration.c17.e5573f59.c) | type=i32, shape=one-iteration | c17 | `5` |
| [`loop-unroll-shape.i32.one-past-the-copy-limit.c17.212720fb`](loop-unroll-shape.i32.one-past-the-copy-limit.c17.212720fb.c) | type=i32, shape=one-past-the-copy-limit | c17 | `136` |
| [`loop-unroll-shape.i32.right-at-the-copy-limit.c17.014cb9a4`](loop-unroll-shape.i32.right-at-the-copy-limit.c17.014cb9a4.c) | type=i32, shape=right-at-the-copy-limit | c17 | `120` |
| [`loop-unroll-shape.i32.stores-through-a-pointer.c17.2329da93`](loop-unroll-shape.i32.stores-through-a-pointer.c17.2329da93.c) | type=i32, shape=stores-through-a-pointer | c17 | `45` |
| [`loop-unroll-shape.i32.testing-at-the-bottom.c17.57dda1bb`](loop-unroll-shape.i32.testing-at-the-bottom.c17.57dda1bb.c) | type=i32, shape=testing-at-the-bottom | c17 | `15` |
| [`loop-unroll-shape.i64.a-branch-inside-the-body.c17.4aa97990`](loop-unroll-shape.i64.a-branch-inside-the-body.c17.4aa97990.c) | type=i64, shape=a-branch-inside-the-body | c17 | `6` |
| [`loop-unroll-shape.i64.a-call-in-the-body.c17.facb6db4`](loop-unroll-shape.i64.a-call-in-the-body.c17.facb6db4.c) | type=i64, shape=a-call-in-the-body | c17 | `5 5` |
| [`loop-unroll-shape.i64.a-counter-narrower-than-int.c17.ba6191b6`](loop-unroll-shape.i64.a-counter-narrower-than-int.c17.ba6191b6.c) | type=i64, shape=a-counter-narrower-than-int | c17 | `200` |
| [`loop-unroll-shape.i64.a-counter-read-after-the-loop.c17.5a6f4f94`](loop-unroll-shape.i64.a-counter-read-after-the-loop.c17.5a6f4f94.c) | type=i64, shape=a-counter-read-after-the-loop | c17 | `4 10` |
| [`loop-unroll-shape.i64.a-nest-both-counted.c17.1b845020`](loop-unroll-shape.i64.a-nest-both-counted.c17.1b845020.c) | type=i64, shape=a-nest-both-counted | c17 | `18` |
| [`loop-unroll-shape.i64.a-nest-the-inner-one-unknown.c17.d16c3a17`](loop-unroll-shape.i64.a-nest-the-inner-one-unknown.c17.d16c3a17.c) | type=i64, shape=a-nest-the-inner-one-unknown | c17 | `18` |
| [`loop-unroll-shape.i64.a-second-way-out.c17.c276f7d9`](loop-unroll-shape.i64.a-second-way-out.c17.c276f7d9.c) | type=i64, shape=a-second-way-out | c17 | `6` |
| [`loop-unroll-shape.i64.a-step-that-does-not-divide.c17.e028609a`](loop-unroll-shape.i64.a-step-that-does-not-divide.c17.e028609a.c) | type=i64, shape=a-step-that-does-not-divide | c17 | `18` |
| [`loop-unroll-shape.i64.an-exclusive-bound.c17.53a6a46e`](loop-unroll-shape.i64.an-exclusive-bound.c17.53a6a46e.c) | type=i64, shape=an-exclusive-bound | c17 | `21` |
| [`loop-unroll-shape.i64.an-inclusive-bound.c17.1bf35c8c`](loop-unroll-shape.i64.an-inclusive-bound.c17.1bf35c8c.c) | type=i64, shape=an-inclusive-bound | c17 | `28` |
| [`loop-unroll-shape.i64.an-unsigned-counter.c17.54c82248`](loop-unroll-shape.i64.an-unsigned-counter.c17.54c82248.c) | type=i64, shape=an-unsigned-counter | c17 | `36` |
| [`loop-unroll-shape.i64.counting-down-inclusive.c17.397719ce`](loop-unroll-shape.i64.counting-down-inclusive.c17.397719ce.c) | type=i64, shape=counting-down-inclusive | c17 | `21` |
| [`loop-unroll-shape.i64.counting-down.c17.76c6982c`](loop-unroll-shape.i64.counting-down.c17.76c6982c.c) | type=i64, shape=counting-down | c17 | `21` |
| [`loop-unroll-shape.i64.landing-on-the-bound-exactly.c17.fb13bde9`](loop-unroll-shape.i64.landing-on-the-bound-exactly.c17.fb13bde9.c) | type=i64, shape=landing-on-the-bound-exactly | c17 | `12` |
| [`loop-unroll-shape.i64.no-iterations.c17.fc2c9f01`](loop-unroll-shape.i64.no-iterations.c17.fc2c9f01.c) | type=i64, shape=no-iterations | c17 | `99` |
| [`loop-unroll-shape.i64.one-iteration.c17.46bdda29`](loop-unroll-shape.i64.one-iteration.c17.46bdda29.c) | type=i64, shape=one-iteration | c17 | `5` |
| [`loop-unroll-shape.i64.one-past-the-copy-limit.c17.71c901f5`](loop-unroll-shape.i64.one-past-the-copy-limit.c17.71c901f5.c) | type=i64, shape=one-past-the-copy-limit | c17 | `136` |
| [`loop-unroll-shape.i64.right-at-the-copy-limit.c17.16a42c75`](loop-unroll-shape.i64.right-at-the-copy-limit.c17.16a42c75.c) | type=i64, shape=right-at-the-copy-limit | c17 | `120` |
| [`loop-unroll-shape.i64.stores-through-a-pointer.c17.5bba503b`](loop-unroll-shape.i64.stores-through-a-pointer.c17.5bba503b.c) | type=i64, shape=stores-through-a-pointer | c17 | `45` |
| [`loop-unroll-shape.i64.testing-at-the-bottom.c17.de5e5979`](loop-unroll-shape.i64.testing-at-the-bottom.c17.de5e5979.c) | type=i64, shape=testing-at-the-bottom | c17 | `15` |
| [`loop-unroll-shape.u32.a-branch-inside-the-body.c17.3bcd6ccb`](loop-unroll-shape.u32.a-branch-inside-the-body.c17.3bcd6ccb.c) | type=u32, shape=a-branch-inside-the-body | c17 | `6` |
| [`loop-unroll-shape.u32.a-call-in-the-body.c17.d7540a33`](loop-unroll-shape.u32.a-call-in-the-body.c17.d7540a33.c) | type=u32, shape=a-call-in-the-body | c17 | `5 5` |
| [`loop-unroll-shape.u32.a-counter-narrower-than-int.c17.72b278a4`](loop-unroll-shape.u32.a-counter-narrower-than-int.c17.72b278a4.c) | type=u32, shape=a-counter-narrower-than-int | c17 | `200` |
| [`loop-unroll-shape.u32.a-counter-read-after-the-loop.c17.5c7689d4`](loop-unroll-shape.u32.a-counter-read-after-the-loop.c17.5c7689d4.c) | type=u32, shape=a-counter-read-after-the-loop | c17 | `4 10` |
| [`loop-unroll-shape.u32.a-nest-both-counted.c17.1b277a80`](loop-unroll-shape.u32.a-nest-both-counted.c17.1b277a80.c) | type=u32, shape=a-nest-both-counted | c17 | `18` |
| [`loop-unroll-shape.u32.a-nest-the-inner-one-unknown.c17.9d0c83fb`](loop-unroll-shape.u32.a-nest-the-inner-one-unknown.c17.9d0c83fb.c) | type=u32, shape=a-nest-the-inner-one-unknown | c17 | `18` |
| [`loop-unroll-shape.u32.a-second-way-out.c17.9f90528d`](loop-unroll-shape.u32.a-second-way-out.c17.9f90528d.c) | type=u32, shape=a-second-way-out | c17 | `6` |
| [`loop-unroll-shape.u32.a-step-that-does-not-divide.c17.fcb26fa1`](loop-unroll-shape.u32.a-step-that-does-not-divide.c17.fcb26fa1.c) | type=u32, shape=a-step-that-does-not-divide | c17 | `18` |
| [`loop-unroll-shape.u32.an-exclusive-bound.c17.897a2f20`](loop-unroll-shape.u32.an-exclusive-bound.c17.897a2f20.c) | type=u32, shape=an-exclusive-bound | c17 | `21` |
| [`loop-unroll-shape.u32.an-inclusive-bound.c17.99a8fb0a`](loop-unroll-shape.u32.an-inclusive-bound.c17.99a8fb0a.c) | type=u32, shape=an-inclusive-bound | c17 | `28` |
| [`loop-unroll-shape.u32.an-unsigned-counter.c17.be75a6a2`](loop-unroll-shape.u32.an-unsigned-counter.c17.be75a6a2.c) | type=u32, shape=an-unsigned-counter | c17 | `36` |
| [`loop-unroll-shape.u32.counting-down-inclusive.c17.26d2238b`](loop-unroll-shape.u32.counting-down-inclusive.c17.26d2238b.c) | type=u32, shape=counting-down-inclusive | c17 | `21` |
| [`loop-unroll-shape.u32.counting-down.c17.a4026ac3`](loop-unroll-shape.u32.counting-down.c17.a4026ac3.c) | type=u32, shape=counting-down | c17 | `21` |
| [`loop-unroll-shape.u32.landing-on-the-bound-exactly.c17.3f41c234`](loop-unroll-shape.u32.landing-on-the-bound-exactly.c17.3f41c234.c) | type=u32, shape=landing-on-the-bound-exactly | c17 | `12` |
| [`loop-unroll-shape.u32.no-iterations.c17.596080ce`](loop-unroll-shape.u32.no-iterations.c17.596080ce.c) | type=u32, shape=no-iterations | c17 | `99` |
| [`loop-unroll-shape.u32.one-iteration.c17.ed90f025`](loop-unroll-shape.u32.one-iteration.c17.ed90f025.c) | type=u32, shape=one-iteration | c17 | `5` |
| [`loop-unroll-shape.u32.one-past-the-copy-limit.c17.87475024`](loop-unroll-shape.u32.one-past-the-copy-limit.c17.87475024.c) | type=u32, shape=one-past-the-copy-limit | c17 | `136` |
| [`loop-unroll-shape.u32.right-at-the-copy-limit.c17.7467017c`](loop-unroll-shape.u32.right-at-the-copy-limit.c17.7467017c.c) | type=u32, shape=right-at-the-copy-limit | c17 | `120` |
| [`loop-unroll-shape.u32.stores-through-a-pointer.c17.042b158e`](loop-unroll-shape.u32.stores-through-a-pointer.c17.042b158e.c) | type=u32, shape=stores-through-a-pointer | c17 | `45` |
| [`loop-unroll-shape.u32.testing-at-the-bottom.c17.4e9f043d`](loop-unroll-shape.u32.testing-at-the-bottom.c17.4e9f043d.c) | type=u32, shape=testing-at-the-bottom | c17 | `15` |
| [`loop-unroll-shape.u64.a-branch-inside-the-body.c17.41059bc8`](loop-unroll-shape.u64.a-branch-inside-the-body.c17.41059bc8.c) | type=u64, shape=a-branch-inside-the-body | c17 | `6` |
| [`loop-unroll-shape.u64.a-call-in-the-body.c17.db449709`](loop-unroll-shape.u64.a-call-in-the-body.c17.db449709.c) | type=u64, shape=a-call-in-the-body | c17 | `5 5` |
| [`loop-unroll-shape.u64.a-counter-narrower-than-int.c17.5a3a26ca`](loop-unroll-shape.u64.a-counter-narrower-than-int.c17.5a3a26ca.c) | type=u64, shape=a-counter-narrower-than-int | c17 | `200` |
| [`loop-unroll-shape.u64.a-counter-read-after-the-loop.c17.d0947d2b`](loop-unroll-shape.u64.a-counter-read-after-the-loop.c17.d0947d2b.c) | type=u64, shape=a-counter-read-after-the-loop | c17 | `4 10` |
| [`loop-unroll-shape.u64.a-nest-both-counted.c17.ec8bde19`](loop-unroll-shape.u64.a-nest-both-counted.c17.ec8bde19.c) | type=u64, shape=a-nest-both-counted | c17 | `18` |
| [`loop-unroll-shape.u64.a-nest-the-inner-one-unknown.c17.2c1769cd`](loop-unroll-shape.u64.a-nest-the-inner-one-unknown.c17.2c1769cd.c) | type=u64, shape=a-nest-the-inner-one-unknown | c17 | `18` |
| [`loop-unroll-shape.u64.a-second-way-out.c17.df0f4cbe`](loop-unroll-shape.u64.a-second-way-out.c17.df0f4cbe.c) | type=u64, shape=a-second-way-out | c17 | `6` |
| [`loop-unroll-shape.u64.a-step-that-does-not-divide.c17.1745cac3`](loop-unroll-shape.u64.a-step-that-does-not-divide.c17.1745cac3.c) | type=u64, shape=a-step-that-does-not-divide | c17 | `18` |
| [`loop-unroll-shape.u64.an-exclusive-bound.c17.b986a410`](loop-unroll-shape.u64.an-exclusive-bound.c17.b986a410.c) | type=u64, shape=an-exclusive-bound | c17 | `21` |
| [`loop-unroll-shape.u64.an-inclusive-bound.c17.827fe869`](loop-unroll-shape.u64.an-inclusive-bound.c17.827fe869.c) | type=u64, shape=an-inclusive-bound | c17 | `28` |
| [`loop-unroll-shape.u64.an-unsigned-counter.c17.a7d53ee5`](loop-unroll-shape.u64.an-unsigned-counter.c17.a7d53ee5.c) | type=u64, shape=an-unsigned-counter | c17 | `36` |
| [`loop-unroll-shape.u64.counting-down-inclusive.c17.9e0a2cb2`](loop-unroll-shape.u64.counting-down-inclusive.c17.9e0a2cb2.c) | type=u64, shape=counting-down-inclusive | c17 | `21` |
| [`loop-unroll-shape.u64.counting-down.c17.191d43fb`](loop-unroll-shape.u64.counting-down.c17.191d43fb.c) | type=u64, shape=counting-down | c17 | `21` |
| [`loop-unroll-shape.u64.landing-on-the-bound-exactly.c17.6f1edfc8`](loop-unroll-shape.u64.landing-on-the-bound-exactly.c17.6f1edfc8.c) | type=u64, shape=landing-on-the-bound-exactly | c17 | `12` |
| [`loop-unroll-shape.u64.no-iterations.c17.11985136`](loop-unroll-shape.u64.no-iterations.c17.11985136.c) | type=u64, shape=no-iterations | c17 | `99` |
| [`loop-unroll-shape.u64.one-iteration.c17.9e1380ba`](loop-unroll-shape.u64.one-iteration.c17.9e1380ba.c) | type=u64, shape=one-iteration | c17 | `5` |
| [`loop-unroll-shape.u64.one-past-the-copy-limit.c17.ce71b162`](loop-unroll-shape.u64.one-past-the-copy-limit.c17.ce71b162.c) | type=u64, shape=one-past-the-copy-limit | c17 | `136` |
| [`loop-unroll-shape.u64.right-at-the-copy-limit.c17.738ec891`](loop-unroll-shape.u64.right-at-the-copy-limit.c17.738ec891.c) | type=u64, shape=right-at-the-copy-limit | c17 | `120` |
| [`loop-unroll-shape.u64.stores-through-a-pointer.c17.c91a4185`](loop-unroll-shape.u64.stores-through-a-pointer.c17.c91a4185.c) | type=u64, shape=stores-through-a-pointer | c17 | `45` |
| [`loop-unroll-shape.u64.testing-at-the-bottom.c17.f0b5c543`](loop-unroll-shape.u64.testing-at-the-bottom.c17.f0b5c543.c) | type=u64, shape=testing-at-the-bottom | c17 | `15` |

