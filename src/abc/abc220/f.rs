#![cfg_attr(rustfmt, fn_params_layout = "Vertical")]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// Constants
const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const MOD: i64 = 998244353;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

fn dp_size(
    v: usize,
    from: usize,
    tree: &Vec<Vec<usize>>,
    sizes: &mut Vec<usize>,
    depths: &mut Vec<usize>,
    depth: usize,
) -> usize {
    let mut size = 1;
    for next in &tree[v] {
        if *next == from {
            continue;
        }
        size += dp_size(*next, v, tree, sizes, depths, depth + 1);
    }
    depths[v] = depth;
    sizes[v] = size;
    size
}

fn dp_1(
    v: usize,
    from: usize,
    tree: &Vec<Vec<usize>>,
    sizes: &Vec<usize>,
    size_sum_root_0: &mut Vec<usize>,
) -> usize {
    let mut size_sum = 0;
    for next in &tree[v] {
        if *next == from {
            continue;
        }
        let sub_size_sum = dp_1(*next, v, tree, sizes, size_sum_root_0);
        size_sum += sub_size_sum + sizes[*next];
    }
    size_sum_root_0[v] = size_sum;
    size_sum
}

fn dp_2(
    v: usize,
    from: usize,
    tree: &Vec<Vec<usize>>,
    sizes: &Vec<usize>,
    ans: &mut Vec<usize>,
    n: usize,
) {
    let size = sizes[v];
    ans[v] = ans[from] + n - 2 * size;
    for next in &tree[v] {
        if *next == from {
            continue;
        }
        dp_2(*next, v, tree, sizes, ans, n);
    }
}

// Logic goes here
#[allow(unused_macros)]
#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    
    let mut tree = vec![Vec::new(); n];
    for (u, v) in uv {
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut sizes = vec![INF_USIZE; n];
    let mut depths = vec![INF_USIZE; n];
    let mut size_sum_root_0 = vec![INF_USIZE; n];

    dp_size(0, INF_USIZE, &tree, &mut sizes, &mut depths, 0);
    dp_1(0, INF_USIZE, &tree, &sizes, &mut size_sum_root_0);

    for (i, d) in size_sum_root_0.iter().enumerate() {
        md!(i, d);
    }
    
    let mut answers = vec![INF_USIZE; n];
    answers[0] = size_sum_root_0[0];
    
    for next in &tree[0] {
        dp_2(*next, 0, &tree, &sizes, &mut answers, n);
    }

    for ans in answers {
        wl!(ans);
    }
}

// --- Macros ---

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

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint without newline)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mep {
    ($x:expr) => { eprint!("{}", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mep {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint with space)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mepw { // stands for my_eprint_whitespace
    ($x:expr) => { eprint!("{} ", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mepw {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

// Utility functions

// Utility functions
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
