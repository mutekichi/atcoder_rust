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
        h: usize, w: usize,
        A: [[i64; w]; h],
        B: [[i64; w]; h],
    }
    /*
    let mut counters = vec![BTreeMap::new(), BTreeMap::new()];
    let V = vec![A, B];
    for k in 0..2 {
        for i in 0..h {
            for j in 0..w {
                *counters[k].entry(V[k][i][j]).or_insert(0) += 1;
            }
        }
    }
    if counters[0] != counters[1] {
        println!("{}", -1);
        return;
    }
    let map = counters[0]
        .iter()
        .enumerate()
        .map(|v| (*v.1.0, v.0))
        .collect::<BTreeMap<i64, usize>>();

    let mut C = vec![vec![0; w]; h];
    let mut D = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            C[i][j] = *map.get(&V[0][i][j]).unwrap();
            D[i][j] = *map.get(&V[1][i][j]).unwrap();
        }
    }

    let target = encode(&D);
    let mut seen = BTreeSet::new();
    */
    let mut ans = INF_USIZE;

    for V in (0..h).permutations(h) {
        for W in (0..w).permutations(w) {
            let mut ok = true;
            for i in 0..h {
                for j in 0..w {
                    if A[V[i]][W[j]] != B[i][j] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                ans = min(ans, inversion(&V) + inversion(&W));
            }
        }
    }

    println!("{}", if ans != INF_USIZE { ans as i64 } else { -1 });
}

fn encode(X: &Vec<Vec<usize>>) -> String {
    X.iter().flatten().map(|v| v.to_char()).collect::<String>()
}

fn inversion(V: &Vec<usize>) -> usize {
    let mut ret = 0;
    for i in 1..V.len() {
        for j in 0..i {
            if V[i] < V[j] {
                ret += 1;
            }
        }
    }
    md!(V.iter().join(" "), ret);
    ret
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
