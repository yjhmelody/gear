#!/usr/bin/env sh

set -e
cd "$(dirname "$0")/.."

./scripts/build-wasm.sh

tests=$(echo gtest/spec/*.yaml)
if [ ! -z "$1" ]
  then
    tests=$(echo "$tests" | tr " " "\n" | grep "$1") 
fi
cargo run --package gear-test --release -- $tests
