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

    cat >> "$LOCAL_CARGO" <<EOL
[[bin]]
name = "naive"
path = "naive.rs"
EOL
fi

for PROBLEM in $PROBLEMS; do
    FILE_PATH="${TARGET_DIR}/${PROBLEM}.rs"
    if [ ! -f "$FILE_PATH" ]; then
        cp "$TEMPLATE" "$FILE_PATH"
    fi
done

if [ ! -f "${TARGET_DIR}/gen.py" ]; then
    cat << 'EOF' > "${TARGET_DIR}/gen.py"
import random

n = random.randint(1, 10)
a = [random.randint(1, 100) for _ in range(n)]

print(n)
print(*a)
EOF
fi

if [ ! -f "${TARGET_DIR}/naive.rs" ]; then
    cp "$TEMPLATE" "${TARGET_DIR}/naive.rs"
fi

echo "Setup completed for workspace member: $PREFIX"