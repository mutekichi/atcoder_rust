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
        m: usize,
        UV: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }
    let mut graph = vec![vec![]; 9];
    for (u, v) in UV {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut state = vec![8; 9];
    let mut empty = INF_USIZE;
    for i in 0..8 {
        state[p[i]] = i;
    }
    for i in 0..9 {
        if state[i] == 8 {
            empty = i;
        }
    }
    let ans = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ans_code = encode(&ans);
    if encode(&state) == ans_code {
        println!("0");
        return;
    }
    let mut seen = BTreeSet::new();
    seen.insert(encode(&state));
    let mut queue = VecDeque::new();
    queue.push_back((state, empty, 0));

    md!(empty);

    while let Some((state, empty, step)) = queue.pop_front() {
        for &nv in &graph[empty] {
            let mut next_state = state.clone();
            next_state[empty] = next_state[nv];
            next_state[nv] = 8;
            let next_code = encode(&next_state);
            if !seen.contains(&next_code) {
                if next_code == ans_code {
                    println!("{}", step + 1);
                    return;
                }
                seen.insert(next_code);
                queue.push_back((next_state, nv, step + 1));
            }
        }
    }
    println!("{}", -1);
}

fn encode(p: &Vec<usize>) -> usize {
    let mut code = 0;
    for &v in p {
        code += v;
        code *= 9;
    }
    code
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
