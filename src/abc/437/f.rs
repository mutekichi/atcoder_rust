#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

/// Segment Tree (Point Update, Range Query)
///
/// A data structure that allows for point updates and range queries on a monoid
/// (a set with an associative binary operation and an identity element) in O(log N) time.
///
/// # Generics
/// - `T`: The type of elements in the segment tree. Must implement `Clone` and `Debug`.
/// - `F`: The type of the binary operation function. Must implement `Fn(T, T) -> T`.
///
/// # Examples
///
/// ## 1. Comprehensive Usage: Range Minimum Query (RMQ)
///
/// This example demonstrates the full API usage (initialization, query, update, get)
/// for a typical Range Minimum Query problem.
///
/// ```
/// // Assuming this struct is defined in crate::template::segment_tree
/// use atcoder_rust::template::segment_tree::SegmentTree;
///
/// fn main() {
///     let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
///     let inf = std::i64::MAX;
///
///     // 1. Initialization: Create a Segment Tree for RMQ (min operation)
///     // - data: Initial array
///     // - op: Binary operation function (min)
///     // - identity: Identity element (infinity)
///     let mut st = SegmentTree::new(
///         &data,
///         |a, b| std::cmp::min(a, b),
///         inf
///     );
///
///     // 2. Query: Find the minimum in range [0, 8) -> entire array
///     assert_eq!(st.query(0, 8), 1);
///
///     // Query: Find the minimum in range [2, 5) -> sub-array [4, 1, 5]
///     assert_eq!(st.query(2, 5), 1);
///
///     // 3. Get: Retrieve the current value at a specific index
///     assert_eq!(st.get(2), 4);
///     assert_eq!(st.get(6), 2);
///
///     // 4. Update: Change the value at index 2 (was 4) to 0
///     // New state: [3, 1, 0, 1, 5, 9, 2, 6]
///     st.update(2, 0);
///
///     // Verify update with Get
///     assert_eq!(st.get(2), 0);
///
///     // Verify update with Query: Range [2, 5) is now [0, 1, 5]
///     assert_eq!(st.query(2, 5), 0);
///
///     // Verify update affects wider ranges
///     assert_eq!(st.query(0, 8), 0);
/// }
/// ```
///
/// ## 2. Initialization Patterns for Various Monoids
///
/// Below are initialization examples for different types of problems.
/// Assumes `data` is a `Vec<T>` and `SegmentTree::new(data, op, identity)` is called.
///
/// ### Range Sum Query (RSQ)
/// - Problem: Calculate the sum of elements in a range.
/// - Identity: `0`
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| a + b, 0);
/// ```
///
/// ### Range Maximum Query
/// - Problem: Find the maximum value in a range.
/// - Identity: `std::i64::MIN` (or a sufficiently small value)
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| std::cmp::max(a, b), std::i64::MIN);
/// ```
///
/// ### Range Product Query
/// - Problem: Calculate the product of elements.
/// - Identity: `1`
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| a * b, 1);
/// ```
///
/// ### Range XOR Query
/// - Problem: Calculate the bitwise XOR sum.
/// - Identity: `0`
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| a ^ b, 0);
/// ```
///
/// ### Range OR Query
/// - Problem: Calculate the bitwise OR sum.
/// - Identity: `0`
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| a | b, 0);
/// ```
///
/// ### Range AND Query
/// - Problem: Calculate the bitwise AND sum.
/// - Identity: `!0` (All bits set to 1, e.g., `std::u64::MAX`)
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| a & b, std::u64::MAX);
/// ```
///
/// ### Range GCD Query (Greatest Common Divisor)
/// - Problem: Calculate the GCD of elements.
/// - Identity: `0` (Since gcd(x, 0) = x)
/// ```ignore
/// fn gcd(a: u64, b: u64) -> u64 {
///     if b == 0 { a } else { gcd(b, a % b) }
/// }
/// let st = SegmentTree::new(&data, |a, b| gcd(a, b), 0);
/// ```
///
/// ### Range LCM Query (Least Common Multiple)
/// - Problem: Calculate the LCM of elements.
/// - Identity: `1` (Since lcm(x, 1) = x)
/// ```ignore
/// fn lcm(a: u64, b: u64) -> u64 {
///     if a == 0 || b == 0 { 0 } else { (a * b) / gcd(a, b) }
/// }
/// let st = SegmentTree::new(&data, |a, b| lcm(a, b), 1);
/// ```
///
/// ### Range Affine Transformation (Composition of Linear Functions)
/// - Problem: Combine linear functions `f(x) = ax + b`.
/// - Type: `(i64, i64)` representing `a` and `b`.
/// - Operation: Compose `f1(x) = a1*x + b1` and `f2(x) = a2*x + b2`.
///   `f2(f1(x)) = a2(a1*x + b1) + b2 = (a2*a1)x + (a2*b1 + b2)`
/// - Identity: `(1, 0)` (Identity function `f(x) = 1*x + 0`)
/// ```ignore
/// let st = SegmentTree::new(
///     &data, // Vec<(i64, i64)>
///     |f1, f2| (f2.0 * f1.0, f2.0 * f1.1 + f2.1),
///     (1, 0)
/// );
/// ```
///
/// ### Range Matrix Multiplication
/// - Problem: Product of matrices (often used for dynamic DP updates).
/// - Identity: Identity Matrix
/// ```ignore
/// // Assuming Matrix struct and matmul function are defined
/// let identity_matrix = Matrix::identity();
/// let st = SegmentTree::new(
///     &data, // Vec<Matrix>
///     |a, b| a.matmul(&b),
///     identity_matrix
/// );
/// ```
///
/// ### String Concatenation
/// - Problem: Concatenate strings.
/// - Identity: `""` (Empty string)
/// - Note: Can be slow due to memory allocation; implies monoid structure.
/// ```ignore
/// let st = SegmentTree::new(&data, |a, b| a.clone() + &b, String::new());
/// ```
#[derive(Debug, Clone)]
pub struct SegmentTree<T, F> {
    n: usize,
    size: usize,
    tree: Vec<T>,
    op: F,
    identity: T,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    /// Creates a new SegmentTree from the given data.
    ///
    /// # Arguments
    /// - `data`: Initial data slice.
    /// - `op`: An associative binary operation function `f(x, y)`.
    /// - `identity`: The identity element `e` such that `f(x, e) = f(e, x) = x`.
    ///
    /// # Complexity
    /// - O(N)
    pub fn new(
        data: &[T],
        op: F,
        identity: T,
    ) -> Self {
        let n = data.len();
        let mut size = 1;
        while size < n {
            size *= 2;
        }

        // Allocate vector of size 2*size.
        // The tree is 1-indexed for implementation convenience (index 1 is the root).
        let mut tree = vec![identity.clone(); 2 * size];

        // Initialize leaves (indices size to size+n-1)
        for (i, item) in data.iter().enumerate() {
            tree[size + i] = item.clone();
        }

        // Build the tree by updating parents from leaves to root
        for i in (1..size).rev() {
            tree[i] = op(tree[2 * i].clone(), tree[2 * i + 1].clone());
        }

        SegmentTree {
            n,
            size,
            tree,
            op,
            identity,
        }
    }

    /// Updates the element at index `p` to `value`.
    ///
    /// # Arguments
    /// - `p`: Index to update (0-based).
    /// - `value`: The new value.
    ///
    /// # Panics
    /// Panics if `p` is out of bounds.
    ///
    /// # Complexity
    /// - O(log N)
    pub fn update(
        &mut self,
        p: usize,
        value: T,
    ) {
        assert!(p < self.n, "Index out of bounds");

        // Update leaf node
        let mut idx = p + self.size;
        self.tree[idx] = value;

        // Update parents up to the root
        while idx > 1 {
            idx /= 2;
            self.tree[idx] = (self.op)(self.tree[2 * idx].clone(), self.tree[2 * idx + 1].clone());
        }
    }

    /// Queries the result of the binary operation over the range `[l, r)`.
    ///
    /// # Arguments
    /// - `l`: Start index (inclusive, 0-based).
    /// - `r`: End index (exclusive, 0-based).
    ///
    /// # Returns
    /// The result of the operation over elements from index `l` to `r-1`.
    /// Returns the identity element if the range is empty.
    ///
    /// # Panics
    /// Panics if `l > r` or `r > n`.
    ///
    /// # Complexity
    /// - O(log N)
    pub fn query(
        &self,
        l: usize,
        r: usize,
    ) -> T {
        assert!(l <= r && r <= self.n, "Invalid range");

        let mut result_left = self.identity.clone();
        let mut result_right = self.identity.clone();

        let mut l = l + self.size;
        let mut r = r + self.size;

        while l < r {
            if l & 1 == 1 {
                result_left = (self.op)(result_left, self.tree[l].clone());
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                result_right = (self.op)(self.tree[r].clone(), result_right);
            }
            l /= 2;
            r /= 2;
        }

        (self.op)(result_left, result_right)
    }

    /// Gets the current value at index `p`.
    ///
    /// # Arguments
    /// - `p`: Index (0-based).
    ///
    /// # Panics
    /// Panics if `p` is out of bounds.
    ///
    /// # Complexity
    /// - O(1)
    pub fn get(
        &self,
        p: usize,
    ) -> T {
        assert!(p < self.n, "Index out of bounds");
        self.tree[p + self.size].clone()
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
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        q: usize,
        XY: [(i64, i64); n],
    }

    let mut data = vec![];
    for (x, y) in XY {
        data.push((x + y, x + y, x - y, x - y));
    }
    let mut st = SegmentTree::new(
        &data,
        |p, q| {
            (max(p.0, q.0), min(p.1, q.1), max(p.2, q.2), min(p.3, q.3))
        },
        (-INF_I64, INF_I64, -INF_I64, INF_I64),
    );
    
    for _ in 0..q {
        input! {
            qtype: usize,
        }
        if qtype == 1 {
            input! {
                i: Usize1,
                x: i64,
                y: i64,
            }
            st.update(i, (x + y, x + y, x - y, x - y));
        } else {
            input! {
                L: Usize1,
                R: Usize1,
                x: i64,
                y: i64,
            }
            let w = x + y;
            let v = x - y;
            let (wmax, wmin, vmax, vmin) = st.query(L, R + 1);
            md!(wmax, wmin, vmax, vmin);
            let alt1 = (w - wmin).abs();
            let alt2 = (wmax - w).abs();
            let alt3 = (v - vmin).abs();
            let alt4 = (vmax - v).abs();
            wl!(max(max(alt1, alt2), max(alt3, alt4)));
        }

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
