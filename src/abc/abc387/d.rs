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
        S: [Chars; h],
    }
    let mut sy = 0;
    let mut sx = 0;
    for i in 0..h {
        for j in 0..w {
            if S[i][j] == 'S' {
                sy = i;
                sx = j;
            }
        }
    }
    let mut min_steps = vec![vec![vec![INF_USIZE; 2]; w]; h];
    let mut queue = VecDeque::new();
    let directions = vec![vec![(0, 1), (0, -1)], vec![(1, 0), (-1, 0)]];
    queue.push_back((sy, sx, 0, 1));
    min_steps[sy][sx][1] = 0;
    queue.push_back((sy, sx, 0, 0));
    min_steps[sy][sx][0] = 0;
    while let Some((i, j, steps, dir)) = queue.pop_front() {
        md!(i, j, steps, dir);
        if S[i][j] == 'G' {
            println!("{}", steps);
            return;
        }
        for (ni, nj) in get_next_positions(h, w, i, j, &directions[dir]) {
            if S[ni][nj] == '#' {
                continue;
            }
            let next_dir = 1 - dir;
            if min_steps[ni][nj][next_dir] == INF_USIZE {
                md!(ni, nj);
                min_steps[ni][nj][next_dir] = steps + 1;
                queue.push_back((ni, nj, steps + 1, next_dir));
            }
        }
    }
    println!("-1");
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
