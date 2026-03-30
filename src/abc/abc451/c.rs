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
        q: usize,
        TH: [(usize, i64); q],
    }
    let mut height_set = BTreeSet::new();
    for &(_, h) in &TH {
        height_set.insert(h);
    }
    let mut height_vec = height_set.iter().collect::<Vec<_>>();
    let mut height_map = BTreeMap::new();
    for i in 0..height_vec.len() {
        height_map.insert(height_vec[i], i);
    }

    let mut trees = BTreeSet::new();
    for i in 0..q {
        let (t, h) = TH[i];
        if t == 1 {
            trees.insert((h, i));
        } else {
            let mut to_remove = vec![];
            for i in trees.range(..(h + 1, 0)) {
                to_remove.push(*i);
            }
            for r in to_remove {
                trees.remove(&r);
            }
        }
        println!("{}", trees.len());
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
