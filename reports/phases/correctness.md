# correctness

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`barrier`](../../programs/correctness/barrier/README.md) | 13 | 241 | 65 of 65 | 6% less | 36% less | level |
| [`atomics`](../../programs/correctness/atomics/README.md) | 31 | 1,334 | 155 of 155 | 4% less | 41% less | 26% less |
| [`setjmp-longjmp`](../../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 40 of 40 | 12% less | 37% less | 1% more |
| [`sigsetjmp`](../../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 560 of 560 | 2% less | 53% less | 36% less |
| [`builtin-setjmp`](../../programs/correctness/builtin-setjmp/README.md) | 48 | 5,660 | 240 of 240 | 5% more | 57% less | 41% less |
| [`overflow-builtins`](../../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 720 of 720 | 23% more | 51% less | 26% less |
| [`target-attribute`](../../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 260 of 260 | 5% more | 59% less | 35% less |
| [`crc32c`](../../programs/correctness/crc32c/README.md) | 64 | 15,632 | 320 of 320 | 1% more | 56% less | 38% less |
| [`simd-lfind`](../../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 150 of 150 | 67% more | 55% less | 29% less |
| [`crc32c-armv8`](../../programs/correctness/crc32c-armv8/README.md) | 72 | 25,314 | 0 of 0 | not measured | not measured | not measured |
| [`simd-lfind-neon`](../../programs/correctness/simd-lfind-neon/README.md) | 54 | 4,590 | 0 of 0 | not measured | not measured | not measured |
| [`lkmm`](../../programs/correctness/lkmm/README.md) | 24 | 1,164 | 120 of 120 | 17% less | 44% less | 27% less |
| [`mitigations`](../../programs/correctness/mitigations/README.md) | 25 | 1,510 | 125 of 125 | 4% less | 43% less | 29% less |
| [`asm-goto`](../../programs/correctness/asm-goto/README.md) | 6 | 172 | 30 of 30 | 10% less | 35% less | 30% less |
| [`asm-local-labels`](../../programs/correctness/asm-local-labels/README.md) | 5 | 135 | 25 of 25 | 9% less | 38% less | 29% less |
| [`gas-macros`](../../programs/correctness/gas-macros/README.md) | 6 | 151 | 30 of 30 | 10% less | 35% less | 7% less |
| [`constant-p-after-inline`](../../programs/correctness/constant-p-after-inline/README.md) | 5 | 168 | 25 of 25 | 10% less | 45% less | 28% less |
| [`mcmodel-kernel`](../../programs/correctness/mcmodel-kernel/README.md) | 6 | 165 | 30 of 30 | 7% less | 40% less | 29% less |
| [`general-regs-only`](../../programs/correctness/general-regs-only/README.md) | 5 | 463 | 23 of 25 | 7% less | 49% less | 36% less |
| [`i386-kernel`](../../programs/correctness/i386-kernel/README.md) | 9 | 624 | 0 of 0 | not measured | not measured | not measured |
| [`objtool-shapes`](../../programs/correctness/objtool-shapes/README.md) | 5 | 170 | 25 of 25 | 9% less | 45% less | 29% less |
| [`null-pointer-constant`](../../programs/correctness/null-pointer-constant/README.md) | 6 | 302 | 30 of 30 | 8% less | 40% less | 30% less |
| [`const-ice`](../../programs/correctness/const-ice/README.md) | 8 | 408 | 40 of 40 | 8% less | 46% less | 27% less |
| [`section-attr`](../../programs/correctness/section-attr/README.md) | 6 | 320 | 30 of 30 | 10% less | 48% less | 32% less |
| [`file-scope-asm`](../../programs/correctness/file-scope-asm/README.md) | 24 | 1,316 in 38 files | 120 of 120 | 10% less | 51% less | 22% less |
| [`linker-sets`](../../programs/correctness/linker-sets/README.md) | 9 | 562 in 13 files | 45 of 45 | 8% less | 49% less | 28% less |
| [`crt-and-brand`](../../programs/correctness/crt-and-brand/README.md) | 7 | 410 in 8 files | 35 of 35 | 9% less | 55% less | 27% less |
| [`symbol-versions`](../../programs/correctness/symbol-versions/README.md) | 6 | 705 in 7 files | 30 of 30 | 21% less | 57% less | 34% less |
| [`ifunc-static`](../../programs/correctness/ifunc-static/README.md) | 10 | 1,880 in 12 files | 49 of 50 | 8% less | 63% less | 18% less |
| [`boot-abi`](../../programs/correctness/boot-abi/README.md) | 6 | 368 | 0 of 30 | not measured | 79% less | 100% less |
| [`tls-models`](../../programs/correctness/tls-models/README.md) | 6 | 520 in 10 files | 30 of 30 | 9% less | 53% less | 29% less |
| [`unwinder`](../../programs/correctness/unwinder/README.md) | 7 | 459 | 27 of 35 | 18% less | 61% less | 33% less |
| [`boot-sizes`](../../programs/correctness/boot-sizes/README.md) | 4 | 489 | 0 of 20 | not measured | 87% less | 96% less |
| [`netbsd-kernel`](../../programs/correctness/netbsd-kernel/README.md) | 5 | 531 in 8 files | 25 of 25 | 9% less | 52% less | 29% less |
| [`werror-shapes`](../../programs/correctness/werror-shapes/README.md) | 6 | 353 in 7 files | 30 of 30 | 10% less | 33% less | 1% less |
| [`kprintf-format`](../../programs/correctness/kprintf-format/README.md) | 7 | 3,338 in 14 files | 35 of 35 | 9% less | 78% less | 43% less |
| [`bundle`](../../programs/correctness/bundle/README.md) | 33 | 3,651 in 78 files | 165 of 165 | 10% less | 44% less | 31% less |
| [`dllimport`](../../programs/correctness/dllimport/README.md) | 18 | 2,268 in 39 files | 90 of 90 | 9% less | 43% less | 27% less |

The reference compiler said it took 15722 transformations in this phase and wanted 129723 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

## Switches lowered differently

Each of these cases has a `switch` that a compiler under test lowered into a different shape from the one `gcc-16` used: a jump table, a bit test, a lookup table of answers, or compares when it was none of the three. It is a lead to read the assembly for rather than a failure. The run's `report.md` says how many cases agreed.

| compiler | level | case | it used | `gcc-16` used |
|---|---|---|---|---|
| `rucc` | O0 | `netbsd-kernel.switch-thunk.c17.b691e324` | table | compares |
| `rucc` | O0 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | O1 | `mcmodel-kernel.switch.c17.1218421f` | compares | table |
| `rucc` | O1 | `mitigations.ibt.switch.c17.d5e3a807` | compares | table |
| `rucc` | O1 | `mitigations.rethunk.switch.c17.e80371db` | compares | table |
| `rucc` | O1 | `mitigations.sls.switch.c17.11ea7496` | compares | table |
| `rucc` | O1 | `netbsd-kernel.switch-thunk.c17.b691e324` | table | compares |
| `rucc` | O1 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | O2 | `mcmodel-kernel.switch.c17.1218421f` | compares | table |
| `rucc` | O2 | `mitigations.ibt.switch.c17.d5e3a807` | compares | table |
| `rucc` | O2 | `mitigations.rethunk.switch.c17.e80371db` | compares | table |
| `rucc` | O2 | `mitigations.sls.switch.c17.11ea7496` | compares | table |
| `rucc` | O2 | `netbsd-kernel.switch-thunk.c17.b691e324` | table | compares |
| `rucc` | O2 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | O3 | `mcmodel-kernel.switch.c17.1218421f` | compares | table |
| `rucc` | O3 | `mitigations.ibt.switch.c17.d5e3a807` | compares | table |
| `rucc` | O3 | `mitigations.rethunk.switch.c17.e80371db` | compares | table |
| `rucc` | O3 | `mitigations.sls.switch.c17.11ea7496` | compares | table |
| `rucc` | O3 | `netbsd-kernel.switch-thunk.c17.b691e324` | table | compares |
| `rucc` | O3 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | Os | `mitigations.retpoline.switch.c17.6ea4a885` | table | compares |
| `rucc` | Os | `netbsd-kernel.switch-thunk.c17.b691e324` | table | compares |
| `rucc` | Os | `objtool-shapes.jump-table.c17.9da06433` | compares | table |

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
