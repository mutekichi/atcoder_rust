#![allow(unused_imports)]
#![allow(dead_code)]

use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use itertools::Itertools;
use std::cmp::{Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{Write, BufWriter};

// This function is just a container for snippets.
fn snippets<W: Write>(out: &mut W) {

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
    // 4. Data Structures & Algorithms
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
    let idx = sorted_vec.partition_point(|&x| x < 5);
    
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
