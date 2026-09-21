# unused-returns

taking out a return value no call reads, and the result with it. Part of the interprocedural phase of the M4 plan.

36 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`unused-returns.i32.address-escapes.c17.bcf8d970`](unused-returns.i32.address-escapes.c17.bcf8d970.c) | type=i32, shape=address-escapes | c17 | `7 0` |
| [`unused-returns.i32.one-call-reads.c17.bb6431f9`](unused-returns.i32.one-call-reads.c17.bb6431f9.c) | type=i32, shape=one-call-reads | c17 | `7 9` |
| [`unused-returns.i32.one-helper-of-two.c17.42a48d13`](unused-returns.i32.one-helper-of-two.c17.42a48d13.c) | type=i32, shape=one-helper-of-two | c17 | `7 103` |
| [`unused-returns.i32.other-objects-call-it.c17.d6b31ae1`](unused-returns.i32.other-objects-call-it.c17.d6b31ae1.c) | type=i32, shape=other-objects-call-it | c17 | `7 0` |
| [`unused-returns.i32.read-in-a-condition.c17.93b626a4`](unused-returns.i32.read-in-a-condition.c17.93b626a4.c) | type=i32, shape=read-in-a-condition | c17 | `7 5` |
| [`unused-returns.i32.recursive.c17.d5ec3f4e`](unused-returns.i32.recursive.c17.d5ec3f4e.c) | type=i32, shape=recursive | c17 | `6 0` |
| [`unused-returns.i32.result-ignored.c17.8d4773a2`](unused-returns.i32.result-ignored.c17.8d4773a2.c) | type=i32, shape=result-ignored | c17 | `7 0` |
| [`unused-returns.i32.returns-a-pointer.c17.ef913479`](unused-returns.i32.returns-a-pointer.c17.ef913479.c) | type=i32, shape=returns-a-pointer | c17 | `2 0` |
| [`unused-returns.i32.through-a-helper.c17.5605ba7e`](unused-returns.i32.through-a-helper.c17.5605ba7e.c) | type=i32, shape=through-a-helper | c17 | `7 0` |
| [`unused-returns.i64.address-escapes.c17.58262755`](unused-returns.i64.address-escapes.c17.58262755.c) | type=i64, shape=address-escapes | c17 | `7 0` |
| [`unused-returns.i64.one-call-reads.c17.8d436245`](unused-returns.i64.one-call-reads.c17.8d436245.c) | type=i64, shape=one-call-reads | c17 | `7 9` |
| [`unused-returns.i64.one-helper-of-two.c17.d9a0d9be`](unused-returns.i64.one-helper-of-two.c17.d9a0d9be.c) | type=i64, shape=one-helper-of-two | c17 | `7 103` |
| [`unused-returns.i64.other-objects-call-it.c17.45123e39`](unused-returns.i64.other-objects-call-it.c17.45123e39.c) | type=i64, shape=other-objects-call-it | c17 | `7 0` |
| [`unused-returns.i64.read-in-a-condition.c17.f74d6c4d`](unused-returns.i64.read-in-a-condition.c17.f74d6c4d.c) | type=i64, shape=read-in-a-condition | c17 | `7 5` |
| [`unused-returns.i64.recursive.c17.f9917054`](unused-returns.i64.recursive.c17.f9917054.c) | type=i64, shape=recursive | c17 | `6 0` |
| [`unused-returns.i64.result-ignored.c17.cd7b4654`](unused-returns.i64.result-ignored.c17.cd7b4654.c) | type=i64, shape=result-ignored | c17 | `7 0` |
| [`unused-returns.i64.returns-a-pointer.c17.5a88baeb`](unused-returns.i64.returns-a-pointer.c17.5a88baeb.c) | type=i64, shape=returns-a-pointer | c17 | `2 0` |
| [`unused-returns.i64.through-a-helper.c17.210503d0`](unused-returns.i64.through-a-helper.c17.210503d0.c) | type=i64, shape=through-a-helper | c17 | `7 0` |
| [`unused-returns.u32.address-escapes.c17.d4a7ae85`](unused-returns.u32.address-escapes.c17.d4a7ae85.c) | type=u32, shape=address-escapes | c17 | `7 0` |
| [`unused-returns.u32.one-call-reads.c17.a92808ef`](unused-returns.u32.one-call-reads.c17.a92808ef.c) | type=u32, shape=one-call-reads | c17 | `7 9` |
| [`unused-returns.u32.one-helper-of-two.c17.06e1d072`](unused-returns.u32.one-helper-of-two.c17.06e1d072.c) | type=u32, shape=one-helper-of-two | c17 | `7 103` |
| [`unused-returns.u32.other-objects-call-it.c17.e757f748`](unused-returns.u32.other-objects-call-it.c17.e757f748.c) | type=u32, shape=other-objects-call-it | c17 | `7 0` |
| [`unused-returns.u32.read-in-a-condition.c17.6b5e1c6c`](unused-returns.u32.read-in-a-condition.c17.6b5e1c6c.c) | type=u32, shape=read-in-a-condition | c17 | `7 5` |
| [`unused-returns.u32.recursive.c17.abe25e94`](unused-returns.u32.recursive.c17.abe25e94.c) | type=u32, shape=recursive | c17 | `6 0` |
| [`unused-returns.u32.result-ignored.c17.fcc9d97a`](unused-returns.u32.result-ignored.c17.fcc9d97a.c) | type=u32, shape=result-ignored | c17 | `7 0` |
| [`unused-returns.u32.returns-a-pointer.c17.7d002669`](unused-returns.u32.returns-a-pointer.c17.7d002669.c) | type=u32, shape=returns-a-pointer | c17 | `2 0` |
| [`unused-returns.u32.through-a-helper.c17.0e5630b0`](unused-returns.u32.through-a-helper.c17.0e5630b0.c) | type=u32, shape=through-a-helper | c17 | `7 0` |
| [`unused-returns.u64.address-escapes.c17.d3ecfb0c`](unused-returns.u64.address-escapes.c17.d3ecfb0c.c) | type=u64, shape=address-escapes | c17 | `7 0` |
| [`unused-returns.u64.one-call-reads.c17.9fb92942`](unused-returns.u64.one-call-reads.c17.9fb92942.c) | type=u64, shape=one-call-reads | c17 | `7 9` |
| [`unused-returns.u64.one-helper-of-two.c17.7507a072`](unused-returns.u64.one-helper-of-two.c17.7507a072.c) | type=u64, shape=one-helper-of-two | c17 | `7 103` |
| [`unused-returns.u64.other-objects-call-it.c17.165f80ff`](unused-returns.u64.other-objects-call-it.c17.165f80ff.c) | type=u64, shape=other-objects-call-it | c17 | `7 0` |
| [`unused-returns.u64.read-in-a-condition.c17.caa561e6`](unused-returns.u64.read-in-a-condition.c17.caa561e6.c) | type=u64, shape=read-in-a-condition | c17 | `7 5` |
| [`unused-returns.u64.recursive.c17.7b11c12b`](unused-returns.u64.recursive.c17.7b11c12b.c) | type=u64, shape=recursive | c17 | `6 0` |
| [`unused-returns.u64.result-ignored.c17.08991317`](unused-returns.u64.result-ignored.c17.08991317.c) | type=u64, shape=result-ignored | c17 | `7 0` |
| [`unused-returns.u64.returns-a-pointer.c17.1c153118`](unused-returns.u64.returns-a-pointer.c17.1c153118.c) | type=u64, shape=returns-a-pointer | c17 | `2 0` |
| [`unused-returns.u64.through-a-helper.c17.10d86287`](unused-returns.u64.through-a-helper.c17.10d86287.c) | type=u64, shape=through-a-helper | c17 | `7 0` |

