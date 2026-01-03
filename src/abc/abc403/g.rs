#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

/// Trait for Monoid, used in Segment Tree.
///
/// A monoid is a set with an associative binary operation and an identity element.
pub trait Monoid {
    /// The type of the elements in the monoid.
    type S: Copy;
    /// The identity element of the monoid.
    fn identity() -> Self::S;
    /// The associative binary operation.
    fn binary_operation(
        a: &Self::S,
        b: &Self::S,
    ) -> Self::S;
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
    pub fn new(
        range_l: i64,
        range_r: i64,
    ) -> Self {
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
    pub fn update(
        &mut self,
        idx: i64,
        val: M::S,
    ) {
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
    pub fn query(
        &mut self,
        l: i64,
        r: i64,
    ) -> M::S {
        if l >= r || l >= self.range_r || r <= self.range_l {
            return M::identity();
        }
        self.query_recursive(0, self.range_l, self.range_r, l, r)
    }

    fn update_recursive(
        &mut self,
        node_idx: usize,
        node_l: i64,
        node_r: i64,
        target_idx: i64,
        val: M::S,
    ) {
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

        let left_val = self.nodes[node_idx]
            .left
            .map_or(M::identity(), |i| self.nodes[i].value);
        let right_val = self.nodes[node_idx]
            .right
            .map_or(M::identity(), |i| self.nodes[i].value);
        self.nodes[node_idx].value = M::binary_operation(&left_val, &right_val);
    }

    fn query_recursive(
        &mut self,
        node_idx: usize,
        node_l: i64,
        node_r: i64,
        l: i64,
        r: i64,
    ) -> M::S {
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

    fn get_or_create_left(
        &mut self,
        node_idx: usize,
    ) -> usize {
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

    fn get_or_create_right(
        &mut self,
        node_idx: usize,
    ) -> usize {
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

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        q: usize,
        Y: [i64; q],
    }
    struct Data;
    impl Monoid for Data {
        type S = (usize, i64, i64);
        fn identity() -> Self::S { (0, 0, 0) }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (
                a.0 + b.0,
                if a.0 % 2 == 0 {
                    a.1 + b.1
                } else {
                    a.1 + b.2
                },
                if a.0 % 2 == 0 {
                    a.2 + b.2 
                } else {
                    a.2 + b.1
                }
            )
        }
    }

    let mut value = 0i64;
    let ten9 = 1000000000;
    let mut dst = DynamicSegTree::<Data>::new(0, ten9 + 3);
    for y in Y {
        value = (value + y) % ten9 + 1;
        md!(value);
        let original = dst.query(value, value + 1);
        if original == (0, 0, 0) {
            dst.update(value, (1, value, 0));
        }
        else {
            let (count, odd_sum, even_sum) = original;
            if count % 2 == 0 {
                dst.update(value, (count + 1, odd_sum + value, even_sum));
            } else {
                dst.update(value, (count + 1, odd_sum, even_sum + value));
            }
        }
        let ans = dst.query(0, ten9 + 2).1;
        value = ans;
        wl!(ans);
    }
}

// --- Macros ---

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

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint without newline)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mep {
    ($x:expr) => { eprint!("{}", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mep {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint with space)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mepw { // stands for my_eprint_whitespace
    ($x:expr) => { eprint!("{} ", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mepw {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

fn join_with_space<T: ToString>(arr: &[T]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
