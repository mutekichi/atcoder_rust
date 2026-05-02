#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
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

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        A: [i64; n],
        q: usize,
    }

    let mut froms = BTreeMap::new();
    let mut tos = BTreeMap::new();

    froms.insert(INF_I64, A[n - 1]);
    tos.insert(0, A[0]);

    for i in 0..n {
        froms.insert(A[i], if i == 0 { 0 } else { A[i - 1] });
        tos.insert(A[i], if i == n - 1 { INF_I64 } else { A[i + 1] });
    }

    for _ in 0..q {
        input! {
            qtype: usize,
        }
        if qtype == 1 {
            input! {
                x: i64,
                y: i64,
            }
            let to = *tos.get(&x).unwrap();
            *tos.get_mut(&x).unwrap() = y;
            tos.insert(y, to);
            froms.insert(y, x);
            *froms.get_mut(&to).unwrap() = y;
        }
        else {
            input! {
                x: i64,
            }
            let to = *tos.get(&x).unwrap();
            let from = *froms.get(&x).unwrap();
            *tos.get_mut(&from).unwrap() = to;
            md!(from, to);
            *froms.get_mut(&to).unwrap() = from;
            tos.remove_entry(&x);
            froms.remove_entry(&x);
        }
    }
    let mut v = 0;
    let mut ans = vec![];
    while v != INF_I64 {
        if v != 0 {
            ans.push(v);
        }
        v = *tos.get(&v).unwrap();
    }
    println!("{}", ans.iter().join(" "));
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
