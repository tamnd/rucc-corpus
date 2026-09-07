# strength

rewriting an operation into a cheaper one with the same value. Part of the local phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`strength.i16.div.c17.5a5d4d5e`](strength.i16.div.c17.5a5d4d5e.c) | type=i16, op=div | c17 | `16384 32768 -32768 ...` and 93 more lines |
| [`strength.i16.mul.c17.f1b6f1c8`](strength.i16.mul.c17.f1b6f1c8.c) | type=i16, op=mul | c17 | `65536 32768 -32768 ...` and 93 more lines |
| [`strength.i16.rem.c17.c27e6ef5`](strength.i16.rem.c17.c27e6ef5.c) | type=i16, op=rem | c17 | `0 0 0 ...` and 93 more lines |
| [`strength.i32.div.c17.998b0586`](strength.i32.div.c17.998b0586.c) | type=i32, op=div | c17 | `1073741824 -2147483648 -1073741824 ...` and 92 more lines |
| [`strength.i32.mul.c17.e339765a`](strength.i32.mul.c17.e339765a.c) | type=i32, op=mul | c17 | `-2147483648 4 2 ...` and 64 more lines |
| [`strength.i32.rem.c17.52243025`](strength.i32.rem.c17.52243025.c) | type=i32, op=rem | c17 | `0 0 0 ...` and 92 more lines |
| [`strength.i64.div.c17.95805b26`](strength.i64.div.c17.95805b26.c) | type=i64, op=div | c17 | `4611686018427387904 -9223372036854775808 -4611686018427387904 ...` and 92 more lines |
| [`strength.i64.mul.c17.2fca5e79`](strength.i64.mul.c17.2fca5e79.c) | type=i64, op=mul | c17 | `-9223372036854775808 4 2 ...` and 64 more lines |
| [`strength.i64.rem.c17.df8cf22f`](strength.i64.rem.c17.df8cf22f.c) | type=i64, op=rem | c17 | `0 0 0 ...` and 92 more lines |
| [`strength.i8.div.c17.b895ba90`](strength.i8.div.c17.b895ba90.c) | type=i8, op=div | c17 | `64 128 -128 ...` and 75 more lines |
| [`strength.i8.mul.c17.bfa42411`](strength.i8.mul.c17.bfa42411.c) | type=i8, op=mul | c17 | `256 128 -128 ...` and 75 more lines |
| [`strength.i8.rem.c17.89932a9d`](strength.i8.rem.c17.89932a9d.c) | type=i8, op=rem | c17 | `0 0 0 ...` and 75 more lines |
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

