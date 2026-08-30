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
        S: [Chars; h],
    }
    let mut safes = vec![vec![true; w]; h];
    let mut bomb_rows = vec![false; h];
    let mut bomb_cols = vec![false; w];
    for i in 0..h {
        for j in 0..w {
            if S[i][j] == '#' {
                bomb_rows[i] = true;
                bomb_cols[j] = true;
            }
        }
    }
    for i in 0..h {
        if bomb_rows[i] {
            for j in 0..w {
                safes[i][j] = false;
            }
        }
    }
    for j in 0..w {
        if bomb_cols[j] {
            for i in 0..h {
                safes[i][j] = false;
            }
        }
    }

    let mut seen = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if safes[i][j] {
                queue.push_back((i, j, 0));
                seen[i][j] = true;
            }
        }
    }
    while let Some((i, j, turn)) = queue.pop_front() {
        for (ni, nj) in get_next_positions(h, w, i, j, &DIR4) {
            if !seen[ni][nj] && S[ni][nj] == '.' && turn < k {
                queue.push_back((ni, nj, turn + 1usize));
                seen[ni][nj] = true;
            }
        }
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if seen[i][j] {
                count += 1usize;
            }
        }
    }
    println!("{}", count);
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

/// Returns valid neighbor coordinates within the grid (h x w).
/// Usage:
/// ```
/// for (nh, nw) in get_next_positions(h, w, hh, ww, &DIR) {
///     // process (nh, nw)
/// }
/// ```
fn get_next_positions(
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<(usize, usize)> {
    let mut next_positions = Vec::with_capacity(directions.len());

    for &(di, dj) in directions {
        let next_i = i.wrapping_add_signed(di);
        let next_j = j.wrapping_add_signed(dj);
        if next_i < h && next_j < w {
            next_positions.push((next_i, next_j));
        }
    }
    next_positions
}

// END TEMPLATE INJECTIONS
