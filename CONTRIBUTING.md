# Contributing

## Adding a facet

A facet is one named transformation. Adding one means three things.

Add the variant to `Facet` in `crates/corpus-model/src/facet.rs`, give it a name, a phase and a one sentence description. The name is what appears in every report and every directory, so it is lowercase with hyphens and it is the name the transformation actually goes by.

Write the generator in the module for its phase under `crates/corpus-gen/src/facets/`. Name the axes it walks and walk all of them. A facet that emits three programs somebody thought of is a facet that will pass on the day the pass breaks in a shape nobody thought of.

Compute the answer. Do not write the answer down. `lang::eval` knows the integer semantics and it returns nothing when the program would be undefined, and a case with no answer is dropped rather than emitted with a guess. If a facet needs an answer `eval` cannot produce, the fix is to teach `eval`, not to hand-write the number.

Then `cargo run -p rucc-corpus -- gen --clean` and commit the C along with the generator.

## The rules a case has to follow

Everything printed goes out through `long long` or `unsigned long long`, with `%lld` or `%llu`. That is one format for every width and it does not change with the machine.

Nothing printed may depend on the machine. No `%p`, no `sizeof`, no `__FILE__`, no `__LINE__`, no `__DATE__`, no `__TIME__`, no byte order. There is a test that enforces this across the whole corpus and it will catch you.

`printf` is declared by hand rather than included, so a case tests the compiler and not the system headers. Testing the headers is what rucc-compat is for.

Nothing may be undefined. No signed overflow, no shift by more than the width of the promoted type, no reading an uninitialised object, no aliasing violation that is not in the `barrier` facet on purpose.

An opaque input is a read of a `volatile` global. Not inline assembly, which is a portability problem, and not a command line argument, which the harness would have to pass and a person reproducing by hand would not.

## Reviewing a generator change

The diff of `programs/` is the review. That is the whole reason the generated C is checked in. A change that was meant to add one facet and moved four hundred existing files did something else as well, and the diff is where that shows up.

## Style

Plain English. No em dashes. No sentence broken across two lines. Comments explain why, because what the code does is in the code.

A comment that says what a threshold is worth having. A comment that says why the threshold is two percent and not one is worth much more, and it is the one that stops somebody changing it back in six months.

## Before opening a pull request

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
cargo run --release -p rucc-corpus -- gen --clean
git status --short programs
```

The last two are the ones people forget. CI regenerates the corpus and fails if anything moved.

If you have GCC 16, run the corpus against it before opening the pull request. It is the only thing that catches an expected answer that is wrong, and an expected answer that is wrong is the worst bug this repository can have, because it makes a correct compiler look broken.

```sh
cargo run --release -p rucc-corpus -- run --toolchain gcc-16 --reference gcc-16 --level O0 --level O2
```
