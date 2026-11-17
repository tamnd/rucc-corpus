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
| `simplify-cfg` | 2323 of 3912 | 14573 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7585) |

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
| `fold` | 3276 of 3866 | 90759 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (90675) |
| `image` | 91 of 3866 | 224 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (129) |
| `simplify-cfg` | 2855 of 3866 | 62091 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (28761) |
| `sroa` | 543 of 3866 | 1700 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1700) |
| `simplify` | 2601 of 3866 | 26106 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7969) |
| `sccp` | 857 of 3866 | 10547 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8777) |
| `narrow` | 337 of 3866 | 1485 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1485) |
| `short-circuit-free` | 1 of 3866 | 2 | 482 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 199 of 3866 | 928 | 4980 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (795) |
| `phiopt` | 753 of 3866 | 3068 | 827 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (2893) |
| `rangetest` | 14 of 3866 | 28 | 0 | kprintf-format 7, short-circuit 4, lkmm 2 | branches on comparisons of one value merged into one test (28) |
| `thread-copy` | 100 of 3866 | 288 | 4694 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (184) |
| `prune` | 32 of 3866 | 48 | 8232 | prune 14, kprintf-format 7, simd-lfind 6 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (17) |
| `canon` | 2178 of 3866 | 11107 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4725) |
| `header-copy` | 2145 of 3866 | 8210 | 1328 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4598) |
| `licm` | 762 of 3866 | 6235 | 6332 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6235) |
| `number` | 3629 of 3866 | 214925 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (158222) |
| `load-forward` | 415 of 3866 | 3841 | 219 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2768) |
| `rejoin` | 0 of 3866 | 0 | 0 |  |  |
| `constant-p` | 0 of 3866 | 0 | 0 |  |  |
| `hoist` | 0 of 3866 | 0 | 0 |  |  |
| `discharge` | 0 of 3866 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3866 | 0 | 0 |  |  |
| `coalesce` | 0 of 3866 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3866 | 0 | 0 |  |  |
| `dce` | 3494 of 3866 | 60359 | 47243 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (60358) |
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
| `fold` | 2614 of 3882 | 93849 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (93765) |
| `image` | 95 of 3882 | 269 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (153) |
| `simplify-cfg` | 2895 of 3882 | 159056 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (71848) |
| `sroa` | 632 of 3882 | 1871 | 204 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1871) |
| `simplify` | 2717 of 3882 | 29834 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8827) |
| `sccp` | 2433 of 3882 | 79145 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (64839) |
| `narrow` | 338 of 3882 | 1488 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1488) |
| `tail-recursion` | 51 of 3882 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3882 | 36 | 128 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3658 of 3882 | 203039 | 201761 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (203035) |
| `short-circuit` | 117 of 3882 | 257 | 291 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (257) |
| `thread-implied` | 188 of 3882 | 640 | 5066 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (540) |
| `phiopt` | 777 of 3882 | 3208 | 1849 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (3008) |
| `rangeswitch` | 1 of 3882 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3882 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 79 of 3882 | 143 | 4833 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (70) |
| `prune` | 66 of 3882 | 117 | 23674 | prune 14, ifunc-static 10, file-scope-asm 8 | branch the value ranges settle whichever way control reached it replaced by a jump (57) |
| `canon` | 2238 of 3882 | 15619 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8103) |
| `header-copy` | 2205 of 3882 | 9157 | 1373 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4949) |
| `licm` | 686 of 3882 | 6415 | 14095 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6415) |
| `unroll-free` | 1174 of 3882 | 3302 | 3573 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1651) |
| `thread` | 31 of 3882 | 104 | 2514 | ifunc-static 10, file-scope-asm 8, symbol-versions 6 | edge pointed straight at the arm of the branch it arrives at that it decides (77) |
| `lanes` | 0 of 3882 | 0 | 0 |  |  |
| `number` | 3344 of 3882 | 210619 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (146794) |
| `load-forward` | 534 of 3882 | 5727 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3035) |
| `redundant-load` | 200 of 3882 | 535 | 30101 | builtin-setjmp 48, frame-size 27, target-attribute 14 | load replaced by the value an earlier load of the same address already had (301) |
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
| `widen` | 801 of 3882 | 4922 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1777) |
| `ivopts` | 477 of 3882 | 2318 | 176 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1421) |
| `discharge` | 0 of 3882 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3882 | 0 | 0 |  |  |
| `coalesce` | 0 of 3882 | 0 | 0 |  |  |
| `gcm` | 301 of 3882 | 7172 | 618 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6112) |
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
| `fold` | 2608 of 3866 | 95714 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (95630) |
| `image` | 96 of 3866 | 277 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (153) |
| `simplify-cfg` | 2879 of 3866 | 165691 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (74335) |
| `sroa` | 633 of 3866 | 1888 | 204 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1888) |
| `simplify` | 2712 of 3866 | 30211 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8903) |
| `sccp` | 2416 of 3866 | 80691 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (66213) |
| `narrow` | 337 of 3866 | 1487 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1487) |
| `tail-recursion` | 51 of 3866 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3866 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `dce` | 3642 of 3866 | 205684 | 202238 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (205680) |
| `short-circuit` | 117 of 3866 | 257 | 291 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (257) |
| `thread-implied` | 188 of 3866 | 640 | 5169 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (540) |
| `phiopt` | 778 of 3866 | 3289 | 1875 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (3089) |
| `rangeswitch` | 1 of 3866 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 83 of 3866 | 116 | 0 | crc32c 32, target-attribute 28, kprintf-format 7 | comparisons of one value merged into interval tests (116) |
| `thread-copy` | 79 of 3866 | 143 | 4936 | crc32c 32, target-attribute 28, file-scope-asm 8 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (70) |
| `prune` | 65 of 3866 | 115 | 24073 | prune 14, ifunc-static 10, file-scope-asm 8 | branch the value ranges settle whichever way control reached it replaced by a jump (56) |
| `canon` | 2222 of 3866 | 15911 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (8260) |
| `header-copy` | 2189 of 3866 | 9367 | 1377 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (5056) |
| `licm` | 684 of 3866 | 6699 | 14157 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (6699) |
| `unroll` | 1178 of 3866 | 3456 | 3587 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1728) |
| `thread` | 31 of 3866 | 104 | 2522 | ifunc-static 10, file-scope-asm 8, symbol-versions 6 | edge pointed straight at the arm of the branch it arrives at that it decides (77) |
| `lanes` | 0 of 3866 | 0 | 0 |  |  |
| `number` | 3338 of 3866 | 218164 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (153490) |
| `load-forward` | 522 of 3866 | 5926 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3237) |
| `redundant-load` | 220 of 3866 | 573 | 32341 | builtin-setjmp 48, frame-size 27, crc32c 16 | load replaced by the value an earlier load of the same address already had (319) |
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
| `widen` | 795 of 3866 | 4942 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1782) |
| `ivopts` | 472 of 3866 | 2582 | 175 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1591) |
| `discharge` | 0 of 3866 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3866 | 0 | 0 |  |  |
| `coalesce` | 0 of 3866 | 0 | 0 |  |  |
| `gcm` | 300 of 3866 | 7300 | 677 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6244) |
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
| `inline` | 915 of 3866 | 2720 | 4716 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1136) |
| `libcall` | 24 of 3861 | 65 | 0 | libcall-fold 17, kprintf-format 7 | call to the library folded (39) |
| `ipa-cp` | 88 of 3866 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 132 of 3866 | 175 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (167) |
| `ipa-vrp` | 274 of 3866 | 393 | 773 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (392) |
| `expect` | 32 of 3866 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2583 of 3866 | 88810 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (88725) |
| `image` | 91 of 3866 | 224 | 0 | kencc-varargck 43, brace-elision 16, function-purity 12 | load from a read only object folded to what it was initialized to (129) |
| `simplify-cfg` | 2866 of 3866 | 66441 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (30168) |
| `sroa` | 543 of 3866 | 1700 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1700) |
| `simplify` | 2584 of 3866 | 26966 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8606) |
| `sccp` | 926 of 3866 | 11233 | 0 | overflow-builtins 108, sigsetjmp 96, crc32c 64 | access alignment raised to what its address is proved to be (8746) |
| `narrow` | 337 of 3866 | 1485 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1485) |
| `tail-recursion` | 51 of 3866 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3866 | 36 | 124 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3866 | 2 | 546 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 199 of 3866 | 966 | 7410 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (833) |
| `phiopt` | 753 of 3866 | 3068 | 874 | compare-fold 118, crc32c 64, kencc-conversions 64 | branch whose two arms only work out a value replaced by the value and no branch (2893) |
| `rangetest` | 13 of 3866 | 27 | 0 | kprintf-format 7, short-circuit 3, lkmm 2 | branches on comparisons of one value merged into one test (27) |
| `prune` | 98 of 3866 | 112 | 8296 | crc32c 32, target-attribute 28, prune 14 | branch the value ranges settle whichever way control reached it replaced by a jump (80) |
| `canon` | 2221 of 3866 | 11622 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4762) |
| `header-copy-small` | 2162 of 3866 | 8805 | 1328 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4759) |
| `number` | 3629 of 3866 | 217312 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (160176) |
| `load-forward` | 454 of 3866 | 3821 | 219 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2768) |
| `dse` | 223 of 3866 | 1106 | 367 | sigsetjmp 112, builtin-setjmp 48, unused-returns 20 | store removed, nothing reads it before it is overwritten or goes out of scope (1102) |
| `dce` | 3501 of 3866 | 63932 | 92196 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (63932) |
| `bswap` | 99 of 3866 | 108 | 0 | crc32c 64, target-attribute 24, simd-lfind 6 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3866 | 0 | 0 |  |  |
| `constant-p` | 0 of 3866 | 0 | 0 |  |  |
| `arm-hoist` | 40 of 3866 | 77 | 0 | value-settled 11, kprintf-format 7, file-scope-asm 4 | instruction both arms compute moved above the branch (77) |
| `reassoc` | 715 of 3866 | 1493 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (884) |
| `discharge` | 0 of 3866 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3866 | 0 | 0 |  |  |
| `coalesce` | 0 of 3866 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3866 | 0 | 0 |  |  |
| `loop-idiom` | 59 of 3861 | 59 | 2025 | loop-idiom 40, alias-analysis 8, kencc-layout 6 | loop that stores one byte over a range replaced by memset (35) |
| `loop-delete` | 173 of 3866 | 624 | 4821 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (354) |
| `adce` | 27 of 3866 | 156 | 0 | loop-deletion 16, kprintf-format 7, function-purity 4 | instruction nothing necessary reads removed (74) |
| `step-sink` | 248 of 3866 | 1414 | 0 | crc32c 64, register-pressure 36, simd-lfind 30 | step moved after the last read of the value it steps (1414) |

