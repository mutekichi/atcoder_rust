import os
import sys

if len(sys.argv) < 3:
    print("Usage: python3 inject.py <template_path> <target_file>")
    sys.exit(1)

template_path = sys.argv[1]
target_file = sys.argv[2]

def extract_snippet(filepath, visited=None):
    if visited is None:
        visited = set()
    
    abs_path = os.path.abspath(filepath)
    if abs_path in visited:
        return []
    visited.add(abs_path)

    if not os.path.exists(filepath):
        print(f"Error: Template not found at {filepath}")
        sys.exit(1)

    with open(filepath, "r", encoding="utf-8") as f:
        lines = f.readlines()

    snippet = []
    in_snippet = False

    for line in lines:
        if line.strip().startswith("// INJECT:"):
            dep_path = line.strip().split(":", 1)[1].strip()
            dep_snippet = extract_snippet(dep_path, visited)
            snippet.extend(dep_snippet)
            continue

        if "// --- SNAP START ---" in line:
            in_snippet = True
            continue
        if "// --- SNAP END ---" in line:
            in_snippet = False
            break
        
        if in_snippet:
            snippet.append(line)

    return snippet

snippet = extract_snippet(template_path)

if not snippet:
    print(f"Error: No snippet markers found or snippet is empty in {template_path}")
    sys.exit(1)

if not os.path.exists(target_file):
    print(f"Error: Target file '{target_file}' not found")
    sys.exit(1)

with open(target_file, "r", encoding="utf-8") as f:
    target_lines = f.readlines()

insert_idx = -1
for i, line in enumerate(target_lines):
    if "// FOR TEMPLATE INJECTIONS" in line:
        insert_idx = i + 1
        break

if insert_idx == -1:
    target_lines.extend(["\n"] + snippet)
else:
    target_lines.insert(insert_idx, "".join(snippet) + "\n")

with open(target_file, "w", encoding="utf-8") as f:
    f.writelines(target_lines)

print(f"Successfully injected '{template_path}' (and dependencies) into {target_file}")