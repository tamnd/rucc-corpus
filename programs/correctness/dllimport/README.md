# dllimport

modules importing the executable's functions and data the way Windows Postgres does. Part of the correctness phase of the M4 plan.

18 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are an executable and the modules it loads while it runs. The file with `main` in it is the executable and each module listed beside it is built on its own into its name with `.so` on the end, in the same directory, which is the path the program opens. On Linux that is `cc -rdynamic -o case case.c -ldl` and then `cc -fPIC -shared -o m0.so m0.c` for each module. On macOS the module is built with `-fPIC -bundle -bundle_loader case` instead, and on Windows the executable is built with `-Wl,--export-all-symbols -Wl,--out-implib,libcase.a` and the module with `cc -shared -o m0.so m0.c libcase.a`, which is why the executable is built first.

| program | loads | axes | dialect | must print |
|---|---|---|---|---|
| [`dllimport.addresses.v0.c17.3ad79ed2`](dllimport.addresses.v0.c17.3ad79ed2.c) | [`m0`](dllimport.addresses.v0.c17.3ad79ed2.m0.c) | shape=addresses, variant=v0 | c17 | `15 1 1 ...` and 3 more lines |
| [`dllimport.addresses.v1.c17.b99e18ac`](dllimport.addresses.v1.c17.b99e18ac.c) | [`m0`](dllimport.addresses.v1.c17.b99e18ac.m0.c) | shape=addresses, variant=v1 | c17 | `15 1 1 ...` and 3 more lines |
| [`dllimport.addresses.v2.c17.3de7d918`](dllimport.addresses.v2.c17.3de7d918.c) | [`m0`](dllimport.addresses.v2.c17.3de7d918.m0.c) | shape=addresses, variant=v2 | c17 | `15 1 1 ...` and 3 more lines |
| [`dllimport.exports.v0.c17.06a95a17`](dllimport.exports.v0.c17.06a95a17.c) | [`m0`](dllimport.exports.v0.c17.06a95a17.m0.c) | shape=exports, variant=v0 | c17 | `180000 100 32 ...` and 8 more lines |
| [`dllimport.exports.v1.c17.a912bcb0`](dllimport.exports.v1.c17.a912bcb0.c) | [`m0`](dllimport.exports.v1.c17.a912bcb0.m0.c) | shape=exports, variant=v1 | c17 | `180001 100 33 ...` and 9 more lines |
| [`dllimport.exports.v2.c17.0c4f8478`](dllimport.exports.v2.c17.0c4f8478.c) | [`m0`](dllimport.exports.v2.c17.0c4f8478.m0.c) | shape=exports, variant=v2 | c17 | `180002 100 34 ...` and 10 more lines |
| [`dllimport.hooks.v0.c17.b163a23e`](dllimport.hooks.v0.c17.b163a23e.c) | [`m0`](dllimport.hooks.v0.c17.b163a23e.m0.c) | shape=hooks, variant=v0 | c17 | `7 22 22 ...` and 2 more lines |
| [`dllimport.hooks.v1.c17.05f928a7`](dllimport.hooks.v1.c17.05f928a7.c) | [`m0`](dllimport.hooks.v1.c17.05f928a7.m0.c), [`m1`](dllimport.hooks.v1.c17.05f928a7.m1.c) | shape=hooks, variant=v1 | c17 | `7 22 163 ...` and 3 more lines |
| [`dllimport.hooks.v2.c17.8421eac3`](dllimport.hooks.v2.c17.8421eac3.c) | [`m0`](dllimport.hooks.v2.c17.8421eac3.m0.c), [`m1`](dllimport.hooks.v2.c17.8421eac3.m1.c), [`m2`](dllimport.hooks.v2.c17.8421eac3.m2.c) | shape=hooks, variant=v2 | c17 | `7 22 163 ...` and 4 more lines |
| [`dllimport.imported-calls.v0.c17.c6af7e7d`](dllimport.imported-calls.v0.c17.c6af7e7d.c) | [`m0`](dllimport.imported-calls.v0.c17.c6af7e7d.m0.c) | shape=imported-calls, variant=v0 | c17 | `60130042569 161 56985 ...` and 4 more lines |
| [`dllimport.imported-calls.v1.c17.2b32ebb9`](dllimport.imported-calls.v1.c17.2b32ebb9.c) | [`m0`](dllimport.imported-calls.v1.c17.2b32ebb9.m0.c) | shape=imported-calls, variant=v1 | c17 | `68720048636 200 25913 ...` and 4 more lines |
| [`dllimport.imported-calls.v2.c17.5943fec5`](dllimport.imported-calls.v2.c17.5943fec5.c) | [`m0`](dllimport.imported-calls.v2.c17.5943fec5.m0.c) | shape=imported-calls, variant=v2 | c17 | `77310054703 243 60377 ...` and 4 more lines |
| [`dllimport.imported-data.v0.c17.baa15361`](dllimport.imported-data.v0.c17.baa15361.c) | [`m0`](dllimport.imported-data.v0.c17.baa15361.m0.c) | shape=imported-data, variant=v0 | c17 | `902895 602331 3373 ...` and 4 more lines |
| [`dllimport.imported-data.v1.c17.fd2c24c5`](dllimport.imported-data.v1.c17.fd2c24c5.c) | [`m0`](dllimport.imported-data.v1.c17.fd2c24c5.m0.c) | shape=imported-data, variant=v1 | c17 | `903096 602308 4108 ...` and 4 more lines |
| [`dllimport.imported-data.v2.c17.133be02d`](dllimport.imported-data.v2.c17.133be02d.c) | [`m0`](dllimport.imported-data.v2.c17.133be02d.m0.c) | shape=imported-data, variant=v2 | c17 | `903076 602917 4573 ...` and 4 more lines |
| [`dllimport.thunk-calls.v0.c17.361be1b1`](dllimport.thunk-calls.v0.c17.361be1b1.c) | [`m0`](dllimport.thunk-calls.v0.c17.361be1b1.m0.c) | shape=thunk-calls, variant=v0 | c17 | `265 99 180 ...` and 3 more lines |
| [`dllimport.thunk-calls.v1.c17.2ee7f7a9`](dllimport.thunk-calls.v1.c17.2ee7f7a9.c) | [`m0`](dllimport.thunk-calls.v1.c17.2ee7f7a9.m0.c) | shape=thunk-calls, variant=v1 | c17 | `1507 111 711 ...` and 3 more lines |
| [`dllimport.thunk-calls.v2.c17.34af3104`](dllimport.thunk-calls.v2.c17.34af3104.c) | [`m0`](dllimport.thunk-calls.v2.c17.34af3104.m0.c) | shape=thunk-calls, variant=v2 | c17 | `2729 107 2126 ...` and 3 more lines |

