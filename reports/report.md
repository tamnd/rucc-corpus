# rucc corpus report

110 case results did not come out as expected. They are listed below, worst first.

The corpus holds 1831 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `096ade4ec00e39a6`.

That is 57,801 lines of C, 2.0 MiB, in 1,855 files, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.10.6 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 110 failures | no |
| `code-quality:rucc` | within 10 percent | 8 percent more | yes |
| `compile-throughput:rucc` | no worse | 48 percent less | yes |
| `size-model:rucc` | no worse | level | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.
- `size-model:rucc`: the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 9123 | 9123 | 0 | 0 | 0 | 0 | 0 | 0 |
| `rucc` | 9123 | 8883 | 5 | 100 | 130 | 0 | 5 | 0 |

## What went wrong

### `setjmp-longjmp.volatile-survives.c17.59577dbb`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it. This is a case about the jump that leaves a function without returning from it, built at `-O0`.

Expected:

```
15
```

Got:

```
10
```

The program is `programs/correctness/setjmp-longjmp/setjmp-longjmp.volatile-survives.c17.59577dbb.c` and the working directory of the failing build was kept under the run directory.

### `setjmp-longjmp.volatile-survives.c17.59577dbb`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it. This is a case about the jump that leaves a function without returning from it, built at `-O1`.

Expected:

```
15
```

Got:

```
10
```

The program is `programs/correctness/setjmp-longjmp/setjmp-longjmp.volatile-survives.c17.59577dbb.c` and the working directory of the failing build was kept under the run directory.

### `setjmp-longjmp.volatile-survives.c17.59577dbb`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it. This is a case about the jump that leaves a function without returning from it, built at `-O2`.

Expected:

```
15
```

Got:

```
10
```

The program is `programs/correctness/setjmp-longjmp/setjmp-longjmp.volatile-survives.c17.59577dbb.c` and the working directory of the failing build was kept under the run directory.

### `setjmp-longjmp.volatile-survives.c17.59577dbb`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it. This is a case about the jump that leaves a function without returning from it, built at `-O3`.

Expected:

```
15
```

Got:

```
10
```

The program is `programs/correctness/setjmp-longjmp/setjmp-longjmp.volatile-survives.c17.59577dbb.c` and the working directory of the failing build was kept under the run directory.

### `setjmp-longjmp.volatile-survives.c17.59577dbb`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it. This is a case about the jump that leaves a function without returning from it, built at `-Os`.

Expected:

```
15
```

Got:

```
10
```

The program is `programs/correctness/setjmp-longjmp/setjmp-longjmp.volatile-survives.c17.59577dbb.c` and the working directory of the failing build was kept under the run directory.

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

And 85 more, which are all in `findings.sarif` and in `report.json`.

## What is not built yet

These are cases a compiler refused while saying itself that the construct is not implemented. They are valid C and they are counted, and they do not turn the build red, because a compiler that tells you what it has not written yet is doing the right thing. The count going up between two runs is a regression and `rucc-corpus diff` will say so.

### `rucc`

- error: `__atomic_signal_fence` is not implemented yet [E0686] (5 cases, first is `atomics.fence.c17.6b7c6e1f`)
- error: `__builtin_alloca` is not implemented yet [E0686] (15 cases, first is `vla-and-alloca.alloca-in-a-loop.c17.c08d97f0`)
- error: cannot generate code for 'main': no rule lowers a `block_addr` producing a `ptr` [E0653] (80 cases, first is `computed-goto.address-in-variable.16.c17.4419a599`)
- error: cannot generate code for 'main': no rule lowers a `stacksave` producing a `ptr` [E0653] (30 cases, first is `vla-and-alloca.as-a-parameter.c17.798b835c`)

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`, and every number is a median over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the headline.

### `rucc`

Furthest behind:

| facet | phase | cases | code size | run time | compile time |
|---|---|---|---|---|---|
| `bit-builtins` | backend | 22 | 282 percent more | 2 percent more | 50 percent less |
| `loop-restructure` | loops | 48 | 118 percent more | 5 percent more | 49 percent less |
| `long-double` | backend | 10 | 105 percent more | level | 47 percent less |
| `register-alloc` | backend | 40 | 75 percent more | 1 percent less | 49 percent less |
| `scheduling` | backend | 12 | 74 percent more | 1 percent less | 50 percent less |
| `register-pressure` | backend | 48 | 51 percent more | 2 percent less | 47 percent less |
| `loop-idiom` | loops | 64 | 36 percent more | 3 percent less | 51 percent less |
| `induction-variable` | loops | 42 | 29 percent more | level | 47 percent less |

Furthest ahead:

| facet | phase | cases | code size | run time | compile time |
|---|---|---|---|---|---|
| `short-circuit` | local | 22 | 17 percent less | 6 percent less | 57 percent less |
| `conditional-store` | local | 16 | 16 percent less | 2 percent less | 60 percent less |
| `value-settled` | local | 13 | 8 percent less | 2 percent less | 59 percent less |
| `setjmp-longjmp` | correctness | 8 | 7 percent less | 3 percent more | 47 percent less |
| `copy-propagation` | global | 12 | 7 percent less | 11 percent less | 52 percent less |
| `unreachable-code` | local | 8 | 6 percent less | 16 percent less | 45 percent less |
| `reachability` | interprocedural | 9 | 6 percent less | 8 percent less | 47 percent less |
| `dead-code` | local | 12 | 5 percent less | 4 percent less | 47 percent less |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 1823 | 3 percent less | 771 |
| `rucc` | 1777 | level | 1255 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

### `rucc` at `-Os`

Where `-Os` saves the most:

| facet | cases | code size at `-Os` |
|---|---|---|
| `loop-restructure` | 48 | 42 percent less |
| `loop-idiom` | 64 | 15 percent less |
| `loop-hoist` | 64 | 10 percent less |
| `loop-unswitch` | 64 | 7 percent less |
| `function-purity` | 20 | 3 percent less |
| `induction-variable` | 42 | 2 percent less |

Where it saves the least, which is where it is spending size and getting nothing for it when the number is above level:

| facet | cases | code size at `-Os` |
|---|---|---|
| `long-double` | 10 | level |
| `barrier` | 7 | level |
| `atomics` | 30 | level |
| `setjmp-longjmp` | 8 | level |
| `frontend` | 22 | level |
| `loop-invariant` | 32 | 3 percent more |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines | `rucc` passed | `rucc` code size |
|---|---|---|---|---|---|
| floor | 6 | 118 | 3,458 | 383 of 558 | 10 percent more |
| local | 11 | 360 | 18,513 | 1800 of 1800 | 1 percent less |
| global | 10 | 208 | 4,846 | 1040 of 1040 | 1 percent less |
| loops | 12 | 638 | 12,795 | 3190 of 3190 | 13 percent more |
| interprocedural | 7 | 137 | 3,643 in 161 files | 635 of 685 | 10 percent more |
| backend | 13 | 324 | 12,869 | 1620 of 1620 | 27 percent more |
| correctness | 3 | 46 | 1,677 | 215 of 230 | 2 percent more |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `atomics` | correctness | 0 | 4026 |
| `bit-builtins` | backend | 0 | 3352 |
| `inline` | interprocedural | 900 | 372 |
| `float-conversion` | backend | 0 | 850 |
| `simplify` | local | 0 | 806 |
| `tail-call` | interprocedural | 300 | 480 |
| `computed-goto` | floor | 32 | 600 |
| `narrowing` | local | 0 | 528 |
| `loop-hoist` | loops | 117 | 372 |
| `loop-idiom` | loops | 395 | 76 |
| `loop-shape` | loops | 112 | 304 |
| `loop-unroll-shape` | loops | 318 | 65 |
| `iv-selection` | loops | 268 | 112 |
| `loop-unswitch` | loops | 134 | 212 |
| `loop-unroll` | loops | 196 | 128 |
| `loop-restructure` | loops | 262 | 60 |
| `setjmp-longjmp` | correctness | 15 | 302 |
| `strength` | local | 0 | 288 |
| `register-pressure` | backend | 75 | 210 |
| `vla-and-alloca` | floor | 50 | 212 |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1831 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
