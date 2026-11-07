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
        n: usize, d: usize,
        W: [i128; n],
    }
    let naive = {
        let mut weights = vec![0; d];
        let W = W.clone().into_iter().collect::<Vec<_>>();
        for w in W.iter().rev() {
            let mut most_light = 0;
            for i in 0..d {
                if weights[i] < weights[most_light] {
                    most_light = i;
                }
            }
            weights[most_light] += w;
        }
        weights.iter().map(|w| w * w).sum::<i128>()
    };
    md!(naive);
    let sum = W.iter().sum::<i128>();

    let mut ans = naive;
    let mut sums = vec![0; d];
    let W = W.clone().into_iter().collect::<Vec<_>>();

    dfs(
        &W,
        n,
        d,
        0,
        0,
        &mut ans,
        W.iter().map(|v| v * v).sum::<i128>(),
        &mut sums,
    );

    println!(
        "{}",
        (ans * d as i128 - sum * sum) as f64 / d as f64 / d as f64
    );
}

fn dfs(
    W: &Vec<i128>,
    n: usize,
    d: usize,
    idx: usize,
    occupied: usize,
    // vec: &mut Vec<i128>,
    ans: &mut i128,
    total: i128,
    sums: &mut Vec<i128>,
) {
    md!(idx, total);
    if idx == n {
        *ans = min(*ans, total);
        return;
    }
    let w = W[idx];
    for i in 0..min(occupied + 1, d) {
        let next_total = total + sums[i] * w * 2;
        // if next_total < *ans {
        if true {
            // update
            sums[i] += w;
            dfs(
                W,
                n,
                d,
                idx + 1,
                if i == occupied { i + 1 } else { occupied },
                // vec,
                ans,
                next_total,
                sums,
            );
            // roll back
            sums[i] -= w;
        }
    }
}

const INF_I64: i128 = 1 << 60;
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
