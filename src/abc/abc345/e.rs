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

const INF_I64: i64 = 1 << 62;
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
        k: usize,
        CV: [(Usize1, i64); n],
    }
    let mut dp_before = vec![((INF_I64, INF_USIZE - 2), (INF_I64, INF_USIZE - 1)); k + 1];
    let mut dp_after = vec![((INF_I64, INF_USIZE), (INF_I64, INF_USIZE)); k + 1];
    dp_before[0].0 = (0, INF_USIZE);

    for i in 0..n {
        for j in 0..=k {
            let (c, v) = CV[i];
            // remove i-th
            let mut vec = vec![];
            if j == 0 {
                vec.push((INF_I64, INF_USIZE));
            } else {
                vec.push((dp_before[j - 1].0.0 + v, dp_before[j - 1].0.1));
                vec.push((dp_before[j - 1].1.0 + v, dp_before[j - 1].1.1));
            };
            for bef in [dp_before[j].0, dp_before[j].1] {
                if bef.1 != c {
                    vec.push((bef.0, c));
                    break;
                }
            }
            vec.sort_unstable();
            let mut idx = 0;
            let mut used = BTreeSet::new();
            for &e in &vec {
                if !used.contains(&e.1) {
                    if idx == 0 {
                        dp_after[j].0 = e;
                        idx += 1;
                    } else {
                        dp_after[j].1 = e;
                        break;
                    }
                    used.insert(e.1);
                }
            }
        }
        swap(&mut dp_before, &mut dp_after);
    }
    let ans = CV.iter().map(|e| e.1).sum::<i64>() - dp_before[k].0.0;
    println!("{}", if ans > 0 { ans } else { -1 });
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
