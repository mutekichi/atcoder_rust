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
fn testcase() {
    input! {
        n: usize,
        S: Chars,
        X: [i64; n],
        Y: [i64; n - 1],
    }
    let mut dp =  vec![vec![0; 2]; n];
    if S[0] == 'S' {
        dp[0][1] -= X[0];
    }
    else {
        dp[0][0] -= X[0];
    }
    for i in 1..n {
        if S[i] == 'S' {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + Y[i - 1]);
            dp[i][1] = max(dp[i - 1][0], dp[i - 1][1]) - X[i];
        }
        else {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + Y[i - 1]) - X[i];
            dp[i][1] = max(dp[i - 1][0], dp[i - 1][1]);
        }
    }
    println!("{}", max(dp[n - 1][0], dp[n - 1][1]));
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

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
