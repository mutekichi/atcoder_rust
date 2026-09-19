#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use memoise::memoise;
use num_integer::gcd;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::btree_map::Entry;
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            AB: [(i64, i64); n],
        }
        let mut A = AB.iter().map(|(a, b)| *a).collect::<Vec<_>>();
        let mut diffs =
            AB.iter().map(|(a, b)| *a - *b).collect::<Vec<_>>();
        let mut cheapest = AB.iter().map(|(a, b)| *a).min().unwrap();
        let mut coupons = n;
        let mut sum = A.iter().sum::<i64>();

        let mut ans = INF_I64;
        diffs.sort_unstable();
        diffs.reverse();
        // buy cheapest with coupon
        ans = min(ans, sum - diffs.iter().take(n / 2).sum::<i64>());
        md!(ans);
        // buy cheapest normally
        let mut cheapest_idx = INF_USIZE;
        for i in 0..n {
            if AB[i].0 == cheapest {
                cheapest_idx = i;
                break;
            }
        }
        let mut diffs = vec![];
        for i in 0..n {
            let (a, b) = AB[i];
            if i != cheapest_idx {
                diffs.push(a - b);
            }
        }
        diffs.sort_unstable();
        diffs.reverse();
        if n % 2 == 0 {
            let mut sum = sum - diffs.iter().take(n / 2).sum::<i64>();
            for diff in diffs.iter().skip(n / 2).cloned() {
                if diff > cheapest * 2 {
                    sum -= diff - cheapest * 2;
                } else {
                    break;
                }
            }
            ans = min(ans, sum);
        } else {
            let mut sum = sum - diffs.iter().take(n / 2).sum::<i64>();
            md!(sum);
            let mut first = true;
            for diff in diffs.iter().skip(n / 2).cloned() {
                let sub = if first {
                    diff - cheapest
                } else {
                    diff - cheapest * 2
                };
                md!(sub);
                first = false;
                if sub > 0 {
                    sum -= sub;
                } else {
                    break;
                }
                md!(sum);
            }
            ans = min(ans, sum);
        }
        println!("{}", ans);
    }
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[rustfmt::skip]
const DIR8: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
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

trait AsciiExt {
    fn to_idx(self) -> usize;
}

impl AsciiExt for char {
    fn to_idx(self) -> usize {
        (self as u8 - b'a') as usize
    }
}

impl AsciiExt for u8 {
    fn to_idx(self) -> usize {
        (self - b'a') as usize
    }
}

trait UsizeExt {
    fn to_char(self) -> char;
}

impl UsizeExt for usize {
    fn to_char(self) -> char {
        (self as u8 + b'a') as char
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
