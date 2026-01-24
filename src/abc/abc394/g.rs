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
        h: usize,
        w: usize,
        F: [[i64; w]; h],
    }
    let mut pq = BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            if i != h - 1 {
                pq.push((min(F[i][j], F[i + 1][j]), i, j, i + 1, j));
            }
            if j != w - 1 {
                pq.push((min(F[i][j], F[i][j + 1]), i, j, i, j + 1));
            }
        }
    }
    let mut tree = Tree::new(2 * h * w - 1);
    let mut uf = UnionFind::new(2 * h * w - 1);
    let mut weights = vec![];
    while let Some((f, i, j, k, l)) = pq.pop() {
        let u = i * w + j;
        let v = k * w + l;
        if !uf.same(u, v) {
            md!(i, j, k, l);
            tree.add_edge(h * w + weights.len(), uf.find(u), 1);
            tree.add_edge(h * w + weights.len(), uf.find(v), 1);
            let root_u = uf.find(u);
            let root_v = uf.find(v);
            uf.parent[root_u] = h * w + weights.len();
            uf.parent[root_v] = h * w + weights.len();
            weights.push(f);
        }
    }
    tree.build_lca(2 * h * w - 2);
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            a: Usize1, b: Usize1, y: i64,
            c: Usize1, d: Usize1, z: i64,
        }
        if a == c && b == d {
            println!("{}", y.abs_diff(z));
        } else {
            let lca = tree.lca(a * w + b, c * w + d);
            md!(lca);
            let w = weights[lca - h * w];
            md!(w);
            if w < y && w < z {
                println!("{}", y - w + z - w);
            } else {
                println!("{}", y.abs_diff(z));
            }
        }
    }
}

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

#[derive(Debug, Clone)]
/// Union-Find (Disjoint Set Union)
///
/// # Usage
/// ```
/// // Initialize
/// let mut uf = UnionFind::new(n);
///
/// // Unite
/// uf.unite(0, 1);
///
/// // Check connectivity
/// if uf.same(0, 1) {
///     println!("Connected!");
/// }
///
/// // Get component size
/// println!("Size of 0: {}", uf.size(0));
/// ```
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
