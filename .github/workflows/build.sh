#!/bin/bash

# generate coverage report
cargo +stable install cargo-llvm-cov --locked
cargo llvm-cov --html

# generate documentation
mkdir -p dist
cargo doc --no-deps
mv target/doc/* dist/
rm -rf dist/.lock
mv target/llvm-cov/html dist/coverage
cat >dist/index.html <<EOF
<!DOCTYPE html>
<html>
  <head>
    <title>Documentation</title>
  </head>
  <body>
  <script>
    window.location.href = "./notox/";
</script>
    <a href="./notox/">Redirecting to doc</a>
  </body>
</html>
EOF
