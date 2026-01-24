#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

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

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        A: Chars,
    }
    let mut pows = vec![1usize];
    for i in 0..n {
        pows.push(pows[i] * 3);
    }
    wl!(f(&A, 0, n, &pows).1);
}

fn f(
    A: &Vec<char>,
    start: usize,
    level: usize,
    pows: &Vec<usize>,
) -> (u32, usize) {
    if level == 0 {
        return (A[start].to_digit(10).unwrap(), 1);
    } else {
        let mut zero_to_ones = vec![];
        let mut one_to_zeros = vec![];
        for i in 0..3 {
            let (c, steps) = f(A, start + i * pows[level - 1], level - 1, pows);
            if c == 0 {
                zero_to_ones.push(steps);
            } else {
                one_to_zeros.push(steps);
            }
        }
        zero_to_ones.sort_unstable();
        one_to_zeros.sort_unstable();
        if zero_to_ones.len() == 0 {
            (0, one_to_zeros[0] + one_to_zeros[1])
        } else if zero_to_ones.len() == 1 {
            (0, one_to_zeros[0])
        } else if zero_to_ones.len() == 2 {
            (1, zero_to_ones[0])
        } else {
            (1, zero_to_ones[0] + zero_to_ones[1])
        }
    }
}
