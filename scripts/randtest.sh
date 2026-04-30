#!/bin/bash

TYPE=$1
CONTEST=$2
PROBLEM=$3

DIR="src/${TYPE}/${TYPE}${CONTEST}"
TARGET="${PROBLEM}"
NAIVE="naive"

cd "$DIR" || exit 1

cargo build --quiet --release --bin "${NAIVE}"
cargo build --quiet --release --bin "${TARGET}"

while true; do
    python3 gen.py > input.txt
    
    cargo run --quiet --release --bin "${NAIVE}" < input.txt > out_naive.txt
    cargo run --quiet --release --bin "${TARGET}" < input.txt > out.txt
    
    diff -q out.txt out_naive.txt > /dev/null
    if [ $? -ne 0 ]; then
        echo "Found failing test case:"
        cat input.txt
        echo "Expected:"
        cat out_naive.txt
        echo "Actual:"
        cat out.txt
        break
    fi
    echo -n "."
done