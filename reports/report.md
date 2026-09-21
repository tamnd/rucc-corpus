# rucc corpus report

25 case results did not come out as expected. They are listed below, worst first.

The corpus holds 2461 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `02869b71e4d51226`.

That is 74,104 lines of C, 2.4 MiB, in 2,485 files, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.10.73 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 25 failures | no |
| `code-quality:rucc` | within 10 percent | 9 percent more | yes |
| `compile-throughput:rucc` | no worse | 30 percent less | yes |
| `size-model:rucc` | no worse | level | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2, counted over the whole corpus by byte.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.
- `size-model:rucc`: the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 12273 | 12273 | 0 | 0 | 0 | 0 | 0 | 0 |
| `rucc` | 12273 | 12243 | 0 | 25 | 5 | 0 | 0 | 0 |

## What went wrong

### `computed-goto.dispatch-in-loop.16.c17.261af709`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 22 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.16.c17.261af709.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.16.c17.261af709`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 22 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.16.c17.261af709.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.16.c17.261af709`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 22 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.16.c17.261af709.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.16.c17.261af709`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 22 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.16.c17.261af709.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.16.c17.261af709`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %163 arrives at block17 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 22 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.16.c17.261af709.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 add: %8 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.2.c17.3dc137d7.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 add: %8 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.2.c17.3dc137d7.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 add: %8 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.2.c17.3dc137d7.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 add: %8 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.2.c17.3dc137d7.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %37 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 add: %8 arrives at block3 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.2.c17.3dc137d7.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 30 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.32.c17.0aa5c5d2.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 30 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.32.c17.0aa5c5d2.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 30 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.32.c17.0aa5c5d2.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 30 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.32.c17.0aa5c5d2.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %307 arrives at block33 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 30 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.32.c17.0aa5c5d2.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.4.c17.eb694d88`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block8 add: %14 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.4.c17.eb694d88.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.4.c17.eb694d88`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block8 add: %14 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.4.c17.eb694d88.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.4.c17.eb694d88`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block8 add: %14 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.4.c17.eb694d88.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.4.c17.eb694d88`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block8 add: %14 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.4.c17.eb694d88.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.4.c17.eb694d88`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %55 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block8 add: %14 arrives at block5 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.4.c17.eb694d88.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 6 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.8.c17.2e2b4cf5.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 6 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.8.c17.2e2b4cf5.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 6 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.8.c17.2e2b4cf5.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 6 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.8.c17.2e2b4cf5.c` and the working directory of the failing build was kept under the run directory.

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5`

rucc would not compile a valid program about the address of a label, and the indirect jump through it. This is a case about the address of a label, and the indirect jump through it, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block2 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block3 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block4 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block5 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
rucc: error: internal error: invalid IR, @main block6 mul: %91 arrives at block9 and does not reach here [E0652]
rucc: note: this is a bug in rucc rather than in the program, please report it
... and 6 more lines
```

The program is `programs/floor/computed-goto/computed-goto.dispatch-in-loop.8.c17.2e2b4cf5.c` and the working directory of the failing build was kept under the run directory.

## What is not built yet

These are cases a compiler refused while saying itself that the construct is not implemented. They are valid C and they are counted, and they do not turn the build red, because a compiler that tells you what it has not written yet is doing the right thing. The count going up between two runs is a regression and `rucc-corpus diff` will say so.

### `rucc`

- error: `__atomic_signal_fence` is not implemented yet [E0686] (5 cases, first is `atomics.fence.c17.6b7c6e1f`)

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`. The per facet figures are medians over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the facet's number. The headline for each compiler is the corpus total instead, every byte counted once, because that is the number that does not move when somebody adds a facet.

The run time column is the same comparison over the fastest of the repetitions of each program, and it says `inside the noise` where the difference between the two compilers is smaller than the difference this machine produced running one of them several times. Size is exact and time is not, so on most of these facets the size column is the honest signal and the time column is there to be checked rather than quoted. Every repetition is in `runs.jsonl` under `execute.samples`, so any number here can be traced back to what was measured.

The instructions column is how many instructions this compiler's programs ran across the facet, less how many the reference's ran. A count rather than a clock, so it comes back the same to within one part in a hundred thousand on a machine where the wall clock moves by a factor of twelve. It is a difference rather than a ratio on purpose. A program in this corpus retires about a hundred and forty thousand instructions and about a hundred and eight thousand of those are the process starting up before `main`, which is the same code on both sides, so a ratio has all of that in its denominator and comes out at one however the compilers differed. A difference has none of it, because a constant on both sides subtracts away exactly. The ratio is still in `report.json` as `instruction_ratio` for anyone who wants to check that. The column says `not measured` where the machine would not count, which is any machine without `perf` and any machine that will not hand a counter to an unprivileged process.

### `rucc`

Over the whole corpus, `rucc` produces 9 percent more. The middle facet is 2 percent less, over 75 facets. The two differ when the larger facets are the ones going badly, and the total is the one to believe.

Furthest behind:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `bit-builtins` | backend | 22 | 83 percent more | +8,466 | inside the noise | 50 percent less |
| `register-alloc` | backend | 40 | 62 percent more | +4,998 | inside the noise | 35 percent less |
| `long-double` | backend | 10 | 51 percent more | -243 | inside the noise | 9 percent more |
| `scheduling` | backend | 20 | 44 percent more | +93 | inside the noise | 36 percent less |
| `register-pressure` | backend | 60 | 36 percent more | +1,846 | inside the noise | 30 percent more |
| `loop-restructure` | loops | 48 | 34 percent more | +38,845 | inside the noise | 13 percent more |
| `computed-goto` | floor | 25 | 19 percent more | +50,584 | inside the noise | 30 percent less |
| `calling-convention` | backend | 10 | 18 percent more | -490 | inside the noise | 47 percent less |

Furthest ahead:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `conditional-store` | local | 16 | 35 percent less | +79,423 | inside the noise | 38 percent less |
| `short-circuit` | local | 22 | 31 percent less | +92,104 | inside the noise | 52 percent less |
| `prune` | global | 15 | 26 percent less | +84,345 | inside the noise | 31 percent less |
| `value-settled` | local | 13 | 22 percent less | +73,208 | inside the noise | 5 percent less |
| `vla-and-alloca` | floor | 9 | 21 percent less | -7,161 | inside the noise | 29 percent less |
| `iv-selection` | loops | 36 | 20 percent less | -23,367 | inside the noise | 55 percent less |
| `store-fold` | backend | 56 | 13 percent less | -46,944 | inside the noise | 14 percent more |
| `load-fold` | backend | 40 | 13 percent less | -30,289 | inside the noise | 51 percent less |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 2453 | 3 percent less | 1001 |
| `rucc` | 2447 | level | 832 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

### `rucc` at `-Os`

Where `-Os` saves the most:

| facet | cases | code size at `-Os` |
|---|---|---|
| `loop-restructure` | 48 | 21 percent less |
| `scheduling` | 20 | 18 percent less |
| `stack-slots` | 6 | 17 percent less |
| `loop-idiom` | 64 | 6 percent less |
| `loop-hoist` | 64 | 6 percent less |
| `load-fold` | 40 | 6 percent less |

Where it saves the least, which is where it is spending size and getting nothing for it when the number is above level:

| facet | cases | code size at `-Os` |
|---|---|---|
| `frontend` | 22 | level |
| `induction-variable` | 42 | 2 percent more |
| `loop-invariant` | 32 | 2 percent more |
| `function-purity` | 28 | 5 percent more |
| `loop-unroll-shape` | 80 | 6 percent more |
| `loop-rotate` | 64 | 6 percent more |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines | `rucc` passed | `rucc` code size |
|---|---|---|---|---|---|
| floor | 6 | 118 | 3,458 | 533 of 558 | level |
| local | 11 | 369 | 19,362 | 1845 of 1845 | 4 percent less |
| global | 10 | 224 | 5,210 | 1120 of 1120 | 4 percent less |
| loops | 12 | 646 | 12,971 | 3230 of 3230 | 1 percent less |
| interprocedural | 10 | 261 | 6,723 in 285 files | 1305 of 1305 | level |
| backend | 23 | 791 | 24,586 | 3955 of 3955 | 1 percent less |
| correctness | 3 | 52 | 1,794 | 255 of 260 | 2 percent less |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `atomics` | correctness | 0 | 4026 |
| `bit-builtins` | backend | 0 | 3352 |
| `inline` | interprocedural | 900 | 372 |
| `store-fold-constant` | backend | 444 | 820 |
| `compare-fold` | backend | 472 | 584 |
| `float-conversion` | backend | 0 | 850 |
| `simplify` | local | 0 | 844 |
| `tail-call` | interprocedural | 300 | 480 |
| `computed-goto` | floor | 32 | 600 |
| `store-fold` | backend | 212 | 400 |
| `narrowing` | local | 0 | 528 |
| `iv-selection` | loops | 292 | 200 |
| `loop-hoist` | loops | 117 | 372 |
| `memory-effects` | interprocedural | 32 | 456 |
| `call-motion` | interprocedural | 10 | 472 |
| `loop-idiom` | loops | 395 | 76 |
| `unused-params` | interprocedural | 276 | 178 |
| `constant-args` | interprocedural | 216 | 232 |
| `loop-shape` | loops | 112 | 304 |
| `loop-unroll-shape` | loops | 318 | 65 |

## How much of this was measured today

23370 of the 24546 results in this report were read out of the record cache rather than built in this run. The verdicts hold, since a program that printed the wrong answer prints it again. The timings and the memory figures were taken on an earlier run of this machine, so any comparison of seconds that spans them is a comparison across sittings. Run with `--refresh` for a set of numbers that were all measured at once.

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 2461 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
