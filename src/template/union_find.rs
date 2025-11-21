#![allow(dead_code)]

// --- SNAP START ---

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
    /// Creates a new Union-Find structure with `n` elements (0 to n-1).
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    /// Finds the root of the element `x` with path compression.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
            self.parent[x]
        }
    }

    /// Unites the sets containing elements `x` and `y`.
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
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

    /// Returns the size of the set containing element `x`.
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// Checks if elements `x` and `y` are in the same set.
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        assert!(uf.same(0, 1));
        assert!(!uf.same(0, 2));
    }
}