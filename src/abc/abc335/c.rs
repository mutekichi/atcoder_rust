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
        n: usize,
    }
    let mut data = vec![];
    for i in (1..=n).rev() {
        data.push((i as i32, 0));
    }
    input! {
        q: usize,
    }
    let mut count = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            let last = data.last().unwrap();
            let last_x = last.0;
            let last_y = last.1;
            input! {
                c: char
            }
            let mut diff_x = 0;
            let mut diff_y = 0;
            if c == 'R' {
                diff_x = 1;
            } else if c == 'L' {
                diff_x = -1;
            } else if c == 'U' {
                diff_y = 1;
            } else if c == 'D' {
                diff_y = -1;
            }
            let next_x = last_x + diff_x;
            let next_y = last_y + diff_y;
            data.push((next_x, next_y));
            count += 1;
        } else {
            input! {
                p : usize,
            }
            let (x, y) = data[n - p + count];
            println!("{} {}", x, y);
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
