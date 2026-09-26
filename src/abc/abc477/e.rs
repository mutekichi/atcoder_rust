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
        n: usize, q: usize,
        A: [i64; n],
        B: [i64; n],
        ST: [(Usize1, Usize1); q],
    }
    let mut min_dists = vec![INF_I64; n];
    let mut pq = BinaryHeap::new();
    for i in 0..n {
        let b = B[i];
        pq.push(Reverse((b, i)));
    }
    while let Some(Reverse((dist, v))) = pq.pop() {
        if min_dists[v] != INF_I64 {
            continue;
        }
        min_dists[v] = dist;
        let prev = (v + n - 1) % n;
        let next = (v + 1) % n;
        pq.push(Reverse((dist + A[prev], prev)));
        pq.push(Reverse((dist + A[v], next)));
    }
    md!(min_dists.iter().join(" "));

    let mut accum_dists = vec![0];
    for i in 0..2 * n {
        accum_dists.push(accum_dists[i] + A[i % n]);
    }
    md!(accum_dists.iter().join(" "));

    for (s, t) in ST {
        if t == n {
            println!("{}", min_dists[s]);
        } else {
            let mut ans = accum_dists[t] - accum_dists[s];
            ans = ans.min(accum_dists[s + n] - accum_dists[t]);
            ans = ans.min(min_dists[s] + min_dists[t]);
            println!("{}", ans);
        }
    }
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
