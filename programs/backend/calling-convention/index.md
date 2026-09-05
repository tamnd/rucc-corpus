# calling-convention

deciding what a call saves and restores. Part of the backend phase of the M4 plan.

10 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`calling-convention.1.integer.c17.a3085d8f`](calling-convention.1.integer.c17.a3085d8f.c) | count=1, kind=integer | c17 | `1` |
| [`calling-convention.1.one-field.c17.c28c5955`](calling-convention.1.one-field.c17.c28c5955.c) | count=1, kind=one-field | c17 | `1` |
| [`calling-convention.16.integer.c17.05579518`](calling-convention.16.integer.c17.05579518.c) | count=16, kind=integer | c17 | `136` |
| [`calling-convention.16.sixteen-fields.c17.f9f2c136`](calling-convention.16.sixteen-fields.c17.f9f2c136.c) | count=16, kind=sixteen-fields | c17 | `136` |
| [`calling-convention.2.two-fields.c17.b9314f2a`](calling-convention.2.two-fields.c17.b9314f2a.c) | count=2, kind=two-fields | c17 | `3` |
| [`calling-convention.4.four-fields.c17.f50893d4`](calling-convention.4.four-fields.c17.f50893d4.c) | count=4, kind=four-fields | c17 | `10` |
| [`calling-convention.4.integer.c17.371cc5ba`](calling-convention.4.integer.c17.371cc5ba.c) | count=4, kind=integer | c17 | `10` |
| [`calling-convention.6.integer.c17.1b6001db`](calling-convention.6.integer.c17.1b6001db.c) | count=6, kind=integer | c17 | `21` |
| [`calling-convention.6.varargs.c17.0eff369b`](calling-convention.6.varargs.c17.0eff369b.c) | count=6, kind=varargs | c17 | `21` |
| [`calling-convention.8.integer.c17.d47f206a`](calling-convention.8.integer.c17.d47f206a.c) | count=8, kind=integer | c17 | `36` |

