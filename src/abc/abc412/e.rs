#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use num_integer::gcd;
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

const TEN7: usize = 10000010;
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
        L: i128,
        R: i128,
    }

    if R - L == 0 {
        wl!("1");
        return;
    }

    let mut eratosthenes = vec![true; TEN7 as usize];
    eratosthenes[0] = false;
    eratosthenes[1] = false;
    md!(L, R);

    for i in 0..TEN7 {
        if eratosthenes[i] {
            let mut iu = i + i;
            while iu < TEN7 {
                eratosthenes[iu] = false;
                iu += i;
            }
        }
    }
    let mut ext_eratosthenes = vec![true; TEN7 as usize];

    for i in 0..TEN7 {
        let i_128 = i as i128;
        if eratosthenes[i] {
            let min_number = (L + i_128 - 1) / i_128 * i_128;
            let min_idx = (min_number - L) as usize;
            let mut current_idx = min_idx;
            while current_idx < TEN7 {
                ext_eratosthenes[current_idx] = false;
                current_idx += i;
            }
            if min_number == i_128 {
                ext_eratosthenes[min_idx] = true;
            }
        }
    }
    
    let mut ans: i128 = 0;

    for i in 0..TEN7 {
        if eratosthenes[i] {
            let val = i as i128;
            let mut current = val * val;
            while current <= R {
                if L + 1 <= current && current <= R {
                    let idx = (current - L) as usize;
                    ext_eratosthenes[idx] = true;
                }
                current *= val;
            }
        }
    }

    for i in 1..=((R-L) as usize) {
        if ext_eratosthenes[i] {
            ans += 1;
        }
    }

    wl!(ans + 1);
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
