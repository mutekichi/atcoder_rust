#![allow(dead_code)]

// --- SNAP START ---

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
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

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
    pub fn is_prime(&self, x: usize) -> bool {
        if x < 2 { return false; }
        self.min_factor[x] == x
    }

    /// Fast prime factorization using the sieve. O(log x)
    pub fn prime_factorize(&self, mut x: usize) -> Vec<(usize, usize)> {
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
        self.min_factor.iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, &p)| if i == p { Some(i) } else { None })
            .collect()
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functions() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(998244353));
        assert!(!is_prime(998244351));

        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);

        assert_eq!(prime_factorize(1), vec![]);
        assert_eq!(prime_factorize(12), vec![(2, 2), (3, 1)]);
        assert_eq!(prime_factorize(1009), vec![(1009, 1)]);
    }

    #[test]
    fn test_sieve() {
        let sieve = Sieve::new(100);
        
        assert!(!sieve.is_prime(0));
        assert!(!sieve.is_prime(1));
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(97));
        assert!(!sieve.is_prime(100));

        assert_eq!(sieve.prime_factorize(1), vec![]);
        assert_eq!(sieve.prime_factorize(12), vec![(2, 2), (3, 1)]);
        assert_eq!(sieve.prime_factorize(60), vec![(2, 2), (3, 1), (5, 1)]);
        
        let primes = sieve.primes();
        assert_eq!(primes[0], 2);
        assert_eq!(primes[1], 3);
        assert_eq!(primes.last(), Some(&97));
    }
}