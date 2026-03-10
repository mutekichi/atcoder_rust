#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
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
        A: [i64; n],
        UV: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (u, v) in UV {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut ans = vec![false; n];
    let mut counter: BTreeMap<_, _> = BTreeMap::new();
    dfs(0, INF_USIZE, &mut ans, &mut counter, false, &A, &graph);

    for ans in ans {
        if ans {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}

fn dfs(
    v: usize,
    from: usize,
    ans: &mut Vec<bool>,
    counter: &mut BTreeMap<i64, usize>,
    yes: bool,
    A: &Vec<i64>,
    graph: &Vec<Vec<usize>>,
) {
    let mut next_yes = true;
    if yes {
        ans[v] = true;
    }
    else {
        if counter.contains_key(&A[v]) {
            ans[v] = true;
        }
        else {
            next_yes = false;
            ans[v] = false;
        }
    }
        *counter.entry(A[v]).or_insert(0) += 1;
    for &nv in &graph[v] {
        if nv != from {
            dfs(
                nv,
                v,
                ans,
                counter,
                next_yes,
                A,
                graph
            );
        }
    }
    if counter[&A[v]] > 1 {
        *counter.get_mut(&A[v]).unwrap() -= 1;
    } else {
        counter.remove_entry(&A[v]);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
