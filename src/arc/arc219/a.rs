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
        m: usize,
        S: [Chars; n],
    }
    if m < 20 {
        let mut seen = vec![true; 1 << m];
        for s in S {
            let mut num = 0usize;
            for i in 0..m {
                if s[i] == '0' {
                    num += 1 << i;
                }
            }
            md!(num);
            seen[num] = false;
        }
        for i in 0..1 << m {
            if seen[i] {
                let mut str = vec![];
                for j in 0..m {
                    if (i >> j) & 1 == 0 {
                        str.push(0);
                    } else {
                        str.push(1);
                    }
                }
                println!("Yes");
                println!("{}", str.iter().join(""));
                return;
            }
        }
        println!("No");
        return;
    } else {
        let S = S
            .iter()
            .map(|e| e.iter().collect::<String>())
            .collect::<Vec<_>>();
        loop {
            let mut rand_str = String::new();
            let mut rev_str = String::new();
            for i in 0..m {
                if rand::random() {
                    rand_str.push('0');
                    rev_str.push('1');
                } else {
                    rand_str.push('1');
                    rev_str.push('0');
                }
            }
            let mut ok = true;
            for i in 0..n {
                if rev_str == S[i] {
                    md!(rev_str, S[i]);
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            println!("Yes");
            println!("{}", rand_str);
            return;
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
