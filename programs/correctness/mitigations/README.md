# mitigations

programs built with the kernel's thunk, trap and landing pad flags. Part of the correctness phase of the M4 plan.

25 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`mitigations.ibt.early-return.c17.e51b8ea1`](mitigations.ibt.early-return.c17.e51b8ea1.c) | `-fcf-protection=branch` | mitigation=ibt, shape=early-return | c17 | `2928` |
| [`mitigations.ibt.indirect-call.c17.3344dcbb`](mitigations.ibt.indirect-call.c17.3344dcbb.c) | `-fcf-protection=branch` | mitigation=ibt, shape=indirect-call | c17 | `3407135943` |
| [`mitigations.ibt.indirect-tail.c17.03b5d3bf`](mitigations.ibt.indirect-tail.c17.03b5d3bf.c) | `-fcf-protection=branch` | mitigation=ibt, shape=indirect-tail | c17 | `1834505909` |
| [`mitigations.ibt.ops-struct.c17.285d00c0`](mitigations.ibt.ops-struct.c17.285d00c0.c) | `-fcf-protection=branch` | mitigation=ibt, shape=ops-struct | c17 | `1830326174` |
| [`mitigations.ibt.switch.c17.d5e3a807`](mitigations.ibt.switch.c17.d5e3a807.c) | `-fcf-protection=branch` | mitigation=ibt, shape=switch | c17 | `52525655` |
| [`mitigations.kernel.early-return.c17.34ea0103`](mitigations.kernel.early-return.c17.34ea0103.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` `-mfunction-return=thunk-extern` `-mharden-sls=all` `-fcf-protection=branch` `-fno-jump-tables` | mitigation=kernel, shape=early-return | c17 | `2928` |
| [`mitigations.kernel.indirect-call.c17.13c82dab`](mitigations.kernel.indirect-call.c17.13c82dab.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` `-mfunction-return=thunk-extern` `-mharden-sls=all` `-fcf-protection=branch` `-fno-jump-tables` | mitigation=kernel, shape=indirect-call | c17 | `3407135943` |
| [`mitigations.kernel.indirect-tail.c17.a86be44b`](mitigations.kernel.indirect-tail.c17.a86be44b.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` `-mfunction-return=thunk-extern` `-mharden-sls=all` `-fcf-protection=branch` `-fno-jump-tables` | mitigation=kernel, shape=indirect-tail | c17 | `1834505909` |
| [`mitigations.kernel.ops-struct.c17.13665eca`](mitigations.kernel.ops-struct.c17.13665eca.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` `-mfunction-return=thunk-extern` `-mharden-sls=all` `-fcf-protection=branch` `-fno-jump-tables` | mitigation=kernel, shape=ops-struct | c17 | `1830326174` |
| [`mitigations.kernel.switch.c17.3badf797`](mitigations.kernel.switch.c17.3badf797.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` `-mfunction-return=thunk-extern` `-mharden-sls=all` `-fcf-protection=branch` `-fno-jump-tables` | mitigation=kernel, shape=switch | c17 | `52525655` |
| [`mitigations.rethunk.early-return.c17.90207564`](mitigations.rethunk.early-return.c17.90207564.c) | `-mfunction-return=thunk-extern` | mitigation=rethunk, shape=early-return | c17 | `2928` |
| [`mitigations.rethunk.indirect-call.c17.898a56f6`](mitigations.rethunk.indirect-call.c17.898a56f6.c) | `-mfunction-return=thunk-extern` | mitigation=rethunk, shape=indirect-call | c17 | `3407135943` |
| [`mitigations.rethunk.indirect-tail.c17.e8bfbe45`](mitigations.rethunk.indirect-tail.c17.e8bfbe45.c) | `-mfunction-return=thunk-extern` | mitigation=rethunk, shape=indirect-tail | c17 | `1834505909` |
| [`mitigations.rethunk.ops-struct.c17.197898c3`](mitigations.rethunk.ops-struct.c17.197898c3.c) | `-mfunction-return=thunk-extern` | mitigation=rethunk, shape=ops-struct | c17 | `1830326174` |
| [`mitigations.rethunk.switch.c17.e80371db`](mitigations.rethunk.switch.c17.e80371db.c) | `-mfunction-return=thunk-extern` | mitigation=rethunk, shape=switch | c17 | `52525655` |
| [`mitigations.retpoline.early-return.c17.287f847c`](mitigations.retpoline.early-return.c17.287f847c.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` | mitigation=retpoline, shape=early-return | c17 | `2928` |
| [`mitigations.retpoline.indirect-call.c17.8bb886dd`](mitigations.retpoline.indirect-call.c17.8bb886dd.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` | mitigation=retpoline, shape=indirect-call | c17 | `3407135943` |
| [`mitigations.retpoline.indirect-tail.c17.6cc6f440`](mitigations.retpoline.indirect-tail.c17.6cc6f440.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` | mitigation=retpoline, shape=indirect-tail | c17 | `1834505909` |
| [`mitigations.retpoline.ops-struct.c17.b476b4f8`](mitigations.retpoline.ops-struct.c17.b476b4f8.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` | mitigation=retpoline, shape=ops-struct | c17 | `1830326174` |
| [`mitigations.retpoline.switch.c17.6ea4a885`](mitigations.retpoline.switch.c17.6ea4a885.c) | `-mindirect-branch=thunk-extern` `-mindirect-branch-register` | mitigation=retpoline, shape=switch | c17 | `52525655` |
| [`mitigations.sls.early-return.c17.27659525`](mitigations.sls.early-return.c17.27659525.c) | `-mharden-sls=all` | mitigation=sls, shape=early-return | c17 | `2928` |
| [`mitigations.sls.indirect-call.c17.8d2c348a`](mitigations.sls.indirect-call.c17.8d2c348a.c) | `-mharden-sls=all` | mitigation=sls, shape=indirect-call | c17 | `3407135943` |
| [`mitigations.sls.indirect-tail.c17.8a87e29f`](mitigations.sls.indirect-tail.c17.8a87e29f.c) | `-mharden-sls=all` | mitigation=sls, shape=indirect-tail | c17 | `1834505909` |
| [`mitigations.sls.ops-struct.c17.56c4865f`](mitigations.sls.ops-struct.c17.56c4865f.c) | `-mharden-sls=all` | mitigation=sls, shape=ops-struct | c17 | `1830326174` |
| [`mitigations.sls.switch.c17.11ea7496`](mitigations.sls.switch.c17.11ea7496.c) | `-mharden-sls=all` | mitigation=sls, shape=switch | c17 | `52525655` |

