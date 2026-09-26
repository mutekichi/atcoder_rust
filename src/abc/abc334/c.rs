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
        n: usize, k: usize, A: [Usize1; k],
    }
    let mut counts = vec![2; n];
    for a in A {
        counts[a] -= 1;
    }
    let mut socks = vec![];
    for i in 0..n {
        for _ in 0..counts[i] {
            socks.push(i);
        }
    }
    let mut ans = 0;
    if (2 * n - k) % 2 == 0 {
        for i in 0..(2 * n - k) / 2 {
            ans += socks[i * 2 + 1] - socks[i * 2];
        }
    } else {
        let mut intervals_before = vec![0];
        let mut intervals_after = vec![0];
        ans = INF_USIZE;
        for i in 0..(2 * n - k) / 2 {
            intervals_before.push(
                socks[i * 2 + 1] - socks[i * 2] + intervals_before[i],
            );
            intervals_after.push(
                socks[2 * n - k - 1 - i * 2]
                    - socks[2 * n - k - 1 - i * 2 - 1]
                    + intervals_after[i],
            );
        }
        intervals_after.reverse();
        for i in 0..intervals_after.len() {
            ans = ans.min(intervals_before[i] + intervals_after[i]);
        }
    }
    println!("{}", ans);
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
