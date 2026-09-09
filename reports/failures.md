# What went wrong

270 findings, worst first. A finding is one case, one compiler, one level.

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

### `atomics.stdatomic-flag.c17.6551714c` at `O0` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic-flag.c17.6551714c` at `O1` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic-flag.c17.6551714c` at `O2` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic-flag.c17.6551714c` at `O3` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic-flag.c17.6551714c` at `Os` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i32.c17.95fbc2ae` at `O0` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i32.c17.95fbc2ae` at `O1` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i32.c17.95fbc2ae` at `O2` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i32.c17.95fbc2ae` at `O3` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i32.c17.95fbc2ae` at `Os` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i64.c17.19020add` at `O0` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i64.c17.19020add` at `O1` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i64.c17.19020add` at `O2` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i64.c17.19020add` at `O3` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.i64.c17.19020add` at `Os` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u32.c17.efd489a1` at `O0` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u32.c17.efd489a1` at `O1` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u32.c17.efd489a1` at `O2` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u32.c17.efd489a1` at `O3` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u32.c17.efd489a1` at `Os` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u64.c17.53c8ec77` at `O0` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u64.c17.53c8ec77` at `O1` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u64.c17.53c8ec77` at `O2` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u64.c17.53c8ec77` at `O3` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
```

### `atomics.stdatomic.u64.c17.53c8ec77` at `Os` on `rucc`

rucc would not compile a valid program about the atomic builtins at every ordering, and the header over them

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:4:1: error: `stdatomic.h` file not found [E0341]\ncase.c:4:1: note: searched: <builtin>, /usr/local/include, /...
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

And 230 more. The whole list is in `findings.sarif` and `report.json`, both of which are uploaded as artifacts by the run that produced this page.

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

| what the compiler said | cases | compiler |
|---|---|---|
| error: cannot generate code for 'main': no rule lowers a `block_addr` producing a `ptr` [E0653] | 80 | `rucc` |
| error: cannot generate code for 'main': no rule lowers a `stacksave` producing a `ptr` [E0653] | 30 | `rucc` |
| error: `__builtin_alloca` is not implemented yet [E0686] | 15 | `rucc` |
| error: `__atomic_signal_fence` is not implemented yet [E0686] | 5 | `rucc` |
| error: cannot generate code for 'total': no rule lowers a `va_arg` producing a `f80` [E0653] | 5 | `rucc` |

