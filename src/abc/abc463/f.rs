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
        n: usize,
        A: [usize; 2 * n],
    }
    if A.iter().all(|a| *a == A[0]) {
        let mut ans = vec![];
        for i in 0..2 * n {
            ans.push(Mint998::new(1) / Mint998::new(2 * n as i64));
        }
        println!("{}", ans.iter().join(" "));
        return;
    }
    let max_val = *A.iter().max().unwrap();
    let mut has_two_patterns = true;
    let mut vals = vec![];

    let mut c00 = 0;
    let mut c01 = 0;
    let mut c02 = 0;
    let mut c11 = 0;
    let mut c12 = 0;

    for i in 0..n {
        let a = A[2 * i];
        let b = A[2 * i + 1];
        let mut val = vec![];
        for x in vec![a, b] {
            val.push(if x == max_val {
                0
            } else if x == max_val - 1 {
                1
            } else {
                2
            });
        }
        val.sort();
        if val[0] == 0 && val[1] == 0 {
            has_two_patterns = false;
            c00 += 1;
        } else if val[0] == 0 && val[1] == 1 {
            c01 += 1;
        } else if val[0] == 0 && val[1] == 2 {
            c02 += 1;
        } else if val[0] == 1 && val[1] == 1 {
            c11 += 1;
        } else if val[0] == 1 && val[1] == 2 {
            c12 += 1;
        }

        vals.push(val);
    }
    md!(c00, c01, c02, c11, c12);
    let mut ans_vec = vec![Mint998::new(0); 2 * n];
    let comb = Combination::<C998244353>::new(2 * n);

    // pattern 1: score == max_val
    if has_two_patterns {
        md!("has");
        assert_eq!(c00, 0);
        // for 11
        {
            let nn = n;
            let mut ans = Mint998::new(1);
            // c01 のところは 1 が勝つ前提
            let n = c12 as usize;
            for i in 0..=n as i64 {
                ans += comb.ncr(n, i as usize) / Mint998::new(c11 + c01 + i);
            }
            // target c11
            ans /= Mint998::new(2);
            // c12
            for i in 0..n {
                ans /= Mint998::new(2);
            }
            // c01, c02 はすべて 0 でない側が勝つ
            for i in 0..(c01 + c02) {
                ans /= Mint998::new(2);
            }
            for i in 0..nn {
                if vals[i][0] == 1 && vals[i][1] == 1 {
                    md!(i);
                    ans_vec[i * 2] += ans;
                    ans_vec[i * 2 + 1] += ans;
                }
            }
        }
        // for 01
        {
            let nn = n;
            let mut ans = Mint998::new(1);
            let n = c12 as usize;
            for i in 0..=n {
                ans += comb.ncr(n, i as usize) / Mint998::new(c11 + c01 + i as i64);
            }
            for i in 0..n {
                ans /= Mint998::new(2);
            }
            // c01, c02 はすべて 0 でない側が勝つ
            for i in 0..(c01 + c02) {
                ans /= Mint998::new(2);
            }
            for i in 0..nn {
                if vals[i][0] == 0 && vals[i][1] == 1 {
                    let idx = vec![i * 2, i * 2 + 1];
                    for idx in idx {
                        if A[idx] == max_val - 1 {
                            ans_vec[idx] += ans;
                        }
                    }
                }
            }
        }
        // for 12
        {
            let nn = n;
            let mut ans = Mint998::new(1);
            let n = c12 as usize;
            for i in 0..=n - 1 {
                ans += comb.ncr(n - 1, i as usize) / Mint998::new(c11 + c01 + i as i64);
            }
            for i in 0..n {
                ans /= Mint998::new(2);
            }
            for i in 0..(c01 + c02) {
                ans /= Mint998::new(2);
            }
            for i in 0..nn {
                if vals[i][0] == 1 && vals[i][1] == 2 {
                    let idx = vec![i * 2, i * 2 + 1];
                    for idx in idx {
                        if A[idx] == max_val - 1 {
                            ans_vec[idx] += ans;
                        }
                    }
                }
            }
        }
    }

    // pattern 2: score == max_val + 1
    // for 00
    {
        let nn = n;
        let mut ans = Mint998::new(1);
        let n = c01 + c02;
        for i in 0..=n as i64 {
            ans += comb.ncr(n as usize, i as usize) / Mint998::new(c00 + i);
        }
        for i in 0..n + 1 {
            ans /= Mint998::new(2);
        }
        for i in 0..nn {
            if vals[i][0] == 0 && vals[i][1] == 0 {
                ans_vec[i * 2] += ans;
                ans_vec[i * 2 + 1] += ans;
            }
        }
    }

    // for 01 / 02
    {
        let nn = n;
        let mut ans = Mint998::new(1);
        let n = c01 + c02;
        for i in 0..=n as i64 - 1 {
            ans += comb.ncr(n as usize - 1, i as usize) / Mint998::new(c00 + 1 + i);
        }
        for i in 0..n {
            ans /= Mint998::new(2);
        }
        for i in 0..nn {
            if vals[i][0] == 0 && vals[i][1] != 0 {
                let idx = vec![i * 2, i * 2 + 1];
                for idx in idx {
                    if A[idx] == max_val {
                        ans_vec[idx] += ans;
                    }
                }
            }
        }
    }
    println!("{}", ans_vec.iter().join(" "));
}

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
