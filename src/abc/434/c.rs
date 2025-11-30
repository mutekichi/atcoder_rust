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
        n_test: usize,
    }

    for i in 0..n_test {
        input! {
            n: usize,
            h: i128,
            tlu: [(i128, i128, i128); n],
        }

        let mut min_height = h;
        let mut max_height = h;
        let mut t_bef = 0;

        let mut ok = true;

        for &(t, l, u) in tlu.iter() {
            let t_diff = t - t_bef;
            if min_height - t_diff > u {
                wl!("No");
                ok = false;
                break;
            }
            if max_height + t_diff < l {
                wl!("No");
                ok = false;
                break;
            }
            min_height = max(min_height - t_diff, l);
            max_height = min(max_height + t_diff, u);
            t_bef = t;
        };

        if ok {
            wl!("Yes");
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
