# What went wrong

61 findings, worst first. A finding is one case, one compiler, one level.

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

### `ifunc-static.address.dynamic.c17.77ed423b` at `O0` on `rucc`

rucc printed the wrong answer for a case about FreeBSD ifunc resolvers in static and dynamic programs

Facet [`ifunc-static`](../programs/correctness/ifunc-static/README.md).

```
expected: 1\n1923506687\n1\n120
actual:   0\n1923506687\n1\n120
```

### `unwinder.cleanup-cancel.c17.78497b80` at `O1` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 1\n321
actual:   1\n3
```

### `unwinder.cleanup-cancel.c17.78497b80` at `O2` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 1\n321
actual:   1\n3
```

### `unwinder.cleanup-cancel.c17.78497b80` at `O3` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 1\n321
actual:   1\n3
```

### `unwinder.cleanup-cancel.c17.78497b80` at `Os` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 1\n321
actual:   1\n3
```

### `unwinder.cleanup-exit.c17.dee38085` at `O1` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 46\n321
actual:   46\n3
```

### `unwinder.cleanup-exit.c17.dee38085` at `O2` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 46\n321
actual:   46\n3
```

### `unwinder.cleanup-exit.c17.dee38085` at `O3` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 46\n321
actual:   46\n3
```

### `unwinder.cleanup-exit.c17.dee38085` at `Os` on `rucc`

rucc printed the wrong answer for a case about _Unwind_Backtrace over odd frames, and cleanups under pthread_exit and cancel

Facet [`unwinder`](../programs/correctness/unwinder/README.md).

```
expected: 46\n321
actual:   46\n3
```

### `boot-abi.asm-callee-pops.c17.2c1bb800` at `O0` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-pops.c17.2c1bb800` at `O1` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-pops.c17.2c1bb800` at `O2` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-pops.c17.2c1bb800` at `O3` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-pops.c17.2c1bb800` at `Os` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-registers.c17.d1ba49bc` at `O0` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-registers.c17.d1ba49bc` at `O1` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-registers.c17.d1ba49bc` at `O2` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-registers.c17.d1ba49bc` at `O3` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-callee-registers.c17.d1ba49bc` at `Os` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-caller.c17.e8cbb0e2` at `O0` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-caller.c17.e8cbb0e2` at `O1` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-caller.c17.e8cbb0e2` at `O2` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-caller.c17.e8cbb0e2` at `O3` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.asm-caller.c17.e8cbb0e2` at `Os` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.cdecl-override.c17.09fafae6` at `O0` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.cdecl-override.c17.09fafae6` at `O1` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.cdecl-override.c17.09fafae6` at `O2` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.cdecl-override.c17.09fafae6` at `O3` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.cdecl-override.c17.09fafae6` at `Os` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.varargs.c17.2bb6149b` at `O0` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.varargs.c17.2bb6149b` at `O1` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.varargs.c17.2bb6149b` at `O2` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.varargs.c17.2bb6149b` at `O3` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.varargs.c17.2bb6149b` at `Os` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.wide-argument.c17.73591d8b` at `O0` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.wide-argument.c17.73591d8b` at `O1` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.wide-argument.c17.73591d8b` at `O2` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

### `boot-abi.wide-argument.c17.73591d8b` at `O3` on `rucc`

rucc would not compile a valid program about the i386 boot code convention: -mrtd and -mregparm=3 with assembly

Facet [`boot-abi`](../programs/correctness/boot-abi/README.md).

```
expected: the program compiles
actual:   rucc: error: i686-linux-gnu cannot be linked statically against a generated sysroot, because its libc there is a stub...
```

And 21 more. The whole list is in `findings.sarif` and `report.json`, both of which are uploaded as artifacts by the run that produced this page.

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

Nothing. No compiler in this run refused a case on those terms.
