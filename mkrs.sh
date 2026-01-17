#!/bin/bash
# USAGE: ./mkrs.sh [contest_number] (category)

NUMBER=$1
CATEGORY=${2:-abc}
PREFIX="${CATEGORY}${NUMBER}"
TARGET_DIR="./src/${CATEGORY}/${PREFIX}"
TEMPLATE="./template.rs"

mkdir -p "$TARGET_DIR"

# Create Cargo.toml
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

EOL
    # Add binary targets for problems a to g
    for PROBLEM in a b c d e f g; do
        cat >> "$LOCAL_CARGO" <<EOL
[[bin]]
name = "$PROBLEM"
path = "${PROBLEM}.rs"

EOL
    done
fi

# Create problem source files from template
for PROBLEM in a b c d e f g; do
    FILE_PATH="${TARGET_DIR}/${PROBLEM}.rs"
    if [ ! -f "$FILE_PATH" ]; then
        cp "$TEMPLATE" "$FILE_PATH"
    fi
done

echo "Setup completed for workspace member: $PREFIX"