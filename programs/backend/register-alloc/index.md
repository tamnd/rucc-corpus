# register-alloc

assigning registers and deciding what to spill. Part of the backend phase of the M4 plan.

40 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`register-alloc.i32.16.in-a-loop.c17.28a5ec1c`](register-alloc.i32.16.in-a-loop.c17.28a5ec1c.c) | type=i32, live=16, shape=in-a-loop | c17 | `544` |
| [`register-alloc.i32.16.straight.c17.1a7fa466`](register-alloc.i32.16.straight.c17.1a7fa466.c) | type=i32, live=16, shape=straight | c17 | `136` |
| [`register-alloc.i32.32.in-a-loop.c17.2d768b88`](register-alloc.i32.32.in-a-loop.c17.2d768b88.c) | type=i32, live=32, shape=in-a-loop | c17 | `2112` |
| [`register-alloc.i32.32.straight.c17.401521de`](register-alloc.i32.32.straight.c17.401521de.c) | type=i32, live=32, shape=straight | c17 | `528` |
| [`register-alloc.i32.4.in-a-loop.c17.ade8e14a`](register-alloc.i32.4.in-a-loop.c17.ade8e14a.c) | type=i32, live=4, shape=in-a-loop | c17 | `40` |
| [`register-alloc.i32.4.straight.c17.068ada7a`](register-alloc.i32.4.straight.c17.068ada7a.c) | type=i32, live=4, shape=straight | c17 | `10` |
| [`register-alloc.i32.64.in-a-loop.c17.39617d16`](register-alloc.i32.64.in-a-loop.c17.39617d16.c) | type=i32, live=64, shape=in-a-loop | c17 | `8320` |
| [`register-alloc.i32.64.straight.c17.3eab4e38`](register-alloc.i32.64.straight.c17.3eab4e38.c) | type=i32, live=64, shape=straight | c17 | `2080` |
| [`register-alloc.i32.8.in-a-loop.c17.c218cd5a`](register-alloc.i32.8.in-a-loop.c17.c218cd5a.c) | type=i32, live=8, shape=in-a-loop | c17 | `144` |
| [`register-alloc.i32.8.straight.c17.c1fa4594`](register-alloc.i32.8.straight.c17.c1fa4594.c) | type=i32, live=8, shape=straight | c17 | `36` |
| [`register-alloc.i64.16.in-a-loop.c17.487da4c3`](register-alloc.i64.16.in-a-loop.c17.487da4c3.c) | type=i64, live=16, shape=in-a-loop | c17 | `544` |
| [`register-alloc.i64.16.straight.c17.04b299f2`](register-alloc.i64.16.straight.c17.04b299f2.c) | type=i64, live=16, shape=straight | c17 | `136` |
| [`register-alloc.i64.32.in-a-loop.c17.a3302269`](register-alloc.i64.32.in-a-loop.c17.a3302269.c) | type=i64, live=32, shape=in-a-loop | c17 | `2112` |
| [`register-alloc.i64.32.straight.c17.3d0dd035`](register-alloc.i64.32.straight.c17.3d0dd035.c) | type=i64, live=32, shape=straight | c17 | `528` |
| [`register-alloc.i64.4.in-a-loop.c17.4dfad697`](register-alloc.i64.4.in-a-loop.c17.4dfad697.c) | type=i64, live=4, shape=in-a-loop | c17 | `40` |
| [`register-alloc.i64.4.straight.c17.4dd11056`](register-alloc.i64.4.straight.c17.4dd11056.c) | type=i64, live=4, shape=straight | c17 | `10` |
| [`register-alloc.i64.64.in-a-loop.c17.1260669b`](register-alloc.i64.64.in-a-loop.c17.1260669b.c) | type=i64, live=64, shape=in-a-loop | c17 | `8320` |
| [`register-alloc.i64.64.straight.c17.93b036de`](register-alloc.i64.64.straight.c17.93b036de.c) | type=i64, live=64, shape=straight | c17 | `2080` |
| [`register-alloc.i64.8.in-a-loop.c17.a7204eff`](register-alloc.i64.8.in-a-loop.c17.a7204eff.c) | type=i64, live=8, shape=in-a-loop | c17 | `144` |
| [`register-alloc.i64.8.straight.c17.51a3d64e`](register-alloc.i64.8.straight.c17.51a3d64e.c) | type=i64, live=8, shape=straight | c17 | `36` |
| [`register-alloc.u32.16.in-a-loop.c17.27fb2c21`](register-alloc.u32.16.in-a-loop.c17.27fb2c21.c) | type=u32, live=16, shape=in-a-loop | c17 | `544` |
| [`register-alloc.u32.16.straight.c17.0fa2fadf`](register-alloc.u32.16.straight.c17.0fa2fadf.c) | type=u32, live=16, shape=straight | c17 | `136` |
| [`register-alloc.u32.32.in-a-loop.c17.350c6536`](register-alloc.u32.32.in-a-loop.c17.350c6536.c) | type=u32, live=32, shape=in-a-loop | c17 | `2112` |
| [`register-alloc.u32.32.straight.c17.d9064c91`](register-alloc.u32.32.straight.c17.d9064c91.c) | type=u32, live=32, shape=straight | c17 | `528` |
| [`register-alloc.u32.4.in-a-loop.c17.fc614f73`](register-alloc.u32.4.in-a-loop.c17.fc614f73.c) | type=u32, live=4, shape=in-a-loop | c17 | `40` |
| [`register-alloc.u32.4.straight.c17.f18a367e`](register-alloc.u32.4.straight.c17.f18a367e.c) | type=u32, live=4, shape=straight | c17 | `10` |
| [`register-alloc.u32.64.in-a-loop.c17.1536cb94`](register-alloc.u32.64.in-a-loop.c17.1536cb94.c) | type=u32, live=64, shape=in-a-loop | c17 | `8320` |
| [`register-alloc.u32.64.straight.c17.d6e03c2d`](register-alloc.u32.64.straight.c17.d6e03c2d.c) | type=u32, live=64, shape=straight | c17 | `2080` |
| [`register-alloc.u32.8.in-a-loop.c17.e1dcec80`](register-alloc.u32.8.in-a-loop.c17.e1dcec80.c) | type=u32, live=8, shape=in-a-loop | c17 | `144` |
| [`register-alloc.u32.8.straight.c17.beb26766`](register-alloc.u32.8.straight.c17.beb26766.c) | type=u32, live=8, shape=straight | c17 | `36` |
| [`register-alloc.u64.16.in-a-loop.c17.1307f1bf`](register-alloc.u64.16.in-a-loop.c17.1307f1bf.c) | type=u64, live=16, shape=in-a-loop | c17 | `544` |
| [`register-alloc.u64.16.straight.c17.cda7e563`](register-alloc.u64.16.straight.c17.cda7e563.c) | type=u64, live=16, shape=straight | c17 | `136` |
| [`register-alloc.u64.32.in-a-loop.c17.79992ebf`](register-alloc.u64.32.in-a-loop.c17.79992ebf.c) | type=u64, live=32, shape=in-a-loop | c17 | `2112` |
| [`register-alloc.u64.32.straight.c17.aa6fbb51`](register-alloc.u64.32.straight.c17.aa6fbb51.c) | type=u64, live=32, shape=straight | c17 | `528` |
| [`register-alloc.u64.4.in-a-loop.c17.f26544e9`](register-alloc.u64.4.in-a-loop.c17.f26544e9.c) | type=u64, live=4, shape=in-a-loop | c17 | `40` |
| [`register-alloc.u64.4.straight.c17.af493c80`](register-alloc.u64.4.straight.c17.af493c80.c) | type=u64, live=4, shape=straight | c17 | `10` |
| [`register-alloc.u64.64.in-a-loop.c17.6d77267f`](register-alloc.u64.64.in-a-loop.c17.6d77267f.c) | type=u64, live=64, shape=in-a-loop | c17 | `8320` |
| [`register-alloc.u64.64.straight.c17.03bc7a75`](register-alloc.u64.64.straight.c17.03bc7a75.c) | type=u64, live=64, shape=straight | c17 | `2080` |
| [`register-alloc.u64.8.in-a-loop.c17.2437eb98`](register-alloc.u64.8.in-a-loop.c17.2437eb98.c) | type=u64, live=8, shape=in-a-loop | c17 | `144` |
| [`register-alloc.u64.8.straight.c17.381ca586`](register-alloc.u64.8.straight.c17.381ca586.c) | type=u64, live=8, shape=straight | c17 | `36` |

