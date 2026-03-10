#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
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
        q: usize,
        mut A: [i64; n],
    }
    let mut A = A.iter().enumerate().map(|e| (*e.1, e.0)).collect::<Vec<_>>();
    A.sort_unstable();
    let mut tops = vec![];
    for i in 0..6 {
        tops.push(A[i]);
    }
    drop(A);
    let mut map = BTreeMap::new();
    for &(value, index) in &tops {
        map.insert(index, value);
    }
    for _ in 0..q {
        input! {
            k: usize,
            B: [Usize1; k],
        }
        let mut set = BTreeSet::new(); 
        for i in 0..6 {
            set.insert(tops[i].1);
        }
        for b in B {
            set.remove(&b);
        }
        println!("{}", set.iter().map(|i| map[i]).min().unwrap());
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
