# correctness

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`barrier`](../../programs/correctness/barrier/README.md) | 13 | 241 | 65 of 65 | 5% less | 34% less | 160% more |
| [`atomics`](../../programs/correctness/atomics/README.md) | 31 | 1,334 | 155 of 155 | 1% more | 42% less | 103% more |
| [`setjmp-longjmp`](../../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 40 of 40 | 9% less | 33% less | 129% more |
| [`sigsetjmp`](../../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 560 of 560 | level | 53% less | 12% more |
| [`builtin-setjmp`](../../programs/correctness/builtin-setjmp/README.md) | 48 | 5,660 | 240 of 240 | 7% more | 59% less | 26% more |
| [`overflow-builtins`](../../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 720 of 720 | 49% more | 50% less | 31% more |
| [`target-attribute`](../../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 260 of 260 | 6% more | 51% less | 36% more |
| [`crc32c`](../../programs/correctness/crc32c/README.md) | 64 | 15,632 | 320 of 320 | 1% more | 50% less | 8% more |
| [`simd-lfind`](../../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 150 of 150 | 90% more | 50% less | 21% more |
| [`crc32c-armv8`](../../programs/correctness/crc32c-armv8/README.md) | 72 | 25,314 | 0 of 0 | not measured | not measured | not measured |
| [`simd-lfind-neon`](../../programs/correctness/simd-lfind-neon/README.md) | 48 | 4,044 | 0 of 0 | not measured | not measured | not measured |
| [`lkmm`](../../programs/correctness/lkmm/README.md) | 24 | 1,164 | 120 of 120 | 15% less | 37% less | 34% more |
| [`mitigations`](../../programs/correctness/mitigations/README.md) | 25 | 1,510 | 125 of 125 | level | 44% less | 86% more |
| [`asm-goto`](../../programs/correctness/asm-goto/README.md) | 6 | 172 | 30 of 30 | 5% less | 39% less | 72% more |
| [`asm-local-labels`](../../programs/correctness/asm-local-labels/README.md) | 5 | 135 | 25 of 25 | 1% more | 39% less | 96% more |
| [`gas-macros`](../../programs/correctness/gas-macros/README.md) | 6 | 151 | 30 of 30 | 6% less | 38% less | 3% more |
| [`constant-p-after-inline`](../../programs/correctness/constant-p-after-inline/README.md) | 5 | 168 | 25 of 25 | 4% less | 37% less | level |
| [`mcmodel-kernel`](../../programs/correctness/mcmodel-kernel/README.md) | 6 | 165 | 30 of 30 | 3% less | 41% less | 100% more |
| [`general-regs-only`](../../programs/correctness/general-regs-only/README.md) | 5 | 463 | 25 of 25 | 4% less | 46% less | 6% more |
| [`objtool-shapes`](../../programs/correctness/objtool-shapes/README.md) | 5 | 170 | 25 of 25 | level | 43% less | 91% more |
| [`null-pointer-constant`](../../programs/correctness/null-pointer-constant/README.md) | 6 | 302 | 15 of 30 | 1% more | 40% less | level |
| [`const-ice`](../../programs/correctness/const-ice/README.md) | 8 | 408 | 35 of 40 | 1% less | 45% less | 33% more |
| [`section-attr`](../../programs/correctness/section-attr/README.md) | 6 | 320 | 30 of 30 | 3% less | 45% less | 50% more |
| [`bundle`](../../programs/correctness/bundle/README.md) | 33 | 3,651 in 78 files | 165 of 165 | 5% less | 42% less | level |
| [`dllimport`](../../programs/correctness/dllimport/README.md) | 18 | 2,268 in 39 files | 90 of 90 | 7% less | 39% less | 39% more |

The reference compiler said it took 14008 transformations in this phase and wanted 122073 more that it could not take. That is not a pass or fail signal for anybody. It says whether the transformation a case was written for was available in that program at all, which is what tells a case the compiler ignored apart from a case that had nothing in it to do.

## Switches lowered differently

Each of these cases has a `switch` that a compiler under test lowered into a different shape from the one `gcc-16` used: a jump table, a bit test, a lookup table of answers, or compares when it was none of the three. It is a lead to read the assembly for rather than a failure. The run's `report.md` says how many cases agreed.

| compiler | level | case | it used | `gcc-16` used |
|---|---|---|---|---|
| `rucc` | O0 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | O1 | `mcmodel-kernel.switch.c17.1218421f` | compares | table |
| `rucc` | O1 | `mitigations.ibt.switch.c17.d5e3a807` | compares | table |
| `rucc` | O1 | `mitigations.rethunk.switch.c17.e80371db` | compares | table |
| `rucc` | O1 | `mitigations.sls.switch.c17.11ea7496` | compares | table |
| `rucc` | O1 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | O2 | `mcmodel-kernel.switch.c17.1218421f` | compares | table |
| `rucc` | O2 | `mitigations.ibt.switch.c17.d5e3a807` | compares | table |
| `rucc` | O2 | `mitigations.rethunk.switch.c17.e80371db` | compares | table |
| `rucc` | O2 | `mitigations.sls.switch.c17.11ea7496` | compares | table |
| `rucc` | O2 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | O3 | `mcmodel-kernel.switch.c17.1218421f` | compares | table |
| `rucc` | O3 | `mitigations.ibt.switch.c17.d5e3a807` | compares | table |
| `rucc` | O3 | `mitigations.rethunk.switch.c17.e80371db` | compares | table |
| `rucc` | O3 | `mitigations.sls.switch.c17.11ea7496` | compares | table |
| `rucc` | O3 | `objtool-shapes.jump-table.c17.9da06433` | compares | table |
| `rucc` | Os | `mitigations.retpoline.switch.c17.6ea4a885` | table | compares |
| `rucc` | Os | `objtool-shapes.jump-table.c17.9da06433` | compares | table |

Back to [the hub](../README.md), or across to [what it cost](../cost.md).
