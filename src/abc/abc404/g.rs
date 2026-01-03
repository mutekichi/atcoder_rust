#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i64,
}

/// Weighted Graph Structure
///
/// A general-purpose weighted graph container that supports various shortest path and MST algorithms.
///
/// # Supported Algorithms
/// - **Dijkstra**: Single-Source Shortest Path (Non-negative weights). $O(E \log V)$
/// - **Bellman-Ford**: Single-Source Shortest Path (Negative weights allowed). Detects negative cycles. $O(V \times E)$
/// - **Warshall-Floyd**: All-Pairs Shortest Path. $O(V^3)$
/// - **Kruskal**: Minimum Spanning Tree (MST). $O(E \log E)$
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::graph::weighted::WeightedGraph;
///
/// // 1. Initialize
/// let n = 5;
/// let mut graph = WeightedGraph::new(n);
///
/// // 2. Add Edges
/// // Directed: 0 -> 1 (cost 10)
/// graph.add_edge(0, 1, 10);
/// // Undirected: 1 <-> 2 (cost 5)
/// graph.add_undirected_edge(1, 2, 5);
/// graph.add_edge(0, 2, 20);
/// graph.add_edge(2, 3, 10);
/// graph.add_edge(3, 4, 10);
///
/// // 3. Dijkstra (Single Source Shortest Path)
/// // Returns Vec<i64>. Unreachable nodes are INF.
/// // O(E log V)
/// let dist_dijkstra = graph.dijkstra(0);
/// assert_eq!(dist_dijkstra[1], 10);
/// assert_eq!(dist_dijkstra[2], 15); // 0 -> 1 -> 2 (10 + 5)
///
/// // 4. Bellman-Ford (Negative weights supported)
/// // Returns Option<Vec<i64>>. Returns None if a negative cycle is reachable.
/// // O(V * E)
/// if let Some(dist_bf) = graph.bellman_ford(0) {
///     assert_eq!(dist_bf[2], 15);
/// } else {
///     println!("Negative cycle detected!");
/// }
///
/// // 5. Warshall-Floyd (All Pairs Shortest Path)
/// // Returns Vec<Vec<i64>>.
/// // O(V^3)
/// let dist_matrix = graph.warshall_floyd();
/// assert_eq!(dist_matrix[0][2], 15);
/// assert_eq!(dist_matrix[1][0], 1i64 << 60); // 1 -> 0 is unreachable
///
/// // 6. Kruskal (Minimum Spanning Tree)
/// // Returns (total_cost, edges).
/// // O(E log E)
/// let (mst_cost, mst_edges) = graph.kruskal();
/// // MST for connected components.
/// // Edges: (1, 2, 5), (0, 1, 10), (2, 3, 10), (3, 4, 10) -> Total 35
/// assert_eq!(mst_cost, 35);
/// ```
pub struct WeightedGraph {
    n: usize,
    adj: Vec<Vec<(usize, i64)>>, // Adjacency list for Dijkstra/BFS-like traversal
    edges: Vec<Edge>,            // Edge list for Kruskal/Bellman-Ford
}

impl WeightedGraph {
    pub fn new(n: usize) -> Self {
        WeightedGraph {
            n,
            adj: vec![vec![]; n],
            edges: vec![],
        }
    }

    /// Adds a directed e
    pub fn add_edge(
        &mut self,
        u: usize,
        v: usize,
        weight: i64,
    ) {
        self.adj[u].push((v, weight));
        self.edges.push(Edge { u, v, weight });
    }

    /// Adds an undirected edge.
    pub fn add_undirected_edge(
        &mut self,
        u: usize,
        v: usize,
        weight: i64,
    ) {
        self.adj[u].push((v, weight));
        self.adj[v].push((u, weight));
        self.edges.push(Edge { u, v, weight }); // For Kruskal, typically one direction is enough or handle duplication
    }

    // ====================================================
    // 1. Dijkstra
    // ====================================================

    /// Runs Dijkstra's algorithm. O(E log V)
    pub fn dijkstra(
        &self,
        start: usize,
    ) -> Vec<i64> {
        let mut dist = vec![INF; self.n];
        let mut pq = BinaryHeap::new();

        dist[start] = 0;
        pq.push(Reverse((0, start)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }

            for &(v, w) in &self.adj[u] {
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }
        dist
    }

    // ====================================================
    // 2. Bellman-Ford
    // ====================================================

    /// Runs Bellman-Ford algorithm. O(V * E)
    /// Returns None if a negative cycle is reachable.
    pub fn bellman_ford(
        &self,
        start: usize,
    ) -> Option<Vec<i64>> {
        let mut dist = vec![INF; self.n];
        dist[start] = 0;

        for i in 0..self.n {
            let mut updated = false;
            for e in &self.edges {
                if dist[e.u] != INF && dist[e.u] + e.weight < dist[e.v] {
                    dist[e.v] = dist[e.u] + e.weight;
                    updated = true;
                    if i == self.n - 1 {
                        return None; // Negative cycle detected
                    }
                }
            }
            if !updated {
                break;
            }
        }
        Some(dist)
    }

    // ====================================================
    // 3. Warshall-Floyd
    // ====================================================

    /// Runs Warshall-Floyd algorithm. O(V^3)
    /// Returns a 2D vector of shortest paths.
    pub fn warshall_floyd(&self) -> Vec<Vec<i64>> {
        let mut dist = vec![vec![INF; self.n]; self.n];
        for i in 0..self.n {
            dist[i][i] = 0;
        }

        // Initialize from adjacency list
        for u in 0..self.n {
            for &(v, w) in &self.adj[u] {
                dist[u][v] = dist[u][v].min(w);
            }
        }

        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    if dist[i][k] != INF && dist[k][j] != INF {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }
        dist
    }

    // ====================================================
    // 4. Kruskal (MST)
    // ====================================================

    /// Runs Kruskal's algorithm. O(E log E)
    /// Returns (total_cost, mst_edges).
    pub fn kruskal(&self) -> (i64, Vec<Edge>) {
        let mut edges = self.edges.clone();
        edges.sort_by_key(|e| e.weight);

        let mut uf = UnionFind::new(self.n);
        let mut total_cost = 0;
        let mut mst_edges = vec![];

        for e in edges {
            if !uf.same(e.u, e.v) {
                uf.unite(e.u, e.v);
                total_cost += e.weight;
                mst_edges.push(e);
            }
        }
        (total_cost, mst_edges)
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// Creates a new Union-Find structure with `n` elements (0 to n-1).
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    /// Finds the root of the element `x` with path compression.
    pub fn find(
        &mut self,
        x: usize,
    ) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
            self.parent[x]
        }
    }

    /// Unites the sets containing elements `x` and `y`.
    pub fn unite(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            if self.rank[root_x] == self.rank[root_y] {
                self.rank[root_x] += 1;
            }
        }
        true
    }

    /// Returns the size of the set containing element `x`.
    pub fn size(
        &mut self,
        x: usize,
    ) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// Checks if elements `x` and `y` are in the same set.
    pub fn same(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        self.find(x) == self.find(y)
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
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        m: usize,
        LRS: [(usize, usize, i64); m],
    }
    let mut graph = WeightedGraph::new(n + 1);
    for i in 0..n {
        graph.add_edge(i + 1, i, -1);
    }
    for (l, r, s) in LRS {
        graph.add_edge(r, l - 1, -s);
        graph.add_edge(l - 1, r, s);
    }
    if let Some(min_dists) = graph.bellman_ford(n) {
        wl!(-min_dists[0]);
    } else {
        wl!(-1);
    }
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
