#![allow(dead_code)]

// --- SNAP START ---

/// Weighted tree (0-indexed). All traversals are iterative.
///
/// `build_lca(root)` takes O(N log N) time and space using an Euler tour
/// and a Sparse Table. Afterwards, `lca`, `dist`, and `depth_dist` take O(1).
/// Adding an edge invalidates the LCA data; call `build_lca` again.
/// Distances are sums along the unique path, and may be negative.
/// All intermediate distance calculations must fit in i64.
///
/// ```
/// use atcoder_rust::template::graph::tree::WeightedTree;
/// let mut tree = WeightedTree::new(4);
/// tree.add_edge(0, 1, 5);
/// tree.add_edge(1, 2, -2);
/// tree.add_edge(1, 3, 4);
/// tree.build_lca(0);
/// assert_eq!(tree.lca(2, 3), 1);
/// assert_eq!(tree.dist(2, 3), 2);
/// assert_eq!(tree.depth_dist(2, 3), 2);
/// assert_eq!(tree.distances_from(2), vec![3, -2, 0, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct WeightedTree {
    n: usize,
    edges: Vec<Vec<(usize, i64)>>,
    root: usize,
    lca_data: Option<FastLca>,
}

impl WeightedTree {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![vec![]; n],
            root: 0,
            lca_data: None,
        }
    }

    /// Adds an undirected edge. O(1) amortized.
    pub fn add_edge(
        &mut self,
        u: usize,
        v: usize,
        w: i64,
    ) {
        assert!(u < self.n && v < self.n);
        self.edges[u].push((v, w));
        self.edges[v].push((u, w));
        self.lca_data = None;
    }

    /// Adds a parent -> child edge. Traversals must start at a root that
    /// reaches every vertex. `diameter` requires undirected edges.
    pub fn add_directed_edge(
        &mut self,
        u: usize,
        v: usize,
        w: i64,
    ) {
        assert!(u < self.n && v < self.n);
        self.edges[u].push((v, w));
        self.lca_data = None;
    }

    /// O(N log N) preprocessing, O(N log N) space.
    /// Panics for an invalid root or if the reachable graph is not a tree.
    pub fn build_lca(
        &mut self,
        root: usize,
    ) {
        let traversal = self.traverse(root);
        self.lca_data = Some(FastLca::new(traversal));
        self.root = root;
    }

    fn built_lca(&self) -> &FastLca {
        self.lca_data
            .as_ref()
            .expect("LCA not built. Call build_lca(root) first.")
    }

    /// Lowest common ancestor. O(1); requires `build_lca`.
    pub fn lca(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        self.built_lca().lca(u, v)
    }

    /// Sum of edge weights on the unique u-v path. O(1); requires `build_lca`.
    pub fn dist(
        &self,
        u: usize,
        v: usize,
    ) -> i64 {
        let data = self.built_lca();
        let a = data.lca(u, v);
        (data.dist[u] - data.dist[a]) + (data.dist[v] - data.dist[a])
    }

    /// Number of edges on the u-v path. O(1); requires `build_lca`.
    pub fn depth_dist(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        let data = self.built_lca();
        let a = data.lca(u, v);
        (data.depth[u] - data.depth[a]) + (data.depth[v] - data.depth[a])
    }

    /// Distances from one vertex to all vertices. O(N), no LCA build needed.
    /// Negative weights are supported (this is a tree traversal, not Dijkstra).
    pub fn distances_from(
        &self,
        root: usize,
    ) -> Vec<i64> {
        self.traverse(root).dist
    }

    /// Returns (length, endpoint, endpoint) in O(N).
    /// Requires an undirected tree with NONNEGATIVE weights.
    /// An empty tree returns (0, 0, 0); a zero-length path is allowed.
    pub fn diameter(&self) -> (i64, usize, usize) {
        if self.n == 0 {
            return (0, 0, 0);
        }
        assert!(
            self.edges.iter().flatten().all(|&(_, w)| w >= 0),
            "diameter requires nonnegative weights"
        );
        let first = self.distances_from(0);
        let u = (0..self.n).max_by_key(|&v| first[v]).unwrap();
        let second = self.distances_from(u);
        let v = (0..self.n).max_by_key(|&v| second[v]).unwrap();
        (second[v], u, v)
    }

    /// Vertices whose removal leaves components of size at most N/2. O(N).
    pub fn centroids(&self) -> Vec<usize> {
        if self.n == 0 {
            return vec![];
        }
        let traversal = self.traverse(self.root);
        let mut size = vec![1; self.n];
        let mut largest = vec![0; self.n];
        let mut result = vec![];
        for &u in traversal.order.iter().rev() {
            largest[u] = largest[u].max(self.n - size[u]);
            if largest[u] <= self.n / 2 {
                result.push(u);
            }
            let p = traversal.parent[u];
            if p != u {
                size[p] += size[u];
                largest[p] = largest[p].max(size[u]);
            }
        }
        result
    }

    /// Returns (in_time, out_time, preorder, depth, weighted_dist) in O(N).
    /// Subtree u is `preorder[in_time[u]..out_time[u]]`.
    /// Uses the last `build_lca` root, or 0 if none has been built.
    /// This length-N preorder is distinct from the length-(2N-1) LCA tour.
    pub fn euler_tour(
        &self
    ) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<i64>) {
        if self.n == 0 {
            return (vec![], vec![], vec![], vec![], vec![]);
        }
        let t = self.traverse(self.root);
        (t.tin, t.tout, t.order, t.depth, t.dist)
    }

    fn traverse(
        &self,
        root: usize,
    ) -> TreeTraversal {
        assert!(root < self.n, "root must be a vertex of a nonempty tree");
        let mut t = TreeTraversal {
            parent: vec![self.n; self.n],
            depth: vec![0; self.n],
            dist: vec![0; self.n],
            tin: vec![0; self.n],
            tout: vec![0; self.n],
            order: Vec::with_capacity(self.n),
            first: vec![0; self.n],
            tour: Vec::with_capacity(2 * self.n - 1),
        };
        t.parent[root] = root;
        t.order.push(root);
        t.tour.push(root);
        // Each frame stores the next adjacency entry to examine.
        let mut stack = vec![(root, 0)];
        while let Some((u, next)) = stack.pop() {
            if next == self.edges[u].len() {
                t.tout[u] = t.order.len();
                if let Some(&(p, _)) = stack.last() {
                    t.tour.push(p);
                }
                continue;
            }
            stack.push((u, next + 1));
            let (v, w) = self.edges[u][next];
            if v == t.parent[u] && v != u {
                continue;
            }
            assert_eq!(t.parent[v], self.n, "edges must form a tree");
            t.parent[v] = u;
            t.depth[v] = t.depth[u] + 1;
            t.dist[v] = t.dist[u] + w;
            t.tin[v] = t.order.len();
            t.order.push(v);
            t.first[v] = t.tour.len();
            t.tour.push(v);
            stack.push((v, 0));
        }
        assert_eq!(t.order.len(), self.n, "root must reach every vertex");
        t
    }
}

/// Unweighted wrapper over `WeightedTree`; edge weights are always 1.
/// LCA and distance queries take O(1) after O(N log N) preprocessing.
///
/// ```
/// use atcoder_rust::template::graph::tree::UnweightedTree;
/// let mut tree = UnweightedTree::new(4);
/// tree.add_edge(0, 1);
/// tree.add_edge(1, 2);
/// tree.add_edge(1, 3);
/// tree.build_lca(0);
/// assert_eq!(tree.lca(2, 3), 1);
/// assert_eq!(tree.dist(2, 3), 2usize);
/// assert_eq!(tree.diameter().0, 2usize);
/// assert_eq!(tree.centroids(), vec![1]);
/// ```
#[derive(Debug, Clone)]
pub struct UnweightedTree {
    inner: WeightedTree,
}

impl UnweightedTree {
    pub fn new(n: usize) -> Self {
        Self {
            inner: WeightedTree::new(n),
        }
    }

    pub fn add_edge(
        &mut self,
        u: usize,
        v: usize,
    ) {
        self.inner.add_edge(u, v, 1);
    }

    /// Adds a parent -> child edge; see `WeightedTree::add_directed_edge`.
    pub fn add_directed_edge(
        &mut self,
        u: usize,
        v: usize,
    ) {
        self.inner.add_directed_edge(u, v, 1);
    }

    /// O(N log N) time and space.
    pub fn build_lca(
        &mut self,
        root: usize,
    ) {
        self.inner.build_lca(root);
    }

    /// O(1); requires `build_lca`.
    pub fn lca(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        self.inner.lca(u, v)
    }

    /// Number of edges on the path. O(1); requires `build_lca`.
    pub fn dist(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        self.inner.depth_dist(u, v)
    }

    /// Alias of `dist`. O(1).
    pub fn depth_dist(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        self.dist(u, v)
    }

    /// O(N), no LCA build needed.
    pub fn distances_from(
        &self,
        root: usize,
    ) -> Vec<usize> {
        self.inner
            .distances_from(root)
            .into_iter()
            .map(|d| d as usize)
            .collect()
    }

    /// (length, endpoint, endpoint). O(N); requires an undirected tree.
    pub fn diameter(&self) -> (usize, usize, usize) {
        let (d, u, v) = self.inner.diameter();
        (d as usize, u, v)
    }

    /// O(N).
    pub fn centroids(&self) -> Vec<usize> {
        self.inner.centroids()
    }

    /// (in_time, out_time, preorder, depth). O(N).
    /// Subtree u is `preorder[in_time[u]..out_time[u]]`.
    pub fn euler_tour(
        &self
    ) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
        let (tin, tout, order, depth, _) = self.inner.euler_tour();
        (tin, tout, order, depth)
    }
}

#[derive(Debug, Clone)]
struct TreeTraversal {
    parent: Vec<usize>,
    depth: Vec<usize>,
    dist: Vec<i64>,
    tin: Vec<usize>,
    tout: Vec<usize>,
    order: Vec<usize>,
    first: Vec<usize>,
    tour: Vec<usize>,
}

/// Internal Euler-tour RMQ. Each table entry stores a vertex ID;
/// comparisons use unweighted depth, including for a weighted tree.
#[derive(Debug, Clone)]
struct FastLca {
    first: Vec<usize>,
    depth: Vec<usize>,
    dist: Vec<i64>,
    table: Vec<Vec<usize>>,
}

impl FastLca {
    fn new(t: TreeTraversal) -> Self {
        let m = t.tour.len();
        let mut table = vec![t.tour];
        let mut half = 1;
        while half <= m / 2 {
            let prev = table.last().unwrap();
            let row = (0..=m - 2 * half)
                .map(|i| {
                    let (u, v) = (prev[i], prev[i + half]);
                    if t.depth[u] <= t.depth[v] { u } else { v }
                })
                .collect();
            table.push(row);
            half *= 2;
        }
        Self {
            first: t.first,
            depth: t.depth,
            dist: t.dist,
            table,
        }
    }

    fn lca(
        &self,
        u: usize,
        v: usize,
    ) -> usize {
        let (a, b) = (self.first[u], self.first[v]);
        let (l, r) = (a.min(b), a.max(b) + 1);
        let k = (r - l).ilog2() as usize;
        let (x, y) = (self.table[k][l], self.table[k][r - (1 << k)]);
        if self.depth[x] <= self.depth[y] { x } else { y }
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    // Independent parent-walk oracle, without Euler tours or RMQ.
    fn rooted(
        adj: &[Vec<(usize, i64)>],
        root: usize,
    ) -> (Vec<usize>, Vec<usize>, Vec<i64>) {
        let n = adj.len();
        let mut parent = vec![n; n];
        let mut depth = vec![0; n];
        let mut dist = vec![0; n];
        parent[root] = root;
        let mut queue = vec![root];
        let mut i = 0;
        while i < queue.len() {
            let u = queue[i];
            i += 1;
            for &(v, w) in &adj[u] {
                if parent[v] == n {
                    parent[v] = u;
                    depth[v] = depth[u] + 1;
                    dist[v] = dist[u] + w;
                    queue.push(v);
                }
            }
        }
        (parent, depth, dist)
    }

    fn slow_lca(
        mut u: usize,
        mut v: usize,
        parent: &[usize],
        depth: &[usize],
    ) -> usize {
        while depth[u] > depth[v] {
            u = parent[u];
        }
        while depth[v] > depth[u] {
            v = parent[v];
        }
        while u != v {
            u = parent[u];
            v = parent[v];
        }
        u
    }

    #[test]
    fn random_trees_match_parent_walk_and_single_source_distances() {
        let mut seed = 123456789u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed as usize
        };
        for n in 1..=45 {
            let mut weighted = WeightedTree::new(n);
            let mut unweighted = UnweightedTree::new(n);
            let mut adj = vec![vec![]; n];
            for v in 1..n {
                let p = next() % v;
                let w = (next() % 21) as i64 - 10;
                weighted.add_edge(p, v, w);
                unweighted.add_edge(p, v);
                adj[p].push((v, w));
                adj[v].push((p, w));
            }
            for root in [0, n / 2, n - 1] {
                weighted.build_lca(root);
                unweighted.build_lca(root);
                let (parent, depth, root_dist) = rooted(&adj, root);
                let (tin, tout, order, tour_depth, tour_dist) =
                    weighted.euler_tour();
                assert_eq!(tour_depth, depth);
                assert_eq!(tour_dist, root_dist);
                let (a, b, c, d) = unweighted.euler_tour();
                assert_eq!(
                    (a, b, c, d),
                    (
                        tin.clone(),
                        tout.clone(),
                        order.clone(),
                        depth.clone()
                    )
                );
                for u in 0..n {
                    let (_, _, expected_dist) = rooted(&adj, u);
                    assert_eq!(weighted.distances_from(u), expected_dist);
                    let unweighted_dist = unweighted.distances_from(u);
                    assert_eq!(order[tin[u]], u);
                    for v in 0..n {
                        let expected_lca = slow_lca(u, v, &parent, &depth);
                        assert_eq!(weighted.lca(u, v), expected_lca);
                        assert_eq!(unweighted.lca(u, v), expected_lca);
                        assert_eq!(weighted.dist(u, v), expected_dist[v]);
                        let hops =
                            depth[u] + depth[v] - 2 * depth[expected_lca];
                        assert_eq!(weighted.depth_dist(u, v), hops);
                        assert_eq!(unweighted.dist(u, v), hops);
                        assert_eq!(unweighted_dist[v], hops);
                        assert_eq!(
                            tin[u] <= tin[v] && tin[v] < tout[u],
                            expected_lca == u
                        );
                    }
                }
            }
            // Check centroids against explicit component sizes after deletion.
            let mut expected = vec![];
            for removed in 0..n {
                let mut seen = vec![false; n];
                seen[removed] = true;
                let mut largest = 0;
                for start in 0..n {
                    if seen[start] {
                        continue;
                    }
                    let mut stack = vec![start];
                    seen[start] = true;
                    let mut size = 0;
                    while let Some(u) = stack.pop() {
                        size += 1;
                        for &(v, _) in &adj[u] {
                            if !seen[v] {
                                seen[v] = true;
                                stack.push(v);
                            }
                        }
                    }
                    largest = largest.max(size);
                }
                if largest <= n / 2 {
                    expected.push(removed);
                }
            }
            let mut actual = weighted.centroids();
            actual.sort_unstable();
            assert_eq!(actual, expected);
            let mut actual = unweighted.centroids();
            actual.sort_unstable();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn nonnegative_diameter_matches_all_pairs() {
        for n in 1..=30 {
            let mut tree = WeightedTree::new(n);
            let mut adj = vec![vec![]; n];
            for v in 1..n {
                let p = (v - 1) / 2;
                let w = (v % 5) as i64;
                tree.add_edge(p, v, w);
                adj[p].push((v, w));
                adj[v].push((p, w));
            }
            let expected =
                (0..n).flat_map(|u| rooted(&adj, u).2).max().unwrap();
            let (d, u, v) = tree.diameter();
            assert_eq!(d, expected);
            assert_eq!(rooted(&adj, u).2[v], d);
        }
    }

    #[test]
    fn empty_singleton_and_directed_tree() {
        let empty = UnweightedTree::new(0);
        assert_eq!(empty.diameter(), (0, 0, 0));
        assert!(empty.centroids().is_empty());
        assert_eq!(empty.euler_tour(), (vec![], vec![], vec![], vec![]));
        let mut singleton = UnweightedTree::new(1);
        singleton.build_lca(0);
        assert_eq!(singleton.lca(0, 0), 0);
        assert_eq!(singleton.dist(0, 0), 0);
        assert_eq!(singleton.centroids(), vec![0]);
        let mut tree = WeightedTree::new(4);
        tree.add_directed_edge(2, 0, -1);
        tree.add_directed_edge(0, 1, 2);
        tree.add_directed_edge(2, 3, 0);
        tree.build_lca(2);
        assert_eq!(tree.lca(1, 3), 2);
        assert_eq!(tree.dist(1, 3), 1);
        assert_eq!(tree.distances_from(2), vec![-1, 1, 0, 0]);
    }

    #[test]
    fn long_path_uses_no_recursion() {
        let n = 200_000;
        let mut tree = UnweightedTree::new(n);
        for v in 1..n {
            tree.add_edge(v - 1, v);
        }
        tree.build_lca(n - 1);
        assert_eq!(tree.lca(0, n / 2), n / 2);
        assert_eq!(tree.dist(0, n - 1), n - 1);
        assert_eq!(tree.diameter().0, n - 1);
        let mut centers = tree.centroids();
        centers.sort_unstable();
        assert_eq!(centers, vec![n / 2 - 1, n / 2]);
    }

    #[test]
    #[should_panic(expected = "LCA not built")]
    fn querying_before_build_panics() {
        UnweightedTree::new(1).lca(0, 0);
    }

    #[test]
    #[should_panic(expected = "LCA not built")]
    fn adding_an_edge_invalidates_lca() {
        let mut tree = WeightedTree::new(2);
        tree.add_edge(0, 1, 1);
        tree.build_lca(0);
        tree.add_directed_edge(0, 1, 2);
        tree.dist(0, 1);
    }

    #[test]
    #[should_panic(expected = "nonnegative weights")]
    fn negative_diameter_is_rejected() {
        let mut tree = WeightedTree::new(2);
        tree.add_edge(0, 1, -1);
        tree.diameter();
    }

    #[test]
    #[should_panic(expected = "root must reach every vertex")]
    fn disconnected_tree_is_rejected() {
        WeightedTree::new(2).build_lca(0);
    }
}
