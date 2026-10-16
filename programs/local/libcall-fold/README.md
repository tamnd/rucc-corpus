# libcall-fold

calls to printf and the string functions that fold into other calls. Part of the local phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`libcall-fold.fprintf-char.c17.ac850471`](libcall-fold.fprintf-char.c17.ac850471.c) | shape=fprintf-char | c17 | `xxx3` |
| [`libcall-fold.fprintf-letter.c17.b0ed26e5`](libcall-fold.fprintf-letter.c17.b0ed26e5.c) | shape=fprintf-letter | c17 | `az!3` |
| [`libcall-fold.fprintf-string.c17.899d2c37`](libcall-fold.fprintf-string.c17.899d2c37.c) | shape=fprintf-string | c17 | `alphabe3` |
| [`libcall-fold.fprintf-text.c17.47c31e40`](libcall-fold.fprintf-text.c17.47c31e40.c) | shape=fprintf-text | c17 | `some textsome textsome text3` |
| [`libcall-fold.fputs-char.c17.a72e1259`](libcall-fold.fputs-char.c17.a72e1259.c) | shape=fputs-char | c17 | `xxx3` |
| [`libcall-fold.fputs-text.c17.28109f2c`](libcall-fold.fputs-text.c17.28109f2c.c) | shape=fputs-text | c17 | `some textsome textsome text3` |
| [`libcall-fold.memchr.c17.bc81f7be`](libcall-fold.memchr.c17.bc81f7be.c) | shape=memchr | c17 | `2 -1 -1` |
| [`libcall-fold.printf-char.c17.4def1cab`](libcall-fold.printf-char.c17.4def1cab.c) | shape=printf-char | c17 | `xxx3` |
| [`libcall-fold.printf-empty.c17.3cc84f33`](libcall-fold.printf-empty.c17.3cc84f33.c) | shape=printf-empty | c17 | `3` |
| [`libcall-fold.printf-letter.c17.1343e574`](libcall-fold.printf-letter.c17.1343e574.c) | shape=printf-letter | c17 | `az!3` |
| [`libcall-fold.printf-line.c17.6b73d45d`](libcall-fold.printf-line.c17.6b73d45d.c) | shape=printf-line | c17 | `a line a line a line 3` |
| [`libcall-fold.printf-string-line.c17.2bcfa408`](libcall-fold.printf-string-line.c17.2bcfa408.c) | shape=printf-string-line | c17 | `alpha be  3` |
| [`libcall-fold.printf-text.c17.fedbc8b9`](libcall-fold.printf-text.c17.fedbc8b9.c) | shape=printf-text | c17 | `no newlineno newlineno newline3` |
| [`libcall-fold.strchr.c17.fff1c30d`](libcall-fold.strchr.c17.fff1c30d.c) | shape=strchr | c17 | `2 4 -1 5` |
| [`libcall-fold.strcmp.c17.9da430ea`](libcall-fold.strcmp.c17.9da430ea.c) | shape=strcmp | c17 | `-1 0 1 -1` |
| [`libcall-fold.strlen.c17.9d57a9e0`](libcall-fold.strlen.c17.9d57a9e0.c) | shape=strlen | c17 | `0 1 12` |
| [`libcall-fold.strpbrk-one.c17.1a0d83ad`](libcall-fold.strpbrk-one.c17.1a0d83ad.c) | shape=strpbrk-one | c17 | `2 -1 0` |
| [`libcall-fold.strspn.c17.7984d4f3`](libcall-fold.strspn.c17.7984d4f3.c) | shape=strspn | c17 | `4 0 0` |
| [`libcall-fold.strstr-one.c17.1fb89069`](libcall-fold.strstr-one.c17.1fb89069.c) | shape=strstr-one | c17 | `2 -1 0` |
| [`libcall-fold.strstr.c17.f6de5c3e`](libcall-fold.strstr.c17.f6de5c3e.c) | shape=strstr | c17 | `6 0 -1 4` |

