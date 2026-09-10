# short-circuit

collapsing the two branches of a logical operator into one. Part of the local phase of the M4 plan.

22 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`short-circuit.and-before-or.branch.c17.f650bb8b`](short-circuit.and-before-or.branch.c17.f650bb8b.c) | shape=and-before-or, form=branch | c17 | `47` |
| [`short-circuit.and-before-or.value.c17.c37965d1`](short-circuit.and-before-or.value.c17.c37965d1.c) | shape=and-before-or, form=value | c17 | `47` |
| [`short-circuit.calling-right.branch.c17.fedb94f0`](short-circuit.calling-right.branch.c17.fedb94f0.c) | shape=calling-right, form=branch | c17 | `64 128` |
| [`short-circuit.calling-right.value.c17.9ed7d9ca`](short-circuit.calling-right.value.c17.9ed7d9ca.c) | shape=calling-right, form=value | c17 | `64 128` |
| [`short-circuit.chain-of-three.branch.c17.3314cc65`](short-circuit.chain-of-three.branch.c17.3314cc65.c) | shape=chain-of-three, form=branch | c17 | `94` |
| [`short-circuit.chain-of-three.value.c17.1372c19a`](short-circuit.chain-of-three.value.c17.1372c19a.c) | shape=chain-of-three, form=value | c17 | `94` |
| [`short-circuit.cheap-and.branch.c17.04371bc4`](short-circuit.cheap-and.branch.c17.04371bc4.c) | shape=cheap-and, form=branch | c17 | `99` |
| [`short-circuit.cheap-and.value.c17.33cdf0fc`](short-circuit.cheap-and.value.c17.33cdf0fc.c) | shape=cheap-and, form=value | c17 | `99` |
| [`short-circuit.cheap-or.branch.c17.7572e355`](short-circuit.cheap-or.branch.c17.7572e355.c) | shape=cheap-or, form=branch | c17 | `105` |
| [`short-circuit.cheap-or.value.c17.92eedd81`](short-circuit.cheap-or.value.c17.92eedd81.c) | shape=cheap-or, form=value | c17 | `105` |
| [`short-circuit.costly-right.branch.c17.40db102c`](short-circuit.costly-right.branch.c17.40db102c.c) | shape=costly-right, form=branch | c17 | `67` |
| [`short-circuit.costly-right.value.c17.e04b6438`](short-circuit.costly-right.value.c17.e04b6438.c) | shape=costly-right, form=value | c17 | `67` |
| [`short-circuit.guarded-division.branch.c17.d738c252`](short-circuit.guarded-division.branch.c17.d738c252.c) | shape=guarded-division, form=branch | c17 | `224` |
| [`short-circuit.guarded-division.value.c17.29cb8bf8`](short-circuit.guarded-division.value.c17.29cb8bf8.c) | shape=guarded-division, form=value | c17 | `224` |
| [`short-circuit.guarded-index.branch.c17.3f7034ee`](short-circuit.guarded-index.branch.c17.3f7034ee.c) | shape=guarded-index, form=branch | c17 | `64` |
| [`short-circuit.guarded-index.value.c17.0ae73929`](short-circuit.guarded-index.value.c17.0ae73929.c) | shape=guarded-index, form=value | c17 | `64` |
| [`short-circuit.guarded-load.branch.c17.91898640`](short-circuit.guarded-load.branch.c17.91898640.c) | shape=guarded-load, form=branch | c17 | `96` |
| [`short-circuit.guarded-load.value.c17.c29e0f4c`](short-circuit.guarded-load.value.c17.c29e0f4c.c) | shape=guarded-load, form=value | c17 | `96` |
| [`short-circuit.or-before-and.branch.c17.b43ff126`](short-circuit.or-before-and.branch.c17.b43ff126.c) | shape=or-before-and, form=branch | c17 | `47` |
| [`short-circuit.or-before-and.value.c17.830d488d`](short-circuit.or-before-and.value.c17.830d488d.c) | shape=or-before-and, form=value | c17 | `47` |
| [`short-circuit.predictable-left.branch.c17.ba97160c`](short-circuit.predictable-left.branch.c17.ba97160c.c) | shape=predictable-left, form=branch | c17 | `125` |
| [`short-circuit.predictable-left.value.c17.858d5c9e`](short-circuit.predictable-left.value.c17.858d5c9e.c) | shape=predictable-left, form=value | c17 | `125` |

