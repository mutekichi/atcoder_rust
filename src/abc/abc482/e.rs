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
        n: usize,
        UV: [(Usize1, Usize1); n - 1],
        C: [Usize1; n],
    }

    let mut tree = Tree::new(n);
    for (u, v) in UV {
        tree.add_edge(u, v, 1);
    }

    let mut v_by_color = vec![vec![]; n];
    for i in 0..n {
        let c = C[i];
        v_by_color[c].push(i);
    }

    let (diameter, _, _) = tree.diameter();
    let THRESHOLD = 400;
    let centers = tree.centroids();

    tree.build_lca(0);

    let mut ans = 0usize;
    for c in 0..n {
        let l = v_by_color[c].len();
        if v_by_color[c].len() < THRESHOLD {
            md!(c, v_by_color[c].len());
            if l <= 1 {
                continue;
            }
            for i in 0..l - 1 {
                for j in i + 1..l {
                    let (i, j) = (v_by_color[c][i], v_by_color[c][j]);
                    md!(tree.dist(i, j), i, j);
                    if tree.dist(i, j) == diameter {
                        ans += 1;
                    }
                }
            }
        } else {
            let mut seen = vec![false; n];
            let mut data = vec![];
            for &v in &v_by_color[c] {
                if tree.edges[v].len() != 1 {
                    continue;
                }
                data.push((v, 1));
                seen[v] = true;
            }
            for _ in 0..(diameter - 1) / 2 {
                let mut counter = BTreeMap::new();
                for &(v, cnt) in &data {
                    for &(nv, _) in &tree.edges[v] {
                        if seen[nv] {
                            continue;
                        } else {
                            *counter.entry(nv).or_insert(0) += cnt;
                        }
                    }
                }
                data.clear();
                for (nv, count) in counter {
                    seen[nv] = true;
                    data.push((nv, count));
                }
            }

            let counts = data
                .iter()
                .filter(|&(v, _)| tree.edges[*v].len() > 1)
                .map(|&(_, cnt)| cnt)
                .collect::<Vec<_>>();
            md!(c, counts.iter().join(" "));
            if diameter % 2 == 0 {
                md!("diameter even");
                md!(counts.iter().join(" "));
                let sum = counts.iter().sum::<usize>();
                ans += (sum * sum
                    - counts.into_iter().map(|v| v * v).sum::<usize>())
                    / 2;
            } else {
                md!(c);
                if centers.len() == 1 {
                    md!(1);
                    let center = centers[0];
                    let mut cnt_center = 0;
                    let mut cnt_other = 0;
                    for &(v, cnt) in &data {
                        if tree.edges[v].len() <= 1 {
                            continue;
                        }
                        if v == center {
                            cnt_center += cnt;
                        } else {
                            cnt_other += cnt;
                        }
                    }
                    md!(cnt_center, cnt_other);
                    ans += cnt_center * cnt_other;
                    md!(ans);
                } else {
                    md!(2);
                    let mut cnt1 = 0;
                    let mut cnt2 = 0;
                    for &(v, cnt) in &data {
                        if v == centers[0] {
                            cnt1 += cnt;
                        } else if v == centers[1] {
                            cnt2 += cnt;
                        } else {
                            unreachable!();
                        }
                    }
                    md!(cnt1, cnt2);
                    ans += cnt1 * cnt2;
                }
            }
        }
    }
    println!("{}", ans);
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

// The implementations below were created using generative AI before the contest and published at
// https://github.com/mutekichi/atcoder_rust.
// These implementations may have been modified after being copied.
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
        assert!(
            self.lca_ready,
            "LCA not built. Call build_lca(root) first."
        );

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
        self.dist_from_root[u] + self.dist_from_root[v]
            - 2 * self.dist_from_root[lca]
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

    /// Euler Tour of the tree.
    ///
    /// # Returns
    /// - `in_time[u]`: The time index when node `u` is first visited.
    /// - `out_time[u]`: The time index after all nodes in `u`'s subtree have been visited.
    /// - `euler`: The sequence of nodes in the order they are visited (size N).
    /// - `depth`: The depth of each node in the tree.
    /// - `dist`: The weighted distance from the root to each node.
    ///
    /// # Complexity
    /// - O(N)
    pub fn euler_tour(
        &self
    ) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<i64>) {
        let n = self.n;
        let mut in_time = vec![0; n];
        let mut out_time = vec![0; n];
        let mut euler = vec![0; n];
        let mut depth = vec![0; n];
        let mut dist = vec![0; n];
        let mut parent = vec![n; n];
        let mut order = Vec::with_capacity(n);

        let mut timer = 0;
        let mut stack = vec![(self.root, n, 0, 0i64)];

        // DFS to compute in_time, depth, dist, and a topological order
        while let Some((u, p, d, w)) = stack.pop() {
            in_time[u] = timer;
            euler[timer] = u;
            depth[u] = d;
            dist[u] = w;
            parent[u] = p;
            timer += 1;
            order.push(u);

            // Add children in reverse to maintain edge order
            for &(v, weight) in self.edges[u].iter().rev() {
                if v != p {
                    stack.push((v, u, d + 1, w + weight));
                }
            }
        }

        // Post-order traversal using 'order' to calculate subtree sizes
        let mut subtree_size = vec![1; n];
        for &u in order.iter().rev() {
            out_time[u] = in_time[u] + subtree_size[u];
            let p = parent[u];
            if p != n {
                subtree_size[p] += subtree_size[u];
            }
        }

        (in_time, out_time, euler, depth, dist)
    }
}
