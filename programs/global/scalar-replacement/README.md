# scalar-replacement

turning a non-escaping local back into a value. Part of the global phase of the M4 plan.

20 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`scalar-replacement.i32.address-taken.c17.75813675`](scalar-replacement.i32.address-taken.c17.75813675.c) | type=i32, shape=address-taken | c17 | `17` |
| [`scalar-replacement.i32.array-constant-index.c17.dcb4240c`](scalar-replacement.i32.array-constant-index.c17.dcb4240c.c) | type=i32, shape=array-constant-index | c17 | `17` |
| [`scalar-replacement.i32.nested-struct.c17.a674b49a`](scalar-replacement.i32.nested-struct.c17.a674b49a.c) | type=i32, shape=nested-struct | c17 | `17` |
| [`scalar-replacement.i32.struct.c17.f9744029`](scalar-replacement.i32.struct.c17.f9744029.c) | type=i32, shape=struct | c17 | `17` |
| [`scalar-replacement.i32.union.c17.dfe6f52e`](scalar-replacement.i32.union.c17.dfe6f52e.c) | type=i32, shape=union | c17 | `17` |
| [`scalar-replacement.i64.address-taken.c17.2cebd109`](scalar-replacement.i64.address-taken.c17.2cebd109.c) | type=i64, shape=address-taken | c17 | `17` |
| [`scalar-replacement.i64.array-constant-index.c17.1c6a754f`](scalar-replacement.i64.array-constant-index.c17.1c6a754f.c) | type=i64, shape=array-constant-index | c17 | `17` |
| [`scalar-replacement.i64.nested-struct.c17.aee0fdbb`](scalar-replacement.i64.nested-struct.c17.aee0fdbb.c) | type=i64, shape=nested-struct | c17 | `17` |
| [`scalar-replacement.i64.struct.c17.cde95d9f`](scalar-replacement.i64.struct.c17.cde95d9f.c) | type=i64, shape=struct | c17 | `17` |
| [`scalar-replacement.i64.union.c17.40630a14`](scalar-replacement.i64.union.c17.40630a14.c) | type=i64, shape=union | c17 | `17` |
| [`scalar-replacement.u32.address-taken.c17.e18da6e8`](scalar-replacement.u32.address-taken.c17.e18da6e8.c) | type=u32, shape=address-taken | c17 | `17` |
| [`scalar-replacement.u32.array-constant-index.c17.8af42b12`](scalar-replacement.u32.array-constant-index.c17.8af42b12.c) | type=u32, shape=array-constant-index | c17 | `17` |
| [`scalar-replacement.u32.nested-struct.c17.07bd3bec`](scalar-replacement.u32.nested-struct.c17.07bd3bec.c) | type=u32, shape=nested-struct | c17 | `17` |
| [`scalar-replacement.u32.struct.c17.c9cba97f`](scalar-replacement.u32.struct.c17.c9cba97f.c) | type=u32, shape=struct | c17 | `17` |
| [`scalar-replacement.u32.union.c17.ccc75bc4`](scalar-replacement.u32.union.c17.ccc75bc4.c) | type=u32, shape=union | c17 | `17` |
| [`scalar-replacement.u64.address-taken.c17.779dadfc`](scalar-replacement.u64.address-taken.c17.779dadfc.c) | type=u64, shape=address-taken | c17 | `17` |
| [`scalar-replacement.u64.array-constant-index.c17.25f1e065`](scalar-replacement.u64.array-constant-index.c17.25f1e065.c) | type=u64, shape=array-constant-index | c17 | `17` |
| [`scalar-replacement.u64.nested-struct.c17.466fe5a4`](scalar-replacement.u64.nested-struct.c17.466fe5a4.c) | type=u64, shape=nested-struct | c17 | `17` |
| [`scalar-replacement.u64.struct.c17.654cc79d`](scalar-replacement.u64.struct.c17.654cc79d.c) | type=u64, shape=struct | c17 | `17` |
| [`scalar-replacement.u64.union.c17.d5216f9b`](scalar-replacement.u64.union.c17.d5216f9b.c) | type=u64, shape=union | c17 | `17` |

