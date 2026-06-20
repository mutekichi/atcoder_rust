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
    }
    for _ in 0..n {
        input! {
            a: i64, b: i64,
            x: i64, y: i64,
        }
        let x = x.abs();
        let y = y.abs();

        let ans = {
            let mut ans = 0;
            ans += min(x, y) * min(a, b) * 2;
            let diff = x.abs_diff(y) as i64;
            ans += diff / 2 * (a + b);
            if diff % 2 != 0 {
                if (a < b) == (x < y) {
                    ans += max(a, b);
                } else {
                    ans += min(a, b);
                }
            }
            ans
        };

        let ans2 = {
            let mut ans = 0;
            ans += min(x, y) * min(a, b) * 2;
            let diff = x.abs_diff(y) as i64;
            ans += min(a, b) * (diff / 2) * 4;
            md!(ans);
            if diff % 2 != 0 {
                if (a < b) == (x < y) {
                    ans += min(max(a, b), min(a, b) * 3);
                } else {
                    ans += min(a, b);
                }
            }
            ans
        };

        println!("{}", min(ans, ans2));
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
