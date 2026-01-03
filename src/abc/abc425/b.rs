#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};

// External crates (Available in AtCoder)
use itertools::{Itertools, iproduct};
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
        A: [i64; n],
    }
    let minus_one_count = A.iter().filter(|a| **a == -1).count();
    let mut unused_numbers = vec![];
    for i in 1..=n {
        if !A.contains(&(i as i64)) {
            unused_numbers.push(i as i64);
        }
    }
    md!(unused_numbers.len());
    for perm in (0..n).permutations(n) {
        let mut idx = 0;
        let mut ans = vec![0; n];
        let mut ok = true;
        for i in 0..n {
            if A[i] == -1 {
                ans[i] = unused_numbers[idx];
                idx += 1;
            }
            else {
                if A[i] == perm[i] as i64 + 1 {
                    ans[i] = A[i];
                }
                else {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            wl!("Yes");
            wl!(ans.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
            return;
        }
    }
    wl!("No");
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
