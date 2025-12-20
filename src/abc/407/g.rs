#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 62;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// FOR TEMPLATE INJECTIONS

/// Returns valid neighbor coordinates within the grid (h x w).
/// Usage:
/// ```
/// for (nh, nw) in get_next_positions(h, w, hh, ww, &DIR) {
///     // process (nh, nw)
/// }
/// ```
fn get_next_positions(
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<(usize, usize)> {
    let mut next_positions = Vec::with_capacity(directions.len());

    for &(di, dj) in directions {
        let next_i = i.wrapping_add_signed(di);
        let next_j = j.wrapping_add_signed(dj);
        if next_i < h && next_j < w {
            next_positions.push((next_i, next_j));
        }
    }
    next_positions
}

const INF_COST: i64 = 1 << 60;

/// Min Cost Flow (Primal-Dual Algorithm)
///
/// Solves the minimum cost s-t flow problem using Dijkstra's algorithm with potentials.
///
/// # Requirements
/// - Edge weights (costs) must be non-negative initially.
/// - If negative costs are present, use Bellman-Ford to initialize potentials (not implemented here).
///
/// # Complexity
/// - O(F E log V) where F is the amount of flow.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::graph::min_cost_flow::MinCostFlow;
///
/// let mut mcf = MinCostFlow::new(4);
/// // add_edge(u, v, capacity, cost)
/// mcf.add_edge(0, 1, 2, 1);
/// mcf.add_edge(0, 2, 1, 2);
/// mcf.add_edge(1, 2, 1, 1);
/// mcf.add_edge(1, 3, 1, 3);
/// mcf.add_edge(2, 3, 2, 1);
///
/// // Calculate min cost to flow 2 units from 0 to 3
/// let (flow, cost) = mcf.min_cost_flow(0, 3, 2);
/// assert_eq!(flow, 2);
/// assert_eq!(cost, 6); // Path 0->1->2->3 (cost 1+1+1=3), Path 0->2->3 (cost 2+1=3) -> Total 6
/// ```
#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize, // Index of reverse edge
}

pub struct MinCostFlow {
    n: usize,
    graph: Vec<Vec<Edge>>,
    h: Vec<i64>,        // Potential
    dist: Vec<i64>,     // Shortest distance
    prev_v: Vec<usize>, // Previous vertex in shortest path
    prev_e: Vec<usize>, // Previous edge index in shortest path
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        MinCostFlow {
            n,
            graph: vec![vec![]; n],
            h: vec![0; n],
            dist: vec![0; n],
            prev_v: vec![0; n],
            prev_e: vec![0; n],
        }
    }

    /// Adds a directed edge with capacity `cap` and cost `cost`.
    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
        cap: i64,
        cost: i64,
    ) {
        let rev_idx = self.graph[to].len();
        let fwd_idx = self.graph[from].len();

        self.graph[from].push(Edge {
            to,
            cap,
            cost,
            rev: rev_idx,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            cost: -cost,
            rev: fwd_idx,
        });
    }

    /// Calculates the minimum cost to flow `f` amount from `s` to `t`.
    /// Returns `(flow, cost)`. If max flow < f, returns the max flow and its cost.
    pub fn min_cost_flow(
        &mut self,
        s: usize,
        t: usize,
        mut f: i64,
    ) -> (i64, i64) {
        let mut res_cost = 0;
        let mut total_flow = 0;

        // Initialize potential h with 0 (Assuming no negative costs initially)
        self.h = vec![0; self.n];

        while f > 0 {
            // Dijkstra to update potentials
            let mut pq = BinaryHeap::new();
            self.dist = vec![INF_COST; self.n];
            self.dist[s] = 0;
            pq.push(Reverse((0, s)));

            while let Some(Reverse((d, v))) = pq.pop() {
                if d > self.dist[v] {
                    continue;
                }

                for (i, e) in self.graph[v].iter().enumerate() {
                    if e.cap > 0
                        && self.dist[e.to] > self.dist[v] + e.cost + self.h[v] - self.h[e.to]
                    {
                        self.dist[e.to] = self.dist[v] + e.cost + self.h[v] - self.h[e.to];
                        self.prev_v[e.to] = v;
                        self.prev_e[e.to] = i;
                        pq.push(Reverse((self.dist[e.to], e.to)));
                    }
                }
            }

            // If target is unreachable, we can't flow anymore
            if self.dist[t] == INF_COST {
                break;
            }

            // Update potentials
            for v in 0..self.n {
                if self.dist[v] != INF_COST {
                    self.h[v] += self.dist[v];
                }
            }

            // Flow along the shortest path
            let mut d = f;
            let mut v = t;
            while v != s {
                let pv = self.prev_v[v];
                let pe = self.prev_e[v];
                d = min(d, self.graph[pv][pe].cap);
                v = pv;
            }

            f -= d;
            total_flow += d;
            res_cost += d * self.h[t];

            let mut v = t;
            while v != s {
                let pv = self.prev_v[v];
                let pe = self.prev_e[v];

                self.graph[pv][pe].cap -= d;
                let rev_idx = self.graph[pv][pe].rev;
                self.graph[v][rev_idx].cap += d;

                v = pv;
            }
        }

        (total_flow, res_cost)
    }

    /// Calculates the cost slope for increasing flow from `s` to `t`.
    /// Returns a vector of (flow, cost) points representing the cost function.
    pub fn min_cost_slope(
        &mut self,
        s: usize,
        t: usize,
    ) -> Vec<(i64, i64)> {
        let mut res_cost = 0;
        let mut total_flow = 0;
        let mut result = vec![(0, 0)];

        self.h = vec![0; self.n];

        loop {
            let mut pq = BinaryHeap::new();
            self.dist = vec![INF_COST; self.n];
            self.dist[s] = 0;
            pq.push(Reverse((0, s)));

            while let Some(Reverse((d, v))) = pq.pop() {
                if d > self.dist[v] {
                    continue;
                }

                for (i, e) in self.graph[v].iter().enumerate() {
                    if e.cap > 0
                        && self.dist[e.to] > self.dist[v] + e.cost + self.h[v] - self.h[e.to]
                    {
                        self.dist[e.to] = self.dist[v] + e.cost + self.h[v] - self.h[e.to];
                        self.prev_v[e.to] = v;
                        self.prev_e[e.to] = i;
                        pq.push(Reverse((self.dist[e.to], e.to)));
                    }
                }
            }

            if self.dist[t] == INF_COST {
                break;
            }

            for v in 0..self.n {
                if self.dist[v] != INF_COST {
                    self.h[v] += self.dist[v];
                }
            }

            let mut d = i64::MAX;
            let mut v = t;
            while v != s {
                let pv = self.prev_v[v];
                let pe = self.prev_e[v];
                d = min(d, self.graph[pv][pe].cap);
                v = pv;
            }

            total_flow += d;
            res_cost += d * self.h[t];

            // If the slope is same as previous, update the last point to minimize result size
            if result.len() >= 2 {
                let (f1, c1) = result[result.len() - 1];
                let (f0, c0) = result[result.len() - 2];
                if (res_cost - c1) * (f1 - f0) == (c1 - c0) * (total_flow - f1) {
                    result.pop();
                }
            }
            result.push((total_flow, res_cost));

            let mut v = t;
            while v != s {
                let pv = self.prev_v[v];
                let pe = self.prev_e[v];

                self.graph[pv][pe].cap -= d;
                let rev_idx = self.graph[pv][pe].rev;
                self.graph[v][rev_idx].cap += d;

                v = pv;
            }
        }
        result
    }
}

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        h: usize,
        w: usize,
        mut A: [[i64; w]; h],
    }
    let TEN12 = 10i64.pow(12) + 5i64;
    
    let mut mcf = MinCostFlow::new(2 * h * w + 2);
    let mut sum = 0;
    for i in 0..h {
        for j in 0..w {
            sum += A[i][j];
            A[i][j] += TEN12;
        }
    }

    for ih in 0..h {
        for iw in 0..w {
            for (nh, nw) in get_next_positions(h, w, ih, iw, &DIR) {
                if (ih + iw) % 2 == 0 {
                    let cost = A[ih][iw] + A[nh][nw];
                    mcf.add_edge(ih * w + iw, h * w + nh * w + nw, 1, cost);
                    md!(ih, iw, nh, nw);
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            mcf.add_edge(h * w * 2, i * w + j, 1, 0);
            mcf.add_edge(h * w + i * w + j, h * w * 2 + 1, 1, 0);
        }
    }
    let min_cost_flow_slope = mcf.min_cost_slope(h * w * 2, h * w * 2 + 1);

    let mut before_flow = 0;
    let mut before_cost = 0;

    let mut min_flow = INF_I64;

    for (flow, cost) in min_cost_flow_slope {
        md!(flow, cost);
        if flow == 0 {
            before_cost = cost;
            continue;
        }
        let cost_step = (cost - before_cost) / (flow - before_flow);
        for f in before_flow..=flow {
            let current_cost = before_cost + cost_step * (f - before_flow);
            md!(f, current_cost - 2 * f * TEN12);
            min_flow = min(min_flow, current_cost - 2 * f * TEN12);
        }
        before_flow = flow;
        before_cost = cost;
    }
    md!(sum);
    wl!(sum - if min_flow == INF_I64 { 0 } else { min_flow });
}

// --- Macros ---

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

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint without newline)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mep {
    ($x:expr) => { eprint!("{}", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mep {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint with space)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mepw { // stands for my_eprint_whitespace
    ($x:expr) => { eprint!("{} ", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mepw {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

fn join_with_space<T: ToString>(arr: &[T]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
