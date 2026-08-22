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
        mut A: [i64; n],
    }
    A.push(-INF_I64);
    A.push(INF_I64);
    A.push(0);
    A.sort_unstable();
    let mut idx = INF_USIZE;
    for i in 0..A.len() {
        if A[i] == 0 {
            idx = i;
            break;
        }
    }
    md!(idx);
    let mut ans = 0;
    let mut before = idx - 1;
    let mut after = idx + 1;
    for i in 0..n {
        md!(ans);
        let a = A[idx];
        let b = A[before];
        let c = A[after];
        md!(a, b, c);
        if a - b <= c - a {
            ans += a - b;
            idx = before;
            before = before - 1;
        } else {
            ans += c - a;
            idx = after;
            after = after + 1;
        }
    }
    println!("{}", ans);
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
