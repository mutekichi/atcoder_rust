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
        h: i64, w: i64,
        n: usize,
        AB: [(i64, i64); n],
    }
    let mut most_rights = BTreeMap::new();
    let mut floors = BTreeSet::new();
    let mut positions = BTreeMap::new();

    for &(a, b) in &AB {
        let a = a - 1;
        let b = b - 1;
        floors.insert(a);
        most_rights
            .entry(a)
            .and_modify(|e| *e = max(*e, b))
            .or_insert(b);
        let mut new_set = BTreeSet::new();
        new_set.insert(b);
        new_set.insert(0);
        new_set.insert(w - 1);
        positions
            .entry(a)
            .and_modify(|e: &mut BTreeSet<i64>| {
                e.insert(b);
            })
            .or_insert(new_set);
    }

    let mut cost = INF_I64;
    {
        let mut cost_1 = 0;
        for e in &most_rights {
            cost_1 += e.1;
        }
        cost = min(cost, cost_1 * 2);
    }
    {
        let mut total_cost = 0;
        let mut costs = BTreeSet::new();
        let mut key = 0usize;
        for a in floors {
            let positions = positions.get(&a).unwrap();
            let max_gap = positions
                .iter()
                .collect::<Vec<_>>()
                .windows(2)
                .map(|w| w[1] - w[0])
                .max()
                .unwrap();
            costs.insert((w - 1 - max_gap, key));
            key += 1;
        }
        let mut pass = 0;
        for (cost, _) in costs.iter().rev() {
            if pass < 2 {
                pass += 1;
                continue;
            } else {
                total_cost += *cost * 2;
            }
        }
        total_cost += (w - 1) * 2;
        cost = min(cost, total_cost);
    }
    println!("{}", cost);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
