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

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            n: usize,
            AB: [(Usize1, Usize1); n - 1],
        }
        let mut graph = vec![vec![]; n];
        for (a, b) in AB {
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut ans = 1usize;
        dfs(INF_USIZE, 0, &graph, &mut ans);
        println!("{}", ans);
    }
}

fn dfs(
    from: usize,
    v: usize,
    graph: &Vec<Vec<usize>>,
    ans: &mut usize,
) -> usize {
    let mut val = 1;
    let mut vec = BinaryHeap::new();

    for &nv in &graph[v] {
        if nv == from {
            continue;
        }
        let val = dfs(v, nv, graph, ans);
        vec.push(val);
    }
    md!(v, vec.len());
    if vec.len() < 2 {
        val = 0;
    } else {
        let first = vec.pop().unwrap();
        let second = vec.pop().unwrap();
        let has_third = vec.pop().is_some();
        if from != INF_USIZE {
            if has_third {
            *ans = max(*ans, first + second + 1);
            }
            else {
                *ans = max(*ans, first + 1);
            }
            if has_third {
                val = first + 1;
            }
        } else {
            let has_fourth = vec.pop().is_some();
            if has_third && has_fourth {
                *ans = max(*ans, first + second + 1);
            }
            else if has_third {
                *ans = max(*ans, first + 1);
            }
        }
    }
    md!(v, val, ans);
    val
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
