# tls-models

the four TLS models in executables, static programs and dlopen modules. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are an executable and the modules it loads while it runs. The file with `main` in it is the executable and each module listed beside it is built on its own into its name with `.so` on the end, in the same directory, which is the path the program opens. On Linux that is `cc -rdynamic -o case case.c -ldl` and then `cc -fPIC -shared -o m0.so m0.c` for each module. On macOS the module is built with `-fPIC -bundle -bundle_loader case` instead, and on Windows the executable is built with `-Wl,--export-all-symbols -Wl,--out-implib,libcase.a` and the module with `cc -shared -o m0.so m0.c libcase.a`, which is why the executable is built first.

| program | loads | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`tls-models.global-dynamic.dlopen.c17.91d79873`](tls-models.global-dynamic.dlopen.c17.91d79873.c) | [`tlsdata`](tls-models.global-dynamic.dlopen.c17.91d79873.tlsdata.c) | `-pthread` | model=global-dynamic, link=dlopen | c17 | `19557 24117 7 ...` and 4 more lines |
| [`tls-models.initial-exec.dynamic.c17.ad8b9129`](tls-models.initial-exec.dynamic.c17.ad8b9129.c) | [`tlsdata`](tls-models.initial-exec.dynamic.c17.ad8b9129.tlsdata.c) | `-pthread` | model=initial-exec, link=dynamic | c17 | `19557 24117 7 ...` and 4 more lines |
| [`tls-models.initial-exec.static.c17.225d440b`](tls-models.initial-exec.static.c17.225d440b.c) | [`tlsdata`](tls-models.initial-exec.static.c17.225d440b.tlsdata.c) | `-pthread` `-static` | model=initial-exec, link=static | c17 | `19557 24117 7 ...` and 4 more lines |
| [`tls-models.local-dynamic.dlopen.c17.3cc7c440`](tls-models.local-dynamic.dlopen.c17.3cc7c440.c) | [`tlsdata`](tls-models.local-dynamic.dlopen.c17.3cc7c440.tlsdata.c) | `-pthread` | model=local-dynamic, link=dlopen | c17 | `19557 24117 7 ...` and 4 more lines |
| [`tls-models.local-exec.dynamic.c17.7ffbd4d7`](tls-models.local-exec.dynamic.c17.7ffbd4d7.c) | nothing, it is one file | `-pthread` | model=local-exec, link=dynamic | c17 | `19557 24117 7 ...` and 4 more lines |
| [`tls-models.local-exec.static.c17.647b319f`](tls-models.local-exec.static.c17.647b319f.c) | nothing, it is one file | `-pthread` `-static` | model=local-exec, link=static | c17 | `19557 24117 7 ...` and 4 more lines |

