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
        k: usize,
        m: usize,
        CV: [(Usize1, i64); n],
    }
    let mut max_values = vec![-INF_I64; n];
    let mut rem_values = vec![];
    for i in 0..n {
        let (c, v) = CV[i];
        if max_values[c] < v {
            if max_values[c] != INF_I64 {
                rem_values.push(max_values[c]);
            }
            max_values[c] = v;
        }
        else {
            rem_values.push(v);
        }
    }
    max_values.sort_unstable();
    max_values.reverse();
    let mut ans = 0;
    ans += max_values.iter().take(m).sum::<i64>();
    for val in max_values.iter().skip(m) {
        rem_values.push(*val);
    }
    rem_values.sort_unstable();
    rem_values.reverse();
    ans += rem_values.iter().take(k - m).sum::<i64>();
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
