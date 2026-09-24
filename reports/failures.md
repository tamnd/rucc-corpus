# What went wrong

5 findings, worst first. A finding is one case, one compiler, one level.

### `atomics.fence.c17.6b7c6e1f` at `O0` on `rucc`

rucc has not built the part of the atomic builtins at every ordering, and the header over them this case needs yet

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:16:5: error: `__atomic_signal_fence` is not implemented yet [E0686]\ncase.c:16:5: note: a call to it would go ...
```

### `atomics.fence.c17.6b7c6e1f` at `O1` on `rucc`

rucc has not built the part of the atomic builtins at every ordering, and the header over them this case needs yet

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:16:5: error: `__atomic_signal_fence` is not implemented yet [E0686]\ncase.c:16:5: note: a call to it would go ...
```

### `atomics.fence.c17.6b7c6e1f` at `O2` on `rucc`

rucc has not built the part of the atomic builtins at every ordering, and the header over them this case needs yet

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:16:5: error: `__atomic_signal_fence` is not implemented yet [E0686]\ncase.c:16:5: note: a call to it would go ...
```

### `atomics.fence.c17.6b7c6e1f` at `O3` on `rucc`

rucc has not built the part of the atomic builtins at every ordering, and the header over them this case needs yet

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:16:5: error: `__atomic_signal_fence` is not implemented yet [E0686]\ncase.c:16:5: note: a call to it would go ...
```

### `atomics.fence.c17.6b7c6e1f` at `Os` on `rucc`

rucc has not built the part of the atomic builtins at every ordering, and the header over them this case needs yet

Facet [`atomics`](../programs/correctness/atomics/README.md).

```
expected: the program compiles
actual:   case.c:16:5: error: `__atomic_signal_fence` is not implemented yet [E0686]\ncase.c:16:5: note: a call to it would go ...
```

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

| what the compiler said | cases | compiler |
|---|---|---|
| error: `__atomic_signal_fence` is not implemented yet [E0686] | 5 | `rucc` |

