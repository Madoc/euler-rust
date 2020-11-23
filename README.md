# Project Euler solution in Rust

Some of the [Project Euler] problems, solved by me in [Rust].
I did it to learn the programming language.

Some solutions might not be satisfactory yet.
Those are marked with a todo comment.

The Git history of some solutions may reveal an optimisation evolution.

## How to work with this

To test all minimalistic examples from the problem descriptions, run the unit tests:

```bash
cargo test
```

Build with:

```bash
cargo build --release
```

To output a calculated solution:

```bash
./target/release/euler-rust solve 12
```

Or, to build and run at the same time:

```bash
cargo run --release solve 12
```

Several problem numbers can be given, or `all` for printing all calculated solutions.

For checking if solutions are correct, without printing the solutions:

```bash
./target/release/euler-rust check 12
```

Again, several problems can be passed, or `all` for checking all of them.

`check` will only print incorrect solutions, but never spoil the correct ones.
`solve` will only print the calculated solutions, but not check if they are correct.

To locate the solution implementation for a specific problem, look under `src/problems`.
Each solution is contained in a separate file, such as `src/problems/p005.rs`, for problem number 5.

New solution algorithms can be added to `src/problems/mod.rs`.
The actual problem solution values get added to `src/spoilers.rs`.

After changing the code, apply the code formatter:

```bash
cargo fmt
```

## Lessons learned

Things I learned about Rust from implementing this:

* String formatting and output.
* Destructuring with the `match` statement.
* Module management.
* Unit testing.
* Working with command-line arguments.
* Arrays, vectors, sets and maps.
* Memory allocation.
* Functional collection operators.
* Loop continuation with labels.
* Including static resources with the `!include*` macros.

[Project Euler]: https://projecteuler.net/
[Rust]: https://www.rust-lang.org/
