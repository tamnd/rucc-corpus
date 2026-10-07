# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3484 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3484 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3484 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2166 of 3484 | 10537 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (6134) |

## `rucc` at -O1

29 passes ran on 3484 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3484 | 0 |  |
| `constant-p` | 3484 | 0 |  |
| `hoist` | 3484 | 0 |  |
| `discharge` | 3484 | 0 |  |
| `dead-plane` | 3484 | 0 |  |
| `coalesce` | 3484 | 0 |  |
| `plane-sink` | 3484 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 657 of 3484 | 1728 | 86 | sigsetjmp 98, crc32c 64, target-attribute 52 | call to a static function called once inlined (695) |
| `libcall` | 0 of 3484 | 0 | 0 |  |  |
| `expect` | 32 of 3484 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2910 of 3484 | 85812 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (85812) |
| `image` | 27 of 3484 | 41 | 0 | function-purity 12, constant-args 4, unused-params 4 | call through the address of a function made a direct call (15) |
| `sroa` | 465 of 3484 | 1618 | 67 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1618) |
| `simplify` | 2356 of 3484 | 23613 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (6531) |
| `sccp` | 401 of 3484 | 3729 | 0 | frame-size 45, loop-idiom 40, target-attribute 40 | access alignment raised to what its address is proved to be (2171) |
| `narrow` | 242 of 3484 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `short-circuit-free` | 1 of 3484 | 2 | 242 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 126 of 3484 | 255 | 3524 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (251) |
| `phiopt` | 615 of 3484 | 2751 | 640 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 7 of 3484 | 7 | 0 | short-circuit 4, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (7) |
| `thread-copy` | 75 of 3484 | 289 | 3367 | crc32c 32, target-attribute 28, short-circuit 6 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (216) |
| `prune` | 69 of 3484 | 90 | 6456 | crc32c 32, target-attribute 28, prune 3 | branch the value ranges settle whichever way control reached it replaced by a jump (68) |
| `canon` | 2027 of 3484 | 9303 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4258) |
| `header-copy` | 1981 of 3484 | 7588 | 957 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4195) |
| `licm` | 693 of 3484 | 4855 | 5794 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4855) |
| `simplify-cfg` | 2516 of 3484 | 48699 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (24414) |
| `number` | 3219 of 3484 | 175720 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (148810) |
| `load-forward` | 371 of 3484 | 4641 | 218 | builtin-setjmp 48, interpreter-dispatch 36, induction-variable 32 | load replaced by the value a store in the same block wrote there (2494) |
| `constant-p` | 0 of 3484 | 0 | 0 |  |  |
| `hoist` | 0 of 3484 | 0 | 0 |  |  |
| `discharge` | 0 of 3484 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3484 | 0 | 0 |  |  |
| `coalesce` | 0 of 3484 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3484 | 0 | 0 |  |  |
| `dce` | 3105 of 3484 | 54321 | 44688 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (54321) |
| `loop-delete` | 138 of 3484 | 530 | 4220 | loop-deletion 64, function-purity 20, register-pressure 12 | block nothing reaches removed (276) |

## `rucc` at -O2

48 passes ran on 3499 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3499 | 0 |  |
| `lanes` | 3499 | 0 |  |
| `hoist` | 3499 | 0 |  |
| `plane-sink` | 3499 | 0 |  |
| `split` | 3499 | 0 |  |
| `discharge` | 3499 | 0 |  |
| `dead-plane` | 3499 | 0 |  |
| `coalesce` | 3499 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 800 of 3499 | 2662 | 4073 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1353) |
| `libcall` | 0 of 3499 | 0 | 0 |  |  |
| `ipa-cp` | 53 of 3499 | 127 | 0 | frame-size 35, builtin-setjmp 8, general-regs-only 5 | instruction with constant operands folded to a constant (69) |
| `ipa-sra` | 111 of 3499 | 144 | 0 | frame-size 53, builtin-setjmp 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (136) |
| `ipa-vrp` | 274 of 3499 | 372 | 527 | crc32c 64, tail-call 36, frame-size 26 | range every call agrees on written on a parameter (372) |
| `expect` | 32 of 3499 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2959 of 3499 | 151034 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (151034) |
| `image` | 31 of 3499 | 95 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 549 of 3499 | 1793 | 173 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1793) |
| `simplify` | 2503 of 3499 | 27285 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7378) |
| `sccp` | 499 of 3499 | 4583 | 0 | crc32c 64, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2203) |
| `narrow` | 242 of 3499 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `switch-conv` | 37 of 3499 | 42 | 121 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3499 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 113 of 3499 | 181 | 3640 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (177) |
| `phiopt` | 634 of 3499 | 2823 | 1658 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2626) |
| `rangeswitch` | 1 of 3499 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3499 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 68 of 3499 | 193 | 3495 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (131) |
| `prune` | 25 of 3499 | 623 | 18396 | switch-lowering 15, prune 3, objtool-shapes 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2049 of 3499 | 12482 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6335) |
| `header-copy` | 1996 of 3499 | 8684 | 917 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4467) |
| `licm` | 729 of 3499 | 5193 | 13320 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5193) |
| `unroll` | 1135 of 3499 | 3172 | 2966 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1586) |
| `simplify-cfg` | 2565 of 3499 | 150492 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (62660) |
| `lanes` | 0 of 3499 | 0 | 0 |  |  |
| `number` | 3291 of 3499 | 300263 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (263849) |
| `load-forward` | 477 of 3499 | 6268 | 250 | crc32c 64, builtin-setjmp 48, loop-restructure 44 | load replaced by the value a store in the same block wrote there (3855) |
| `redundant-load` | 149 of 3499 | 332 | 24751 | builtin-setjmp 48, frame-size 27, target-attribute 26 | load replaced by the value of the store the walk found (232) |
| `dse` | 271 of 3499 | 1861 | 2159 | sigsetjmp 104, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1840) |
| `bswap` | 107 of 3499 | 273 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `constant-p` | 5 of 3499 | 35 | 1 | constant-p-after-inline 5 | instruction with constant operands folded to a constant (11) |
| `subscript` | 38 of 3499 | 65 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (65) |
| `hoist` | 0 of 3499 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3499 | 0 | 0 |  |  |
| `split` | 0 of 3499 | 0 | 0 |  |  |
| `reassoc` | 742 of 3499 | 1685 | 0 | register-pressure 60, sigsetjmp 56, frame-size 48 | integer tree rebuilt in rank order (905) |
| `loop-idiom` | 17 of 3499 | 17 | 942 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3499 | 2 | 2751 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 716 of 3499 | 4521 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1630) |
| `ivopts` | 424 of 3499 | 1986 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1235) |
| `discharge` | 0 of 3499 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3499 | 0 | 0 |  |  |
| `coalesce` | 0 of 3499 | 0 | 0 |  |  |
| `dce` | 3201 of 3499 | 82594 | 47944 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (82594) |
| `gcm` | 191 of 3499 | 5924 | 439 | frame-size 45, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5924) |
| `loop-delete` | 105 of 3499 | 512 | 2880 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 24 of 3499 | 124 | 0 | loop-deletion 16, branch-probability 4, function-purity 4 | block nothing reaches removed (44) |

## `rucc` at -O3

48 passes ran on 3484 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3484 | 0 |  |
| `lanes` | 3484 | 0 |  |
| `constant-p` | 3484 | 0 |  |
| `hoist` | 3484 | 0 |  |
| `plane-sink` | 3484 | 0 |  |
| `split` | 3484 | 0 |  |
| `discharge` | 3484 | 0 |  |
| `dead-plane` | 3484 | 0 |  |
| `coalesce` | 3484 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 792 of 3484 | 3521 | 3138 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2374) |
| `libcall` | 0 of 3484 | 0 | 0 |  |  |
| `ipa-cp` | 47 of 3484 | 121 | 0 | frame-size 30, builtin-setjmp 8, general-regs-only 5 | instruction with constant operands folded to a constant (69) |
| `ipa-sra` | 105 of 3484 | 138 | 0 | builtin-setjmp 48, frame-size 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (130) |
| `ipa-vrp` | 227 of 3484 | 317 | 502 | tail-call 36, crc32c 32, target-attribute 26 | range every call agrees on written on a parameter (317) |
| `expect` | 32 of 3484 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2944 of 3484 | 153182 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (153182) |
| `image` | 31 of 3484 | 95 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 549 of 3484 | 1797 | 173 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1797) |
| `simplify` | 2493 of 3484 | 27480 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7394) |
| `sccp` | 500 of 3484 | 4601 | 0 | crc32c 64, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2211) |
| `narrow` | 242 of 3484 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `switch-conv` | 37 of 3484 | 42 | 118 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3484 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 113 of 3484 | 181 | 3716 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (177) |
| `phiopt` | 634 of 3484 | 2879 | 1668 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2694) |
| `rangeswitch` | 1 of 3484 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3484 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 68 of 3484 | 193 | 3571 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (131) |
| `prune` | 23 of 3484 | 619 | 18309 | switch-lowering 15, prune 3, switch-dispatch 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2034 of 3484 | 12536 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6381) |
| `header-copy` | 1981 of 3484 | 8820 | 893 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4523) |
| `licm` | 727 of 3484 | 5438 | 13336 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5438) |
| `unroll` | 1129 of 3484 | 3270 | 2957 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1635) |
| `simplify-cfg` | 2550 of 3484 | 155649 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (64198) |
| `lanes` | 0 of 3484 | 0 | 0 |  |  |
| `number` | 3280 of 3484 | 308041 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (270781) |
| `load-forward` | 471 of 3484 | 6359 | 250 | crc32c 64, builtin-setjmp 48, loop-restructure 44 | load replaced by the value a store in the same block wrote there (3834) |
| `redundant-load` | 181 of 3484 | 382 | 26919 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 271 of 3484 | 1861 | 2207 | sigsetjmp 104, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1840) |
| `bswap` | 107 of 3484 | 357 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (160) |
| `constant-p` | 0 of 3484 | 0 | 0 |  |  |
| `subscript` | 38 of 3484 | 65 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (65) |
| `hoist` | 0 of 3484 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3484 | 0 | 0 |  |  |
| `split` | 0 of 3484 | 0 | 0 |  |  |
| `reassoc` | 740 of 3484 | 1683 | 0 | register-pressure 60, sigsetjmp 56, frame-size 48 | integer tree rebuilt in rank order (904) |
| `loop-idiom` | 17 of 3484 | 17 | 917 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3484 | 2 | 2703 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 711 of 3484 | 4519 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1621) |
| `ivopts` | 424 of 3484 | 2222 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1383) |
| `discharge` | 0 of 3484 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3484 | 0 | 0 |  |  |
| `coalesce` | 0 of 3484 | 0 | 0 |  |  |
| `dce` | 3186 of 3484 | 83089 | 48117 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (83089) |
| `gcm` | 191 of 3484 | 5824 | 487 | frame-size 45, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5824) |
| `loop-delete` | 105 of 3484 | 512 | 2852 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 24 of 3484 | 124 | 0 | loop-deletion 16, branch-probability 4, function-purity 4 | block nothing reaches removed (44) |

## `rucc` at -Os

35 passes ran on 3484 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3484 | 0 |  |
| `constant-p` | 3484 | 0 |  |
| `discharge` | 3484 | 0 |  |
| `dead-plane` | 3484 | 0 |  |
| `coalesce` | 3484 | 0 |  |
| `plane-sink` | 3484 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 657 of 3484 | 1728 | 86 | sigsetjmp 98, crc32c 64, target-attribute 52 | call to a static function called once inlined (695) |
| `libcall` | 0 of 3484 | 0 | 0 |  |  |
| `ipa-cp` | 69 of 3484 | 163 | 0 | frame-size 30, sigsetjmp 14, builtin-setjmp 12 | instruction with constant operands folded to a constant (89) |
| `ipa-sra` | 135 of 3484 | 172 | 0 | builtin-setjmp 48, frame-size 48, sigsetjmp 14 | parameter nothing reads removed, and the argument at every call with it (152) |
| `ipa-vrp` | 296 of 3484 | 392 | 639 | crc32c 64, sigsetjmp 42, target-attribute 26 | range every call agrees on written on a parameter (392) |
| `expect` | 32 of 3484 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2906 of 3484 | 85576 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (85576) |
| `image` | 27 of 3484 | 41 | 0 | function-purity 12, constant-args 4, unused-params 4 | call through the address of a function made a direct call (15) |
| `sroa` | 465 of 3484 | 1618 | 67 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1618) |
| `simplify` | 2356 of 3484 | 23533 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (6531) |
| `sccp` | 401 of 3484 | 3729 | 0 | frame-size 45, loop-idiom 40, target-attribute 40 | access alignment raised to what its address is proved to be (2171) |
| `narrow` | 242 of 3484 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `switch-conv` | 37 of 3484 | 37 | 108 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3484 | 2 | 242 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 126 of 3484 | 257 | 3708 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (253) |
| `phiopt` | 615 of 3484 | 2751 | 640 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 6 of 3484 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 69 of 3484 | 88 | 6498 | crc32c 32, target-attribute 28, prune 3 | branch the value ranges settle whichever way control reached it replaced by a jump (67) |
| `canon` | 2026 of 3484 | 9096 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4239) |
| `header-copy-small` | 1976 of 3484 | 7644 | 882 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4176) |
| `simplify-cfg` | 2511 of 3484 | 47451 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (24119) |
| `number` | 3215 of 3484 | 174663 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (147663) |
| `load-forward` | 373 of 3484 | 4642 | 218 | builtin-setjmp 48, interpreter-dispatch 36, loop-restructure 36 | load replaced by the value a store in the same block wrote there (2495) |
| `dse` | 225 of 3484 | 1672 | 1083 | sigsetjmp 100, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1651) |
| `bswap` | 107 of 3484 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `constant-p` | 0 of 3484 | 0 | 0 |  |  |
| `reassoc` | 662 of 3484 | 1491 | 0 | register-pressure 60, sigsetjmp 56, register-alloc 40 | integer tree rebuilt in rank order (900) |
| `discharge` | 0 of 3484 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3484 | 0 | 0 |  |  |
| `coalesce` | 0 of 3484 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3484 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3484 | 53 | 1788 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3119 of 3484 | 57336 | 43051 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (57336) |
| `loop-delete` | 165 of 3484 | 585 | 4193 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (330) |
| `adce` | 24 of 3484 | 104 | 0 | loop-deletion 16, branch-probability 8 | instruction nothing necessary reads removed (48) |

