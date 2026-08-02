# rust-book

Practical examples of The Book of Rust in <https://doc.rust-lang.org/book/>

## Commands

Init a cargo package:

```shell
cargo new <project_name>
```

Check that the code compiles (is faster than building):

```shell
cargo check
```

Build the executable:

```shell
cargo build
cargo build --release
```

Run the executable:

```shell
cargo run
cargo run --profile release
```

Testing:

```shell
cargo test                              # runs test suite
cargo test -- --show-output             # shows captured stdout (by default it is not shown)
cargo test test_add_works_2             # runs a single test: `test_add_works_2`
cargo test add                          # runs multiple tests that match `add` -> test_add_works & test_add_works_2
cargo test -- --ignored                 # run ignored tests
cargo test --test integration_test      # run only the integration test named `integration_test`
```
