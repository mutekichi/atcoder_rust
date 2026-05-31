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
        S: [Chars; n]
    }
    let mut ans = vec![];
    for i in 0..n {
        let c = S[i][0];
        md!(c);
        if c == 'a' || c == 'b' || c == 'c' {
            ans.push(2);
        }
        if c == 'd' || c == 'e' || c == 'f' {
            ans.push(3);
        }
        if c == 'g' || c == 'h' || c == 'i' {
            ans.push(4);
        }
        if c == 'j' || c == 'k' || c == 'l' {
            ans.push(5);
        }
        if c == 'm' || c == 'n' || c == 'o' {
            ans.push(6);
        }
        if c == 'p' || c == 'q' || c == 'r' || c == 's' {
            ans.push(7);
        }
        if c == 't' || c == 'u' || c == 'v' {
            ans.push(8);
        }
        if c == 'w' || c == 'x' || c == 'y' || c == 'z' {
            ans.push(9);
        }
    }
    println!("{}", ans.iter().join(""));
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
