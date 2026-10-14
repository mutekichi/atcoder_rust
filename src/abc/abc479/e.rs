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
        n: usize,
        AB: [(Usize1, Usize1); n - 1],
    }
    let mut tree = vec![vec![]; n];
    for i in 0..n - 1 {
        let (a, b) = AB[i];
        tree[a].push((b, i + 1));
        tree[b].push((a, i + 1));
    }
    let mut seen = vec![false; n];
    let mut degs = vec![INF_USIZE; n];
    let mut new_to_orig = vec![];
    dfs(0, INF_USIZE, &tree, &mut seen, &mut new_to_orig, &mut degs);
    let mut st = vec![0isize; n + 1];
    md!(new_to_orig.iter().join(" "));
    md!(degs.iter().join(" "));

    let mut base = 0;

    let mut stack = vec![];
    let mut seen = vec![false; n];
    stack.push(0);
    seen[0] = true;
    let mut orig_to_new = vec![INF_USIZE; n];
    for i in 0..n {
        orig_to_new[new_to_orig[i]] = i;
    }
    while let Some(i) = stack.pop() {
        for &(v, cost) in &tree[i] {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            let nv = orig_to_new[v]; // v: orig nv: to
            let deg = degs[v];
            md!(v, nv, deg);

            if deg > n / 2 {
                md!("high");
                st[nv] += cost as isize;
                st[nv + deg] -= cost as isize;
            } else {
                md!("low");
                base += cost as isize;
                st[nv] -= cost as isize;
                st[nv + deg] += cost as isize;
            }
            stack.push(v);
        }
    }
    for i in 0..n {
        st[i + 1] += st[i];
    }
    md!(base);
    md!(st.iter().join(" "));

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = base as isize + st[orig_to_new[i]];
    }
    println!("{}", ans.iter().join(" "));
}

fn dfs(
    v: usize,
    from: usize,
    tree: &Vec<Vec<(usize, usize)>>,
    seen: &mut Vec<bool>,
    new_to_orig_: &mut Vec<usize>,
    degs: &mut Vec<usize>,
) -> usize {
    if !seen[v] {
        new_to_orig_.push(v);
        seen[v] = true;
    }
    let mut deg = 1;
    for &(nv, _) in &tree[v] {
        if nv != from {
            deg += dfs(nv, v, tree, seen, new_to_orig_, degs);
        }
    }
    degs[v] = deg;
    deg
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
