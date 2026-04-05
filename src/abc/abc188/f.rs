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
        x: i64, y: i64,
    }
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((x, 0usize, INF_USIZE));
    // 0: -1, 1: +1, 2: *2
    while let Some((val, count, prev)) = queue.pop_front() {
        if val == y {
            println!("{}", count);
        } else {
            
            if prev == 0 {
                if val > 0 && !seen.contains(&(2 * (val - 1))) {
                    seen.insert(2 * (val - 1));
                    queue.push_back((val - 1, count + 1, 0));
                }
                if !seen.contains(&(2 * val)) {
                    seen.insert(2 * val);
                    queue.push_back((val * 2, count + 1, 2));
                }
            } else if prev == 1 {
                let next_vals = vec![(1, val + 1), (2, val * 2)];
                for (prev, next_val) in next_vals {
                    if !seen.contains(&next_val) {
                        queue.push_back((next_val, count + 1, prev));
                    }
                }
            } else {
                let mut next_vals = vec![(1, val + 1), (2, val * 2)];
                if val > 0 {
                    next_vals.push((0, val - 1));
                }
                for (prev, next_val) in next_vals {
                    if !seen.contains(&next_val) {
                        queue.push_back((next_val, count + 1, prev));
                    }
                }
            }
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
