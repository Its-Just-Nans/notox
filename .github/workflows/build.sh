#!/bin/bash

# generate coverage report
cargo +stable install cargo-llvm-cov --locked
cargo llvm-cov --html

# generate documentation
cargo install comrak
mkdir -p dist
cp -r target/llvm-cov/html dist/coverage
sed 's|CHANGELOG.md|CHANGELOG.html|g' README.md >README.md.tmp
comrak --gfm -o dist/index.html --width 10 README.md.tmp
comrak --gfm -o dist/CHANGELOG.html --width 10 CHANGELOG.md
rm README.md.tmp
