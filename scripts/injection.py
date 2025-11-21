import os
import sys

# Usage: python3 inject.py <template_name> <target_file>

if len(sys.argv) < 3:
    print("Usage: python3 inject.py <template_name> <target_file>")
    sys.exit(1)

template_name = sys.argv[1]
target_file = sys.argv[2]
template_path = os.path.join(os.path.dirname(__file__),
                             f"../src/template/{template_name}.rs")

# 1. テンプレートの読み込みと抽出
if not os.path.exists(template_path):
    print(f"Error: Template '{template_name}' not found at {template_path}")
    sys.exit(1)

with open(template_path, 'r') as f:
    t_lines = f.readlines()

snippet = []
in_snippet = False
for line in t_lines:
    if "// --- SNAP START ---" in line:
        in_snippet = True
        continue
    if "// --- SNAP END ---" in line:
        in_snippet = False
        break
    if in_snippet:
        snippet.append(line)

if not snippet:
    print(f"Error: No snippet markers found in {template_name}.rs")
    sys.exit(1)

# 2. ターゲットファイルの読み込み
if not os.path.exists(target_file):
    print(f"Error: Target file '{target_file}' not found")
    sys.exit(1)

with open(target_file, 'r') as f:
    target_lines = f.readlines()

# 3. 挿入位置の決定
insert_idx = -1
for i, line in enumerate(target_lines):
    if "FOR TEMPLATE INJECTIONS" in line:
        insert_idx = i + 1
        break

# 見つからなければファイルの最後に追加
if insert_idx == -1:
    target_lines.extend(['\n'] + snippet)
else:
    target_lines.insert(insert_idx, "".join(snippet) + "\n")

# 4. 書き込み
with open(target_file, 'w') as f:
    f.writelines(target_lines)

print(f"Successfully injected '{template_name}' into {target_file}")
