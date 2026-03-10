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
        mut S: Chars,
        C: [i64; n],
    }
    for i in 0..n {
        if i % 2 == 0 {
            S[i] = if S[i] == '0' { '1' } else { '0' };
        }
    }
    let cost_all_zeros: i64 = (0..n).filter(|&j| S[j] == '1').map(|j| C[j]).sum();
    let mut ans = INF_I64;
    let mut cost_ones_zeros = cost_all_zeros;
    for i in 0..n-1 {
        if S[i] == '1' {
            cost_ones_zeros -= C[i];
        } else {
            cost_ones_zeros += C[i];
        }
        ans = min(ans, cost_ones_zeros);
    }
    let mut cost_zeros_ones = cost_all_zeros;
    for i in (1..n).rev() {
        if S[i] == '1' {
            cost_zeros_ones -= C[i];
        } else {
            cost_zeros_ones += C[i];
        }
        ans = min(ans, cost_zeros_ones);
    }
    println!("{}", ans);
}
