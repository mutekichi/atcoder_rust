#!/bin/bash
# USAGE: ./mkrs.sh [contest_number] (category)

NUMBER=$1
CATEGORY=${2:-abc}
PREFIX="${CATEGORY}${NUMBER}"
TARGET_DIR="./src/${CATEGORY}/${PREFIX}"
TEMPLATE="./template.rs"

if [[ "$CATEGORY" != "abc" && "$CATEGORY" != "arc" && "$CATEGORY" != "ahc" ]]; then
    echo "Error: Category must be abc, arc, or ahc."
    exit 1
fi

mkdir -p "$TARGET_DIR"

if [ "$CATEGORY" == "abc" ]; then
    PROBLEMS="a b c d e f g"
elif [ "$CATEGORY" == "arc" ]; then
    PROBLEMS="a b c d e f g h"
elif [ "$CATEGORY" == "ahc" ]; then
    PROBLEMS="a"
fi

LOCAL_CARGO="${TARGET_DIR}/Cargo.toml"
if [ ! -f "$LOCAL_CARGO" ]; then
    cat > "$LOCAL_CARGO" <<EOL
[package]
name = "$PREFIX"
version = "0.1.0"
edition = "2024"

[dependencies]
proconio = { workspace = true }
itertools = { workspace = true }
superslice = { workspace = true }
num-integer = { workspace = true }
rand = { workspace = true }
memoise = { workspace = true }

EOL

    for PROBLEM in $PROBLEMS; do
        cat >> "$LOCAL_CARGO" <<EOL
[[bin]]
name = "$PROBLEM"
path = "${PROBLEM}.rs"

EOL
    done

    if [ "$CATEGORY" == "ahc" ]; then
        cat >> "$LOCAL_CARGO" <<EOL
[[bin]]
name = "calc_score"
path = "calc_score.rs"

EOL
    fi
fi

for PROBLEM in $PROBLEMS; do
    FILE_PATH="${TARGET_DIR}/${PROBLEM}.rs"
    if [ ! -f "$FILE_PATH" ]; then
        cp "$TEMPLATE" "$FILE_PATH"
    fi
done

if [ "$CATEGORY" == "ahc" ]; then
    FILE_PATH="${TARGET_DIR}/calc_score.rs"
    if [ ! -f "$FILE_PATH" ]; then
        cat > "$FILE_PATH" <<'EOF'
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <in_file> <out_file>", args[0]);
        std::process::exit(1);
    }
    let in_file = &args[1];
    let out_file = &args[2];
    
    let score: i64 = 0;
    
    println!("{}", score);
}
EOF
    fi
    mkdir -p "${TARGET_DIR}/in" "${TARGET_DIR}/out"
fi

echo "Setup completed for workspace member: $PREFIX"