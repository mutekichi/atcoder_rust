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
        h: usize, w: usize, k: usize,
        grid: [Chars; h],
    }
    let mut ans = INF_USIZE;
    if w >= k {
        for i in 0..h {
            let mut o_count = 0;
            let mut x_count = 0;
            for j in 0..k {
                let c = grid[i][j];
                if c == 'o' {
                    o_count += 1;
                } else if c == 'x' {
                    x_count += 1;
                }
            }
            if x_count == 0 {
                ans = min(ans, k - o_count);
            }
            for j in k..w {
                let prev = grid[i][j - k];
                if prev == 'o' {
                    o_count -= 1;
                } else if prev == 'x' {
                    x_count += 1;
                }
                let next = grid[i][j];
                if next == 'o' {
                    o_count += 1;
                } else if next == 'x' {
                    x_count -= 1;
                }
                if x_count == 0 {
                    ans = min(ans, k - o_count);
                }
            }
        }
    }
    if h >= k {
        for j in 0..w {
            let mut o_count = 0;
            let mut x_count = 0;
            for i in 0..k {
                let c = grid[i][j];
                if c == 'o' {
                    o_count += 1;
                } else if c == 'x' {
                    x_count += 1;
                }
            }
            if x_count == 0 {
                ans = min(ans, k - o_count);
            }
            for i in k..h {
                let prev = grid[i - k][j];
                if prev == 'o' {
                    o_count -= 1;
                } else if prev == 'x' {
                    x_count += 1;
                }
                let next = grid[i][j];
                if next == 'o' {
                    o_count += 1;
                } else if next == 'x' {
                    x_count -= 1;
                }
                if x_count == 0 {
                    ans = min(ans, k - o_count);
                }
            }
        }
    }
    if ans == INF_USIZE {
        println!("{}", -1);
    } else {
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
