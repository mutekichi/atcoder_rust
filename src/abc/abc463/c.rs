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
        HL: [(i64, i64); n],
        q: usize,
        T: [i64; q]
    }
    let mut heights = BTreeSet::new();
    let mut times = vec![];
    for i in 0..n {
        let (h, l) = HL[i];
        heights.insert((h, i));
        times.push((l, i));
    }
    let mut ans = vec![0; q];
    for i in 0..q {
        times.push((T[i], i + INF_USIZE));
    }
    times.sort_unstable();
    for (t, i) in times {
        md!(t, i);
        if i >= INF_USIZE {
            let i = i - INF_USIZE;
            ans[i as usize] = heights.last().unwrap().0;
        }
        else {
            heights.remove(&(HL[i].0, i));
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
