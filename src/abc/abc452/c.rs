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
        AB: [(Usize1, Usize1); n],
        m: usize,
        S: [Chars; m],
    }
    let mut data = vec![vec![vec![false; 26]; 10]; 10];
    for s in &S {
        for i in 0..s.len() {
            let c = (s[i] as u8 - b'a') as usize;
            data[s.len() - 1][i][c] = true;
        }
    }
    for s in &S {
        if s.len() != n {
            println!("No");
        } else {
            let mut ok = true;
            for i in 0..n {
                let c = (s[i] as u8 - b'a') as usize;
                let (a, b) = AB[i];
                if !data[AB[i].0][AB[i].1][c] {
                    ok = false;
                    println!("No");
                    break;
                }
            }
            if ok {
                println!("Yes");
            }
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
