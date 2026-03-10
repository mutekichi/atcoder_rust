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
        w: usize,
        h: usize,
        XY: [(usize, usize); n],
    }
    let mut state = vec![vec![false; w]; h];
    let mut ans = false;
    rec(h, w, n, &XY, (1<<n) - 1, &mut state, &mut ans);
    if ans {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

fn rec(
    h: usize,
    w: usize,
    n: usize,
    XY: &Vec<(usize, usize)>,
    rem: usize,
    state: &mut Vec<Vec<bool>>,
    ans: &mut bool,
) {
    let mut s = INF_USIZE;
    let mut t = INF_USIZE;
    for i in 0..h {
        for j in 0..w {
            if !state[i][j] {
                s = i;
                t = j;
                break;
            }
        }
        if s != INF_USIZE {
            break;
        }
    }
    if s == INF_USIZE && t == INF_USIZE {
        *ans = true;
        return;
    }
    for i in 0..n {
        if (rem >> i) & 1 == 0 {
            continue;
        }
        let (x, y) = XY[i];
        if check(h, w, s, t, state, x, y) {
            for i in s..(s + x) {
                for j in t..(t + y) {
                    state[i][j] = true;
                }
            }
            rec(h, w, n, XY, rem ^ (1 << i), state, ans);
            for i in s..(s + x) {
                for j in t..(t + y) {
                    state[i][j] = false;
                }
            }
        }
        if x != y {
            let (x, y) = (y, x);
            if check(h, w, s, t, state, x, y) {
                for i in s..(s + x) {
                    for j in t..(t + y) {
                        state[i][j] = true;
                    }
                }
                rec(h, w, n, XY, rem ^ (1 << i), state, ans);
                for i in s..(s + x) {
                    for j in t..(t + y) {
                        state[i][j] = false;
                    }
                }
            }
        }
    }
}

fn check(
    h: usize,
    w: usize,
    s: usize,
    t: usize,
    state: &Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> bool {
    if s + x > h {
        return false;
    }
    if t + y > w {
        return false;
    }
    state[s..(s + x)]
        .iter()
        .all(|row| row[t..(t + y)].iter().all(|e| !*e))
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
