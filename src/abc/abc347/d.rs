#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::arch::x86_64::_popcnt64;
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
        a: usize,
        b: usize,
        c: i64,
    }
    let popcount_c = c.count_ones() as usize;
    let max_popcount = if a + b <= 60 {
        a + b
    } else {
        120 - a - b
    };
    if popcount_c > max_popcount  || a.abs_diff(b) > popcount_c || (a + b - popcount_c) % 2 == 1 {
        println!("-1");
        return;
    }
    let mut cnt_common = (a + b - popcount_c) / 2;
    let mut cnt_a = a - cnt_common;
    let mut cnt_b = b - cnt_common;

    let mut ans_a = 0i64;
    let mut ans_b = 0i64;

    for i in 0..62 {
        if (c >> i) & 1 == 1 {
            if cnt_a > 0 {
                ans_a += 1 << i;
                cnt_a -= 1;
            } else if cnt_b > 0 {
                ans_b += 1 << i;
                cnt_b -= 1;
            }
        } else {
            if cnt_common > 0 {
                ans_a += 1 << i;
                ans_b += 1 << i;
                cnt_common -= 1;
            }
        }
    }
    assert_eq!(ans_a.count_ones() as usize, a);
    assert_eq!(ans_b.count_ones() as usize, b);
    assert_eq!(ans_a ^ ans_b, c);

    println!("{} {}", ans_a, ans_b);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
