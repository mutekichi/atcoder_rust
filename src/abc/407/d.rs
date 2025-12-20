#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

fn print_state(
    h: usize,
    w: usize,
    state: &u32,
) {
    for i in 0..h {
        for j in 0..w {
            if state >> (i * w + j) & 1 == 1 {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    eprintln!("==");
}

fn rec(
    h: usize,
    w: usize,
    A: &Vec<Vec<i64>>,
    state: &mut u32,
    ans: &mut i64,
    idx: usize,
) {
    if idx == h * w - 1 {
        // print_state(h, w, state);
        let mut accum = 0i64;
        for (i, j) in iproduct!(0..h, 0..w) {
            if (*state >> (i * w + j)) & 1 == 0 {
                accum ^= A[i][j];
            }
        }
        *ans = max(*ans, accum);
        return;
    }
    if (*state >> idx) & 1 == 1 {
        rec(h, w, &A, state, ans, idx + 1);
    } else {
        let hh = idx / w;
        let ww = idx % w;
        rec(h, w, &A, state, ans, idx + 1);
        if ww != w - 1 && (*state >> (idx + 1)) & 1 == 0 {
            *state ^= 1 << (idx + 1);
            *state ^= 1 << idx;
            rec(h, w, &A, state, ans, idx + 1);
            *state ^= 1 << (idx + 1);
            *state ^= 1 << idx;
        }
        if hh != h - 1 && (*state >> (idx + w)) & 1 == 0 {
            *state ^= 1 << (idx + w);
            *state ^= 1 << idx;
            rec(h, w, &A, state, ans, idx + 1);
            *state ^= 1 << (idx + w);
            *state ^= 1 << idx;
        }
    }
}

#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        h: usize,
        w: usize,
        A: [[i64; w]; h],
    }

    let mut state = 0u32;
    let mut ans = 0;
    
    rec(h, w, &A,  &mut state,  &mut ans, 0);
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

fn join_with_space<T: ToString>(arr: &[T]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
