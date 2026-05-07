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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            A: [i64; n],
        }
        let mut counter = BTreeMap::new();
        for a in A {
            *counter.entry(a).or_insert(0) += 1usize;
        }
        let mut prev = 0;
        let mut vec = vec![];
        let mut contains_zero = false;
        let mut multiple_zeros = true;
        for (&val, &count) in counter.iter() {
            if val == 0 {
                contains_zero = true;
                multiple_zeros = count > 1;
            } else {
                vec.push((val - prev, count));
                prev = val;
            }
        }
        if contains_zero {
            if multiple_zeros {
                println!("Alice");
                continue;
            }
        }
        if vec.len() == 0 {
            println!("Alice");
            continue;
        }
        for i in 0..vec.len() {
            md!(vec[i].0, vec[i].1);
        }
        let (val, count) = vec[vec.len() - 1];
        let mut last_wins_alice = if val > 1 { true } else { false };
        for &(val, count) in vec.iter().take(vec.len() - 1) {
            if val == 1 && count == 1 {
                continue;
            } else if count == 1 {
                last_wins_alice = true;
                break;
            } else if val == 1 {
                last_wins_alice = false;
                break;
            } else {
                last_wins_alice = true;
                break;
            }
        }
        if contains_zero && !multiple_zeros {
            last_wins_alice = !last_wins_alice;
        }
        if last_wins_alice {
            println!("Alice");
        } else {
            println!("Bob");
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
