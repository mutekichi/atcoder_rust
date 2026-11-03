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
        h: usize, w: usize, n: usize, q: usize,
        RC: [(usize, usize); n],
        TXY: [(usize, usize, usize); q],
    }
    let mut ans = vec![0; q];
    let mut row_data = vec![];
    let mut col_data = vec![];

    for i in 0..n {
        let (r, c) = RC[i];
        row_data.push((r, 1, 0));
        col_data.push((c, 1, 0));
    }

    for i in 0..q {
        let (t, x, y) = TXY[i];
        if t == 1 {
            row_data.push((x, 0, i));
            row_data.push((y, 2, i));
        }
        else {
            col_data.push((x, 0, i));
            col_data.push((y, 2, i));
        }
    }
    row_data.sort_unstable();
    col_data.sort_unstable();
    let mut row_cnt = 0i64;
    for i in 0..row_data.len() {
        let (x, c, j) = row_data[i];
        if c == 0 {
            ans[j] -= row_cnt;
        }
        else if c == 1 {
            row_cnt += 1;
        }
        else {
            ans[j] += row_cnt;
        }
    }
    let _ = row_cnt;
    let mut col_cnt = 0i64;
    for i in 0..col_data.len() {
        let (x, c, j) = col_data[i];
        if c == 0 {
            ans[j] -= col_cnt;
        }
        else if c == 1 {
            col_cnt += 1;
        }
        else {
            ans[j] += col_cnt;
        }
    }
    for i in 0..q {
        println!("{}", ans[i]);
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
