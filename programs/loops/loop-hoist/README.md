# loop-hoist

whether an invariant computation is allowed out of its loop. Part of the loops phase of the M4 plan.

64 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`loop-hoist.f32.14-live-under-pressure.c17.d9eb990c`](loop-hoist.f32.14-live-under-pressure.c17.d9eb990c.c) | type=f32, shape=14-live-under-pressure | c17 | `1680` |
| [`loop-hoist.f32.4-live-under-pressure.c17.2f5b92fd`](loop-hoist.f32.4-live-under-pressure.c17.2f5b92fd.c) | type=f32, shape=4-live-under-pressure | c17 | `320` |
| [`loop-hoist.f32.6-live-under-pressure.c17.34af9df4`](loop-hoist.f32.6-live-under-pressure.c17.34af9df4.c) | type=f32, shape=6-live-under-pressure | c17 | `528` |
| [`loop-hoist.f32.7-live-under-pressure.c17.f90359cb`](loop-hoist.f32.7-live-under-pressure.c17.f90359cb.c) | type=f32, shape=7-live-under-pressure | c17 | `644` |
| [`loop-hoist.f64.14-live-under-pressure.c17.5c00e919`](loop-hoist.f64.14-live-under-pressure.c17.5c00e919.c) | type=f64, shape=14-live-under-pressure | c17 | `1680` |
| [`loop-hoist.f64.4-live-under-pressure.c17.3120e46d`](loop-hoist.f64.4-live-under-pressure.c17.3120e46d.c) | type=f64, shape=4-live-under-pressure | c17 | `320` |
| [`loop-hoist.f64.6-live-under-pressure.c17.130c3d26`](loop-hoist.f64.6-live-under-pressure.c17.130c3d26.c) | type=f64, shape=6-live-under-pressure | c17 | `528` |
| [`loop-hoist.f64.7-live-under-pressure.c17.473b1f13`](loop-hoist.f64.7-live-under-pressure.c17.473b1f13.c) | type=f64, shape=7-live-under-pressure | c17 | `644` |
| [`loop-hoist.i32.a-call-with-nothing-in-it.c17.b01c8fd2`](loop-hoist.i32.a-call-with-nothing-in-it.c17.b01c8fd2.c) | type=i32, shape=a-call-with-nothing-in-it | c17 | `100` |
| [`loop-hoist.i32.a-chain-of-three.c17.53aa2571`](loop-hoist.i32.a-chain-of-three.c17.53aa2571.c) | type=i32, shape=a-chain-of-three | c17 | `1116` |
| [`loop-hoist.i32.a-store-of-something-invariant.c17.bfcd2d20`](loop-hoist.i32.a-store-of-something-invariant.c17.bfcd2d20.c) | type=i32, shape=a-store-of-something-invariant | c17 | `8 28` |
| [`loop-hoist.i32.an-array-element.c17.f863688a`](loop-hoist.i32.an-array-element.c17.f863688a.c) | type=i32, shape=an-array-element | c17 | `100` |
| [`loop-hoist.i32.divide-at-the-bottom.c17.a3b200ba`](loop-hoist.i32.divide-at-the-bottom.c17.a3b200ba.c) | type=i32, shape=divide-at-the-bottom | c17 | `196` |
| [`loop-hoist.i32.divide-that-may-not-run.c17.dacb7403`](loop-hoist.i32.divide-that-may-not-run.c17.dacb7403.c) | type=i32, shape=divide-that-may-not-run | c17 | `0` |
| [`loop-hoist.i32.divide-under-a-test.c17.f8055fd2`](loop-hoist.i32.divide-under-a-test.c17.f8055fd2.c) | type=i32, shape=divide-under-a-test | c17 | `28` |
| [`loop-hoist.i32.global-load-written.c17.3a80805f`](loop-hoist.i32.global-load-written.c17.3a80805f.c) | type=i32, shape=global-load-written | c17 | `52` |
| [`loop-hoist.i32.global-load.c17.d14cefca`](loop-hoist.i32.global-load.c17.d14cefca.c) | type=i32, shape=global-load | c17 | `52` |
| [`loop-hoist.i32.load-that-always-runs.c17.b216829b`](loop-hoist.i32.load-that-always-runs.c17.b216829b.c) | type=i32, shape=load-that-always-runs | c17 | `356` |
| [`loop-hoist.i32.load-that-may-not-run.c17.7b3650d4`](loop-hoist.i32.load-that-may-not-run.c17.7b3650d4.c) | type=i32, shape=load-that-may-not-run | c17 | `0` |
| [`loop-hoist.i32.out-of-a-nest.c17.a53623de`](loop-hoist.i32.out-of-a-nest.c17.a53623de.c) | type=i32, shape=out-of-a-nest | c17 | `288` |
| [`loop-hoist.i32.under-pressure.c17.da9a1a8e`](loop-hoist.i32.under-pressure.c17.da9a1a8e.c) | type=i32, shape=under-pressure | c17 | `536` |
| [`loop-hoist.i32.volatile-load.c17.9675fbfa`](loop-hoist.i32.volatile-load.c17.9675fbfa.c) | type=i32, shape=volatile-load | c17 | `68` |
| [`loop-hoist.i64.a-call-with-nothing-in-it.c17.787c330a`](loop-hoist.i64.a-call-with-nothing-in-it.c17.787c330a.c) | type=i64, shape=a-call-with-nothing-in-it | c17 | `100` |
| [`loop-hoist.i64.a-chain-of-three.c17.f16a6966`](loop-hoist.i64.a-chain-of-three.c17.f16a6966.c) | type=i64, shape=a-chain-of-three | c17 | `1116` |
| [`loop-hoist.i64.a-store-of-something-invariant.c17.93493f48`](loop-hoist.i64.a-store-of-something-invariant.c17.93493f48.c) | type=i64, shape=a-store-of-something-invariant | c17 | `8 28` |
| [`loop-hoist.i64.an-array-element.c17.d0752ad5`](loop-hoist.i64.an-array-element.c17.d0752ad5.c) | type=i64, shape=an-array-element | c17 | `100` |
| [`loop-hoist.i64.divide-at-the-bottom.c17.50c8cfca`](loop-hoist.i64.divide-at-the-bottom.c17.50c8cfca.c) | type=i64, shape=divide-at-the-bottom | c17 | `196` |
| [`loop-hoist.i64.divide-that-may-not-run.c17.deb59636`](loop-hoist.i64.divide-that-may-not-run.c17.deb59636.c) | type=i64, shape=divide-that-may-not-run | c17 | `0` |
| [`loop-hoist.i64.divide-under-a-test.c17.91e877d9`](loop-hoist.i64.divide-under-a-test.c17.91e877d9.c) | type=i64, shape=divide-under-a-test | c17 | `28` |
| [`loop-hoist.i64.global-load-written.c17.1b9e5d4f`](loop-hoist.i64.global-load-written.c17.1b9e5d4f.c) | type=i64, shape=global-load-written | c17 | `52` |
| [`loop-hoist.i64.global-load.c17.83bf7dff`](loop-hoist.i64.global-load.c17.83bf7dff.c) | type=i64, shape=global-load | c17 | `52` |
| [`loop-hoist.i64.load-that-always-runs.c17.c783bf02`](loop-hoist.i64.load-that-always-runs.c17.c783bf02.c) | type=i64, shape=load-that-always-runs | c17 | `356` |
| [`loop-hoist.i64.load-that-may-not-run.c17.f265afe5`](loop-hoist.i64.load-that-may-not-run.c17.f265afe5.c) | type=i64, shape=load-that-may-not-run | c17 | `0` |
| [`loop-hoist.i64.out-of-a-nest.c17.1e5cdf1e`](loop-hoist.i64.out-of-a-nest.c17.1e5cdf1e.c) | type=i64, shape=out-of-a-nest | c17 | `288` |
| [`loop-hoist.i64.under-pressure.c17.b4993329`](loop-hoist.i64.under-pressure.c17.b4993329.c) | type=i64, shape=under-pressure | c17 | `536` |
| [`loop-hoist.i64.volatile-load.c17.893c3c3f`](loop-hoist.i64.volatile-load.c17.893c3c3f.c) | type=i64, shape=volatile-load | c17 | `68` |
| [`loop-hoist.u32.a-call-with-nothing-in-it.c17.71391307`](loop-hoist.u32.a-call-with-nothing-in-it.c17.71391307.c) | type=u32, shape=a-call-with-nothing-in-it | c17 | `100` |
| [`loop-hoist.u32.a-chain-of-three.c17.7d49863c`](loop-hoist.u32.a-chain-of-three.c17.7d49863c.c) | type=u32, shape=a-chain-of-three | c17 | `1116` |
| [`loop-hoist.u32.a-store-of-something-invariant.c17.f882112f`](loop-hoist.u32.a-store-of-something-invariant.c17.f882112f.c) | type=u32, shape=a-store-of-something-invariant | c17 | `8 28` |
| [`loop-hoist.u32.an-array-element.c17.3d541be5`](loop-hoist.u32.an-array-element.c17.3d541be5.c) | type=u32, shape=an-array-element | c17 | `100` |
| [`loop-hoist.u32.divide-at-the-bottom.c17.a64ff3bb`](loop-hoist.u32.divide-at-the-bottom.c17.a64ff3bb.c) | type=u32, shape=divide-at-the-bottom | c17 | `196` |
| [`loop-hoist.u32.divide-that-may-not-run.c17.4352da1e`](loop-hoist.u32.divide-that-may-not-run.c17.4352da1e.c) | type=u32, shape=divide-that-may-not-run | c17 | `0` |
| [`loop-hoist.u32.divide-under-a-test.c17.ebed2f7d`](loop-hoist.u32.divide-under-a-test.c17.ebed2f7d.c) | type=u32, shape=divide-under-a-test | c17 | `28` |
| [`loop-hoist.u32.global-load-written.c17.557430b6`](loop-hoist.u32.global-load-written.c17.557430b6.c) | type=u32, shape=global-load-written | c17 | `52` |
| [`loop-hoist.u32.global-load.c17.e4a01f6f`](loop-hoist.u32.global-load.c17.e4a01f6f.c) | type=u32, shape=global-load | c17 | `52` |
| [`loop-hoist.u32.load-that-always-runs.c17.44c4a145`](loop-hoist.u32.load-that-always-runs.c17.44c4a145.c) | type=u32, shape=load-that-always-runs | c17 | `356` |
| [`loop-hoist.u32.load-that-may-not-run.c17.cca01a15`](loop-hoist.u32.load-that-may-not-run.c17.cca01a15.c) | type=u32, shape=load-that-may-not-run | c17 | `0` |
| [`loop-hoist.u32.out-of-a-nest.c17.3f0a81d8`](loop-hoist.u32.out-of-a-nest.c17.3f0a81d8.c) | type=u32, shape=out-of-a-nest | c17 | `288` |
| [`loop-hoist.u32.under-pressure.c17.4c21dc6f`](loop-hoist.u32.under-pressure.c17.4c21dc6f.c) | type=u32, shape=under-pressure | c17 | `536` |
| [`loop-hoist.u32.volatile-load.c17.f1ca78f5`](loop-hoist.u32.volatile-load.c17.f1ca78f5.c) | type=u32, shape=volatile-load | c17 | `68` |
| [`loop-hoist.u64.a-call-with-nothing-in-it.c17.f867fca9`](loop-hoist.u64.a-call-with-nothing-in-it.c17.f867fca9.c) | type=u64, shape=a-call-with-nothing-in-it | c17 | `100` |
| [`loop-hoist.u64.a-chain-of-three.c17.07f9da25`](loop-hoist.u64.a-chain-of-three.c17.07f9da25.c) | type=u64, shape=a-chain-of-three | c17 | `1116` |
| [`loop-hoist.u64.a-store-of-something-invariant.c17.17d0e4dd`](loop-hoist.u64.a-store-of-something-invariant.c17.17d0e4dd.c) | type=u64, shape=a-store-of-something-invariant | c17 | `8 28` |
| [`loop-hoist.u64.an-array-element.c17.e6ab3e9b`](loop-hoist.u64.an-array-element.c17.e6ab3e9b.c) | type=u64, shape=an-array-element | c17 | `100` |
| [`loop-hoist.u64.divide-at-the-bottom.c17.5b9b466a`](loop-hoist.u64.divide-at-the-bottom.c17.5b9b466a.c) | type=u64, shape=divide-at-the-bottom | c17 | `196` |
| [`loop-hoist.u64.divide-that-may-not-run.c17.da0d064e`](loop-hoist.u64.divide-that-may-not-run.c17.da0d064e.c) | type=u64, shape=divide-that-may-not-run | c17 | `0` |
| [`loop-hoist.u64.divide-under-a-test.c17.a11e74b0`](loop-hoist.u64.divide-under-a-test.c17.a11e74b0.c) | type=u64, shape=divide-under-a-test | c17 | `28` |
| [`loop-hoist.u64.global-load-written.c17.57f6e3ec`](loop-hoist.u64.global-load-written.c17.57f6e3ec.c) | type=u64, shape=global-load-written | c17 | `52` |
| [`loop-hoist.u64.global-load.c17.575a4614`](loop-hoist.u64.global-load.c17.575a4614.c) | type=u64, shape=global-load | c17 | `52` |
| [`loop-hoist.u64.load-that-always-runs.c17.668247cd`](loop-hoist.u64.load-that-always-runs.c17.668247cd.c) | type=u64, shape=load-that-always-runs | c17 | `356` |
| [`loop-hoist.u64.load-that-may-not-run.c17.4fe51275`](loop-hoist.u64.load-that-may-not-run.c17.4fe51275.c) | type=u64, shape=load-that-may-not-run | c17 | `0` |
| [`loop-hoist.u64.out-of-a-nest.c17.a0d58ad1`](loop-hoist.u64.out-of-a-nest.c17.a0d58ad1.c) | type=u64, shape=out-of-a-nest | c17 | `288` |
| [`loop-hoist.u64.under-pressure.c17.068bf5db`](loop-hoist.u64.under-pressure.c17.068bf5db.c) | type=u64, shape=under-pressure | c17 | `536` |
| [`loop-hoist.u64.volatile-load.c17.b3785699`](loop-hoist.u64.volatile-load.c17.b3785699.c) | type=u64, shape=volatile-load | c17 | `68` |

