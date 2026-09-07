# if-conversion

turning a short branch into branchless code. Part of the backend phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`if-conversion.i32.abs.c17.4fb723f5`](if-conversion.i32.abs.c17.4fb723f5.c) | type=i32, shape=abs | c17 | `16000` |
| [`if-conversion.i32.clamp.c17.a1d174d0`](if-conversion.i32.clamp.c17.a1d174d0.c) | type=i32, shape=clamp | c17 | `16500` |
| [`if-conversion.i32.conditional-add.c17.4e310e10`](if-conversion.i32.conditional-add.c17.4e310e10.c) | type=i32, shape=conditional-add | c17 | `10000` |
| [`if-conversion.i32.min.c17.08421b4d`](if-conversion.i32.min.c17.08421b4d.c) | type=i32, shape=min | c17 | `12000` |
| [`if-conversion.i32.select.c17.cab85f49`](if-conversion.i32.select.c17.cab85f49.c) | type=i32, shape=select | c17 | `16000` |
| [`if-conversion.i64.abs.c17.43278f76`](if-conversion.i64.abs.c17.43278f76.c) | type=i64, shape=abs | c17 | `16000` |
| [`if-conversion.i64.clamp.c17.ebdb1694`](if-conversion.i64.clamp.c17.ebdb1694.c) | type=i64, shape=clamp | c17 | `16500` |
| [`if-conversion.i64.conditional-add.c17.51fb0879`](if-conversion.i64.conditional-add.c17.51fb0879.c) | type=i64, shape=conditional-add | c17 | `10000` |
| [`if-conversion.i64.min.c17.9a0c96e2`](if-conversion.i64.min.c17.9a0c96e2.c) | type=i64, shape=min | c17 | `12000` |
| [`if-conversion.i64.select.c17.1ce50553`](if-conversion.i64.select.c17.1ce50553.c) | type=i64, shape=select | c17 | `16000` |
| [`if-conversion.u32.abs.c17.fd4ca49f`](if-conversion.u32.abs.c17.fd4ca49f.c) | type=u32, shape=abs | c17 | `16000` |
| [`if-conversion.u32.clamp.c17.219bf747`](if-conversion.u32.clamp.c17.219bf747.c) | type=u32, shape=clamp | c17 | `16500` |
| [`if-conversion.u32.conditional-add.c17.f035ec4b`](if-conversion.u32.conditional-add.c17.f035ec4b.c) | type=u32, shape=conditional-add | c17 | `10000` |
| [`if-conversion.u32.min.c17.b9f77220`](if-conversion.u32.min.c17.b9f77220.c) | type=u32, shape=min | c17 | `12000` |
| [`if-conversion.u32.select.c17.35472caf`](if-conversion.u32.select.c17.35472caf.c) | type=u32, shape=select | c17 | `16000` |
| [`if-conversion.u64.abs.c17.3bc41673`](if-conversion.u64.abs.c17.3bc41673.c) | type=u64, shape=abs | c17 | `16000` |
| [`if-conversion.u64.clamp.c17.2c10ec8f`](if-conversion.u64.clamp.c17.2c10ec8f.c) | type=u64, shape=clamp | c17 | `16500` |
| [`if-conversion.u64.conditional-add.c17.44c1e2f9`](if-conversion.u64.conditional-add.c17.44c1e2f9.c) | type=u64, shape=conditional-add | c17 | `10000` |
| [`if-conversion.u64.min.c17.57148564`](if-conversion.u64.min.c17.57148564.c) | type=u64, shape=min | c17 | `12000` |
| [`if-conversion.u64.select.c17.315c29f6`](if-conversion.u64.select.c17.315c29f6.c) | type=u64, shape=select | c17 | `16000` |

