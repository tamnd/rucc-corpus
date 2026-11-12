# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3798 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3798 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3798 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2288 of 3798 | 13988 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7542) |

## `rucc` at -O1

30 passes ran on 3752 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3752 | 0 |  |
| `constant-p` | 3752 | 0 |  |
| `hoist` | 3752 | 0 |  |
| `discharge` | 3752 | 0 |  |
| `dead-plane` | 3752 | 0 |  |
| `coalesce` | 3752 | 0 |  |
| `plane-sink` | 3752 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 851 of 3752 | 2269 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (821) |
| `libcall` | 24 of 3747 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `expect` | 32 of 3752 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3164 of 3752 | 90031 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (90031) |
| `image` | 87 of 3752 | 213 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (124) |
| `simplify-cfg` | 2750 of 3752 | 60037 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (28129) |
| `sroa` | 541 of 3752 | 1696 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1696) |
| `simplify` | 2498 of 3752 | 25511 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7842) |
| `sccp` | 780 of 3752 | 10309 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8657) |
| `narrow` | 305 of 3752 | 1429 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1429) |
| `short-circuit-free` | 1 of 3752 | 2 | 426 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 183 of 3752 | 806 | 4778 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (689) |
| `phiopt` | 679 of 3752 | 2937 | 795 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2762) |
| `rangetest` | 14 of 3752 | 28 | 0 | kprintf-format 7, short-circuit 4, lkmm 2 | branches on comparisons of one value merged into one test (28) |
| `thread-copy` | 88 of 3752 | 276 | 4650 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (172) |
| `prune` | 32 of 3752 | 48 | 7896 | prune 14, kprintf-format 7, simd-lfind 6 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (17) |
| `canon` | 2144 of 3752 | 10712 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4595) |
| `header-copy` | 2117 of 3752 | 8059 | 1224 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4491) |
| `licm` | 736 of 3752 | 6119 | 6316 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6119) |
| `number` | 3517 of 3752 | 211193 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (155837) |
| `load-forward` | 396 of 3752 | 3806 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2735) |
| `rejoin` | 0 of 3752 | 0 | 0 |  |  |
| `constant-p` | 0 of 3752 | 0 | 0 |  |  |
| `hoist` | 0 of 3752 | 0 | 0 |  |  |
| `discharge` | 0 of 3752 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3752 | 0 | 0 |  |  |
| `coalesce` | 0 of 3752 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3752 | 0 | 0 |  |  |
| `dce` | 3382 of 3752 | 58923 | 45711 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (58922) |
| `loop-delete` | 144 of 3752 | 567 | 4633 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (296) |

## `rucc` at -O2

51 passes ran on 3768 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3768 | 0 |  |
| `rejoin` | 3768 | 0 |  |
| `hoist` | 3768 | 0 |  |
| `plane-sink` | 3768 | 0 |  |
| `split` | 3768 | 0 |  |
| `discharge` | 3768 | 0 |  |
| `dead-plane` | 3768 | 0 |  |
| `coalesce` | 3768 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 917 of 3768 | 2936 | 4188 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1324) |
| `libcall` | 24 of 3763 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 94 of 3768 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 137 of 3768 | 178 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (170) |
| `ipa-vrp` | 292 of 3768 | 414 | 780 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (413) |
| `expect` | 32 of 3768 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2499 of 3768 | 92973 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (92973) |
| `image` | 91 of 3768 | 258 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (148) |
| `simplify-cfg` | 2791 of 3768 | 155829 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (70946) |
| `sroa` | 626 of 3768 | 1861 | 202 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1861) |
| `simplify` | 2614 of 3768 | 29206 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8679) |
| `sccp` | 2356 of 3768 | 78773 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (64625) |
| `narrow` | 306 of 3768 | 1432 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1432) |
| `tail-recursion` | 51 of 3768 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3768 | 36 | 128 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3544 of 3768 | 199922 | 145709 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (199918) |
| `short-circuit` | 111 of 3768 | 251 | 241 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (251) |
| `thread-implied` | 172 of 3768 | 518 | 4870 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (434) |
| `phiopt` | 704 of 3768 | 3085 | 1802 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2885) |
| `rangeswitch` | 1 of 3768 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3768 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 73 of 3768 | 137 | 4789 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (64) |
| `prune` | 50 of 3768 | 101 | 22491 | prune 14, file-scope-asm 8, kprintf-format 7 | branch the value ranges settle whichever way control reached it replaced by a jump (41) |
| `canon` | 2204 of 3768 | 14930 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (7899) |
| `header-copy` | 2177 of 3768 | 9001 | 1243 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4837) |
| `licm` | 660 of 3768 | 6257 | 14079 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6257) |
| `unroll-free` | 1162 of 3768 | 3266 | 3417 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1633) |
| `thread` | 15 of 3768 | 56 | 2504 | file-scope-asm 8, loop-unroll-shape 4, crt-and-brand 2 | edge pointed straight at the arm of the branch it arrives at that it decides (45) |
| `lanes` | 0 of 3768 | 0 | 0 |  |  |
| `number` | 3232 of 3768 | 207245 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (144815) |
| `load-forward` | 515 of 3768 | 5685 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3003) |
| `redundant-load` | 171 of 3768 | 462 | 28975 | builtin-setjmp 48, frame-size 27, target-attribute 14 | load replaced by the value an earlier load of the same address already had (242) |
| `dse` | 245 of 3768 | 1241 | 576 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1237) |
| `bswap` | 99 of 3768 | 252 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3768 | 0 | 0 |  |  |
| `constant-p` | 1 of 3768 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 40 of 3768 | 69 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (69) |
| `hoist` | 0 of 3768 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3768 | 0 | 0 |  |  |
| `split` | 0 of 3768 | 0 | 0 |  |  |
| `reassoc` | 777 of 3768 | 1715 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (973) |
| `loop-idiom` | 17 of 3763 | 17 | 1115 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3768 | 3 | 3105 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 780 of 3768 | 4714 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1699) |
| `ivopts` | 461 of 3768 | 2174 | 144 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1325) |
| `discharge` | 0 of 3768 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3768 | 0 | 0 |  |  |
| `coalesce` | 0 of 3768 | 0 | 0 |  |  |
| `gcm` | 278 of 3768 | 7065 | 608 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6024) |
| `loop-delete` | 106 of 3768 | 517 | 3311 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 63 of 3768 | 301 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (175) |

## `rucc` at -O3

51 passes ran on 3752 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3752 | 0 |  |
| `rejoin` | 3752 | 0 |  |
| `constant-p` | 3752 | 0 |  |
| `hoist` | 3752 | 0 |  |
| `plane-sink` | 3752 | 0 |  |
| `split` | 3752 | 0 |  |
| `discharge` | 3752 | 0 |  |
| `dead-plane` | 3752 | 0 |  |
| `coalesce` | 3752 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 909 of 3752 | 3852 | 3258 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2397) |
| `libcall` | 24 of 3747 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3752 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3752 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 242 of 3752 | 356 | 754 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (356) |
| `expect` | 32 of 3752 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2493 of 3752 | 94836 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (94836) |
| `image` | 92 of 3752 | 266 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (148) |
| `simplify-cfg` | 2775 of 3752 | 162433 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (73432) |
| `sroa` | 627 of 3752 | 1878 | 202 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1878) |
| `simplify` | 2609 of 3752 | 29583 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8755) |
| `sccp` | 2339 of 3752 | 80319 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (65999) |
| `narrow` | 305 of 3752 | 1431 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1431) |
| `tail-recursion` | 51 of 3752 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3752 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3528 of 3752 | 202563 | 146045 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (202559) |
| `short-circuit` | 111 of 3752 | 251 | 241 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (251) |
| `thread-implied` | 172 of 3752 | 518 | 4973 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (434) |
| `phiopt` | 705 of 3752 | 3166 | 1828 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2966) |
| `rangeswitch` | 1 of 3752 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3752 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 73 of 3752 | 137 | 4892 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (64) |
| `prune` | 49 of 3752 | 99 | 22890 | prune 14, file-scope-asm 8, kprintf-format 7 | branch the value ranges settle whichever way control reached it replaced by a jump (40) |
| `canon` | 2188 of 3752 | 15222 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8056) |
| `header-copy` | 2161 of 3752 | 9211 | 1247 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4944) |
| `licm` | 658 of 3752 | 6541 | 14141 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6541) |
| `unroll` | 1166 of 3752 | 3420 | 3431 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1710) |
| `thread` | 15 of 3752 | 56 | 2512 | file-scope-asm 8, loop-unroll-shape 4, crt-and-brand 2 | edge pointed straight at the arm of the branch it arrives at that it decides (45) |
| `lanes` | 0 of 3752 | 0 | 0 |  |  |
| `number` | 3226 of 3752 | 214792 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (151509) |
| `load-forward` | 503 of 3752 | 5884 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3205) |
| `redundant-load` | 191 of 3752 | 500 | 31215 | builtin-setjmp 48, frame-size 27, crc32c 16 | load replaced by the value an earlier load of the same address already had (260) |
| `dse` | 245 of 3752 | 1241 | 592 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1237) |
| `bswap` | 99 of 3752 | 364 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3752 | 0 | 0 |  |  |
| `constant-p` | 0 of 3752 | 0 | 0 |  |  |
| `subscript` | 44 of 3752 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3752 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3752 | 0 | 0 |  |  |
| `split` | 0 of 3752 | 0 | 0 |  |  |
| `reassoc` | 776 of 3752 | 1726 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (985) |
| `loop-idiom` | 17 of 3747 | 17 | 1086 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3752 | 3 | 3136 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 774 of 3752 | 4734 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1704) |
| `ivopts` | 456 of 3752 | 2438 | 143 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1495) |
| `discharge` | 0 of 3752 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3752 | 0 | 0 |  |  |
| `coalesce` | 0 of 3752 | 0 | 0 |  |  |
| `gcm` | 277 of 3752 | 7193 | 667 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6156) |
| `loop-delete` | 106 of 3752 | 517 | 3362 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 63 of 3752 | 301 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (175) |

## `rucc` at -Os

37 passes ran on 3752 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3752 | 0 |  |
| `constant-p` | 3752 | 0 |  |
| `discharge` | 3752 | 0 |  |
| `dead-plane` | 3752 | 0 |  |
| `coalesce` | 3752 | 0 |  |
| `plane-sink` | 3752 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 851 of 3752 | 2269 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (821) |
| `libcall` | 24 of 3747 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3752 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3752 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 269 of 3752 | 372 | 815 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (371) |
| `expect` | 32 of 3752 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3154 of 3752 | 89891 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (89891) |
| `image` | 87 of 3752 | 213 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (124) |
| `simplify-cfg` | 2749 of 3752 | 61620 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (29251) |
| `sroa` | 541 of 3752 | 1696 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1696) |
| `simplify` | 2481 of 3752 | 25408 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7836) |
| `sccp` | 781 of 3752 | 10310 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8657) |
| `narrow` | 305 of 3752 | 1429 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1429) |
| `tail-recursion` | 51 of 3752 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3752 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3752 | 2 | 426 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 183 of 3752 | 844 | 7184 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (727) |
| `phiopt` | 679 of 3752 | 2937 | 746 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2762) |
| `rangetest` | 13 of 3752 | 27 | 0 | kprintf-format 7, short-circuit 3, lkmm 2 | branches on comparisons of one value merged into one test (27) |
| `prune` | 98 of 3752 | 112 | 7904 | crc32c 32, target-attribute 28, prune 14 | branch the value ranges settle whichever way control reached it replaced by a jump (80) |
| `canon` | 2187 of 3752 | 10843 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4632) |
| `header-copy-small` | 2134 of 3752 | 8142 | 1224 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4524) |
| `number` | 3517 of 3752 | 210755 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (155037) |
| `load-forward` | 435 of 3752 | 3844 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2735) |
| `dse` | 223 of 3752 | 1108 | 361 | sigsetjmp 112, builtin-setjmp 48, unused-returns 20 | store removed, nothing reads it before it is overwritten or goes out of scope (1104) |
| `bswap` | 107 of 3752 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3752 | 0 | 0 |  |  |
| `constant-p` | 0 of 3752 | 0 | 0 |  |  |
| `reassoc` | 705 of 3752 | 1493 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (911) |
| `discharge` | 0 of 3752 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3752 | 0 | 0 |  |  |
| `coalesce` | 0 of 3752 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3752 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3747 | 53 | 1983 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3380 of 3752 | 62000 | 44625 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (62000) |
| `loop-delete` | 167 of 3752 | 606 | 4679 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (342) |
| `adce` | 27 of 3752 | 184 | 0 | loop-deletion 16, kprintf-format 7, function-purity 4 | instruction nothing necessary reads removed (88) |

