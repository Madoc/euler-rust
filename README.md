# Project Euler solution in Rust

Some of the [Project Euler] problems, solved by me in [Rust].
I did it to learn the programming language.

Some solutions might not be satisfactory yet.
Those were marked with a todo comment.

The Git history of some solutions may reveal an optimisation evolution.

## How to work with this

In order to run the solution to a specific problem:

1. Open `main.rs`.
   Change the value of `PROBLEM_NUMBER` to the number of the problem to run.
2. ```bash
   cargo run
   ```

To locate the solution for a specific problem, look under `src/problems`.
Each solution is contained in a separate file, such as `src/problems/p005.rs`, for problem number 5.

The example solutions, as given in the problem descriptions for a reduced case of each problem, have been implemented as
 Rust unit tests.
For running the unit tests:

```bash
cargo test
```

New solutions can be added to `src/problems/mod.rs`.

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
* Loop continuation with labels.
* Arrays, vectors, sets and maps.
* Functional collection operators.

[Project Euler]: https://projecteuler.net/
[Rust]: https://www.rust-lang.org/
