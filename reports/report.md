# rucc corpus report

60 case results did not come out as expected. They are listed below, worst first.

The corpus holds 1910 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `cffa24ab30a9fa5b`.

That is 60,117 lines of C, 2.0 MiB, in 1,934 files, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.10.31 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 60 failures | no |
| `code-quality:rucc` | within 10 percent | 18 percent more | no |
| `compile-throughput:rucc` | no worse | 48 percent less | yes |
| `size-model:rucc` | no worse | level | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2, counted over the whole corpus by byte.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.
- `size-model:rucc`: the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 9518 | 9518 | 0 | 0 | 0 | 0 | 0 | 0 |
| `rucc` | 9518 | 9358 | 5 | 50 | 100 | 0 | 5 | 0 |

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

And 35 more, which are all in `findings.sarif` and in `report.json`.

## What is not built yet

These are cases a compiler refused while saying itself that the construct is not implemented. They are valid C and they are counted, and they do not turn the build red, because a compiler that tells you what it has not written yet is doing the right thing. The count going up between two runs is a regression and `rucc-corpus diff` will say so.

### `rucc`

- error: `__atomic_signal_fence` is not implemented yet [E0686] (5 cases, first is `atomics.fence.c17.6b7c6e1f`)
- error: `__builtin_alloca` is not implemented yet [E0686] (15 cases, first is `vla-and-alloca.alloca-in-a-loop.c17.c08d97f0`)
- error: cannot generate code for 'main': no rule lowers a `block_addr` producing a `ptr` [E0653] (80 cases, first is `computed-goto.address-in-variable.16.c17.4419a599`)

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`. The per facet figures are medians over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the facet's number. The headline for each compiler is the corpus total instead, every byte counted once, because that is the number that does not move when somebody adds a facet.

The run time column is the same comparison over the fastest of the repetitions of each program, and it says `inside the noise` where the difference between the two compilers is smaller than the difference this machine produced running one of them several times. Size is exact and time is not, so on most of these facets the size column is the honest signal and the time column is there to be checked rather than quoted. Every repetition is in `runs.jsonl` under `execute.samples`, so any number here can be traced back to what was measured.

The instructions column is how many instructions this compiler's programs ran across the facet, less how many the reference's ran. A count rather than a clock, so it comes back the same to within one part in a hundred thousand on a machine where the wall clock moves by a factor of twelve. It is a difference rather than a ratio on purpose. A program in this corpus retires about a hundred and forty thousand instructions and about a hundred and eight thousand of those are the process starting up before `main`, which is the same code on both sides, so a ratio has all of that in its denominator and comes out at one however the compilers differed. A difference has none of it, because a constant on both sides subtracts away exactly. The ratio is still in `report.json` as `instruction_ratio` for anyone who wants to check that. The column says `not measured` where the machine would not count, which is any machine without `perf` and any machine that will not hand a counter to an unprivileged process.

### `rucc`

Over the whole corpus, `rucc` produces 18 percent more. The middle facet is 3 percent more, over 64 facets. The two differ when the larger facets are the ones going badly, and the total is the one to believe.

Furthest behind:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `bit-builtins` | backend | 22 | 88 percent more | not measured | inside the noise | 54 percent less |
| `long-double` | backend | 10 | 76 percent more | not measured | inside the noise | 49 percent less |
| `register-alloc` | backend | 40 | 75 percent more | not measured | level | 50 percent less |
| `scheduling` | backend | 12 | 71 percent more | not measured | inside the noise | 50 percent less |
| `register-pressure` | backend | 60 | 47 percent more | not measured | inside the noise | 47 percent less |
| `loop-restructure` | loops | 48 | 45 percent more | not measured | inside the noise | 47 percent less |
| `float-conversion` | backend | 66 | 27 percent more | not measured | inside the noise | 48 percent less |
| `calling-convention` | backend | 10 | 24 percent more | not measured | inside the noise | 48 percent less |

Furthest ahead:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `conditional-store` | local | 16 | 31 percent less | not measured | inside the noise | 60 percent less |
| `short-circuit` | local | 22 | 29 percent less | not measured | inside the noise | 57 percent less |
| `prune` | global | 15 | 24 percent less | not measured | inside the noise | 56 percent less |
| `value-settled` | local | 13 | 19 percent less | not measured | inside the noise | 57 percent less |
| `iv-selection` | loops | 36 | 16 percent less | not measured | inside the noise | 58 percent less |
| `setjmp-longjmp` | correctness | 8 | 8 percent less | not measured | inside the noise | 41 percent less |
| `loop-rotate` | loops | 64 | 7 percent less | not measured | inside the noise | 47 percent less |
| `copy-propagation` | global | 12 | 7 percent less | not measured | inside the noise | 44 percent less |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 1902 | 2 percent less | 821 |
| `rucc` | 1872 | level | 1294 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

### `rucc` at `-Os`

Where `-Os` saves the most:

| facet | cases | code size at `-Os` |
|---|---|---|
| `loop-restructure` | 48 | 22 percent less |
| `loop-hoist` | 64 | 5 percent less |
| `loop-idiom` | 64 | 5 percent less |
| `loop-unswitch` | 64 | 5 percent less |
| `function-purity` | 20 | 3 percent less |
| `vla-and-alloca` | 6 | 2 percent less |

Where it saves the least, which is where it is spending size and getting nothing for it when the number is above level:

| facet | cases | code size at `-Os` |
|---|---|---|
| `frontend` | 22 | level |
| `block-layout` | 4 | 1 percent more |
| `induction-variable` | 42 | 1 percent more |
| `loop-invariant` | 32 | 3 percent more |
| `loop-unroll-shape` | 80 | 6 percent more |
| `loop-rotate` | 64 | 6 percent more |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines | `rucc` passed | `rucc` code size |
|---|---|---|---|---|---|
| floor | 6 | 118 | 3,458 | 413 of 558 | 4 percent more |
| local | 11 | 360 | 18,513 | 1800 of 1800 | 2 percent less |
| global | 10 | 224 | 5,210 | 1120 of 1120 | 2 percent less |
| loops | 12 | 646 | 12,971 | 3230 of 3230 | 3 percent more |
| interprocedural | 7 | 137 | 3,643 in 161 files | 685 of 685 | 9 percent more |
| backend | 16 | 379 | 14,645 | 1895 of 1895 | 15 percent more |
| correctness | 3 | 46 | 1,677 | 215 of 230 | 1 percent less |

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
| `iv-selection` | loops | 292 | 200 |
| `loop-hoist` | loops | 117 | 372 |
| `loop-idiom` | loops | 395 | 76 |
| `loop-shape` | loops | 112 | 304 |
| `loop-unroll-shape` | loops | 318 | 65 |
| `register-pressure` | backend | 111 | 270 |
| `loop-unswitch` | loops | 134 | 212 |
| `loop-unroll` | loops | 196 | 128 |
| `loop-restructure` | loops | 262 | 60 |
| `setjmp-longjmp` | correctness | 15 | 302 |
| `strength` | local | 0 | 288 |
| `vla-and-alloca` | floor | 50 | 212 |

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 1910 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
