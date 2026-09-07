# function-purity

proving purity and using it at the call sites. Part of the interprocedural phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`function-purity.i32.declared-const.c17.25c9ab01`](function-purity.i32.declared-const.c17.25c9ab01.c) | type=i32, shape=declared-const | c17 | `60` |
| [`function-purity.i32.declared-pure.c17.0bd35ef7`](function-purity.i32.declared-pure.c17.0bd35ef7.c) | type=i32, shape=declared-pure | c17 | `60` |
| [`function-purity.i32.impure.c17.4b4e235b`](function-purity.i32.impure.c17.4b4e235b.c) | type=i32, shape=impure | c17 | `60 5` |
| [`function-purity.i32.inferred-const.c17.2b203045`](function-purity.i32.inferred-const.c17.2b203045.c) | type=i32, shape=inferred-const | c17 | `60` |
| [`function-purity.i32.inferred-pure.c17.0cc22cc6`](function-purity.i32.inferred-pure.c17.0cc22cc6.c) | type=i32, shape=inferred-pure | c17 | `60` |
| [`function-purity.i64.declared-const.c17.f1133e42`](function-purity.i64.declared-const.c17.f1133e42.c) | type=i64, shape=declared-const | c17 | `60` |
| [`function-purity.i64.declared-pure.c17.2ab8a650`](function-purity.i64.declared-pure.c17.2ab8a650.c) | type=i64, shape=declared-pure | c17 | `60` |
| [`function-purity.i64.impure.c17.cd4a39d3`](function-purity.i64.impure.c17.cd4a39d3.c) | type=i64, shape=impure | c17 | `60 5` |
| [`function-purity.i64.inferred-const.c17.ef8f94d2`](function-purity.i64.inferred-const.c17.ef8f94d2.c) | type=i64, shape=inferred-const | c17 | `60` |
| [`function-purity.i64.inferred-pure.c17.520a612d`](function-purity.i64.inferred-pure.c17.520a612d.c) | type=i64, shape=inferred-pure | c17 | `60` |
| [`function-purity.u32.declared-const.c17.ee35f0e7`](function-purity.u32.declared-const.c17.ee35f0e7.c) | type=u32, shape=declared-const | c17 | `60` |
| [`function-purity.u32.declared-pure.c17.928b02bd`](function-purity.u32.declared-pure.c17.928b02bd.c) | type=u32, shape=declared-pure | c17 | `60` |
| [`function-purity.u32.impure.c17.6736590f`](function-purity.u32.impure.c17.6736590f.c) | type=u32, shape=impure | c17 | `60 5` |
| [`function-purity.u32.inferred-const.c17.dcad2836`](function-purity.u32.inferred-const.c17.dcad2836.c) | type=u32, shape=inferred-const | c17 | `60` |
| [`function-purity.u32.inferred-pure.c17.aa2e6fa0`](function-purity.u32.inferred-pure.c17.aa2e6fa0.c) | type=u32, shape=inferred-pure | c17 | `60` |
| [`function-purity.u64.declared-const.c17.f5eb7de1`](function-purity.u64.declared-const.c17.f5eb7de1.c) | type=u64, shape=declared-const | c17 | `60` |
| [`function-purity.u64.declared-pure.c17.7b878484`](function-purity.u64.declared-pure.c17.7b878484.c) | type=u64, shape=declared-pure | c17 | `60` |
| [`function-purity.u64.impure.c17.d85ab196`](function-purity.u64.impure.c17.d85ab196.c) | type=u64, shape=impure | c17 | `60 5` |
| [`function-purity.u64.inferred-const.c17.36b54b84`](function-purity.u64.inferred-const.c17.36b54b84.c) | type=u64, shape=inferred-const | c17 | `60` |
| [`function-purity.u64.inferred-pure.c17.7e1a3655`](function-purity.u64.inferred-pure.c17.7e1a3655.c) | type=u64, shape=inferred-pure | c17 | `60` |

