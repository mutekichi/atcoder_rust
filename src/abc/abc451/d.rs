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
    }
    let mut set = BTreeSet::new();
    let mut pows = vec![];
    let mut num = 1;
    while num <= 10i64.pow(9) {
        pows.push((num, keta(num)));
        num *= 2;
    }
    f(0, &mut set, &pows);
    println!("{}", set.iter().skip(n - 1).next().unwrap());
}
fn keta(i: i64) -> i64 {
    let mut i = i;
    let mut keta = 0;
    while i > 0 {
        i /= 10;
        keta += 1;
    }
    keta
}

fn f(
    i: i64,
    set: &mut BTreeSet<i64>,
    pows: &Vec<(i64, i64)>,
) {
    let keta_i = keta(i);
    for &(val, keta) in pows {
        if keta_i + keta > 9 {
            break;
        }
        let new_val = i * 10i64.pow(keta as u32) + val;
        set.insert(new_val);
        f(new_val, set, pows);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
