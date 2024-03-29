#!/bin/bash

# generate coverage report
cargo +stable install cargo-llvm-cov --locked
cargo llvm-cov --html

# generate documentation
cargo install comrak
mkdir -p dist
cp -r target/llvm-cov/html dist/coverage
sed 's|CHANGELOG.md|CHANGELOG.html|g' README.md >README.md.tmp
comrak --gfm -o dist/index.html --width 80 README.md.tmp
comrak --gfm -o dist/CHANGELOG.html --width 80 CHANGELOG.md

# cleanup
rm README.md.tmp
