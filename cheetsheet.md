| C++ | Rust | 備考 |
| :--- | :--- | :--- |
| `vector<T>` | `Vec<T>` | `vec![初期値; サイズ]` マクロが便利 |
| `pair<T, U>` | `(T, U)` | タプルを使用 |
| `long long` | `i64` | 競プロでは `i64` が基本 |
| `size_t` | `usize` | **配列の添字は必ず `usize`** |
| `string` | `String` / `Vec<char>` | 文字単位のアクセスが多いなら `Vec<char>` 推奨 |
| `priority_queue` | `BinaryHeap` | デフォルトは最大ヒープ |
| `priority_queue` (min) | `BinaryHeap<Reverse<T>>` | `std::cmp::Reverse` でラップする |
| `set` | `BTreeSet` | `HashSet` は順序保証なし・定数倍が重い場合あり |
| `map` | `BTreeMap` | `HashMap` は同上 |
| `deque` | `VecDeque` | `use std::collections::VecDeque;` |
| `lower_bound` | `partition_point` | または `binary_search` (完全一致) |
| `next_permutation` | `itertools::permutations` | `itertools` クレートを使用 |

-----
