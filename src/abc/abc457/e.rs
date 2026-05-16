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
        n: usize, m: usize,
        LR: [(Usize1, Usize1); m],
        q: usize,
        ST: [(Usize1, Usize1); q],
    }
    let mut ng_from_left = vec![false; q];
    let mut ranges_from_left = vec![None; q];
    {
        let mut rs = vec![BTreeSet::new(); n];
        for i in 0..m {
            let (l, r) = LR[i];
            rs[l].insert((r, i));
        }
        for i in 0..q {
            let (s, t) = ST[i];
            let res = rs[s].range((s, 0)..=(t, INF_USIZE)).next_back();
            if res.is_none() {
                ranges_from_left[i] = None;
            } else {
                let &(val, key) = res.unwrap();
                if val == t {
                    rs[s].remove(&(val, key));
                    let contains_other = rs[s].range((s, 0)..=(t, INF_USIZE)).next().is_some();
                    if !contains_other {
                        ng_from_left[i] = true;
                    }
                    rs[s].insert((val, key));
                }
                ranges_from_left[i] = Some(val);
            }
        }
    }

    let mut ng_from_right = vec![false; q];
    let mut ranges_from_right = vec![None; q];
    {
        let mut ls = vec![BTreeSet::new(); n];
        for i in 0..m {
            let (l, r) = LR[i];
            ls[r].insert((l, i));
        }
        for i in 0..q {
            let (s, t) = ST[i];
            let res = ls[t].range((s, 0)..=(t, INF_USIZE)).next();
            if res.is_none() {
                ranges_from_right[i] = None;
            } else {
                let &(val, key) = res.unwrap();
                if val == s {
                    ls[t].remove(&(val, key));
                    let contains_other = ls[t].range((s, 0)..=(t, INF_USIZE)).next().is_some();
                    if !contains_other {
                        ng_from_right[i] = true;
                    }
                    ls[t].insert((val, key));
                }
                ranges_from_right[i] = Some(val);
            }
        }
    }

    let mut data = BTreeSet::new();
    let mut LR_2 = vec![];
    let mut contains_other_range = vec![false; m];
    for i in 0..m {
        let (l, r) = (LR[i].0, n - 1 - LR[i].1);
        LR_2.push((l, r, i));
    }
    LR_2.sort_unstable();
    for (l, r, i) in LR_2.into_iter().rev() {
        if data.range(r..).next().is_some() {
            contains_other_range[i] = true;
        }
        data.insert(r);
    }

    let mut to_range_id = BTreeMap::new();
    for i in 0..m {
        to_range_id.insert(LR[i], Reverse(i));
    }

    for i in 0..q {
        if ranges_from_left[i].is_none() || ranges_from_right[i].is_none() {
            println!("No");
        } else {
            if ranges_from_left[i].unwrap() + 1 >= ranges_from_right[i].unwrap() {
                if ng_from_left[i] && ng_from_right[i] {
                    let &Reverse(range_id) = to_range_id.get(&ST[i]).unwrap();
                    if contains_other_range[range_id] {
                        md!("ok");
                        println!("Yes");
                    } else {
                        md!("ng");
                        println!("No");
                    }
                } else {
                    println!("Yes");
                }
            } else {
                println!("No");
            }
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
