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
        q: usize,
        A: [usize; n],
        LRX: [(Usize1, usize, usize); q],
    }
    let invs = (0..=(n + 2))
        .map(|i| {
            if i == 0 {
                Mint998::new(1)
            } else {
                Mint998::new(1) / Mint998::new(i as i64)
            }
        })
        .collect::<Vec<_>>();
    let mut factorials = vec![Mint998::new(1); n + 2];
    for i in 1..=(n + 1) {
        factorials[i] = factorials[i - 1] * Mint998::new(i as i64);
    }
    let mut inv_factorials = vec![Mint998::new(1); n + 2];
    for i in 1..=(n + 1) {
        inv_factorials[i] = inv_factorials[i - 1] * invs[i];
    }

    let bsize_q = 512;
    let mut query_ord = (0..q).collect::<Vec<_>>();
    query_ord.sort_by_cached_key(|x| {
        let (l, r, _) = LRX[*x];
        let lb = l / bsize_q;
        let rkey = if lb % 2 == 0 { r } else { n - r };
        (lb, rkey)
    });

    let bsize_d = 512;
    let mut counts = vec![0usize; n + 2];
    let mut range_data = vec![(0, Mint998::new(1)); n / bsize_d + 1];
    let mut current_r = 0;
    let mut current_l = 0;
    let mut ans = vec![Mint998::new(0); q];

    macro_rules! add {
        ($i:expr) => {
            let a = A[$i];
            md!(a);
            counts[a] += 1;
            range_data[a / bsize_d].0 += 1;
            range_data[a / bsize_d].1 *= invs[counts[a]];
        };
    }
    macro_rules! sub {
        ($i: expr) => {
            let a = A[$i];
            counts[a] -= 1;
            range_data[a / bsize_d].0 -= 1;
            range_data[a / bsize_d].1 *= Mint998::new(counts[a] as i64 + 1);
        };
    }

    for q_index in query_ord {
        let (query_l, query_r, x) = LRX[q_index];
        // query_l < current_l
        for i in query_l..current_l {
            add!(i);
        }
        for i in current_r..query_r {
            md!(current_r, query_r, i);
            add!(i);
        }
        for i in current_l..query_l {
            sub!(i);
        }
        for i in query_r..current_r {
            sub!(i);
        }
        let mut count_sum = 0;
        let mut inv_fact_prod = Mint998::new(1);
        let d_batch_id = x / bsize_d;
        for i in 0..d_batch_id {
            count_sum += range_data[i].0;
            inv_fact_prod *= range_data[i].1;
        }
        for i in (d_batch_id * bsize_d)..x {
            count_sum += counts[i];
            inv_fact_prod *= inv_factorials[counts[i]];
        }

        ans[q_index] = factorials[count_sum] * inv_fact_prod;
        current_r = query_r;
        current_l = query_l;
    }
    for a in ans {
        wl!(a);
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
