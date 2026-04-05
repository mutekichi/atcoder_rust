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
        A: [i64; 1<<n],
    }
    md!(A.len());
    let mut players = (0..(1 << n)).zip(A.into_iter()).collect::<Vec<_>>();
    let mut next = vec![];
    md!(players.len());
    while players.len() > 2 {
        for i in 0..players.len() / 2 {
            md!(i);
            if players[2 * i].1 < players[2 * i + 1].1 {
                next.push(players[2 * i + 1]);
                md!(2 * i + 1);
                md!(players[2 * i + 1].0)
            } else {
                next.push(players[2 * i]);
                md!(2 * i);
                md!(players[2 * i].0)
            }
        }
        swap(&mut players, &mut next);
        next.clear();
    }
    md!(players[0].0, players[1].0);
    if players[0].1 < players[1].1 {
        println!("{}", players[0].0 + 1);
    } else {
        println!("{}", players[1].0 + 1);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
