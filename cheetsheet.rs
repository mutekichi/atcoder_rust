#![allow(unused_imports)]
#![allow(dead_code)]

use itertools::Itertools;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write};

// This function is just a container for snippets.
fn snippets<W: Write>(out: &mut W) {
    // ==========================================
    // 0. Variable Initialization
    // ==========================================

    // vector<vector<int>> v(n, vector<int>(m, initial_value));
    let v: Vec<Vec<usize>> = vec![vec![0; 10]; 5]; // 5 x 10 zero matrix
                                                   // vector<vector<int>> graph(n);
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); 5]; // 5 empty vectors
    graph[2].push(7);

    // ==========================================
    // 1. Input Patterns (proconio)
    // ==========================================

    input! {
        // Basic integers
        n: usize,
        m: i64,

        // Vector (A_1, ..., A_N)
        a: [i64; n],

        // String (S) -> Vec<char> (Recommended)
        s: Chars,

        // String (S) -> Vec<u8> (Faster, for ASCII)
        s_bytes: Bytes,

        // 1-based index input -> 0-based integer (P_i)
        p: [Usize1; n],

        // Grid (H x W character matrix)
        h: usize, w: usize,
        grid: [Chars; h],

        // Graph edges (u_i, v_i) * m rows (1-based -> 0-based)
        edges: [(Usize1, Usize1); m],

        // Tuple vector
        tuples: [(usize, i64); n],
    }

    // Reading variable length input interactively (less common)
    // input! {
    //     k: usize,
    //     v: [usize; k], // Read k, then read k elements
    // }

    // ==========================================
    // 2. Output Patterns
    // ==========================================

    let ans = 42;

    // Basic output with newline
    writeln!(out, "{}", ans).unwrap();

    // Print Vector joined by spaces (e.g., "1 2 3")
    // Requires: use itertools::Itertools;
    let vec = vec![1, 2, 3];
    writeln!(out, "{}", vec.iter().format(" ")).unwrap();

    // Yes/No
    let condition = true;
    writeln!(out, "{}", if condition { "Yes" } else { "No" }).unwrap();

    // Floating point (10 decimal places)
    let val = 3.1415926535;
    writeln!(out, "{:.10}", val).unwrap();

    // Interactive flush (for interactive problems)
    // out.flush().unwrap();

    // ==========================================
    // 3. Itertools Patterns
    // ==========================================

    // Permutations: nPk (e.g., 3P2)
    for p in (0..3).permutations(2) {
        // p is Vec<usize>
        let (first, second) = (p[0], p[1]);
    }

    // Combinations: nCk (e.g., 5C3)
    for c in (0..5).combinations(3) {
        // c is Vec<usize>
    }

    // Cartesian Product (Multi-loop)
    // Equivalent to: for i in 0..h { for j in 0..w { ... } }
    for (i, j) in iproduct!(0..h, 0..w) {
        // Access grid[i][j]
    }

    // Run Length Encoding (Group consecutive elements)
    let data = vec![1, 1, 2, 2, 2, 3];
    for (key, group) in &data.into_iter().group_by(|&x| x) {
        // key: 1, count: group.count() -> 2
    }

    // ==========================================
    // 4. String Operations (Vec<char> mainly)
    // ==========================================

    // Note: In AtCoder, usually treat string as Vec<char> for random access.
    let mut s: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    // 1. Conversion: Vec<char> <-> String
    let s_string: String = s.iter().collect();
    let s_back: Vec<char> = s_string.chars().collect();

    // 2. Char <-> Integer (0-25) map for 'a'-'z'
    // Useful for counting characters or graph nodes
    let c = 'c';
    let idx = (c as u8 - b'a') as usize; // 'a'->0, 'b'->1, ...
    let c_restored = (b'a' + idx as u8) as char;

    // 3. Char <-> Digit (numeric char)
    let num_c = '9';
    if let Some(d) = num_c.to_digit(10) {
        // d is u32 (9)
    }
    // Convert number to char
    let char_from_digit = std::char::from_digit(9, 10).unwrap(); // '9'

    // 4. Basic Manipulation
    s.reverse(); // ['e', 'd', 'c', 'b', 'a']
    s.sort(); // ['a', 'b', 'c', 'd', 'e']
    s.rotate_left(1); // ['b', 'c', 'd', 'e', 'a'] (Cyclic shift)
    s.rotate_right(1); // ['a', 'b', 'c', 'd', 'e']

    // 5. Substring (Slice)
    // Note: Slicing a String directly is dangerous (byte indices).
    // Slicing Vec<char> is safe.
    let sub = &s[1..3]; // ['b', 'c'] (slice)
    let sub_vec = s[1..3].to_vec(); // New vector

    // 6. Check properties
    let is_lower = 'a'.is_ascii_lowercase();
    let upper = 'a'.to_ascii_uppercase(); // 'A'

    // ==========================================
    // 5. Data Structures & Algorithms
    // ==========================================

    // Priority Queue (Max Heap)
    let mut heap = BinaryHeap::new();
    heap.push(10);

    // Priority Queue (Min Heap)
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(10)); // Wrap with Reverse

    // Binary Search (Standard library)
    let sorted_vec = vec![1, 3, 5, 7, 9];
    let x = 5;
    match sorted_vec.binary_search(&x) {
        Ok(index) => writeln!(out, "Found at {}", index).unwrap(),
        Err(insert_pos) => writeln!(out, "Not found, insert at {}", insert_pos).unwrap(),
    }

    // Lower Bound (using partition_point)
    // Find the first index where x >= 5
    let idx = sorted_vec.partition_point(|&x| x < 5); // idx = 2

    // Deque (Double-ended queue)
    let mut deque = VecDeque::new();
    deque.push_front(1);
    deque.push_back(2);
}

fn interactive_example() {
    loop {
        // TODO: Implement interaction logic here
    }
}

// {
//     //              0  1  2  3  4  5  6
//     let data = vec![1, 2, 4, 4, 4, 7, 9];

//     // 存在判定
//     let x = 4;
//     let found = data.binary_search(&x).is_ok();
//     let y = 5;
//     let not_found = data.binary_search(&y).is_err();
//     let idx_to_insert = data.binary_search(&y).err().unwrap(); // 5 を挿入すべきインデックス
//     assert_eq!(idx_to_insert, 5);

//     // 4 以上の最初の要素のインデックス (Lower Bound)
//     // "x < 4" is true for [1, 2]. The partition point is index 2 (value 4).
//     let idx_lower = data.partition_point(|&x| x < 4);

//     // 4 より大きい最初の要素のインデックス (Upper Bound)
//     // "x <= 4" is true for [1, 2, 4, 4, 4]. The partition point is index 5 (value 7).
//     let idx_upper = data.partition_point(|&x| x <= 4);
// }

// {
//     //              0   1  2  3  4
//     let data = vec![10, 8, 6, 4, 2];

//     // Find the first element that is <= 5.
//     // The partition point divides the slice into:
//     // Left (true): elements > 5
//     // Right (false): elements <= 5
//     let idx = data.partition_point(|&x| x > 5);

//     println!("Index: {}", idx); // Output: 3 (value is 4)
//     if idx < data.len() {
//         println!("Value: {}", data[idx]);
//     }
// }
