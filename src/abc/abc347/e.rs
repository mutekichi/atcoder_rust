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
        X: [Usize1; q],
    }
    let mut ans = vec![0; n];
    let mut set = BTreeSet::new();
    let mut last_accum = vec![INF_USIZE; n];
    let mut accum = 0;
    for i in 0..q {
        let x = X[i];
        if set.contains(&x) {
            ans[x] += accum - last_accum[x];
            set.remove(&x);
            accum += set.len();

            last_accum[x] = INF_USIZE;
        }
        else {
            last_accum[x] = accum;
            set.insert(x);
            accum += set.len();
        }
    }
    for x in 0..n {
        if last_accum[x] != INF_USIZE {
            ans[x] += accum - last_accum[x];
        }
    }
    println!("{}", ans.iter().join(" "));
}