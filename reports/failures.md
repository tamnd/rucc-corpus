# What went wrong

160 findings, worst first. A finding is one case, one compiler, one level.

### `setjmp-longjmp.volatile-survives.c17.59577dbb` at `O0` on `rucc`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it

Facet [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md).

```
expected: 15
actual:   10
```

### `setjmp-longjmp.volatile-survives.c17.59577dbb` at `O1` on `rucc`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it

Facet [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md).

```
expected: 15
actual:   10
```

### `setjmp-longjmp.volatile-survives.c17.59577dbb` at `O2` on `rucc`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it

Facet [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md).

```
expected: 15
actual:   10
```

### `setjmp-longjmp.volatile-survives.c17.59577dbb` at `O3` on `rucc`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it

Facet [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md).

```
expected: 15
actual:   10
```

### `setjmp-longjmp.volatile-survives.c17.59577dbb` at `Os` on `rucc`

rucc printed the wrong answer for a case about the jump that leaves a function without returning from it

Facet [`setjmp-longjmp`](../programs/correctness/setjmp-longjmp/README.md).

```
expected: 15
actual:   10
```

### `computed-goto.dispatch-in-loop.16.c17.261af709` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.16.c17.261af709` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.16.c17.261af709` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.16.c17.261af709` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.16.c17.261af709` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %163 arrives at block17 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.2.c17.3dc137d7` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %37 arrives at block3 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.32.c17.0aa5c5d2` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %307 arrives at block33 and does not reach here [E0652]\nr...
```

### `computed-goto.dispatch-in-loop.4.c17.eb694d88` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.4.c17.eb694d88` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.4.c17.eb694d88` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.4.c17.eb694d88` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.4.c17.eb694d88` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %55 arrives at block5 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]\nruc...
```

### `computed-goto.dispatch-in-loop.8.c17.2e2b4cf5` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   rucc: error: internal error: invalid IR, @main block1 mul: %91 arrives at block9 and does not reach here [E0652]\nruc...
```

### `computed-goto.static-table.16.c17.7956dc9b` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:32: error: initializer element is not constant [E0618]\ncase.c:10:40: error: initializer element is not con...
```

### `computed-goto.static-table.16.c17.7956dc9b` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:32: error: initializer element is not constant [E0618]\ncase.c:10:40: error: initializer element is not con...
```

### `computed-goto.static-table.16.c17.7956dc9b` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:32: error: initializer element is not constant [E0618]\ncase.c:10:40: error: initializer element is not con...
```

### `computed-goto.static-table.16.c17.7956dc9b` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:32: error: initializer element is not constant [E0618]\ncase.c:10:40: error: initializer element is not con...
```

### `computed-goto.static-table.16.c17.7956dc9b` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:32: error: initializer element is not constant [E0618]\ncase.c:10:40: error: initializer element is not con...
```

### `computed-goto.static-table.2.c17.10f906d7` at `O0` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:31: error: initializer element is not constant [E0618]\ncase.c:10:39: error: initializer element is not con...
```

### `computed-goto.static-table.2.c17.10f906d7` at `O1` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:31: error: initializer element is not constant [E0618]\ncase.c:10:39: error: initializer element is not con...
```

### `computed-goto.static-table.2.c17.10f906d7` at `O2` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:31: error: initializer element is not constant [E0618]\ncase.c:10:39: error: initializer element is not con...
```

### `computed-goto.static-table.2.c17.10f906d7` at `O3` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:31: error: initializer element is not constant [E0618]\ncase.c:10:39: error: initializer element is not con...
```

### `computed-goto.static-table.2.c17.10f906d7` at `Os` on `rucc`

rucc would not compile a valid program about the address of a label, and the indirect jump through it

Facet [`computed-goto`](../programs/floor/computed-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:31: error: initializer element is not constant [E0618]\ncase.c:10:39: error: initializer element is not con...
```

And 120 more. The whole list is in `findings.sarif` and `report.json`, both of which are uploaded as artifacts by the run that produced this page.

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

| what the compiler said | cases | compiler |
|---|---|---|
| error: cannot generate code for 'main': no rule lowers a `block_addr` producing a `ptr` [E0653] | 80 | `rucc` |
| error: `__builtin_alloca` is not implemented yet [E0686] | 15 | `rucc` |
| error: `__atomic_signal_fence` is not implemented yet [E0686] | 5 | `rucc` |

