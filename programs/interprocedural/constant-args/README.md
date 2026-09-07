# constant-args

specializing a function to a constant argument. Part of the interprocedural phase of the M4 plan.

12 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`constant-args.i32.all-sites-agree.c17.ba3f81da`](constant-args.i32.all-sites-agree.c17.ba3f81da.c) | type=i32, shape=all-sites-agree | c17 | `90` |
| [`constant-args.i32.one-site.c17.89238871`](constant-args.i32.one-site.c17.89238871.c) | type=i32, shape=one-site | c17 | `30` |
| [`constant-args.i32.sites-disagree.c17.22baf9b3`](constant-args.i32.sites-disagree.c17.22baf9b3.c) | type=i32, shape=sites-disagree | c17 | `72` |
| [`constant-args.i64.all-sites-agree.c17.74064c85`](constant-args.i64.all-sites-agree.c17.74064c85.c) | type=i64, shape=all-sites-agree | c17 | `90` |
| [`constant-args.i64.one-site.c17.2749e65c`](constant-args.i64.one-site.c17.2749e65c.c) | type=i64, shape=one-site | c17 | `30` |
| [`constant-args.i64.sites-disagree.c17.16ec79b9`](constant-args.i64.sites-disagree.c17.16ec79b9.c) | type=i64, shape=sites-disagree | c17 | `72` |
| [`constant-args.u32.all-sites-agree.c17.d48e717f`](constant-args.u32.all-sites-agree.c17.d48e717f.c) | type=u32, shape=all-sites-agree | c17 | `90` |
| [`constant-args.u32.one-site.c17.6276e18c`](constant-args.u32.one-site.c17.6276e18c.c) | type=u32, shape=one-site | c17 | `30` |
| [`constant-args.u32.sites-disagree.c17.124c88ef`](constant-args.u32.sites-disagree.c17.124c88ef.c) | type=u32, shape=sites-disagree | c17 | `72` |
| [`constant-args.u64.all-sites-agree.c17.eb58afce`](constant-args.u64.all-sites-agree.c17.eb58afce.c) | type=u64, shape=all-sites-agree | c17 | `90` |
| [`constant-args.u64.one-site.c17.507e7404`](constant-args.u64.one-site.c17.507e7404.c) | type=u64, shape=one-site | c17 | `30` |
| [`constant-args.u64.sites-disagree.c17.be50775e`](constant-args.u64.sites-disagree.c17.be50775e.c) | type=u64, shape=sites-disagree | c17 | `72` |

