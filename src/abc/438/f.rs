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
    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.edges[u].push((v, w));
        self.edges[v].push((u, w));
    }

    /// Adds a directed edge from `u` to `v` with weight `w`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, w: i64) {
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

    fn bfs_farthest(&self, start: usize) -> usize {
        let (dist, _) = self.bfs_dist(start);
        dist.iter()
            .enumerate()
            .max_by_key(|&(_, &d)| d)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn bfs_dist(&self, start: usize) -> (Vec<i64>, usize) {
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
    pub fn build_lca(&mut self, root: usize) {
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
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
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
    pub fn dist(&self, u: usize, v: usize) -> i64 {
        let lca = self.lca(u, v);
        self.dist_from_root[u] + self.dist_from_root[v] - 2 * self.dist_from_root[lca]
    }
    
    /// Returns the distance (number of edges) between nodes `u` and `v`.
    ///
    /// # Complexity
    /// - O(log N)
    pub fn depth_dist(&self, u: usize, v: usize) -> usize {
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

fn dfs_size(
    v: usize,
    from: usize,
    graph: &Vec<Vec<usize>>,
    sizes: &mut Vec<usize>
) -> usize {
    let mut size = 1;
    for &nv in &graph[v] {
        if nv != from {
            size += dfs_size(nv, v, graph, sizes);
        }
    }
    sizes[v] = size;
    size
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        UV: [(usize, usize); n - 1],
    }
    let mut ans: i64 = 0;
    let mut tree = Tree::new(n);
    let mut graph = vec![vec![]; n];
    for (u, v) in UV {
        tree.add_edge(u, v, 1);
        graph[u].push(v);
        graph[v].push(u);
    }

    tree.build_lca(0);
    
    let mut sizes = vec![0; n];
    dfs_size(
        0,
        INF_USIZE,
        &graph,
        &mut sizes
    );
    ans += n as i64 * (n + 1) as i64 / 2;
    for &nv in &graph[0] {
        let size = sizes[nv] as i64;
        md!(size);
        ans -= size * (size + 1) / 2;
    }
    md!(ans);

    let mut start = 0;
    let mut end = 0;
    
    let path_includes = |from: usize, to: usize, v: usize| {
        tree.dist(from, v) + tree.dist(v, to) == tree.dist(from, to)
    };

    let mut size_0 = INF_USIZE;

    for k in 1..n {
        if path_includes(start, end, k) {

        }
        else if path_includes(start, k, end) {
            end = k;
        }
        else if path_includes(k, end, start) {
            start = k;
        }
        else {
            break;
        }
        let size_start = if start != 0 {
            sizes[start]
        } else {
            if size_0 == INF_USIZE {
                let mut next_0 = INF_USIZE;
                for &nv in &graph[0] {
                    if path_includes(0, end, nv) {
                        next_0 = nv;
                        break;
                    }
                    else {
                        md!("not include", 0, end, nv);
                    }
                }
                size_0 = n - sizes[next_0];
                size_0
            }
            else {
                size_0
            }
        };
        let size_end = if end != 0 {
            sizes[end]
        } else {
            if size_0 == INF_USIZE {
                let mut next_0 = INF_USIZE;
                for &nv in &graph[0] {
                    if path_includes(0, start, nv) {
                        next_0 = nv;
                        break;
                    }
                    else {
                        md!("NOTE INCLUDE", 0, start, nv);
                    }
                }
                size_0 = n - sizes[next_0];
                size_0
            }
            else {
                size_0
            }
        };
        md!(start, size_start);
        md!(end, size_end);
        ans += size_start as i64 * size_end as i64;
        md!(ans);
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

fn join_with_space<T: ToString>(arr: &[T]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
