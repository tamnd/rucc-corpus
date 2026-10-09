# What each rucc pass did

Every case rucc builds is compiled with `-frucc-trace`, and the trace says what each optimizer pass did to it. This page adds that up over the corpus, one section per compiler and level. A pass fires on a case when it rewrote something there. What it said it missed is counted next to that and is not firing.

A pass that never fires on programs written for the transformations it makes is dead code or a bug, which is what section 42.2 of rucc's `spec/optimizer/42-measurement.md` asks to find, so those come first. A run of the whole corpus fails on a quiet pass that `quiet-passes.json` does not name with a reason.

## `rucc` at -O0

3 passes ran on 3502 cases, and 0 of them never fired.

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 142 of 3502 | 579 | 0 | crc32c 64, target-attribute 44, simd-lfind 30 | always_inline call inlined (579) |
| `expect` | 32 of 3502 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `simplify-cfg` | 2184 of 3502 | 12633 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block with one way into it merged into the block above it (7385) |

## `rucc` at -O1

29 passes ran on 3502 cases, and 7 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3502 | 0 |  |
| `constant-p` | 3502 | 0 |  |
| `hoist` | 3502 | 0 |  |
| `discharge` | 3502 | 0 |  |
| `dead-plane` | 3502 | 0 |  |
| `coalesce` | 3502 | 0 |  |
| `plane-sink` | 3502 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 747 of 3502 | 2059 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (704) |
| `libcall` | 0 of 3502 | 0 | 0 |  |  |
| `expect` | 32 of 3502 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2930 of 3502 | 88615 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88615) |
| `image` | 27 of 3502 | 45 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `sroa` | 465 of 3502 | 1618 | 67 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1618) |
| `simplify` | 2389 of 3502 | 25285 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8103) |
| `sccp` | 436 of 3502 | 3816 | 0 | interpreter-dispatch 54, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2225) |
| `narrow` | 242 of 3502 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `short-circuit-free` | 1 of 3502 | 2 | 244 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 126 of 3502 | 255 | 3569 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (251) |
| `phiopt` | 615 of 3502 | 2751 | 588 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 7 of 3502 | 7 | 0 | short-circuit 4, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (7) |
| `thread-copy` | 75 of 3502 | 289 | 3412 | crc32c 32, target-attribute 28, short-circuit 6 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (216) |
| `prune` | 84 of 3502 | 795 | 6871 | crc32c 32, target-attribute 28, switch-lowering 15 | block nothing reaches removed (356) |
| `canon` | 2045 of 3502 | 9520 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4325) |
| `header-copy` | 1999 of 3502 | 7682 | 979 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4244) |
| `licm` | 716 of 3502 | 4945 | 6364 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (4945) |
| `simplify-cfg` | 2552 of 3502 | 52901 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (25003) |
| `number` | 3236 of 3502 | 182463 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (154030) |
| `load-forward` | 410 of 3502 | 5015 | 218 | interpreter-dispatch 54, builtin-setjmp 48, induction-variable 32 | load replaced by what an earlier load of the same address read (2763) |
| `constant-p` | 0 of 3502 | 0 | 0 |  |  |
| `hoist` | 0 of 3502 | 0 | 0 |  |  |
| `discharge` | 0 of 3502 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3502 | 0 | 0 |  |  |
| `coalesce` | 0 of 3502 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3502 | 0 | 0 |  |  |
| `dce` | 3129 of 3502 | 56760 | 44886 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with no effects and no users removed (56760) |
| `loop-delete` | 142 of 3502 | 558 | 4279 | loop-deletion 64, function-purity 24, register-pressure 12 | block nothing reaches removed (292) |

## `rucc` at -O2

49 passes ran on 3517 cases, and 8 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3517 | 0 |  |
| `lanes` | 3517 | 0 |  |
| `hoist` | 3517 | 0 |  |
| `plane-sink` | 3517 | 0 |  |
| `split` | 3517 | 0 |  |
| `discharge` | 3517 | 0 |  |
| `dead-plane` | 3517 | 0 |  |
| `coalesce` | 3517 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 809 of 3517 | 2671 | 4201 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (1361) |
| `libcall` | 0 of 3517 | 0 | 0 |  |  |
| `ipa-cp` | 94 of 3517 | 182 | 0 | tail-call 36, frame-size 35, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (105) |
| `ipa-sra` | 111 of 3517 | 144 | 0 | frame-size 53, builtin-setjmp 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (136) |
| `ipa-vrp` | 286 of 3517 | 408 | 659 | crc32c 64, interpreter-dispatch 36, tail-call 36 | range every call agrees on written on a parameter (408) |
| `expect` | 32 of 3517 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2977 of 3517 | 154928 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (154928) |
| `image` | 31 of 3517 | 95 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 549 of 3517 | 1793 | 173 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1793) |
| `simplify` | 2521 of 3517 | 28944 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8938) |
| `sccp` | 522 of 3517 | 4650 | 0 | crc32c 64, interpreter-dispatch 54, frame-size 45 | access alignment raised to what its address is proved to be (2257) |
| `narrow` | 242 of 3517 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `tail-recursion` | 51 of 3517 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3517 | 42 | 139 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3517 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 113 of 3517 | 181 | 3685 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (177) |
| `phiopt` | 659 of 3517 | 2848 | 1917 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2651) |
| `rangeswitch` | 1 of 3517 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3517 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 68 of 3517 | 193 | 3540 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (131) |
| `prune` | 25 of 3517 | 623 | 19808 | switch-lowering 15, prune 3, objtool-shapes 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2111 of 3517 | 13238 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6833) |
| `header-copy` | 2058 of 3517 | 8839 | 1004 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4577) |
| `licm` | 744 of 3517 | 5259 | 14306 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5259) |
| `unroll` | 1153 of 3517 | 3244 | 3062 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1622) |
| `simplify-cfg` | 2583 of 3517 | 157768 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (65960) |
| `lanes` | 0 of 3517 | 0 | 0 |  |  |
| `number` | 3314 of 3517 | 308758 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (271024) |
| `load-forward` | 495 of 3517 | 6880 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3855) |
| `redundant-load` | 155 of 3517 | 398 | 30069 | builtin-setjmp 48, frame-size 27, target-attribute 26 | load replaced by the value of the store the walk found (232) |
| `dse` | 271 of 3517 | 1861 | 2159 | sigsetjmp 104, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1840) |
| `bswap` | 107 of 3517 | 273 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (188) |
| `constant-p` | 5 of 3517 | 35 | 1 | constant-p-after-inline 5 | instruction with constant operands folded to a constant (11) |
| `subscript` | 38 of 3517 | 65 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (65) |
| `hoist` | 0 of 3517 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3517 | 0 | 0 |  |  |
| `split` | 0 of 3517 | 0 | 0 |  |  |
| `reassoc` | 761 of 3517 | 1884 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1104) |
| `loop-idiom` | 17 of 3517 | 17 | 955 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3517 | 2 | 2847 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 725 of 3517 | 4548 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1639) |
| `ivopts` | 433 of 3517 | 2022 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1253) |
| `discharge` | 0 of 3517 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3517 | 0 | 0 |  |  |
| `coalesce` | 0 of 3517 | 0 | 0 |  |  |
| `dce` | 3219 of 3517 | 85732 | 48476 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (85732) |
| `gcm` | 191 of 3517 | 5924 | 601 | frame-size 45, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5924) |
| `loop-delete` | 105 of 3517 | 512 | 2976 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 24 of 3517 | 124 | 0 | loop-deletion 16, branch-probability 4, function-purity 4 | block nothing reaches removed (44) |

## `rucc` at -O3

49 passes ran on 3502 cases, and 9 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3502 | 0 |  |
| `lanes` | 3502 | 0 |  |
| `constant-p` | 3502 | 0 |  |
| `hoist` | 3502 | 0 |  |
| `plane-sink` | 3502 | 0 |  |
| `split` | 3502 | 0 |  |
| `discharge` | 3502 | 0 |  |
| `dead-plane` | 3502 | 0 |  |
| `coalesce` | 3502 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 801 of 3502 | 3530 | 3266 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a small function inlined (2383) |
| `libcall` | 0 of 3502 | 0 | 0 |  |  |
| `ipa-cp` | 88 of 3502 | 176 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 105 of 3502 | 138 | 0 | builtin-setjmp 48, frame-size 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (130) |
| `ipa-vrp` | 239 of 3502 | 353 | 634 | interpreter-dispatch 36, tail-call 36, crc32c 32 | range every call agrees on written on a parameter (353) |
| `expect` | 32 of 3502 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2962 of 3502 | 157076 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (157076) |
| `image` | 31 of 3502 | 95 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to the address it was initialized to (42) |
| `sroa` | 549 of 3502 | 1797 | 173 | overflow-builtins 120, loop-idiom 62, crc32c 56 | local replaced by values, one for each piece of it the function uses (1797) |
| `simplify` | 2511 of 3502 | 29139 | 0 | overflow-builtins 144, compare-fold 118, simplify 114 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8954) |
| `sccp` | 523 of 3502 | 4668 | 0 | crc32c 64, interpreter-dispatch 54, frame-size 45 | access alignment raised to what its address is proved to be (2265) |
| `narrow` | 242 of 3502 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `tail-recursion` | 51 of 3502 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3502 | 42 | 136 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit` | 93 of 3502 | 131 | 179 | crc32c 32, target-attribute 28, short-circuit 14 | both halves of an and-and or an or-or worked out at once, and the branch went (131) |
| `thread` | 113 of 3502 | 181 | 3761 | crc32c 64, target-attribute 32, short-circuit 7 | edge pointed straight at the arm of the branch it arrives at that it decides (177) |
| `phiopt` | 659 of 3502 | 2904 | 1919 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2719) |
| `rangeswitch` | 1 of 3502 | 1 | 0 | short-circuit 1 | branch on comparisons of one value turned into a switch (1) |
| `rangetest` | 76 of 3502 | 81 | 0 | crc32c 32, target-attribute 28, short-circuit 7 | comparisons of one value merged into interval tests (81) |
| `thread-copy` | 68 of 3502 | 193 | 3616 | crc32c 32, target-attribute 28, branch-probability 4 | edge pointed at a copy of the block it arrives at that goes straight to the arm it decides (131) |
| `prune` | 23 of 3502 | 619 | 19721 | switch-lowering 15, prune 3, switch-dispatch 2 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2096 of 3502 | 13292 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | value used outside the loop that defines it passed through the exit instead (6879) |
| `header-copy` | 2043 of 3502 | 8975 | 980 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4633) |
| `licm` | 742 of 3502 | 5504 | 14322 | division 72, crc32c 64, loop-unswitch 64 | computation moved in front of the loop, nothing in the loop changes it (5504) |
| `unroll` | 1147 of 3502 | 3342 | 3053 | compare-fold 118, store-fold-constant 110, crc32c 64 | block nothing reaches removed (1671) |
| `simplify-cfg` | 2568 of 3502 | 162925 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (67498) |
| `lanes` | 0 of 3502 | 0 | 0 |  |  |
| `number` | 3303 of 3502 | 316536 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction removed, an earlier one in the block computes the same thing (277956) |
| `load-forward` | 489 of 3502 | 6971 | 250 | crc32c 64, interpreter-dispatch 54, builtin-setjmp 48 | load replaced by the value a store in the same block wrote there (3834) |
| `redundant-load` | 187 of 3502 | 448 | 32237 | builtin-setjmp 48, crc32c 32, frame-size 27 | load replaced by the value of the store the walk found (264) |
| `dse` | 271 of 3502 | 1861 | 2207 | sigsetjmp 104, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1840) |
| `bswap` | 107 of 3502 | 357 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (160) |
| `constant-p` | 0 of 3502 | 0 | 0 |  |  |
| `subscript` | 38 of 3502 | 65 | 0 | store-fold-constant 6, vla-and-alloca 5, address-fold 4 | constant of a narrow subscript moved out into the address (65) |
| `hoist` | 0 of 3502 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3502 | 0 | 0 |  |  |
| `split` | 0 of 3502 | 0 | 0 |  |  |
| `reassoc` | 759 of 3502 | 1882 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1103) |
| `loop-idiom` | 17 of 3502 | 17 | 930 | loop-idiom 16, conditional-store 1 | loop that stores one byte over a range replaced by memset (9) |
| `vectorize` | 2 of 3502 | 2 | 2799 | short-circuit 2 | loop done four int at a time in the vector registers (2) |
| `widen` | 720 of 3502 | 4546 | 0 | sigsetjmp 112, division 72, crc32c 64 | widening of a counter read off its sixty four bit twin (1630) |
| `ivopts` | 433 of 3502 | 2258 | 111 | division 72, crc32c 64, target-attribute 52 | address use rewritten to read off a pointer of the loop's own (1401) |
| `discharge` | 0 of 3502 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3502 | 0 | 0 |  |  |
| `coalesce` | 0 of 3502 | 0 | 0 |  |  |
| `dce` | 3204 of 3502 | 86227 | 48649 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (86227) |
| `gcm` | 191 of 3502 | 5824 | 649 | frame-size 45, sigsetjmp 42, simd-lfind 30 | computation moved down to where its reads need it (5824) |
| `loop-delete` | 105 of 3502 | 512 | 2948 | loop-deletion 48, loop-idiom 16, bit-loops 8 | block nothing reaches removed (282) |
| `adce` | 24 of 3502 | 124 | 0 | loop-deletion 16, branch-probability 4, function-purity 4 | block nothing reaches removed (44) |

## `rucc` at -Os

36 passes ran on 3502 cases, and 6 of them never fired.

### Never fired

| pass | ran on | missed | what it said |
|---|--:|--:|---|
| `libcall` | 3502 | 0 |  |
| `constant-p` | 3502 | 0 |  |
| `discharge` | 3502 | 0 |  |
| `dead-plane` | 3502 | 0 |  |
| `coalesce` | 3502 | 0 |  |
| `plane-sink` | 3502 | 0 |  |

### Every pass, in the order it runs

| pass | fired on | rewrites | missed | facets it fired on most | what it did most |
|---|--:|--:|--:|---|---|
| `inline` | 747 of 3502 | 2059 | 106 | sigsetjmp 112, crc32c 64, target-attribute 52 | call to a static function called once inlined (704) |
| `libcall` | 0 of 3502 | 0 | 0 |  |  |
| `ipa-cp` | 88 of 3502 | 172 | 0 | tail-call 36, frame-size 30, builtin-setjmp 8 | parameter is the same constant at every call and is now a constant in the body (99) |
| `ipa-sra` | 105 of 3502 | 138 | 0 | builtin-setjmp 48, frame-size 48, general-regs-only 5 | parameter nothing reads removed, and the argument at every call with it (130) |
| `ipa-vrp` | 262 of 3502 | 365 | 694 | crc32c 64, interpreter-dispatch 36, target-attribute 26 | range every call agrees on written on a parameter (365) |
| `expect` | 32 of 3502 | 36 | 0 | if-conversion 10, block-layout 8, switch-lowering 8 | branch weight written from a __builtin_expect on its condition (36) |
| `fold` | 2926 of 3502 | 88381 | 0 | overflow-builtins 144, compare-fold 118, store-fold-constant 116 | instruction with constant operands folded to a constant (88381) |
| `image` | 27 of 3502 | 45 | 0 | function-purity 12, constant-args 4, unused-params 4 | load from a read only object folded to what it was initialized to (17) |
| `sroa` | 465 of 3502 | 1618 | 67 | overflow-builtins 120, crc32c 56, target-attribute 44 | local replaced by values, one for each piece of it the function uses (1618) |
| `simplify` | 2389 of 3502 | 25209 | 0 | overflow-builtins 144, simplify 114, sigsetjmp 112 | (mul.i64 (value.i64 x) (iconst.i64 k)) (8107) |
| `sccp` | 436 of 3502 | 3816 | 0 | interpreter-dispatch 54, frame-size 45, loop-idiom 40 | access alignment raised to what its address is proved to be (2225) |
| `narrow` | 242 of 3502 | 1275 | 0 | compare-fold 58, store-fold-constant 58, frame-size 27 | arithmetic redone at the width the program truncates it to (1275) |
| `tail-recursion` | 51 of 3502 | 69 | 0 | tail-call 36, constant-args 4, unused-params 4 | a call to the function itself in tail position became a jump to its top (69) |
| `switch-conv` | 37 of 3502 | 42 | 136 | switch-dispatch 20, switch-lowering 11, switch-runs 6 | switch replaced by a range check and a load from a table of its answers (23) |
| `short-circuit-free` | 1 of 3502 | 2 | 244 | value-range 1 | both halves of an and-and or an or-or worked out at once, and the branch went (2) |
| `thread` | 126 of 3502 | 257 | 3753 | crc32c 64, target-attribute 32, short-circuit 14 | edge pointed straight at the arm of the branch it arrives at that it decides (253) |
| `phiopt` | 615 of 3502 | 2751 | 588 | compare-fold 118, crc32c 64, loop-unswitch 64 | branch whose two arms only work out a value replaced by the value and no branch (2571) |
| `rangetest` | 6 of 3502 | 6 | 0 | short-circuit 3, lkmm 2, value-range 1 | branches on comparisons of one value merged into one test (6) |
| `prune` | 84 of 3502 | 683 | 6931 | crc32c 32, target-attribute 28, switch-lowering 15 | case whose value the switched value cannot hold taken out of the switch (351) |
| `canon` | 2088 of 3502 | 9652 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | back edges to one header routed through one latch (4371) |
| `header-copy-small` | 2038 of 3502 | 7803 | 973 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | loop header copied in front of the loop so the test is at the bottom (4290) |
| `simplify-cfg` | 2547 of 3502 | 53511 | 0 | compare-fold 118, sigsetjmp 112, store-fold-constant 110 | block parameter that arrives as the same value every way in removed (26059) |
| `number` | 3232 of 3502 | 181426 | 0 | overflow-builtins 144, simplify 114, constant-fold 112 | instruction removed, an earlier one in the block computes the same thing (152835) |
| `load-forward` | 416 of 3502 | 5020 | 218 | interpreter-dispatch 54, builtin-setjmp 48, loop-restructure 36 | load replaced by what an earlier load of the same address read (2763) |
| `dse` | 245 of 3502 | 1706 | 1108 | sigsetjmp 100, builtin-setjmp 43, overflow-builtins 24 | store removed, nothing reads it before it is overwritten or goes out of scope (1685) |
| `bswap` | 107 of 3502 | 129 | 0 | crc32c 64, target-attribute 24, bit-liveness 8 | loads of neighbouring bytes merged into one load (92) |
| `constant-p` | 0 of 3502 | 0 | 0 |  |  |
| `reassoc` | 702 of 3502 | 1647 | 0 | register-pressure 60, sigsetjmp 56, interpreter-dispatch 54 | integer tree rebuilt in rank order (1012) |
| `discharge` | 0 of 3502 | 0 | 0 |  |  |
| `dead-plane` | 0 of 3502 | 0 | 0 |  |  |
| `coalesce` | 0 of 3502 | 0 | 0 |  |  |
| `plane-sink` | 0 of 3502 | 0 | 0 |  |  |
| `loop-idiom` | 53 of 3502 | 53 | 1819 | loop-idiom 40, alias-analysis 8, load-forwarding 4 | loop that stores one byte over a range replaced by memset (29) |
| `dce` | 3149 of 3502 | 59825 | 43233 | overflow-builtins 144, compare-fold 118, store-fold-constant 118 | instruction with no effects and no users removed (59825) |
| `loop-delete` | 165 of 3502 | 597 | 4325 | loop-deletion 64, loop-idiom 40, register-pressure 12 | block nothing reaches removed (338) |
| `adce` | 28 of 3502 | 112 | 0 | loop-deletion 16, branch-probability 8, function-purity 4 | instruction nothing necessary reads removed (48) |

