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
        S: [Chars; t]
    }
    for S in S {
        let mut counter = vec![0; 26];
        for &c in &S {
            counter[(c as u8 - b'a') as usize] += 1;
        }
        let mut set = BTreeSet::new();
        for i in 0..26 {
            set.insert((counter[i], i));
        }
        let max_cnt = set.last().unwrap().0;
        let lim = (S.len() + 1) / 2;
        if max_cnt > lim {
            println!("No");
            continue;
        }
        else {
            println!("Yes");
            let mut idx = 0;
            let mut ans = vec!['a'; S.len()];
            for &(cnt, c) in set.iter().rev() {
                for i in 0..cnt {
                    ans[idx] = (c as u8+ b'a') as char;
                    idx += 2;
                    if idx >= S.len() {
                        idx = 1;
                    }
                }
            }
            println!("{}", ans.iter().join(""));
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
