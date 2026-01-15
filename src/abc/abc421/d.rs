#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Common imports
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::io::{BufWriter, Write, stdout};

// External crates (Available in AtCoder)
use itertools::{Itertools, iproduct};
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
        ra: i64, ca: i64, rb: i64, cb: i64,
        n: i64,
        m: usize,
        l: usize,
        mut A: [(char, i64); m],
        mut B: [(char, i64); l],
    }

    A.push(('U', INF_I64));
    B.push(('U', INF_I64));
    for i in 1..(m + 1) {
        A[i].1 += A[i - 1].1;
    }
    for i in 1..(l + 1) {
        B[i].1 += B[i - 1].1;
    }
    
    let mut time = 0;
    let mut ia = 0;
    let mut ib = 0;
    let mut moves = vec![];
    while time < n {
        if A[ia].1 < B[ib].1 {
            moves.push((A[ia].1 - time, get_move(A[ia].0), get_move(B[ib].0)));
            time = A[ia].1;
            ia += 1;
        }
        else if A[ia].1 > B[ib].1 {
            moves.push((B[ib].1 - time, get_move(A[ia].0), get_move(B[ib].0)));
            time = B[ib].1;
            ib += 1;
        }
        else {
            moves.push((A[ia].1 - time, get_move(A[ia].0), get_move(B[ib].0)));
            time = A[ia].1;
            ia += 1;
            ib += 1;
        }
    }

    let mut pa = (ra, ca);
    let mut pb = (rb, cb);

    let mut ans = 0;
    for (duration, move_a, move_b) in moves {
        if pa == pb {
            if move_a == move_b {
                ans += duration;
            }
        }
        else if (pa.0 - pb.0).abs() == (pa.1 - pb.1).abs() {
            let diff = (pa.0 - pb.0).abs();
            if duration >= diff && add(pa, move_a, diff) == add(pb, move_b, diff) {
                ans += 1;
            }
        }
        else if pa.0 == pb.0 && move_a.0 == 0 && move_b.0 == 0 {
            let a = pa.1;
            let b = pb.1;
            let da = move_a.1;
            let db = move_b.1;
            if da + db == 0 && ((a < b && da == 1) || (a > b && db == 1)) && (a - b).abs() <= 2 * duration && (a - b).abs() % 2 == 0{
                ans += 1;
            }
        }
        else if pa.1 == pb.1 && move_a.1 == 0 && move_b.1 == 0 {
            let a = pa.0;
            let b = pb.0;
            let da = move_a.0;
            let db = move_b.0;
            if da + db == 0 && ((a < b && da == 1) || (a > b && db == 1)) && (a - b).abs() >= 2 * duration && (a - b).abs() % 2 == 0{
                ans += 1;
            }
        }
        pa = add(pa, move_a, duration);
        pb = add(pb, move_b, duration);
    }
    wl!(ans);
}
struct Point {
    x: i64,
    y: i64,
}
impl std::fmt::Display for Point {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn add(
    p: (i64, i64),
    d: (i64, i64),
    c: i64,
) -> (i64, i64) {
    (p.0 + d.0 * c, p.1 + d.1 * c)
}
fn get_move(c: char) -> (i64, i64) {
    if c == 'U' {
        (-1, 0)
    } else if c == 'D' {
        (1, 0)
    } else if c == 'L' {
        (0, -1)
    } else {
        (0, 1)
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
