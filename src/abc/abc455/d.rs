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
        n: usize, q: usize,
        CP: [(Usize1, Usize1); q],
    }
    let mut data = vec![];
    for i in 0..n {
        data.push((INF_USIZE, INF_USIZE));
    }
    let mut tos = (0..n).collect::<Vec<_>>();
    for (c, p) in CP {
        let (from, to) = data[c].clone();
        if from == INF_USIZE {
            tos[c] = INF_USIZE;
        } else {
            let to_change = data[c].0;
            data[to_change].1 = INF_USIZE;
        }
        data[c].0 = p;
        data[p].1 = c;
        if false {
            let mut ans = vec![];
            for i in 0..n {
                if tos[i] == INF_USIZE {
                    ans.push(0);
                } else {
                    let mut cnt = 0;
                    let mut d = data[i];
                    while d.1 != INF_USIZE {
                        cnt += 1;
                        d = data[d.1];
                    }
                    ans.push(cnt);
                }
            }
            println!("{}", ans.iter().join(" "));
        }
    }
    {
        let mut ans = vec![];
        for i in 0..n {
            if tos[i] == INF_USIZE {
                ans.push(0);
            } else {
                let mut cnt = 1;
                let mut d = data[i];
                while d.1 != INF_USIZE {
                    cnt += 1;
                    d = data[d.1];
                }
                ans.push(cnt);
            }
        }
        println!("{}", ans.iter().join(" "));
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
