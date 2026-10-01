# What went wrong

230 findings, worst first. A finding is one case, one compiler, one level.

### `asm-goto.branch.c17.33690266` at `O0` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:10:5: error: ‘asm’ undeclared (first use in this function)\n   10 |     asm goto(...
```

### `asm-goto.branch.c17.33690266` at `O1` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:10:5: error: ‘asm’ undeclared (first use in this function)\n   10 |     asm goto(...
```

### `asm-goto.branch.c17.33690266` at `O2` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:10:5: error: ‘asm’ undeclared (first use in this function)\n   10 |     asm goto(...
```

### `asm-goto.branch.c17.33690266` at `O3` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:10:5: error: ‘asm’ undeclared (first use in this function)\n   10 |     asm goto(...
```

### `asm-goto.branch.c17.33690266` at `Os` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:10:5: error: ‘asm’ undeclared (first use in this function)\n   10 |     asm goto(...
```

### `asm-goto.branch.c17.33690266` at `O0` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.branch.c17.33690266` at `O1` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.branch.c17.33690266` at `O2` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.branch.c17.33690266` at `O3` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.branch.c17.33690266` at `Os` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:10:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.in-loop.c17.340e5676` at `O0` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘bits’:\ncase.c:12:9: error: ‘asm’ undeclared (first use in this function)\n   12 |         asm g...
```

### `asm-goto.in-loop.c17.340e5676` at `O1` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘bits’:\ncase.c:12:9: error: ‘asm’ undeclared (first use in this function)\n   12 |         asm g...
```

### `asm-goto.in-loop.c17.340e5676` at `O2` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘bits’:\ncase.c:12:9: error: ‘asm’ undeclared (first use in this function)\n   12 |         asm g...
```

### `asm-goto.in-loop.c17.340e5676` at `O3` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘bits’:\ncase.c:12:9: error: ‘asm’ undeclared (first use in this function)\n   12 |         asm g...
```

### `asm-goto.in-loop.c17.340e5676` at `Os` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘bits’:\ncase.c:12:9: error: ‘asm’ undeclared (first use in this function)\n   12 |         asm g...
```

### `asm-goto.in-loop.c17.340e5676` at `O0` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:12:12: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.in-loop.c17.340e5676` at `O1` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:12:12: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.in-loop.c17.340e5676` at `O2` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:12:12: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.in-loop.c17.340e5676` at `O3` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:12:12: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.in-loop.c17.340e5676` at `Os` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:12:12: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.output.c17.2a15ca0e` at `O0` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:11:5: error: ‘asm’ undeclared (first use in this function)\n   11 |     asm goto(...
```

### `asm-goto.output.c17.2a15ca0e` at `O1` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:11:5: error: ‘asm’ undeclared (first use in this function)\n   11 |     asm goto(...
```

### `asm-goto.output.c17.2a15ca0e` at `O2` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:11:5: error: ‘asm’ undeclared (first use in this function)\n   11 |     asm goto(...
```

### `asm-goto.output.c17.2a15ca0e` at `O3` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:11:5: error: ‘asm’ undeclared (first use in this function)\n   11 |     asm goto(...
```

### `asm-goto.output.c17.2a15ca0e` at `Os` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘step’:\ncase.c:11:5: error: ‘asm’ undeclared (first use in this function)\n   11 |     asm goto(...
```

### `asm-goto.output.c17.2a15ca0e` at `O0` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:11:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.output.c17.2a15ca0e` at `O1` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:11:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.output.c17.2a15ca0e` at `O2` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:11:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.output.c17.2a15ca0e` at `O3` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:11:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.output.c17.2a15ca0e` at `Os` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:11:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O0` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘key_enabled’:\ncase.c:9:5: error: ‘asm’ undeclared (first use in this function)\n    9 |     asm...
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O1` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘key_enabled’:\ncase.c:9:5: error: ‘asm’ undeclared (first use in this function)\n    9 |     asm...
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O2` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘key_enabled’:\ncase.c:9:5: error: ‘asm’ undeclared (first use in this function)\n    9 |     asm...
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O3` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘key_enabled’:\ncase.c:9:5: error: ‘asm’ undeclared (first use in this function)\n    9 |     asm...
```

### `asm-goto.static-key-off.c17.c699bc6d` at `Os` on `gcc-16`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c: In function ‘key_enabled’:\ncase.c:9:5: error: ‘asm’ undeclared (first use in this function)\n    9 |     asm...
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O0` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:9:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O1` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:9:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O2` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:9:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.static-key-off.c17.c699bc6d` at `O3` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:9:8: error: expected `;`, found `goto` [E0400]
```

### `asm-goto.static-key-off.c17.c699bc6d` at `Os` on `rucc`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs

Facet [`asm-goto`](../programs/correctness/asm-goto/README.md).

```
expected: the program compiles
actual:   case.c:9:8: error: expected `;`, found `goto` [E0400]
```

And 190 more. The whole list is in `findings.sarif` and `report.json`, both of which are uploaded as artifacts by the run that produced this page.

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

Nothing. No compiler in this run refused a case on those terms.
