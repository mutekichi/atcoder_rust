#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

#[macro_export]
#[cfg(debug_assertions)] // for debug build
macro_rules! md { // stands for my_dbg
    ($($arg:expr),* $(,)?) => {{
        eprint!("[{}:{}] ", file!(), line!());

        let mut _first = true;
        $(
            if !_first {
                eprint!(", ");
            }
            eprint!("{}: {}", stringify!($arg), $arg);
            _first = false;
        )*
        eprintln!();
    }};
}

#[macro_export]
#[cfg(not(debug_assertions))] // for release build
macro_rules! md {
    ($($arg:expr),* $(,)?) => {{
        // do nothing
    }};
}

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        testcase();
    }
}
fn testcase() {
    input! {
        n: usize, m: usize,
        ABX: [(Usize1, Usize1, i64); m],
    }
    if n % 2 == 1 {
        println!("{}", -1);
        return;
    }
    let mut graph = vec![vec![]; n];
    for (a, b, x) in ABX {
        graph[a].push((b, x));
        graph[b].push((a, x));
    }
    let mut queue = VecDeque::new();
    let mut seen = vec![false; n];
    queue.push_back((0, 0));
    seen[0] = true;
    let mut values = vec![0; n];
    while let Some((v, val)) = queue.pop_back() {
        values[v] = val;
        for &(nv, weight) in &graph[v] {
            if !seen[nv] {
                queue.push_back((nv, val ^ weight));
                seen[nv] = true;
            }
        }
    }
    let mut total_parity = 0;
    for i in 1..=n {
        total_parity ^= i;
    }
    let mut graph_parity = 0;
    for i in 0..n {
        graph_parity ^= values[i] as usize;
    }
    println!("{}", total_parity ^ graph_parity)
}

fn _testcase() {
    input! {
        n: usize, m: usize,
        ABX: [(Usize1, Usize1, i64); m],
    }

    let mut parity_counts_total = vec![0; 20];
    for num in 0..=n {
        for i in 0..20 {
            if (num >> i) & 1 == 1 {
                parity_counts_total[i] += 1;
            }
        }
    }
    let mut graph = vec![vec![]; n];
    for (a, b, x) in ABX {
        graph[a].push((b, x));
        graph[b].push((a, x));
    }
    let mut queue = VecDeque::new();
    let mut seen = vec![false; n];
    queue.push_back((0, 0));
    seen[0] = true;
    let mut parity_counts_graph = vec![0; 20];
    while let Some((v, val)) = queue.pop_back() {
        for i in 0..20 {
            if (val >> i) & 1 == 1 {
                parity_counts_graph[i] += 1;
            }
        }
        for &(nv, weight) in &graph[v] {
            if !seen[nv] {
                queue.push_back((nv, val ^ weight));
                seen[nv] = true;
            }
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
