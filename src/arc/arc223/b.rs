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

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
    }
    let comb = Combination::<C998244353>::new(200010);
    let mut kaijos = vec![Mint998::new(1)];
    let mut inv_kaijos = vec![Mint998::new(1)];
    for i in 0..200010 {
        kaijos.push(kaijos[i] * (i + 1));
        inv_kaijos.push(inv_kaijos[i] / (i + 1));
    }
    for _ in 0..t {
        testcase(&comb, &kaijos, &inv_kaijos);
    }
}

#[allow(unused_variables)]
fn testcase(
    comb: &Combination<C998244353>,
    kaijos: &Vec<Mint998>,
    inv_kaijos: &Vec<Mint998>,
) {
    input! {
        n: usize, k: i64,
        A: [i64; n],
    }
    let mut ans = Mint998::new(1);

    let mut prev_val = INF_I64;
    let mut is_half = false;
    let mut is_small = false;
    let mut is_zero = false;
    let mut small_count = 0;
    let mut large_count = 0;
    let mut counter = BTreeMap::new();

    for i in 0..n {
        let a = A[i] % k;
        md!(a);
        if a == 0 {
            if is_zero {
                // do nothing
            } else {
                ans *= calc_and_initialize(
                    &mut prev_val,
                    &mut is_half,
                    &mut is_small,
                    &mut is_zero,
                    &mut small_count,
                    &mut large_count,
                    &comb,
                    &mut counter,
                    &kaijos,
                    &inv_kaijos,
                );
                is_zero = true;
            }
        } else if k % 2 == 0 && a == k / 2 {
            if is_half {
                // do nothing
            } else {
                ans *= calc_and_initialize(
                    &mut prev_val,
                    &mut is_half,
                    &mut is_small,
                    &mut is_zero,
                    &mut small_count,
                    &mut large_count,
                    &comb,
                    &mut counter,
                    &kaijos,
                    &inv_kaijos,
                );
                is_half = true;
            }
        } else if a < (k + 1) / 2 {
            if a == prev_val {
                small_count += 1;
            } else {
                ans *= calc_and_initialize(
                    &mut prev_val,
                    &mut is_half,
                    &mut is_small,
                    &mut is_zero,
                    &mut small_count,
                    &mut large_count,
                    &comb,
                    &mut counter,
                    &kaijos,
                    &inv_kaijos,
                );
                small_count += 1;
                is_small = true;
                prev_val = a;
            }
        } else {
            if a == k - prev_val {
                large_count += 1;
            } else {
                ans *= calc_and_initialize(
                    &mut prev_val,
                    &mut is_half,
                    &mut is_small,
                    &mut is_zero,
                    &mut small_count,
                    &mut large_count,
                    &comb,
                    &mut counter,
                    &kaijos,
                    &inv_kaijos,
                );
                large_count += 1;
                is_small = false;
                prev_val = k - a;
            }
        }
        *counter.entry(A[i]).or_insert(0) += 1usize;
    }
    ans *= calc_and_initialize(
        &mut prev_val,
        &mut is_half,
        &mut is_small,
        &mut is_zero,
        &mut small_count,
        &mut large_count,
        &comb,
        &mut counter,
        &kaijos,
        &inv_kaijos,
    );
    println!("{}", ans);
}

fn calc_and_initialize(
    prev_val: &mut i64,
    is_half: &mut bool,
    is_small: &mut bool,
    is_zero: &mut bool,
    small_count: &mut i64,
    large_count: &mut i64,
    comb: &Combination<C998244353>,
    counter: &mut BTreeMap<i64, usize>,
    kaijos: &Vec<Mint998>,
    inv_kaijos: &Vec<Mint998>,
) -> Mint998 {
    let mut ans = Mint998::new(1);
    if *is_half || *is_zero {
        let mut total = 0;
        for (_, &count) in counter.iter() {
            md!(count);
            total += count;
            ans *= inv_kaijos[count];
        }
        ans *= kaijos[total];
    } else {
        md!(small_count, large_count);
        ans = comb.ncr(
            *small_count as usize + *large_count as usize,
            *small_count as usize,
        );
    }
    *prev_val = INF_I64;
    *is_half = false;
    *is_small = false;
    *is_zero = false;
    *small_count = 0;
    *large_count = 0;
    counter.clear();

    return ans;
}

// FOR TEMPLATE INJECTIONS
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Mint998 = ModInt<998_244_353>;
pub type Mint107 = ModInt<1_000_000_007>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<const M: u64> {
    val: u64,
}

impl<const M: u64> ModInt<M> {
    pub fn new(x: i64) -> Self {
        let mut x = x % M as i64;
        if x < 0 {
            x += M as i64;
        }
        ModInt { val: x as u64 }
    }

    pub fn val(&self) -> u64 {
        self.val
    }

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

    pub fn inv(&self) -> Self {
        self.pow(M - 2)
    }
}

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

impl<const M: u64> From<u32> for ModInt<M> {
    fn from(item: u32) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        ModInt::new(-(self.val as i64))
    }
}

impl<const M: u64> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(0), |a, b| a + b)
    }
}

impl<const M: u64> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(1), |a, b| a * b)
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

impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self + other;
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

macro_rules! impl_modint_ops {
    ($($t:ty),*) => {
        $(
            impl<const M: u64> Add<$t> for ModInt<M> {
                type Output = Self;
                fn add(self, other: $t) -> Self { self + ModInt::from(other) }
            }
            impl<const M: u64> Sub<$t> for ModInt<M> {
                type Output = Self;
                fn sub(self, other: $t) -> Self { self - ModInt::from(other) }
            }
            impl<const M: u64> Mul<$t> for ModInt<M> {
                type Output = Self;
                fn mul(self, other: $t) -> Self { self * ModInt::from(other) }
            }
            impl<const M: u64> Div<$t> for ModInt<M> {
                type Output = Self;
                fn div(self, other: $t) -> Self { self / ModInt::from(other) }
            }
            impl<const M: u64> AddAssign<$t> for ModInt<M> {
                fn add_assign(&mut self, other: $t) { *self = *self + other; }
            }
            impl<const M: u64> SubAssign<$t> for ModInt<M> {
                fn sub_assign(&mut self, other: $t) { *self = *self - other; }
            }
            impl<const M: u64> MulAssign<$t> for ModInt<M> {
                fn mul_assign(&mut self, other: $t) { *self = *self * other; }
            }
            impl<const M: u64> DivAssign<$t> for ModInt<M> {
                fn div_assign(&mut self, other: $t) { *self = *self / other; }
            }
            impl<const M: u64> Add<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn add(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) + other }
            }
            impl<const M: u64> Sub<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn sub(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) - other }
            }
            impl<const M: u64> Mul<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn mul(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) * other }
            }
            impl<const M: u64> Div<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn div(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) / other }
            }
        )*
    };
}

impl_modint_ops!(i32, i64, u32, u64, usize);

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

pub struct Combination<const M: u64> {
    fact: Vec<ModInt<M>>,
    inv_fact: Vec<ModInt<M>>,
}

impl<const M: u64> Combination<M> {
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

    pub fn fact(
        &self,
        n: usize,
    ) -> ModInt<M> {
        self.fact[n]
    }
}
