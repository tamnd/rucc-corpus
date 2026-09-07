# register-pressure

how many values are live at once, and what that costs. Part of the backend phase of the M4 plan.

48 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`register-pressure.i32.12.across-a-call.c17.caaa024e`](register-pressure.i32.12.across-a-call.c17.caaa024e.c) | type=i32, live=12, shape=across-a-call | c17 | `80` |
| [`register-pressure.i32.12.both-classes.c17.1f507e37`](register-pressure.i32.12.both-classes.c17.1f507e37.c) | type=i32, live=12, shape=both-classes | c17 | `88` |
| [`register-pressure.i32.12.one-arm-only.c17.07af0c20`](register-pressure.i32.12.one-arm-only.c17.07af0c20.c) | type=i32, live=12, shape=one-arm-only | c17 | `79` |
| [`register-pressure.i32.12.through-a-loop.c17.7170c19d`](register-pressure.i32.12.through-a-loop.c17.7170c19d.c) | type=i32, live=12, shape=through-a-loop | c17 | `82` |
| [`register-pressure.i32.24.across-a-call.c17.8b0a10f0`](register-pressure.i32.24.across-a-call.c17.8b0a10f0.c) | type=i32, live=24, shape=across-a-call | c17 | `302` |
| [`register-pressure.i32.24.both-classes.c17.350ca39c`](register-pressure.i32.24.both-classes.c17.350ca39c.c) | type=i32, live=24, shape=both-classes | c17 | `310` |
| [`register-pressure.i32.24.one-arm-only.c17.7a2100e9`](register-pressure.i32.24.one-arm-only.c17.7a2100e9.c) | type=i32, live=24, shape=one-arm-only | c17 | `301` |
| [`register-pressure.i32.24.through-a-loop.c17.ab0b79aa`](register-pressure.i32.24.through-a-loop.c17.ab0b79aa.c) | type=i32, live=24, shape=through-a-loop | c17 | `304` |
| [`register-pressure.i32.6.across-a-call.c17.a474b50f`](register-pressure.i32.6.across-a-call.c17.a474b50f.c) | type=i32, live=6, shape=across-a-call | c17 | `23` |
| [`register-pressure.i32.6.both-classes.c17.742e9860`](register-pressure.i32.6.both-classes.c17.742e9860.c) | type=i32, live=6, shape=both-classes | c17 | `31` |
| [`register-pressure.i32.6.one-arm-only.c17.f5c89089`](register-pressure.i32.6.one-arm-only.c17.f5c89089.c) | type=i32, live=6, shape=one-arm-only | c17 | `22` |
| [`register-pressure.i32.6.through-a-loop.c17.5bbe07c3`](register-pressure.i32.6.through-a-loop.c17.5bbe07c3.c) | type=i32, live=6, shape=through-a-loop | c17 | `25` |
| [`register-pressure.i64.12.across-a-call.c17.62c5d8aa`](register-pressure.i64.12.across-a-call.c17.62c5d8aa.c) | type=i64, live=12, shape=across-a-call | c17 | `80` |
| [`register-pressure.i64.12.both-classes.c17.3d60a192`](register-pressure.i64.12.both-classes.c17.3d60a192.c) | type=i64, live=12, shape=both-classes | c17 | `88` |
| [`register-pressure.i64.12.one-arm-only.c17.d361e758`](register-pressure.i64.12.one-arm-only.c17.d361e758.c) | type=i64, live=12, shape=one-arm-only | c17 | `79` |
| [`register-pressure.i64.12.through-a-loop.c17.fa46d54c`](register-pressure.i64.12.through-a-loop.c17.fa46d54c.c) | type=i64, live=12, shape=through-a-loop | c17 | `82` |
| [`register-pressure.i64.24.across-a-call.c17.64bfb751`](register-pressure.i64.24.across-a-call.c17.64bfb751.c) | type=i64, live=24, shape=across-a-call | c17 | `302` |
| [`register-pressure.i64.24.both-classes.c17.e763ed59`](register-pressure.i64.24.both-classes.c17.e763ed59.c) | type=i64, live=24, shape=both-classes | c17 | `310` |
| [`register-pressure.i64.24.one-arm-only.c17.858659b1`](register-pressure.i64.24.one-arm-only.c17.858659b1.c) | type=i64, live=24, shape=one-arm-only | c17 | `301` |
| [`register-pressure.i64.24.through-a-loop.c17.e4e96a97`](register-pressure.i64.24.through-a-loop.c17.e4e96a97.c) | type=i64, live=24, shape=through-a-loop | c17 | `304` |
| [`register-pressure.i64.6.across-a-call.c17.70f08b42`](register-pressure.i64.6.across-a-call.c17.70f08b42.c) | type=i64, live=6, shape=across-a-call | c17 | `23` |
| [`register-pressure.i64.6.both-classes.c17.789cb301`](register-pressure.i64.6.both-classes.c17.789cb301.c) | type=i64, live=6, shape=both-classes | c17 | `31` |
| [`register-pressure.i64.6.one-arm-only.c17.a39a6993`](register-pressure.i64.6.one-arm-only.c17.a39a6993.c) | type=i64, live=6, shape=one-arm-only | c17 | `22` |
| [`register-pressure.i64.6.through-a-loop.c17.73ec9fe6`](register-pressure.i64.6.through-a-loop.c17.73ec9fe6.c) | type=i64, live=6, shape=through-a-loop | c17 | `25` |
| [`register-pressure.u32.12.across-a-call.c17.69f3951c`](register-pressure.u32.12.across-a-call.c17.69f3951c.c) | type=u32, live=12, shape=across-a-call | c17 | `80` |
| [`register-pressure.u32.12.both-classes.c17.fc92b97f`](register-pressure.u32.12.both-classes.c17.fc92b97f.c) | type=u32, live=12, shape=both-classes | c17 | `88` |
| [`register-pressure.u32.12.one-arm-only.c17.8e773622`](register-pressure.u32.12.one-arm-only.c17.8e773622.c) | type=u32, live=12, shape=one-arm-only | c17 | `79` |
| [`register-pressure.u32.12.through-a-loop.c17.b58b0fda`](register-pressure.u32.12.through-a-loop.c17.b58b0fda.c) | type=u32, live=12, shape=through-a-loop | c17 | `82` |
| [`register-pressure.u32.24.across-a-call.c17.43055abb`](register-pressure.u32.24.across-a-call.c17.43055abb.c) | type=u32, live=24, shape=across-a-call | c17 | `302` |
| [`register-pressure.u32.24.both-classes.c17.6d503e5a`](register-pressure.u32.24.both-classes.c17.6d503e5a.c) | type=u32, live=24, shape=both-classes | c17 | `310` |
| [`register-pressure.u32.24.one-arm-only.c17.eaf52679`](register-pressure.u32.24.one-arm-only.c17.eaf52679.c) | type=u32, live=24, shape=one-arm-only | c17 | `301` |
| [`register-pressure.u32.24.through-a-loop.c17.98f0bc3e`](register-pressure.u32.24.through-a-loop.c17.98f0bc3e.c) | type=u32, live=24, shape=through-a-loop | c17 | `304` |
| [`register-pressure.u32.6.across-a-call.c17.eff82b4f`](register-pressure.u32.6.across-a-call.c17.eff82b4f.c) | type=u32, live=6, shape=across-a-call | c17 | `23` |
| [`register-pressure.u32.6.both-classes.c17.c85c303f`](register-pressure.u32.6.both-classes.c17.c85c303f.c) | type=u32, live=6, shape=both-classes | c17 | `31` |
| [`register-pressure.u32.6.one-arm-only.c17.52f1a6b1`](register-pressure.u32.6.one-arm-only.c17.52f1a6b1.c) | type=u32, live=6, shape=one-arm-only | c17 | `22` |
| [`register-pressure.u32.6.through-a-loop.c17.48fb7b57`](register-pressure.u32.6.through-a-loop.c17.48fb7b57.c) | type=u32, live=6, shape=through-a-loop | c17 | `25` |
| [`register-pressure.u64.12.across-a-call.c17.bb64b9f0`](register-pressure.u64.12.across-a-call.c17.bb64b9f0.c) | type=u64, live=12, shape=across-a-call | c17 | `80` |
| [`register-pressure.u64.12.both-classes.c17.37791435`](register-pressure.u64.12.both-classes.c17.37791435.c) | type=u64, live=12, shape=both-classes | c17 | `88` |
| [`register-pressure.u64.12.one-arm-only.c17.f846cf6f`](register-pressure.u64.12.one-arm-only.c17.f846cf6f.c) | type=u64, live=12, shape=one-arm-only | c17 | `79` |
| [`register-pressure.u64.12.through-a-loop.c17.f729e12a`](register-pressure.u64.12.through-a-loop.c17.f729e12a.c) | type=u64, live=12, shape=through-a-loop | c17 | `82` |
| [`register-pressure.u64.24.across-a-call.c17.e67c4a45`](register-pressure.u64.24.across-a-call.c17.e67c4a45.c) | type=u64, live=24, shape=across-a-call | c17 | `302` |
| [`register-pressure.u64.24.both-classes.c17.c9171d96`](register-pressure.u64.24.both-classes.c17.c9171d96.c) | type=u64, live=24, shape=both-classes | c17 | `310` |
| [`register-pressure.u64.24.one-arm-only.c17.b6a2fb86`](register-pressure.u64.24.one-arm-only.c17.b6a2fb86.c) | type=u64, live=24, shape=one-arm-only | c17 | `301` |
| [`register-pressure.u64.24.through-a-loop.c17.6003de8f`](register-pressure.u64.24.through-a-loop.c17.6003de8f.c) | type=u64, live=24, shape=through-a-loop | c17 | `304` |
| [`register-pressure.u64.6.across-a-call.c17.f4db9de6`](register-pressure.u64.6.across-a-call.c17.f4db9de6.c) | type=u64, live=6, shape=across-a-call | c17 | `23` |
| [`register-pressure.u64.6.both-classes.c17.af33b1d8`](register-pressure.u64.6.both-classes.c17.af33b1d8.c) | type=u64, live=6, shape=both-classes | c17 | `31` |
| [`register-pressure.u64.6.one-arm-only.c17.e60c2655`](register-pressure.u64.6.one-arm-only.c17.e60c2655.c) | type=u64, live=6, shape=one-arm-only | c17 | `22` |
| [`register-pressure.u64.6.through-a-loop.c17.f458eff3`](register-pressure.u64.6.through-a-loop.c17.f458eff3.c) | type=u64, live=6, shape=through-a-loop | c17 | `25` |

