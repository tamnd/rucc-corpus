# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3547 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3547 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3547 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2229 of 3547 | 12708 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7473) |

## `rucc` at -O1

29 passes ran on 3547 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3547 | 0 |  |
| `constant-p` | 3547 | 0 |  |
| `hoist` | 3547 | 0 |  |
| `discharge` | 3547 | 0 |  |
| `dead-plane` | 3547 | 0 |  |
| `coalesce` | 3547 | 0 |  |
| `plane-sink` | 3547 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 748 of 3547 | 2060 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (704) |
| `libcall` | 0 of 3547 | 0 | 0 |  |  |
| `expect` | 32 of 3547 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2968 of 3547 | 88626 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88626) |
| `image` | 27 of 3547 | 45 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `sroa` | 470 of 3547 | 1623 | 73 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1623) |
| `simplify` | 2402 of 3547 | 25371 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8103) |
| `sccp` | 442 of 3547 | 3797 | 0 | interpreter-dispatch 54, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2206) |
| `narrow` | 286 of 3547 | 1319 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1319) |
| `short-circuit-free` | 1 of 3547 | 2 | 244 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 130 of 3547 | 319 | 3593 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (315) |
| `phiopt` | 615 of 3547 | 2751 | 484 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 7 of 3547 | 7 | 0 | short-circuit 4, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (7) |
| `thread-copy` | 71 of 3547 | 225 | 3500 | crc32c 32, target-attribute 28, short-circuit 6 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (151) |
| `prune` | 84 of 3547 | 795 | 6961 | crc32c 32, target-attribute 28, switch-lowering 15 | block nothing reaches removed (356) |
| `canon` | 2089 of 3547 | 9652 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4413) |
| `header-copy` | 2043 of 3547 | 7858 | 979 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4332) |
| `licm` | 716 of 3547 | 4942 | 6350 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4942) |
| `simplify-cfg` | 2597 of 3547 | 52787 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (25311) |
| `number` | 3277 of 3547 | 182874 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (154243) |
| `load-forward` | 410 of 3547 | 5015 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2763) |
| `constant-p` | 0 of 3547 | 0 | 0 |  |  |
| `hoist` | 0 of 3547 | 0 | 0 |  |  |
| `discharge` | 0 of 3547 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3547 | 0 | 0 |  |  |
| `coalesce` | 0 of 3547 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3547 | 0 | 0 |  |  |
| `dce` | 3178 of 3547 | 57011 | 45134 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (57011) |
| `loop-delete` | 142 of 3547 | 558 | 4367 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (292) |

## `rucc` at -O2

49 passes ran on 3563 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3563 | 0 |  |
| `lanes` | 3563 | 0 |  |
| `hoist` | 3563 | 0 |  |
| `plane-sink` | 3563 | 0 |  |
| `split` | 3563 | 0 |  |
| `discharge` | 3563 | 0 |  |
| `dead-plane` | 3563 | 0 |  |
| `coalesce` | 3563 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 810 of 3563 | 2674 | 4201 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1363) |
| `libcall` | 0 of 3563 | 0 | 0 |  |  |
| `ipa-cp` | 94 of 3563 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 125 of 3563 | 158 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (150) |
| `ipa-vrp` | 287 of 3563 | 409 | 660 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (409) |
| `expect` | 32 of 3563 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3016 of 3563 | 155227 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (155227) |
| `image` | 31 of 3563 | 95 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 555 of 3563 | 1799 | 185 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1799) |
| `simplify` | 2539 of 3563 | 29044 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8938) |
| `sccp` | 528 of 3563 | 4631 | 0 | crc32c 64, interpreter-dispatch 54, frame-size 45 | access alignment raised to what its address is proved to be (2238) |
| `narrow` | 287 of 3563 | 1320 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1320) |
| `tail-recursion` | 51 of 3563 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3563 | 42 | 140 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3563 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 117 of 3563 | 245 | 3709 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (241) |
| `phiopt` | 661 of 3563 | 2852 | 1826 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2654) |
| `rangeswitch` | 1 of 3563 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3563 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 64 of 3563 | 129 | 3629 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (66) |
| `prune` | 25 of 3563 | 623 | 20057 | switch-lowering 15, prune 3, objtool-shapes 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2156 of 3563 | 13417 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6923) |
| `header-copy` | 2103 of 3563 | 9017 | 1004 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4666) |
| `licm` | 744 of 3563 | 5256 | 14292 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5256) |
| `unroll` | 1161 of 3563 | 3260 | 3143 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1630) |
| `simplify-cfg` | 2629 of 3563 | 158055 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (66529) |
| `lanes` | 0 of 3563 | 0 | 0 |  |  |
| `number` | 3356 of 3563 | 309472 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (271453) |
| `load-forward` | 529 of 3563 | 6914 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3889) |
| `redundant-load` | 165 of 3563 | 408 | 30259 | builtin-setjmp 48, frame-size 27, target-attribute 26 | load replaced by the value of the store the walk found (232) |
| `dse` | 273 of 3563 | 1863 | 733 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1859) |
| `bswap` | 107 of 3563 | 273 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `constant-p` | 1 of 3563 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 38 of 3563 | 65 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (65) |
| `hoist` | 0 of 3563 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3563 | 0 | 0 |  |  |
| `split` | 0 of 3563 | 0 | 0 |  |  |
| `reassoc` | 761 of 3563 | 1884 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1104) |
| `loop-idiom` | 17 of 3563 | 17 | 1013 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3563 | 2 | 2928 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 761 of 3563 | 4656 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1675) |
| `ivopts` | 433 of 3563 | 2022 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1253) |
| `discharge` | 0 of 3563 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3563 | 0 | 0 |  |  |
| `coalesce` | 0 of 3563 | 0 | 0 |  |  |
| `dce` | 3269 of 3563 | 86119 | 48816 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (86119) |
| `gcm` | 196 of 3563 | 6149 | 601 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5932) |
| `loop-delete` | 105 of 3563 | 512 | 3057 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 52 of 3563 | 207 | 0 | frame-size 27, loop-deletion 16, branch-probability 4 | instruction nothing necessary reads removed (123) |

## `rucc` at -O3

49 passes ran on 3547 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3547 | 0 |  |
| `lanes` | 3547 | 0 |  |
| `constant-p` | 3547 | 0 |  |
| `hoist` | 3547 | 0 |  |
| `plane-sink` | 3547 | 0 |  |
| `split` | 3547 | 0 |  |
| `discharge` | 3547 | 0 |  |
| `dead-plane` | 3547 | 0 |  |
| `coalesce` | 3547 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 802 of 3547 | 3533 | 3266 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2385) |
| `libcall` | 0 of 3547 | 0 | 0 |  |  |
| `ipa-cp` | 88 of 3547 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 119 of 3547 | 152 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (144) |
| `ipa-vrp` | 239 of 3547 | 353 | 634 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (353) |
| `expect` | 32 of 3547 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3000 of 3547 | 157357 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (157357) |
| `image` | 31 of 3547 | 95 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 555 of 3547 | 1803 | 185 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1803) |
| `simplify` | 2532 of 3547 | 29244 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8954) |
| `sccp` | 529 of 3547 | 4649 | 0 | crc32c 64, interpreter-dispatch 54, frame-size 45 | access alignment raised to what its address is proved to be (2246) |
| `narrow` | 286 of 3547 | 1319 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1319) |
| `tail-recursion` | 51 of 3547 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3547 | 42 | 136 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3547 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 117 of 3547 | 245 | 3784 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (241) |
| `phiopt` | 661 of 3547 | 2908 | 1828 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2722) |
| `rangeswitch` | 1 of 3547 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3547 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 64 of 3547 | 129 | 3704 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (66) |
| `prune` | 23 of 3547 | 619 | 19969 | switch-lowering 15, prune 3, switch-dispatch 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2140 of 3547 | 13468 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6967) |
| `header-copy` | 2087 of 3547 | 9151 | 980 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4721) |
| `licm` | 742 of 3547 | 5501 | 14308 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5501) |
| `unroll` | 1155 of 3547 | 3358 | 3133 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1679) |
| `simplify-cfg` | 2613 of 3547 | 163188 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (68061) |
| `lanes` | 0 of 3547 | 0 | 0 |  |  |
| `number` | 3344 of 3547 | 317249 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (278385) |
| `load-forward` | 523 of 3547 | 7005 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3868) |
| `redundant-load` | 197 of 3547 | 458 | 32425 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 273 of 3547 | 1863 | 733 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1859) |
| `bswap` | 107 of 3547 | 357 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (160) |
| `constant-p` | 0 of 3547 | 0 | 0 |  |  |
| `subscript` | 38 of 3547 | 65 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (65) |
| `hoist` | 0 of 3547 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3547 | 0 | 0 |  |  |
| `split` | 0 of 3547 | 0 | 0 |  |  |
| `reassoc` | 759 of 3547 | 1882 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1103) |
| `loop-idiom` | 17 of 3547 | 17 | 988 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3547 | 2 | 2879 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 756 of 3547 | 4654 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1666) |
| `ivopts` | 433 of 3547 | 2258 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1401) |
| `discharge` | 0 of 3547 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3547 | 0 | 0 |  |  |
| `coalesce` | 0 of 3547 | 0 | 0 |  |  |
| `dce` | 3253 of 3547 | 86599 | 48987 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (86599) |
| `gcm` | 196 of 3547 | 6049 | 649 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5832) |
| `loop-delete` | 105 of 3547 | 512 | 3028 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 52 of 3547 | 207 | 0 | frame-size 27, loop-deletion 16, branch-probability 4 | instruction nothing necessary reads removed (123) |

## `rucc` at -Os

36 passes ran on 3547 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3547 | 0 |  |
| `constant-p` | 3547 | 0 |  |
| `discharge` | 3547 | 0 |  |
| `dead-plane` | 3547 | 0 |  |
| `coalesce` | 3547 | 0 |  |
| `plane-sink` | 3547 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 748 of 3547 | 2060 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (704) |
| `libcall` | 0 of 3547 | 0 | 0 |  |  |
| `ipa-cp` | 88 of 3547 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 119 of 3547 | 152 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (144) |
| `ipa-vrp` | 262 of 3547 | 365 | 695 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (365) |
| `expect` | 32 of 3547 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2966 of 3547 | 88481 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88481) |
| `image` | 27 of 3547 | 45 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `sroa` | 470 of 3547 | 1623 | 73 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1623) |
| `simplify` | 2402 of 3547 | 25295 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8107) |
| `sccp` | 442 of 3547 | 3797 | 0 | interpreter-dispatch 54, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2206) |
| `narrow` | 286 of 3547 | 1319 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1319) |
| `tail-recursion` | 51 of 3547 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3547 | 42 | 136 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3547 | 2 | 244 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 130 of 3547 | 322 | 3652 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (318) |
| `phiopt` | 615 of 3547 | 2751 | 484 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 6 of 3547 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 84 of 3547 | 743 | 6961 | crc32c 32, target-attribute 28, switch-lowering 15 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2132 of 3547 | 9784 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4459) |
| `header-copy-small` | 2082 of 3547 | 7979 | 973 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4378) |
| `simplify-cfg` | 2596 of 3547 | 53784 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (26432) |
| `number` | 3277 of 3547 | 182073 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (153306) |
| `load-forward` | 450 of 3547 | 5054 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2763) |
| `dse` | 245 of 3547 | 1706 | 374 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1702) |
| `bswap` | 107 of 3547 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `constant-p` | 0 of 3547 | 0 | 0 |  |  |
| `reassoc` | 703 of 3547 | 1648 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1012) |
| `discharge` | 0 of 3547 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3547 | 0 | 0 |  |  |
| `coalesce` | 0 of 3547 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3547 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3547 | 53 | 1885 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3198 of 3547 | 60123 | 43464 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (60123) |
| `loop-delete` | 165 of 3547 | 597 | 4413 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (338) |
| `adce` | 28 of 3547 | 112 | 0 | loop-deletion 16, branch-probability 8, function-purity 4 | instruction nothing necessary reads removed (48) |

