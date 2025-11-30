#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// Constants
const INF: i64 = 1 << 60;
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
        n: usize,
        udlr: [(usize, usize, usize, usize); n],
    }

    let mut grid: Vec<Vec<i128>> = vec![vec![0; 2002]; 2002];

    for &(u, d, l, r) in udlr.iter() {
        grid[u][l] += 1;
        grid[d + 1][r + 1] += 1;
        grid[u][r + 1] -= 1;
        grid[d + 1][l] -= 1;
    }
    
    for i in 0..2002 {
        for j in 1..2002 {
            grid[i][j] += grid[i][j - 1];
        }
    }

    for i in 0..2002 {
        for j in 1..2002 {
            grid[j][i] += grid[j - 1][i];
        }
    }

    let mut covered: i128 = 0;
    
    for (i, j) in iproduct!(0..2002, 0..2002) {
        if grid[i][j] != 0 {
            covered += 1;
        }
        if grid[i][j] != 1 {
            grid[i][j] = 0;
        }
    }
    
    for i in 0..2002 {
        for j in 1..2002 {
            grid[i][j] += grid[i][j - 1];
        }
    }

    for i in 0..2002 {
        for j in 1..2002 {
            grid[j][i] += grid[j - 1][i];
        }
    }
    
    md!(covered);
    for &(u, d, l, r) in udlr.iter() {
        let to_sub = grid[d][r] - grid[d][l - 1] - grid[u - 1][r] + grid[u - 1][l - 1];
        
        wl!(2000 * 2000 - covered + to_sub);
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
