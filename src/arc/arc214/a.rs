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
        S: [Chars; n]
    }
    let mut vals = vec![None; 2 * n - 1];
    for i in 0..n {
        for j in 0..n {
            if S[i][j] == '?' {
                continue;
            } else {
                let digit = S[i][j].to_digit(10).unwrap();
                if vals[i + j].is_none() {
                    vals[i + j] = Some(digit);
                } else {
                    if vals[i + j].unwrap() != digit {
                        println!("{}", -1);
                        return;
                    }
                }
            }
        }
    }
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if vals[i + j].is_some() {
                ans[i][j] = vals[i + j].unwrap();
            }
        }
    }
    for i in 0..n {
        println!("{}", ans[i].iter().join(""));
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
