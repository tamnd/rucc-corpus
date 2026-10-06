# null-pointer-constant

__is_constexpr, is_const and the null pointer constants they rest on. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`null-pointer-constant.choose-expr.c17.b696768c`](null-pointer-constant.choose-expr.c17.b696768c.c) | shape=choose-expr | c17 | `599464357` |
| [`null-pointer-constant.compare.c17.324fe500`](null-pointer-constant.compare.c17.324fe500.c) | shape=compare | c17 | `3936235271` |
| [`null-pointer-constant.conditional-type.c17.b33903a7`](null-pointer-constant.conditional-type.c17.b33903a7.c) | shape=conditional-type | c17 | `34503` |
| [`null-pointer-constant.convert.c17.195a9318`](null-pointer-constant.convert.c17.195a9318.c) | shape=convert | c17 | `3319106094` |
| [`null-pointer-constant.is-const.c17.0749e5ff`](null-pointer-constant.is-const.c17.0749e5ff.c) | shape=is-const | c17 | `2515880359` |
| [`null-pointer-constant.is-constexpr.c17.b3a54e24`](null-pointer-constant.is-constexpr.c17.b3a54e24.c) | shape=is-constexpr | c17 | `991576615` |

