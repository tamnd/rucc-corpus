# What went wrong

7 findings, worst first. A finding is one case, one compiler, one level.

### `const-ice.const-local.c17.d372496d` at `O0` on `rucc`

rucc printed the wrong answer for a case about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: 16992
actual:   15083
```

### `const-ice.const-local.c17.d372496d` at `O1` on `rucc`

rucc printed the wrong answer for a case about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: 16992
actual:   15083
```

### `const-ice.const-local.c17.d372496d` at `O2` on `rucc`

rucc printed the wrong answer for a case about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: 16992
actual:   15083
```

### `const-ice.const-local.c17.d372496d` at `O3` on `rucc`

rucc printed the wrong answer for a case about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: 16992
actual:   15083
```

### `const-ice.const-local.c17.d372496d` at `Os` on `rucc`

rucc printed the wrong answer for a case about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: 16992
actual:   15083
```

### `general-regs-only.struct-copy.c17.85360170` at `Os` on `rucc`

rucc printed the wrong answer for a case about programs built with -mgeneral-regs-only that check no vector register moved

Facet [`general-regs-only`](../programs/correctness/general-regs-only/README.md).

```
expected: 2188897103
actual:   0
```

### `general-regs-only.zeroed.c17.ca262554` at `Os` on `rucc`

rucc printed the wrong answer for a case about programs built with -mgeneral-regs-only that check no vector register moved

Facet [`general-regs-only`](../programs/correctness/general-regs-only/README.md).

```
expected: 3575246495
actual:   0
```

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

Nothing. No compiler in this run refused a case on those terms.
