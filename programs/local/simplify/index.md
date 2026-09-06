# simplify

algebraic identities and the local peephole rules. Part of the local phase of the M4 plan.

53 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`simplify.bool.one-bit.c17.4257870d`](simplify.bool.one-bit.c17.4257870d.c) | type=bool, group=one-bit | c17 | `1 1 0 ...` and 36 more lines |
| [`simplify.i16.additive.c17.470faaa8`](simplify.i16.additive.c17.470faaa8.c) | type=i16, group=additive | c17 | `-32768 -32768 -32768 ...` and 21 more lines |
| [`simplify.i16.bitwise-constant.c17.32d3064b`](simplify.i16.bitwise-constant.c17.32d3064b.c) | type=i16, group=bitwise-constant | c17 | `0 0 -32768 ...` and 57 more lines |
| [`simplify.i16.bitwise-self.c17.260d272e`](simplify.i16.bitwise-self.c17.260d272e.c) | type=i16, group=bitwise-self | c17 | `-32768 -32768 0 ...` and 15 more lines |
| [`simplify.i16.involution.c17.922f2816`](simplify.i16.involution.c17.922f2816.c) | type=i16, group=involution | c17 | `-32768 -32768 1 ...` and 45 more lines |
| [`simplify.i16.multiplicative.c17.98634f21`](simplify.i16.multiplicative.c17.98634f21.c) | type=i16, group=multiplicative | c17 | `-32768 -32768 0 ...` and 33 more lines |
| [`simplify.i16.narrowed.c17.019953e8`](simplify.i16.narrowed.c17.019953e8.c) | type=i16, group=narrowed | c17 | `-32768 -32768 -32768 ...` and 44 more lines |
| [`simplify.i16.shift.c17.10b729aa`](simplify.i16.shift.c17.10b729aa.c) | type=i16, group=shift | c17 | `1 1 7 ...` and 5 more lines |
| [`simplify.i32.additive.c17.7209983c`](simplify.i32.additive.c17.7209983c.c) | type=i32, group=additive | c17 | `-2147483648 -2147483648 -2147483648 ...` and 21 more lines |
| [`simplify.i32.bitwise-constant.c17.b29bfed8`](simplify.i32.bitwise-constant.c17.b29bfed8.c) | type=i32, group=bitwise-constant | c17 | `0 0 -2147483648 ...` and 57 more lines |
| [`simplify.i32.bitwise-self.c17.1fc40dfd`](simplify.i32.bitwise-self.c17.1fc40dfd.c) | type=i32, group=bitwise-self | c17 | `-2147483648 -2147483648 0 ...` and 15 more lines |
| [`simplify.i32.involution.c17.d3542390`](simplify.i32.involution.c17.d3542390.c) | type=i32, group=involution | c17 | `-2147483648 1 0 ...` and 44 more lines |
| [`simplify.i32.multiplicative.c17.f33a52c9`](simplify.i32.multiplicative.c17.f33a52c9.c) | type=i32, group=multiplicative | c17 | `-2147483648 -2147483648 0 ...` and 33 more lines |
| [`simplify.i32.shift.c17.87d2e87f`](simplify.i32.shift.c17.87d2e87f.c) | type=i32, group=shift | c17 | `1 1 7 ...` and 5 more lines |
| [`simplify.i64.additive.c17.d41e2f60`](simplify.i64.additive.c17.d41e2f60.c) | type=i64, group=additive | c17 | `-9223372036854775808 -9223372036854775808 -9223372036854775808 ...` and 21 more lines |
| [`simplify.i64.bitwise-constant.c17.7ab0e0de`](simplify.i64.bitwise-constant.c17.7ab0e0de.c) | type=i64, group=bitwise-constant | c17 | `0 0 -9223372036854775808 ...` and 57 more lines |
| [`simplify.i64.bitwise-self.c17.4fae386b`](simplify.i64.bitwise-self.c17.4fae386b.c) | type=i64, group=bitwise-self | c17 | `-9223372036854775808 -9223372036854775808 0 ...` and 15 more lines |
| [`simplify.i64.involution.c17.00667165`](simplify.i64.involution.c17.00667165.c) | type=i64, group=involution | c17 | `-9223372036854775808 1 0 ...` and 44 more lines |
| [`simplify.i64.multiplicative.c17.ff8281fa`](simplify.i64.multiplicative.c17.ff8281fa.c) | type=i64, group=multiplicative | c17 | `-9223372036854775808 -9223372036854775808 0 ...` and 33 more lines |
| [`simplify.i64.shift.c17.a4969485`](simplify.i64.shift.c17.a4969485.c) | type=i64, group=shift | c17 | `1 1 7 ...` and 5 more lines |
| [`simplify.i8.additive.c17.024e60f7`](simplify.i8.additive.c17.024e60f7.c) | type=i8, group=additive | c17 | `-128 -128 -128 ...` and 21 more lines |
| [`simplify.i8.bitwise-constant.c17.ff88f848`](simplify.i8.bitwise-constant.c17.ff88f848.c) | type=i8, group=bitwise-constant | c17 | `0 0 -128 ...` and 57 more lines |
| [`simplify.i8.bitwise-self.c17.bf297a5a`](simplify.i8.bitwise-self.c17.bf297a5a.c) | type=i8, group=bitwise-self | c17 | `-128 -128 0 ...` and 15 more lines |
| [`simplify.i8.involution.c17.444afe1d`](simplify.i8.involution.c17.444afe1d.c) | type=i8, group=involution | c17 | `-128 -128 1 ...` and 45 more lines |
| [`simplify.i8.multiplicative.c17.1f3c2659`](simplify.i8.multiplicative.c17.1f3c2659.c) | type=i8, group=multiplicative | c17 | `-128 -128 0 ...` and 33 more lines |
| [`simplify.i8.narrowed.c17.936736f0`](simplify.i8.narrowed.c17.936736f0.c) | type=i8, group=narrowed | c17 | `-128 -128 -128 ...` and 45 more lines |
| [`simplify.i8.shift.c17.f0619d0f`](simplify.i8.shift.c17.f0619d0f.c) | type=i8, group=shift | c17 | `0 0 2 ...` and 5 more lines |
| [`simplify.u16.additive.c17.d5e616db`](simplify.u16.additive.c17.d5e616db.c) | type=u16, group=additive | c17 | `0 0 0 ...` and 21 more lines |
| [`simplify.u16.bitwise-constant.c17.2ddfa508`](simplify.u16.bitwise-constant.c17.2ddfa508.c) | type=u16, group=bitwise-constant | c17 | `0 0 0 ...` and 57 more lines |
| [`simplify.u16.bitwise-self.c17.6ee62fa2`](simplify.u16.bitwise-self.c17.6ee62fa2.c) | type=u16, group=bitwise-self | c17 | `0 0 0 ...` and 15 more lines |
| [`simplify.u16.involution.c17.69f4f927`](simplify.u16.involution.c17.69f4f927.c) | type=u16, group=involution | c17 | `0 0 1 ...` and 45 more lines |
| [`simplify.u16.multiplicative.c17.24d34d8c`](simplify.u16.multiplicative.c17.24d34d8c.c) | type=u16, group=multiplicative | c17 | `0 0 0 ...` and 33 more lines |
| [`simplify.u16.narrowed.c17.d50d8f9b`](simplify.u16.narrowed.c17.d50d8f9b.c) | type=u16, group=narrowed | c17 | `0 0 0 ...` and 44 more lines |
| [`simplify.u16.shift.c17.ad9ec013`](simplify.u16.shift.c17.ad9ec013.c) | type=u16, group=shift | c17 | `0 0 2 ...` and 9 more lines |
| [`simplify.u32.additive.c17.c90b534b`](simplify.u32.additive.c17.c90b534b.c) | type=u32, group=additive | c17 | `0 0 0 ...` and 21 more lines |
| [`simplify.u32.bitwise-constant.c17.2db73241`](simplify.u32.bitwise-constant.c17.2db73241.c) | type=u32, group=bitwise-constant | c17 | `0 0 0 ...` and 57 more lines |
| [`simplify.u32.bitwise-self.c17.525b129d`](simplify.u32.bitwise-self.c17.525b129d.c) | type=u32, group=bitwise-self | c17 | `0 0 0 ...` and 15 more lines |
| [`simplify.u32.involution.c17.9383248e`](simplify.u32.involution.c17.9383248e.c) | type=u32, group=involution | c17 | `0 0 1 ...` and 45 more lines |
| [`simplify.u32.multiplicative.c17.fae95a40`](simplify.u32.multiplicative.c17.fae95a40.c) | type=u32, group=multiplicative | c17 | `0 0 0 ...` and 33 more lines |
| [`simplify.u32.shift.c17.8926185d`](simplify.u32.shift.c17.8926185d.c) | type=u32, group=shift | c17 | `0 0 2 ...` and 9 more lines |
| [`simplify.u64.additive.c17.a05a7d6f`](simplify.u64.additive.c17.a05a7d6f.c) | type=u64, group=additive | c17 | `0 0 0 ...` and 21 more lines |
| [`simplify.u64.bitwise-constant.c17.30638273`](simplify.u64.bitwise-constant.c17.30638273.c) | type=u64, group=bitwise-constant | c17 | `0 0 0 ...` and 57 more lines |
| [`simplify.u64.bitwise-self.c17.673be6b3`](simplify.u64.bitwise-self.c17.673be6b3.c) | type=u64, group=bitwise-self | c17 | `0 0 0 ...` and 15 more lines |
| [`simplify.u64.involution.c17.7aac3d7e`](simplify.u64.involution.c17.7aac3d7e.c) | type=u64, group=involution | c17 | `0 0 1 ...` and 45 more lines |
| [`simplify.u64.multiplicative.c17.7c9152e3`](simplify.u64.multiplicative.c17.7c9152e3.c) | type=u64, group=multiplicative | c17 | `0 0 0 ...` and 33 more lines |
| [`simplify.u64.shift.c17.afa752f1`](simplify.u64.shift.c17.afa752f1.c) | type=u64, group=shift | c17 | `0 0 2 ...` and 9 more lines |
| [`simplify.u8.additive.c17.88c93d56`](simplify.u8.additive.c17.88c93d56.c) | type=u8, group=additive | c17 | `0 0 0 ...` and 21 more lines |
| [`simplify.u8.bitwise-constant.c17.35a4c3ab`](simplify.u8.bitwise-constant.c17.35a4c3ab.c) | type=u8, group=bitwise-constant | c17 | `0 0 0 ...` and 57 more lines |
| [`simplify.u8.bitwise-self.c17.43bae474`](simplify.u8.bitwise-self.c17.43bae474.c) | type=u8, group=bitwise-self | c17 | `0 0 0 ...` and 15 more lines |
| [`simplify.u8.involution.c17.0c94b030`](simplify.u8.involution.c17.0c94b030.c) | type=u8, group=involution | c17 | `0 0 1 ...` and 45 more lines |
| [`simplify.u8.multiplicative.c17.166f5f93`](simplify.u8.multiplicative.c17.166f5f93.c) | type=u8, group=multiplicative | c17 | `0 0 0 ...` and 33 more lines |
| [`simplify.u8.narrowed.c17.ab57e320`](simplify.u8.narrowed.c17.ab57e320.c) | type=u8, group=narrowed | c17 | `0 0 0 ...` and 45 more lines |
| [`simplify.u8.shift.c17.9ff42073`](simplify.u8.shift.c17.9ff42073.c) | type=u8, group=shift | c17 | `0 0 1 ...` and 9 more lines |

