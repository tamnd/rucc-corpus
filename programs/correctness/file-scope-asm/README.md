# file-scope-asm

the file scope asm of the BSD headers: aliases, .ident, notes and the section stack. Part of the correctness phase of the M4 plan.

24 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

These programs are more than one translation unit each. The other units are listed beside the one that has `main` in it, and they are all handed to the compiler on one command line, in that order.

| program | linked with | also built with | axes | dialect | must print |
|---|---|---|---|---|---|
| [`file-scope-asm.ident.off.c17.1090cbeb`](file-scope-asm.ident.off.c17.1090cbeb.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=ident, reader=off | c17 | `4400` |
| [`file-scope-asm.ident.on.c17.ff8925d7`](file-scope-asm.ident.on.c17.ff8925d7.c) | nothing, it is one file | nothing extra | shape=ident, reader=on | c17 | `4400` |
| [`file-scope-asm.note-named.off.c17.05f771e2`](file-scope-asm.note-named.off.c17.05f771e2.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=note-named, reader=off | c17 | `1100000000 28930 15` |
| [`file-scope-asm.note-named.on.c17.7831d751`](file-scope-asm.note-named.on.c17.7831d751.c) | nothing, it is one file | nothing extra | shape=note-named, reader=on | c17 | `1100000000 28930 15` |
| [`file-scope-asm.note-typed.off.c17.0dba4a21`](file-scope-asm.note-typed.off.c17.0dba4a21.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=note-typed, reader=off | c17 | `1501000 28930` |
| [`file-scope-asm.note-typed.on.c17.566e2839`](file-scope-asm.note-typed.on.c17.566e2839.c) | nothing, it is one file | nothing extra | shape=note-typed, reader=on | c17 | `1501000 28930` |
| [`file-scope-asm.pushsection-nested.off.c17.9dbd796b`](file-scope-asm.pushsection-nested.off.c17.9dbd796b.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=pushsection-nested, reader=off | c17 | `292 180 4` |
| [`file-scope-asm.pushsection-nested.on.c17.dbebd039`](file-scope-asm.pushsection-nested.on.c17.dbebd039.c) | nothing, it is one file | nothing extra | shape=pushsection-nested, reader=on | c17 | `292 180 4` |
| [`file-scope-asm.rcsid.off.c17.00905cb9`](file-scope-asm.rcsid.off.c17.00905cb9.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=rcsid, reader=off | c17 | `4400 4` |
| [`file-scope-asm.rcsid.on.c17.ee9c7a4f`](file-scope-asm.rcsid.on.c17.ee9c7a4f.c) | nothing, it is one file | nothing extra | shape=rcsid, reader=on | c17 | `4400 4` |
| [`file-scope-asm.section-previous.off.c17.d2f943b1`](file-scope-asm.section-previous.off.c17.d2f943b1.c) | nothing, it is one file | `-frucc-asm-reader=off` | shape=section-previous, reader=off | c17 | `4 819 9` |
| [`file-scope-asm.section-previous.on.c17.71e84671`](file-scope-asm.section-previous.on.c17.71e84671.c) | nothing, it is one file | nothing extra | shape=section-previous, reader=on | c17 | `4 819 9` |
| [`file-scope-asm.strong-alias.off.c17.829b588f`](file-scope-asm.strong-alias.off.c17.829b588f.c) | [`lib`](file-scope-asm.strong-alias.off.c17.829b588f.lib.c) | `-frucc-asm-reader=off` | shape=strong-alias, reader=off | c17 | `42 42` |
| [`file-scope-asm.strong-alias.on.c17.7538e2cb`](file-scope-asm.strong-alias.on.c17.7538e2cb.c) | [`lib`](file-scope-asm.strong-alias.on.c17.7538e2cb.lib.c) | nothing extra | shape=strong-alias, reader=on | c17 | `42 42` |
| [`file-scope-asm.weak-alias.off.c17.40118fed`](file-scope-asm.weak-alias.off.c17.40118fed.c) | [`lib`](file-scope-asm.weak-alias.off.c17.40118fed.lib.c) | `-frucc-asm-reader=off` | shape=weak-alias, reader=off | c17 | `16 22` |
| [`file-scope-asm.weak-alias.on.c17.dc426de1`](file-scope-asm.weak-alias.on.c17.dc426de1.c) | [`lib`](file-scope-asm.weak-alias.on.c17.dc426de1.lib.c) | nothing extra | shape=weak-alias, reader=on | c17 | `16 22` |
| [`file-scope-asm.weak-equ-after.off.c17.5305461d`](file-scope-asm.weak-equ-after.off.c17.5305461d.c) | [`lib`](file-scope-asm.weak-equ-after.off.c17.5305461d.lib.c) | `-frucc-asm-reader=off` | shape=weak-equ-after, reader=off | c17 | `16 22` |
| [`file-scope-asm.weak-equ-after.on.c17.5275ff01`](file-scope-asm.weak-equ-after.on.c17.5275ff01.c) | [`lib`](file-scope-asm.weak-equ-after.on.c17.5275ff01.lib.c) | nothing extra | shape=weak-equ-after, reader=on | c17 | `16 22` |
| [`file-scope-asm.weak-equ-before.off.c17.bece41ef`](file-scope-asm.weak-equ-before.off.c17.bece41ef.c) | [`lib`](file-scope-asm.weak-equ-before.off.c17.bece41ef.lib.c) | `-frucc-asm-reader=off` | shape=weak-equ-before, reader=off | c17 | `16 22` |
| [`file-scope-asm.weak-equ-before.on.c17.06391991`](file-scope-asm.weak-equ-before.on.c17.06391991.c) | [`lib`](file-scope-asm.weak-equ-before.on.c17.06391991.lib.c) | nothing extra | shape=weak-equ-before, reader=on | c17 | `16 22` |
| [`file-scope-asm.weak-equ-data.off.c17.f3550f73`](file-scope-asm.weak-equ-data.off.c17.f3550f73.c) | [`lib`](file-scope-asm.weak-equ-data.off.c17.f3550f73.lib.c) | `-frucc-asm-reader=off` | shape=weak-equ-data, reader=off | c17 | `42 42` |
| [`file-scope-asm.weak-equ-data.on.c17.9a6e04f3`](file-scope-asm.weak-equ-data.on.c17.9a6e04f3.c) | [`lib`](file-scope-asm.weak-equ-data.on.c17.9a6e04f3.lib.c) | nothing extra | shape=weak-equ-data, reader=on | c17 | `42 42` |
| [`file-scope-asm.weak-equ-override.off.c17.500084e8`](file-scope-asm.weak-equ-override.off.c17.500084e8.c) | [`lib`](file-scope-asm.weak-equ-override.off.c17.500084e8.lib.c), [`strong`](file-scope-asm.weak-equ-override.off.c17.500084e8.strong.c) | `-frucc-asm-reader=off` | shape=weak-equ-override, reader=off | c17 | `105 16` |
| [`file-scope-asm.weak-equ-override.on.c17.f3266f45`](file-scope-asm.weak-equ-override.on.c17.f3266f45.c) | [`lib`](file-scope-asm.weak-equ-override.on.c17.f3266f45.lib.c), [`strong`](file-scope-asm.weak-equ-override.on.c17.f3266f45.strong.c) | nothing extra | shape=weak-equ-override, reader=on | c17 | `105 16` |

