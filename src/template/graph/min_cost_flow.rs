#![allow(dead_code)]

// --- SNAP START ---

use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

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
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_flow() {
        let mut mcf = MinCostFlow::new(4);
        mcf.add_edge(0, 1, 2, 1);
        mcf.add_edge(0, 2, 1, 2);
        mcf.add_edge(1, 2, 1, 1);
        mcf.add_edge(1, 3, 1, 3);
        mcf.add_edge(2, 3, 2, 1);

        // Path 1: 0->1->2->3 (cap 1, cost 1+1+1=3)
        // Path 2: 0->2->3 (cap 1, cost 2+1=3)
        // Total cost for flow 2 = 6
        let (flow, cost) = mcf.min_cost_flow(0, 3, 2);
        assert_eq!(flow, 2);
        assert_eq!(cost, 6);
    }
}
