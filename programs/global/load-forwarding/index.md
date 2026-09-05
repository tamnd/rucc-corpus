# load-forwarding

replacing a load with the value already in that place. Part of the global phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`load-forwarding.i32.across-branch.c17.125a63df`](load-forwarding.i32.across-branch.c17.125a63df.c) | type=i32, shape=across-branch | c17 | `41` |
| [`load-forwarding.i32.other-slot-between.c17.c0a8dc5e`](load-forwarding.i32.other-slot-between.c17.c0a8dc5e.c) | type=i32, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.i32.same-slot.c17.73db4c8a`](load-forwarding.i32.same-slot.c17.73db4c8a.c) | type=i32, shape=same-slot | c17 | `41` |
| [`load-forwarding.i32.through-pointer.c17.e4d8e947`](load-forwarding.i32.through-pointer.c17.e4d8e947.c) | type=i32, shape=through-pointer | c17 | `41` |
| [`load-forwarding.i64.across-branch.c17.f652bc9a`](load-forwarding.i64.across-branch.c17.f652bc9a.c) | type=i64, shape=across-branch | c17 | `41` |
| [`load-forwarding.i64.other-slot-between.c17.cbdec153`](load-forwarding.i64.other-slot-between.c17.cbdec153.c) | type=i64, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.i64.same-slot.c17.dc24222a`](load-forwarding.i64.same-slot.c17.dc24222a.c) | type=i64, shape=same-slot | c17 | `41` |
| [`load-forwarding.i64.through-pointer.c17.fa90a69a`](load-forwarding.i64.through-pointer.c17.fa90a69a.c) | type=i64, shape=through-pointer | c17 | `41` |
| [`load-forwarding.u32.across-branch.c17.bc2b8d54`](load-forwarding.u32.across-branch.c17.bc2b8d54.c) | type=u32, shape=across-branch | c17 | `41` |
| [`load-forwarding.u32.other-slot-between.c17.2b50cf29`](load-forwarding.u32.other-slot-between.c17.2b50cf29.c) | type=u32, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.u32.same-slot.c17.ff249032`](load-forwarding.u32.same-slot.c17.ff249032.c) | type=u32, shape=same-slot | c17 | `41` |
| [`load-forwarding.u32.through-pointer.c17.e779100c`](load-forwarding.u32.through-pointer.c17.e779100c.c) | type=u32, shape=through-pointer | c17 | `41` |
| [`load-forwarding.u64.across-branch.c17.c7f88101`](load-forwarding.u64.across-branch.c17.c7f88101.c) | type=u64, shape=across-branch | c17 | `41` |
| [`load-forwarding.u64.other-slot-between.c17.47fc1edf`](load-forwarding.u64.other-slot-between.c17.47fc1edf.c) | type=u64, shape=other-slot-between | c17 | `41` |
| [`load-forwarding.u64.same-slot.c17.054b9e2f`](load-forwarding.u64.same-slot.c17.054b9e2f.c) | type=u64, shape=same-slot | c17 | `41` |
| [`load-forwarding.u64.through-pointer.c17.82e6a750`](load-forwarding.u64.through-pointer.c17.82e6a750.c) | type=u64, shape=through-pointer | c17 | `41` |

