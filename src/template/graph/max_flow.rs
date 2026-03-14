#![allow(dead_code)]
use std::cmp;
use std::collections::VecDeque;

// --- SNAP START ---

#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cap: i64,
    pub flow: i64,
    pub rev: usize,
}

pub struct MfGraph {
    n: usize,
    graph: Vec<Vec<Edge>>,
}

impl MfGraph {
    pub fn new(n: usize) -> Self {
        MfGraph {
            n,
            graph: vec![vec![]; n],
        }
    }

    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
        cap: i64,
    ) {
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
            cap: 0,
            flow: 0,
            rev: from_len,
        });
    }

    pub fn get_edge(
        &self,
        i: usize,
        j: usize,
    ) -> &Edge {
        &self.graph[i][j]
    }

    // --- Ford-Fulkerson Algorithm ---
    pub fn max_flow_ff(
        &mut self,
        s: usize,
        t: usize,
    ) -> i64 {
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

    fn dfs_ff(
        &mut self,
        v: usize,
        t: usize,
        f: i64,
        used: &mut Vec<bool>,
    ) -> i64 {
        if v == t {
            return f;
        }
        used[v] = true;

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

    // --- Dinic's Algorithm ---
    pub fn max_flow_dinic(
        &mut self,
        s: usize,
        t: usize,
    ) -> i64 {
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

    fn bfs_dinic(
        &self,
        s: usize,
    ) -> Vec<usize> {
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
        level: &[usize],
        iter: &mut Vec<usize>,
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

    // --- Push-Relabel Algorithm ---
    pub fn max_flow_push_relabel(
        &mut self,
        s: usize,
        t: usize,
    ) -> i64 {
        let n = self.n;
        let mut excess = vec![0; n];
        let mut height = vec![0; n];
        let mut active = vec![false; n];
        let mut count = vec![0; n * 2 + 1];
        let mut queue = VecDeque::new();

        height[s] = n;
        active[s] = true;
        count[0] = n - 1;
        count[n] = 1;

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

            while excess[u] > 0 {
                let mut pushed = false;
                let mut min_h = usize::MAX;

                for i in 0..self.graph[u].len() {
                    let cap = self.graph[u][i].cap;
                    let flow = self.graph[u][i].flow;
                    let to = self.graph[u][i].to;

                    if cap - flow > 0 {
                        if height[u] == height[to] + 1 {
                            let d = cmp::min(excess[u], cap - flow);
                            self.graph[u][i].flow += d;
                            let rev = self.graph[u][i].rev;
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
                            min_h = cmp::min(min_h, height[to]);
                        }
                    }
                }

                if excess[u] == 0 {
                    break;
                }

                if !pushed {
                    if count[height[u]] == 1 {
                        let h_u = height[u];
                        for v in 0..n {
                            if height[v] >= h_u && height[v] < n {
                                count[height[v]] -= 1;
                                height[v] = n + 1;
                                count[height[v]] += 1;
                            }
                        }
                    } else {
                        count[height[u]] -= 1;
                        height[u] = if min_h == usize::MAX {
                            n + 1
                        } else {
                            min_h + 1
                        };
                        count[height[u]] += 1;
                    }
                }
            }

            if height[u] < n && excess[u] > 0 {
                if !active[u] {
                    active[u] = true;
                    queue.push_back(u);
                }
            }
        }

        excess[t]
    }

    pub fn min_cut(
        &self,
        s: usize,
    ) -> Vec<bool> {
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
