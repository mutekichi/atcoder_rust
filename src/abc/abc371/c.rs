#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use memoise::memoise;
use num_integer::gcd;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::btree_map::Entry;
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        mg: usize,
        UVg: [(Usize1, Usize1); mg],
        mh: usize,
        UVh: [(Usize1, Usize1); mh],
    }
    let mut costs = vec![vec![INF_I64; n]; n];
    for i in 0..n - 1 {
        input! {
            A: [i64; n - i - 1],
        }
        for k in i + 1..n {
            costs[i][k] = A[k - i - 1];
            costs[k][i] = A[k - i - 1];
        }
    }
    let mut graph = vec![vec![vec![false; n]; n]; 2];
    for (u, v) in UVh {
        graph[0][u][v] = true;
        graph[0][v][u] = true;
    }
    for (u, v) in UVg {
        graph[1][u][v] = true;
        graph[1][v][u] = true;
    }

    let mut ans = INF_I64;
    for v in (0..n).permutations(n) {
        let mut cost = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if graph[0][i][j] != graph[1][v[i]][v[j]] {
                    cost += costs[i][j];
                }
            }
        }
        ans = ans.min(cost);
    }
    println!("{}", ans);
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[rustfmt::skip]
const DIR8: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
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

trait AsciiExt {
    fn to_idx(self) -> usize;
}

impl AsciiExt for char {
    fn to_idx(self) -> usize {
        (self as u8 - b'a') as usize
    }
}

impl AsciiExt for u8 {
    fn to_idx(self) -> usize {
        (self - b'a') as usize
    }
}

trait UsizeExt {
    fn to_char(self) -> char;
}

impl UsizeExt for usize {
    fn to_char(self) -> char {
        (self as u8 + b'a') as char
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
