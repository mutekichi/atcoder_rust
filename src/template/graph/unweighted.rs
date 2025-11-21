#![allow(dead_code)]

use std::collections::VecDeque;

const INF: usize = 1 << 60; 

/// --- SNAP START ---

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
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.rev_adj[v].push(u);
    }

    /// Adds an undirected edge.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    // ====================================================
    // 1. BFS (Breadth First Search)
    // ====================================================

    /// Computes shortest distance from `start` using BFS. O(V + E)
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut dist = vec![INF; self.n];
        let mut queue = VecDeque::new();

        dist[start] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for &v in &self.adj[u] {
                if dist[v] == INF {
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

    fn dfs_post_order(&self, u: usize, visited: &mut Vec<bool>, post_order: &mut Vec<usize>) {
        visited[u] = true;
        for &v in &self.adj[u] {
            if !visited[v] {
                self.dfs_post_order(v, visited, post_order);
            }
        }
        post_order.push(u);
    }

    fn dfs_reverse(&self, u: usize, visited: &mut Vec<bool>, component: &mut Vec<usize>) {
        visited[u] = true;
        component.push(u);
        for &v in &self.rev_adj[u] {
            if !visited[v] {
                self.dfs_reverse(v, visited, component);
            }
        }
    }
}
