# brace-elision

initializers that leave out the inner braces of an aggregate. Part of the floor phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`brace-elision.character-matrix-flat.block.c17.6c97fe81`](brace-elision.character-matrix-flat.block.c17.6c97fe81.c) | shape=character-matrix-flat, scope=block | c17 | `99 100 0 0` |
| [`brace-elision.character-matrix-flat.file.c17.5b00369d`](brace-elision.character-matrix-flat.file.c17.5b00369d.c) | shape=character-matrix-flat, scope=file | c17 | `99 100 0 0` |
| [`brace-elision.character-matrix.block.c17.c7332ac5`](brace-elision.character-matrix.block.c17.c7332ac5.c) | shape=character-matrix, scope=block | c17 | `97 0 101 ...` and 3 more lines |
| [`brace-elision.character-matrix.file.c17.9eb03a86`](brace-elision.character-matrix.file.c17.9eb03a86.c) | shape=character-matrix, scope=file | c17 | `97 0 101 ...` and 3 more lines |
| [`brace-elision.integer-matrix.block.c17.aca7a371`](brace-elision.integer-matrix.block.c17.aca7a371.c) | shape=integer-matrix, scope=block | c17 | `3 4 0 0` |
| [`brace-elision.integer-matrix.file.c17.80cc7a0d`](brace-elision.integer-matrix.file.c17.80cc7a0d.c) | shape=integer-matrix, scope=file | c17 | `3 4 0 0` |
| [`brace-elision.pointer-cube.block.c17.caa65319`](brace-elision.pointer-cube.block.c17.caa65319.c) | shape=pointer-cube, scope=block | c17 | `97 100 101 0` |
| [`brace-elision.pointer-cube.file.c17.c38d7a2c`](brace-elision.pointer-cube.file.c17.c38d7a2c.c) | shape=pointer-cube, scope=file | c17 | `97 100 101 0` |
| [`brace-elision.pointer-matrix-mixed.block.c17.5ae53c96`](brace-elision.pointer-matrix-mixed.block.c17.5ae53c96.c) | shape=pointer-matrix-mixed, scope=block | c17 | `97 0 99 ...` and 3 more lines |
| [`brace-elision.pointer-matrix-mixed.file.c17.6080c84c`](brace-elision.pointer-matrix-mixed.file.c17.6080c84c.c) | shape=pointer-matrix-mixed, scope=file | c17 | `97 0 99 ...` and 3 more lines |
| [`brace-elision.pointer-matrix-unsized.block.c17.24f8fb3a`](brace-elision.pointer-matrix-unsized.block.c17.24f8fb3a.c) | shape=pointer-matrix-unsized, scope=block | c17 | `2 98 99 0` |
| [`brace-elision.pointer-matrix-unsized.file.c17.291b18e1`](brace-elision.pointer-matrix-unsized.file.c17.291b18e1.c) | shape=pointer-matrix-unsized, scope=file | c17 | `2 98 99 0` |
| [`brace-elision.pointer-matrix.block.c17.eb38ab64`](brace-elision.pointer-matrix.block.c17.eb38ab64.c) | shape=pointer-matrix, scope=block | c17 | `97 98 99 100` |
| [`brace-elision.pointer-matrix.file.c17.1d98b4f0`](brace-elision.pointer-matrix.file.c17.1d98b4f0.c) | shape=pointer-matrix, scope=file | c17 | `97 98 99 100` |
| [`brace-elision.record-of-pointers.block.c17.c9651bfb`](brace-elision.record-of-pointers.block.c17.c9651bfb.c) | shape=record-of-pointers, scope=block | c17 | `3 112 1 ...` and 5 more lines |
| [`brace-elision.record-of-pointers.file.c17.6e50f995`](brace-elision.record-of-pointers.file.c17.6e50f995.c) | shape=record-of-pointers, scope=file | c17 | `3 112 1 ...` and 5 more lines |
| [`brace-elision.record-with-character-array.block.c17.d56e817b`](brace-elision.record-with-character-array.block.c17.d56e817b.c) | shape=record-with-character-array, scope=block | c17 | `2 101 0 ...` and 3 more lines |
| [`brace-elision.record-with-character-array.file.c17.a6baf5ba`](brace-elision.record-with-character-array.file.c17.a6baf5ba.c) | shape=record-with-character-array, scope=file | c17 | `2 101 0 ...` and 3 more lines |
| [`brace-elision.record-with-pointer-array.block.c17.2fde4215`](brace-elision.record-with-pointer-array.block.c17.2fde4215.c) | shape=record-with-pointer-array, scope=block | c17 | `110 110 1 ...` and 3 more lines |
| [`brace-elision.record-with-pointer-array.file.c17.2d12fb9b`](brace-elision.record-with-pointer-array.file.c17.2d12fb9b.c) | shape=record-with-pointer-array, scope=file | c17 | `110 110 1 ...` and 3 more lines |

