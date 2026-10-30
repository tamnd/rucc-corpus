# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3615 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3615 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3615 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2243 of 3615 | 12724 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7485) |

## `rucc` at -O1

30 passes ran on 3615 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3615 | 0 |  |
| `constant-p` | 3615 | 0 |  |
| `hoist` | 3615 | 0 |  |
| `discharge` | 3615 | 0 |  |
| `dead-plane` | 3615 | 0 |  |
| `coalesce` | 3615 | 0 |  |
| `plane-sink` | 3615 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 749 of 3615 | 2064 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (705) |
| `libcall` | 17 of 3615 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `expect` | 32 of 3615 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3045 of 3615 | 88775 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88775) |
| `image` | 28 of 3615 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `simplify-cfg` | 2637 of 3615 | 52982 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (24839) |
| `sroa` | 476 of 3615 | 1629 | 67 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1629) |
| `simplify` | 2452 of 3615 | 25359 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8121) |
| `sccp` | 757 of 3615 | 10247 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8605) |
| `narrow` | 286 of 3615 | 1283 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1283) |
| `short-circuit-free` | 1 of 3615 | 2 | 230 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 130 of 3615 | 315 | 3605 | crc32c 64, target-attribute 28, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (307) |
| `phiopt` | 670 of 3615 | 2802 | 1104 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2622) |
| `rangetest` | 7 of 3615 | 7 | 0 | short-circuit 4, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (7) |
| `thread-copy` | 71 of 3615 | 225 | 3512 | crc32c 32, target-attribute 28, short-circuit 6 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (151) |
| `prune` | 68 of 3615 | 86 | 6974 | crc32c 32, target-attribute 28, prune 3 | branch the value ranges settle whichever way control reached it replaced by a jump (68) |
| `canon` | 2107 of 3615 | 9738 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4460) |
| `header-copy` | 2082 of 3615 | 7930 | 999 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4371) |
| `licm` | 716 of 3615 | 4949 | 6320 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4949) |
| `number` | 3374 of 3615 | 209452 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (154440) |
| `load-forward` | 410 of 3615 | 5015 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2763) |
| `rejoin` | 0 of 3615 | 0 | 0 |  |  |
| `constant-p` | 0 of 3615 | 0 | 0 |  |  |
| `hoist` | 0 of 3615 | 0 | 0 |  |  |
| `discharge` | 0 of 3615 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3615 | 0 | 0 |  |  |
| `coalesce` | 0 of 3615 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3615 | 0 | 0 |  |  |
| `dce` | 3265 of 3615 | 57302 | 45483 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (57301) |
| `loop-delete` | 143 of 3615 | 562 | 4378 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (294) |

## `rucc` at -O2

50 passes ran on 3631 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3631 | 0 |  |
| `rejoin` | 3631 | 0 |  |
| `hoist` | 3631 | 0 |  |
| `plane-sink` | 3631 | 0 |  |
| `split` | 3631 | 0 |  |
| `discharge` | 3631 | 0 |  |
| `dead-plane` | 3631 | 0 |  |
| `coalesce` | 3631 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 811 of 3631 | 2826 | 4154 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1034) |
| `libcall` | 17 of 3631 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 94 of 3631 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 137 of 3631 | 178 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (170) |
| `ipa-vrp` | 289 of 3631 | 411 | 780 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (411) |
| `expect` | 32 of 3631 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2380 of 3631 | 91632 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (91632) |
| `image` | 31 of 3631 | 88 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (38) |
| `simplify-cfg` | 2669 of 3631 | 145461 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (66015) |
| `sroa` | 561 of 3631 | 1793 | 173 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1793) |
| `simplify` | 2569 of 3631 | 29044 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8958) |
| `sccp` | 2330 of 3631 | 80351 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (65690) |
| `narrow` | 287 of 3631 | 1284 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1284) |
| `tail-recursion` | 51 of 3631 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3631 | 37 | 121 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3631 | 131 | 165 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 121 of 3631 | 253 | 3901 | crc32c 64, target-attribute 28, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (245) |
| `phiopt` | 690 of 3631 | 2881 | 2461 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2683) |
| `rangeswitch` | 1 of 3631 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3631 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 64 of 3631 | 129 | 3657 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (66) |
| `prune` | 11 of 3631 | 28 | 20046 | loop-unroll-shape 4, prune 3, switch-dispatch 2 | branch the value ranges settle whichever way control reached it replaced by a jump (10) |
| `canon` | 2167 of 3631 | 13404 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6865) |
| `header-copy` | 2142 of 3631 | 8857 | 1024 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4717) |
| `licm` | 744 of 3631 | 5260 | 14318 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5260) |
| `unroll-free` | 1161 of 3631 | 3264 | 3160 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1632) |
| `lanes` | 0 of 3631 | 0 | 0 |  |  |
| `number` | 3445 of 3631 | 336339 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (271888) |
| `load-forward` | 529 of 3631 | 6902 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3871) |
| `redundant-load` | 161 of 3631 | 404 | 29110 | builtin-setjmp 48, frame-size 27, target-attribute 26 | load replaced by the value of the store the walk found (232) |
| `dse` | 267 of 3631 | 1839 | 720 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1835) |
| `bswap` | 107 of 3631 | 273 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3631 | 0 | 0 |  |  |
| `constant-p` | 1 of 3631 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 40 of 3631 | 69 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (69) |
| `hoist` | 0 of 3631 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3631 | 0 | 0 |  |  |
| `split` | 0 of 3631 | 0 | 0 |  |  |
| `reassoc` | 760 of 3631 | 1881 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1107) |
| `loop-idiom` | 17 of 3631 | 17 | 1017 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3631 | 2 | 2919 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 762 of 3631 | 4634 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1661) |
| `ivopts` | 438 of 3631 | 2062 | 112 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1275) |
| `discharge` | 0 of 3631 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3631 | 0 | 0 |  |  |
| `coalesce` | 0 of 3631 | 0 | 0 |  |  |
| `dce` | 3328 of 3631 | 85908 | 49062 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (85908) |
| `gcm` | 222 of 3631 | 6745 | 615 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5796) |
| `loop-delete` | 105 of 3631 | 512 | 3055 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 52 of 3631 | 207 | 0 | frame-size 27, loop-deletion 16, branch-probability 4 | instruction nothing necessary reads removed (123) |

## `rucc` at -O3

50 passes ran on 3615 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `lanes` | 3615 | 0 |  |
| `rejoin` | 3615 | 0 |  |
| `constant-p` | 3615 | 0 |  |
| `hoist` | 3615 | 0 |  |
| `plane-sink` | 3615 | 0 |  |
| `split` | 3615 | 0 |  |
| `discharge` | 3615 | 0 |  |
| `dead-plane` | 3615 | 0 |  |
| `coalesce` | 3615 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 803 of 3615 | 3569 | 3236 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2202) |
| `libcall` | 17 of 3615 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 88 of 3615 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3615 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 241 of 3615 | 355 | 754 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (355) |
| `expect` | 32 of 3615 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2374 of 3615 | 93420 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (93420) |
| `image` | 32 of 3615 | 96 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `simplify-cfg` | 2653 of 3615 | 151875 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (68460) |
| `sroa` | 561 of 3615 | 1809 | 173 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1809) |
| `simplify` | 2564 of 3615 | 29415 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (9034) |
| `sccp` | 2314 of 3615 | 81898 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (67065) |
| `narrow` | 286 of 3615 | 1283 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1283) |
| `tail-recursion` | 51 of 3615 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3615 | 37 | 117 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3615 | 131 | 165 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 121 of 3615 | 253 | 4004 | crc32c 64, target-attribute 28, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (245) |
| `phiopt` | 690 of 3615 | 2961 | 2484 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2763) |
| `rangeswitch` | 1 of 3615 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3615 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 64 of 3615 | 129 | 3760 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (66) |
| `prune` | 10 of 3615 | 26 | 20341 | loop-unroll-shape 4, prune 3, switch-dispatch 2 | branch the value ranges settle whichever way control reached it replaced by a jump (9) |
| `canon` | 2151 of 3615 | 13678 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (7018) |
| `header-copy` | 2126 of 3615 | 9063 | 1024 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4820) |
| `licm` | 742 of 3615 | 5544 | 14382 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5544) |
| `unroll` | 1165 of 3615 | 3418 | 3170 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1709) |
| `lanes` | 0 of 3615 | 0 | 0 |  |  |
| `number` | 3433 of 3615 | 345772 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (280468) |
| `load-forward` | 517 of 3615 | 7083 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3850) |
| `redundant-load` | 193 of 3615 | 454 | 31328 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 267 of 3615 | 1839 | 736 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1835) |
| `bswap` | 107 of 3615 | 385 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3615 | 0 | 0 |  |  |
| `constant-p` | 0 of 3615 | 0 | 0 |  |  |
| `subscript` | 44 of 3615 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3615 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3615 | 0 | 0 |  |  |
| `split` | 0 of 3615 | 0 | 0 |  |  |
| `reassoc` | 759 of 3615 | 1892 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1119) |
| `loop-idiom` | 17 of 3615 | 17 | 988 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3615 | 2 | 2950 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 756 of 3615 | 4654 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1666) |
| `ivopts` | 433 of 3615 | 2326 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1445) |
| `discharge` | 0 of 3615 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3615 | 0 | 0 |  |  |
| `coalesce` | 0 of 3615 | 0 | 0 |  |  |
| `dce` | 3312 of 3615 | 87268 | 49184 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (87268) |
| `gcm` | 222 of 3615 | 6877 | 675 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5928) |
| `loop-delete` | 105 of 3615 | 512 | 3102 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 52 of 3615 | 207 | 0 | frame-size 27, loop-deletion 16, branch-probability 4 | instruction nothing necessary reads removed (123) |

## `rucc` at -Os

37 passes ran on 3615 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `rejoin` | 3615 | 0 |  |
| `constant-p` | 3615 | 0 |  |
| `discharge` | 3615 | 0 |  |
| `dead-plane` | 3615 | 0 |  |
| `coalesce` | 3615 | 0 |  |
| `plane-sink` | 3615 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 749 of 3615 | 2064 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (705) |
| `libcall` | 17 of 3615 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 88 of 3615 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3615 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 264 of 3615 | 367 | 815 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (367) |
| `expect` | 32 of 3615 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 3035 of 3615 | 88634 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88634) |
| `image` | 28 of 3615 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `simplify-cfg` | 2636 of 3615 | 54602 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (25972) |
| `sroa` | 476 of 3615 | 1629 | 67 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1629) |
| `simplify` | 2436 of 3615 | 25258 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8115) |
| `sccp` | 757 of 3615 | 10247 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8605) |
| `narrow` | 286 of 3615 | 1283 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1283) |
| `tail-recursion` | 51 of 3615 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3615 | 37 | 117 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3615 | 2 | 230 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 130 of 3615 | 318 | 3664 | crc32c 64, target-attribute 28, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (310) |
| `phiopt` | 670 of 3615 | 2802 | 1055 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2622) |
| `rangetest` | 6 of 3615 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 68 of 3615 | 144 | 6974 | crc32c 32, target-attribute 28, prune 3 | branch the value ranges settle whichever way control reached it replaced by a jump (127) |
| `canon` | 2150 of 3615 | 9871 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4497) |
| `header-copy-small` | 2099 of 3615 | 8012 | 1011 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4404) |
| `number` | 3374 of 3615 | 208604 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (153462) |
| `load-forward` | 450 of 3615 | 5054 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2763) |
| `dse` | 245 of 3615 | 1706 | 361 | sigsetjmp 112, builtin-setjmp 48, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1702) |
| `bswap` | 107 of 3615 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3615 | 0 | 0 |  |  |
| `constant-p` | 0 of 3615 | 0 | 0 |  |  |
| `reassoc` | 700 of 3615 | 1643 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1013) |
| `discharge` | 0 of 3615 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3615 | 0 | 0 |  |  |
| `coalesce` | 0 of 3615 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3615 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3615 | 53 | 1885 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3267 of 3615 | 60400 | 43797 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (60400) |
| `loop-delete` | 166 of 3615 | 601 | 4424 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (340) |
| `adce` | 20 of 3615 | 72 | 0 | loop-deletion 16, function-purity 4 | instruction nothing necessary reads removed (32) |

