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
        S: Chars,
    }
    let n = S.len();
    let mut len = Mint998::new(1);
    let mut ans = Mint998::new(0);
    for i in 0..n - 1 {
        if S[i] == S[i + 1] {
            ans += len * (len + 1) / 2;
            len = Mint998::new(1);
        } else {
            len += 1;
        }
    }
    ans += len * (len + 1) / 2;
    println!("{}", ans);
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

// END TEMPLATE INJECTIONS
