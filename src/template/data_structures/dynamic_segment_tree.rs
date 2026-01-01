#![allow(dead_code)]

// --- SNAP START ---

/// Trait for Monoid, used in Segment Tree.
///
/// A monoid is a set with an associative binary operation and an identity element.
pub trait Monoid {
    /// The type of the elements in the monoid.
    type S: Copy;
    /// The identity element of the monoid.
    fn identity() -> Self::S;
    /// The associative binary operation.
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

/// Dynamic Segment Tree (Point Update, Range Query)
///
/// A data structure that allows for point updates and range queries on a monoid
/// in O(log N) time, where N is the coordinate range. Unlike a standard segment tree,
/// it allocates nodes on-demand, making it suitable for very large ranges (e.g., 0 to 10^18).
///
/// # Examples
///
/// ```
/// struct RangeSum;
/// impl Monoid for RangeSum {
///     type S = i64;
///     fn identity() -> Self::S { 0 }
///     fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S { a + b }
/// }
///
/// // Create a Dynamic Segment Tree for range [0, 10^9)
/// let mut st = DynamicSegTree::<RangeSum>::new(0, 1_000_000_000);
/// st.update(100, 10);
/// st.update(200, 20);
/// assert_eq!(st.query(0, 150), 10);
/// assert_eq!(st.query(0, 300), 30);
/// ```
pub struct DynamicSegTree<M: Monoid> {
    range_l: i64,
    range_r: i64,
    nodes: Vec<Node<M::S>>,
}

struct Node<S> {
    value: S,
    left: Option<usize>,
    right: Option<usize>,
}

impl<M: Monoid> DynamicSegTree<M> {
    /// Creates a new DynamicSegmentTree for the given range [range_l, range_r).
    ///
    /// # Arguments
    /// - `range_l`: The lower bound of the coordinate range (inclusive).
    /// - `range_r`: The upper bound of the coordinate range (exclusive).
    ///
    /// # Complexity
    /// - O(1)
    pub fn new(range_l: i64, range_r: i64) -> Self {
        let root_node = Node {
            value: M::identity(),
            left: None,
            right: None,
        };
        Self {
            range_l,
            range_r,
            nodes: vec![root_node],
        }
    }

    /// Updates the element at index `idx` to `val`.
    ///
    /// # Arguments
    /// - `idx`: The index to update.
    /// - `val`: The new value.
    ///
    /// # Panics
    /// Panics if `idx` is out of the range [range_l, range_r).
    ///
    /// # Complexity
    /// - O(log N), where N is the range width.
    pub fn update(&mut self, idx: i64, val: M::S) {
        assert!(idx >= self.range_l && idx < self.range_r);
        self.update_recursive(0, self.range_l, self.range_r, idx, val);
    }

    /// Queries the result of the binary operation over the range [l, r).
    ///
    /// # Arguments
    /// - `l`: Start index (inclusive).
    /// - `r`: End index (exclusive).
    ///
    /// # Returns
    /// The result of the operation over the given range. 
    /// Returns the identity element if the range is empty or outside the tree's range.
    ///
    /// # Complexity
    /// - O(log N), where N is the range width.
    pub fn query(&mut self, l: i64, r: i64) -> M::S {
        if l >= r || l >= self.range_r || r <= self.range_l {
            return M::identity();
        }
        self.query_recursive(0, self.range_l, self.range_r, l, r)
    }

    fn update_recursive(&mut self, node_idx: usize, node_l: i64, node_r: i64, target_idx: i64, val: M::S) {
        if node_r - node_l == 1 {
            self.nodes[node_idx].value = val;
            return;
        }

        let mid = node_l + (node_r - node_l) / 2;
        if target_idx < mid {
            let left_child = self.get_or_create_left(node_idx);
            self.update_recursive(left_child, node_l, mid, target_idx, val);
        } else {
            let right_child = self.get_or_create_right(node_idx);
            self.update_recursive(right_child, mid, node_r, target_idx, val);
        }

        let left_val = self.nodes[node_idx].left.map_or(M::identity(), |i| self.nodes[i].value);
        let right_val = self.nodes[node_idx].right.map_or(M::identity(), |i| self.nodes[i].value);
        self.nodes[node_idx].value = M::binary_operation(&left_val, &right_val);
    }

    fn query_recursive(&mut self, node_idx: usize, node_l: i64, node_r: i64, l: i64, r: i64) -> M::S {
        if r <= node_l || node_r <= l {
            return M::identity();
        }
        if l <= node_l && node_r <= r {
            return self.nodes[node_idx].value;
        }

        let mid = node_l + (node_r - node_l) / 2;
        let left_val = if let Some(left_idx) = self.nodes[node_idx].left {
            self.query_recursive(left_idx, node_l, mid, l, r)
        } else {
            M::identity()
        };
        let right_val = if let Some(right_idx) = self.nodes[node_idx].right {
            self.query_recursive(right_idx, mid, node_r, l, r)
        } else {
            M::identity()
        };

        M::binary_operation(&left_val, &right_val)
    }

    fn get_or_create_left(&mut self, node_idx: usize) -> usize {
        if let Some(child) = self.nodes[node_idx].left {
            child
        } else {
            let new_idx = self.nodes.len();
            self.nodes.push(Node {
                value: M::identity(),
                left: None,
                right: None,
            });
            self.nodes[node_idx].left = Some(new_idx);
            new_idx
        }
    }

    fn get_or_create_right(&mut self, node_idx: usize) -> usize {
        if let Some(child) = self.nodes[node_idx].right {
            child
        } else {
            let new_idx = self.nodes.len();
            self.nodes.push(Node {
                value: M::identity(),
                left: None,
                right: None,
            });
            self.nodes[node_idx].right = Some(new_idx);
            new_idx
        }
    }
}