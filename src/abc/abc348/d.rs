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

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    input! {
        h: usize,
        w: usize,
        A: [Chars; h],
        n: usize,
        RCE: [(Usize1, Usize1, usize); n],
    }
    let (mut sy, mut sx) = (INF_USIZE, INF_USIZE);
    let (mut ty, mut tx) = (INF_USIZE, INF_USIZE);

    let mut status = vec![vec![INF_USIZE; w]; h];
    for i in 0..h {
        for j in 0..w {
            if A[i][j] == '.' {
                status[i][j] = 0;
            } else if A[i][j] == 'S' {
                status[i][j] = 0;
                (sy, sx) = (i, j);
            } else if A[i][j] == 'T' {
                status[i][j] = 0;
                (ty, tx) = (i, j);
            }
        }
    }
    for (r, c, e) in RCE {
        status[r][c] = e;
    }
    let mut max_scores = vec![vec![INF_USIZE; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((sy, sx));
    max_scores[sy][sx] = 0;
    while let Some((i, j)) = queue.pop_back() {
        let mut score = max_scores[i][j];
        if status[i][j] != INF_USIZE && score < status[i][j] {
            score = status[i][j];
            status[i][j] = 0;
        }
        if (i, j) == (ty, tx) {
            println!("Yes");
            return;
        }
        if score == 0 {
            continue;
        }
        md!(i, j, score);
        score -= 1;
        for (ni, nj) in get_next_positions(h, w, i, j, &DIR) {
            if status[ni][nj] == INF_USIZE {
                continue;
            }
            if max_scores[ni][nj] == INF_USIZE || max_scores[ni][nj] < score {
                max_scores[ni][nj] = score;
                queue.push_back((ni, nj));
            }
        }
    }
    println!("No");
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
