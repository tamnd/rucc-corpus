# load-forwarding

replacing a load with the value already in that place. Part of the global phase of the M4 plan.

40 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`load-forwarding.i32.across-branch.c17.125a63df`](load-forwarding.i32.across-branch.c17.125a63df.c) | type=i32, shape=across-branch | c17 | `41` |
| [`load-forwarding.i32.across-opaque-call.c17.6b4a3786`](load-forwarding.i32.across-opaque-call.c17.6b4a3786.c) | type=i32, shape=across-opaque-call | c17 | `41` |
| [`load-forwarding.i32.across-reading-call.c17.ef5eb9ac`](load-forwarding.i32.across-reading-call.c17.ef5eb9ac.c) | type=i32, shape=across-reading-call | c17 | `41` |
| [`load-forwarding.i32.copied-over.c17.e0fcf64d`](load-forwarding.i32.copied-over.c17.e0fcf64d.c) | type=i32, shape=copied-over | c17 | `41` |
| [`load-forwarding.i32.long-chain.c17.a095137d`](load-forwarding.i32.long-chain.c17.a095137d.c) | type=i32, shape=long-chain | c17 | `41` |
| [`load-forwarding.i32.other-slot-between.c17.c0a8dc5e`](load-forwarding.i32.other-slot-between.c17.c0a8dc5e.c) | type=i32, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.i32.phi-translation.c17.7cdf5aa7`](load-forwarding.i32.phi-translation.c17.7cdf5aa7.c) | type=i32, shape=phi-translation | c17 | `41` |
| [`load-forwarding.i32.same-slot.c17.73db4c8a`](load-forwarding.i32.same-slot.c17.73db4c8a.c) | type=i32, shape=same-slot | c17 | `41` |
| [`load-forwarding.i32.through-pointer.c17.e4d8e947`](load-forwarding.i32.through-pointer.c17.e4d8e947.c) | type=i32, shape=through-pointer | c17 | `41` |
| [`load-forwarding.i32.volatile-store.c17.18ffa92a`](load-forwarding.i32.volatile-store.c17.18ffa92a.c) | type=i32, shape=volatile-store | c17 | `41` |
| [`load-forwarding.i64.across-branch.c17.f652bc9a`](load-forwarding.i64.across-branch.c17.f652bc9a.c) | type=i64, shape=across-branch | c17 | `41` |
| [`load-forwarding.i64.across-opaque-call.c17.c88fea48`](load-forwarding.i64.across-opaque-call.c17.c88fea48.c) | type=i64, shape=across-opaque-call | c17 | `41` |
| [`load-forwarding.i64.across-reading-call.c17.b8610027`](load-forwarding.i64.across-reading-call.c17.b8610027.c) | type=i64, shape=across-reading-call | c17 | `41` |
| [`load-forwarding.i64.copied-over.c17.38b176a4`](load-forwarding.i64.copied-over.c17.38b176a4.c) | type=i64, shape=copied-over | c17 | `41` |
| [`load-forwarding.i64.long-chain.c17.96fc26fd`](load-forwarding.i64.long-chain.c17.96fc26fd.c) | type=i64, shape=long-chain | c17 | `41` |
| [`load-forwarding.i64.other-slot-between.c17.cbdec153`](load-forwarding.i64.other-slot-between.c17.cbdec153.c) | type=i64, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.i64.phi-translation.c17.8b44e7cb`](load-forwarding.i64.phi-translation.c17.8b44e7cb.c) | type=i64, shape=phi-translation | c17 | `41` |
| [`load-forwarding.i64.same-slot.c17.dc24222a`](load-forwarding.i64.same-slot.c17.dc24222a.c) | type=i64, shape=same-slot | c17 | `41` |
| [`load-forwarding.i64.through-pointer.c17.fa90a69a`](load-forwarding.i64.through-pointer.c17.fa90a69a.c) | type=i64, shape=through-pointer | c17 | `41` |
| [`load-forwarding.i64.volatile-store.c17.9b1ce9ce`](load-forwarding.i64.volatile-store.c17.9b1ce9ce.c) | type=i64, shape=volatile-store | c17 | `41` |
| [`load-forwarding.u32.across-branch.c17.bc2b8d54`](load-forwarding.u32.across-branch.c17.bc2b8d54.c) | type=u32, shape=across-branch | c17 | `41` |
| [`load-forwarding.u32.across-opaque-call.c17.5958a7c5`](load-forwarding.u32.across-opaque-call.c17.5958a7c5.c) | type=u32, shape=across-opaque-call | c17 | `41` |
| [`load-forwarding.u32.across-reading-call.c17.2b17af57`](load-forwarding.u32.across-reading-call.c17.2b17af57.c) | type=u32, shape=across-reading-call | c17 | `41` |
| [`load-forwarding.u32.copied-over.c17.158ec6ec`](load-forwarding.u32.copied-over.c17.158ec6ec.c) | type=u32, shape=copied-over | c17 | `41` |
| [`load-forwarding.u32.long-chain.c17.31198196`](load-forwarding.u32.long-chain.c17.31198196.c) | type=u32, shape=long-chain | c17 | `41` |
| [`load-forwarding.u32.other-slot-between.c17.2b50cf29`](load-forwarding.u32.other-slot-between.c17.2b50cf29.c) | type=u32, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.u32.phi-translation.c17.ef38d87d`](load-forwarding.u32.phi-translation.c17.ef38d87d.c) | type=u32, shape=phi-translation | c17 | `41` |
| [`load-forwarding.u32.same-slot.c17.ff249032`](load-forwarding.u32.same-slot.c17.ff249032.c) | type=u32, shape=same-slot | c17 | `41` |
| [`load-forwarding.u32.through-pointer.c17.e779100c`](load-forwarding.u32.through-pointer.c17.e779100c.c) | type=u32, shape=through-pointer | c17 | `41` |
| [`load-forwarding.u32.volatile-store.c17.073cad35`](load-forwarding.u32.volatile-store.c17.073cad35.c) | type=u32, shape=volatile-store | c17 | `41` |
| [`load-forwarding.u64.across-branch.c17.c7f88101`](load-forwarding.u64.across-branch.c17.c7f88101.c) | type=u64, shape=across-branch | c17 | `41` |
| [`load-forwarding.u64.across-opaque-call.c17.f98d7aa6`](load-forwarding.u64.across-opaque-call.c17.f98d7aa6.c) | type=u64, shape=across-opaque-call | c17 | `41` |
| [`load-forwarding.u64.across-reading-call.c17.639d1167`](load-forwarding.u64.across-reading-call.c17.639d1167.c) | type=u64, shape=across-reading-call | c17 | `41` |
| [`load-forwarding.u64.copied-over.c17.04d71d0b`](load-forwarding.u64.copied-over.c17.04d71d0b.c) | type=u64, shape=copied-over | c17 | `41` |
| [`load-forwarding.u64.long-chain.c17.ee7f67a0`](load-forwarding.u64.long-chain.c17.ee7f67a0.c) | type=u64, shape=long-chain | c17 | `41` |
| [`load-forwarding.u64.other-slot-between.c17.47fc1edf`](load-forwarding.u64.other-slot-between.c17.47fc1edf.c) | type=u64, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.u64.phi-translation.c17.aee2513c`](load-forwarding.u64.phi-translation.c17.aee2513c.c) | type=u64, shape=phi-translation | c17 | `41` |
| [`load-forwarding.u64.same-slot.c17.054b9e2f`](load-forwarding.u64.same-slot.c17.054b9e2f.c) | type=u64, shape=same-slot | c17 | `41` |
| [`load-forwarding.u64.through-pointer.c17.82e6a750`](load-forwarding.u64.through-pointer.c17.82e6a750.c) | type=u64, shape=through-pointer | c17 | `41` |
| [`load-forwarding.u64.volatile-store.c17.bee8690f`](load-forwarding.u64.volatile-store.c17.bee8690f.c) | type=u64, shape=volatile-store | c17 | `41` |

