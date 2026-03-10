#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::alloc::handle_alloc_error;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
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
        k: usize,
        UV: [(Usize1, Usize1); m],
    }
    if k % 2 == 1 {
        println!("No");
        return;
    }
    let mut graph = vec![vec![]; n];
    let mut map = BTreeMap::new();
    for i in 0..m {
        let (u, v) = UV[i];
        graph[u].push(v);
        graph[v].push(u);
        map.insert((u.min(v), u.max(v)), i + 1);
    }
    let mut ans = vec![];
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        else {
            seen[i] = true;
            dfs(i, INF_USIZE, &graph, &mut seen, &mut ans);
        }
    }
    let mut state = vec![false; n];
    let mut count = 0;
    let mut idx = 0;
    while count < k {
        if idx >= ans.len() {
            println!("No");
            return;
        }
        let (u, v) = ans[idx];
        for u in vec![u, v] {
            if state[u] {
                count -= 1;
                state[u] = false;
            }
            else {
                count += 1;
                state[u] = true;
            }
        }
        idx += 1;
    }
    println!("Yes");
    println!("{}", idx);
    println!("{}", ans.iter().take(idx).map(|e| map[e]).join(" "));

}

fn dfs(
    v: usize,
    from: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    ans: &mut Vec<(usize, usize)>,
) -> bool {
    let mut accum = true;
    for &nv in &graph[v] {
        if seen[nv] || nv == from {
            continue;
        }
        seen[nv] = true;
        accum ^= dfs(
            nv,
            v,
            graph,
            seen,
            ans,
        );
    }
    if accum && from != INF_USIZE {
        ans.push((v.min(from), v.max(from)));
    }
    accum
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
