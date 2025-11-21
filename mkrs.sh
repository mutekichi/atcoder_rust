#!/bin/bash

# USAGE: ./mkrs.sh [contest_number] (category)
# Example: ./mkrs.sh 341 (default category is abc)

# 引数チェック
if [ $# -lt 1 ]; then
    echo "Usage: ./mkrs.sh [contest_number] (category)"
    echo "Example: ./mkrs.sh 341 (default category is abc)"
    exit 1
fi

NUMBER=$1
CATEGORY=${2:-abc} # 第2引数がなければ "abc"
PREFIX="${CATEGORY}${NUMBER}"
TARGET_DIR="./src/${CATEGORY}/${NUMBER}"
TEMPLATE="./template.rs"
CARGO_TOML="./Cargo.toml"

# テンプレート確認
if [ ! -f "$TEMPLATE" ]; then
    echo "Error: $TEMPLATE not found."
    exit 1
fi

# ディレクトリ作成
mkdir -p "$TARGET_DIR"

# a.rs ～ g.rs を作成し、Cargo.toml に追記
for PROBLEM in a b c d e f g; do
    FILE_NAME="${PROBLEM}.rs"
    FILE_PATH="${TARGET_DIR}/${FILE_NAME}"
    BIN_NAME="${PREFIX}_${PROBLEM}"

    # ファイル作成
    if [ ! -f "$FILE_PATH" ]; then
        cp "$TEMPLATE" "$FILE_PATH"
        echo "Created: $FILE_PATH"
    else
        echo "Skipped: $FILE_PATH (already exists)"
    fi

    # Cargo.toml に未登録なら追記
    if ! grep -q "name = \"$BIN_NAME\"" "$CARGO_TOML"; then
        cat >> "$CARGO_TOML" <<EOL

[[bin]]
name = "$BIN_NAME"
path = "$FILE_PATH"
EOL
        echo "Registered '$BIN_NAME' to Cargo.toml"
    fi
done