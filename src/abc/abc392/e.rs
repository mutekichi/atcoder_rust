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
use std::mem;
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
        n: usize,
        m: usize,
        AB: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (a, b) = AB[i];
        graph[a].push((b, i));
        graph[b].push((a, i));
    }

    let mut ans = vec![];
    let mut alones = vec![];
    let mut used = vec![false; m];

    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut queue = vec![];
        seen[i] = true;
        queue.push(i);
        let mut temp_rem_cables = vec![];
        while let Some(v) = queue.pop() {
            for &(nv, idx) in &graph[v] {
                md!(nv, idx);
                if seen[nv] {
                    if !used[idx] {
                        temp_rem_cables.push(idx);
                    }
                } else {
                    seen[nv] = true;
                    queue.push(nv);
                }
                used[idx] = true;
            }
        }
        alones.push((i, temp_rem_cables));
    }
    alones.sort_unstable_by(|a, b| b.1.len().cmp(&a.1.len()));

    let mut deque = VecDeque::new();
    for alone in alones {
        deque.push_back(alone);
    }

    while deque.len() > 1 {
        let (i, cables) = deque.pop_front().unwrap();
        for cable in cables {
            if let Some((j, _)) = deque.pop_back() {
                ans.push((cable, j));
            }
        }
        deque.push_back((i, vec![]));
    }

    println!(
        "{}\n{}",
        ans.len(),
        ans.iter()
            .map(|(cable, alone)| {
                vec![cable + 1, AB[*cable].0 + 1, alone + 1]
                    .iter()
                    .join(" ")
            })
            .join("\n")
    );
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
