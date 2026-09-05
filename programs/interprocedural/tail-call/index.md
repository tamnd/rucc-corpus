# tail-call

turning a call in tail position into a jump. Part of the interprocedural phase of the M4 plan.

36 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`tail-call.i32.1000.forward.c17.ffe28547`](tail-call.i32.1000.forward.c17.ffe28547.c) | type=i32, depth=1000, shape=forward | c17 | `1000` |
| [`tail-call.i32.1000.mutual.c17.a0deb2c3`](tail-call.i32.1000.mutual.c17.a0deb2c3.c) | type=i32, depth=1000, shape=mutual | c17 | `1000` |
| [`tail-call.i32.1000.self.c17.c73d94ab`](tail-call.i32.1000.self.c17.c73d94ab.c) | type=i32, depth=1000, shape=self | c17 | `1000` |
| [`tail-call.i32.5000.forward.c17.91d1b3e6`](tail-call.i32.5000.forward.c17.91d1b3e6.c) | type=i32, depth=5000, shape=forward | c17 | `5000` |
| [`tail-call.i32.5000.mutual.c17.cbf3bebd`](tail-call.i32.5000.mutual.c17.cbf3bebd.c) | type=i32, depth=5000, shape=mutual | c17 | `5000` |
| [`tail-call.i32.5000.self.c17.4fe36f97`](tail-call.i32.5000.self.c17.4fe36f97.c) | type=i32, depth=5000, shape=self | c17 | `5000` |
| [`tail-call.i32.8.forward.c17.d4f5b459`](tail-call.i32.8.forward.c17.d4f5b459.c) | type=i32, depth=8, shape=forward | c17 | `8` |
| [`tail-call.i32.8.mutual.c17.f6eb162f`](tail-call.i32.8.mutual.c17.f6eb162f.c) | type=i32, depth=8, shape=mutual | c17 | `8` |
| [`tail-call.i32.8.self.c17.42a7a14c`](tail-call.i32.8.self.c17.42a7a14c.c) | type=i32, depth=8, shape=self | c17 | `8` |
| [`tail-call.i64.1000.forward.c17.30e06a0f`](tail-call.i64.1000.forward.c17.30e06a0f.c) | type=i64, depth=1000, shape=forward | c17 | `1000` |
| [`tail-call.i64.1000.mutual.c17.4ef8eb8b`](tail-call.i64.1000.mutual.c17.4ef8eb8b.c) | type=i64, depth=1000, shape=mutual | c17 | `1000` |
| [`tail-call.i64.1000.self.c17.488187d8`](tail-call.i64.1000.self.c17.488187d8.c) | type=i64, depth=1000, shape=self | c17 | `1000` |
| [`tail-call.i64.5000.forward.c17.da93c5f0`](tail-call.i64.5000.forward.c17.da93c5f0.c) | type=i64, depth=5000, shape=forward | c17 | `5000` |
| [`tail-call.i64.5000.mutual.c17.266a627e`](tail-call.i64.5000.mutual.c17.266a627e.c) | type=i64, depth=5000, shape=mutual | c17 | `5000` |
| [`tail-call.i64.5000.self.c17.aa24028a`](tail-call.i64.5000.self.c17.aa24028a.c) | type=i64, depth=5000, shape=self | c17 | `5000` |
| [`tail-call.i64.8.forward.c17.5ab49567`](tail-call.i64.8.forward.c17.5ab49567.c) | type=i64, depth=8, shape=forward | c17 | `8` |
| [`tail-call.i64.8.mutual.c17.b2afa327`](tail-call.i64.8.mutual.c17.b2afa327.c) | type=i64, depth=8, shape=mutual | c17 | `8` |
| [`tail-call.i64.8.self.c17.7850227b`](tail-call.i64.8.self.c17.7850227b.c) | type=i64, depth=8, shape=self | c17 | `8` |
| [`tail-call.u32.1000.forward.c17.606da8f6`](tail-call.u32.1000.forward.c17.606da8f6.c) | type=u32, depth=1000, shape=forward | c17 | `1000` |
| [`tail-call.u32.1000.mutual.c17.af2242ef`](tail-call.u32.1000.mutual.c17.af2242ef.c) | type=u32, depth=1000, shape=mutual | c17 | `1000` |
| [`tail-call.u32.1000.self.c17.e0e09ac9`](tail-call.u32.1000.self.c17.e0e09ac9.c) | type=u32, depth=1000, shape=self | c17 | `1000` |
| [`tail-call.u32.5000.forward.c17.05b4a53d`](tail-call.u32.5000.forward.c17.05b4a53d.c) | type=u32, depth=5000, shape=forward | c17 | `5000` |
| [`tail-call.u32.5000.mutual.c17.83f7f44f`](tail-call.u32.5000.mutual.c17.83f7f44f.c) | type=u32, depth=5000, shape=mutual | c17 | `5000` |
| [`tail-call.u32.5000.self.c17.8d8d171f`](tail-call.u32.5000.self.c17.8d8d171f.c) | type=u32, depth=5000, shape=self | c17 | `5000` |
| [`tail-call.u32.8.forward.c17.05f6ac5b`](tail-call.u32.8.forward.c17.05f6ac5b.c) | type=u32, depth=8, shape=forward | c17 | `8` |
| [`tail-call.u32.8.mutual.c17.362e28a2`](tail-call.u32.8.mutual.c17.362e28a2.c) | type=u32, depth=8, shape=mutual | c17 | `8` |
| [`tail-call.u32.8.self.c17.6d00a469`](tail-call.u32.8.self.c17.6d00a469.c) | type=u32, depth=8, shape=self | c17 | `8` |
| [`tail-call.u64.1000.forward.c17.7de1ce93`](tail-call.u64.1000.forward.c17.7de1ce93.c) | type=u64, depth=1000, shape=forward | c17 | `1000` |
| [`tail-call.u64.1000.mutual.c17.7228d34c`](tail-call.u64.1000.mutual.c17.7228d34c.c) | type=u64, depth=1000, shape=mutual | c17 | `1000` |
| [`tail-call.u64.1000.self.c17.18bef552`](tail-call.u64.1000.self.c17.18bef552.c) | type=u64, depth=1000, shape=self | c17 | `1000` |
| [`tail-call.u64.5000.forward.c17.0e97200f`](tail-call.u64.5000.forward.c17.0e97200f.c) | type=u64, depth=5000, shape=forward | c17 | `5000` |
| [`tail-call.u64.5000.mutual.c17.53cf3d28`](tail-call.u64.5000.mutual.c17.53cf3d28.c) | type=u64, depth=5000, shape=mutual | c17 | `5000` |
| [`tail-call.u64.5000.self.c17.969099ec`](tail-call.u64.5000.self.c17.969099ec.c) | type=u64, depth=5000, shape=self | c17 | `5000` |
| [`tail-call.u64.8.forward.c17.f426d4da`](tail-call.u64.8.forward.c17.f426d4da.c) | type=u64, depth=8, shape=forward | c17 | `8` |
| [`tail-call.u64.8.mutual.c17.05f25c14`](tail-call.u64.8.mutual.c17.05f25c14.c) | type=u64, depth=8, shape=mutual | c17 | `8` |
| [`tail-call.u64.8.self.c17.9fd93a34`](tail-call.u64.8.self.c17.9fd93a34.c) | type=u64, depth=8, shape=self | c17 | `8` |

