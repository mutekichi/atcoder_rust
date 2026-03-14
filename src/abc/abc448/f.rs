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
        XY: [(i64, i64); n],
    }
    let mut XYI = XY.clone().into_iter().zip(0..n).collect::<Vec<_>>();
    let size = (2i64 * 10i64.pow(7)) / (n as i64).isqrt();
    XYI.sort_by(|&((x1, y1), _), &((x2, y2), _)| {
        let b1 = x1 / size;
        let b2 = x2 / size;
        if b1 != b2 {
            b1.cmp(&b2)
        } else {
            if b1 % 2 == 0 {
                y1.cmp(&y2)
            } else {
                y2.cmp(&y1)
            }
        }
    });
    let ans = XYI
        .iter()
        .map(|e| e.1 + 1)
        .cycle()
        .skip_while(|e| *e != 1)
        .take(n)
        .collect::<Vec<_>>();
    println!("{}", ans.iter().join(" "));

    let mut sum = 0u64;
    for i in 0..n - 1 {
        sum += manhattan(XY[ans[i] - 1], XY[ans[i + 1] - 1]);
    }
    sum += manhattan(XY[ans[0] - 1], XY[ans[ans.len() - 1] - 1]);
    md!(sum);
}
fn manhattan(
    p1: (i64, i64),
    p2: (i64, i64),
) -> u64 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
