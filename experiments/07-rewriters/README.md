# Experiment 7: the rewriters

This is experiment 7 of section 42.5 in rucc's [optimizer measurement plan](https://github.com/tamnd/rucc/blob/main/spec/optimizer/42-measurement.md), the one that settles the central architectural bet of the middle end, and the measurement behind the answer to open question one in [`spec/19-open-questions.md`](https://github.com/tamnd/rucc/blob/main/spec/19-open-questions.md). It is tamnd/rucc#2832, and the answer is written up in [section 12.9 of the e-graph specification](https://github.com/tamnd/rucc/blob/main/spec/optimizer/12-egraph.md#129-what-the-experiment-found).

**The answer is that the conventional pipeline ships.** By the rule of [section 12.3](https://github.com/tamnd/rucc/blob/main/spec/optimizer/12-egraph.md#123-the-three-way-experiment), the hash-consed rewriter would have to retire 1% fewer instructions than the classical arm, and it retires 0.001% fewer. The e-graph would have to retire 0.5% fewer than the hash-consed rewriter, and it retires 0.0002% more. Every arm prints the right answer for every case at both levels, so the decision is about cost alone.

The machine-readable numbers are in [results.json](results.json).

## The arms

| arm | flag | what it is | lines it adds to the middle end |
|---|---|---|---|
| `conventional` | none, it is the default | `simplify` then `number`, each once, where a level runs them | 0 |
| `classical` | `-Zrewriter=classical` | arm A: `simplify` to a bounded fixpoint, then `number` | 39 |
| `consed` | `-Zrewriter=consed` | arm B: rewrite at construction with hash-consing, one representative per value | 269 |
| `egraph` | `-Zrewriter=egraph` | arm C: arm B plus e-classes and extraction by the costs of section 40 of the optimizer specification | 555 |
| `egraph-forms` | `-Zrewriter=egraph`, after tamnd/rucc#2831 | arm C with the classes kept into instruction selection | 650 |

The lines are the lines of Rust, not counting tests, comments or blank lines, that each arm adds over the conventional pipeline, counted the same way for every arm. `rucc-opt` is about 34,000 such lines. The rule set, the cost model, global code motion and everything else in the pipeline is shared, which is what section 12.3 asks for, so the arms differ in how the rules are applied and nothing else.

## How it was run

Every case of rucc-corpus at commit `b3b7e40`, 3271 of them, built at `-O2` and `-O3` by each arm with gcc-16 as the reference, three timed repeats and five counted ones each, on server2, an x86-64 Linux machine. SQLite's amalgamation was built by each arm at both levels, with and without `-Zverify-each`, linked into the shell and run on a query file against gcc-16's output. The compile times come from a separate pass over every file with `-frucc-trace`, all five arms back to back on each file, so the machine was as busy for one as for the others.

Instructions retired stand in for execution time, because they barely move from run to run and the differences here are far smaller than the noise in a timer. They are summed over the cases every arm and gcc-16 have a count for, 3158 at `-O2` and 3159 at `-O3`. That leaves out the 112 `sigsetjmp` cases, whose counts differ from one run to the next, and the one or two the counter missed on a busy machine.

## The totals at -O2

| arm | right answers | text bytes | against conventional | smaller | larger | instructions retired | against conventional | against gcc-16 |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 3271 of 3271 | 3,042,182 |  |  |  | 19,149,757,538 |  |  |
| `conventional` | 3271 of 3271 | 3,729,571 |  |  |  | 23,498,345,427 |  | +22.708% |
| `classical` | 3271 of 3271 | 3,735,166 | +0.150% | 76 | 105 | 23,498,611,227 | +0.001% | +22.710% |
| `consed` | 3271 of 3271 | 3,727,089 | -0.067% | 89 | 82 | 23,498,430,565 | +0.000% | +22.709% |
| `egraph` | 3271 of 3271 | 3,727,919 | -0.044% | 107 | 136 | 23,498,481,469 | +0.001% | +22.709% |
| `egraph-forms` | 3271 of 3271 | 3,727,895 | -0.045% | 115 | 136 | 23,498,480,826 | +0.001% | +22.709% |

| arm | compile seconds | against conventional | optimizer seconds | against conventional | rewriter seconds |
|---|---|---|---|---|---|
| `conventional` | 177.2 |  | 78.3 |  | 13.9 |
| `classical` | 183.7 | +3.689% | 86.0 | +9.821% | 17.8 |
| `consed` | 185.2 | +4.548% | 86.2 | +10.115% | 19.8 |
| `egraph` | 187.4 | +5.779% | 88.5 | +13.008% | 23.2 |
| `egraph-forms` | 189.0 | +6.665% | 91.4 | +16.766% | 24.2 |

The times are summed over the 3339 files that compile. The rewriter seconds are the time in `simplify`, `number`, `simplify-fixpoint`, `cons` and `egraph`, whichever of them the arm runs.

## The totals at -O3

| arm | right answers | text bytes | against conventional | smaller | larger | instructions retired | against conventional | against gcc-16 |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 3271 of 3271 | 3,717,655 |  |  |  | 17,552,669,892 |  |  |
| `conventional` | 3271 of 3271 | 3,729,571 |  |  |  | 23,498,459,879 |  | +33.874% |
| `classical` | 3271 of 3271 | 3,735,166 | +0.150% | 76 | 105 | 23,498,725,205 | +0.001% | +33.876% |
| `consed` | 3271 of 3271 | 3,727,089 | -0.067% | 89 | 82 | 23,498,544,631 | +0.000% | +33.874% |
| `egraph` | 3271 of 3271 | 3,727,919 | -0.044% | 107 | 136 | 23,498,594,808 | +0.001% | +33.875% |
| `egraph-forms` | 3271 of 3271 | 3,727,895 | -0.045% | 115 | 136 | 23,498,594,625 | +0.001% | +33.875% |

| arm | compile seconds | against conventional | optimizer seconds | against conventional | rewriter seconds |
|---|---|---|---|---|---|
| `conventional` | 195.3 |  | 90.0 |  | 14.6 |
| `classical` | 198.5 | +1.644% | 94.4 | +4.813% | 19.0 |
| `consed` | 203.0 | +3.922% | 98.9 | +9.820% | 22.7 |
| `egraph` | 209.5 | +7.247% | 103.3 | +14.713% | 25.5 |
| `egraph-forms` | 208.8 | +6.892% | 103.5 | +14.940% | 27.2 |

The times are summed over the 3339 files that compile. The rewriter seconds are the time in `simplify`, `number`, `simplify-fixpoint`, `cons` and `egraph`, whichever of them the arm runs.

## SQLite

| arm | text bytes at -O2 and -O3 | against conventional |
|---|---|---|
| `conventional` | 1,169,344 | +0.000% |
| `classical` | 1,171,231 | +0.161% |
| `consed` | 1,170,064 | +0.062% |
| `egraph` | 1,169,776 | +0.037% |
| `egraph-forms` | 1,169,776 | +0.037% |

Every arm's shell prints what gcc-16's prints, and `-Zverify-each` passes for every arm. SQLite's compile times are not here. Each arm built it three times at each level, but the machine was shared and busy, and the same pass took three times as long in one build as in the next, which is far more than the arms differ by. The corpus times above do not have that problem, because the five arms ran one after another on each file, so whatever else the machine was doing fell on all of them alike over 3339 files.

## The e-classes

Over every file in the corpus at `-O2`, the e-graph held 1,627,468 forms in 1,479,735 classes, an average of 1.10 forms per class. Cranelift reports 1.13. 74,901 of the forms were kept for instruction selection by `egraph-forms`, and eight cases took one.

## Where the arms differ

The arms differ by a fraction of a percent, and the cases behind that are few enough to name.

- `classical` against `conventional`: 76 smaller and 105 larger. The second round of rewrites pays in `target-attribute` and `simd-lfind`, 47 cases and 1,473 bytes between them, and costs in `frame-size`, `interpreter-dispatch`, `register-pressure` and `value-settled`, 74 cases and 6,023 bytes, where the forms it reaches keep more values live. In instructions it is level.
- `consed` against `conventional`: 89 smaller and 82 larger, and level in instructions. Rewriting at construction takes 3,072 bytes out of the 64 `crc32c` cases and 300 out of `loop-idiom`, and puts back a few bytes each in `computed-goto`, `declared-purity`, `constant-args` and `sigsetjmp`.
- `egraph` against `consed`: 6 smaller and 48 larger. The larger ones are 28 `loop-unswitch` cases where extraction keeps constants loaded higher up and live across the loop, 12 `register-pressure.*.through-a-loop` cases where it extracts `x * 5 + (x + 5)` over the folded `x * 6`, and 3 `simplify.*.select.minus-one-zero` cases where it extracts a select over a negated compare. Each is the cost model pricing two forms the same or the wrong way round, which is the risk open question one named when it was asked: a cost model over a canonical graph is a weaker instrument than a pass that knows what it just did.
- `egraph-forms` against `egraph`: 8 smaller and none larger, the `simplify.*.widened-boolean` cases, where a comparison against zero of a difference becomes one `cmp`.

## By facet

Text bytes and instructions retired per facet at `-O2`, each arm against `conventional`. A blank is no change, or for instructions a change under 0.01%, which is as far as the count moves from one run of the same program to the next. Only the 27 facets where some arm moved are here, and the other 63 are the same under every arm. The same table for `-O3`, and the raw numbers for every facet at both levels, are in [results.json](results.json).

| facet | phase | cases | `classical` text | `classical` instructions | `consed` text | `consed` instructions | `egraph` text | `egraph` instructions | `egraph-forms` text | `egraph-forms` instructions |
|---|---|---|---|---|---|---|---|---|---|---|
| `atomics` | correctness | 31 | -0.040% |  |  |  |  |  |  |  |
| `baseline` | floor | 10 |  |  | -0.200% |  | -0.200% |  | -0.200% |  |
| `code-motion` | global | 12 |  |  | -1.113% |  | -1.113% |  | -1.113% |  |
| `computed-goto` | floor | 25 |  |  | +0.334% | +0.029% | +0.334% | +0.029% | +0.334% | +0.029% |
| `conditional-store` | local | 20 | -0.191% | -0.084% |  |  |  |  |  |  |
| `constant-args` | interprocedural | 36 |  |  | +0.684% |  | +0.684% |  | +0.684% |  |
| `crc32c` | correctness | 64 |  | +0.037% | -0.806% | +0.462% | -0.806% | +0.462% | -0.806% | +0.462% |
| `declared-purity` | interprocedural | 32 |  |  | +0.853% |  | +0.853% |  | +0.853% |  |
| `frame-size` | backend | 54 | +2.634% | +0.545% |  |  |  |  |  |  |
| `function-purity` | interprocedural | 28 | -0.097% |  |  |  |  |  |  |  |
| `interpreter-dispatch` | backend | 36 | +0.295% | -0.020% |  |  |  |  |  |  |
| `iv-selection` | loops | 36 |  |  | +0.203% |  | +0.203% |  | +0.203% |  |
| `jump-threading` | global | 5 |  |  | +0.880% | +0.638% | +0.880% | +0.638% | +0.880% | +0.639% |
| `load-forwarding` | global | 40 |  |  | +0.188% |  | +0.188% |  | +0.188% |  |
| `loop-idiom` | loops | 78 |  |  | -0.959% |  | -0.959% |  | -0.959% |  |
| `loop-restructure` | loops | 48 | -0.269% |  |  |  |  |  |  |  |
| `loop-shape` | loops | 64 |  |  | -0.067% |  | -0.067% |  | -0.067% |  |
| `loop-unswitch` | loops | 64 |  |  | +0.057% | +0.011% | +2.909% | +0.014% | +2.909% | +0.014% |
| `register-pressure` | backend | 60 | +4.066% |  |  |  | +0.360% |  | +0.360% |  |
| `short-circuit` | local | 24 | +0.017% |  |  |  |  |  |  |  |
| `sigsetjmp` | correctness | 112 | +0.545% |  | +0.205% |  | +0.192% |  | +0.192% |  |
| `simd-lfind` | correctness | 30 | -1.040% | +0.559% |  |  |  |  |  |  |
| `simplify` | local | 114 |  |  | +0.131% |  | +0.161% | +0.237% | +0.138% | +0.237% |
| `target-attribute` | correctness | 52 | -0.401% |  |  |  | -0.031% | +0.013% | -0.031% | +0.013% |
| `value-range` | global | 13 |  |  | +0.102% |  | +0.102% |  | +0.102% |  |
| `value-settled` | local | 13 | +1.470% | +0.357% | +0.124% | +0.032% | +0.124% | +0.032% | +0.124% | +0.032% |
| `vla-and-alloca` | floor | 9 | -0.291% |  | -0.272% |  | -0.272% |  | -0.272% |  |

## What happens to the arms that lost

They stay. `-Zrewriter=classical`, `consed` and `egraph` are behind a flag and off by default, they cost nothing when they are not asked for, and they are what makes the experiment cheap to run again. The rule set is small today, and the place the e-graph could still win is a rule set large enough that the order of rewrites matters. Each time the rule set grows by a phase, this experiment is run again and this page is written again from the new numbers.

## Running it again

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 --reference gcc-16 \
    --toolchain conventional=rucc \
    --toolchain classical=rucc --flag classical=-Zrewriter=classical \
    --toolchain consed=rucc --flag consed=-Zrewriter=consed \
    --toolchain egraph=rucc --flag egraph=-Zrewriter=egraph \
    --level O2 --level O3 --repeats 3 --out experiments/07-rewriters/run
```

The compile times are the `seconds`, `phases.optimize` and `passes` fields that `-frucc-trace=FILE` writes for each compile, summed over every file. rucc writes one JSON line per compile to that file, so a loop over the corpus with the flag on gives them.
