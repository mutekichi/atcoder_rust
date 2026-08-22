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

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        m: usize,
        LDKCAB: [(i64, i64, i64, i64, Usize1, Usize1); m],
    }
    let mut pq = BinaryHeap::new();
    let mut ans = vec![-1i64; n];
    let mut graph = vec![vec![]; n];
    for (l, d, k, c, a, b) in LDKCAB {
        graph[b].push((a, l, d, k, c));
    }
    pq.push((INF_I64, n - 1));
    while let Some((t, v)) = pq.pop() {
        if ans[v] != -1 {
            continue;
        }
        ans[v] = t;
        for &(nv, l, d, k, c) in &graph[v] {
            if let Some(next_time) = get_next_time(l, d, k, c, t) {
                md!(next_time);
                pq.push((next_time, nv));
            }
        }
    }
    for i in 0..n - 1 {
        if ans[i] == -1 {
            println!("Unreachable");
        } else {
            println!("{}", ans[i]);
        }
    }
}

fn get_next_time(
    l: i64,
    d: i64,
    k: i64,
    c: i64,
    t: i64,
) -> Option<i64> {
    if t - c - l <= 0 {
        return None;
    } else {
        let mut x = (t - c - l) / d;
        x = min(x, k - 1);
        return Some(l + x * d);
    }
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIR8: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
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

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
