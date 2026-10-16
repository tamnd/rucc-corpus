# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3567 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3567 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3567 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2241 of 3567 | 12720 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7485) |

## `rucc` at -O1

29 passes ran on 3567 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `constant-p` | 3567 | 0 |  |
| `hoist` | 3567 | 0 |  |
| `discharge` | 3567 | 0 |  |
| `dead-plane` | 3567 | 0 |  |
| `coalesce` | 3567 | 0 |  |
| `plane-sink` | 3567 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 749 of 3567 | 2064 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (704) |
| `libcall` | 17 of 3567 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `expect` | 32 of 3567 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2986 of 3567 | 88685 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88685) |
| `image` | 28 of 3567 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `sroa` | 470 of 3567 | 1623 | 73 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1623) |
| `simplify` | 2417 of 3567 | 25392 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8121) |
| `sccp` | 446 of 3567 | 3809 | 0 | interpreter-dispatch 54, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2206) |
| `narrow` | 286 of 3567 | 1319 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1319) |
| `short-circuit-free` | 1 of 3567 | 2 | 244 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 130 of 3567 | 319 | 3605 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (315) |
| `phiopt` | 615 of 3567 | 2751 | 497 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 7 of 3567 | 7 | 0 | short-circuit 4, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (7) |
| `thread-copy` | 71 of 3567 | 225 | 3512 | crc32c 32, target-attribute 28, short-circuit 6 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (151) |
| `prune` | 84 of 3567 | 795 | 6981 | crc32c 32, target-attribute 28, switch-lowering 15 | block nothing reaches removed (356) |
| `canon` | 2101 of 3567 | 9676 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4425) |
| `header-copy` | 2055 of 3567 | 7882 | 979 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4344) |
| `licm` | 716 of 3567 | 4942 | 6350 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4942) |
| `simplify-cfg` | 2613 of 3567 | 52920 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (25372) |
| `number` | 3296 of 3567 | 183000 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (154365) |
| `load-forward` | 410 of 3567 | 5015 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2763) |
| `constant-p` | 0 of 3567 | 0 | 0 |  |  |
| `hoist` | 0 of 3567 | 0 | 0 |  |  |
| `discharge` | 0 of 3567 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3567 | 0 | 0 |  |  |
| `coalesce` | 0 of 3567 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3567 | 0 | 0 |  |  |
| `dce` | 3198 of 3567 | 57145 | 45195 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (57144) |
| `loop-delete` | 143 of 3567 | 562 | 4378 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (294) |

## `rucc` at -O2

49 passes ran on 3583 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3583 | 0 |  |
| `hoist` | 3583 | 0 |  |
| `plane-sink` | 3583 | 0 |  |
| `split` | 3583 | 0 |  |
| `discharge` | 3583 | 0 |  |
| `dead-plane` | 3583 | 0 |  |
| `coalesce` | 3583 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 811 of 3583 | 2678 | 4201 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1367) |
| `libcall` | 17 of 3583 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 94 of 3583 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 137 of 3583 | 178 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (170) |
| `ipa-vrp` | 289 of 3583 | 411 | 660 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (411) |
| `expect` | 32 of 3583 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3034 of 3583 | 155354 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (155354) |
| `image` | 32 of 3583 | 96 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 555 of 3583 | 1799 | 185 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1799) |
| `simplify` | 2546 of 3583 | 29059 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8946) |
| `sccp` | 532 of 3583 | 4643 | 0 | crc32c 64, interpreter-dispatch 54, frame-size 45 | access alignment raised to what its address is proved to be (2238) |
| `narrow` | 287 of 3583 | 1320 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1320) |
| `tail-recursion` | 51 of 3583 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3583 | 42 | 140 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3583 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 117 of 3583 | 245 | 3721 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (241) |
| `phiopt` | 661 of 3583 | 2852 | 1847 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2654) |
| `rangeswitch` | 1 of 3583 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3583 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 64 of 3583 | 129 | 3641 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (66) |
| `prune` | 25 of 3583 | 623 | 20093 | switch-lowering 15, prune 3, objtool-shapes 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2168 of 3583 | 13441 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6935) |
| `header-copy` | 2115 of 3583 | 9041 | 1004 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4678) |
| `licm` | 744 of 3583 | 5256 | 14292 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5256) |
| `unroll` | 1173 of 3583 | 3284 | 3143 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1642) |
| `simplify-cfg` | 2645 of 3583 | 158404 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (66634) |
| `lanes` | 0 of 3583 | 0 | 0 |  |  |
| `number` | 3375 of 3583 | 309769 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (271726) |
| `load-forward` | 529 of 3583 | 6914 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3889) |
| `redundant-load` | 165 of 3583 | 408 | 30283 | builtin-setjmp 48, frame-size 27, target-attribute 26 | load replaced by the value of the store the walk found (232) |
| `dse` | 273 of 3583 | 1863 | 733 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1859) |
| `bswap` | 107 of 3583 | 273 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `constant-p` | 1 of 3583 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 44 of 3583 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3583 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3583 | 0 | 0 |  |  |
| `split` | 0 of 3583 | 0 | 0 |  |  |
| `reassoc` | 761 of 3583 | 1884 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1104) |
| `loop-idiom` | 17 of 3583 | 17 | 1013 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3583 | 2 | 2928 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 761 of 3583 | 4656 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1675) |
| `ivopts` | 433 of 3583 | 2022 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1253) |
| `discharge` | 0 of 3583 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3583 | 0 | 0 |  |  |
| `coalesce` | 0 of 3583 | 0 | 0 |  |  |
| `dce` | 3289 of 3583 | 86291 | 48907 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (86291) |
| `gcm` | 196 of 3583 | 6149 | 601 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5932) |
| `loop-delete` | 105 of 3583 | 512 | 3057 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 52 of 3583 | 207 | 0 | frame-size 27, loop-deletion 16, branch-probability 4 | instruction nothing necessary reads removed (123) |

## `rucc` at -O3

49 passes ran on 3567 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3567 | 0 |  |
| `constant-p` | 3567 | 0 |  |
| `hoist` | 3567 | 0 |  |
| `plane-sink` | 3567 | 0 |  |
| `split` | 3567 | 0 |  |
| `discharge` | 3567 | 0 |  |
| `dead-plane` | 3567 | 0 |  |
| `coalesce` | 3567 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 803 of 3567 | 3537 | 3266 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2389) |
| `libcall` | 17 of 3567 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 88 of 3567 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3567 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 241 of 3567 | 355 | 634 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (355) |
| `expect` | 32 of 3567 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3018 of 3567 | 157484 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (157484) |
| `image` | 32 of 3567 | 96 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 555 of 3567 | 1803 | 185 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1803) |
| `simplify` | 2539 of 3567 | 29259 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8962) |
| `sccp` | 533 of 3567 | 4661 | 0 | crc32c 64, interpreter-dispatch 54, frame-size 45 | access alignment raised to what its address is proved to be (2246) |
| `narrow` | 286 of 3567 | 1319 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1319) |
| `tail-recursion` | 51 of 3567 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3567 | 42 | 136 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3567 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 117 of 3567 | 245 | 3796 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (241) |
| `phiopt` | 661 of 3567 | 2908 | 1849 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2722) |
| `rangeswitch` | 1 of 3567 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3567 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 64 of 3567 | 129 | 3716 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (66) |
| `prune` | 23 of 3567 | 619 | 20005 | switch-lowering 15, prune 3, switch-dispatch 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2152 of 3567 | 13492 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6979) |
| `header-copy` | 2099 of 3567 | 9175 | 980 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4733) |
| `licm` | 742 of 3567 | 5501 | 14308 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5501) |
| `unroll` | 1167 of 3567 | 3382 | 3133 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1691) |
| `simplify-cfg` | 2629 of 3567 | 163537 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (68166) |
| `lanes` | 0 of 3567 | 0 | 0 |  |  |
| `number` | 3363 of 3567 | 317546 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (278658) |
| `load-forward` | 523 of 3567 | 7005 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3868) |
| `redundant-load` | 197 of 3567 | 458 | 32449 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 273 of 3567 | 1863 | 733 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1859) |
| `bswap` | 107 of 3567 | 357 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (160) |
| `constant-p` | 0 of 3567 | 0 | 0 |  |  |
| `subscript` | 44 of 3567 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3567 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3567 | 0 | 0 |  |  |
| `split` | 0 of 3567 | 0 | 0 |  |  |
| `reassoc` | 759 of 3567 | 1882 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1103) |
| `loop-idiom` | 17 of 3567 | 17 | 988 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3567 | 2 | 2879 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 756 of 3567 | 4654 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1666) |
| `ivopts` | 433 of 3567 | 2258 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1401) |
| `discharge` | 0 of 3567 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3567 | 0 | 0 |  |  |
| `coalesce` | 0 of 3567 | 0 | 0 |  |  |
| `dce` | 3273 of 3567 | 86771 | 49078 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (86771) |
| `gcm` | 196 of 3567 | 6049 | 649 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5832) |
| `loop-delete` | 105 of 3567 | 512 | 3028 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 52 of 3567 | 207 | 0 | frame-size 27, loop-deletion 16, branch-probability 4 | instruction nothing necessary reads removed (123) |

## `rucc` at -Os

36 passes ran on 3567 cases, and 5 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `constant-p` | 3567 | 0 |  |
| `discharge` | 3567 | 0 |  |
| `dead-plane` | 3567 | 0 |  |
| `coalesce` | 3567 | 0 |  |
| `plane-sink` | 3567 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 749 of 3567 | 2064 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (704) |
| `libcall` | 17 of 3567 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 88 of 3567 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3567 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 264 of 3567 | 367 | 695 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (367) |
| `expect` | 32 of 3567 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2984 of 3567 | 88540 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88540) |
| `image` | 28 of 3567 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `sroa` | 470 of 3567 | 1623 | 73 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1623) |
| `simplify` | 2407 of 3567 | 25306 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8115) |
| `sccp` | 446 of 3567 | 3809 | 0 | interpreter-dispatch 54, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2206) |
| `narrow` | 286 of 3567 | 1319 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1319) |
| `tail-recursion` | 51 of 3567 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3567 | 42 | 136 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3567 | 2 | 244 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 130 of 3567 | 322 | 3664 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (318) |
| `phiopt` | 615 of 3567 | 2751 | 497 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 6 of 3567 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 84 of 3567 | 743 | 6981 | crc32c 32, target-attribute 28, switch-lowering 15 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2144 of 3567 | 9808 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4471) |
| `header-copy-small` | 2094 of 3567 | 8003 | 973 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4390) |
| `simplify-cfg` | 2612 of 3567 | 53917 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (26493) |
| `number` | 3296 of 3567 | 182175 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (153404) |
| `load-forward` | 450 of 3567 | 5054 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2763) |
| `dse` | 245 of 3567 | 1706 | 374 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1702) |
| `bswap` | 107 of 3567 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `constant-p` | 0 of 3567 | 0 | 0 |  |  |
| `reassoc` | 703 of 3567 | 1648 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1012) |
| `discharge` | 0 of 3567 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3567 | 0 | 0 |  |  |
| `coalesce` | 0 of 3567 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3567 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3567 | 53 | 1885 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3218 of 3567 | 60235 | 43533 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (60235) |
| `loop-delete` | 166 of 3567 | 601 | 4424 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (340) |
| `adce` | 28 of 3567 | 112 | 0 | loop-deletion 16, branch-probability 8, function-purity 4 | instruction nothing necessary reads removed (48) |

