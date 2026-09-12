# compare-elim

a comparison the instruction in front of it has already made. Part of the backend phase of the M4 plan.

26 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`compare-elim.i32.after-and.c17.d99551cf`](compare-elim.i32.after-and.c17.d99551cf.c) | type=i32, shape=after-and | c17 | `19` |
| [`compare-elim.i32.after-or.c17.d7aaf762`](compare-elim.i32.after-or.c17.d7aaf762.c) | type=i32, shape=after-or | c17 | `8` |
| [`compare-elim.i32.after-sub-ordered.c17.09981ca4`](compare-elim.i32.after-sub-ordered.c17.09981ca4.c) | type=i32, shape=after-sub-ordered | c17 | `9` |
| [`compare-elim.i32.after-sub.c17.5f5e2fed`](compare-elim.i32.after-sub.c17.5f5e2fed.c) | type=i32, shape=after-sub | c17 | `6` |
| [`compare-elim.i32.byte-and-branch.c17.221b5a90`](compare-elim.i32.byte-and-branch.c17.221b5a90.c) | type=i32, shape=byte-and-branch | c17 | `7` |
| [`compare-elim.i32.same-comparison.c17.17a823e8`](compare-elim.i32.same-comparison.c17.17a823e8.c) | type=i32, shape=same-comparison | c17 | `1` |
| [`compare-elim.i32.written-in-between.c17.ea67eca1`](compare-elim.i32.written-in-between.c17.ea67eca1.c) | type=i32, shape=written-in-between | c17 | `11` |
| [`compare-elim.i64.after-and.c17.6f7f9de6`](compare-elim.i64.after-and.c17.6f7f9de6.c) | type=i64, shape=after-and | c17 | `19` |
| [`compare-elim.i64.after-or.c17.2426907a`](compare-elim.i64.after-or.c17.2426907a.c) | type=i64, shape=after-or | c17 | `8` |
| [`compare-elim.i64.after-sub-ordered.c17.a641ace8`](compare-elim.i64.after-sub-ordered.c17.a641ace8.c) | type=i64, shape=after-sub-ordered | c17 | `9` |
| [`compare-elim.i64.after-sub.c17.13a05519`](compare-elim.i64.after-sub.c17.13a05519.c) | type=i64, shape=after-sub | c17 | `6` |
| [`compare-elim.i64.byte-and-branch.c17.7945bad6`](compare-elim.i64.byte-and-branch.c17.7945bad6.c) | type=i64, shape=byte-and-branch | c17 | `7` |
| [`compare-elim.i64.same-comparison.c17.aff4137f`](compare-elim.i64.same-comparison.c17.aff4137f.c) | type=i64, shape=same-comparison | c17 | `1` |
| [`compare-elim.i64.written-in-between.c17.a00486c6`](compare-elim.i64.written-in-between.c17.a00486c6.c) | type=i64, shape=written-in-between | c17 | `11` |
| [`compare-elim.u32.after-and.c17.e47b465b`](compare-elim.u32.after-and.c17.e47b465b.c) | type=u32, shape=after-and | c17 | `19` |
| [`compare-elim.u32.after-or.c17.e72a49c9`](compare-elim.u32.after-or.c17.e72a49c9.c) | type=u32, shape=after-or | c17 | `8` |
| [`compare-elim.u32.after-sub.c17.e2412cd0`](compare-elim.u32.after-sub.c17.e2412cd0.c) | type=u32, shape=after-sub | c17 | `6` |
| [`compare-elim.u32.byte-and-branch.c17.48fbe737`](compare-elim.u32.byte-and-branch.c17.48fbe737.c) | type=u32, shape=byte-and-branch | c17 | `7` |
| [`compare-elim.u32.same-comparison.c17.810c4c68`](compare-elim.u32.same-comparison.c17.810c4c68.c) | type=u32, shape=same-comparison | c17 | `1` |
| [`compare-elim.u32.written-in-between.c17.03cf6aa9`](compare-elim.u32.written-in-between.c17.03cf6aa9.c) | type=u32, shape=written-in-between | c17 | `11` |
| [`compare-elim.u64.after-and.c17.308b3cac`](compare-elim.u64.after-and.c17.308b3cac.c) | type=u64, shape=after-and | c17 | `19` |
| [`compare-elim.u64.after-or.c17.01a6b5e4`](compare-elim.u64.after-or.c17.01a6b5e4.c) | type=u64, shape=after-or | c17 | `8` |
| [`compare-elim.u64.after-sub.c17.93bf4f3b`](compare-elim.u64.after-sub.c17.93bf4f3b.c) | type=u64, shape=after-sub | c17 | `6` |
| [`compare-elim.u64.byte-and-branch.c17.71dcbe63`](compare-elim.u64.byte-and-branch.c17.71dcbe63.c) | type=u64, shape=byte-and-branch | c17 | `7` |
| [`compare-elim.u64.same-comparison.c17.2f91d874`](compare-elim.u64.same-comparison.c17.2f91d874.c) | type=u64, shape=same-comparison | c17 | `1` |
| [`compare-elim.u64.written-in-between.c17.fac1fac1`](compare-elim.u64.written-in-between.c17.fac1fac1.c) | type=u64, shape=written-in-between | c17 | `11` |

