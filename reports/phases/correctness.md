# correctness

The cases whose job is to prove that nothing happened, and the ones about the shape of the language rather than about code generation.

| facet | cases | lines | `rucc` passed | `rucc` code | `rucc` compile | `rucc` memory |
|---|---|---|---|---|---|---|
| [`barrier`](../../programs/correctness/barrier/README.md) | 13 | 241 | 65 of 65 | 4% less | 29% less | 170% more |
| [`atomics`](../../programs/correctness/atomics/README.md) | 31 | 1,334 | 155 of 155 | 1% more | 41% less | 110% more |
| [`setjmp-longjmp`](../../programs/correctness/setjmp-longjmp/README.md) | 8 | 219 | 40 of 40 | 9% less | 36% less | 42% less |
| [`sigsetjmp`](../../programs/correctness/sigsetjmp/README.md) | 112 | 12,712 | 560 of 560 | 2% more | 50% less | 37% more |
| [`builtin-setjmp`](../../programs/correctness/builtin-setjmp/README.md) | 48 | 5,660 | 240 of 240 | 8% more | 57% less | 19% more |
| [`overflow-builtins`](../../programs/correctness/overflow-builtins/README.md) | 144 | 13,992 | 720 of 720 | 56% more | 46% less | 65% more |
| [`target-attribute`](../../programs/correctness/target-attribute/README.md) | 52 | 4,940 | 260 of 260 | 13% more | 43% less | 35% more |
| [`crc32c`](../../programs/correctness/crc32c/README.md) | 64 | 15,632 | 320 of 320 | 4% more | 17% less | level |
| [`simd-lfind`](../../programs/correctness/simd-lfind/README.md) | 30 | 2,232 | 150 of 150 | 85% more | 45% less | 46% more |
| [`crc32c-armv8`](../../programs/correctness/crc32c-armv8/README.md) | 72 | 25,314 | 0 of 0 | not measured | not measured | not measured |
| [`simd-lfind-neon`](../../programs/correctness/simd-lfind-neon/README.md) | 48 | 4,044 | 0 of 0 | not measured | not measured | not measured |
| [`lkmm`](../../programs/correctness/lkmm/README.md) | 24 | 1,164 | 120 of 120 | 14% less | 31% less | 39% more |
| [`mitigations`](../../programs/correctness/mitigations/README.md) | 25 | 1,510 | 125 of 125 | level | 42% less | 85% more |
| [`asm-goto`](../../programs/correctness/asm-goto/README.md) | 6 | 172 | 30 of 30 | 5% less | 35% less | 121% more |
| [`asm-local-labels`](../../programs/correctness/asm-local-labels/README.md) | 5 | 135 | 25 of 25 | 1% more | 39% less | 88% more |
| [`gas-macros`](../../programs/correctness/gas-macros/README.md) | 6 | 151 | 30 of 30 | 6% less | 38% less | 92% more |
| [`constant-p-after-inline`](../../programs/correctness/constant-p-after-inline/README.md) | 5 | 168 | 25 of 25 | 4% less | 37% less | 145% more |
| [`mcmodel-kernel`](../../programs/correctness/mcmodel-kernel/README.md) | 6 | 165 | 30 of 30 | 2% less | 35% less | 110% more |
| [`general-regs-only`](../../programs/correctness/general-regs-only/README.md) | 5 | 463 | 25 of 25 | 4% less | 43% less | 38% more |
| [`objtool-shapes`](../../programs/correctness/objtool-shapes/README.md) | 5 | 170 | 25 of 25 | level | 39% less | 82% more |
| [`null-pointer-constant`](../../programs/correctness/null-pointer-constant/README.md) | 6 | 302 | 15 of 30 | 1% more | 42% less | 93% more |
| [`const-ice`](../../programs/correctness/const-ice/README.md) | 8 | 408 | 35 of 40 | 1% less | 43% less | 83% more |
| [`section-attr`](../../programs/correctness/section-attr/README.md) | 6 | 320 | 30 of 30 | 3% less | 46% less | 81% more |
| [`bundle`](../../programs/correctness/bundle/README.md) | 33 | 3,651 in 78 files | 165 of 165 | level | 41% less | level |
| [`dllimport`](../../programs/correctness/dllimport/README.md) | 18 | 2,268 in 39 files | 90 of 90 | 4% less | 38% less | level |

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
