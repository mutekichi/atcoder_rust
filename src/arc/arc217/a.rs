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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: i64,
        }
        let mut ans = vec![];
        for i in 0..(n + 8) / 4 {
            let values = [i * 4, i * 4 + 1, i * 4 + 3, i * 4 + 2];
            for val in values {
                if val != 0 && val <= n {
                    ans.push(val);
                }
            }
        }
        println!("{}", ans.iter().join(" "));
        md!(calc(&ans));
    }
}

fn test() {
    input! {
        n: i64,
    }
    let mut m = INF_I64;
    for perm in (1..=n).permutations(n as usize) {
        let mut accum = 0;
        let mut sum = 0;
        for p in perm {
            accum ^= p;
            sum += accum;
        }
        m = min(m, sum);
    }
    println!("minimum: {}", m);
    for perm in (1..=n).permutations(n as usize) {
        let mut accum = 0;
        let mut sum = 0;
        for &p in &perm {
            accum ^= p;
            sum += accum;
        }
        if sum == m {
            md!(perm.iter().join(" "));
            if false {
                println!(
                    "{}",
                    perm.iter().map(|e| format!("{} {:04b}", e, e)).join("\n")
                );

                println!();
            }
        }
    }
}

fn calc(perm: &Vec<i64>) -> i64 {
    let mut accum = 0;
    let mut sum = 0;
    for &p in perm {
        accum ^= p;
        sum += accum;
    }
    sum
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
