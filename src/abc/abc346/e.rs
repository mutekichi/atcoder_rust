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
        h: usize,
        w: usize,
        m: usize,
        TAX: [(Usize1, Usize1, usize); m],
    }
    let mut ans = vec![0i64; 200100];
    let mut used_h = vec![false; h];
    let mut used_w = vec![false; w];
    let mut set_h = BTreeSet::new();
    let mut set_w = BTreeSet::new();

    for &(t, a, x) in TAX.iter().rev() {
        if t == 0 {
            if used_h[a] {
                continue;
            }
            else {
                used_h[a] = true;
                ans[x] += (w - set_w.len()) as i64;
                set_h.insert(a);
            }
        }
        else {
            if used_w[a] {
                continue;
            }
            else {
                used_w[a] = true;
                ans[x] += (h - set_h.len()) as i64;
                set_w.insert(a);
            }
        }
    }
    let mut vec = vec![];
    let mut sum = 0;
    let contains_zero = ans[0] > 0;
    for i in 0..200005 {
        if ans[i] > 0 {
            vec.push((i, ans[i]));
            sum += ans[i];
        }
    }
    let rem = h as i64 * w as i64 - sum;
    md!(h, w, h * w, rem);
    if !contains_zero && rem > 0  {
        vec.insert(0, (0, rem));
    }
    println!("{}", vec.len());
    for (i, cnt) in vec {
        let mut cnt = cnt;
        if i == 0  && contains_zero {
            cnt += rem;
        }
        println!("{} {}", i, cnt);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
