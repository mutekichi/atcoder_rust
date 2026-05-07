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
        n: usize, A: [i64; n],
    }
    let mut memo = vec![INF_I64; n];
    let ans = f(0, n, &mut memo, &A);
    println!("{}", ans);
}

fn f(
    idx: usize,
    n: usize,
    memo: &mut Vec<i64>,
    A: &Vec<i64>,
) -> i64 {
    md!("S", idx);
    let ret = if idx == n {
        0
    } else if idx == n - 1 {
        A[idx]
    } else {
        let cand1 = if memo[idx + 1] != INF_I64 {
            memo[idx + 1]
        } else {
            f(idx + 1, n, memo, A)
        };

        let cand2 = if idx + 2 == n {
            0
        } else if memo[idx + 2] != INF_I64 {
            memo[idx + 2]
        } else {
            f(idx + 2, n, memo, A)
        };

        if idx == 3 {
            md!(cand1, A[idx], cand2);
        }

        let ret = max(cand1 + A[idx], cand2);
        ret
    };
    memo[idx] = ret;
    md!("G", idx, ret);
    return ret;
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
