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
const INF: i64 = 1 << 60;
const MOD: i64 = 998244353;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

fn f(a: isize) -> isize {
    // let mut b = 0;
    // let mut ac = a.to_owned();
    // while ac != 0 {
    //     b += ac % 10;
    //     ac /= 10;
    // }
    // return b;
    a.to_string().chars().map(|c| c.to_digit(10).unwrap() as isize).sum()
}

// Logic goes here
#[allow(unused_macros)]
#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input!{
        n: usize,
    }

    let mut s = vec![0; n + 1];
    s[0] = 1;

    for i in 1..=n {
        for j in 0..i {
            s[i] += f(s[j]);
        }
    }

    wl!(s[n as usize]);
}

// --- Macros ---

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