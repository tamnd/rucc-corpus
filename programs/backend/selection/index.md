# selection

choosing the machine instruction for an operation. Part of the backend phase of the M4 plan.

40 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`selection.i32.average.c17.89ab4963`](selection.i32.average.c17.89ab4963.c) | type=i32, pattern=average | c17 | `16` |
| [`selection.i32.compare-set.c17.7c8a953a`](selection.i32.compare-set.c17.7c8a953a.c) | type=i32, pattern=compare-set | c17 | `1` |
| [`selection.i32.mask-low.c17.a6a4e0eb`](selection.i32.mask-low.c17.a6a4e0eb.c) | type=i32, pattern=mask-low | c17 | `20` |
| [`selection.i32.max.c17.886b792c`](selection.i32.max.c17.886b792c.c) | type=i32, pattern=max | c17 | `20` |
| [`selection.i32.min.c17.554bc586`](selection.i32.min.c17.554bc586.c) | type=i32, pattern=min | c17 | `12` |
| [`selection.i32.multiply-add.c17.c29b9b03`](selection.i32.multiply-add.c17.c29b9b03.c) | type=i32, pattern=multiply-add | c17 | `340` |
| [`selection.i32.negate-add.c17.e16660bc`](selection.i32.negate-add.c17.e16660bc.c) | type=i32, pattern=negate-add | c17 | `80` |
| [`selection.i32.round-down.c17.aae64378`](selection.i32.round-down.c17.aae64378.c) | type=i32, pattern=round-down | c17 | `16` |
| [`selection.i32.shift-add.c17.7bc54245`](selection.i32.shift-add.c17.7bc54245.c) | type=i32, pattern=shift-add | c17 | `172` |
| [`selection.i32.test-bit.c17.ae18e25a`](selection.i32.test-bit.c17.ae18e25a.c) | type=i32, pattern=test-bit | c17 | `1` |
| [`selection.i64.average.c17.32a42447`](selection.i64.average.c17.32a42447.c) | type=i64, pattern=average | c17 | `16` |
| [`selection.i64.compare-set.c17.501288b2`](selection.i64.compare-set.c17.501288b2.c) | type=i64, pattern=compare-set | c17 | `1` |
| [`selection.i64.mask-low.c17.9e049b3c`](selection.i64.mask-low.c17.9e049b3c.c) | type=i64, pattern=mask-low | c17 | `20` |
| [`selection.i64.max.c17.416fe7ec`](selection.i64.max.c17.416fe7ec.c) | type=i64, pattern=max | c17 | `20` |
| [`selection.i64.min.c17.374ab209`](selection.i64.min.c17.374ab209.c) | type=i64, pattern=min | c17 | `12` |
| [`selection.i64.multiply-add.c17.89d17785`](selection.i64.multiply-add.c17.89d17785.c) | type=i64, pattern=multiply-add | c17 | `340` |
| [`selection.i64.negate-add.c17.a7e58bed`](selection.i64.negate-add.c17.a7e58bed.c) | type=i64, pattern=negate-add | c17 | `80` |
| [`selection.i64.round-down.c17.6891b4b2`](selection.i64.round-down.c17.6891b4b2.c) | type=i64, pattern=round-down | c17 | `16` |
| [`selection.i64.shift-add.c17.2c7795d5`](selection.i64.shift-add.c17.2c7795d5.c) | type=i64, pattern=shift-add | c17 | `172` |
| [`selection.i64.test-bit.c17.9a71aa79`](selection.i64.test-bit.c17.9a71aa79.c) | type=i64, pattern=test-bit | c17 | `1` |
| [`selection.u32.average.c17.029f7846`](selection.u32.average.c17.029f7846.c) | type=u32, pattern=average | c17 | `16` |
| [`selection.u32.compare-set.c17.1f5c9a33`](selection.u32.compare-set.c17.1f5c9a33.c) | type=u32, pattern=compare-set | c17 | `1` |
| [`selection.u32.mask-low.c17.a8ef5a23`](selection.u32.mask-low.c17.a8ef5a23.c) | type=u32, pattern=mask-low | c17 | `20` |
| [`selection.u32.max.c17.1e75b02b`](selection.u32.max.c17.1e75b02b.c) | type=u32, pattern=max | c17 | `20` |
| [`selection.u32.min.c17.92d075bd`](selection.u32.min.c17.92d075bd.c) | type=u32, pattern=min | c17 | `12` |
| [`selection.u32.multiply-add.c17.29106034`](selection.u32.multiply-add.c17.29106034.c) | type=u32, pattern=multiply-add | c17 | `340` |
| [`selection.u32.negate-add.c17.b94729fe`](selection.u32.negate-add.c17.b94729fe.c) | type=u32, pattern=negate-add | c17 | `80` |
| [`selection.u32.round-down.c17.f19b3b16`](selection.u32.round-down.c17.f19b3b16.c) | type=u32, pattern=round-down | c17 | `16` |
| [`selection.u32.shift-add.c17.7b96440c`](selection.u32.shift-add.c17.7b96440c.c) | type=u32, pattern=shift-add | c17 | `172` |
| [`selection.u32.test-bit.c17.985826c2`](selection.u32.test-bit.c17.985826c2.c) | type=u32, pattern=test-bit | c17 | `1` |
| [`selection.u64.average.c17.97b29e2e`](selection.u64.average.c17.97b29e2e.c) | type=u64, pattern=average | c17 | `16` |
| [`selection.u64.compare-set.c17.84975acc`](selection.u64.compare-set.c17.84975acc.c) | type=u64, pattern=compare-set | c17 | `1` |
| [`selection.u64.mask-low.c17.af320916`](selection.u64.mask-low.c17.af320916.c) | type=u64, pattern=mask-low | c17 | `20` |
| [`selection.u64.max.c17.11581b8c`](selection.u64.max.c17.11581b8c.c) | type=u64, pattern=max | c17 | `20` |
| [`selection.u64.min.c17.b0f6c646`](selection.u64.min.c17.b0f6c646.c) | type=u64, pattern=min | c17 | `12` |
| [`selection.u64.multiply-add.c17.c84c377d`](selection.u64.multiply-add.c17.c84c377d.c) | type=u64, pattern=multiply-add | c17 | `340` |
| [`selection.u64.negate-add.c17.86687118`](selection.u64.negate-add.c17.86687118.c) | type=u64, pattern=negate-add | c17 | `80` |
| [`selection.u64.round-down.c17.b9ad948f`](selection.u64.round-down.c17.b9ad948f.c) | type=u64, pattern=round-down | c17 | `16` |
| [`selection.u64.shift-add.c17.f966febf`](selection.u64.shift-add.c17.f966febf.c) | type=u64, pattern=shift-add | c17 | `172` |
| [`selection.u64.test-bit.c17.77c3482d`](selection.u64.test-bit.c17.77c3482d.c) | type=u64, pattern=test-bit | c17 | `1` |

