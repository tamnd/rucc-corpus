# unused-params

taking out a parameter nothing reads, and the argument with it. Part of the interprocedural phase of the M4 plan.

44 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`unused-params.i32.address-escapes.c17.0ba7b05c`](unused-params.i32.address-escapes.c17.0ba7b05c.c) | type=i32, shape=address-escapes | c17 | `8` |
| [`unused-params.i32.all-unread.c17.9bfdab0c`](unused-params.i32.all-unread.c17.9bfdab0c.c) | type=i32, shape=all-unread | c17 | `14` |
| [`unused-params.i32.argument-has-a-side-effect.c17.eb9a75e4`](unused-params.i32.argument-has-a-side-effect.c17.eb9a75e4.c) | type=i32, shape=argument-has-a-side-effect | c17 | `2 8` |
| [`unused-params.i32.argument-is-an-address.c17.9414476b`](unused-params.i32.argument-is-an-address.c17.9414476b.c) | type=i32, shape=argument-is-an-address | c17 | `9 4` |
| [`unused-params.i32.constant-then-unread.c17.f92a694d`](unused-params.i32.constant-then-unread.c17.f92a694d.c) | type=i32, shape=constant-then-unread | c17 | `60` |
| [`unused-params.i32.middle-unread.c17.cae7efb2`](unused-params.i32.middle-unread.c17.cae7efb2.c) | type=i32, shape=middle-unread | c17 | `69` |
| [`unused-params.i32.one-unread.c17.2394f025`](unused-params.i32.one-unread.c17.2394f025.c) | type=i32, shape=one-unread | c17 | `8` |
| [`unused-params.i32.other-objects-call-it.c17.28374ece`](unused-params.i32.other-objects-call-it.c17.28374ece.c) | type=i32, shape=other-objects-call-it | c17 | `8` |
| [`unused-params.i32.recursive-only.c17.aae2d0ca`](unused-params.i32.recursive-only.c17.aae2d0ca.c) | type=i32, shape=recursive-only | c17 | `6` |
| [`unused-params.i32.transitive-sum.c17.d2492471`](unused-params.i32.transitive-sum.c17.d2492471.c) | type=i32, shape=transitive-sum | c17 | `8` |
| [`unused-params.i32.variadic.c17.f6126a5d`](unused-params.i32.variadic.c17.f6126a5d.c) | type=i32, shape=variadic | c17 | `8` |
| [`unused-params.i64.address-escapes.c17.9dd9952f`](unused-params.i64.address-escapes.c17.9dd9952f.c) | type=i64, shape=address-escapes | c17 | `8` |
| [`unused-params.i64.all-unread.c17.83b6b28f`](unused-params.i64.all-unread.c17.83b6b28f.c) | type=i64, shape=all-unread | c17 | `14` |
| [`unused-params.i64.argument-has-a-side-effect.c17.84063ca0`](unused-params.i64.argument-has-a-side-effect.c17.84063ca0.c) | type=i64, shape=argument-has-a-side-effect | c17 | `2 8` |
| [`unused-params.i64.argument-is-an-address.c17.35029e20`](unused-params.i64.argument-is-an-address.c17.35029e20.c) | type=i64, shape=argument-is-an-address | c17 | `9 4` |
| [`unused-params.i64.constant-then-unread.c17.7a6954a5`](unused-params.i64.constant-then-unread.c17.7a6954a5.c) | type=i64, shape=constant-then-unread | c17 | `60` |
| [`unused-params.i64.middle-unread.c17.a6809e52`](unused-params.i64.middle-unread.c17.a6809e52.c) | type=i64, shape=middle-unread | c17 | `69` |
| [`unused-params.i64.one-unread.c17.4459c3a1`](unused-params.i64.one-unread.c17.4459c3a1.c) | type=i64, shape=one-unread | c17 | `8` |
| [`unused-params.i64.other-objects-call-it.c17.646764d4`](unused-params.i64.other-objects-call-it.c17.646764d4.c) | type=i64, shape=other-objects-call-it | c17 | `8` |
| [`unused-params.i64.recursive-only.c17.46821f6b`](unused-params.i64.recursive-only.c17.46821f6b.c) | type=i64, shape=recursive-only | c17 | `6` |
| [`unused-params.i64.transitive-sum.c17.5d315a24`](unused-params.i64.transitive-sum.c17.5d315a24.c) | type=i64, shape=transitive-sum | c17 | `8` |
| [`unused-params.i64.variadic.c17.1dc196ba`](unused-params.i64.variadic.c17.1dc196ba.c) | type=i64, shape=variadic | c17 | `8` |
| [`unused-params.u32.address-escapes.c17.5718c064`](unused-params.u32.address-escapes.c17.5718c064.c) | type=u32, shape=address-escapes | c17 | `8` |
| [`unused-params.u32.all-unread.c17.500cb8a3`](unused-params.u32.all-unread.c17.500cb8a3.c) | type=u32, shape=all-unread | c17 | `14` |
| [`unused-params.u32.argument-has-a-side-effect.c17.5151059c`](unused-params.u32.argument-has-a-side-effect.c17.5151059c.c) | type=u32, shape=argument-has-a-side-effect | c17 | `2 8` |
| [`unused-params.u32.argument-is-an-address.c17.07029d1b`](unused-params.u32.argument-is-an-address.c17.07029d1b.c) | type=u32, shape=argument-is-an-address | c17 | `9 4` |
| [`unused-params.u32.constant-then-unread.c17.4d0f3ad6`](unused-params.u32.constant-then-unread.c17.4d0f3ad6.c) | type=u32, shape=constant-then-unread | c17 | `60` |
| [`unused-params.u32.middle-unread.c17.e21c3932`](unused-params.u32.middle-unread.c17.e21c3932.c) | type=u32, shape=middle-unread | c17 | `69` |
| [`unused-params.u32.one-unread.c17.ef8a3624`](unused-params.u32.one-unread.c17.ef8a3624.c) | type=u32, shape=one-unread | c17 | `8` |
| [`unused-params.u32.other-objects-call-it.c17.e2314221`](unused-params.u32.other-objects-call-it.c17.e2314221.c) | type=u32, shape=other-objects-call-it | c17 | `8` |
| [`unused-params.u32.recursive-only.c17.2b66252e`](unused-params.u32.recursive-only.c17.2b66252e.c) | type=u32, shape=recursive-only | c17 | `6` |
| [`unused-params.u32.transitive-sum.c17.f0804e3f`](unused-params.u32.transitive-sum.c17.f0804e3f.c) | type=u32, shape=transitive-sum | c17 | `8` |
| [`unused-params.u32.variadic.c17.043f10d4`](unused-params.u32.variadic.c17.043f10d4.c) | type=u32, shape=variadic | c17 | `8` |
| [`unused-params.u64.address-escapes.c17.894ad9ac`](unused-params.u64.address-escapes.c17.894ad9ac.c) | type=u64, shape=address-escapes | c17 | `8` |
| [`unused-params.u64.all-unread.c17.b407f3f0`](unused-params.u64.all-unread.c17.b407f3f0.c) | type=u64, shape=all-unread | c17 | `14` |
| [`unused-params.u64.argument-has-a-side-effect.c17.366741a2`](unused-params.u64.argument-has-a-side-effect.c17.366741a2.c) | type=u64, shape=argument-has-a-side-effect | c17 | `2 8` |
| [`unused-params.u64.argument-is-an-address.c17.db7cf0ea`](unused-params.u64.argument-is-an-address.c17.db7cf0ea.c) | type=u64, shape=argument-is-an-address | c17 | `9 4` |
| [`unused-params.u64.constant-then-unread.c17.f0baa7bd`](unused-params.u64.constant-then-unread.c17.f0baa7bd.c) | type=u64, shape=constant-then-unread | c17 | `60` |
| [`unused-params.u64.middle-unread.c17.ebc9824d`](unused-params.u64.middle-unread.c17.ebc9824d.c) | type=u64, shape=middle-unread | c17 | `69` |
| [`unused-params.u64.one-unread.c17.121b79e9`](unused-params.u64.one-unread.c17.121b79e9.c) | type=u64, shape=one-unread | c17 | `8` |
| [`unused-params.u64.other-objects-call-it.c17.2afb6556`](unused-params.u64.other-objects-call-it.c17.2afb6556.c) | type=u64, shape=other-objects-call-it | c17 | `8` |
| [`unused-params.u64.recursive-only.c17.70f17dc3`](unused-params.u64.recursive-only.c17.70f17dc3.c) | type=u64, shape=recursive-only | c17 | `6` |
| [`unused-params.u64.transitive-sum.c17.26ce8524`](unused-params.u64.transitive-sum.c17.26ce8524.c) | type=u64, shape=transitive-sum | c17 | `8` |
| [`unused-params.u64.variadic.c17.b79ca88b`](unused-params.u64.variadic.c17.b79ca88b.c) | type=u64, shape=variadic | c17 | `8` |

