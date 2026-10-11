#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use memoise::memoise;
use num_integer::gcd;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::btree_map::Entry;
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize, t: usize, m: usize,
        AB: [(Usize1, Usize1); m],
    }
    let mut peaceful = vec![vec![true; n]; n];
    for (a, b) in AB {
        peaceful[a][b] = false;
        peaceful[b][a] = false;
    }
    let mut teams = vec![vec![]; t];
    let mut ans = 0;
    f(n, 0, &peaceful, &mut teams, &mut ans);
    println!("{}", ans);
}

fn f(
    n: usize,
    to_add: usize,
    peaceful: &Vec<Vec<bool>>,
    teams: &mut Vec<Vec<usize>>,
    ans: &mut usize,
) {
    if to_add == n {
        if teams.last().unwrap().len() > 0 {
            *ans += 1;
        }
        return;
    }
    for i in 0..teams.len() {
        if !teams[i].is_empty() {
            if teams[i].iter().all(|&p| peaceful[to_add][p]) {
                teams[i].push(to_add);
                f(n, to_add + 1, peaceful, teams, ans);
                teams[i].pop();
            }
        } else {
            teams[i].push(to_add);
            f(n, to_add + 1, peaceful, teams, ans);
            teams[i].pop();
            break;
        }
    }
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[rustfmt::skip]
const DIR8: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
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

trait AsciiExt {
    fn to_idx(self) -> usize;
}

impl AsciiExt for char {
    fn to_idx(self) -> usize {
        (self as u8 - b'a') as usize
    }
}

impl AsciiExt for u8 {
    fn to_idx(self) -> usize {
        (self - b'a') as usize
    }
}

trait UsizeExt {
    fn to_char(self) -> char;
}

impl UsizeExt for usize {
    fn to_char(self) -> char {
        (self as u8 + b'a') as char
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
