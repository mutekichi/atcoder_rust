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
        n: usize,
        s: Usize1,
        l: i64,
        A: [i64; n -1],
    }
    let mut ans = 0;
    let mut befs = A.iter().cloned().take(s).rev().collect::<Vec<_>>();
    let mut afts = A.iter().cloned().skip(s).collect::<Vec<_>>();

    solve(&befs, &afts, l, &mut ans);
    // bef -> aft
    swap(&mut befs, &mut afts);
    solve(&befs, &afts, l, &mut ans);
    println!("{}", ans);
}
fn solve(
    befs: &Vec<i64>,
    afts: &Vec<i64>,
    l: i64,
    ans: &mut usize,
) {
    let mut accums_bef = BTreeSet::new();
    let mut accums_aft = BTreeSet::new();

    let mut sum_bef = 0;
    accums_bef.insert((sum_bef, 0));
    for i in 0..befs.len() {
        sum_bef += befs[i];
        accums_bef.insert((sum_bef, i + 1));
    }
    let mut sum_aft = 0;
    accums_aft.insert((sum_aft, 0));
    for i in 0..afts.len() {
        sum_aft += afts[i];
        accums_aft.insert((sum_aft, i + 1));
    }

    let cand = accums_bef.range(..(l + 1, 0)).next_back().unwrap().1;
    md!(cand);
    *ans = max(*ans, cand + 1);
    let cand = accums_aft.range(..(l + 1, 0)).next_back().unwrap().1;
    md!(cand);
    *ans = max(*ans, cand + 1);

    md!(befs.iter().join(" "));
    md!(afts.iter().join(" "));
    let mut sum_bef = 0;
    for i in 0..befs.len() {
        sum_bef += befs[i];
        md!(sum_bef);
        if l > sum_bef * 2 {
            let cand = accums_aft
                .range(..(l - sum_bef * 2 + 1, 0))
                .next_back()
                .unwrap()
                .1;
            md!(i, cand);
            *ans = max(*ans, i + cand + 2);
        }
    }
    let mut sum_aft = 0;
    for i in 0..afts.len() {
        sum_aft += afts[i];
        if l > sum_aft * 2 {
            let cand = accums_bef
                .range(..(l - sum_aft * 2 + 1, 0))
                .next_back()
                .unwrap()
                .1;
            md!(i, cand);
            *ans = max(*ans, i + cand + 2);
        }
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
