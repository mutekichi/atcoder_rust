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

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Type alias for ModInt with modulus 998244353
pub type Mint998 = ModInt<998_244_353>;

/// Type alias for ModInt with modulus 1000000007
pub type Mint107 = ModInt<1_000_000_007>;

/// A struct for modular arithmetic.
///
/// Automatically handles modulo operations for addition, subtraction, multiplication, and division.
///
/// # Generics
/// - `M`: The modulus (e.g., 998244353). Must be a prime number for division to work correctly via Fermat's Little Theorem.
///
/// # Examples
///
/// ## 1. Basic Arithmetic
/// ```
/// use atcoder_rust::template::modint::Mint998;
///
/// let a = Mint998::new(10);
/// let b = Mint998::new(20);
///
/// assert_eq!((a + b).val(), 30);
/// assert_eq!((a - b).val(), 998244343); // 10 - 20 + MOD
/// assert_eq!((a * b).val(), 200);
/// assert_eq!(a.pow(3).val(), 1000);
/// ```
///
/// ## 2. Combination (nCr) Calculation
/// ```
/// use atcoder_rust::template::modint::Mint998;
///
/// fn combinations(n: usize, k: usize) -> Mint998 {
///     if k > n { return Mint998::new(0); }
///     
///     let mut num = Mint998::new(1);
///     let mut den = Mint998::new(1);
///     
///     for i in 0..k {
///         num *= (n - i) as i64;
///         den *= (i + 1) as i64;
///     }
///     
///     num / den
/// }
///
/// // 5C2 = 10
/// assert_eq!(combinations(5, 2).val(), 10);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<const M: u64> {
    val: u64,
}

impl<const M: u64> ModInt<M> {
    /// Creates a new `ModInt`.
    ///
    /// # Arguments
    /// - `x`: The initial value. Can be negative.
    pub fn new(x: i64) -> Self {
        let mut x = x % M as i64;
        if x < 0 {
            x += M as i64;
        }
        ModInt { val: x as u64 }
    }

    /// Returns the inner value (guaranteed to be in [0, M)).
    pub fn val(&self) -> u64 {
        self.val
    }

    /// Calculates base^exp % M.
    ///
    /// # Complexity
    /// - O(log exp)
    pub fn pow(
        &self,
        mut exp: u64,
    ) -> Self {
        let mut base = self.val;
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % M;
            }
            base = (base * base) % M;
            exp /= 2;
        }
        ModInt { val: res }
    }

    /// Calculates the modular inverse using Fermat's Little Theorem.
    ///
    /// # Note
    /// - Requires `M` to be prime.
    pub fn inv(&self) -> Self {
        self.pow(M - 2)
    }
}

// --- Trait Implementations ---

impl<const M: u64> fmt::Display for ModInt<M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u64> fmt::Debug for ModInt<M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u64> From<i64> for ModInt<M> {
    fn from(item: i64) -> Self {
        ModInt::new(item)
    }
}

impl<const M: u64> From<u64> for ModInt<M> {
    fn from(item: u64) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> From<usize> for ModInt<M> {
    fn from(item: usize) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> From<i32> for ModInt<M> {
    fn from(item: i32) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> Add for ModInt<M> {
    type Output = Self;
    fn add(
        self,
        other: Self,
    ) -> Self {
        let mut res = self.val + other.val;
        if res >= M {
            res -= M;
        }
        ModInt { val: res }
    }
}

impl<const M: u64> Add<i64> for ModInt<M> {
    type Output = Self;
    fn add(
        self,
        other: i64,
    ) -> Self {
        self + ModInt::new(other)
    }
}

impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self + other;
    }
}

impl<const M: u64> AddAssign<i64> for ModInt<M> {
    fn add_assign(
        &mut self,
        other: i64,
    ) {
        *self = *self + ModInt::new(other);
    }
}

impl<const M: u64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(
        self,
        other: Self,
    ) -> Self {
        let mut res = self.val;
        if res < other.val {
            res += M;
        }
        res -= other.val;
        ModInt { val: res }
    }
}

impl<const M: u64> Sub<i64> for ModInt<M> {
    type Output = Self;
    fn sub(
        self,
        other: i64,
    ) -> Self {
        self - ModInt::new(other)
    }
}

impl<const M: u64> SubAssign for ModInt<M> {
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self - other;
    }
}

impl<const M: u64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(
        self,
        other: Self,
    ) -> Self {
        ModInt {
            val: (self.val * other.val) % M,
        }
    }
}

impl<const M: u64> Mul<i64> for ModInt<M> {
    type Output = Self;
    fn mul(
        self,
        other: i64,
    ) -> Self {
        self * ModInt::new(other)
    }
}

impl<const M: u64> MulAssign for ModInt<M> {
    fn mul_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self * other;
    }
}

impl<const M: u64> Div for ModInt<M> {
    type Output = Self;
    fn div(
        self,
        other: Self,
    ) -> Self {
        self * other.inv()
    }
}

impl<const M: u64> DivAssign for ModInt<M> {
    fn div_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self / other;
    }
}

// Enable parsing from input (using proconio)
impl<const M: u64> proconio::source::Readable for ModInt<M> {
    type Output = Self;
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
        let val = i64::read(source);
        ModInt::new(val)
    }
}

impl<const M: u64> Default for ModInt<M> {
    fn default() -> Self {
        ModInt::new(0)
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
        A: [Usize1; n],
    }

    let mut st_n = SegmentTree::new(&vec![Mint998::new(0); n], |a, b| a + b, Mint998::new(0));
    let mut st_m = SegmentTree::new(&vec![Mint998::new(0); n], |a, b| a + b, Mint998::new(0));
    let mut st_p = SegmentTree::new(&vec![Mint998::new(0); n], |a, b| a + b, Mint998::new(0));

    for i in 0..n {
        let a = A[i];
        let new_n = st_n.query(0, a) + st_m.query(0, a) + st_p.query(0, a);
        let new_m = st_m.query(a, n) + st_n.query(a, n);
        let new_p = Mint998::new(1);
        st_n.update(a, new_n);
        st_m.update(a, new_m);
        st_p.update(a, new_p);
    }
    let mut ans = Mint998::new(0);
    for i in 0..n {
        ans += st_m.get(i);
        md!(st_m.get(i));
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
