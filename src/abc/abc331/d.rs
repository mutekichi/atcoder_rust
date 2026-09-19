#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use memoise::memoise;
use num_integer::gcd;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::btree_map::Entry;
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        q: usize,
        P: [Chars; n],
        ABCD: [(usize, usize, usize, usize); q],
    }
    let mut accums = vec![vec![0; n + 1]; n + 1];

    for i in 0..n {
        for j in 0..n {
            if P[i][j] == 'B' {
                accums[i + 1][j + 1] += 1;
            }
        }
    }
    for i in 0..n + 1 {
        for j in 0..n {
            accums[i][j + 1] += accums[i][j];
        }
    }
    for i in 0..n {
        for j in 0..n + 1 {
            accums[i + 1][j] += accums[i][j];
        }
    }
    for (a, b, c, d) in ABCD {
        let ak = a / n;
        let bk = b / n;
        let ck = c / n;
        let dk = d / n;
        md!(ak, bk, ck, dk);

        let am = a % n;
        let bm = b % n;
        let cm = c % n;
        let dm = d % n;
        md!(am, bm, cm, dm);

        let val = {
            let n = n - 1;
            if ak == ck {
                if bk == dk {
                    calc(am, bm, cm, dm, &accums)
                } else {
                    let mut val = 0;
                    if bm != 0 {
                        val += calc(am, bm, cm, n, &accums);
                    }
                    val += calc(am, 0, cm, dm, &accums);
                    val += calc(am, 0, cm, n, &accums) * (dk - bk);
                    val
                }
            } else {
                if bk == dk {
                    let mut val = 0;
                    if am != 0 {
                        val += calc(am, bm, n, dm, &accums);
                    }
                    md!(val);
                    val += calc(0, bm, cm, dm, &accums);
                    md!(val);
                    val += calc(0, bm, n, dm, &accums) * (ck - ak);
                    md!(val);
                    val
                } else {
                    let mut val = 0;
                    val += calc(0, bm, cm, dm, &accums);
                    md!(val);
                    val += calc(am, 0, cm, dm, &accums);
                    md!(val);
                    val += calc(am, bm, n, dm, &accums);
                    md!(val);
                    val += calc(am, bm, cm, n, &accums);
                    md!(val);
                    val += calc(am, 0, 0, 0, &accums);
                    md!(val);
                    val += calc(0, bm, 0, 0, &accums);
                    md!(val);
                    val += calc(0, 0, cm, 0, &accums);
                    md!(val);
                    val += calc(0, 0, 0, dm, &accums);
                    md!(val);
                    val +=
                        calc(0, 0, n, n, &accums) * (ck - ak) * (dk - bk);
                    md!(val);
                    val
                }
            }
        };
        println!("{}", val);
    }
}

fn calc(
    am: usize,
    bm: usize,
    cm: usize,
    dm: usize,
    accums: &Vec<Vec<usize>>,
) -> usize {
    accums[cm + 1][dm + 1] + accums[am][bm]
        - accums[cm + 1][bm]
        - accums[am][dm + 1]
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[rustfmt::skip]
const DIR8: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
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

trait AsciiExt {
    fn to_idx(self) -> usize;
}

impl AsciiExt for char {
    fn to_idx(self) -> usize {
        (self as u8 - b'a') as usize
    }
}

impl AsciiExt for u8 {
    fn to_idx(self) -> usize {
        (self - b'a') as usize
    }
}

trait UsizeExt {
    fn to_char(self) -> char;
}

impl UsizeExt for usize {
    fn to_char(self) -> char {
        (self as u8 + b'a') as char
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
