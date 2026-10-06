# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3482 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3482 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3482 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2164 of 3482 | 10259 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (6130) |

## `rucc` at -O1

29 passes ran on 3482 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3482 | 0 |  |
| `constant-p` | 3482 | 0 |  |
| `hoist` | 3482 | 0 |  |
| `discharge` | 3482 | 0 |  |
| `dead-plane` | 3482 | 0 |  |
| `coalesce` | 3482 | 0 |  |
| `plane-sink` | 3482 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 657 of 3482 | 1728 | 86 | sigsetjmp 98, crc32c 64, target-attribute 52 | call to a static function called once inlined (695) |
| `libcall` | 0 of 3482 | 0 | 0 |  |  |
| `expect` | 32 of 3482 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2889 of 3482 | 83975 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (83975) |
| `image` | 27 of 3482 | 41 | 0 | function-purity 12, constant-args 4, unused-params 4 | call through the address of a function made a direct call (15) |
| `sroa` | 463 of 3482 | 1616 | 69 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1616) |
| `simplify` | 2354 of 3482 | 23468 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (6527) |
| `sccp` | 372 of 3482 | 3343 | 0 | frame-size 45, loop-idiom 40, interpreter-dispatch 36 | access alignment raised to what its address is proved to be (2175) |
| `narrow` | 244 of 3482 | 1281 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1281) |
| `short-circuit-free` | 1 of 3482 | 2 | 222 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 125 of 3482 | 254 | 3502 | crc32c 64, target-attribute 32, short-circuit 13 | edge pointed straight at the arm of the branch it arrives at that it decides (250) |
| `phiopt` | 615 of 3482 | 2751 | 621 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 6 of 3482 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `thread-copy` | 73 of 3482 | 199 | 3363 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (135) |
| `prune` | 69 of 3482 | 90 | 6431 | crc32c 32, target-attribute 28, prune 3 | branch the value ranges settle whichever way control reached it replaced by a jump (68) |
| `canon` | 2025 of 3482 | 9297 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4254) |
| `header-copy` | 1979 of 3482 | 7580 | 957 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4191) |
| `licm` | 700 of 3482 | 4947 | 5794 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4947) |
| `simplify-cfg` | 2514 of 3482 | 48567 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (24381) |
| `number` | 3208 of 3482 | 175232 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (148380) |
| `load-forward` | 373 of 3482 | 4643 | 220 | builtin-setjmp 48, interpreter-dispatch 36, induction-variable 32 | load replaced by the value a store in the same block wrote there (2496) |
| `constant-p` | 0 of 3482 | 0 | 0 |  |  |
| `hoist` | 0 of 3482 | 0 | 0 |  |  |
| `discharge` | 0 of 3482 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3482 | 0 | 0 |  |  |
| `coalesce` | 0 of 3482 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3482 | 0 | 0 |  |  |
| `dce` | 3089 of 3482 | 52282 | 44687 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (52282) |
| `loop-delete` | 138 of 3482 | 530 | 4216 | loop-deletion 64, function-purity 20, register-pressure 12 | block nothing reaches removed (276) |

## `rucc` at -O2

45 passes ran on 3497 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3497 | 0 |  |
| `rangeswitch` | 3497 | 0 |  |
| `lanes` | 3497 | 0 |  |
| `hoist` | 3497 | 0 |  |
| `plane-sink` | 3497 | 0 |  |
| `split` | 3497 | 0 |  |
| `discharge` | 3497 | 0 |  |
| `dead-plane` | 3497 | 0 |  |
| `coalesce` | 3497 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 823 of 3497 | 3980 | 2914 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2710) |
| `libcall` | 0 of 3497 | 0 | 0 |  |  |
| `ipa-cp` | 53 of 3497 | 127 | 0 | frame-size 35, builtin-setjmp 8, general-regs-only 5 | instruction with constant operands folded to a constant (69) |
| `ipa-sra` | 111 of 3497 | 144 | 0 | frame-size 53, builtin-setjmp 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (136) |
| `ipa-vrp` | 239 of 3497 | 312 | 449 | tail-call 36, crc32c 32, frame-size 26 | range every call agrees on written on a parameter (312) |
| `expect` | 32 of 3497 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2938 of 3497 | 163886 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (163886) |
| `image` | 67 of 3497 | 311 | 0 | interpreter-dispatch 36, function-purity 12, constant-args 4 | call through the address of a function made a direct call (231) |
| `sroa` | 547 of 3497 | 1791 | 177 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1791) |
| `simplify` | 2501 of 3497 | 27880 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7801) |
| `sccp` | 476 of 3497 | 4575 | 0 | crc32c 64, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2567) |
| `narrow` | 244 of 3497 | 1281 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1281) |
| `switch-conv` | 37 of 3497 | 42 | 121 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 91 of 3497 | 111 | 179 | crc32c 32, target-attribute 28, short-circuit 12 | both halves of an and-and or an or-or worked out at once, and the branch went (111) |
| `thread` | 102 of 3497 | 170 | 4072 | crc32c 64, target-attribute 21, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (166) |
| `phiopt` | 634 of 3497 | 2823 | 1970 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2626) |
| `rangeswitch` | 0 of 3497 | 0 | 0 |  |  |
| `rangetest` | 75 of 3497 | 80 | 0 | crc32c 32, target-attribute 28, short-circuit 6 | comparisons of one value merged into interval tests (80) |
| `thread-copy` | 68 of 3497 | 193 | 3927 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (131) |
| `prune` | 25 of 3497 | 623 | 19438 | switch-lowering 15, prune 3, objtool-shapes 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2047 of 3497 | 13160 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6596) |
| `header-copy` | 1994 of 3497 | 9518 | 923 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4884) |
| `licm` | 736 of 3497 | 5285 | 13332 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5285) |
| `unroll` | 1137 of 3497 | 3990 | 2980 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1995) |
| `simplify-cfg` | 2563 of 3497 | 176767 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (73657) |
| `lanes` | 0 of 3497 | 0 | 0 |  |  |
| `number` | 3280 of 3497 | 341401 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (298214) |
| `load-forward` | 479 of 3497 | 6270 | 252 | crc32c 64, builtin-setjmp 48, loop-restructure 44 | load replaced by the value a store in the same block wrote there (3857) |
| `redundant-load` | 181 of 3497 | 394 | 29881 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 271 of 3497 | 1861 | 2232 | sigsetjmp 104, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1840) |
| `bswap` | 110 of 3497 | 276 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `constant-p` | 5 of 3497 | 35 | 1 | constant-p-after-inline 5 | instruction with constant operands folded to a constant (11) |
| `hoist` | 0 of 3497 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3497 | 0 | 0 |  |  |
| `split` | 0 of 3497 | 0 | 0 |  |  |
| `reassoc` | 742 of 3497 | 1685 | 0 | register-pressure 60, sigsetjmp 56, frame-size 48 | integer tree rebuilt in rank order (905) |
| `loop-idiom` | 17 of 3497 | 17 | 939 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `ivopts` | 502 of 3497 | 3224 | 44 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1737) |
| `discharge` | 0 of 3497 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3497 | 0 | 0 |  |  |
| `coalesce` | 0 of 3497 | 0 | 0 |  |  |
| `dce` | 3165 of 3497 | 79981 | 49878 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (79981) |
| `gcm` | 192 of 3497 | 6242 | 452 | frame-size 45, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6242) |
| `loop-delete` | 105 of 3497 | 512 | 2894 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 24 of 3497 | 124 | 0 | loop-deletion 16, branch-probability 4, function-purity 4 | block nothing reaches removed (44) |

## `rucc` at -O3

45 passes ran on 3482 cases, and 10 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3482 | 0 |  |
| `rangeswitch` | 3482 | 0 |  |
| `lanes` | 3482 | 0 |  |
| `constant-p` | 3482 | 0 |  |
| `hoist` | 3482 | 0 |  |
| `plane-sink` | 3482 | 0 |  |
| `split` | 3482 | 0 |  |
| `discharge` | 3482 | 0 |  |
| `dead-plane` | 3482 | 0 |  |
| `coalesce` | 3482 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 815 of 3482 | 4013 | 2833 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2710) |
| `libcall` | 0 of 3482 | 0 | 0 |  |  |
| `ipa-cp` | 47 of 3482 | 121 | 0 | frame-size 30, builtin-setjmp 8, general-regs-only 5 | instruction with constant operands folded to a constant (69) |
| `ipa-sra` | 105 of 3482 | 138 | 0 | builtin-setjmp 48, frame-size 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (130) |
| `ipa-vrp` | 224 of 3482 | 293 | 424 | tail-call 36, crc32c 32, target-attribute 26 | range every call agrees on written on a parameter (293) |
| `expect` | 32 of 3482 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2923 of 3482 | 166398 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (166398) |
| `image` | 67 of 3482 | 311 | 0 | interpreter-dispatch 36, function-purity 12, constant-args 4 | call through the address of a function made a direct call (231) |
| `sroa` | 547 of 3482 | 1795 | 177 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1795) |
| `simplify` | 2491 of 3482 | 28159 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7861) |
| `sccp` | 471 of 3482 | 4599 | 0 | crc32c 64, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2599) |
| `narrow` | 244 of 3482 | 1281 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1281) |
| `switch-conv` | 37 of 3482 | 42 | 118 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 91 of 3482 | 111 | 179 | crc32c 32, target-attribute 28, short-circuit 12 | both halves of an and-and or an or-or worked out at once, and the branch went (111) |
| `thread` | 102 of 3482 | 170 | 4160 | crc32c 64, target-attribute 21, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (166) |
| `phiopt` | 634 of 3482 | 2903 | 1962 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2706) |
| `rangeswitch` | 0 of 3482 | 0 | 0 |  |  |
| `rangetest` | 75 of 3482 | 80 | 0 | crc32c 32, target-attribute 28, short-circuit 6 | comparisons of one value merged into interval tests (80) |
| `thread-copy` | 68 of 3482 | 193 | 4015 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (131) |
| `prune` | 23 of 3482 | 619 | 19553 | switch-lowering 15, prune 3, switch-dispatch 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2032 of 3482 | 13370 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6718) |
| `header-copy` | 1979 of 3482 | 9694 | 923 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4972) |
| `licm` | 734 of 3482 | 5566 | 13396 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5566) |
| `unroll` | 1135 of 3482 | 4104 | 2995 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (2052) |
| `simplify-cfg` | 2548 of 3482 | 180184 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (74869) |
| `lanes` | 0 of 3482 | 0 | 0 |  |  |
| `number` | 3269 of 3482 | 347367 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (304070) |
| `load-forward` | 473 of 3482 | 6457 | 252 | crc32c 64, builtin-setjmp 48, loop-restructure 44 | load replaced by the value a store in the same block wrote there (3836) |
| `redundant-load` | 181 of 3482 | 394 | 30287 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 271 of 3482 | 1861 | 2280 | sigsetjmp 104, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1840) |
| `bswap` | 110 of 3482 | 388 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `constant-p` | 0 of 3482 | 0 | 0 |  |  |
| `hoist` | 0 of 3482 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3482 | 0 | 0 |  |  |
| `split` | 0 of 3482 | 0 | 0 |  |  |
| `reassoc` | 740 of 3482 | 1695 | 0 | register-pressure 60, sigsetjmp 56, frame-size 48 | integer tree rebuilt in rank order (916) |
| `loop-idiom` | 17 of 3482 | 17 | 914 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `ivopts` | 497 of 3482 | 3436 | 44 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1871) |
| `discharge` | 0 of 3482 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3482 | 0 | 0 |  |  |
| `coalesce` | 0 of 3482 | 0 | 0 |  |  |
| `dce` | 3150 of 3482 | 80759 | 50051 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (80759) |
| `gcm` | 192 of 3482 | 6242 | 504 | frame-size 45, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (6242) |
| `loop-delete` | 105 of 3482 | 512 | 2922 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 24 of 3482 | 124 | 0 | loop-deletion 16, branch-probability 4, function-purity 4 | block nothing reaches removed (44) |

## `rucc` at -Os

35 passes ran on 3482 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3482 | 0 |  |
| `constant-p` | 3482 | 0 |  |
| `discharge` | 3482 | 0 |  |
| `dead-plane` | 3482 | 0 |  |
| `coalesce` | 3482 | 0 |  |
| `plane-sink` | 3482 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 657 of 3482 | 1728 | 86 | sigsetjmp 98, crc32c 64, target-attribute 52 | call to a static function called once inlined (695) |
| `libcall` | 0 of 3482 | 0 | 0 |  |  |
| `ipa-cp` | 69 of 3482 | 163 | 0 | frame-size 30, sigsetjmp 14, builtin-setjmp 12 | instruction with constant operands folded to a constant (89) |
| `ipa-sra` | 135 of 3482 | 172 | 0 | builtin-setjmp 48, frame-size 48, sigsetjmp 14 | parameter nothing reads removed, and the argument at every call with it (152) |
| `ipa-vrp` | 296 of 3482 | 392 | 639 | crc32c 64, sigsetjmp 42, target-attribute 26 | range every call agrees on written on a parameter (392) |
| `expect` | 32 of 3482 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2885 of 3482 | 83739 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (83739) |
| `image` | 27 of 3482 | 41 | 0 | function-purity 12, constant-args 4, unused-params 4 | call through the address of a function made a direct call (15) |
| `sroa` | 463 of 3482 | 1616 | 69 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1616) |
| `simplify` | 2354 of 3482 | 23391 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (6527) |
| `sccp` | 372 of 3482 | 3343 | 0 | frame-size 45, loop-idiom 40, interpreter-dispatch 36 | access alignment raised to what its address is proved to be (2175) |
| `narrow` | 244 of 3482 | 1281 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1281) |
| `switch-conv` | 37 of 3482 | 37 | 108 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3482 | 2 | 222 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 125 of 3482 | 255 | 3669 | crc32c 64, target-attribute 32, short-circuit 13 | edge pointed straight at the arm of the branch it arrives at that it decides (251) |
| `phiopt` | 615 of 3482 | 2751 | 621 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 6 of 3482 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 69 of 3482 | 88 | 6473 | crc32c 32, target-attribute 28, prune 3 | branch the value ranges settle whichever way control reached it replaced by a jump (67) |
| `canon` | 2024 of 3482 | 9090 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4235) |
| `header-copy-small` | 1974 of 3482 | 7636 | 882 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4172) |
| `simplify-cfg` | 2509 of 3482 | 47417 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (24103) |
| `number` | 3204 of 3482 | 174267 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (147325) |
| `load-forward` | 375 of 3482 | 4644 | 220 | builtin-setjmp 48, interpreter-dispatch 36, loop-restructure 36 | load replaced by the value a store in the same block wrote there (2497) |
| `dse` | 225 of 3482 | 1672 | 1085 | sigsetjmp 100, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1651) |
| `bswap` | 110 of 3482 | 132 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `constant-p` | 0 of 3482 | 0 | 0 |  |  |
| `reassoc` | 662 of 3482 | 1491 | 0 | register-pressure 60, sigsetjmp 56, register-alloc 40 | integer tree rebuilt in rank order (900) |
| `discharge` | 0 of 3482 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3482 | 0 | 0 |  |  |
| `coalesce` | 0 of 3482 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3482 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3482 | 53 | 1785 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3103 of 3482 | 55278 | 43050 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (55278) |
| `loop-delete` | 165 of 3482 | 585 | 4189 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (330) |
| `adce` | 24 of 3482 | 104 | 0 | loop-deletion 16, branch-probability 8 | instruction nothing necessary reads removed (48) |

