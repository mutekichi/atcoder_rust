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

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            UV: [(Usize1, Usize1); m],
            w: usize,
            S: [Chars; n],
        }

        let mut holidays = vec![];
        for s in S {
            holidays.push(s.iter().map(|&c| c == 'o').collect::<Vec<_>>());
        }

        let mut exgraph = UnweightedGraph::new(n * w);

        for i in 0..n {
            for day in 0..w {
                let nday = (day + 1) % w;
                if holidays[i][day] && holidays[i][nday] {
                    exgraph.add_edge(i * w + day, i * w + nday);
                }
            }
        }
        for &(u, v) in &UV {
            for day in 0..w {
                let nday = (day + 1) % w;
                if holidays[u][day] && holidays[v][nday] {
                    let uidx = u * w + day;
                    let vidx = v * w + nday;
                    exgraph.add_edge(uidx, vidx);
                }
                if holidays[v][day] && holidays[u][nday] {
                    let uidx = u * w + nday;
                    let vidx = v * w + day;
                    exgraph.add_edge(vidx, uidx);
                }
            }
        }

        if exgraph.topological_sort().is_none() {
            println!("Yes");
        } else {
            println!("No");
        }

        // let mut OK = false;
        // for i in 0..n {
        //     let mut seen = vec![vec![false; w]; n];
        //     let mut queue = VecDeque::new();
        //     if !holidays[i][0] {
        //         continue;
        //     }
        //     queue.push_back((i, 0));
        //     seen[i][0] = true;
        //     let mut ok = false;
        //     while let Some((town, day)) = queue.pop_front() {
        //         md!(town, day);
        //         let next_day = (day + 1) % w;
        //         let mut next_list = vec![];
        //         if holidays[town][next_day] {
        //             next_list.push((town, next_day));
        //         }
        //         for &next_town in &graph[town] {
        //             if holidays[next_town][next_day] {
        //                 next_list.push((next_town, next_day));
        //             }
        //         }
        //         for (ntown, nday) in next_list {
        //             if seen[ntown][nday] {
        //                 md!(ntown, nday);
        //                 ok = true;
        //                 break;
        //             } else {
        //                 seen[ntown][nday] = true;
        //                 queue.push_back((ntown, nday));
        //             }
        //         }
        //         if ok {
        //             break;
        //         }
        //     }
        //     if ok {
        //         println!("Yes");
        //         OK = true;
        //         break;
        //     }
        // }
        // if !OK {
        //     println!("No");
        // }

        // let mut uf = UnionFind::new(n * w + 1);
        // for i in 0..n {
        //     uf.unite(n * w, i);
        // }
        // for i in 0..n {
        //     for day in 0..w {
        //         let nday = (day + 1) % w;
        //         if holidays[i][day] && holidays[i][nday] {
        //             uf.unite(i * w + day, i * w + nday);
        //         }
        //     }
        // }

        // let mut ok = true;

        // for &(u, v) in &UV {
        //     for day in 0..w {
        //         let nday = (day + 1) % w;
        //         if holidays[u][day] && holidays[v][nday] {
        //             let uidx = u * w + day;
        //             let vidx = v * w + nday;
        //             if uf.same(uidx, vidx) && uf.find(uidx) == n * w {
        //                 md!(u, v, day, nday);
        //                 ok = true;
        //             } else {
        //                 uf.unite(uidx, vidx);
        //             }
        //         }
        //         if holidays[v][day] && holidays[u][nday] {
        //             let uidx = u * w + nday;
        //             let vidx = v * w + day;
        //             if uf.same(uidx, vidx) && uf.find(uidx) == n * w {
        //                 md!(u, v, day, nday);
        //                 ok = true;
        //             } else {
        //                 uf.unite(uidx, vidx);
        //             }
        //         }
        //     }
        // }
        // if ok {
        //     println!("Yes");
        // } else {
        //     println!("No");
        // }
    }
}

// FOR TEMPLATE INJECTIONS

/// Unweighted Graph Structure
///
/// A simple graph container for unweighted graphs (edge weight = 1).
/// Supports directed/undirected edges and common algorithms for unweighted graphs.
///
/// # Supported Algorithms
/// - **BFS**: Single-Source Shortest Path (Edge weight = 1). $O(V + E)$
/// - **Topological Sort**: Linear ordering of vertices (DAG only). $O(V + E)$
/// - **SCC (Strongly Connected Components)**: Decomposes graph into SCCs. $O(V + E)$
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::graph::unweighted::UnweightedGraph;
///
/// // 1. Initialize
/// let n = 6;
/// let mut graph = UnweightedGraph::new(n);
///
/// // 2. Add Edges
/// // Directed: 0 -> 1
/// graph.add_edge(0, 1);
/// // Undirected: 1 <-> 2
/// graph.add_undirected_edge(1, 2);
///
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 1); // Cycle 1-2-3-1
/// graph.add_edge(3, 4);
/// graph.add_edge(4, 5);
///
/// // 3. BFS (Shortest Path)
/// // Returns Vec<usize>. Unreachable nodes are INF (approx 1e18).
/// let dist = graph.bfs(0);
/// assert_eq!(dist[1], 1);
/// assert_eq!(dist[2], 2);
/// assert_eq!(dist[3], 3);
///
/// // 4. Topological Sort
/// // Returns Option<Vec<usize>>. Returns None if the graph contains a cycle (not a DAG).
/// // This graph has a cycle (1-2-3), so it returns None.
/// assert_eq!(graph.topological_sort(), None);
///
/// // Let's make a DAG
/// let mut dag = UnweightedGraph::new(3);
/// dag.add_edge(0, 1);
/// dag.add_edge(1, 2);
/// let sorted = dag.topological_sort();
/// assert_eq!(sorted, Some(vec![0, 1, 2]));
///
/// // 5. SCC (Strongly Connected Components)
/// // Returns Vec<Vec<usize>>. Each inner vector is a component.
/// // Components are topologically sorted.
/// let scc = graph.scc();
/// // Components: {0}, {1, 2, 3}, {4}, {5} (Order may vary within components)
/// assert_eq!(scc.len(), 4);
/// ```
pub struct UnweightedGraph {
    n: usize,
    adj: Vec<Vec<usize>>,
    rev_adj: Vec<Vec<usize>>, // For SCC (Kosaraju)
}

impl UnweightedGraph {
    pub fn new(n: usize) -> Self {
        UnweightedGraph {
            n,
            adj: vec![vec![]; n],
            rev_adj: vec![vec![]; n],
        }
    }

    /// Adds a directed edge.
    pub fn add_edge(
        &mut self,
        u: usize,
        v: usize,
    ) {
        self.adj[u].push(v);
        self.rev_adj[v].push(u);
    }

    /// Adds an undirected edge.
    pub fn add_undirected_edge(
        &mut self,
        u: usize,
        v: usize,
    ) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    // ====================================================
    // 1. BFS (Breadth First Search)
    // ====================================================

    /// Computes shortest distance from `start` using BFS. O(V + E)
    pub fn bfs(
        &self,
        start: usize,
    ) -> Vec<usize> {
        let mut dist = vec![INF_USIZE; self.n];
        let mut queue = VecDeque::new();

        dist[start] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for &v in &self.adj[u] {
                if dist[v] == INF_USIZE {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    // ====================================================
    // 2. Topological Sort
    // ====================================================

    /// Performs Topological Sort using Kahn's Algorithm (In-degree). O(V + E)
    /// Returns None if the graph contains a cycle (not a DAG).
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let mut in_degree = vec![0; self.n];
        for u in 0..self.n {
            for &v in &self.adj[u] {
                in_degree[v] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for i in 0..self.n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut result = Vec::new();
        while let Some(u) = queue.pop_front() {
            result.push(u);
            for &v in &self.adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if result.len() == self.n {
            Some(result)
        } else {
            None // Cycle detected
        }
    }

    // ====================================================
    // 3. SCC (Strongly Connected Components)
    // ====================================================

    /// Decomposes the graph into Strongly Connected Components (SCC) using Kosaraju's Algorithm. O(V + E)
    /// Returns a vector of components, where each component is a vector of node indices.
    /// The components are topologically sorted.
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut post_order = Vec::new();

        // 1. First DFS to determine processing order
        for i in 0..self.n {
            if !visited[i] {
                self.dfs_post_order(i, &mut visited, &mut post_order);
            }
        }

        // 2. Second DFS on reversed graph
        visited.fill(false);
        let mut scc_groups = Vec::new();

        for &i in post_order.iter().rev() {
            if !visited[i] {
                let mut component = Vec::new();
                self.dfs_reverse(i, &mut visited, &mut component);
                scc_groups.push(component);
            }
        }

        scc_groups
    }

    fn dfs_post_order(
        &self,
        u: usize,
        visited: &mut Vec<bool>,
        post_order: &mut Vec<usize>,
    ) {
        visited[u] = true;
        for &v in &self.adj[u] {
            if !visited[v] {
                self.dfs_post_order(v, visited, post_order);
            }
        }
        post_order.push(u);
    }

    fn dfs_reverse(
        &self,
        u: usize,
        visited: &mut Vec<bool>,
        component: &mut Vec<usize>,
    ) {
        visited[u] = true;
        component.push(u);
        for &v in &self.rev_adj[u] {
            if !visited[v] {
                self.dfs_reverse(v, visited, component);
            }
        }
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
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    /// Path compression helper
    fn compress(
        &mut self,
        x: usize,
        root: usize,
    ) {
        self.parent[x] = root;
    }

    pub fn find(
        &mut self,
        x: usize,
    ) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            let root = self.find(p);

            // Comment out the line below to disable path compression
            self.compress(x, root);

            root
        }
    }

    pub fn unite(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        // self.unite_by_size(x, y)
        self.unite_by_rank(x, y)
    }

    pub fn unite_by_rank(
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

    pub fn unite_by_size(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    pub fn size(
        &mut self,
        x: usize,
    ) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    pub fn same(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        self.find(x) == self.find(y)
    }
}

// END TEMPLATE INJECTIONS
