# bit-builtins

the bit counting builtins, over every position at both widths. Part of the backend phase of the M4 plan.

22 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`bit-builtins.clz.32.constant.c17.28dfb88c`](bit-builtins.clz.32.constant.c17.28dfb88c.c) | builtin=clz, width=32, form=constant | c17 | `31 30 29 ...` and 35 more lines |
| [`bit-builtins.clz.32.opaque.c17.ae6288bb`](bit-builtins.clz.32.opaque.c17.ae6288bb.c) | builtin=clz, width=32, form=opaque | c17 | `31 30 29 ...` and 35 more lines |
| [`bit-builtins.clz.64.constant.c17.f4df51de`](bit-builtins.clz.64.constant.c17.f4df51de.c) | builtin=clz, width=64, form=constant | c17 | `63 62 61 ...` and 67 more lines |
| [`bit-builtins.clz.64.opaque.c17.141bb983`](bit-builtins.clz.64.opaque.c17.141bb983.c) | builtin=clz, width=64, form=opaque | c17 | `63 62 61 ...` and 67 more lines |
| [`bit-builtins.ctz.32.constant.c17.e6049bf4`](bit-builtins.ctz.32.constant.c17.e6049bf4.c) | builtin=ctz, width=32, form=constant | c17 | `0 1 2 ...` and 35 more lines |
| [`bit-builtins.ctz.32.opaque.c17.17ff467a`](bit-builtins.ctz.32.opaque.c17.17ff467a.c) | builtin=ctz, width=32, form=opaque | c17 | `0 1 2 ...` and 35 more lines |
| [`bit-builtins.ctz.64.constant.c17.47733a8a`](bit-builtins.ctz.64.constant.c17.47733a8a.c) | builtin=ctz, width=64, form=constant | c17 | `0 1 2 ...` and 67 more lines |
| [`bit-builtins.ctz.64.opaque.c17.7db4c85e`](bit-builtins.ctz.64.opaque.c17.7db4c85e.c) | builtin=ctz, width=64, form=opaque | c17 | `0 1 2 ...` and 67 more lines |
| [`bit-builtins.family.32.zero-argument.c17.e04c2836`](bit-builtins.family.32.zero-argument.c17.e04c2836.c) | builtin=family, width=32, form=zero-argument | c17 | `0 0 0 ...` and 5 more lines |
| [`bit-builtins.family.64.zero-argument.c17.2cb5a63a`](bit-builtins.family.64.zero-argument.c17.2cb5a63a.c) | builtin=family, width=64, form=zero-argument | c17 | `0 0 0 ...` and 5 more lines |
| [`bit-builtins.ffs.32.constant.c17.87d91dc6`](bit-builtins.ffs.32.constant.c17.87d91dc6.c) | builtin=ffs, width=32, form=constant | c17 | `1 2 3 ...` and 35 more lines |
| [`bit-builtins.ffs.32.opaque.c17.6a377103`](bit-builtins.ffs.32.opaque.c17.6a377103.c) | builtin=ffs, width=32, form=opaque | c17 | `1 2 3 ...` and 35 more lines |
| [`bit-builtins.ffs.64.constant.c17.1ba535b8`](bit-builtins.ffs.64.constant.c17.1ba535b8.c) | builtin=ffs, width=64, form=constant | c17 | `1 2 3 ...` and 67 more lines |
| [`bit-builtins.ffs.64.opaque.c17.f819e4ba`](bit-builtins.ffs.64.opaque.c17.f819e4ba.c) | builtin=ffs, width=64, form=opaque | c17 | `1 2 3 ...` and 67 more lines |
| [`bit-builtins.parity.32.constant.c17.6c9cd445`](bit-builtins.parity.32.constant.c17.6c9cd445.c) | builtin=parity, width=32, form=constant | c17 | `1 1 1 ...` and 35 more lines |
| [`bit-builtins.parity.32.opaque.c17.3d0509eb`](bit-builtins.parity.32.opaque.c17.3d0509eb.c) | builtin=parity, width=32, form=opaque | c17 | `1 1 1 ...` and 35 more lines |
| [`bit-builtins.parity.64.constant.c17.b55da1a0`](bit-builtins.parity.64.constant.c17.b55da1a0.c) | builtin=parity, width=64, form=constant | c17 | `1 1 1 ...` and 67 more lines |
| [`bit-builtins.parity.64.opaque.c17.101268bd`](bit-builtins.parity.64.opaque.c17.101268bd.c) | builtin=parity, width=64, form=opaque | c17 | `1 1 1 ...` and 67 more lines |
| [`bit-builtins.popcount.32.constant.c17.57b6685a`](bit-builtins.popcount.32.constant.c17.57b6685a.c) | builtin=popcount, width=32, form=constant | c17 | `1 1 1 ...` and 35 more lines |
| [`bit-builtins.popcount.32.opaque.c17.d12f5f5b`](bit-builtins.popcount.32.opaque.c17.d12f5f5b.c) | builtin=popcount, width=32, form=opaque | c17 | `1 1 1 ...` and 35 more lines |
| [`bit-builtins.popcount.64.constant.c17.ae754876`](bit-builtins.popcount.64.constant.c17.ae754876.c) | builtin=popcount, width=64, form=constant | c17 | `1 1 1 ...` and 67 more lines |
| [`bit-builtins.popcount.64.opaque.c17.570eadf7`](bit-builtins.popcount.64.opaque.c17.570eadf7.c) | builtin=popcount, width=64, form=opaque | c17 | `1 1 1 ...` and 67 more lines |

