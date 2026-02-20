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
use std::mem;
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
        n: usize,
        q: usize,
        XY: [(i64, i64); n],
        AB: [(Usize1, Usize1); q],
    }
    let mut points = (0..n).collect::<Vec<_>>();
    points.sort_unstable_by(|a, b| argcmp(XY[*a], XY[*b]));

    let mut counts = vec![];
    let mut count = 0;
    let mut orig_to_counts_index = vec![INF_USIZE; n];

    for i in 0..points.len() {
        if i == 0 {
            count = 1;
        } else if are_same_dir(XY[points[i]], XY[points[i - 1]]) {
            count += 1;
        } else {
            counts.push(count);
            count = 1;
        }
        orig_to_counts_index[points[i]] = counts.len();
    }
    counts.push(count);

    let mut accum_counts = vec![0];
    for i in 0..counts.len() {
        accum_counts.push(accum_counts[i] + counts[i]);
    }

    for (a, b) in AB {
        let idx_a = orig_to_counts_index[a];
        let idx_b = orig_to_counts_index[b];
        md!(idx_a, idx_b);

        let ans = if idx_a < idx_b {
            n + counts[idx_b] + counts[idx_a] - (accum_counts[idx_b + 1] - accum_counts[idx_a])
        } else {
            accum_counts[idx_a + 1] - accum_counts[idx_b]
        };
        println!("{}", ans);
    }

    md!(counts.iter().join(" "));
    md!(orig_to_counts_index.iter().join(" "));
}
// https://ngtkana.hatenablog.com/entry/2021/11/13/202103
fn argcmp(
    (x0, y0): (i64, i64),
    (x1, y1): (i64, i64),
) -> Ordering {
    ((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}

fn are_same_dir(
    (x0, y0): (i64, i64),
    (x1, y1): (i64, i64),
) -> bool {
    if x0 == 0 {
        x1 == 0 && !((y0 > 0) ^ (y1 > 0))
    } else if y0 == 0 {
        y1 == 0 && !((x0 > 0) ^ (x1 > 0))
    } else {
        x0 * y1 == x1 * y0
    }
}
// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
