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

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    input! {
        n: usize,
        AB: [(Usize1, Usize1); n - 1],
        C: [i128; n],
    }
    let (graph, mut costs) = {
        let mut relabel_list = vec![INF_USIZE; n];
        let mut graph = vec![vec![]; n];
        for &(a, b) in &AB {
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut seen = vec![false; n];
        let mut stack = VecDeque::new();
        stack.push_back(0);
        seen[0] = true;
        let mut relabel_index = 0;
        while let Some(v) = stack.pop_back() {
            relabel_list[v] = relabel_index;
            for &nv in &graph[v] {
                if !seen[nv] {
                    stack.push_back(nv);
                    seen[nv] = true;
                }
            }
            relabel_index += 1;
        }
        md!(relabel_list.iter().join(" "));
        let mut graph = vec![vec![]; n];
        let mut costs = vec![INF_I128; n];
        for (a, b) in AB {
            let aa = relabel_list[a];
            let bb = relabel_list[b];
            graph[aa].push(bb);
            graph[bb].push(aa);
        }
        for i in 0..n {
            graph[i].sort_unstable();
            costs[relabel_list[i]] = C[i];
        }
        (graph, costs)
    };
    let mut next_list = vec![INF_USIZE; n];
    let mut time = 0;
    let cost = dfs_cost(0, INF_USIZE, &graph, &costs, 0, &mut time, &mut next_list);
    md!(cost);
    let mut ans = INF_I128;
    let sum: i128 = costs.iter().sum();
    costs.insert(0, 0);
    for i in 0..n {
        costs[i + 1] += costs[i];
    }
    dfs_main(
        0, INF_USIZE, &graph, &costs, &next_list, cost, sum, &mut ans,
    );
    println!("{}", ans);
}
fn dfs_main(
    v: usize,
    from: usize,
    graph: &Vec<Vec<usize>>,
    costs: &Vec<i128>,
    next_list: &Vec<usize>,
    cost: i128,
    sum: i128,
    ans: &mut i128,
) {
    md!(v, cost);
    *ans = min(cost, *ans);
    for &nv in &graph[v] {
        if nv != from {
            let to_sub = costs[next_list[nv]] - costs[nv];
            let next_cost = cost + sum - to_sub * 2;
            dfs_main(nv, v, graph, costs, next_list, next_cost, sum, ans);
        }
    }
}
fn dfs_cost(
    v: usize,
    from: usize,
    graph: &Vec<Vec<usize>>,
    costs: &Vec<i128>,
    deg: usize,
    time: &mut usize,
    next_list: &mut Vec<usize>,
) -> i128 {
    let mut sum = 0;
    md!(v, costs[v], deg);
    sum += costs[v] * (deg as i128);
    *time += 1;
    eprintln!("{}", v);
    for &nv in &graph[v] {
        if nv != from {
            sum += dfs_cost(nv, v, graph, costs, deg + 1, time, next_list);
        }
    }
    next_list[v] = *time;
    sum
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
