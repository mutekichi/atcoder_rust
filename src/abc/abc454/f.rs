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
            n: usize, m: i64, A: [i64; n],
        }
        let mut diff_set = BTreeSet::new();
        let mut array = vec![0];
        for i in 0..n / 2 {
            array.push((A[i] + m - A[n - i - 1]) % m);
        }
        array.push(0);
        let mut idx = 0;
        for w in array.windows(2) {
            diff_set.insert(((w[1] + m - w[0]) % m, idx));
            idx += 1;
        }

        let mut ans = 0;
        while !diff_set.is_empty() {
            let min_val = diff_set.first().unwrap().0;
            let max_val = diff_set.last().unwrap().0;
            if diff_set.len() > 1 {
                if min_val == m - max_val {
                    ans += min_val;
                    diff_set.pop_first();
                    diff_set.pop_last();
                } else if min_val < m - max_val {
                    ans += min_val;
                    diff_set.pop_first();
                    diff_set.pop_last();
                    diff_set.insert(((max_val + min_val) % m, idx));
                    idx += 1;
                } else {
                    ans += m - max_val;
                    diff_set.pop_first();
                    diff_set.pop_last();
                    diff_set.insert(((min_val + m - (m - max_val)) % m, idx));
                    idx += 1;
                }
            } else {
                ans += min(min_val, m - max_val);
                diff_set.pop_last();
            }
        }
        println!("{ans}");
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
