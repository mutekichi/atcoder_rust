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
const DIR: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
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
        h: usize, w: usize,
        mut S: [Chars; h],
    }
    let first = S[0][0];
    let mut ok = true;
    for i in 0..h {
        for j in 0..w {
            if S[i][j] != first {
                ok = false;
                break;
            }
        }
    }
    if ok {
        for _ in 0..h {
            for _ in 0..w {
                print!(".");
            }
            println!("");
        }
        return;
    }

    let mut T = vec![vec!['r'; w]; h];

    for _ in 0..2 {
        for i in 0..h {
            for j in 0..w {
                if S[i][j] == '.' {
                    let mut ok = false;
                    for (ni, nj) in get_next_positions(h, w, i, j, &DIR) {
                        if S[ni][nj] == '#' {
                            ok = true;
                            break;
                        }
                    }
                    if ok {
                        T[i][j] = '#';
                    } else {
                        T[i][j] = '.';
                    }
                } else {
                    T[i][j] = '.';
                }
            }
        }
        swap(&mut S, &mut T);
    }

    let mut queue = VecDeque::new();
    let mut seen = vec![vec![false; w]; h];
    let mut ans = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if S[i][j] == '#' {
                queue.push_back((i, j, true));
                seen[i][j] = true;
            }
        }
    }
    while let Some((i, j, parity)) = queue.pop_front() {
        md!(i, j);
        ans[i][j] = parity;
        for (ni, nj) in get_next_positions(h, w, i, j, &DIR) {
            if !seen[ni][nj] {
                queue.push_back((ni, nj, !parity));
                seen[ni][nj] = true;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", if ans[i][j] { '#' } else { '.' });
        }
        println!();
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
