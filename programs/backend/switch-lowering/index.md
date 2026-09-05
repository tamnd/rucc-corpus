# switch-lowering

choosing how to lower a switch. Part of the backend phase of the M4 plan.

9 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`switch-lowering.dense.3.c17.6d522d5d`](switch-lowering.dense.3.c17.6d522d5d.c) | density=dense, labels=3 | c17 | `6` |
| [`switch-lowering.dense.40.c17.85e83015`](switch-lowering.dense.40.c17.85e83015.c) | density=dense, labels=40 | c17 | `820` |
| [`switch-lowering.dense.8.c17.35614fce`](switch-lowering.dense.8.c17.35614fce.c) | density=dense, labels=8 | c17 | `36` |
| [`switch-lowering.sparse.3.c17.4edd9173`](switch-lowering.sparse.3.c17.4edd9173.c) | density=sparse, labels=3 | c17 | `6` |
| [`switch-lowering.sparse.40.c17.1621aa90`](switch-lowering.sparse.40.c17.1621aa90.c) | density=sparse, labels=40 | c17 | `820` |
| [`switch-lowering.sparse.8.c17.a5a20337`](switch-lowering.sparse.8.c17.a5a20337.c) | density=sparse, labels=8 | c17 | `36` |
| [`switch-lowering.very-sparse.3.c17.50ddccc0`](switch-lowering.very-sparse.3.c17.50ddccc0.c) | density=very-sparse, labels=3 | c17 | `6` |
| [`switch-lowering.very-sparse.40.c17.95a1d82e`](switch-lowering.very-sparse.40.c17.95a1d82e.c) | density=very-sparse, labels=40 | c17 | `820` |
| [`switch-lowering.very-sparse.8.c17.67a8d965`](switch-lowering.very-sparse.8.c17.67a8d965.c) | density=very-sparse, labels=8 | c17 | `36` |

