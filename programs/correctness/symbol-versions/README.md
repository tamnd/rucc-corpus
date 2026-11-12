# symbol-versions

FreeBSD symbol versions: __sym_compat, __sym_default, symver and versioned aliases. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`symbol-versions.sym-compat-reader-off.c17.70dc27ca`](symbol-versions.sym-compat-reader-off.c17.70dc27ca.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=sym-compat-reader-off | c17 | `42 102 112 ...` and 2 more lines |
| [`symbol-versions.sym-compat.c17.4763f806`](symbol-versions.sym-compat.c17.4763f806.c) | nothing, it is one file | nothing extra | shape=sym-compat | c17 | `42 102 112 ...` and 2 more lines |
| [`symbol-versions.symver-attribute.c17.95910aa8`](symbol-versions.symver-attribute.c17.95910aa8.c) | nothing, it is one file | nothing extra | shape=symver-attribute | c17 | `8 102 112 1` |
| [`symbol-versions.symver-units.c17.a478a664`](symbol-versions.symver-units.c17.a478a664.c) | [`compat`](symbol-versions.symver-units.c17.a478a664.compat.c) | nothing extra | shape=symver-units | c17 | `42 102 112 ...` and 2 more lines |
| [`symbol-versions.versioned-alias.c17.6244e64e`](symbol-versions.versioned-alias.c17.6244e64e.c) | nothing, it is one file | nothing extra | shape=versioned-alias | c17 | `15 102 112 1` |
| [`symbol-versions.weak-version.c17.eac019c7`](symbol-versions.weak-version.c17.eac019c7.c) | nothing, it is one file | nothing extra | shape=weak-version | c17 | `111 102 112 1` |

