# What went wrong

20 findings, worst first. A finding is one case, one compiler, one level.

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O0` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O1` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O2` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O3` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `Os` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.is-const.c17.0749e5ff` at `O0` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 2515880359
actual:   2563465191
```

### `null-pointer-constant.is-const.c17.0749e5ff` at `O1` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 2515880359
actual:   2563465191
```

### `null-pointer-constant.is-const.c17.0749e5ff` at `O2` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 2515880359
actual:   2563465191
```

### `null-pointer-constant.is-const.c17.0749e5ff` at `O3` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 2515880359
actual:   2563465191
```

### `null-pointer-constant.is-const.c17.0749e5ff` at `Os` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 2515880359
actual:   2563465191
```

### `null-pointer-constant.is-constexpr.c17.b3a54e24` at `O0` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 991576615
actual:   81932839
```

### `null-pointer-constant.is-constexpr.c17.b3a54e24` at `O1` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 991576615
actual:   81932839
```

### `null-pointer-constant.is-constexpr.c17.b3a54e24` at `O2` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 991576615
actual:   81932839
```

### `null-pointer-constant.is-constexpr.c17.b3a54e24` at `O3` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 991576615
actual:   81932839
```

### `null-pointer-constant.is-constexpr.c17.b3a54e24` at `Os` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 991576615
actual:   81932839
```

### `const-ice.const-local.c17.d372496d` at `O0` on `rucc`

rucc would not compile a valid program about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: the program compiles
actual:   case.c:16:19: error: enumerator value for 'RK_N' is not an integer constant [E0564]\ncase.c:16:29: error: enumerator ...
```

### `const-ice.const-local.c17.d372496d` at `O1` on `rucc`

rucc would not compile a valid program about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: the program compiles
actual:   case.c:16:19: error: enumerator value for 'RK_N' is not an integer constant [E0564]\ncase.c:16:29: error: enumerator ...
```

### `const-ice.const-local.c17.d372496d` at `O2` on `rucc`

rucc would not compile a valid program about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: the program compiles
actual:   case.c:16:19: error: enumerator value for 'RK_N' is not an integer constant [E0564]\ncase.c:16:29: error: enumerator ...
```

### `const-ice.const-local.c17.d372496d` at `O3` on `rucc`

rucc would not compile a valid program about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: the program compiles
actual:   case.c:16:19: error: enumerator value for 'RK_N' is not an integer constant [E0564]\ncase.c:16:29: error: enumerator ...
```

### `const-ice.const-local.c17.d372496d` at `Os` on `rucc`

rucc would not compile a valid program about sizeof, offsetof and const objects in integer constant expressions

Facet [`const-ice`](../programs/correctness/const-ice/README.md).

```
expected: the program compiles
actual:   case.c:16:19: error: enumerator value for 'RK_N' is not an integer constant [E0564]\ncase.c:16:29: error: enumerator ...
```

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

Nothing. No compiler in this run refused a case on those terms.
