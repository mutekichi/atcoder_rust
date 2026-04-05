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
        A: [i64; n]
    }
    let mut seen = vec![false; n];
    let mut status = vec![1; n];
    let order = (0..n)
        .sorted_by(|i, j| A[*i].cmp(&A[*j]))
        .collect::<Vec<_>>();
    let mut ans = 0;
    for &i in order.iter().rev() {
        if i > 0 && seen[i - 1] {
            if i < n - 1 && seen[i + 1] {
                md!("both");
                let width = 1 + status[i - 1] + status[i + 1];
                ans = max(ans, width * A[i]);
                let left = i - status[i - 1] as usize;
                status[left] = width;
                let right = i + status[i + 1] as usize;
                status[right] = width;
            } else {
                md!("left");
                let width = 1 + status[i - 1];
                ans = max(ans, width * A[i]);
                let left = i - status[i - 1] as usize;
                status[left] = width;
                status[i] = width;
            }
        } else if i < n - 1 && seen[i + 1] {
            md!("right");
            let width = 1 + status[i + 1];
            ans = max(ans, width * A[i]);
            let right = i + status[i + 1] as usize;
            status[right] = width;
            status[i] = width;
        } else {
            md!("alone");
            ans = max(ans, A[i]);
        }
        seen[i] = true;
        md!(i, ans);
        md!(status.iter().join(" "));
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
