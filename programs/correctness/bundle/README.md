# bundle

modules loaded with dlopen that call back into the executable. Part of the correctness phase of the M4 plan.

33 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are an executable and the modules it loads with `dlopen`. The file with `main` in it is the executable and each module listed beside it is built on its own into its name with `.so` on the end, in the same directory, which is the path the program opens. On Linux that is `cc -rdynamic -o case case.c -ldl` and then `cc -fPIC -shared -o m0.so m0.c` for each module, and on macOS the module is built with `-fPIC -bundle -bundle_loader case` instead, which is why the executable is built first.

| program | loads | axes | dialect | must print |
|---|---|---|---|---|
| [`bundle.call-exe.v0.c17.e2bc6aec`](bundle.call-exe.v0.c17.e2bc6aec.c) | [`m0`](bundle.call-exe.v0.c17.e2bc6aec.m0.c) | shape=call-exe, variant=v0 | c17 | `-985 -1786` |
| [`bundle.call-exe.v1.c17.1ac870a8`](bundle.call-exe.v1.c17.1ac870a8.c) | [`m0`](bundle.call-exe.v1.c17.1ac870a8.m0.c) | shape=call-exe, variant=v1 | c17 | `-1443 -1705` |
| [`bundle.call-exe.v2.c17.68a3bf76`](bundle.call-exe.v2.c17.68a3bf76.c) | [`m0`](bundle.call-exe.v2.c17.68a3bf76.m0.c) | shape=call-exe, variant=v2 | c17 | `-1590 -696` |
| [`bundle.callbacks.v0.c17.bd548f8b`](bundle.callbacks.v0.c17.bd548f8b.c) | [`m0`](bundle.callbacks.v0.c17.bd548f8b.m0.c), [`m1`](bundle.callbacks.v0.c17.bd548f8b.m1.c) | shape=callbacks, variant=v0 | c17 | `5 2695 3030 ...` and 3 more lines |
| [`bundle.callbacks.v1.c17.e2ca093a`](bundle.callbacks.v1.c17.e2ca093a.c) | [`m0`](bundle.callbacks.v1.c17.e2ca093a.m0.c), [`m1`](bundle.callbacks.v1.c17.e2ca093a.m1.c) | shape=callbacks, variant=v1 | c17 | `7 26446 31092 ...` and 3 more lines |
| [`bundle.callbacks.v2.c17.d13793ac`](bundle.callbacks.v2.c17.d13793ac.c) | [`m0`](bundle.callbacks.v2.c17.d13793ac.m0.c), [`m1`](bundle.callbacks.v2.c17.d13793ac.m1.c) | shape=callbacks, variant=v2 | c17 | `9 517913 545158 ...` and 3 more lines |
| [`bundle.constructor.v0.c17.342e6db0`](bundle.constructor.v0.c17.342e6db0.c) | [`m0`](bundle.constructor.v0.c17.342e6db0.m0.c) | shape=constructor, variant=v0 | c17 | `0 1 11 ...` and 3 more lines |
| [`bundle.constructor.v1.c17.87a9033e`](bundle.constructor.v1.c17.87a9033e.c) | [`m0`](bundle.constructor.v1.c17.87a9033e.m0.c), [`m1`](bundle.constructor.v1.c17.87a9033e.m1.c) | shape=constructor, variant=v1 | c17 | `0 1 12 ...` and 6 more lines |
| [`bundle.constructor.v2.c17.4e6dbae4`](bundle.constructor.v2.c17.4e6dbae4.c) | [`m0`](bundle.constructor.v2.c17.4e6dbae4.m0.c), [`m1`](bundle.constructor.v2.c17.4e6dbae4.m1.c), [`m2`](bundle.constructor.v2.c17.4e6dbae4.m2.c) | shape=constructor, variant=v2 | c17 | `0 1 13 ...` and 9 more lines |
| [`bundle.dlsym-calls.v0.c17.af4cb387`](bundle.dlsym-calls.v0.c17.af4cb387.c) | [`m0`](bundle.dlsym-calls.v0.c17.af4cb387.m0.c) | shape=dlsym-calls, variant=v0 | c17 | `11 45 45 ...` and 8 more lines |
| [`bundle.dlsym-calls.v1.c17.85a61118`](bundle.dlsym-calls.v1.c17.85a61118.c) | [`m0`](bundle.dlsym-calls.v1.c17.85a61118.m0.c) | shape=dlsym-calls, variant=v1 | c17 | `12 46 46 ...` and 8 more lines |
| [`bundle.dlsym-calls.v2.c17.f4cd91f9`](bundle.dlsym-calls.v2.c17.f4cd91f9.c) | [`m0`](bundle.dlsym-calls.v2.c17.f4cd91f9.m0.c) | shape=dlsym-calls, variant=v2 | c17 | `13 47 47 ...` and 8 more lines |
| [`bundle.elog-varargs.v0.c17.3963a13f`](bundle.elog-varargs.v0.c17.3963a13f.c) | [`m0`](bundle.elog-varargs.v0.c17.3963a13f.m0.c) | shape=elog-varargs, variant=v0 | c17 | `6 1 12045920457139117978` |
| [`bundle.elog-varargs.v1.c17.071fb78f`](bundle.elog-varargs.v1.c17.071fb78f.c) | [`m0`](bundle.elog-varargs.v1.c17.071fb78f.m0.c) | shape=elog-varargs, variant=v1 | c17 | `8 1 18276659839574165844` |
| [`bundle.elog-varargs.v2.c17.a775c2bf`](bundle.elog-varargs.v2.c17.a775c2bf.c) | [`m0`](bundle.elog-varargs.v2.c17.a775c2bf.m0.c) | shape=elog-varargs, variant=v2 | c17 | `10 2 7851562668395100363` |
| [`bundle.exe-globals.v0.c17.8b915eb2`](bundle.exe-globals.v0.c17.8b915eb2.c) | [`m0`](bundle.exe-globals.v0.c17.8b915eb2.m0.c) | shape=exe-globals, variant=v0 | c17 | `201230 302501 3380 ...` and 4 more lines |
| [`bundle.exe-globals.v1.c17.28af6f27`](bundle.exe-globals.v1.c17.28af6f27.c) | [`m0`](bundle.exe-globals.v1.c17.28af6f27.m0.c) | shape=exe-globals, variant=v1 | c17 | `201349 303206 4687 ...` and 4 more lines |
| [`bundle.exe-globals.v2.c17.bae4aeea`](bundle.exe-globals.v2.c17.bae4aeea.c) | [`m0`](bundle.exe-globals.v2.c17.bae4aeea.m0.c) | shape=exe-globals, variant=v2 | c17 | `201501 303707 6754 ...` and 4 more lines |
| [`bundle.guc.v0.c17.10341e35`](bundle.guc.v0.c17.10341e35.c) | [`m0`](bundle.guc.v0.c17.10341e35.m0.c) | shape=guc, variant=v0 | c17 | `2 4091 4156 ...` and 11 more lines |
| [`bundle.guc.v1.c17.b56908cb`](bundle.guc.v1.c17.b56908cb.c) | [`m0`](bundle.guc.v1.c17.b56908cb.m0.c) | shape=guc, variant=v1 | c17 | `2 4092 4181 ...` and 11 more lines |
| [`bundle.guc.v2.c17.67bbaea4`](bundle.guc.v2.c17.67bbaea4.c) | [`m0`](bundle.guc.v2.c17.67bbaea4.m0.c) | shape=guc, variant=v2 | c17 | `2 4093 4208 ...` and 11 more lines |
| [`bundle.hook-chain.v0.c17.b2f230aa`](bundle.hook-chain.v0.c17.b2f230aa.c) | [`m0`](bundle.hook-chain.v0.c17.b2f230aa.m0.c) | shape=hook-chain, variant=v0 | c17 | `4 9 9 ...` and 3 more lines |
| [`bundle.hook-chain.v1.c17.291fef35`](bundle.hook-chain.v1.c17.291fef35.c) | [`m0`](bundle.hook-chain.v1.c17.291fef35.m0.c), [`m1`](bundle.hook-chain.v1.c17.291fef35.m1.c) | shape=hook-chain, variant=v1 | c17 | `4 9 38 ...` and 5 more lines |
| [`bundle.hook-chain.v2.c17.8562f7a2`](bundle.hook-chain.v2.c17.8562f7a2.c) | [`m0`](bundle.hook-chain.v2.c17.8562f7a2.m0.c), [`m1`](bundle.hook-chain.v2.c17.8562f7a2.m1.c), [`m2`](bundle.hook-chain.v2.c17.8562f7a2.m2.c) | shape=hook-chain, variant=v2 | c17 | `4 9 38 ...` and 7 more lines |
| [`bundle.module-statics.v0.c17.a23adbf0`](bundle.module-statics.v0.c17.a23adbf0.c) | [`m0`](bundle.module-statics.v0.c17.a23adbf0.m0.c) | shape=module-statics, variant=v0 | c17 | `107 228 350 ...` and 7 more lines |
| [`bundle.module-statics.v1.c17.9d70e4f3`](bundle.module-statics.v1.c17.9d70e4f3.c) | [`m0`](bundle.module-statics.v1.c17.9d70e4f3.m0.c) | shape=module-statics, variant=v1 | c17 | `1116 1240 1364 ...` and 8 more lines |
| [`bundle.module-statics.v2.c17.6c2d18d2`](bundle.module-statics.v2.c17.6c2d18d2.c) | [`m0`](bundle.module-statics.v2.c17.6c2d18d2.m0.c) | shape=module-statics, variant=v2 | c17 | `2125 2253 2365 ...` and 9 more lines |
| [`bundle.same-names.v0.c17.e2b60d0f`](bundle.same-names.v0.c17.e2b60d0f.c) | [`m0`](bundle.same-names.v0.c17.e2b60d0f.m0.c), [`m1`](bundle.same-names.v0.c17.e2b60d0f.m1.c) | shape=same-names, variant=v0 | c17 | `1 8 252 ...` and 6 more lines |
| [`bundle.same-names.v1.c17.e9f1455e`](bundle.same-names.v1.c17.e9f1455e.c) | [`m0`](bundle.same-names.v1.c17.e9f1455e.m0.c), [`m1`](bundle.same-names.v1.c17.e9f1455e.m1.c) | shape=same-names, variant=v1 | c17 | `1 10 255 ...` and 7 more lines |
| [`bundle.same-names.v2.c17.e6b78b0b`](bundle.same-names.v2.c17.e6b78b0b.c) | [`m0`](bundle.same-names.v2.c17.e6b78b0b.m0.c), [`m1`](bundle.same-names.v2.c17.e6b78b0b.m1.c) | shape=same-names, variant=v2 | c17 | `1 12 258 ...` and 8 more lines |
| [`bundle.thread-local.v0.c17.e20aa023`](bundle.thread-local.v0.c17.e20aa023.c) | [`m0`](bundle.thread-local.v0.c17.e20aa023.m0.c) | shape=thread-local, variant=v0 | c17 | `25 67 139 ...` and 4 more lines |
| [`bundle.thread-local.v1.c17.15af7682`](bundle.thread-local.v1.c17.15af7682.c) | [`m0`](bundle.thread-local.v1.c17.15af7682.m0.c) | shape=thread-local, variant=v1 | c17 | `22 62 134 ...` and 5 more lines |
| [`bundle.thread-local.v2.c17.83d104ac`](bundle.thread-local.v2.c17.83d104ac.c) | [`m0`](bundle.thread-local.v2.c17.83d104ac.m0.c) | shape=thread-local, variant=v2 | c17 | `17 53 123 ...` and 6 more lines |

