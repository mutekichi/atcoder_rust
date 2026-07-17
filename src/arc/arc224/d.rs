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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize, k: usize,
        }
        let mut ok = false;
        let mut rem = k - 1;
        let mut ans = 0;
        for i in 1..=n {
            let val = ncr(n, i);
            for _ in 0..val {
                ans += keta(rem) * i;
                rem -= 1;
                if rem == 0 {
                    println!("{}", ans);
                    ok = true;
                    break;
                }
            }
            if ok {
                break;
            }
        }
        if !ok {
            println!("{}", -1);
        }
    }
}
fn ncr(
    n: usize,
    m: usize,
) -> usize {
    let mut ret = 1;
    for i in 0..m {
        ret *= n - i;
    }
    for i in 1..=m {
        ret /= i;
    }
    return ret;
}

fn keta(n: usize) -> usize {
    if n < 10 {
        return 1;
    }
    if n < 100 {
        return 2;
    }
    if n < 1000 {
        return 3;
    }
    if n < 10000 {
        return 4;
    }
    if n < 100000 {
        return 5;
    }
    if n < 1000000 {
        return 6;
    }
    return 7;
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
