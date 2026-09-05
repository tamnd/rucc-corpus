# simplify

algebraic identities and the local peephole rules. Part of the local phase of the M4 plan.

8 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`simplify.i16.c17.a6ed7fcc`](simplify.i16.c17.a6ed7fcc.c) | type=i16 | c17 | `-32768 -32768 -32768 ...` and 107 more lines |
| [`simplify.i32.c17.bc94da60`](simplify.i32.c17.bc94da60.c) | type=i32 | c17 | `-2147483648 -2147483648 -2147483648 ...` and 107 more lines |
| [`simplify.i64.c17.e9732cf0`](simplify.i64.c17.e9732cf0.c) | type=i64 | c17 | `-9223372036854775808 -9223372036854775808 -9223372036854775808 ...` and 107 more lines |
| [`simplify.i8.c17.fa664622`](simplify.i8.c17.fa664622.c) | type=i8 | c17 | `-128 -128 -128 ...` and 107 more lines |
| [`simplify.u16.c17.4049fe2e`](simplify.u16.c17.4049fe2e.c) | type=u16 | c17 | `0 0 0 ...` and 111 more lines |
| [`simplify.u32.c17.1252e39c`](simplify.u32.c17.1252e39c.c) | type=u32 | c17 | `0 0 0 ...` and 111 more lines |
| [`simplify.u64.c17.44597f21`](simplify.u64.c17.44597f21.c) | type=u64 | c17 | `0 0 0 ...` and 111 more lines |
| [`simplify.u8.c17.5c173b0c`](simplify.u8.c17.5c173b0c.c) | type=u8 | c17 | `0 0 0 ...` and 111 more lines |

