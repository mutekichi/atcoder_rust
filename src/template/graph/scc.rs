#![allow(dead_code)]
// --- SNAP START ---

pub struct SccGraph {
    n: usize,
    pub edges: Vec<(usize, usize)>,
}

pub struct SccResult {
    /// component ID for each vertex (topological order)
    pub ids: Vec<usize>,
    /// vertices belonging to each component
    pub groups: Vec<Vec<usize>>,
    /// adjacency list of the condensed DAG
    pub condensed_adj: Vec<Vec<usize>>,
}

impl SccGraph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
    ) {
        self.edges.push((from, to));
    }

    pub fn scc(&self) -> SccResult {
        let mut start = vec![0; self.n + 1];
        for &(u, _) in &self.edges {
            start[u + 1] += 1;
        }
        for i in 0..self.n {
            start[i + 1] += start[i];
        }

        let mut counter = start.clone();
        let mut elist = vec![0; self.edges.len()];
        for &(u, v) in &self.edges {
            elist[counter[u]] = v;
            counter[u] += 1;
        }

        let mut visited = Vec::with_capacity(self.n);
        let mut ord = vec![!0; self.n];
        let mut low = vec![!0; self.n];
        let mut now_ord = 0;
        let mut ids = vec![0; self.n];
        let mut group_num = 0;
        let mut stack = Vec::new();

        for i in 0..self.n {
            if ord[i] == !0 {
                self.dfs(
                    i,
                    &mut now_ord,
                    &start,
                    &elist,
                    &mut visited,
                    &mut ord,
                    &mut low,
                    &mut ids,
                    &mut group_num,
                    &mut stack,
                );
            }
        }

        for x in &mut ids {
            *x = group_num - 1 - *x;
        }

        let mut groups = vec![Vec::new(); group_num];
        for i in 0..self.n {
            groups[ids[i]].push(i);
        }

        // build condensed graph
        let mut condensed_adj = vec![Vec::new(); group_num];
        for &(u, v) in &self.edges {
            let id_u = ids[u];
            let id_v = ids[v];
            if id_u != id_v {
                condensed_adj[id_u].push(id_v);
            }
        }
        for v in &mut condensed_adj {
            v.sort_unstable();
            v.dedup();
        }

        SccResult {
            ids,
            groups,
            condensed_adj,
        }
    }

    fn dfs(
        &self,
        v: usize,
        now_ord: &mut usize,
        start: &[usize],
        elist: &[usize],
        visited: &mut Vec<usize>,
        ord: &mut [usize],
        low: &mut [usize],
        ids: &mut [usize],
        group_num: &mut usize,
        stack: &mut Vec<(usize, usize)>,
    ) {
        stack.push((v, start[v]));
        ord[v] = *now_ord;
        low[v] = *now_ord;
        *now_ord += 1;
        visited.push(v);

        while let Some((curr, next_idx)) = stack.pop() {
            if next_idx < start[curr + 1] {
                let to = elist[next_idx];
                stack.push((curr, next_idx + 1));
                if ord[to] == !0 {
                    ord[to] = *now_ord;
                    low[to] = *now_ord;
                    *now_ord += 1;
                    visited.push(to);
                    stack.push((to, start[to]));
                } else {
                    low[curr] = low[curr].min(ord[to]);
                }
            } else {
                if low[curr] == ord[curr] {
                    loop {
                        let u = visited.pop().unwrap();
                        ord[u] = self.n;
                        ids[u] = *group_num;
                        if u == curr {
                            break;
                        }
                    }
                    *group_num += 1;
                }
                if let Some((prev, _)) = stack.last() {
                    low[*prev] = low[*prev].min(low[curr]);
                }
            }
        }
    }
}
