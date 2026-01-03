#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

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
        // INPUT
        t: usize,
    }

    // 辺の通過回数について上限回数を決めるような BFS は，次数の大きすぎる
    // 頂点があるときに，Θ(N^2) 回のループとなってしまう可能性があるので注意
    
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            uv: [(Usize1, Usize1); n - 1],
        }
        let mut graph = vec![vec![]; n];
        for (i, (u, v)) in uv.iter().enumerate() {
            graph[*u].push((*v, i * 2));
            graph[*v].push((*u, i * 2 + 1));
        }
        let mut edge_min_steps = vec![vec![INF_USIZE; k]; 2*(n-1)];
        let mut ans = vec![INF_USIZE; n];
        let mut node_times = vec![vec![0; k]; n];

        let mut q = VecDeque::new();
        q.push_back((0, 0, 0, INF_USIZE)); // (node, tern, step, from)
        while let Some(f) = q.pop_front() {
            let (node, tern, step, from) = f;
            node_times[node][step] += 1;
            if node_times[node][step] > 2 {
                continue;
            }
            md!(node, tern, step);
            let next_tern = if step == k - 1 { tern + 1 } else { tern };
            let next_step = if step == k - 1 { 0 } else { step + 1 };
            if step == 0 {
                chmin!(ans[node], tern);
            }
            for &(next_node, i_edge) in &graph[node] {
                if step != 0 && from == next_node {
                    continue;
                }
                if edge_min_steps[i_edge][step] == INF_USIZE {
                    edge_min_steps[i_edge][step] = tern;
                    q.push_back((next_node, next_tern, next_step, node));
                }
            }
        }
        let ans_str = ans[1..n].iter().map(
            |x| {
                if *x == INF_USIZE { "-1".to_string() } else { x.to_string() }
            }
        ).collect::<Vec<_>>().join(" ");
        md!("==");
        wl!(ans_str);
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
