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
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

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
        t: usize,
    }

    let mut comb: Combination<C998244353> = Combination::new(103);

    for _ in 0..t {
        input! {
            n: usize,
            c: i64,
            mut A: [i64; n],
        }
        A.sort_unstable();
        let mut data = vec![];
        let mut tmp_prev = 0;
        let mut tmp_count = 0;

        for &a in A.iter() {
            if a == tmp_prev {
                tmp_count += 1;
            } else {
                if tmp_prev != 0 {
                    data.push((tmp_prev, tmp_count));
                }
                tmp_prev = a;
                tmp_count = 1;
            }
        }
        data.push((tmp_prev, tmp_count));

        let mut ans = vec![];
        let mut accum_prob = Mint998::new(0);
        let mut accum_count = 0;

        for val in A {
            let prob = comb.ncr(n, accum_count)
                * pow(
                    Mint998::new(val - 1) / Mint998::new(c as i64),
                    n - accum_count,
                )
                * pow(
                    Mint998::new(c - val + 1) / Mint998::new(c as i64),
                    accum_count,
                );
            accum_count += 1;
            ans.push(prob - accum_prob);
            accum_prob = prob;
        }

        ans.push(Mint998::new(1) - accum_prob);

        println!("{}", ans.iter().join(" "));
    }
}

fn pow(
    a: Mint998,
    b: usize,
) -> Mint998 {
    let mut res = Mint998::new(1);
    for _ in 0..b {
        res *= a;
    }
    res
}

// FOR TEMPLATE INJECTIONS

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

type Mint998 = ModInt<998244353>;

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

/// Combination utilities using precomputed factorials and inverse factorials.
///
/// Works with `ModInt`.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::math::modint::Mint998;
/// use atcoder_rust::template::math::combination::Combination;
///
/// let comb = Combination::<998244353>::new(1000);
///
/// // 5C2 = 10
/// assert_eq!(comb.ncr(5, 2).val(), 10);
/// // 5P2 = 20
/// assert_eq!(comb.npr(5, 2).val(), 20);
/// ```
pub struct Combination<const M: u64> {
    fact: Vec<ModInt<M>>,
    inv_fact: Vec<ModInt<M>>,
}

impl<const M: u64> Combination<M> {
    /// Precomputes factorials up to `max_n`. Complexity: O(max_n)
    pub fn new(max_n: usize) -> Self {
        let mut fact = vec![ModInt::new(1); max_n + 1];
        let mut inv_fact = vec![ModInt::new(1); max_n + 1];

        for i in 1..=max_n {
            fact[i] = fact[i - 1] * (i as i64);
        }

        inv_fact[max_n] = fact[max_n].inv();
        for i in (1..=max_n).rev() {
            inv_fact[i - 1] = inv_fact[i] * (i as i64);
        }

        Combination { fact, inv_fact }
    }

    /// Calculates nCr (Combinations). O(1)
    pub fn ncr(
        &self,
        n: usize,
        r: usize,
    ) -> ModInt<M> {
        if r > n {
            return ModInt::new(0);
        }
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }

    /// Calculates nPr (Permutations). O(1)
    pub fn npr(
        &self,
        n: usize,
        r: usize,
    ) -> ModInt<M> {
        if r > n {
            return ModInt::new(0);
        }
        self.fact[n] * self.inv_fact[n - r]
    }

    /// Calculates nHr (Homogeneous Combinations). O(1)
    /// nHr = (n+r-1)Cr
    pub fn nhr(
        &self,
        n: usize,
        r: usize,
    ) -> ModInt<M> {
        if n == 0 && r == 0 {
            return ModInt::new(1);
        }
        self.ncr(n + r - 1, r)
    }

    /// Returns n! (Factorial). O(1)
    pub fn fact(
        &self,
        n: usize,
    ) -> ModInt<M> {
        self.fact[n]
    }
}
