# strength

rewriting an operation into a cheaper one with the same value. Part of the local phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`strength.i16.div.c17.e1a1fd43`](strength.i16.div.c17.e1a1fd43.c) | type=i16, op=div | c17 | `-32768 -16384 -10922 ...` and 81 more lines |
| [`strength.i16.mul.c17.37ff079b`](strength.i16.mul.c17.37ff079b.c) | type=i16, op=mul | c17 | `-32768 -65536 -98304 ...` and 81 more lines |
| [`strength.i16.rem.c17.fcae141c`](strength.i16.rem.c17.fcae141c.c) | type=i16, op=rem | c17 | `0 0 -2 ...` and 81 more lines |
| [`strength.i32.div.c17.dc96948a`](strength.i32.div.c17.dc96948a.c) | type=i32, op=div | c17 | `-2147483648 -1073741824 -715827882 ...` and 81 more lines |
| [`strength.i32.mul.c17.dfc2bf9b`](strength.i32.mul.c17.dfc2bf9b.c) | type=i32, op=mul | c17 | `-2147483648 -2 -4 ...` and 55 more lines |
| [`strength.i32.rem.c17.60100504`](strength.i32.rem.c17.60100504.c) | type=i32, op=rem | c17 | `0 0 -2 ...` and 81 more lines |
| [`strength.i64.div.c17.40a54e30`](strength.i64.div.c17.40a54e30.c) | type=i64, op=div | c17 | `-9223372036854775808 -4611686018427387904 -3074457345618258602 ...` and 81 more lines |
| [`strength.i64.mul.c17.e3803e20`](strength.i64.mul.c17.e3803e20.c) | type=i64, op=mul | c17 | `-9223372036854775808 -2 -4 ...` and 55 more lines |
| [`strength.i64.rem.c17.fed2e9e0`](strength.i64.rem.c17.fed2e9e0.c) | type=i64, op=rem | c17 | `0 0 -2 ...` and 81 more lines |
| [`strength.i8.div.c17.89da150e`](strength.i8.div.c17.89da150e.c) | type=i8, op=div | c17 | `-128 -64 -42 ...` and 63 more lines |
| [`strength.i8.mul.c17.e8aac7dd`](strength.i8.mul.c17.e8aac7dd.c) | type=i8, op=mul | c17 | `-128 -256 -384 ...` and 63 more lines |
| [`strength.i8.rem.c17.683d0b2c`](strength.i8.rem.c17.683d0b2c.c) | type=i8, op=rem | c17 | `0 0 -2 ...` and 63 more lines |
| [`strength.u16.div.c17.fbbe6202`](strength.u16.div.c17.fbbe6202.c) | type=u16, op=div | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u16.mul.c17.9d2af0c9`](strength.u16.mul.c17.9d2af0c9.c) | type=u16, op=mul | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u16.rem.c17.01759f90`](strength.u16.rem.c17.01759f90.c) | type=u16, op=rem | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u32.div.c17.3be84d9b`](strength.u32.div.c17.3be84d9b.c) | type=u32, op=div | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u32.mul.c17.cfeae344`](strength.u32.mul.c17.cfeae344.c) | type=u32, op=mul | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u32.rem.c17.39cee52b`](strength.u32.rem.c17.39cee52b.c) | type=u32, op=rem | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u64.div.c17.33efe5b9`](strength.u64.div.c17.33efe5b9.c) | type=u64, op=div | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u64.mul.c17.7ba9c980`](strength.u64.mul.c17.7ba9c980.c) | type=u64, op=mul | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u64.rem.c17.eb9c339e`](strength.u64.rem.c17.eb9c339e.c) | type=u64, op=rem | c17 | `0 0 0 ...` and 81 more lines |
| [`strength.u8.div.c17.e481cc3d`](strength.u8.div.c17.e481cc3d.c) | type=u8, op=div | c17 | `0 0 0 ...` and 69 more lines |
| [`strength.u8.mul.c17.f310bda3`](strength.u8.mul.c17.f310bda3.c) | type=u8, op=mul | c17 | `0 0 0 ...` and 69 more lines |
| [`strength.u8.rem.c17.4d94d09c`](strength.u8.rem.c17.4d94d09c.c) | type=u8, op=rem | c17 | `0 0 0 ...` and 69 more lines |

