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
        n: usize, q: usize,
        XY: [(i128, i128); n],
        UV: [(Usize1, Usize1); q],
    }

    let mut accum_ps = vec![(0, 0, 0)];
    let x0 = XY[0].0;
    let y0 = XY[0].1;
    for i in 0..n - 2 {
        let p = calc_p(XY[0], XY[i + 1], XY[i + 2]);
        md!(p.0, p.1, p.2);
        accum_ps.push(kajuu_p(accum_ps[i], p));
    }
    let total_p = accum_ps[n - 2];

    for (u, v) in UV {
        let mut ans = (0, 0, 0);
        if u == 0 {
            ans = accum_ps[v - 1];
        } else if v == 0 {
            let sub = accum_ps[u - 1];
            ans = kajuu_sub_p(total_p, sub);
        } else if u < v {
            ans = kajuu_sub_p(accum_ps[v - 1], accum_ps[u - 1]);
            ans = kajuu_sub_p(ans, calc_p(XY[0], XY[u], XY[v]));
        } else if v < u {
            let (u, v) = (v, u);
            ans = kajuu_sub_p(accum_ps[v - 1], accum_ps[u - 1]);
            ans = kajuu_sub_p(ans, calc_p(XY[0], XY[u], XY[v]));
            ans = kajuu_sub_p(total_p, ans);
        } else {
            // ng
        }
        println!(
            "{} {}",
            ans.0 as f64 / 3. / ans.2 as f64,
            ans.1 as f64 / 3. / ans.2 as f64
        );
    }
}

// p: (center x * 3 * area, center y * 3 * area, area)

fn calc_p(
    t0: (i128, i128),
    t1: (i128, i128),
    t2: (i128, i128),
) -> (i128, i128, i128) {
    let s = area(t0, t1, t2);
    ((t0.0 + t1.0 + t2.0) * s, (t0.1 + t1.1 + t2.1) * s, s)
}

fn area(
    t0: (i128, i128),
    t1: (i128, i128),
    t2: (i128, i128),
) -> i128 {
    let x1 = t1.0 - t0.0;
    let y1 = t1.1 - t0.1;
    let x2 = t2.0 - t0.0;
    let y2 = t2.1 - t0.1;
    return x1 * y2 - x2 * y1;
}

fn kajuu_p(
    p1: (i128, i128, i128),
    p2: (i128, i128, i128),
) -> (i128, i128, i128) {
    let (x1, y1, s1) = p1;
    let (x2, y2, s2) = p2;

    ((x1 + x2), (y1 + y2), s1 + s2)
}

fn kajuu_sub_p(
    p1: (i128, i128, i128),
    p2: (i128, i128, i128),
) -> (i128, i128, i128) {
    let (x1, y1, s1) = p1;
    let (x2, y2, s2) = p2;
    ((x1 - x2), (y1 - y2), s1 - s2)
}

fn kajuu(
    x1: f64,
    y1: f64,
    s1: f64,
    x2: f64,
    y2: f64,
    s2: f64,
) -> (f64, f64) {
    (
        (x1 * s2 + x2 * s1) / (s1 + s2),
        (y1 * s2 + y2 * s1) / (s1 + s2),
    )
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[rustfmt::skip]
const DIR8: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
const C998244353: i128 = 998244353;
const C1000000007: i128 = 1000000007;

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
