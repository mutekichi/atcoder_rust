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

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        A: [usize; n],
    }
    let sieve = Sieve::new(200_010);
    let mut counter = BTreeMap::new();
    let mut zeros = 0;
    for a in A {
        if a == 0 {
            zeros += 1;
            continue;
        }
        let pf = sieve.prime_factorize(a);
        let mut nums = vec![];
        for (num, exp) in pf {
            if exp % 2 == 1 {
                nums.push(num);
            }
        }
        *counter.entry(nums).or_insert(0) += 1;
    }
    let mut ans = 0usize;
    if zeros > 0 {
        ans += zeros * (n - 1) - zeros * (zeros - 1) / 2;
    }
    for (vec, count) in counter {
        md!(vec.iter().join(" "), count);
        ans += count * (count - 1) / 2;
    }
    println!("{}", ans);
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIR8: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
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

// FOR TEMPLATE INJECTIONS

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
pub fn is_prime(n: u64) -> bool {
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
pub fn divisors(n: u64) -> Vec<u64> {
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
pub fn prime_factorize(mut n: u64) -> Vec<(u64, usize)> {
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
