# constant-args

specializing a function to a constant argument. Part of the interprocedural phase of the M4 plan.

36 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`constant-args.i32.address-escapes.c17.b18cf7b6`](constant-args.i32.address-escapes.c17.b18cf7b6.c) | type=i32, shape=address-escapes | c17 | `42` |
| [`constant-args.i32.all-sites-agree.c17.ba3f81da`](constant-args.i32.all-sites-agree.c17.ba3f81da.c) | type=i32, shape=all-sites-agree | c17 | `90` |
| [`constant-args.i32.handed-on.c17.f15a5c74`](constant-args.i32.handed-on.c17.f15a5c74.c) | type=i32, shape=handed-on | c17 | `60` |
| [`constant-args.i32.one-more-level.c17.ba5f1bf2`](constant-args.i32.one-more-level.c17.ba5f1bf2.c) | type=i32, shape=one-more-level | c17 | `60` |
| [`constant-args.i32.one-site.c17.89238871`](constant-args.i32.one-site.c17.89238871.c) | type=i32, shape=one-site | c17 | `30` |
| [`constant-args.i32.other-objects-call-it.c17.63ce23f8`](constant-args.i32.other-objects-call-it.c17.63ce23f8.c) | type=i32, shape=other-objects-call-it | c17 | `60` |
| [`constant-args.i32.recursive.c17.aceb03ef`](constant-args.i32.recursive.c17.aceb03ef.c) | type=i32, shape=recursive | c17 | `60` |
| [`constant-args.i32.sites-disagree.c17.22baf9b3`](constant-args.i32.sites-disagree.c17.22baf9b3.c) | type=i32, shape=sites-disagree | c17 | `72` |
| [`constant-args.i32.through-varargs.c17.6bd0bf6e`](constant-args.i32.through-varargs.c17.6bd0bf6e.c) | type=i32, shape=through-varargs | c17 | `60` |
| [`constant-args.i64.address-escapes.c17.98a3f00d`](constant-args.i64.address-escapes.c17.98a3f00d.c) | type=i64, shape=address-escapes | c17 | `42` |
| [`constant-args.i64.all-sites-agree.c17.74064c85`](constant-args.i64.all-sites-agree.c17.74064c85.c) | type=i64, shape=all-sites-agree | c17 | `90` |
| [`constant-args.i64.handed-on.c17.efff917f`](constant-args.i64.handed-on.c17.efff917f.c) | type=i64, shape=handed-on | c17 | `60` |
| [`constant-args.i64.one-more-level.c17.893d6460`](constant-args.i64.one-more-level.c17.893d6460.c) | type=i64, shape=one-more-level | c17 | `60` |
| [`constant-args.i64.one-site.c17.2749e65c`](constant-args.i64.one-site.c17.2749e65c.c) | type=i64, shape=one-site | c17 | `30` |
| [`constant-args.i64.other-objects-call-it.c17.c3373420`](constant-args.i64.other-objects-call-it.c17.c3373420.c) | type=i64, shape=other-objects-call-it | c17 | `60` |
| [`constant-args.i64.recursive.c17.1a24bca1`](constant-args.i64.recursive.c17.1a24bca1.c) | type=i64, shape=recursive | c17 | `60` |
| [`constant-args.i64.sites-disagree.c17.16ec79b9`](constant-args.i64.sites-disagree.c17.16ec79b9.c) | type=i64, shape=sites-disagree | c17 | `72` |
| [`constant-args.i64.through-varargs.c17.b07197a3`](constant-args.i64.through-varargs.c17.b07197a3.c) | type=i64, shape=through-varargs | c17 | `60` |
| [`constant-args.u32.address-escapes.c17.ba20b293`](constant-args.u32.address-escapes.c17.ba20b293.c) | type=u32, shape=address-escapes | c17 | `42` |
| [`constant-args.u32.all-sites-agree.c17.d48e717f`](constant-args.u32.all-sites-agree.c17.d48e717f.c) | type=u32, shape=all-sites-agree | c17 | `90` |
| [`constant-args.u32.handed-on.c17.c1ee8b27`](constant-args.u32.handed-on.c17.c1ee8b27.c) | type=u32, shape=handed-on | c17 | `60` |
| [`constant-args.u32.one-more-level.c17.2b38d319`](constant-args.u32.one-more-level.c17.2b38d319.c) | type=u32, shape=one-more-level | c17 | `60` |
| [`constant-args.u32.one-site.c17.6276e18c`](constant-args.u32.one-site.c17.6276e18c.c) | type=u32, shape=one-site | c17 | `30` |
| [`constant-args.u32.other-objects-call-it.c17.fe7af459`](constant-args.u32.other-objects-call-it.c17.fe7af459.c) | type=u32, shape=other-objects-call-it | c17 | `60` |
| [`constant-args.u32.recursive.c17.7d2f0f95`](constant-args.u32.recursive.c17.7d2f0f95.c) | type=u32, shape=recursive | c17 | `60` |
| [`constant-args.u32.sites-disagree.c17.124c88ef`](constant-args.u32.sites-disagree.c17.124c88ef.c) | type=u32, shape=sites-disagree | c17 | `72` |
| [`constant-args.u32.through-varargs.c17.52081925`](constant-args.u32.through-varargs.c17.52081925.c) | type=u32, shape=through-varargs | c17 | `60` |
| [`constant-args.u64.address-escapes.c17.b8768fe3`](constant-args.u64.address-escapes.c17.b8768fe3.c) | type=u64, shape=address-escapes | c17 | `42` |
| [`constant-args.u64.all-sites-agree.c17.eb58afce`](constant-args.u64.all-sites-agree.c17.eb58afce.c) | type=u64, shape=all-sites-agree | c17 | `90` |
| [`constant-args.u64.handed-on.c17.da8ba0b1`](constant-args.u64.handed-on.c17.da8ba0b1.c) | type=u64, shape=handed-on | c17 | `60` |
| [`constant-args.u64.one-more-level.c17.4354cf53`](constant-args.u64.one-more-level.c17.4354cf53.c) | type=u64, shape=one-more-level | c17 | `60` |
| [`constant-args.u64.one-site.c17.507e7404`](constant-args.u64.one-site.c17.507e7404.c) | type=u64, shape=one-site | c17 | `30` |
| [`constant-args.u64.other-objects-call-it.c17.5b0d27f8`](constant-args.u64.other-objects-call-it.c17.5b0d27f8.c) | type=u64, shape=other-objects-call-it | c17 | `60` |
| [`constant-args.u64.recursive.c17.bba1bad1`](constant-args.u64.recursive.c17.bba1bad1.c) | type=u64, shape=recursive | c17 | `60` |
| [`constant-args.u64.sites-disagree.c17.be50775e`](constant-args.u64.sites-disagree.c17.be50775e.c) | type=u64, shape=sites-disagree | c17 | `72` |
| [`constant-args.u64.through-varargs.c17.7b45c5b8`](constant-args.u64.through-varargs.c17.7b45c5b8.c) | type=u64, shape=through-varargs | c17 | `60` |

