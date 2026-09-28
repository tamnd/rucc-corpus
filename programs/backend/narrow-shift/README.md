# narrow-shift

a shift or rotate of a byte or a short by a count below its width. Part of the backend phase of the M4 plan.

8 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`narrow-shift.u16.bitset-flip.c17.772ab7bf`](narrow-shift.u16.bitset-flip.c17.772ab7bf.c) | type=u16, shape=bitset-flip | c17 | `69338990924` |
| [`narrow-shift.u16.bitset-test.c17.966e73ec`](narrow-shift.u16.bitset-test.c17.966e73ec.c) | type=u16, shape=bitset-test | c17 | `70208273090` |
| [`narrow-shift.u16.rotate.c17.7ebc64fd`](narrow-shift.u16.rotate.c17.7ebc64fd.c) | type=u16, shape=rotate | c17 | `70207219298` |
| [`narrow-shift.u16.shift-xor.c17.3b7439d6`](narrow-shift.u16.shift-xor.c17.3b7439d6.c) | type=u16, shape=shift-xor | c17 | `70207219298` |
| [`narrow-shift.u8.bitset-flip.c17.2cbd3c23`](narrow-shift.u8.bitset-flip.c17.2cbd3c23.c) | type=u8, shape=bitset-flip | c17 | `267724682` |
| [`narrow-shift.u8.bitset-test.c17.907e06c5`](narrow-shift.u8.bitset-test.c17.907e06c5.c) | type=u8, shape=bitset-test | c17 | `269389270` |
| [`narrow-shift.u8.rotate.c17.1e96ea13`](narrow-shift.u8.rotate.c17.1e96ea13.c) | type=u8, shape=rotate | c17 | `268338274` |
| [`narrow-shift.u8.shift-xor.c17.cb9f6aa9`](narrow-shift.u8.shift-xor.c17.cb9f6aa9.c) | type=u8, shape=shift-xor | c17 | `268338274` |

