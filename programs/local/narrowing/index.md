# narrowing

taking the width back off arithmetic that C promoted. Part of the local phase of the M4 plan.

52 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`narrowing.i16.below-source.c17.a090d6ab`](narrowing.i16.below-source.c17.a090d6ab.c) | type=i16, shape=below-source | c17 | `0 0 -2 ...` and 7 more lines |
| [`narrowing.i16.bit-field.c17.f8c2c249`](narrowing.i16.bit-field.c17.f8c2c249.c) | type=i16, shape=bit-field | c17 | `7 7 7 21` |
| [`narrowing.i16.divide.c17.7fd20627`](narrowing.i16.divide.c17.7fd20627.c) | type=i16, shape=divide | c17 | `-10922 -2 0 ...` and 7 more lines |
| [`narrowing.i16.mixed-sign.c17.cf5906bf`](narrowing.i16.mixed-sign.c17.cf5906bf.c) | type=i16, shape=mixed-sign | c17 | `-32765 1 5 ...` and 2 more lines |
| [`narrowing.i16.round-trip.c17.001ce1de`](narrowing.i16.round-trip.c17.001ce1de.c) | type=i16, shape=round-trip | c17 | `1 -32767 1 ...` and 7 more lines |
| [`narrowing.i16.round-trip.c17.a1a3b749`](narrowing.i16.round-trip.c17.a1a3b749.c) | type=i16, shape=round-trip | c17 | `-32768 -32768 -2 ...` and 7 more lines |
| [`narrowing.i16.shift.c17.132bb9fc`](narrowing.i16.shift.c17.132bb9fc.c) | type=i16, shape=shift | c17 | `16 1024 128 ...` and 3 more lines |
| [`narrowing.i16.stop-early.c17.2091cc82`](narrowing.i16.stop-early.c17.2091cc82.c) | type=i16, shape=stop-early | c17 | `-32768 -2 2 ...` and 2 more lines |
| [`narrowing.i16.stored-back.c17.ff62d4be`](narrowing.i16.stored-back.c17.ff62d4be.c) | type=i16, shape=stored-back | c17 | `-32763 32763 -32768 ...` and 27 more lines |
| [`narrowing.i16.wide-use.c17.565bc239`](narrowing.i16.wide-use.c17.565bc239.c) | type=i16, shape=wide-use | c17 | `-1 32765 32769 ...` and 2 more lines |
| [`narrowing.i16.widen-twice.c17.35604e34`](narrowing.i16.widen-twice.c17.35604e34.c) | type=i16, shape=widen-twice | c17 | `-32768 -2 2 ...` and 2 more lines |
| [`narrowing.i32.below-source.c17.8577982c`](narrowing.i32.below-source.c17.8577982c.c) | type=i32, shape=below-source | c17 | `0 0 -2 ...` and 7 more lines |
| [`narrowing.i32.narrow-twice.c17.b074c2d9`](narrowing.i32.narrow-twice.c17.b074c2d9.c) | type=i32, shape=narrow-twice | c17 | `0 -2 2 ...` and 2 more lines |
| [`narrowing.i32.round-trip.c17.0a65f4c6`](narrowing.i32.round-trip.c17.0a65f4c6.c) | type=i32, shape=round-trip | c17 | `-2147483648 -2 2 ...` and 2 more lines |
| [`narrowing.i64.narrow-twice.c17.41c659ad`](narrowing.i64.narrow-twice.c17.41c659ad.c) | type=i64, shape=narrow-twice | c17 | `0 0 0 ...` and 12 more lines |
| [`narrowing.i8.bit-field.c17.d539a5ca`](narrowing.i8.bit-field.c17.d539a5ca.c) | type=i8, shape=bit-field | c17 | `7 7 7 21` |
| [`narrowing.i8.divide.c17.686084da`](narrowing.i8.divide.c17.686084da.c) | type=i8, shape=divide | c17 | `-42 -2 0 ...` and 7 more lines |
| [`narrowing.i8.mixed-sign.c17.3623efef`](narrowing.i8.mixed-sign.c17.3623efef.c) | type=i8, shape=mixed-sign | c17 | `-125 1 4 ...` and 2 more lines |
| [`narrowing.i8.round-trip.c17.62cc97f4`](narrowing.i8.round-trip.c17.62cc97f4.c) | type=i8, shape=round-trip | c17 | `-128 -128 -128 ...` and 12 more lines |
| [`narrowing.i8.round-trip.c17.d04f6291`](narrowing.i8.round-trip.c17.d04f6291.c) | type=i8, shape=round-trip | c17 | `1 -127 1 ...` and 7 more lines |
| [`narrowing.i8.shift.c17.be6a1a70`](narrowing.i8.shift.c17.be6a1a70.c) | type=i8, shape=shift | c17 | `8 0 56 ...` and 3 more lines |
| [`narrowing.i8.stop-early.c17.8685a464`](narrowing.i8.stop-early.c17.8685a464.c) | type=i8, shape=stop-early | c17 | `-128 -128 -128 ...` and 12 more lines |
| [`narrowing.i8.stored-back.c17.e7b4d379`](narrowing.i8.stored-back.c17.e7b4d379.c) | type=i8, shape=stored-back | c17 | `-123 123 -128 ...` and 27 more lines |
| [`narrowing.i8.wide-use.c17.f9828900`](narrowing.i8.wide-use.c17.f9828900.c) | type=i8, shape=wide-use | c17 | `-1 125 128 ...` and 2 more lines |
| [`narrowing.i8.widen-twice.c17.487f71ae`](narrowing.i8.widen-twice.c17.487f71ae.c) | type=i8, shape=widen-twice | c17 | `-128 -128 -128 ...` and 12 more lines |
| [`narrowing.u16.below-source.c17.36ec7f81`](narrowing.u16.below-source.c17.36ec7f81.c) | type=u16, shape=below-source | c17 | `0 0 2 ...` and 7 more lines |
| [`narrowing.u16.bit-field.c17.8b84bb86`](narrowing.u16.bit-field.c17.8b84bb86.c) | type=u16, shape=bit-field | c17 | `7 7 7 21` |
| [`narrowing.u16.divide.c17.1380b1a1`](narrowing.u16.divide.c17.1380b1a1.c) | type=u16, shape=divide | c17 | `0 0 0 ...` and 7 more lines |
| [`narrowing.u16.mixed-sign.c17.691a0dce`](narrowing.u16.mixed-sign.c17.691a0dce.c) | type=u16, shape=mixed-sign | c17 | `3 5 11 ...` and 2 more lines |
| [`narrowing.u16.mixed.c17.98bcbe12`](narrowing.u16.mixed.c17.98bcbe12.c) | type=u16, shape=mixed | c17 | `0 2 8 ...` and 2 more lines |
| [`narrowing.u16.round-trip.c17.a603092d`](narrowing.u16.round-trip.c17.a603092d.c) | type=u16, shape=round-trip | c17 | `1 1 1 ...` and 7 more lines |
| [`narrowing.u16.round-trip.c17.f6035c93`](narrowing.u16.round-trip.c17.f6035c93.c) | type=u16, shape=round-trip | c17 | `0 0 2 ...` and 7 more lines |
| [`narrowing.u16.shift.c17.1255246f`](narrowing.u16.shift.c17.1255246f.c) | type=u16, shape=shift | c17 | `0 0 16 ...` and 7 more lines |
| [`narrowing.u16.stop-early.c17.c290cf77`](narrowing.u16.stop-early.c17.c290cf77.c) | type=u16, shape=stop-early | c17 | `0 2 8 ...` and 2 more lines |
| [`narrowing.u16.stored-back.c17.91c77af5`](narrowing.u16.stored-back.c17.91c77af5.c) | type=u16, shape=stored-back | c17 | `5 65531 0 ...` and 27 more lines |
| [`narrowing.u16.wide-use.c17.03304a4e`](narrowing.u16.wide-use.c17.03304a4e.c) | type=u16, shape=wide-use | c17 | `65535 65537 65543 ...` and 2 more lines |
| [`narrowing.u16.widen-twice.c17.ebd3f53d`](narrowing.u16.widen-twice.c17.ebd3f53d.c) | type=u16, shape=widen-twice | c17 | `0 2 8 ...` and 2 more lines |
| [`narrowing.u32.below-source.c17.c2022707`](narrowing.u32.below-source.c17.c2022707.c) | type=u32, shape=below-source | c17 | `0 0 2 ...` and 7 more lines |
| [`narrowing.u32.narrow-twice.c17.7498b5b9`](narrowing.u32.narrow-twice.c17.7498b5b9.c) | type=u32, shape=narrow-twice | c17 | `0 2 8 ...` and 2 more lines |
| [`narrowing.u32.round-trip.c17.91c56a34`](narrowing.u32.round-trip.c17.91c56a34.c) | type=u32, shape=round-trip | c17 | `0 2 8 ...` and 2 more lines |
| [`narrowing.u64.narrow-twice.c17.dd3483b2`](narrowing.u64.narrow-twice.c17.dd3483b2.c) | type=u64, shape=narrow-twice | c17 | `0 0 0 ...` and 12 more lines |
| [`narrowing.u8.bit-field.c17.454b8dd2`](narrowing.u8.bit-field.c17.454b8dd2.c) | type=u8, shape=bit-field | c17 | `7 7 7 21` |
| [`narrowing.u8.divide.c17.4f85b987`](narrowing.u8.divide.c17.4f85b987.c) | type=u8, shape=divide | c17 | `0 0 0 ...` and 7 more lines |
| [`narrowing.u8.mixed-sign.c17.d49085f3`](narrowing.u8.mixed-sign.c17.d49085f3.c) | type=u8, shape=mixed-sign | c17 | `3 4 6 ...` and 2 more lines |
| [`narrowing.u8.mixed.c17.1279171e`](narrowing.u8.mixed.c17.1279171e.c) | type=u8, shape=mixed | c17 | `0 0 0 ...` and 12 more lines |
| [`narrowing.u8.round-trip.c17.55bf6dbc`](narrowing.u8.round-trip.c17.55bf6dbc.c) | type=u8, shape=round-trip | c17 | `1 1 1 ...` and 7 more lines |
| [`narrowing.u8.round-trip.c17.f5c88c5c`](narrowing.u8.round-trip.c17.f5c88c5c.c) | type=u8, shape=round-trip | c17 | `0 0 0 ...` and 12 more lines |
| [`narrowing.u8.shift.c17.46cde2e6`](narrowing.u8.shift.c17.46cde2e6.c) | type=u8, shape=shift | c17 | `0 0 8 ...` and 7 more lines |
| [`narrowing.u8.stop-early.c17.205008ff`](narrowing.u8.stop-early.c17.205008ff.c) | type=u8, shape=stop-early | c17 | `0 0 0 ...` and 12 more lines |
| [`narrowing.u8.stored-back.c17.80cffd5b`](narrowing.u8.stored-back.c17.80cffd5b.c) | type=u8, shape=stored-back | c17 | `5 251 0 ...` and 27 more lines |
| [`narrowing.u8.wide-use.c17.a76ee834`](narrowing.u8.wide-use.c17.a76ee834.c) | type=u8, shape=wide-use | c17 | `255 256 258 ...` and 2 more lines |
| [`narrowing.u8.widen-twice.c17.834b0c49`](narrowing.u8.widen-twice.c17.834b0c49.c) | type=u8, shape=widen-twice | c17 | `0 0 0 ...` and 12 more lines |

