# bit-liveness

a widening whose upper bits nothing reads, across a block boundary. Part of the backend phase of the M4 plan.

28 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`bit-liveness.i16.both-arms.c17.afdf1834`](bit-liveness.i16.both-arms.c17.afdf1834.c) | type=i16, shape=both-arms | c17 | `40` |
| [`bit-liveness.i16.chain-of-widths.c17.8fd6ae97`](bit-liveness.i16.chain-of-widths.c17.8fd6ae97.c) | type=i16, shape=chain-of-widths | c17 | `40` |
| [`bit-liveness.i16.in-a-loop.c17.da3d2bc4`](bit-liveness.i16.in-a-loop.c17.da3d2bc4.c) | type=i16, shape=in-a-loop | c17 | `15` |
| [`bit-liveness.i16.in-another-block.c17.e68a2076`](bit-liveness.i16.in-another-block.c17.e68a2076.c) | type=i16, shape=in-another-block | c17 | `40` |
| [`bit-liveness.i16.narrowed-from-a-double.c17.4b416c34`](bit-liveness.i16.narrowed-from-a-double.c17.4b416c34.c) | type=i16, shape=narrowed-from-a-double | c17 | `41` |
| [`bit-liveness.i16.stored-narrow.c17.9606fc69`](bit-liveness.i16.stored-narrow.c17.9606fc69.c) | type=i16, shape=stored-narrow | c17 | `40` |
| [`bit-liveness.i16.upper-bits-read.c17.eae9fdc3`](bit-liveness.i16.upper-bits-read.c17.eae9fdc3.c) | type=i16, shape=upper-bits-read | c17 | `-55` |
| [`bit-liveness.i8.both-arms.c17.8b535399`](bit-liveness.i8.both-arms.c17.8b535399.c) | type=i8, shape=both-arms | c17 | `40` |
| [`bit-liveness.i8.chain-of-widths.c17.27446065`](bit-liveness.i8.chain-of-widths.c17.27446065.c) | type=i8, shape=chain-of-widths | c17 | `40` |
| [`bit-liveness.i8.in-a-loop.c17.31783309`](bit-liveness.i8.in-a-loop.c17.31783309.c) | type=i8, shape=in-a-loop | c17 | `15` |
| [`bit-liveness.i8.in-another-block.c17.323db93d`](bit-liveness.i8.in-another-block.c17.323db93d.c) | type=i8, shape=in-another-block | c17 | `40` |
| [`bit-liveness.i8.narrowed-from-a-double.c17.3f76aae2`](bit-liveness.i8.narrowed-from-a-double.c17.3f76aae2.c) | type=i8, shape=narrowed-from-a-double | c17 | `41` |
| [`bit-liveness.i8.stored-narrow.c17.c9603208`](bit-liveness.i8.stored-narrow.c17.c9603208.c) | type=i8, shape=stored-narrow | c17 | `40` |
| [`bit-liveness.i8.upper-bits-read.c17.d58c7f09`](bit-liveness.i8.upper-bits-read.c17.d58c7f09.c) | type=i8, shape=upper-bits-read | c17 | `-55` |
| [`bit-liveness.u16.both-arms.c17.f793356c`](bit-liveness.u16.both-arms.c17.f793356c.c) | type=u16, shape=both-arms | c17 | `40` |
| [`bit-liveness.u16.chain-of-widths.c17.d0ca25ae`](bit-liveness.u16.chain-of-widths.c17.d0ca25ae.c) | type=u16, shape=chain-of-widths | c17 | `40` |
| [`bit-liveness.u16.in-a-loop.c17.3976d2dc`](bit-liveness.u16.in-a-loop.c17.3976d2dc.c) | type=u16, shape=in-a-loop | c17 | `15` |
| [`bit-liveness.u16.in-another-block.c17.4ca68dd3`](bit-liveness.u16.in-another-block.c17.4ca68dd3.c) | type=u16, shape=in-another-block | c17 | `40` |
| [`bit-liveness.u16.narrowed-from-a-double.c17.844e4b72`](bit-liveness.u16.narrowed-from-a-double.c17.844e4b72.c) | type=u16, shape=narrowed-from-a-double | c17 | `41` |
| [`bit-liveness.u16.stored-narrow.c17.bdff292e`](bit-liveness.u16.stored-narrow.c17.bdff292e.c) | type=u16, shape=stored-narrow | c17 | `40` |
| [`bit-liveness.u16.upper-bits-read.c17.969727eb`](bit-liveness.u16.upper-bits-read.c17.969727eb.c) | type=u16, shape=upper-bits-read | c17 | `201` |
| [`bit-liveness.u8.both-arms.c17.bad345e9`](bit-liveness.u8.both-arms.c17.bad345e9.c) | type=u8, shape=both-arms | c17 | `40` |
| [`bit-liveness.u8.chain-of-widths.c17.77fffebd`](bit-liveness.u8.chain-of-widths.c17.77fffebd.c) | type=u8, shape=chain-of-widths | c17 | `40` |
| [`bit-liveness.u8.in-a-loop.c17.4a55d8ba`](bit-liveness.u8.in-a-loop.c17.4a55d8ba.c) | type=u8, shape=in-a-loop | c17 | `15` |
| [`bit-liveness.u8.in-another-block.c17.56955c58`](bit-liveness.u8.in-another-block.c17.56955c58.c) | type=u8, shape=in-another-block | c17 | `40` |
| [`bit-liveness.u8.narrowed-from-a-double.c17.a03b0c86`](bit-liveness.u8.narrowed-from-a-double.c17.a03b0c86.c) | type=u8, shape=narrowed-from-a-double | c17 | `41` |
| [`bit-liveness.u8.stored-narrow.c17.fe7d50d2`](bit-liveness.u8.stored-narrow.c17.fe7d50d2.c) | type=u8, shape=stored-narrow | c17 | `40` |
| [`bit-liveness.u8.upper-bits-read.c17.65888582`](bit-liveness.u8.upper-bits-read.c17.65888582.c) | type=u8, shape=upper-bits-read | c17 | `201` |

