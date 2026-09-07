# inline

replacing a call with the body of what it called. Part of the interprocedural phase of the M4 plan.

36 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`inline.i32.1.1.c17.ae0e3d28`](inline.i32.1.1.c17.ae0e3d28.c) | type=i32, body=1, sites=1 | c17 | `1` |
| [`inline.i32.1.20.c17.8d078649`](inline.i32.1.20.c17.8d078649.c) | type=i32, body=1, sites=20 | c17 | `210` |
| [`inline.i32.1.4.c17.f73782ae`](inline.i32.1.4.c17.f73782ae.c) | type=i32, body=1, sites=4 | c17 | `10` |
| [`inline.i32.16.1.c17.d52e66ba`](inline.i32.16.1.c17.d52e66ba.c) | type=i32, body=16, sites=1 | c17 | `136` |
| [`inline.i32.16.20.c17.99e05149`](inline.i32.16.20.c17.99e05149.c) | type=i32, body=16, sites=20 | c17 | `2910` |
| [`inline.i32.16.4.c17.5a293351`](inline.i32.16.4.c17.5a293351.c) | type=i32, body=16, sites=4 | c17 | `550` |
| [`inline.i32.4.1.c17.423429d3`](inline.i32.4.1.c17.423429d3.c) | type=i32, body=4, sites=1 | c17 | `10` |
| [`inline.i32.4.20.c17.325d9e3c`](inline.i32.4.20.c17.325d9e3c.c) | type=i32, body=4, sites=20 | c17 | `390` |
| [`inline.i32.4.4.c17.1f513add`](inline.i32.4.4.c17.1f513add.c) | type=i32, body=4, sites=4 | c17 | `46` |
| [`inline.i64.1.1.c17.e890f5eb`](inline.i64.1.1.c17.e890f5eb.c) | type=i64, body=1, sites=1 | c17 | `1` |
| [`inline.i64.1.20.c17.8867c39d`](inline.i64.1.20.c17.8867c39d.c) | type=i64, body=1, sites=20 | c17 | `210` |
| [`inline.i64.1.4.c17.0303ff3a`](inline.i64.1.4.c17.0303ff3a.c) | type=i64, body=1, sites=4 | c17 | `10` |
| [`inline.i64.16.1.c17.126e048c`](inline.i64.16.1.c17.126e048c.c) | type=i64, body=16, sites=1 | c17 | `136` |
| [`inline.i64.16.20.c17.fc5233c2`](inline.i64.16.20.c17.fc5233c2.c) | type=i64, body=16, sites=20 | c17 | `2910` |
| [`inline.i64.16.4.c17.1978e160`](inline.i64.16.4.c17.1978e160.c) | type=i64, body=16, sites=4 | c17 | `550` |
| [`inline.i64.4.1.c17.39640585`](inline.i64.4.1.c17.39640585.c) | type=i64, body=4, sites=1 | c17 | `10` |
| [`inline.i64.4.20.c17.af5b8887`](inline.i64.4.20.c17.af5b8887.c) | type=i64, body=4, sites=20 | c17 | `390` |
| [`inline.i64.4.4.c17.bd6471ba`](inline.i64.4.4.c17.bd6471ba.c) | type=i64, body=4, sites=4 | c17 | `46` |
| [`inline.u32.1.1.c17.0ae3c9d4`](inline.u32.1.1.c17.0ae3c9d4.c) | type=u32, body=1, sites=1 | c17 | `1` |
| [`inline.u32.1.20.c17.e3f07eae`](inline.u32.1.20.c17.e3f07eae.c) | type=u32, body=1, sites=20 | c17 | `210` |
| [`inline.u32.1.4.c17.e1ca5eb4`](inline.u32.1.4.c17.e1ca5eb4.c) | type=u32, body=1, sites=4 | c17 | `10` |
| [`inline.u32.16.1.c17.91197eb0`](inline.u32.16.1.c17.91197eb0.c) | type=u32, body=16, sites=1 | c17 | `136` |
| [`inline.u32.16.20.c17.d21eed50`](inline.u32.16.20.c17.d21eed50.c) | type=u32, body=16, sites=20 | c17 | `2910` |
| [`inline.u32.16.4.c17.31c5897b`](inline.u32.16.4.c17.31c5897b.c) | type=u32, body=16, sites=4 | c17 | `550` |
| [`inline.u32.4.1.c17.20ed9efb`](inline.u32.4.1.c17.20ed9efb.c) | type=u32, body=4, sites=1 | c17 | `10` |
| [`inline.u32.4.20.c17.9d6d88b8`](inline.u32.4.20.c17.9d6d88b8.c) | type=u32, body=4, sites=20 | c17 | `390` |
| [`inline.u32.4.4.c17.b0032d8e`](inline.u32.4.4.c17.b0032d8e.c) | type=u32, body=4, sites=4 | c17 | `46` |
| [`inline.u64.1.1.c17.1c15d523`](inline.u64.1.1.c17.1c15d523.c) | type=u64, body=1, sites=1 | c17 | `1` |
| [`inline.u64.1.20.c17.4531810e`](inline.u64.1.20.c17.4531810e.c) | type=u64, body=1, sites=20 | c17 | `210` |
| [`inline.u64.1.4.c17.e9aa2536`](inline.u64.1.4.c17.e9aa2536.c) | type=u64, body=1, sites=4 | c17 | `10` |
| [`inline.u64.16.1.c17.e58f3555`](inline.u64.16.1.c17.e58f3555.c) | type=u64, body=16, sites=1 | c17 | `136` |
| [`inline.u64.16.20.c17.a9224ad2`](inline.u64.16.20.c17.a9224ad2.c) | type=u64, body=16, sites=20 | c17 | `2910` |
| [`inline.u64.16.4.c17.e8e6a35b`](inline.u64.16.4.c17.e8e6a35b.c) | type=u64, body=16, sites=4 | c17 | `550` |
| [`inline.u64.4.1.c17.3054e875`](inline.u64.4.1.c17.3054e875.c) | type=u64, body=4, sites=1 | c17 | `10` |
| [`inline.u64.4.20.c17.2599ce08`](inline.u64.4.20.c17.2599ce08.c) | type=u64, body=4, sites=20 | c17 | `390` |
| [`inline.u64.4.4.c17.bc54037b`](inline.u64.4.4.c17.bc54037b.c) | type=u64, body=4, sites=4 | c17 | `46` |

