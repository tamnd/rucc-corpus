# ifunc-static

FreeBSD ifunc resolvers in static and dynamic programs. Part of the correctness phase of the M4 plan.

10 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`ifunc-static.address.dynamic.c17.77ed423b`](ifunc-static.address.dynamic.c17.77ed423b.c) | nothing, it is one file | nothing extra | shape=address, link=dynamic | c17 | `1 1923506687 1 120` |
| [`ifunc-static.address.static.c17.a75aa469`](ifunc-static.address.static.c17.a75aa469.c) | nothing, it is one file | `-static` | shape=address, link=static | c17 | `1 1923506687 1 ...` and 3 more lines |
| [`ifunc-static.call.dynamic.c17.e4e0e7ac`](ifunc-static.call.dynamic.c17.e4e0e7ac.c) | nothing, it is one file | nothing extra | shape=call, link=dynamic | c17 | `1923506687 1 120` |
| [`ifunc-static.call.static.c17.c08770dd`](ifunc-static.call.static.c17.c08770dd.c) | nothing, it is one file | `-static` | shape=call, link=static | c17 | `1923506687 1 120 ...` and 2 more lines |
| [`ifunc-static.memcpy-family.dynamic.c17.70c70bef`](ifunc-static.memcpy-family.dynamic.c17.70c70bef.c) | nothing, it is one file | nothing extra | shape=memcpy-family, link=dynamic | c17 | `1923506687 16 48571 ...` and 3 more lines |
| [`ifunc-static.memcpy-family.static.c17.57abb2d1`](ifunc-static.memcpy-family.static.c17.57abb2d1.c) | nothing, it is one file | `-static` | shape=memcpy-family, link=static | c17 | `1923506687 16 48571 ...` and 5 more lines |
| [`ifunc-static.resolver-before-constructors.dynamic.c17.4afbd73f`](ifunc-static.resolver-before-constructors.dynamic.c17.4afbd73f.c) | nothing, it is one file | nothing extra | shape=resolver-before-constructors, link=dynamic | c17 | `1 1923506687 1 120` |
| [`ifunc-static.resolver-before-constructors.static.c17.13037fde`](ifunc-static.resolver-before-constructors.static.c17.13037fde.c) | nothing, it is one file | `-static` | shape=resolver-before-constructors, link=static | c17 | `1 1923506687 1 ...` and 3 more lines |
| [`ifunc-static.units.dynamic.c17.834a476e`](ifunc-static.units.dynamic.c17.834a476e.c) | [`resolver`](ifunc-static.units.dynamic.c17.834a476e.resolver.c) | nothing extra | shape=units, link=dynamic | c17 | `1923506687 1 120` |
| [`ifunc-static.units.static.c17.8f933f51`](ifunc-static.units.static.c17.8f933f51.c) | [`resolver`](ifunc-static.units.static.c17.8f933f51.resolver.c) | `-static` | shape=units, link=static | c17 | `1923506687 1 120 ...` and 2 more lines |

