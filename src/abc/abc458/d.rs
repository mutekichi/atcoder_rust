#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;
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
        x: i64,
        q: usize,
        AB: [(i64, i64); q]
    }

    {
        let mut heap = BinaryHeap::new();
        heap.push(1); heap.push(2);
    }

    let mut upper = BinaryHeap::new();
    let mut lower = BinaryHeap::new();
    let mut center = x;
    for (a, b) in AB {
        if center <= a {
            upper.push(Reverse(a));
            lower.push(center);
        }
        else {
            upper.push(Reverse(center));
            lower.push(a);
        }
        let Reverse(upper_peek) = *upper.peek().unwrap();
        let lower_peek = *lower.peek().unwrap();
        if b < lower_peek {
            center = lower_peek;
            lower.pop();
            lower.push(b);
        } else if b < upper_peek {
            center = b;
        }
        else {
            center = upper_peek;
            upper.pop();
            upper.push(Reverse(b));
        }
        println!("{}", center);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
