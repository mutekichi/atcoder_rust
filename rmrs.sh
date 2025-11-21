#!/bin/bash

# 引数チェック
if [ $# -lt 1 ]; then
    echo "Usage: ./rmrs.sh [contest_number] (category)"
    echo "Example: ./rmrs.sh 108 arc"
    exit 1
fi

NUMBER=$1
CATEGORY=${2:-abc}
TARGET_PATH_PART="src/${CATEGORY}/${NUMBER}/" # 削除対象のパス識別子
TARGET_DIR="./src/${CATEGORY}/${NUMBER}"
CARGO_TOML="./Cargo.toml"

# 1. ディレクトリの削除
if [ -d "$TARGET_DIR" ]; then
    rm -rf "$TARGET_DIR"
    echo "Deleted directory: $TARGET_DIR"
else
    echo "Directory not found: $TARGET_DIR (Skipping)"
fi

# 2. Cargo.toml のクリーニング
# Pythonを使用して、削除したパスを含む [[bin]] ブロックを除去します
if [ -f "$CARGO_TOML" ]; then
    python3 -c "
import sys

toml_file = '$CARGO_TOML'
target_str = '$TARGET_PATH_PART'

with open(toml_file, 'r') as f:
    lines = f.readlines()

new_lines = []
current_block = []
in_bin_block = False
delete_block = False

for line in lines:
    # [[bin]] が来たら新しいブロックの開始とみなす
    if line.strip() == '[[bin]]':
        # 前のブロックを処理
        if in_bin_block and not delete_block:
            new_lines.extend(current_block)
        
        # ブロック変数の初期化
        in_bin_block = True
        delete_block = False
        current_block = [line]
    elif in_bin_block:
        current_block.append(line)
        # ブロック内に削除対象のパスが含まれていたらフラグを立てる
        if 'path' in line and target_str in line:
            delete_block = True
    else:
        # [[bin]] ブロック以外（[package]など）はそのまま保持
        new_lines.append(line)

# 最後のブロックの処理
if in_bin_block and not delete_block:
    new_lines.extend(current_block)

with open(toml_file, 'w') as f:
    f.writelines(new_lines)
"
    echo "Cleaned $CARGO_TOML"
else
    echo "Error: $CARGO_TOML not found."
fi