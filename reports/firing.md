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
| `simplify-cfg` | 2252 of 3615 | 13534 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7485) |

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
| `fold` | 3045 of 3615 | 88779 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88779) |
| `image` | 28 of 3615 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `simplify-cfg` | 2637 of 3615 | 53875 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (24859) |
| `sroa` | 500 of 3615 | 1653 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1653) |
| `simplify` | 2452 of 3615 | 25125 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7764) |
| `sccp` | 757 of 3615 | 10247 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8605) |
| `narrow` | 286 of 3615 | 1283 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1283) |
| `short-circuit-free` | 1 of 3615 | 2 | 230 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 165 of 3615 | 550 | 4725 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (444) |
| `phiopt` | 647 of 3615 | 2774 | 611 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2599) |
| `rangetest` | 7 of 3615 | 7 | 0 | short-circuit 4, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (7) |
| `thread-copy` | 72 of 3615 | 228 | 4628 | crc32c 32, target-attribute 28, short-circuit 6 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (153) |
| `prune` | 25 of 3615 | 41 | 6933 | prune 14, simd-lfind 6, switch-dispatch 2 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (17) |
| `canon` | 2107 of 3615 | 9774 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4460) |
| `header-copy` | 2082 of 3615 | 7930 | 999 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4371) |
| `licm` | 716 of 3615 | 4949 | 5953 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4949) |
| `number` | 3374 of 3615 | 208291 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (153503) |
| `load-forward` | 387 of 3615 | 3788 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2735) |
| `rejoin` | 0 of 3615 | 0 | 0 |  |  |
| `constant-p` | 0 of 3615 | 0 | 0 |  |  |
| `hoist` | 0 of 3615 | 0 | 0 |  |  |
| `discharge` | 0 of 3615 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3615 | 0 | 0 |  |  |
| `coalesce` | 0 of 3615 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3615 | 0 | 0 |  |  |
| `dce` | 3269 of 3615 | 56610 | 44855 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (56609) |
| `loop-delete` | 144 of 3615 | 567 | 4377 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (296) |

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
| `inline` | 811 of 3631 | 2698 | 4152 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1222) |
| `libcall` | 17 of 3631 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 94 of 3631 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 137 of 3631 | 178 | 0 | frame-size 53, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (170) |
| `ipa-vrp` | 289 of 3631 | 411 | 780 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (411) |
| `expect` | 32 of 3631 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2380 of 3631 | 91632 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (91632) |
| `image` | 31 of 3631 | 88 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (38) |
| `simplify-cfg` | 2669 of 3631 | 146479 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (66031) |
| `sroa` | 585 of 3631 | 1817 | 125 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1817) |
| `simplify` | 2569 of 3631 | 28810 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8601) |
| `sccp` | 2330 of 3631 | 80355 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (65694) |
| `narrow` | 287 of 3631 | 1284 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1284) |
| `tail-recursion` | 51 of 3631 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3631 | 36 | 120 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3631 | 131 | 165 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 156 of 3631 | 367 | 7295 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (321) |
| `phiopt` | 666 of 3631 | 2852 | 1501 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2659) |
| `rangeswitch` | 1 of 3631 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3631 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 65 of 3631 | 132 | 4773 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (68) |
| `prune` | 29 of 3631 | 44 | 19925 | prune 14, simd-lfind 6, loop-unroll-shape 4 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (18) |
| `canon` | 2167 of 3631 | 13462 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6851) |
| `header-copy` | 2142 of 3631 | 8857 | 1024 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4717) |
| `licm` | 744 of 3631 | 5260 | 13612 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5260) |
| `unroll-free` | 1161 of 3631 | 3264 | 3160 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1632) |
| `lanes` | 0 of 3631 | 0 | 0 |  |  |
| `number` | 3445 of 3631 | 335064 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (270868) |
| `load-forward` | 506 of 3631 | 5675 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3003) |
| `redundant-load` | 161 of 3631 | 366 | 28382 | builtin-setjmp 48, frame-size 27, target-attribute 26 | load replaced by the value of the store the walk found (232) |
| `dse` | 243 of 3631 | 1239 | 720 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1235) |
| `bswap` | 107 of 3631 | 273 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3631 | 0 | 0 |  |  |
| `constant-p` | 1 of 3631 | 5 | 1 | constant-p-after-inline 1 | __builtin_constant_p answered (1) |
| `subscript` | 40 of 3631 | 69 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (69) |
| `hoist` | 0 of 3631 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3631 | 0 | 0 |  |  |
| `split` | 0 of 3631 | 0 | 0 |  |  |
| `reassoc` | 760 of 3631 | 1713 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (987) |
| `loop-idiom` | 17 of 3631 | 17 | 1017 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3631 | 2 | 2919 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 762 of 3631 | 4634 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1661) |
| `ivopts` | 438 of 3631 | 2062 | 112 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1275) |
| `discharge` | 0 of 3631 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3631 | 0 | 0 |  |  |
| `coalesce` | 0 of 3631 | 0 | 0 |  |  |
| `dce` | 3332 of 3631 | 85115 | 49031 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (85115) |
| `gcm` | 255 of 3631 | 6799 | 531 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5781) |
| `loop-delete` | 106 of 3631 | 517 | 3054 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
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
| `inline` | 803 of 3615 | 3569 | 3235 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2202) |
| `libcall` | 17 of 3615 | 40 | 0 | libcall-fold 17 | call to the library whose answer is known folded (25) |
| `ipa-cp` | 88 of 3615 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 131 of 3615 | 172 | 0 | frame-size 48, builtin-setjmp 40, block-copy 22 | parameter nothing reads removed, and the argument at every call with it (164) |
| `ipa-vrp` | 241 of 3615 | 355 | 754 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (355) |
| `expect` | 32 of 3615 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2374 of 3615 | 93420 | 0 | overflow-builtins 144, constant-fold 112, sigsetjmp 112 | instruction with constant operands folded to a constant (93420) |
| `image` | 32 of 3615 | 96 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `simplify-cfg` | 2653 of 3615 | 152893 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (68476) |
| `sroa` | 585 of 3615 | 1833 | 125 | overflow-builtins 144, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1833) |
| `simplify` | 2564 of 3615 | 29181 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8677) |
| `sccp` | 2314 of 3615 | 81902 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value proved constant and replaced (67069) |
| `narrow` | 286 of 3615 | 1283 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1283) |
| `tail-recursion` | 51 of 3615 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3615 | 36 | 116 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3615 | 131 | 165 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 156 of 3615 | 367 | 7398 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (321) |
| `phiopt` | 666 of 3615 | 2932 | 1525 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2739) |
| `rangeswitch` | 1 of 3615 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3615 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 65 of 3615 | 132 | 4876 | crc32c 32, target-attribute 28, jump-threading 2 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (68) |
| `prune` | 28 of 3615 | 42 | 20220 | prune 14, simd-lfind 6, loop-unroll-shape 4 | select whose condition the value ranges settle where it is read replaced there by the arm it takes (18) |
| `canon` | 2151 of 3615 | 13736 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (7004) |
| `header-copy` | 2126 of 3615 | 9063 | 1024 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4820) |
| `licm` | 742 of 3615 | 5544 | 13676 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5544) |
| `unroll` | 1165 of 3615 | 3418 | 3170 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1709) |
| `lanes` | 0 of 3615 | 0 | 0 |  |  |
| `number` | 3433 of 3615 | 344497 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (279448) |
| `load-forward` | 494 of 3615 | 5856 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by what an earlier load of the same address read (3205) |
| `redundant-load` | 193 of 3615 | 416 | 30600 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 243 of 3615 | 1239 | 736 | sigsetjmp 112, builtin-setjmp 48, unused-returns 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1235) |
| `bswap` | 107 of 3615 | 385 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `rejoin` | 0 of 3615 | 0 | 0 |  |  |
| `constant-p` | 0 of 3615 | 0 | 0 |  |  |
| `subscript` | 44 of 3615 | 77 | 0 | libcall-fold 6, store-fold-constant 6, vla-and-alloca 5 | constant of a narrow subscript moved out into the address (77) |
| `hoist` | 0 of 3615 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3615 | 0 | 0 |  |  |
| `split` | 0 of 3615 | 0 | 0 |  |  |
| `reassoc` | 759 of 3615 | 1724 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (999) |
| `loop-idiom` | 17 of 3615 | 17 | 988 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3615 | 2 | 2950 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 756 of 3615 | 4654 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1666) |
| `ivopts` | 433 of 3615 | 2326 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1445) |
| `discharge` | 0 of 3615 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3615 | 0 | 0 |  |  |
| `coalesce` | 0 of 3615 | 0 | 0 |  |  |
| `dce` | 3316 of 3615 | 86475 | 49153 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (86475) |
| `gcm` | 255 of 3615 | 6931 | 591 | frame-size 48, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5913) |
| `loop-delete` | 106 of 3615 | 517 | 3101 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (284) |
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
| `fold` | 3035 of 3615 | 88638 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88638) |
| `image` | 28 of 3615 | 46 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `simplify-cfg` | 2636 of 3615 | 55475 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (25991) |
| `sroa` | 500 of 3615 | 1653 | 43 | overflow-builtins 144, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1653) |
| `simplify` | 2436 of 3615 | 25024 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (7758) |
| `sccp` | 757 of 3615 | 10247 | 0 | overflow-builtins 108, sigsetjmp 96, interpreter-dispatch 54 | access alignment raised to what its address is proved to be (8605) |
| `narrow` | 286 of 3615 | 1283 | 0 | compare-fold 58, store-fold-constant 58, block-copy 44 | arithmetic redone at the width the program truncates it to (1283) |
| `tail-recursion` | 51 of 3615 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 36 of 3615 | 36 | 116 | switch-dispatch 19, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3615 | 2 | 230 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 165 of 3615 | 553 | 5904 | crc32c 64, target-attribute 28, bundle 25 | edge pointed straight at the arm of the branch it arrives at that it decides (447) |
| `phiopt` | 647 of 3615 | 2774 | 562 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2599) |
| `rangetest` | 6 of 3615 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 85 of 3615 | 99 | 6934 | crc32c 32, target-attribute 28, prune 14 | branch the value ranges settle whichever way control reached it replaced by a jump (67) |
| `canon` | 2150 of 3615 | 9907 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4497) |
| `header-copy-small` | 2099 of 3615 | 8012 | 1011 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4404) |
| `number` | 3374 of 3615 | 207443 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (152525) |
| `load-forward` | 426 of 3615 | 3826 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2735) |
| `dse` | 221 of 3615 | 1106 | 361 | sigsetjmp 112, builtin-setjmp 48, unused-returns 20 | store removed, nothing reads it before it is overwritten or goes out of scope (1102) |
| `bswap` | 107 of 3615 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `rejoin` | 0 of 3615 | 0 | 0 |  |  |
| `constant-p` | 0 of 3615 | 0 | 0 |  |  |
| `reassoc` | 700 of 3615 | 1475 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (893) |
| `discharge` | 0 of 3615 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3615 | 0 | 0 |  |  |
| `coalesce` | 0 of 3615 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3615 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3615 | 53 | 1885 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3271 of 3615 | 59678 | 43769 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (59678) |
| `loop-delete` | 167 of 3615 | 606 | 4423 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (342) |
| `adce` | 20 of 3615 | 72 | 0 | loop-deletion 16, function-purity 4 | instruction nothing necessary reads removed (32) |

