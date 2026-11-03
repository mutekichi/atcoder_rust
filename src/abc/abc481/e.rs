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
            mut XY: [(i64, i64); n],
        }
        let mut lefts = vec![];
        let mut rights = vec![];
        let mut all = vec![];
        let mut upper = 0;
        let mut lower = 0;

        for i in 0..n {
            let (x, y) = XY[i];
            if x == 0 {
                if y > 0 {
                    upper += 1;
                } else {
                    lower += 1;
                }
            } else if x < 0 {
                lefts.push((x, y));
                all.push((-x, -y, false));
                upper += 1;
            } else {
                rights.push((x, y));
                all.push((x, y, true));
                lower += 1;
            }
        }
        md!(upper, lower);

        all.sort_by(|&P, &Q| (P.0 * Q.1).cmp(&(P.1 * Q.0)));

        let mut prev = None;
        let mut ans = max(upper, lower);
        let mut accum = 0;
        for (x, y, is_on_right) in all {
            accum += if is_on_right { 1 } else { -1 };
            if prev.is_none() || {
                let (prevx, prevy) = prev.unwrap();
                prevx * y != prevy * x
            } {
                md!("updated");
                upper += accum;
                lower -= accum;
                accum = 0;
            }
            prev = Some((x, y));
            ans = ans.max(max(upper, lower));
            md!(ans, upper, lower);
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
