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

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize, m : usize, AB: [(Usize1, Usize1); m],
    }
    if n == 2 {
        println!("1");
        return;
    }
    let mut vals = vec![BTreeSet::new(); n];
    for &(a, b) in &AB {
        vals[a].insert(b);
        vals[b].insert(a);
    }
    let mut lengths = vec![];
    for i in 0..n {
        lengths.push((vals[i].len(), i));
    }
    lengths.sort_unstable();
    lengths.reverse();
    let mut count = 0;
    // patern 1 : > 2
    if lengths[0].0 >= 2 {
        let idx = lengths[0].1;
        let mut set = BTreeSet::new();
        for &(a, b) in &AB {
            if a != idx && b != idx {
                set.insert((a, b));
            }
        }
        if set.len() == 0 {
            count += n - 1;
        } else if set.len() == 1 {
            count += 2;
        } else {
            let kouhos = set.pop_first().unwrap();
            let mut ok = false;
            for kouho in vec![kouhos.0, kouhos.1] {
                if set.iter().all(|(a, b)| *a == kouho || *b == kouho) {
                    ok = true;
                }
            }
            if ok {
                count += 1;
            } else {
                count += 0;
            }
        }
        if lengths[0].0 == 2 {
            let cands = vals[idx].iter().cloned().collect::<Vec<_>>();
            assert_eq!(cands.len(), 2);
            if set
                .iter()
                .all(|(a, b)| *a == cands[0] || *a == cands[1] || *b == cands[0] || *b == cands[1])
            {
                count += 1;
            }
        }
    } else {
        let mut set = BTreeSet::new();
        for &(a, b) in &AB {
            set.insert((a, b));
        }
        if set.len() == 1 {
            count += n * 2 - 3;
        } else if set.len() == 2 {
            count += 4;
        } else {
            count += 0;
        }
    }

    println!("{}", count);
}

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

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
