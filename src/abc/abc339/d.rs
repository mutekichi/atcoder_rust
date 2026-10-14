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
        S: [Chars; n],
    }
    let mut p = vec![];
    for i in 0..n {
        for j in 0..n {
            if S[i][j] == 'P' {
                p.push((i, j));
            }
        }
    }
    p.sort();
    let mut seen = vec![vec![vec![vec![false; n]; n]; n]; n];
    let mut queue = VecDeque::new();
    queue.push_back((p, 0));
    while let Some((p, count)) = queue.pop_front() {
        if p[0] == p[1] {
            md!(p[0].0, p[0].1);
            println!("{}", count);
            return;
        }
        if seen[p[0].0][p[0].1][p[1].0][p[1].1] {
            continue;
        }
        seen[p[0].0][p[0].1][p[1].0][p[1].1] = true;
        for &dir in &DIR4 {
            let mut next_p = vec![];
            for &p in &p {
                let (mut y, mut x) = p;
                if dir.0 == 0 {
                    let nx = x as isize + dir.1;
                    if nx == -1
                        || nx == n as isize
                        || S[y][nx as usize] == '#'
                    {
                    } else {
                        x = nx as usize;
                    }
                } else {
                    let ny = y as isize + dir.0;
                    if ny == -1
                        || ny == n as isize
                        || S[ny as usize][x] == '#'
                    {
                    } else {
                        y = ny as usize;
                    }
                }
                next_p.push((y, x));
            }
            next_p.sort();
            queue.push_back((next_p, count + 1usize));
        }
    }
    println!("{}", -1);
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
