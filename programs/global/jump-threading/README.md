# jump-threading

sending an edge that decides a branch below straight to its arm. Part of the global phase of the M4 plan.

5 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`jump-threading.empty-join.c17.64db4499`](jump-threading.empty-join.c17.64db4499.c) | shape=empty-join | c17 | `27590 5050 775 1010` |
| [`jump-threading.found-in-loop.c17.5a27a887`](jump-threading.found-in-loop.c17.5a27a887.c) | shape=found-in-loop | c17 | `13746 18894 460 1128` |
| [`jump-threading.read-below-long.c17.e6787a1c`](jump-threading.read-below-long.c17.e6787a1c.c) | shape=read-below-long | c17 | `80703676 17176878 775 1010` |
| [`jump-threading.read-below.c17.042405fe`](jump-threading.read-below.c17.042405fe.c) | shape=read-below | c17 | `82925 15251 775 1010` |
| [`jump-threading.tested-twice.c17.2cce3c70`](jump-threading.tested-twice.c17.2cce3c70.c) | shape=tested-twice | c17 | `27590 98176 775 310` |

