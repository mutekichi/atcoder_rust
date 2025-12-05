#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// Constants
const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const MOD: i64 = 998244353;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

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
    pub fn pow(&self, mut exp: u64) -> Self {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u64> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn add(self, other: Self) -> Self {
        let mut res = self.val + other.val;
        if res >= M {
            res -= M;
        }
        ModInt { val: res }
    }
}

impl<const M: u64> Add<i64> for ModInt<M> {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        self + ModInt::new(other)
    }
}

impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const M: u64> AddAssign<i64> for ModInt<M> {
    fn add_assign(&mut self, other: i64) {
        *self = *self + ModInt::new(other);
    }
}

impl<const M: u64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
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
    fn sub(self, other: i64) -> Self {
        self - ModInt::new(other)
    }
}

impl<const M: u64> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const M: u64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ModInt {
            val: (self.val * other.val) % M,
        }
    }
}

impl<const M: u64> Mul<i64> for ModInt<M> {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        self * ModInt::new(other)
    }
}

impl<const M: u64> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<const M: u64> Div for ModInt<M> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl<const M: u64> DivAssign for ModInt<M> {
    fn div_assign(&mut self, other: Self) {
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

/// Lazy Segment Tree (Range Update, Range Query)
///
/// Processes range updates and range queries in O(log N) for a Monoid with an Operator.
///
/// # Generics
/// - `T`: Type of data (Element of Monoid)
/// - `U`: Type of operator (Element of Operator Monoid)
/// - `F`: Binary operation for data `f(T, T) -> T`
/// - `M`: Function to apply operator to data `mapping(f(T), U) -> T`
/// - `C`: Function to compose operators `composition(f(U), g(U)) -> U` (f ∘ g)
///
/// # Examples
///
/// ## 1. Range Update Query (RUQ) + Range Minimum Query (RMQ)
/// Range update, Range minimum.
/// - Data: `i64` (Inf)
/// - Operator: `i64` (None if specialized value, here we use i64::MAX as 'None')
///
/// ```
/// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
///
/// let inf = std::i64::MAX;
/// let id = std::i64::MAX; // Identity element (represents no update)
///
/// let data = vec![1, 2, 3, 4, 5];
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| std::cmp::min(a, b),                     // op: min
///     inf,                                            // e: inf
///     |acc, f| if f == id { acc } else { f },         // mapping: update if valid
///     |f, g| if f == id { g } else { f },             // composition: overwrite with new op f (f ∘ g)
///     id                                              // id: no update
/// );
///
/// assert_eq!(st.prod(0, 5), 1);
///
/// // Range Update: update [1, 4) to 0 -> [1, 0, 0, 0, 5]
/// st.apply_range(1, 4, 0);
///
/// assert_eq!(st.prod(0, 5), 0);
/// assert_eq!(st.prod(2, 4), 0);
/// assert_eq!(st.get(1), 0);
/// assert_eq!(st.get(4), 5);
/// ```
///
/// ## 2. Range Add Query (RAQ) + Range Sum Query (RSQ)
/// Range add, Range sum.
/// **Note**: When adding uniformly to a range sum, the "length of the range" is required when applying the operator.
/// Therefore, it is standard to include size information in the data type `T`, e.g., `(value, size)`.
///
/// ```
/// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
///
/// // (value, size)
/// #[derive(Clone, Copy, Debug)]
/// struct S { val: i64, size: i64 }
///
/// let data_raw = vec![1, 2, 3, 4, 5];
/// let data: Vec<S> = data_raw.iter().map(|&x| S { val: x, size: 1 }).collect();
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| S { val: a.val + b.val, size: a.size + b.size },     // op: sum
///     S { val: 0, size: 0 },                                      // e
///     |acc, f| S { val: acc.val + f * acc.size, size: acc.size }, // mapping: add (f * width)
///     |f, g| f + g,                                               // composition: sum of additions
///     0                                                           // id: 0
/// );
///
/// assert_eq!(st.prod(0, 5).val, 15);
///
/// // Range Add: add +1 to [0, 5) -> [2, 3, 4, 5, 6]
/// st.apply_range(0, 5, 1);
/// assert_eq!(st.prod(0, 5).val, 20); // 15 + 1*5
/// ```
///
/// ## 3. Range Affine Update + Range Sum Query
/// Range affine transformation (x <- bx + c), Range sum.
/// This also requires range width, so we include size in data type `T`.
///
/// ```
/// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
///
/// #[derive(Clone, Copy, Debug)]
/// struct S { val: i64, size: i64 }
/// // Operator: (b, c) -> x * b + c
/// type F = (i64, i64);
///
/// let data_raw = vec![1, 2, 3, 4, 5];
/// let data: Vec<S> = data_raw.iter().map(|&x| S { val: x, size: 1 }).collect();
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| S { val: a.val + b.val, size: a.size + b.size },
///     S { val: 0, size: 0 },
///     |acc, f| S { val: acc.val * f.0 + f.1 * acc.size, size: acc.size }, // mapping
///     |f, g| (f.0 * g.0, f.0 * g.1 + f.1), // composition: f(g(x)) -> f.b(g.b*x + g.c) + f.c
///     (1, 0) // id: x * 1 + 0
/// );
///
/// // Apply x * 2 + 1 to [0, 5) -> [3, 5, 7, 9, 11]
/// st.apply_range(0, 5, (2, 1));
/// assert_eq!(st.prod(0, 5).val, 35);
/// ```
#[derive(Debug, Clone)]
pub struct LazySegmentTree<T, U, F, M, C> {
    n: usize,
    size: usize,
    log: usize,
    tree: Vec<T>,
    lazy: Vec<U>,
    op: F,
    e: T,
    mapping: M,
    composition: C,
    id: U,
}

impl<T, U, F, M, C> LazySegmentTree<T, U, F, M, C>
where
    T: Copy + Clone + std::fmt::Debug,
    U: Copy + Clone + std::fmt::Debug + PartialEq,
    F: Fn(T, T) -> T,
    M: Fn(T, U) -> T,
    C: Fn(U, U) -> U,
{
    /// Creates a new LazySegmentTree
    /// Usage:
    /// ```
    /// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let mut lst = LazySegmentTree::new(
    ///    &data,
    ///   |a, b| a + b,                    // op
    ///   0,                               // e
    ///  |acc, f| acc + f,                 // mapping
    ///  |f, g| f + g,                     // composition
    ///  0                                // id
    /// );
    ///
    pub fn new(data: &[T], op: F, e: T, mapping: M, composition: C, id: U) -> Self {
        let n = data.len();
        let mut log = 0;
        while (1 << log) < n {
            log += 1;
        }
        let size = 1 << log;

        let mut tree = vec![e; 2 * size];
        let lazy = vec![id; size];

        for (i, &item) in data.iter().enumerate() {
            tree[size + i] = item;
        }
        for i in (1..size).rev() {
            tree[i] = op(tree[2 * i], tree[2 * i + 1]);
        }

        LazySegmentTree {
            n,
            size,
            log,
            tree,
            lazy,
            op,
            e,
            mapping,
            composition,
            id,
        }
    }

    /// Updates the element at index `p` to `x`.
    pub fn set(&mut self, mut p: usize, x: T) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// Gets the element at index `p`.
    pub fn get(&mut self, mut p: usize) -> T {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p]
    }

    /// Queries the aggregate value in range `[l, r)`.
    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e;
        }

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let mut sml = self.e;
        let mut smr = self.e;

        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(sml, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(self.tree[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(sml, smr)
    }

    /// Gets the aggregate value of the entire range.
    pub fn all_prod(&self) -> T {
        self.tree[1]
    }

    /// Applies the operation `f` to the element at index `p`.
    pub fn apply(&mut self, mut p: usize, f: U) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p] = (self.mapping)(self.tree[p], f);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// Applies the operation `f` to all elements in range `[l, r)`.
    pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: U) {
        assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let l2 = l;
        let r2 = r;

        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, f);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }

        l = l2;
        r = r2;

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn update(&mut self, k: usize) {
        self.tree[k] = (self.op)(self.tree[2 * k], self.tree[2 * k + 1]);
    }

    fn all_apply(&mut self, k: usize, f: U) {
        self.tree[k] = (self.mapping)(self.tree[k], f);
        if k < self.size {
            self.lazy[k] = (self.composition)(f, self.lazy[k]);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lazy[k]);
        self.all_apply(2 * k + 1, self.lazy[k]);
        self.lazy[k] = self.id;
    }
}

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

// Logic goes here
#[allow(unused_macros)]
#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        m: usize,
        vec_a: [i64; n],
        lr: [(usize, usize); m],
    }

    type S = (Mint998, i64);
    type F = Option<Mint998>;

    let data: Vec<(Mint998, i64)> = vec_a.into_iter().map(
        |a| (Mint998::new(a), 1)
    ).collect();

    let mut lst = LazySegmentTree::new(
        &data,
        |a: S, b: S| { (a.0 + b.0, a.1 + b.1) },
        (Mint998::new(0), 0),
        |acc, f| {
            match f {
                Some(f) => (f * acc.1, acc.1),
                None => acc,
            }
        },
        |g, f| {
            match g {
                Some(g) => Some(g),
                None => f,
            }
        },
        F::None,
    );

    for &(l, r) in lr.iter() {
        let sum = lst.prod(l-1, r).0;
        md!(l, r, sum);
        let ave = sum / (r - l + 1).into();
        lst.apply_range(l - 1, r, Some(ave));
    }

    let ans = (0..n).into_iter().map(
            |i| { lst.prod(i, i + 1).0.to_string() }
        ).collect::<Vec<String>>().join(" ");
    
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

// Utility functions

// Utility functions
/// Returns valid neighbor coordinates within the grid (h x w).
/// Usage:
/// ```
/// for (nh, nw) in get_next_positions(h, w, hh, ww, &DIR) {
///     // process (nh, nw)
/// }
/// ```
fn get_next_positions(
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<(usize, usize)> {
    let mut next_positions = Vec::with_capacity(directions.len());

    for &(di, dj) in directions {
        let next_i = i.wrapping_add_signed(di);
        let next_j = j.wrapping_add_signed(dj);
        if next_i < h && next_j < w {
            next_positions.push((next_i, next_j));
        }
    }
    next_positions
}
