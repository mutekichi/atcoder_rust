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
        n: usize,
        A: [u64; n],
    }
    let forward = forward(&A);
    let backward = backward(&A);
    let mut ans = 0;
    md!(forward.iter().join(" "));
    md!(backward.iter().join(" "));
    for i in 0..n {
        ans = max(ans, min(forward[i], backward[i]));
    }
    println!("{}", ans);
}
fn backward(A: &Vec<u64>) -> Vec<u64> {
    let mut val = 0;
    let mut ret = vec![0; A.len()];
    for i in (0..A.len()).rev() {
        let a = A[i];
        if a > val {
            val += 1;
        }
        else {
            val = a
        }
        ret[i] = val;
    }
    return ret;
}

fn forward(A: &Vec<u64>) -> Vec<u64> {
    let mut val = 0;
    let mut ret = vec![];
    for i in 0..A.len() {
        let a = A[i];
        if a > val {
            val += 1;
        }
        else {
            val = a;
        }
        ret.push(val);
    }
    return ret;
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
