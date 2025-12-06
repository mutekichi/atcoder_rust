#![allow(dead_code)]

// --- SNAP START ---

use std::cmp::min;
use std::collections::VecDeque;

/// Max Flow (Dinic's Algorithm)
///
/// Solves the maximum flow problem in a directed graph.
///
/// # Complexity
/// - O(V^2 E) in general.
/// - O(E min(V^{2/3}, E^{1/2})) for unit capacity networks (e.g., Bipartite Matching).
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::graph::max_flow::Dinic;
///
/// let mut dinic = Dinic::new(4);
/// // Add edges: u -> v with capacity
/// dinic.add_edge(0, 1, 2);
/// dinic.add_edge(0, 2, 1);
/// dinic.add_edge(1, 2, 1);
/// dinic.add_edge(1, 3, 1);
/// dinic.add_edge(2, 3, 2);
///
/// // Calculate max flow from 0 to 3
/// assert_eq!(dinic.max_flow(0, 3), 3);
///
/// // Check min-cut: Get the set of nodes reachable from s in the residual graph
/// let cut = dinic.min_cut(0);
/// assert!(cut[0]); // Source is always reachable
/// assert!(!cut[3]); // Sink is not reachable
/// ```
#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cap: i64,
    pub rev: usize, // Index of the reverse edge in graph[to]
}

pub struct Dinic {
    n: usize,
    graph: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

impl Dinic {
    /// Creates a new graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Dinic {
            n,
            graph: vec![vec![]; n],
            level: vec![],
            iter: vec![],
        }
    }

    /// Adds a directed edge from `from` to `to` with capacity `cap`.
    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
        cap: i64,
    ) {
        let rev_idx = self.graph[to].len();
        let fwd_idx = self.graph[from].len();

        self.graph[from].push(Edge {
            to,
            cap,
            rev: rev_idx,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev: fwd_idx,
        });
    }

    /// BFS to calculate levels (distance from s).
    fn bfs(
        &mut self,
        s: usize,
    ) {
        self.level = vec![-1; self.n];
        self.level[s] = 0;
        let mut que = VecDeque::new();
        que.push_back(s);

        while let Some(v) = que.pop_front() {
            for e in &self.graph[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    que.push_back(e.to);
                }
            }
        }
    }

    /// DFS to find augmenting paths.
    fn dfs(
        &mut self,
        v: usize,
        t: usize,
        f: i64,
    ) -> i64 {
        if v == t {
            return f;
        }
        for i in self.iter[v]..self.graph[v].len() {
            self.iter[v] = i; // Update iter to avoid rescanning edges
            let e = self.graph[v][i].clone();
            // Note: We need to access graph[v][i] later, so cloning edge info is safer
            // or we use indices carefully. Here cloning simplified struct is cheap.

            if e.cap > 0 && self.level[v] < self.level[e.to] {
                let d = self.dfs(e.to, t, min(f, e.cap));
                if d > 0 {
                    self.graph[v][i].cap -= d;
                    let rev = e.rev;
                    self.graph[e.to][rev].cap += d;
                    return d;
                }
            }
        }
        0
    }

    /// Computes the maximum flow from `s` to `t`.
    pub fn max_flow(
        &mut self,
        s: usize,
        t: usize,
    ) -> i64 {
        let mut flow = 0;
        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                return flow;
            }
            self.iter = vec![0; self.n];
            loop {
                let f = self.dfs(s, t, std::i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
    }

    /// Returns a boolean vector indicating nodes reachable from `s` in the residual graph.
    /// Useful for reconstructing the Min-Cut.
    pub fn min_cut(
        &mut self,
        s: usize,
    ) -> Vec<bool> {
        self.bfs(s);
        self.level.iter().map(|&x| x != -1).collect()
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_flow() {
        let mut dinic = Dinic::new(4);
        dinic.add_edge(0, 1, 2);
        dinic.add_edge(0, 2, 1);
        dinic.add_edge(1, 2, 1);
        dinic.add_edge(1, 3, 1);
        dinic.add_edge(2, 3, 2);
        assert_eq!(dinic.max_flow(0, 3), 3);
    }
}
