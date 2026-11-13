# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3912 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3912 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3912 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2319 of 3912 | 14221 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7585) |

## `rucc` at -O1

30 passes ran on 3866 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3866 | 0 |  |
| `constant-p` | 3866 | 0 |  |
| `hoist` | 3866 | 0 |  |
| `discharge` | 3866 | 0 |  |
| `dead-plane` | 3866 | 0 |  |
| `coalesce` | 3866 | 0 |  |
| `plane-sink` | 3866 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 874 of 3866 | 2329 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (866) |
| `libcall` | 24 of 3861 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `expect` | 32 of 3866 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3276 of 3866 | 90746 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (90746) |
| `image` | 91 of 3866 | 224 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (129) |
| `simplify-cfg` | 2852 of 3866 | 61718 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (28842) |
| `sroa` | 543 of 3866 | 1700 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1700) |
| `simplify` | 2601 of 3866 | 26106 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7969) |
| `sccp` | 857 of 3866 | 10547 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8777) |
| `narrow` | 337 of 3866 | 1485 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1485) |
| `short-circuit-free` | 1 of 3866 | 2 | 482 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 199 of 3866 | 928 | 4962 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (795) |
| `phiopt` | 753 of 3866 | 3075 | 828 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (2900) |
| `rangetest` | 14 of 3866 | 28 | 0 | kprintf-format 7, short-circuit 4, lkmm 2 | branches on comparisons of one value merged into one test (28) |
| `thread-copy` | 100 of 3866 | 288 | 4694 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (184) |
| `prune` | 32 of 3866 | 48 | 8225 | prune 14, kprintf-format 7, simd-lfind 6 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (17) |
| `canon` | 2178 of 3866 | 11063 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4704) |
| `header-copy` | 2145 of 3866 | 8210 | 1328 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4598) |
| `licm` | 763 of 3866 | 6235 | 6394 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6235) |
| `number` | 3629 of 3866 | 214870 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (158155) |
| `load-forward` | 415 of 3866 | 3841 | 219 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2768) |
| `rejoin` | 0 of 3866 | 0 | 0 |  |  |
| `constant-p` | 0 of 3866 | 0 | 0 |  |  |
| `hoist` | 0 of 3866 | 0 | 0 |  |  |
| `discharge` | 0 of 3866 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3866 | 0 | 0 |  |  |
| `coalesce` | 0 of 3866 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3866 | 0 | 0 |  |  |
| `dce` | 3494 of 3866 | 60334 | 47243 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (60333) |
| `loop-delete` | 144 of 3866 | 567 | 4781 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (296) |

## `rucc` at -O2

51 passes ran on 3882 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3882 | 0 |  |
| `rejoin` | 3882 | 0 |  |
| `hoist` | 3882 | 0 |  |
| `plane-sink` | 3882 | 0 |  |
| `split` | 3882 | 0 |  |
| `discharge` | 3882 | 0 |  |
| `dead-plane` | 3882 | 0 |  |
| `coalesce` | 3882 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 948 of 3882 | 3044 | 4258 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1390) |
| `libcall` | 24 of 3877 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 94 of 3882 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 138 of 3882 | 181 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (173) |
| `ipa-vrp` | 293 of 3882 | 415 | 783 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (414) |
| `expect` | 32 of 3882 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2612 of 3882 | 93774 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (93774) |
| `image` | 95 of 3882 | 269 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (153) |
| `simplify-cfg` | 2893 of 3882 | 158609 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (71938) |
| `sroa` | 632 of 3882 | 1871 | 204 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1871) |
| `simplify` | 2717 of 3882 | 29834 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8827) |
| `sccp` | 2433 of 3882 | 79207 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (64901) |
| `narrow` | 338 of 3882 | 1488 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1488) |
| `tail-recursion` | 51 of 3882 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3882 | 36 | 128 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3658 of 3882 | 202798 | 150289 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (202794) |
| `short-circuit` | 117 of 3882 | 257 | 291 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (257) |
| `thread-implied` | 188 of 3882 | 640 | 5048 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (540) |
| `phiopt` | 778 of 3882 | 3223 | 1850 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (3023) |
| `rangeswitch` | 1 of 3882 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3882 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 79 of 3882 | 143 | 4833 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (70) |
| `prune` | 66 of 3882 | 117 | 23637 | prune 14, ifunc-static 10, file-scope-asm 8 | branch the value ranges settle whichever way control reached it replaced by a jump (57) |
| `canon` | 2238 of 3882 | 15500 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8103) |
| `header-copy` | 2205 of 3882 | 9157 | 1373 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4949) |
| `licm` | 687 of 3882 | 6415 | 14219 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6415) |
| `unroll-free` | 1174 of 3882 | 3302 | 3573 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1651) |
| `thread` | 31 of 3882 | 104 | 2514 | ifunc-static 10, file-scope-asm 8, symbol-versions 6 | edge pointed straight at the arm of the branch it arrives at that it decides (77) |
| `lanes` | 0 of 3882 | 0 | 0 |  |  |
| `number` | 3344 of 3882 | 210781 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (146940) |
| `load-forward` | 534 of 3882 | 5727 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3035) |
| `redundant-load` | 200 of 3882 | 535 | 30099 | builtin-setjmp 48, frame-size 27, target-attribute 14 | load replaced by the value an earlier load of the same address already had (301) |
| `dse` | 247 of 3882 | 1278 | 576 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1274) |
| `bswap` | 99 of 3882 | 252 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3882 | 0 | 0 |  |  |
| `constant-p` | 1 of 3882 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 40 of 3882 | 69 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (69) |
| `hoist` | 0 of 3882 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3882 | 0 | 0 |  |  |
| `split` | 0 of 3882 | 0 | 0 |  |  |
| `reassoc` | 788 of 3882 | 1726 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (979) |
| `loop-idiom` | 17 of 3877 | 17 | 1157 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3882 | 3 | 3229 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 801 of 3882 | 4916 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1777) |
| `ivopts` | 477 of 3882 | 2318 | 176 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1421) |
| `discharge` | 0 of 3882 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3882 | 0 | 0 |  |  |
| `coalesce` | 0 of 3882 | 0 | 0 |  |  |
| `gcm` | 301 of 3882 | 7172 | 624 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6112) |
| `loop-delete` | 106 of 3882 | 517 | 3467 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 65 of 3882 | 309 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (183) |

## `rucc` at -O3

51 passes ran on 3866 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3866 | 0 |  |
| `rejoin` | 3866 | 0 |  |
| `constant-p` | 3866 | 0 |  |
| `hoist` | 3866 | 0 |  |
| `plane-sink` | 3866 | 0 |  |
| `split` | 3866 | 0 |  |
| `discharge` | 3866 | 0 |  |
| `dead-plane` | 3866 | 0 |  |
| `coalesce` | 3866 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 940 of 3866 | 3961 | 3326 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2471) |
| `libcall` | 24 of 3861 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3866 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 132 of 3866 | 175 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (167) |
| `ipa-vrp` | 243 of 3866 | 357 | 757 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (357) |
| `expect` | 32 of 3866 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2606 of 3866 | 95639 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (95639) |
| `image` | 96 of 3866 | 277 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (153) |
| `simplify-cfg` | 2877 of 3866 | 165220 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (74425) |
| `sroa` | 633 of 3866 | 1888 | 204 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1888) |
| `simplify` | 2712 of 3866 | 30211 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8903) |
| `sccp` | 2416 of 3866 | 80753 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (66275) |
| `narrow` | 337 of 3866 | 1487 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1487) |
| `tail-recursion` | 51 of 3866 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3866 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3642 of 3866 | 205443 | 150631 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (205439) |
| `short-circuit` | 117 of 3866 | 257 | 291 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (257) |
| `thread-implied` | 188 of 3866 | 640 | 5151 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (540) |
| `phiopt` | 779 of 3866 | 3304 | 1876 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (3104) |
| `rangeswitch` | 1 of 3866 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3866 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 79 of 3866 | 143 | 4936 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (70) |
| `prune` | 65 of 3866 | 115 | 24036 | prune 14, ifunc-static 10, file-scope-asm 8 | branch the value ranges settle whichever way control reached it replaced by a jump (56) |
| `canon` | 2222 of 3866 | 15792 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8260) |
| `header-copy` | 2189 of 3866 | 9367 | 1377 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (5056) |
| `licm` | 685 of 3866 | 6699 | 14281 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6699) |
| `unroll` | 1178 of 3866 | 3456 | 3587 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1728) |
| `thread` | 31 of 3866 | 104 | 2522 | ifunc-static 10, file-scope-asm 8, symbol-versions 6 | edge pointed straight at the arm of the branch it arrives at that it decides (77) |
| `lanes` | 0 of 3866 | 0 | 0 |  |  |
| `number` | 3338 of 3866 | 218330 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (153636) |
| `load-forward` | 522 of 3866 | 5926 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3237) |
| `redundant-load` | 220 of 3866 | 573 | 32339 | builtin-setjmp 48, frame-size 27, crc32c 16 | load replaced by the value an earlier load of the same address already had (319) |
| `dse` | 247 of 3866 | 1278 | 592 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1274) |
| `bswap` | 99 of 3866 | 364 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3866 | 0 | 0 |  |  |
| `constant-p` | 0 of 3866 | 0 | 0 |  |  |
| `subscript` | 44 of 3866 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3866 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3866 | 0 | 0 |  |  |
| `split` | 0 of 3866 | 0 | 0 |  |  |
| `reassoc` | 787 of 3866 | 1737 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (991) |
| `loop-idiom` | 17 of 3861 | 17 | 1128 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 3 of 3866 | 3 | 3260 | short-circuit 2, conditional-store 1 | loop done four int at a time in the vector registers (3) |
| `widen` | 795 of 3866 | 4936 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1782) |
| `ivopts` | 472 of 3866 | 2582 | 175 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1591) |
| `discharge` | 0 of 3866 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3866 | 0 | 0 |  |  |
| `coalesce` | 0 of 3866 | 0 | 0 |  |  |
| `gcm` | 300 of 3866 | 7300 | 683 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6244) |
| `loop-delete` | 106 of 3866 | 517 | 3518 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
| `adce` | 65 of 3866 | 309 | 0 | frame-size 27, loop-deletion 16, kprintf-format 7 | instruction nothing necessary reads removed (183) |

## `rucc` at -Os

39 passes ran on 3866 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3866 | 0 |  |
| `constant-p` | 3866 | 0 |  |
| `discharge` | 3866 | 0 |  |
| `dead-plane` | 3866 | 0 |  |
| `coalesce` | 3866 | 0 |  |
| `plane-sink` | 3866 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 874 of 3866 | 2329 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (866) |
| `libcall` | 24 of 3861 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3866 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 132 of 3866 | 175 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (167) |
| `ipa-vrp` | 270 of 3866 | 373 | 823 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (372) |
| `expect` | 32 of 3866 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3266 of 3866 | 90606 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (90606) |
| `image` | 91 of 3866 | 224 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (129) |
| `simplify-cfg` | 2851 of 3866 | 63300 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (29959) |
| `sroa` | 543 of 3866 | 1700 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1700) |
| `simplify` | 2584 of 3866 | 26003 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7963) |
| `sccp` | 858 of 3866 | 10548 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8777) |
| `narrow` | 337 of 3866 | 1485 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1485) |
| `tail-recursion` | 51 of 3866 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3866 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3866 | 2 | 482 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 199 of 3866 | 966 | 7392 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (833) |
| `phiopt` | 753 of 3866 | 3075 | 779 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (2900) |
| `rangetest` | 13 of 3866 | 27 | 0 | kprintf-format 7, short-circuit 3, lkmm 2 | branches on comparisons of one value merged into one test (27) |
| `prune` | 98 of 3866 | 112 | 8233 | crc32c 32, target-attribute 28, prune 14 | branch the value ranges settle whichever way control reached it replaced by a jump (80) |
| `canon` | 2221 of 3866 | 11194 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4741) |
| `header-copy-small` | 2162 of 3866 | 8293 | 1328 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4631) |
| `number` | 3629 of 3866 | 214498 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (157376) |
| `load-forward` | 454 of 3866 | 3879 | 219 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2768) |
| `dse` | 223 of 3866 | 1108 | 361 | sigsetjmp 112, builtin-setjmp 48, unused-returns 20 | store removed, nothing reads it before it is overwritten or goes out of scope (1104) |
| `bswap` | 107 of 3866 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3866 | 0 | 0 |  |  |
| `constant-p` | 0 of 3866 | 0 | 0 |  |  |
| `arm-hoist` | 44 of 3866 | 81 | 0 | value-settled 11, kprintf-format 7, branch-probability 4 | instruction both arms compute moved above the branch (81) |
| `reassoc` | 716 of 3866 | 1505 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (918) |
| `discharge` | 0 of 3866 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3866 | 0 | 0 |  |  |
| `coalesce` | 0 of 3866 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3866 | 0 | 0 |  |  |
| `loop-idiom` | 59 of 3861 | 59 | 2025 | loop-idiom 40, alias-analysis 8, kencc-layout 6 | loop that stores one byte over a range replaced by memset (35) |
| `dce` | 3492 of 3866 | 63473 | 46157 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (63473) |
| `loop-delete` | 173 of 3866 | 624 | 4821 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (354) |
| `adce` | 27 of 3866 | 184 | 0 | loop-deletion 16, kprintf-format 7, function-purity 4 | instruction nothing necessary reads removed (88) |
| `step-sink` | 259 of 3866 | 1429 | 0 | crc32c 64, register-alloc 40, register-pressure 36 | step moved after the last read of the value it steps (1429) |

