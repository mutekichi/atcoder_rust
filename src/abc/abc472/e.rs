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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            AB: [(Usize1, Usize1); m],
        }
        let mut colors = vec![false; n];
        let mut seen = vec![false; n];
        let mut graph = vec![vec![]; n];
        for (a, b) in AB {
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut stack = vec![];
        let mut ans = None;
        dfs(
            0,
            INF_USIZE,
            true,
            &mut graph,
            &mut colors,
            &mut seen,
            &mut stack,
            &mut ans,
        );
        if ans.is_some() {
            let ans = ans.unwrap();
            println!("{}", ans.len());
            println!("{}", ans.iter().map(|&v| v + 1).join(" "));
        } else {
            println!("{}", -1);
        }
    }
}

fn dfs(
    v: usize,
    from: usize,
    color: bool,
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<bool>,
    seen: &mut Vec<bool>,
    stack: &mut Vec<usize>,
    ans: &mut Option<Vec<usize>>,
) {
    md!(v, stack.iter().join(" "));
    if seen[v] {
        return;
    }
    if ans.is_some() {
        return;
    }
    seen[v] = true;
    colors[v] = color;
    stack.push(v);
    for &nv in graph[v].iter() {
        if nv == from {
            continue;
        }
        if seen[nv] && colors[nv] == colors[v] {
            md!(stack.iter().join(" "), v, nv);
            if ans.is_none() {
                md!("here");
                let mut seen = vec![false; seen.len()];
                let mut vec = vec![];
                seen[nv] = true;
                vec.push(nv);
                while let Some(v) = stack.pop() {
                    md!(v);
                    if !seen[v] {
                        vec.push(v);
                        seen[v] = true;
                    } else {
                        *ans =
                            Some(vec.iter().cloned().collect::<Vec<_>>());
                        return;
                    }
                }
                return;
            }
            return;
        }
        if !seen[nv] {
            dfs(nv, v, !color, graph, colors, seen, stack, ans);
        }
    }
    stack.pop();
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
