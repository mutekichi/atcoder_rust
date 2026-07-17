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

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
        N: [i64; t],
    }
    for n in N {
        let mut ans = 0;
        if n == 1 {
            ans = 0;
        } else if n == 2 {
            ans = 1;
        } else if n == 3 {
            ans = 2;
        } else if n == 4 {
            ans = 4;
        } else {
            let m = n.isqrt();
            ans += 2 * m * (m - 1);

            let rem = n - m * m;
            let bottom = min(rem, m);
            ans += max(0, bottom * 2 - 1);

            let right = max(0, rem - m);
            ans += max(0, right * 2 - 1);
        }
        println!("{}", ans);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
