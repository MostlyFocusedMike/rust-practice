- Use `cargo test` to run all the `#[test]` attribute denoted tests
- You can pass in args to this with `--`
- `cargo test --help` is the basic helps `cargo test -- --help` shows all the `--` options
- By default test capture println if they're successful, undo this with `cargo test -- -show-output`
- only run tests that match a name (the function names are what will be tested agains) `cargo test add` get a more specific pattern to test as little as a single function at at ime
- You can also ignore tests with the `#[ignore]` trait, and then you can run ONLY those tests with `cargo test -- --ignored`

## The Tests Module and `#[cfg(test)]`
- the `#[cfg(test)]` attribute tells rust that the module it refers to should only be built for `cargo test` and not `run` which saves a lot of time
- unit tests need this if going in the same directory as your code, but say integration tests do not because they can go in a separate directory
- fun fact you can test private functions by using the `use super::*` rules.

## Integration test
- these pull in your crate just like a third party, so shouldn't go in your build dir
- Make a top level `tests` directoy and put it in there
- Then run test, although it seems to not run integration tests if there are any failed unit tests, which i guess makes sense?
- the `tests` directory is a special name
- Any file in the tests dir gets run, so if you want to share modules for test helpers, put it in the `tests/common/mod.rs` file, that is a special name and path that is not going to get tested

## Integration Tests for Binary Crates (from book, research binary vs lib crates)
If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate with use to make the important functionality available. If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.