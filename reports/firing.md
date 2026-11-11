# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3709 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3709 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3709 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2288 of 3709 | 13988 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7542) |

## `rucc` at -O1

30 passes ran on 3709 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3709 | 0 |  |
| `constant-p` | 3709 | 0 |  |
| `hoist` | 3709 | 0 |  |
| `discharge` | 3709 | 0 |  |
| `dead-plane` | 3709 | 0 |  |
| `coalesce` | 3709 | 0 |  |
| `plane-sink` | 3709 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 808 of 3709 | 2226 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (778) |
| `libcall` | 24 of 3704 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `expect` | 32 of 3709 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3121 of 3709 | 89859 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (89859) |
| `image` | 44 of 3709 | 170 | 0 | brace-elision 16, function-purity 12, constant-args 4 | load from a read only object folded to what it was initialized to (81) |
| `simplify-cfg` | 2707 of 3709 | 59908 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (28086) |
| `sroa` | 531 of 3709 | 1686 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1686) |
| `simplify` | 2498 of 3709 | 25511 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7842) |
| `sccp` | 780 of 3709 | 10309 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8657) |
| `narrow` | 305 of 3709 | 1429 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1429) |
| `short-circuit-free` | 1 of 3709 | 2 | 426 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 183 of 3709 | 806 | 4778 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (689) |
| `phiopt` | 679 of 3709 | 2937 | 795 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2762) |
| `rangetest` | 14 of 3709 | 28 | 0 | kprintf-format 7, short-circuit 4, lkmm 2 | branches on comparisons of one value merged into one test (28) |
| `thread-copy` | 88 of 3709 | 276 | 4650 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (172) |
| `prune` | 32 of 3709 | 48 | 7896 | prune 14, kprintf-format 7, simd-lfind 6 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (17) |
| `canon` | 2144 of 3709 | 10712 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4595) |
| `header-copy` | 2117 of 3709 | 8059 | 1224 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4491) |
| `licm` | 736 of 3709 | 6119 | 6316 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6119) |
| `number` | 3474 of 3709 | 211121 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (155765) |
| `load-forward` | 396 of 3709 | 3806 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2735) |
| `rejoin` | 0 of 3709 | 0 | 0 |  |  |
| `constant-p` | 0 of 3709 | 0 | 0 |  |  |
| `hoist` | 0 of 3709 | 0 | 0 |  |  |
| `discharge` | 0 of 3709 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3709 | 0 | 0 |  |  |
| `coalesce` | 0 of 3709 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3709 | 0 | 0 |  |  |
| `dce` | 3339 of 3709 | 58617 | 45668 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (58616) |
| `loop-delete` | 144 of 3709 | 567 | 4633 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (296) |

## `rucc` at -O2

51 passes ran on 3725 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3725 | 0 |  |
| `rejoin` | 3725 | 0 |  |
| `hoist` | 3725 | 0 |  |
| `plane-sink` | 3725 | 0 |  |
| `split` | 3725 | 0 |  |
| `discharge` | 3725 | 0 |  |
| `dead-plane` | 3725 | 0 |  |
| `coalesce` | 3725 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 874 of 3725 | 2893 | 4188 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1281) |
| `libcall` | 24 of 3720 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 94 of 3725 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 137 of 3725 | 178 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (170) |
| `ipa-vrp` | 292 of 3725 | 414 | 780 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (413) |
| `expect` | 32 of 3725 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2456 of 3725 | 92801 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (92801) |
| `image` | 48 of 3725 | 215 | 0 | brace-elision 16, function-purity 12, constant-args 4 | load from a read only object folded to what it was initialized to (105) |
| `simplify-cfg` | 2748 of 3725 | 155700 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (70903) |
| `sroa` | 616 of 3725 | 1851 | 202 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1851) |
| `simplify` | 2614 of 3725 | 29206 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8679) |
| `sccp` | 2356 of 3725 | 78773 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (64625) |
| `narrow` | 306 of 3725 | 1432 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1432) |
| `tail-recursion` | 51 of 3725 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3725 | 36 | 128 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3501 of 3725 | 199544 | 145580 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (199540) |
| `short-circuit` | 111 of 3725 | 251 | 241 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (251) |
| `thread-implied` | 172 of 3725 | 518 | 4870 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (434) |
| `phiopt` | 704 of 3725 | 3085 | 1802 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2885) |
| `rangeswitch` | 1 of 3725 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3725 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 73 of 3725 | 137 | 4789 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (64) |
| `prune` | 50 of 3725 | 101 | 22491 | prune 14, file-scope-asm 8, kprintf-format 7 | branch the value ranges settle whichever way control reached it replaced by a jump (41) |
| `canon` | 2204 of 3725 | 14930 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (7899) |
| `header-copy` | 2177 of 3725 | 9001 | 1243 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4837) |
| `licm` | 660 of 3725 | 6257 | 14079 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6257) |
| `unroll-free` | 1162 of 3725 | 3266 | 3417 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1633) |
| `thread` | 15 of 3725 | 56 | 2504 | file-scope-asm 8, loop-unroll-shape 4, crt-and-brand 2 | edge pointed straight at the arm of the branch it arrives at that it decides (45) |
| `lanes` | 0 of 3725 | 0 | 0 |  |  |
| `number` | 3232 of 3725 | 207245 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (144815) |
| `load-forward` | 515 of 3725 | 5685 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3003) |
| `redundant-load` | 171 of 3725 | 462 | 28975 | builtin-setjmp 48, frame-size 27, target-attribute 14 | load replaced by the value an earlier load of the same address already had (242) |
| `dse` | 245 of 3725 | 1241 | 576 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1237) |
| `bswap` | 99 of 3725 | 252 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3725 | 0 | 0 |  |  |
| `constant-p` | 1 of 3725 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 40 of 3725 | 69 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (69) |
| `hoist` | 0 of 3725 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3725 | 0 | 0 |  |  |
| `split` | 0 of 3725 | 0 | 0 |  |  |
| `reassoc` | 777 of 3725 | 1715 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (973) |
| `loop-idiom` | 17 of 3720 | 17 | 1115 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3725 | 3 | 3105 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 780 of 3725 | 4714 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1699) |
| `ivopts` | 461 of 3725 | 2174 | 144 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1325) |
| `discharge` | 0 of 3725 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3725 | 0 | 0 |  |  |
| `coalesce` | 0 of 3725 | 0 | 0 |  |  |
| `gcm` | 278 of 3725 | 7065 | 608 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6024) |
| `loop-delete` | 106 of 3725 | 517 | 3311 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 63 of 3725 | 301 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (175) |

## `rucc` at -O3

51 passes ran on 3709 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3709 | 0 |  |
| `rejoin` | 3709 | 0 |  |
| `constant-p` | 3709 | 0 |  |
| `hoist` | 3709 | 0 |  |
| `plane-sink` | 3709 | 0 |  |
| `split` | 3709 | 0 |  |
| `discharge` | 3709 | 0 |  |
| `dead-plane` | 3709 | 0 |  |
| `coalesce` | 3709 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 866 of 3709 | 3809 | 3258 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2354) |
| `libcall` | 24 of 3704 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3709 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3709 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 242 of 3709 | 356 | 754 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (356) |
| `expect` | 32 of 3709 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2450 of 3709 | 94664 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (94664) |
| `image` | 49 of 3709 | 223 | 0 | brace-elision 16, function-purity 12, constant-args 4 | load from a read only object folded to what it was initialized to (105) |
| `simplify-cfg` | 2732 of 3709 | 162304 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (73389) |
| `sroa` | 617 of 3709 | 1868 | 202 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1868) |
| `simplify` | 2609 of 3709 | 29583 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8755) |
| `sccp` | 2339 of 3709 | 80319 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (65999) |
| `narrow` | 305 of 3709 | 1431 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1431) |
| `tail-recursion` | 51 of 3709 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3709 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3485 of 3709 | 202185 | 145916 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (202181) |
| `short-circuit` | 111 of 3709 | 251 | 241 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (251) |
| `thread-implied` | 172 of 3709 | 518 | 4973 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (434) |
| `phiopt` | 705 of 3709 | 3166 | 1828 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2966) |
| `rangeswitch` | 1 of 3709 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3709 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 73 of 3709 | 137 | 4892 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (64) |
| `prune` | 49 of 3709 | 99 | 22890 | prune 14, file-scope-asm 8, kprintf-format 7 | branch the value ranges settle whichever way control reached it replaced by a jump (40) |
| `canon` | 2188 of 3709 | 15222 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8056) |
| `header-copy` | 2161 of 3709 | 9211 | 1247 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4944) |
| `licm` | 658 of 3709 | 6541 | 14141 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6541) |
| `unroll` | 1166 of 3709 | 3420 | 3431 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1710) |
| `thread` | 15 of 3709 | 56 | 2512 | file-scope-asm 8, loop-unroll-shape 4, crt-and-brand 2 | edge pointed straight at the arm of the branch it arrives at that it decides (45) |
| `lanes` | 0 of 3709 | 0 | 0 |  |  |
| `number` | 3226 of 3709 | 214792 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (151509) |
| `load-forward` | 503 of 3709 | 5884 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3205) |
| `redundant-load` | 191 of 3709 | 500 | 31215 | builtin-setjmp 48, frame-size 27, crc32c 16 | load replaced by the value an earlier load of the same address already had (260) |
| `dse` | 245 of 3709 | 1241 | 592 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1237) |
| `bswap` | 99 of 3709 | 364 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3709 | 0 | 0 |  |  |
| `constant-p` | 0 of 3709 | 0 | 0 |  |  |
| `subscript` | 44 of 3709 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3709 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3709 | 0 | 0 |  |  |
| `split` | 0 of 3709 | 0 | 0 |  |  |
| `reassoc` | 776 of 3709 | 1726 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (985) |
| `loop-idiom` | 17 of 3704 | 17 | 1086 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3709 | 3 | 3136 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 774 of 3709 | 4734 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1704) |
| `ivopts` | 456 of 3709 | 2438 | 143 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1495) |
| `discharge` | 0 of 3709 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3709 | 0 | 0 |  |  |
| `coalesce` | 0 of 3709 | 0 | 0 |  |  |
| `gcm` | 277 of 3709 | 7193 | 667 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6156) |
| `loop-delete` | 106 of 3709 | 517 | 3362 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 63 of 3709 | 301 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (175) |

## `rucc` at -Os

37 passes ran on 3709 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3709 | 0 |  |
| `constant-p` | 3709 | 0 |  |
| `discharge` | 3709 | 0 |  |
| `dead-plane` | 3709 | 0 |  |
| `coalesce` | 3709 | 0 |  |
| `plane-sink` | 3709 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 808 of 3709 | 2226 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (778) |
| `libcall` | 24 of 3704 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3709 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3709 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 269 of 3709 | 372 | 815 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (371) |
| `expect` | 32 of 3709 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3111 of 3709 | 89719 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (89719) |
| `image` | 44 of 3709 | 170 | 0 | brace-elision 16, function-purity 12, constant-args 4 | load from a read only object folded to what it was initialized to (81) |
| `simplify-cfg` | 2706 of 3709 | 61489 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (29207) |
| `sroa` | 531 of 3709 | 1686 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1686) |
| `simplify` | 2481 of 3709 | 25408 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7836) |
| `sccp` | 781 of 3709 | 10310 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8657) |
| `narrow` | 305 of 3709 | 1429 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1429) |
| `tail-recursion` | 51 of 3709 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3709 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3709 | 2 | 426 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 183 of 3709 | 843 | 5984 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (726) |
| `phiopt` | 679 of 3709 | 2937 | 746 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2762) |
| `rangetest` | 13 of 3709 | 27 | 0 | kprintf-format 7, short-circuit 3, lkmm 2 | branches on comparisons of one value merged into one test (27) |
| `prune` | 98 of 3709 | 112 | 7904 | crc32c 32, target-attribute 28, prune 14 | branch the value ranges settle whichever way control reached it replaced by a jump (80) |
| `canon` | 2187 of 3709 | 10843 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4632) |
| `header-copy-small` | 2134 of 3709 | 8142 | 1224 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4524) |
| `number` | 3474 of 3709 | 209763 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (154241) |
| `load-forward` | 435 of 3709 | 3844 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2735) |
| `dse` | 223 of 3709 | 1108 | 361 | sigsetjmp 112, builtin-setjmp 48, unused-returns 20 | store removed, nothing reads it before it is overwritten or goes out of scope (1104) |
| `bswap` | 107 of 3709 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3709 | 0 | 0 |  |  |
| `constant-p` | 0 of 3709 | 0 | 0 |  |  |
| `reassoc` | 702 of 3709 | 1478 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (896) |
| `discharge` | 0 of 3709 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3709 | 0 | 0 |  |  |
| `coalesce` | 0 of 3709 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3709 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3704 | 53 | 1983 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3341 of 3709 | 61716 | 44582 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (61716) |
| `loop-delete` | 167 of 3709 | 606 | 4679 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (342) |
| `adce` | 27 of 3709 | 184 | 0 | loop-deletion 16, kprintf-format 7, function-purity 4 | instruction nothing necessary reads removed (88) |

