#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use rand::Rng;
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
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

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

    /// Returns the largest index `r` in `[l, n]` such that `f(op(a[l], a[l+1], ..., a[r-1]))` is true.
    ///
    /// The predicate `f` must be monotonic. That is, if `f` returns true for a range,
    /// it must also return true for any sub-range starting at `l`.
    /// Specifically, `f(e)` must be true.
    ///
    /// # Arguments
    /// * `l` - The starting index (inclusive).
    /// * `f` - A predicate function `Fn(T) -> bool`.
    ///
    /// # Returns
    /// The index `r`. The range satisfying the condition is `[l, r)`.
    /// If `l == n`, returns `n`.
    ///
    /// # Panics
    /// Panics if `l > n`.
    ///
    /// # Complexity
    /// O(log N)
    pub fn max_right<P>(
        &self,
        mut l: usize,
        f: P,
    ) -> usize
    where
        P: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(self.identity.clone()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = self.identity.clone();
        loop {
            // Remove factors of 2 to move up the tree
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f((self.op)(sm.clone(), self.tree[l].clone())) {
                // If condition fails, dive down to find the exact boundary
                while l < self.size {
                    l = 2 * l;
                    if f((self.op)(sm.clone(), self.tree[l].clone())) {
                        sm = (self.op)(sm, self.tree[l].clone());
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = (self.op)(sm.clone(), self.tree[l].clone());
            l += 1;
            // Break if l is a power of 2 (reached the right edge of a subtree)
            if (l & (l.wrapping_neg())) == l {
                break;
            }
        }
        self.n
    }

    /// Returns the smallest index `l` in `[0, r]` such that `f(op(a[l], a[l+1], ..., a[r-1]))` is true.
    ///
    /// The predicate `f` must be monotonic. That is, if `f` returns true for a range `[l, r)`,
    /// it must also return true for any sub-range ending at `r`.
    /// Specifically, `f(e)` must be true.
    ///
    /// # Arguments
    /// * `r` - The ending index (exclusive).
    /// * `f` - A predicate function `Fn(T) -> bool`.
    ///
    /// # Returns
    /// The index `l`. The range satisfying the condition is `[l, r)`.
    /// If `r == 0`, returns `0`.
    ///
    /// # Panics
    /// Panics if `r > n`.
    ///
    /// # Complexity
    /// O(log N)
    pub fn min_left<P>(
        &self,
        mut r: usize,
        f: P,
    ) -> usize
    where
        P: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(self.identity.clone()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = self.identity.clone();
        loop {
            r -= 1;
            while r > 1 && (r % 2 == 1) {
                r >>= 1;
            }
            if !f((self.op)(self.tree[r].clone(), sm.clone())) {
                while r < self.size {
                    r = 2 * r + 1;
                    if f((self.op)(self.tree[r].clone(), sm.clone())) {
                        sm = (self.op)(self.tree[r].clone(), sm.clone());
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = (self.op)(self.tree[r].clone(), sm.clone());
            if (r & (r.wrapping_neg())) == r {
                break;
            }
        }
        0
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
        n: usize,
        S: Chars,
    }
    let mut st = SegmentTree::new(&vec![0i64; n * 2 + 2], |a, b| a + b, 0i64);
    let mut cur = n + 1;
    st.update(cur, st.get(cur) + 1);
    let mut ans = 0;
    for i in 0..n {
        if S[i] == 'A' {
            cur -= 1;
        } else if S[i] == 'B' {
            cur += 1;
        }
        ans += st.query(cur + 1, n * 2 + 2);
        st.update(cur, st.get(cur) + 1);
    }
    wl!(ans);
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

trait JoinExtended {
    fn join_with(
        self,
        sep: &str,
    ) -> String;
}

impl<I> JoinExtended for I
where
    I: Iterator,
    I::Item: Joinable,
{
    fn join_with(
        self,
        sep: &str,
    ) -> String {
        let mut peekable = self.peekable();
        let is_2d = if let Some(first) = peekable.peek() {
            first.is_container()
        } else {
            false
        };

        let res = peekable.map(|item| item.join_item(sep)).collect::<Vec<_>>();

        // Use newline for 2D rows, provided sep for 1D elements
        res.join(if is_2d { "\n" } else { sep })
    }
}

trait Joinable {
    fn join_item(
        &self,
        sep: &str,
    ) -> String;
    fn is_container(&self) -> bool;
}

macro_rules! impl_joinable_scalar {
    ($($t:ty),*) => {
        $(
            impl Joinable for &$t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
            impl Joinable for $t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
        )*
    };
}

impl_joinable_scalar!(
    i32, i64, i128, u32, u64, u128, usize, isize, f32, f64, char, String, &str
);

impl<T: std::fmt::Display> Joinable for &Vec<T> {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}

impl<T: std::fmt::Display> Joinable for &[T] {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}
