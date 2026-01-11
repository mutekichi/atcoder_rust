#[allow(dead_code)]

// --- SNAP START ---

use std::cmp;
use std::collections::VecDeque;

// Edge structure used in the graph
#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cap: i64,
    pub flow: i64,
    pub rev: usize, // Index of the reverse edge in the adjacency list of 'to'
}

// Max Flow Graph structure
pub struct MfGraph {
    n: usize,
    graph: Vec<Vec<Edge>>,
}

impl MfGraph {
    // Initialize with n vertices
    pub fn new(n: usize) -> Self {
        MfGraph {
            n,
            graph: vec![vec![]; n],
        }
    }

    // Add a directed edge with capacity
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let from_len = self.graph[from].len();
        let to_len = self.graph[to].len();
        
        self.graph[from].push(Edge {
            to,
            cap,
            flow: 0,
            rev: to_len,
        });
        
        self.graph[to].push(Edge {
            to: from,
            cap: 0, // Capacity 0 for the reverse edge (in directed graph)
            flow: 0,
            rev: from_len,
        });
    }

    // Get the current flow state of an edge
    pub fn get_edge(&self, i: usize, j: usize) -> &Edge {
        &self.graph[i][j]
    }

    // --- Ford-Fulkerson Algorithm (DFS) ---
    // Complexity: O(F * E), good for small integers
    pub fn max_flow_ff(&mut self, s: usize, t: usize) -> i64 {
        let mut total_flow = 0;
        loop {
            let mut used = vec![false; self.n];
            let f = self.dfs_ff(s, t, i64::MAX, &mut used);
            if f == 0 {
                break;
            }
            total_flow += f;
        }
        total_flow
    }

    fn dfs_ff(&mut self, v: usize, t: usize, f: i64, used: &mut Vec<bool>) -> i64 {
        if v == t {
            return f;
        }
        used[v] = true;
        
        // Iterate through edges. Note: we need indices to mutate self.graph
        for i in 0..self.graph[v].len() {
            let cap = self.graph[v][i].cap;
            let flow = self.graph[v][i].flow;
            let to = self.graph[v][i].to;
            
            if !used[to] && cap - flow > 0 {
                let d = self.dfs_ff(to, t, cmp::min(f, cap - flow), used);
                if d > 0 {
                    self.graph[v][i].flow += d;
                    let rev = self.graph[v][i].rev;
                    self.graph[to][rev].flow -= d;
                    return d;
                }
            }
        }
        0
    }

    // --- Dinic's Algorithm (BFS + DFS) ---
    // Complexity: O(V^2 * E), standard for competitive programming
    pub fn max_flow_dinic(&mut self, s: usize, t: usize) -> i64 {
        let mut total_flow = 0;
        loop {
            let level = self.bfs_dinic(s);
            if level[t] == usize::MAX {
                break;
            }
            let mut iter = vec![0; self.n];
            loop {
                let f = self.dfs_dinic(s, t, i64::MAX, &level, &mut iter);
                if f == 0 {
                    break;
                }
                total_flow += f;
            }
        }
        total_flow
    }

    fn bfs_dinic(&self, s: usize) -> Vec<usize> {
        let mut level = vec![usize::MAX; self.n];
        let mut que = VecDeque::new();
        level[s] = 0;
        que.push_back(s);
        
        while let Some(v) = que.pop_front() {
            for e in &self.graph[v] {
                if e.cap - e.flow > 0 && level[e.to] == usize::MAX {
                    level[e.to] = level[v] + 1;
                    que.push_back(e.to);
                }
            }
        }
        level
    }

    fn dfs_dinic(
        &mut self, 
        v: usize, 
        t: usize, 
        f: i64, 
        level: &Vec<usize>, 
        iter: &mut Vec<usize>
    ) -> i64 {
        if v == t {
            return f;
        }
        
        while iter[v] < self.graph[v].len() {
            let i = iter[v];
            let cap = self.graph[v][i].cap;
            let flow = self.graph[v][i].flow;
            let to = self.graph[v][i].to;
            
            if cap - flow > 0 && level[v] < level[to] {
                let d = self.dfs_dinic(to, t, cmp::min(f, cap - flow), level, iter);
                if d > 0 {
                    self.graph[v][i].flow += d;
                    let rev = self.graph[v][i].rev;
                    self.graph[to][rev].flow -= d;
                    return d;
                }
            }
            iter[v] += 1;
        }
        0
    }

    // --- Push-Relabel Algorithm (FIFO with Gap Heuristic) ---
    // Complexity: O(V^3), robust against pathological cases for Dinic/FF
    pub fn max_flow_push_relabel(&mut self, s: usize, t: usize) -> i64 {
        let n = self.n;
        let mut excess = vec![0; n];
        let mut height = vec![0; n];
        let mut active = vec![false; n];
        let mut count = vec![0; n * 2]; // Height frequency for Gap Heuristic
        let mut queue = VecDeque::new();

        height[s] = n;
        active[s] = true;
        count[0] = n - 1;
        count[n] = 1;

        // Initial push from source
        for i in 0..self.graph[s].len() {
            let cap = self.graph[s][i].cap;
            if cap > 0 {
                self.graph[s][i].flow += cap;
                let to = self.graph[s][i].to;
                let rev = self.graph[s][i].rev;
                self.graph[to][rev].flow -= cap;
                
                excess[to] += cap;
                excess[s] -= cap;
                
                if !active[to] && to != t && to != s {
                    active[to] = true;
                    queue.push_back(to);
                }
            }
        }

        while let Some(u) = queue.pop_front() {
            active[u] = false;
            let mut discharged = false;

            // Discharge loop
            while excess[u] > 0 {
                let mut pushed = false;
                let mut min_h = usize::MAX;

                for i in 0..self.graph[u].len() {
                    let e = &self.graph[u][i];
                    if e.cap - e.flow > 0 {
                        if height[u] == height[e.to] + 1 {
                            // Push
                            let d = cmp::min(excess[u], e.cap - e.flow);
                            self.graph[u][i].flow += d;
                            let rev = self.graph[u][i].rev;
                            let to = e.to;
                            self.graph[to][rev].flow -= d;
                            
                            excess[u] -= d;
                            excess[to] += d;
                            
                            if !active[to] && to != t && to != s {
                                active[to] = true;
                                queue.push_back(to);
                            }
                            pushed = true;
                            if excess[u] == 0 {
                                break;
                            }
                        } else {
                            min_h = cmp::min(min_h, height[e.to]);
                        }
                    }
                }

                if excess[u] == 0 {
                    discharged = true;
                    break;
                }

                if !pushed {
                    // Relabel
                    if count[height[u]] == 1 {
                        // Gap Heuristic: Lift all nodes with height > height[u]
                        let h_u = height[u];
                        for h in height.iter_mut() {
                            if *h >= h_u && *h < n {
                                count[*h] -= 1;
                                *h = n + 1;
                            }
                        }
                    } else {
                        count[height[u]] -= 1;
                        height[u] = if min_h == usize::MAX { n + 1 } else { min_h + 1 };
                        if height[u] < n {
                            count[height[u]] += 1;
                        }
                    }
                }
            }
            
            // If still active (excess > 0) after processing, put back in queue
            if !discharged && height[u] < n && excess[u] > 0 {
                 if !active[u] {
                    active[u] = true;
                    queue.push_back(u);
                 }
            }
        }

        // The excess at t is the max flow
        excess[t] + self.graph[t].iter().map(|e| e.flow).sum::<i64>() 
        // Note: For Push-Relabel, simply excess[t] usually suffices if flow conservation holds, 
        // but calculating incoming flow to t is safer given initialization.
        // Actually, in preflow-push, 'excess[t]' accumulates the flow arriving at t.
        // However, standard implementation simply tracks excess. 
        // Let's rely on conservation: flow out of s = flow into t.
        // Or simpler: sum of flows on edges pointing to t.
        // Here, we return accumulated excess at t.
        excess[t]
    }

    // --- Additional Information ---
    
    // Returns a boolean vector indicating the min-cut partition (S-side)
    // true: reachable from s in residual graph (belongs to S)
    // false: unreachable (belongs to T)
    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self.n];
        let mut queue = VecDeque::new();
        visited[s] = true;
        queue.push_back(s);
        
        while let Some(v) = queue.pop_front() {
            for e in &self.graph[v] {
                if e.cap - e.flow > 0 && !visited[e.to] {
                    visited[e.to] = true;
                    queue.push_back(e.to);
                }
            }
        }
        visited
    }
}