# CHANGELOG

## How to release

```sh
cargo publish --dry-run
cargo publish
```

## How to check coverage

```sh
# install llvm-cov
cargo +stable install cargo-llvm-cov --locked

# run test with coverage
cargo llvm-cov --html

# open coverage report
open target/llvm-cov/html/index.html
```

## How to run test

```sh
cargo test

# show the output of the test
cargo test -- --show-output

# run a specific test
cargo test <test_name>
```

## 2024-01-20

- Create a `lib.rs` file
- Add tests
- Bump to version `1.0.4`

## 2024-01-14

- Version `1.0.3`

## 2024-01-01

- Creation of the package
