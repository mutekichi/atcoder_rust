#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};

// External crates (Available in AtCoder)
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::input;
use itertools::Itertools;

// Constants
const INF_I: i64 = 1 << 60;
const INF_U: u64 = 1 << 60 as u64;
const MOD: i64 = 998244353;
const MOD2: i64 = 1_000_000_007;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

// Logic goes here
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }
    input! {
        n:usize,
        m:usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = INF_U;
    for i in 0..(1<<n) as usize {
        let mut count = 0;
        for &(u, v) in &edges {
            if (i >> u) & 1 == (i >> v) & 1  {
                count += 1;
            }
        }
        chmin!(ans, count);
        md!(ans);
    }

    wl!(ans);
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