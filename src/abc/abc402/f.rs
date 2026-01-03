#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

fn create_mod_set(
    x: usize,
    y: usize,
    n: usize,
    m: i128,
    downs: usize,
    rights: usize,
    a_mods: &Vec<Vec<i128>>,
    set: &mut BTreeSet<i128>,
    accum: i128,
) {
    if downs == 0 && rights == 0 {
        set.insert(accum);
        return;
    }
    if downs != 0 {
        create_mod_set(
            x, y + 1, n, m, downs - 1, rights, a_mods, set,
            (accum + a_mods[x][y + 1]) % m,
        );
    }
    if rights != 0 {
        create_mod_set(
            x + 1, y, n, m, downs, rights - 1, a_mods, set,
            (accum + a_mods[x + 1][y]) % m
        );
    }
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        M: i128,
        A: [[i128; n]; n],
    }
    if n == 1 {
        wl!(A[0][0] % M);
        return;
    }
    let mut ans = 0;
    let mut mod_table = vec![1; 42];
    for i in 1..42 {
        mod_table[i] = mod_table[i - 1] * 10 % M;
    }
    let mut a_mods = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            a_mods[i][j] = A[i][j] * mod_table[2 * n - 2 - i - j] % M;
        }
    }
    for i in 0..n {
        let mut res_mod_set = BTreeSet::new();
        create_mod_set(
            n - 1 - i,
            i,
            n,
            M,
            n - 1 - i,
            i,
            &a_mods,
            &mut res_mod_set,
            0
        );
        let mut first_mod_set = BTreeSet::new();
        create_mod_set(
            0,
            0,
            n,
            M,
            i,
            n - 1 - i,
            &a_mods,
            &mut first_mod_set,
            a_mods[0][0],
        );
        for first in first_mod_set.iter() {
            let res_max = M - first - 1;
            if let Some(next) = res_mod_set.range(0..=res_max).next_back() {
                ans = max(first + next, ans);
            } else {
                if let Some(largest) = res_mod_set.iter().next_back() {
                    ans = max((first + largest) % M, ans);
                }
            }
        }
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
