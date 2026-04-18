#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize, usize); 4] = [(0, 1, 0), (1, 0, 1), (0, -1, 2), (-1, 0, 3)];
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

const DC: [char; 4] = ['L', 'U', 'R', 'D'];

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

#[allow(unused_variables)]
fn main() {
    input! {
        h: usize, w: usize,
        S: [Chars; h],
    }
    let (start, end) = {
        let mut start = (0, 0);
        let mut end = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if S[i][j] == 'S' {
                    start = (i, j);
                } else if S[i][j] == 'G' {
                    end = (i, j);
                }
            }
        }
        (start, end)
    };

    let mut dists = vec![vec![vec![INF_USIZE; 4]; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, INF_USIZE, 0));
    while let Some((i, j, dir, dist)) = queue.pop_front() {
        let mut directions = vec![];
        if (i, j) == end {
            break;
        }
        if S[i][j] == 'o' {
            directions.push(DIR[dir]);
        } else if S[i][j] == 'x' {
            for i in 0..4 {
                if i != dir {
                    directions.push(DIR[i]);
                }
            }
        } else if S[i][j] != '#' {
            for i in 0..4 {
                directions.push(DIR[i]);
            }
        }
        for (ni, nj, dir) in get_next_positions(h, w, i, j, &directions) {
            if dists[ni][nj][dir] == INF_USIZE {
                dists[ni][nj][dir] = dist + 1usize;
                queue.push_back((ni, nj, dir, dist + 1));
            }
        }
    }

    let mut ans = vec![];
    let (i, j) = end;
    let mut dir = INF_USIZE;
    let mut dist = INF_USIZE;
    for d in 0..4 {
        if dists[i][j][d] != INF_USIZE {
            dir = (d + 2) % 4;
            dist = dists[i][j][d];
            md!(dir, dist);
        }
    }
    if dist == INF_USIZE {
        println!("No");
        return;
    }
    println!("Yes");
    let mut ok = false;

    dfs(i, j, h, w, dir, dist, &S, &dists, &mut ans, &mut ok, &start);
}

fn dfs(
    i: usize,
    j: usize,
    h: usize,
    w: usize,
    dir: usize,
    dist: usize,
    S: &Vec<Vec<char>>,
    dists: &Vec<Vec<Vec<usize>>>,
    ans: &mut Vec<usize>,
    ok: &mut bool,
    start: &(usize, usize),
) {
    md!(i, j, dir, dist, S[i][j]);
    if *ok {
        return;
    }
    if (i, j) == *start {
        *ok = true;
        println!("{}", ans.iter().rev().map(|e| DC[*e]).join(""));
        return;
    }

    let mut directions = vec![];
    if S[i][j] == 'G' {
        directions.push(DIR[dir]);
    } else if S[i][j] == 'o' {
        directions.push(DIR[dir]);
    } else if S[i][j] == 'x' {
        for i in 0..4 {
            if i != dir {
                directions.push(DIR[i]);
            }
        }
    } else {
        for i in 0..4 {
            directions.push(DIR[i]);
        }
    }

    for (ni, nj, dir) in get_next_positions(h, w, i, j, &directions) {
        md!(dists[i][j][(dir + 2) % 4]);
        if dists[i][j][(dir + 2) % 4] == dist {
            ans.push(dir);
            dfs(ni, nj, h, w, dir, dist - 1, S, dists, ans, ok, start);
            ans.pop();
        }
    }
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
    directions: &[(isize, isize, usize)],
) -> Vec<(usize, usize, usize)> {
    let mut next_positions = Vec::with_capacity(directions.len());

    for &(di, dj, dir) in directions {
        let next_i = i.wrapping_add_signed(di);
        let next_j = j.wrapping_add_signed(dj);
        if next_i < h && next_j < w {
            next_positions.push((next_i, next_j, dir));
        }
    }
    next_positions
}

// END TEMPLATE INJECTIONS
