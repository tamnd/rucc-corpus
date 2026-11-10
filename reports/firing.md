# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3673 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3673 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3673 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2288 of 3673 | 13988 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7542) |

## `rucc` at -O1

30 passes ran on 3673 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3673 | 0 |  |
| `constant-p` | 3673 | 0 |  |
| `hoist` | 3673 | 0 |  |
| `discharge` | 3673 | 0 |  |
| `dead-plane` | 3673 | 0 |  |
| `coalesce` | 3673 | 0 |  |
| `plane-sink` | 3673 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 793 of 3673 | 2171 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (777) |
| `libcall` | 24 of 3668 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `expect` | 32 of 3673 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3085 of 3673 | 89228 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (89228) |
| `image` | 28 of 3673 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `simplify-cfg` | 2692 of 3673 | 59518 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (27991) |
| `sroa` | 507 of 3673 | 1660 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1660) |
| `simplify` | 2482 of 3673 | 25453 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7842) |
| `sccp` | 775 of 3673 | 10298 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8656) |
| `narrow` | 305 of 3673 | 1429 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1429) |
| `short-circuit-free` | 1 of 3673 | 2 | 426 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 183 of 3673 | 806 | 4778 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (689) |
| `phiopt` | 679 of 3673 | 2937 | 765 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2762) |
| `rangetest` | 14 of 3673 | 28 | 0 | kprintf-format 7, short-circuit 4, lkmm 2 | branches on comparisons of one value merged into one test (28) |
| `thread-copy` | 88 of 3673 | 276 | 4650 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (172) |
| `prune` | 32 of 3673 | 48 | 7871 | prune 14, kprintf-format 7, simd-lfind 6 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (17) |
| `canon` | 2144 of 3673 | 10712 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4595) |
| `header-copy` | 2117 of 3673 | 8059 | 1224 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4491) |
| `licm` | 736 of 3673 | 6119 | 6316 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6119) |
| `number` | 3438 of 3673 | 210119 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (154932) |
| `load-forward` | 394 of 3673 | 3802 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2735) |
| `rejoin` | 0 of 3673 | 0 | 0 |  |  |
| `constant-p` | 0 of 3673 | 0 | 0 |  |  |
| `hoist` | 0 of 3673 | 0 | 0 |  |  |
| `discharge` | 0 of 3673 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3673 | 0 | 0 |  |  |
| `coalesce` | 0 of 3673 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3673 | 0 | 0 |  |  |
| `dce` | 3303 of 3673 | 57903 | 45513 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (57902) |
| `loop-delete` | 144 of 3673 | 567 | 4633 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (296) |

## `rucc` at -O2

51 passes ran on 3689 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3689 | 0 |  |
| `rejoin` | 3689 | 0 |  |
| `hoist` | 3689 | 0 |  |
| `plane-sink` | 3689 | 0 |  |
| `split` | 3689 | 0 |  |
| `discharge` | 3689 | 0 |  |
| `dead-plane` | 3689 | 0 |  |
| `coalesce` | 3689 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 859 of 3689 | 2838 | 4188 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1226) |
| `libcall` | 24 of 3684 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 94 of 3689 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 137 of 3689 | 178 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (170) |
| `ipa-vrp` | 292 of 3689 | 414 | 780 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (413) |
| `expect` | 32 of 3689 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2420 of 3689 | 92170 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (92170) |
| `image` | 32 of 3689 | 91 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (41) |
| `simplify-cfg` | 2733 of 3689 | 155310 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (70808) |
| `sroa` | 592 of 3689 | 1825 | 202 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1825) |
| `simplify` | 2598 of 3689 | 29148 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8679) |
| `sccp` | 2351 of 3689 | 78762 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (64615) |
| `narrow` | 306 of 3689 | 1432 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1432) |
| `tail-recursion` | 51 of 3689 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3689 | 36 | 128 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3465 of 3689 | 197946 | 145119 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (197942) |
| `short-circuit` | 111 of 3689 | 251 | 241 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (251) |
| `thread-implied` | 172 of 3689 | 518 | 4870 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (434) |
| `phiopt` | 704 of 3689 | 3085 | 1747 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2885) |
| `rangeswitch` | 1 of 3689 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3689 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 73 of 3689 | 137 | 4789 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (64) |
| `prune` | 50 of 3689 | 101 | 22456 | prune 14, file-scope-asm 8, kprintf-format 7 | branch the value ranges settle whichever way control reached it replaced by a jump (41) |
| `canon` | 2204 of 3689 | 14930 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (7899) |
| `header-copy` | 2177 of 3689 | 9001 | 1243 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4837) |
| `licm` | 660 of 3689 | 6257 | 14079 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6257) |
| `unroll-free` | 1162 of 3689 | 3266 | 3417 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1633) |
| `thread` | 15 of 3689 | 56 | 2504 | file-scope-asm 8, loop-unroll-shape 4, crt-and-brand 2 | edge pointed straight at the arm of the branch it arrives at that it decides (45) |
| `lanes` | 0 of 3689 | 0 | 0 |  |  |
| `number` | 3197 of 3689 | 207110 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (144776) |
| `load-forward` | 513 of 3689 | 5681 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3003) |
| `redundant-load` | 171 of 3689 | 462 | 28961 | builtin-setjmp 48, frame-size 27, target-attribute 14 | load replaced by the value an earlier load of the same address already had (242) |
| `dse` | 243 of 3689 | 1239 | 576 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1235) |
| `bswap` | 99 of 3689 | 252 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3689 | 0 | 0 |  |  |
| `constant-p` | 1 of 3689 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 40 of 3689 | 69 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (69) |
| `hoist` | 0 of 3689 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3689 | 0 | 0 |  |  |
| `split` | 0 of 3689 | 0 | 0 |  |  |
| `reassoc` | 777 of 3689 | 1715 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (973) |
| `loop-idiom` | 17 of 3684 | 17 | 1115 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3689 | 3 | 3105 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 780 of 3689 | 4714 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1699) |
| `ivopts` | 461 of 3689 | 2174 | 144 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1325) |
| `discharge` | 0 of 3689 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3689 | 0 | 0 |  |  |
| `coalesce` | 0 of 3689 | 0 | 0 |  |  |
| `gcm` | 274 of 3689 | 7060 | 608 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6024) |
| `loop-delete` | 106 of 3689 | 517 | 3311 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 59 of 3689 | 291 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (165) |

## `rucc` at -O3

51 passes ran on 3673 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3673 | 0 |  |
| `rejoin` | 3673 | 0 |  |
| `constant-p` | 3673 | 0 |  |
| `hoist` | 3673 | 0 |  |
| `plane-sink` | 3673 | 0 |  |
| `split` | 3673 | 0 |  |
| `discharge` | 3673 | 0 |  |
| `dead-plane` | 3673 | 0 |  |
| `coalesce` | 3673 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 851 of 3673 | 3754 | 3258 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2299) |
| `libcall` | 24 of 3668 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3673 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3673 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 242 of 3673 | 356 | 754 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (356) |
| `expect` | 32 of 3673 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2414 of 3673 | 94033 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (94033) |
| `image` | 33 of 3673 | 99 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `simplify-cfg` | 2717 of 3673 | 161914 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (73294) |
| `sroa` | 593 of 3673 | 1842 | 202 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1842) |
| `simplify` | 2593 of 3673 | 29525 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8755) |
| `sccp` | 2334 of 3673 | 80308 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (65989) |
| `narrow` | 305 of 3673 | 1431 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1431) |
| `tail-recursion` | 51 of 3673 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3673 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3449 of 3673 | 200587 | 145455 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (200583) |
| `short-circuit` | 111 of 3673 | 251 | 241 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (251) |
| `thread-implied` | 172 of 3673 | 518 | 4973 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (434) |
| `phiopt` | 705 of 3673 | 3166 | 1773 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2966) |
| `rangeswitch` | 1 of 3673 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3673 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 73 of 3673 | 137 | 4892 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (64) |
| `prune` | 49 of 3673 | 99 | 22855 | prune 14, file-scope-asm 8, kprintf-format 7 | branch the value ranges settle whichever way control reached it replaced by a jump (40) |
| `canon` | 2188 of 3673 | 15222 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8056) |
| `header-copy` | 2161 of 3673 | 9211 | 1247 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4944) |
| `licm` | 658 of 3673 | 6541 | 14141 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6541) |
| `unroll` | 1166 of 3673 | 3420 | 3431 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1710) |
| `thread` | 15 of 3673 | 56 | 2512 | file-scope-asm 8, loop-unroll-shape 4, crt-and-brand 2 | edge pointed straight at the arm of the branch it arrives at that it decides (45) |
| `lanes` | 0 of 3673 | 0 | 0 |  |  |
| `number` | 3191 of 3673 | 214657 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (151470) |
| `load-forward` | 501 of 3673 | 5880 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3205) |
| `redundant-load` | 191 of 3673 | 500 | 31201 | builtin-setjmp 48, frame-size 27, crc32c 16 | load replaced by the value an earlier load of the same address already had (260) |
| `dse` | 243 of 3673 | 1239 | 592 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1235) |
| `bswap` | 99 of 3673 | 364 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3673 | 0 | 0 |  |  |
| `constant-p` | 0 of 3673 | 0 | 0 |  |  |
| `subscript` | 44 of 3673 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3673 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3673 | 0 | 0 |  |  |
| `split` | 0 of 3673 | 0 | 0 |  |  |
| `reassoc` | 776 of 3673 | 1726 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (985) |
| `loop-idiom` | 17 of 3668 | 17 | 1086 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3673 | 3 | 3136 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 774 of 3673 | 4734 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1704) |
| `ivopts` | 456 of 3673 | 2438 | 143 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1495) |
| `discharge` | 0 of 3673 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3673 | 0 | 0 |  |  |
| `coalesce` | 0 of 3673 | 0 | 0 |  |  |
| `gcm` | 273 of 3673 | 7188 | 667 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6156) |
| `loop-delete` | 106 of 3673 | 517 | 3362 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 59 of 3673 | 291 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (165) |

## `rucc` at -Os

37 passes ran on 3673 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3673 | 0 |  |
| `constant-p` | 3673 | 0 |  |
| `discharge` | 3673 | 0 |  |
| `dead-plane` | 3673 | 0 |  |
| `coalesce` | 3673 | 0 |  |
| `plane-sink` | 3673 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 793 of 3673 | 2171 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (777) |
| `libcall` | 24 of 3668 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3673 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3673 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 269 of 3673 | 372 | 815 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (371) |
| `expect` | 32 of 3673 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3075 of 3673 | 89088 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (89088) |
| `image` | 28 of 3673 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `simplify-cfg` | 2691 of 3673 | 61099 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (29112) |
| `sroa` | 507 of 3673 | 1660 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1660) |
| `simplify` | 2465 of 3673 | 25350 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7836) |
| `sccp` | 776 of 3673 | 10299 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8656) |
| `narrow` | 305 of 3673 | 1429 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1429) |
| `tail-recursion` | 51 of 3673 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3673 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3673 | 2 | 426 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 183 of 3673 | 843 | 5984 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (726) |
| `phiopt` | 679 of 3673 | 2937 | 716 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2762) |
| `rangetest` | 13 of 3673 | 27 | 0 | kprintf-format 7, short-circuit 3, lkmm 2 | branches on comparisons of one value merged into one test (27) |
| `prune` | 98 of 3673 | 112 | 7879 | crc32c 32, target-attribute 28, prune 14 | branch the value ranges settle whichever way control reached it replaced by a jump (80) |
| `canon` | 2187 of 3673 | 10843 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4632) |
| `header-copy-small` | 2134 of 3673 | 8142 | 1224 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4524) |
| `number` | 3438 of 3673 | 208761 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (153408) |
| `load-forward` | 433 of 3673 | 3840 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2735) |
| `dse` | 221 of 3673 | 1106 | 361 | sigsetjmp 112, builtin-setjmp 48, unused-returns 20 | store removed, nothing reads it before it is overwritten or goes out of scope (1102) |
| `bswap` | 107 of 3673 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3673 | 0 | 0 |  |  |
| `constant-p` | 0 of 3673 | 0 | 0 |  |  |
| `reassoc` | 702 of 3673 | 1478 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (896) |
| `discharge` | 0 of 3673 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3673 | 0 | 0 |  |  |
| `coalesce` | 0 of 3673 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3673 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3668 | 53 | 1983 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3305 of 3673 | 61000 | 44429 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (61000) |
| `loop-delete` | 167 of 3673 | 606 | 4679 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (342) |
| `adce` | 27 of 3673 | 184 | 0 | loop-deletion 16, kprintf-format 7, function-purity 4 | instruction nothing necessary reads removed (88) |

