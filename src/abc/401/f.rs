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

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

/// Tree Utilities: Diameter, Centroid, LCA (Lowest Common Ancestor)
///
/// A versatile Tree structure supporting:
/// - Tree Diameter
/// - Tree Centroids
/// - Lowest Common Ancestor (LCA) via Binary Lifting
/// - Distance between two nodes
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::graph::tree::Tree;
///
/// let mut tree = Tree::new(5);
/// tree.add_edge(0, 1, 1);
/// tree.add_edge(1, 2, 1);
/// tree.add_edge(1, 3, 1);
/// tree.add_edge(3, 4, 1);
///
/// // 1. Diameter
/// let (len, u, v) = tree.diameter();
/// assert_eq!(len, 3); // Path: 2-1-3-4 or 0-1-3-4
///
/// // 2. Centroid
/// let centers = tree.centroids();
/// assert!(centers.contains(&1));
///
/// // 3. LCA & Distance (Requires build_lca(root))
/// tree.build_lca(0);
/// assert_eq!(tree.lca(2, 4), 1);
/// assert_eq!(tree.dist(2, 4), 3);
/// ```
#[derive(Debug, Clone)]
pub struct Tree {
    n: usize,
    edges: Vec<Vec<(usize, i64)>>, // (to, cost)
    // For LCA
    root: usize,
    parent: Vec<Vec<Option<usize>>>, // parent[k][u] = 2^k-th parent of u
    depth: Vec<usize>,
    dist_from_root: Vec<i64>,
    lca_ready: bool,
}

impl Tree {
    /// Creates a new Tree with `n` nodes (0 to n-1).
    pub fn new(n: usize) -> Self {
        Tree {
            n,
            edges: vec![vec![]; n],
            root: 0,
            parent: vec![],
            depth: vec![],
            dist_from_root: vec![],
            lca_ready: false,
        }
    }

    /// Adds an undirected edge between `u` and `v` with weight `w`.
    /// For unweighted trees, use `w = 1`.
    pub fn add_edge(
        &mut self,
        u: usize,
        v: usize,
        w: i64,
    ) {
        self.edges[u].push((v, w));
        self.edges[v].push((u, w));
    }

    /// Adds a directed edge from `u` to `v` with weight `w`.
    pub fn add_directed_edge(
        &mut self,
        u: usize,
        v: usize,
        w: i64,
    ) {
        self.edges[u].push((v, w));
    }

    // ====================================================
    // 1. Tree Diameter
    // ====================================================

    /// Calculates the diameter of the tree.
    ///
    /// # Returns
    /// `(diameter_length, u, v)` where `u` and `v` are the endpoints of the diameter.
    ///
    /// # Complexity
    /// - O(N)
    pub fn diameter(&self) -> (i64, usize, usize) {
        if self.n == 0 {
            return (0, 0, 0);
        }
        // 1st DFS: Find farthest node from an arbitrary node (0)
        let r = self.bfs_farthest(0);
        // 2nd DFS: Find farthest node from r
        let (dist, v) = self.bfs_dist(r);
        (dist[v], r, v)
    }

    fn bfs_farthest(
        &self,
        start: usize,
    ) -> usize {
        let (dist, _) = self.bfs_dist(start);
        dist.iter()
            .enumerate()
            .max_by_key(|&(_, &d)| d)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn bfs_dist(
        &self,
        start: usize,
    ) -> (Vec<i64>, usize) {
        let mut dist = vec![-1; self.n];
        let mut queue = std::collections::VecDeque::new();
        dist[start] = 0;
        queue.push_back(start);

        let mut farthest_node = start;
        let mut max_dist = 0;

        while let Some(u) = queue.pop_front() {
            if dist[u] > max_dist {
                max_dist = dist[u];
                farthest_node = u;
            }

            for &(v, w) in &self.edges[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + w;
                    queue.push_back(v);
                }
            }
        }
        (dist, farthest_node)
    }

    // ====================================================
    // 2. Tree Centroid
    // ====================================================

    /// Finds the centroid(s) of the tree.
    /// A centroid is a node such that when removed, the size of the largest connected component
    /// is at most N/2. A tree has 1 or 2 centroids.
    ///
    /// # Returns
    /// A vector containing the centroid(s).
    ///
    /// # Complexity
    /// - O(N)
    pub fn centroids(&self) -> Vec<usize> {
        let mut centroid = vec![];
        let mut sz = vec![0; self.n];
        self.dfs_centroid(0, self.n, &mut sz, &mut centroid);
        centroid
    }

    fn dfs_centroid(
        &self,
        u: usize,
        prev: usize,
        sz: &mut Vec<usize>,
        centroid: &mut Vec<usize>,
    ) {
        sz[u] = 1;
        let mut is_centroid = true;
        for &(v, _) in &self.edges[u] {
            if v != prev {
                self.dfs_centroid(v, u, sz, centroid);
                sz[u] += sz[v];
                if sz[v] > self.n / 2 {
                    is_centroid = false;
                }
            }
        }
        if self.n - sz[u] > self.n / 2 {
            is_centroid = false;
        }
        if is_centroid {
            centroid.push(u);
        }
    }

    // ====================================================
    // 3. LCA (Lowest Common Ancestor) & Distance
    // ====================================================

    /// Precomputes data structures for LCA queries using Binary Lifting.
    /// Must be called before `lca()` or `dist()`.
    ///
    /// # Arguments
    /// - `root`: The root of the tree.
    ///
    /// # Complexity
    /// - O(N log N)
    pub fn build_lca(
        &mut self,
        root: usize,
    ) {
        self.root = root;
        let log_n = (self.n as f64).log2().ceil() as usize;
        let log_n = if log_n == 0 { 1 } else { log_n };

        self.parent = vec![vec![None; self.n]; log_n + 1];
        self.depth = vec![0; self.n];
        self.dist_from_root = vec![0; self.n];

        let mut stack = vec![(root, None::<usize>, 0, 0)]; // u, p, d, dist

        // Iterative DFS
        while let Some((u, p, d, dist)) = stack.pop() {
            self.parent[0][u] = p;
            self.depth[u] = d;
            self.dist_from_root[u] = dist;

            for &(v, w) in &self.edges[u] {
                if Some(v) != p {
                    stack.push((v, Some(u), d + 1, dist + w));
                }
            }
        }

        // Doubling
        for k in 0..log_n {
            for v in 0..self.n {
                if let Some(p) = self.parent[k][v] {
                    self.parent[k + 1][v] = self.parent[k][p];
                }
            }
        }
        self.lca_ready = true;
    }

    /// Finds the Lowest Common Ancestor of nodes `u` and `v`.
    ///
    /// # Panics
    /// Panics if `build_lca` has not been called.
    ///
    /// # Complexity
    /// - O(log N)
    pub fn lca(
        &self,
        mut u: usize,
        mut v: usize,
    ) -> usize {
        assert!(self.lca_ready, "LCA not built. Call build_lca(root) first.");

        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // Lift v to the same depth as u
        let log_n = self.parent.len();
        for k in 0..log_n {
            if ((self.depth[v] - self.depth[u]) >> k) & 1 == 1 {
                v = self.parent[k][v].unwrap();
            }
        }

        if u == v {
            return u;
        }

        // Lift both u and v
        for k in (0..log_n).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u].unwrap();
                v = self.parent[k][v].unwrap();
            }
        }

        self.parent[0][u].unwrap()
    }

    /// Calculates the distance between nodes `u` and `v`.
    /// Supports weighted edges.
    ///
    /// # Complexity
    /// - O(log N)
    pub fn dist(
        &self,
        u: usize,
        v: usize,
    ) -> i64 {
        let lca = self.lca(u, v);
        self.dist_from_root[u] + self.dist_from_root[v] - 2 * self.dist_from_root[lca]
    }

    /// Returns the distance (number of edges) between nodes `u` and `v`.
    ///
    /// # Complexity
    /// - O(log N)
    pub fn depth_dist(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        let lca = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[lca]
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
        n1: usize,
        UV1: [(Usize1, Usize1); n1 - 1],
        n2: usize,
        UV2: [(Usize1, Usize1); n2 - 1],
    }
    let mut tree1 = Tree::new(n1);
    for (u, v) in UV1 {
        tree1.add_edge(u, v, 1);
    }
    let (diameter1, end1, end2) = tree1.diameter();
    let distances1 = {
        let (distances_end1, _) = tree1.bfs_dist(end1);
        let (distances_end2, _) = tree1.bfs_dist(end2);
        let distances = (0..n1)
            .map(|i| max(distances_end1[i], distances_end2[i]))
            .collect::<Vec<_>>();
        distances
    };
    let distances2 = {
        let mut tree = Tree::new(n2);
        for (u, v) in UV2 {
            tree.add_edge(u, v, 1);
        }
        let (diameter, end1, end2) = tree.diameter();
        let (distances_end1, _) = tree.bfs_dist(end1);
        let (distances_end2, _) = tree.bfs_dist(end2);
        let distances = (0..n2)
            .map(|i| max(distances_end1[i], distances_end2[i]))
            .collect::<Vec<_>>();
        distances
    };
    let mut accum_count = vec![0usize; 200004];
    let mut accum_dist = vec![0i64; 200004];
    for &distance in &distances2 {
        accum_count[distance as usize] += 1;
        accum_dist[distance as usize] += distance;
    }
    for i in 1..accum_count.len() {
        accum_count[i] += accum_count[i - 1];
        accum_dist[i] += accum_dist[i - 1];
    }
    let total_dist = distances2.iter().sum::<i64>();

    let mut ans = 0i64;

    for i in 0..n1 {
        let distance1 = distances1[i];
        md!(distance1, i + 1);
        let diff = diameter1 - distance1;
        let count = if diff > 1 {
            accum_count[diff as usize - 1]
        } else {
            0
        };
        let res_distances_sum = if diff > 1 {
            total_dist - accum_dist[diff as usize - 2]
        } else {
            total_dist
        };
        md!(res_distances_sum);
        ans += count as i64 * diameter1;
        ans += res_distances_sum;
        ans += (distance1 + 1) * (n1 - count) as i64;
    }
    wl!(ans);
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

trait JoinExtended {
    fn join_with(
        self,
        sep: &str,
    ) -> String;
}

impl<I> JoinExtended for I
where
    I: Iterator,
    I::Item: Joinable,
{
    fn join_with(
        self,
        sep: &str,
    ) -> String {
        let mut peekable = self.peekable();
        let is_2d = if let Some(first) = peekable.peek() {
            first.is_container()
        } else {
            false
        };

        let res = peekable.map(|item| item.join_item(sep)).collect::<Vec<_>>();

        // Use newline for 2D rows, provided sep for 1D elements
        res.join(if is_2d { "\n" } else { sep })
    }
}

trait Joinable {
    fn join_item(
        &self,
        sep: &str,
    ) -> String;
    fn is_container(&self) -> bool;
}

macro_rules! impl_joinable_scalar {
    ($($t:ty),*) => {
        $(
            impl Joinable for &$t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
            impl Joinable for $t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
        )*
    };
}

impl_joinable_scalar!(
    i32, i64, i128, u32, u64, u128, usize, isize, f32, f64, char, String, &str
);

impl<T: std::fmt::Display> Joinable for &Vec<T> {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}

impl<T: std::fmt::Display> Joinable for &[T] {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}
