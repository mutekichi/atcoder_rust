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
        m: usize,
        A: [i64; n],
        XY: [(Usize1, Usize1); m],
    }
    let forward = {
        let mut mins = vec![INF_I64; n];
        let mut graph = vec![vec![]; n];
        for &(x, y) in &XY {
            graph[x].push(y);
        }
        for i in 0..n {
            for &j in &graph[i] {
                mins[j] = min(mins[j], min(A[i], mins[i]));
            }
        }
        mins
    };
    let backward = {
        let mut maxes = A.iter().cloned().collect::<Vec<_>>();
        let mut graph = vec![vec![]; n];
        for &(x, y) in &XY {
            graph[y].push(x);
        }
        for i in (0..n).rev() {
            for &j in &graph[i] {
                maxes[j] = max(maxes[j], maxes[i]);
            }
        }
        maxes
    };
    let mut ans = -INF_I64;
    for i in 0..n {
        md!(backward[i], forward[i]);
        ans = max(ans, backward[i] - forward[i]);
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
