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
        n: usize,
        c: i64,
        ABC: [(i64, i64, i64); n],
    }
    let mut days_set = BTreeSet::new();
    for &(a, b, c) in &ABC {
        let days = [a - 1, a, a + 1, b - 1, b, b + 1];
        for day in days {
            days_set.insert(day);
        }
    }
    let days_vec = days_set.iter().collect::<Vec<_>>();
    let mut to_day_map = BTreeMap::new();
    for i in 0..days_vec.len() {
        to_day_map.insert(days_vec[i], i);
    }
    let mut accum = vec![0i64; days_vec.len()];
    for (a, b, c) in ABC {
        accum[*to_day_map.get(&a).unwrap()] += c;
        accum[*to_day_map.get(&(b + 1)).unwrap()] -= c;
    }
    for i in 1..accum.len() {
        accum[i] += accum[i - 1];
    }
    let mut ans = 0;
    for i in 0..days_vec.len() - 1 {
        ans += min(c, accum[i]) * (days_vec[i + 1] - days_vec[i]);
    }
    println!("{}", ans);
}
