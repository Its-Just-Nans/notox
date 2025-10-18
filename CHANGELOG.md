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

## 2025-10-18

- Bump to version `1.4.1`
- Add `rayon`
- Change args

## 2025-10-16

- Bump to version `1.2.1`
- Change variable names
- Change internal `PathChange`

## 2025-04-10

- Make in more rustic way
- Use HashSet instead of Vec
- Bump to version `1.1.0`
- Bump to version `1.1.1`

## 2024-12-29

- Fix typos
- Bump to version `1.0.10`

## 2024-12-26

- Cleaner code (function removed)
- Bump to version `1.0.9`

## 2024-12-23

- Clippy the code
- Bump to version `1.0.7`
- Change syntaxic sugar
- Change the `Vec<u8>`->`String`->`char` conversion to `[u8; 4]`->`u32` -> `char`
- Bump to version `1.0.8`

## 2024-02-13

- better separation of options - `VerbosityFields` and `OptionsFields`
- Do `.` by default instead of doing nothing
- Bump to version `1.0.6`

## 2024-01-25

- Add tests
- Use `as_encoded_bytes` instead of `as_bytes`
- Bump to version `1.0.5`

## 2024-01-20

- Create a `lib.rs` file
- Add tests
- Bump to version `1.0.4`

## 2024-01-14

- Version `1.0.3`

## 2024-01-01

- Creation of the package
