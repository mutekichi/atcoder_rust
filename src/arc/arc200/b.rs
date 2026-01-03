#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
use num_integer::gcd;
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
fn calc(
    a: i128,
    b: i128,
    c: i128,
) -> Option<(i128, i128)> {
    if c < max(a, b) || a + b < c {
        None
    } else {
        let (x, y) = if a + b == c {
            (10i128.pow(a as u32) - 1, 10i128.pow(b as u32) - 2)
        } else {
            let small = min(a, b);
            let large = max(a, b);
            let v = 10i128.pow((small - 1) as u32);
            let w_mul = 10i128.pow((small + large - c - 1) as u32);
            let w_base = 10i128.pow((c - small + 1) as u32) - 1;
            let w = w_base * w_mul;
            if a < b {
                (v, w)
            } else {
                (w, v)
            }
        };
        Some((x, y))
    }
}

fn ketasuu(n: i128) -> usize {
    let mut keta = 0usize;
    let mut v = n;
    while v > 0 {
        v /= 10;
        keta += 1;
    }
    keta
}

#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }


    // for i in 1..=17 {
    //     for j in 1..17 {
    //         for k in 1..17 {
    //             if let Some((x, y)) = calc(i as i128, j as i128, k as i128) {
    //                 let z = x / gcd(x, y) * y;
    //                 if ketasuu(z) != k {
    //                     md!(i, j, k, x, y, z);
    //                 }
    //             }
    //         }
    //     }
    // } 
    input! {
       t: usize, 
    }
    
    for _ in 0..t {
        input! {
            a: i128, b: i128, c: i128,
        }
        if let Some((x, y)) = calc(a, b, c) {
            wl!("Yes");
            wl!("{} {}", x, y);
        } else {
            wl!("No");
        }
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
