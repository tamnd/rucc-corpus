# ifunc-static

FreeBSD ifunc resolvers in static and dynamic programs. Part of the correctness phase of the M4 plan.

10 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`ifunc-static.address.dynamic.c17.0db26930`](ifunc-static.address.dynamic.c17.0db26930.c) | nothing, it is one file | nothing extra | shape=address, link=dynamic | c17 | `1 1923506687 1 120` |
| [`ifunc-static.address.static.c17.258d43ba`](ifunc-static.address.static.c17.258d43ba.c) | nothing, it is one file | `-static` | shape=address, link=static | c17 | `1 1923506687 1 ...` and 3 more lines |
| [`ifunc-static.call.dynamic.c17.6118f843`](ifunc-static.call.dynamic.c17.6118f843.c) | nothing, it is one file | nothing extra | shape=call, link=dynamic | c17 | `1923506687 1 120` |
| [`ifunc-static.call.static.c17.603061be`](ifunc-static.call.static.c17.603061be.c) | nothing, it is one file | `-static` | shape=call, link=static | c17 | `1923506687 1 120 ...` and 2 more lines |
| [`ifunc-static.memcpy-family.dynamic.c17.d969a21a`](ifunc-static.memcpy-family.dynamic.c17.d969a21a.c) | nothing, it is one file | nothing extra | shape=memcpy-family, link=dynamic | c17 | `1923506687 16 48571 ...` and 3 more lines |
| [`ifunc-static.memcpy-family.static.c17.66dc0730`](ifunc-static.memcpy-family.static.c17.66dc0730.c) | nothing, it is one file | `-static` | shape=memcpy-family, link=static | c17 | `1923506687 16 48571 ...` and 5 more lines |
| [`ifunc-static.resolver-before-constructors.dynamic.c17.3daaa50f`](ifunc-static.resolver-before-constructors.dynamic.c17.3daaa50f.c) | nothing, it is one file | nothing extra | shape=resolver-before-constructors, link=dynamic | c17 | `1 1923506687 1 120` |
| [`ifunc-static.resolver-before-constructors.static.c17.3cf4d7c1`](ifunc-static.resolver-before-constructors.static.c17.3cf4d7c1.c) | nothing, it is one file | `-static` | shape=resolver-before-constructors, link=static | c17 | `1 1923506687 1 ...` and 3 more lines |
| [`ifunc-static.units.dynamic.c17.1956fe00`](ifunc-static.units.dynamic.c17.1956fe00.c) | [`resolver`](ifunc-static.units.dynamic.c17.1956fe00.resolver.c) | nothing extra | shape=units, link=dynamic | c17 | `1923506687 1 120` |
| [`ifunc-static.units.static.c17.6bebf080`](ifunc-static.units.static.c17.6bebf080.c) | [`resolver`](ifunc-static.units.static.c17.6bebf080.resolver.c) | `-static` | shape=units, link=static | c17 | `1923506687 1 120 ...` and 2 more lines |

