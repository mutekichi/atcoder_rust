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
        n: usize,
        total_w: i64,
        WV: [(i64, i64); n],
    }
    let mut accums = vec![(0, 0)];
    for i in 0..n {
        let (w, v) = WV[i];
        accums.push((accums[i].0 + w, accums[i].1 + v));
    }
    println!("{}", solve(n, &WV, total_w, 0, n - 1, &accums));
}

fn solve(
    n: usize,
    WV: &Vec<(i64, i64)>,
    rem_w: i64,
    accum_v: i64,
    idx: usize,
    accums: &Vec<(i64, i64)>,
) -> i64 {
    let (w, v) = WV[idx];
    if idx == 0 {
        return accum_v + if rem_w >= w { v } else { 0 };
    }
    if rem_w < w {
        return solve(n, WV, rem_w, accum_v, idx - 1, accums);
    } else {
        return max(
            solve(n, WV, rem_w - w, accum_v + v, idx - 1, accums),
            accum_v + accums[idx].1,
        );
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
