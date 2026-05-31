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
const C998244353: u128 = 998244353;
const C1000000007: u128 = 1000000007;

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

    let pf_table = vec![
        vec![(3, 2)],
        vec![(3, 2), (11, 1)],
        vec![(3, 3), (37, 1)],
        vec![(3, 2), (11, 1), (101, 1)],
        vec![(3, 2), (41, 1), (271, 1)],
        vec![(3, 3), (7, 1), (11, 1), (13, 1), (37, 1)],
        vec![(3, 2), (239, 1), (4649, 1)],
        vec![(3, 2), (11, 1), (73, 1), (101, 1), (137, 1)],
        vec![(3, 4), (37, 1), (333667, 1)],
        vec![(3, 2), (11, 1), (41, 1), (271, 1), (9091, 1)],
        vec![(3, 2), (21649, 1), (513239, 1)],
        vec![
            (3, 3),
            (7, 1),
            (11, 1),
            (13, 1),
            (37, 1),
            (101, 1),
            (9901, 1),
        ],
        vec![(3, 2), (53, 1), (79, 1), (265371653, 1)],
        vec![(3, 2), (11, 1), (239, 1), (4649, 1), (909091, 1)],
        vec![(3, 3), (31, 1), (37, 1), (41, 1), (271, 1), (2906161, 1)],
        vec![
            (3, 2),
            (11, 1),
            (17, 1),
            (73, 1),
            (101, 1),
            (137, 1),
            (5882353, 1),
        ],
        vec![(3, 2), (2071723, 1), (5363222357u128, 1)],
        vec![
            (3, 4),
            (7, 1),
            (11, 1),
            (13, 1),
            (19, 1),
            (37, 1),
            (52579, 1),
            (333667, 1),
        ],
        vec![(3, 2), (1111111111111111111, 1)],
        vec![
            (3, 2),
            (11, 1),
            (41, 1),
            (101, 1),
            (271, 1),
            (3541, 1),
            (9091, 1),
            (27961, 1),
        ],
    ];
    for _ in 0..t {
        input! {
            n: u128, m: u128,
        }
        let mut ans = Mint998::new(0);
        let pf_m = prime_factorize(m as u128);
        for keta_y in 1..=20 {
            let mut map = BTreeMap::new();
            for &(prime, exp) in &pf_m {
                map.insert(prime, exp);
            }
            for &(prime, exp) in &pf_table[keta_y - 1] {
                if map.contains_key(&prime) {
                    *map.get_mut(&prime).unwrap() = map.get(&prime).unwrap().saturating_sub(exp);
                }
            }
            let mut baisuu = 1;
            for (prime, exp) in map {
                baisuu *= prime.pow(exp as u32);
            }
            let mut to_add = Mint998::new((n / baisuu) as i64);
            let y_min = 10u128.pow(keta_y as u32 - 1);
            let y_max = min(n, 10u128.pow(keta_y as u32) - 1);
            if y_max >= y_min {
                to_add *= Mint998::new((y_max - y_min + 1) as i64);
                ans += to_add;
            }
        }
        println!("{}", ans);
    }
}

// FOR TEMPLATE INJECTIONS

use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Mint998 = ModInt<998_244_353>;
pub type Mint107 = ModInt<1_000_000_007>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<const M: u128> {
    val: u128,
}

impl<const M: u128> ModInt<M> {
    pub fn new(x: i64) -> Self {
        let mut x = x % M as i64;
        if x < 0 {
            x += M as i64;
        }
        ModInt { val: x as u128 }
    }

    pub fn val(&self) -> u128 {
        self.val
    }

    pub fn pow(
        &self,
        mut exp: u128,
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

    pub fn inv(&self) -> Self {
        self.pow(M - 2)
    }
}

impl<const M: u128> fmt::Display for ModInt<M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u128> fmt::Debug for ModInt<M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u128> From<i64> for ModInt<M> {
    fn from(item: i64) -> Self {
        ModInt::new(item)
    }
}

impl<const M: u128> From<u128> for ModInt<M> {
    fn from(item: u128) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u128> From<usize> for ModInt<M> {
    fn from(item: usize) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u128> From<i32> for ModInt<M> {
    fn from(item: i32) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u128> From<u32> for ModInt<M> {
    fn from(item: u32) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u128> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        ModInt::new(-(self.val as i64))
    }
}

impl<const M: u128> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(0), |a, b| a + b)
    }
}

impl<const M: u128> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(1), |a, b| a * b)
    }
}

impl<const M: u128> Add for ModInt<M> {
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

impl<const M: u128> AddAssign for ModInt<M> {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self + other;
    }
}

impl<const M: u128> Sub for ModInt<M> {
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

impl<const M: u128> SubAssign for ModInt<M> {
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self - other;
    }
}

impl<const M: u128> Mul for ModInt<M> {
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

impl<const M: u128> MulAssign for ModInt<M> {
    fn mul_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self * other;
    }
}

impl<const M: u128> Div for ModInt<M> {
    type Output = Self;
    fn div(
        self,
        other: Self,
    ) -> Self {
        self * other.inv()
    }
}

impl<const M: u128> DivAssign for ModInt<M> {
    fn div_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self / other;
    }
}

macro_rules! impl_modint_ops {
    ($($t:ty),*) => {
        $(
            impl<const M: u128> Add<$t> for ModInt<M> {
                type Output = Self;
                fn add(self, other: $t) -> Self { self + ModInt::from(other) }
            }
            impl<const M: u128> Sub<$t> for ModInt<M> {
                type Output = Self;
                fn sub(self, other: $t) -> Self { self - ModInt::from(other) }
            }
            impl<const M: u128> Mul<$t> for ModInt<M> {
                type Output = Self;
                fn mul(self, other: $t) -> Self { self * ModInt::from(other) }
            }
            impl<const M: u128> Div<$t> for ModInt<M> {
                type Output = Self;
                fn div(self, other: $t) -> Self { self / ModInt::from(other) }
            }
            impl<const M: u128> AddAssign<$t> for ModInt<M> {
                fn add_assign(&mut self, other: $t) { *self = *self + other; }
            }
            impl<const M: u128> SubAssign<$t> for ModInt<M> {
                fn sub_assign(&mut self, other: $t) { *self = *self - other; }
            }
            impl<const M: u128> MulAssign<$t> for ModInt<M> {
                fn mul_assign(&mut self, other: $t) { *self = *self * other; }
            }
            impl<const M: u128> DivAssign<$t> for ModInt<M> {
                fn div_assign(&mut self, other: $t) { *self = *self / other; }
            }
            impl<const M: u128> Add<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn add(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) + other }
            }
            impl<const M: u128> Sub<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn sub(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) - other }
            }
            impl<const M: u128> Mul<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn mul(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) * other }
            }
            impl<const M: u128> Div<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn div(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) / other }
            }
        )*
    };
}

impl_modint_ops!(i32, i64, u32, u128, usize);

impl<const M: u128> proconio::source::Readable for ModInt<M> {
    type Output = Self;
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
        let val = i64::read(source);
        ModInt::new(val)
    }
}

impl<const M: u128> Default for ModInt<M> {
    fn default() -> Self {
        ModInt::new(0)
    }
}

/// Number Theory Utilities
///
/// Includes:
/// - Basic functions for large N (O(sqrt(N))): is_prime, divisors, prime_factorize
/// - Sieve struct for small N precomputation: fast factorization, prime listing
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::math::number_theory::{is_prime, divisors, prime_factorize, Sieve};
///
/// // 1. Basic Functions
/// assert_eq!(is_prime(998244353), true);
/// assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
/// assert_eq!(prime_factorize(12), vec![(2, 2), (3, 1)]);
///
/// // 2. Sieve (Precomputation)
/// let sieve = Sieve::new(100);
/// assert_eq!(sieve.is_prime(97), true);
/// assert_eq!(sieve.prime_factorize(12), vec![(2, 2), (3, 1)]);
/// ```

// ====================================================
// 1. Basic Functions (for large N up to ~10^18)
// ====================================================

/// Checks if n is prime. O(sqrt(n))
pub fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Enumerates all divisors of n. Sorted. O(sqrt(n))
pub fn divisors(n: u128) -> Vec<u128> {
    let mut res = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i * i != n {
                res.push(n / i);
            }
        }
        i += 1;
    }
    res.sort();
    res
}

/// Prime factorization of n. Returns a vector of (prime, exponent). O(sqrt(n))
pub fn prime_factorize(mut n: u128) -> Vec<(u128, usize)> {
    let mut res = Vec::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let mut ex = 0;
            while n % i == 0 {
                n /= i;
                ex += 1;
            }
            res.push((i, ex));
        }
        i += 1;
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

// ====================================================
// 2. Sieve of Eratosthenes (for N up to ~10^6 or 10^7)
// ====================================================

/// Sieve structure for fast prime queries and factorization.
pub struct Sieve {
    min_factor: Vec<usize>, // Smallest prime factor for each number
}

impl Sieve {
    /// Builds the sieve up to n. O(n log log n)
    pub fn new(n: usize) -> Self {
        let mut min_factor: Vec<usize> = (0..=n).collect();

        let mut i = 2;
        while i * i <= n {
            if min_factor[i] == i {
                let mut j = i * i;
                while j <= n {
                    if min_factor[j] == j {
                        min_factor[j] = i;
                    }
                    j += i;
                }
            }
            i += 1;
        }

        Sieve { min_factor }
    }

    /// Checks if x is prime. O(1)
    pub fn is_prime(
        &self,
        x: usize,
    ) -> bool {
        if x < 2 {
            return false;
        }
        self.min_factor[x] == x
    }

    /// Fast prime factorization using the sieve. O(log x)
    pub fn prime_factorize(
        &self,
        mut x: usize,
    ) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        while x > 1 {
            let p = self.min_factor[x];
            let mut ex = 0;
            while x % p == 0 {
                x /= p;
                ex += 1;
            }
            res.push((p, ex));
        }
        res
    }

    /// Returns all primes up to n.
    pub fn primes(&self) -> Vec<usize> {
        self.min_factor
            .iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, &p)| if i == p { Some(i) } else { None })
            .collect()
    }
}

// END TEMPLATE INJECTIONS
