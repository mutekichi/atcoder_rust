#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::{Rng, random_range};
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};
use std::time;

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize, l: usize,
        A: [i64; n],
    }
    let mut memo = vec![vec![vec![-1.; l + 1]; n + 1]; n + 1];

    println!(
        "{}",
        f(0, n, l, &mut memo) * (A.iter().sum::<i64>()) as f64 / n as f64
    );
}

fn f(
    a: usize,
    b: usize,
    l: usize,
    memo: &mut Vec<Vec<Vec<f64>>>,
) -> f64 {
    if l == 0 {
        return 0.;
    }
    if memo[a][b][l] > -0.1 {
        return memo[a][b][l];
    }
    let mut ans = 0.;
    let total = a + b * 2;
    let p1 = a as f64 / total as f64;
    // pattern 1: first a
    if a > 0 {
        ans += p1 * (1. + f(a - 1, b, l, memo));
    }
    // pattern 2: first b
    if b > 0 {
        if l == 1 {
            // pattern 2 - 1: l == 1
            ans += (1.0 - p1) / (total - 1) as f64 * (1. + f(a, b - 1, l, memo));
        } else {
            // pattern 2 - 2: l > 1
            let q1 = 1. / (total - 1) as f64;
            ans += (1.0 - p1) * q1 * (1. + f(a, b - 1, l, memo));
            let q2 = a as f64 / (total - 1) as f64;
            ans += (1.0 - p1) * q2 * (1. + f(a, b - 1, l - 1, memo));

            if b > 1 {
                ans += (1.0 - p1) * (1. - q1 - q2) * f(a + 2, b - 2, l - 1, memo);
            }
        }
    }

    memo[a][b][l] = ans;
    md!(a, b, l, ans);
    return ans;
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIR8: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
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

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
