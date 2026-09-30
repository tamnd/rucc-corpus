# lkmm

the access and store rules the Linux kernel memory model assumes. Part of the correctness phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`lkmm.control-dependency.u16.c17.ad863e80`](lkmm.control-dependency.u16.c17.ad863e80.c) | shape=control-dependency, width=u16 | c17 | `0 10 2` |
| [`lkmm.control-dependency.u32.c17.44a8fefd`](lkmm.control-dependency.u32.c17.44a8fefd.c) | shape=control-dependency, width=u32 | c17 | `0 10 2` |
| [`lkmm.control-dependency.u64.c17.8a4a5993`](lkmm.control-dependency.u64.c17.8a4a5993.c) | shape=control-dependency, width=u64 | c17 | `0 10 2` |
| [`lkmm.control-dependency.u8.c17.120cb859`](lkmm.control-dependency.u8.c17.120cb859.c) | shape=control-dependency, width=u8 | c17 | `0 10 2` |
| [`lkmm.guarded-loop.u16.c17.804dc078`](lkmm.guarded-loop.u16.c17.804dc078.c) | shape=guarded-loop, width=u16 | c17 | `0 10 8` |
| [`lkmm.guarded-loop.u32.c17.c90a537b`](lkmm.guarded-loop.u32.c17.c90a537b.c) | shape=guarded-loop, width=u32 | c17 | `0 10 8` |
| [`lkmm.guarded-loop.u64.c17.acaf5d59`](lkmm.guarded-loop.u64.c17.acaf5d59.c) | shape=guarded-loop, width=u64 | c17 | `0 10 8` |
| [`lkmm.guarded-loop.u8.c17.f5dafe47`](lkmm.guarded-loop.u8.c17.f5dafe47.c) | shape=guarded-loop, width=u8 | c17 | `0 10 8` |
| [`lkmm.guarded-store.u16.c17.0633e272`](lkmm.guarded-store.u16.c17.0633e272.c) | shape=guarded-store, width=u16 | c17 | `0 10 8` |
| [`lkmm.guarded-store.u32.c17.c2e676f9`](lkmm.guarded-store.u32.c17.c2e676f9.c) | shape=guarded-store, width=u32 | c17 | `0 10 8` |
| [`lkmm.guarded-store.u64.c17.74a14ebc`](lkmm.guarded-store.u64.c17.74a14ebc.c) | shape=guarded-store, width=u64 | c17 | `0 10 8` |
| [`lkmm.guarded-store.u8.c17.5f1d1543`](lkmm.guarded-store.u8.c17.5f1d1543.c) | shape=guarded-store, width=u8 | c17 | `0 10 8` |
| [`lkmm.narrow-field.u16.c17.cd1f6fdb`](lkmm.narrow-field.u16.c17.cd1f6fdb.c) | shape=narrow-field, width=u16 | c17 | `0 10 36` |
| [`lkmm.narrow-field.u32.c17.6b079623`](lkmm.narrow-field.u32.c17.6b079623.c) | shape=narrow-field, width=u32 | c17 | `0 10 36` |
| [`lkmm.narrow-field.u64.c17.06a3c5bc`](lkmm.narrow-field.u64.c17.06a3c5bc.c) | shape=narrow-field, width=u64 | c17 | `0 10 36` |
| [`lkmm.narrow-field.u8.c17.005bb90c`](lkmm.narrow-field.u8.c17.005bb90c.c) | shape=narrow-field, width=u8 | c17 | `0 10 36` |
| [`lkmm.spin.u16.c17.ae1e6039`](lkmm.spin.u16.c17.ae1e6039.c) | shape=spin, width=u16 | c17 | `22` |
| [`lkmm.spin.u32.c17.48668b5c`](lkmm.spin.u32.c17.48668b5c.c) | shape=spin, width=u32 | c17 | `22` |
| [`lkmm.spin.u64.c17.3a32f527`](lkmm.spin.u64.c17.3a32f527.c) | shape=spin, width=u64 | c17 | `22` |
| [`lkmm.spin.u8.c17.ab635e58`](lkmm.spin.u8.c17.ab635e58.c) | shape=spin, width=u8 | c17 | `22` |
| [`lkmm.tearing.u16.c17.1fc1af70`](lkmm.tearing.u16.c17.1fc1af70.c) | shape=tearing, width=u16 | c17 | `0` |
| [`lkmm.tearing.u32.c17.e998a363`](lkmm.tearing.u32.c17.e998a363.c) | shape=tearing, width=u32 | c17 | `0` |
| [`lkmm.tearing.u64.c17.5deafdf1`](lkmm.tearing.u64.c17.5deafdf1.c) | shape=tearing, width=u64 | c17 | `0` |
| [`lkmm.tearing.u8.c17.d53f3f48`](lkmm.tearing.u8.c17.d53f3f48.c) | shape=tearing, width=u8 | c17 | `0` |

