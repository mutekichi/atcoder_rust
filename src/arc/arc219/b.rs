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
    // _experiment(); // return;
    input! { t: usize}
    for _ in 0..t {
        input! {
            n: usize,
            P: [Usize1; n],
        }
        if P[0] != 0 {
            println!("{}", 0);
        } else {
            let mut diff_index = n;
            for i in 0..n {
                if P[i] != i {
                    diff_index = i;
                    break;
                }
            }
            let mut base: ModInt<998244353> = Mint998::new(n as i64) * (n - 1) / 2 + 1;
            md!(base, diff_index);
            if diff_index == n {
            } else if diff_index == n.saturating_sub(2) {
                base -= 2;
            } else if diff_index == n.saturating_sub(3) {
                base -= 4;
            } else {
                base -= 4;
                let diff = n - diff_index;
                for i in 3..diff {
                    base -= i;
                }
            }
            println!("{}", base);
        }
    }
}

#[allow(unused_variables)]
fn _experiment(
    n: usize,
    perm: &Vec<usize>,
) -> usize {
    let mut counter = BTreeMap::new();
    for perm in (1..=n).permutations(n) {
        let mut min_str = "a".to_string();
        for i in 0..n {
            for j in i..n {
                let mut new_perm = vec![];
                for k in 0..i {
                    new_perm.push(perm[k]);
                }
                for k in 0..j - i + 1 {
                    new_perm.push(perm[j - k]);
                }
                for k in 0..n - j - 1 {
                    new_perm.push(perm[j + k + 1]);
                }
                let new_perm = new_perm.iter().join("");
                min_str = min(min_str, new_perm);
            }
        }
        *counter.entry(min_str).or_insert(0) += 1usize;
    }

    return *counter.get(&perm.iter().map(|e| *e + 1).join("")).unwrap();

    let mut count_set = BTreeSet::new();

    for count in counter.iter() {
        let chars = count.0.chars().collect::<Vec<_>>();
        if !(chars[1] == '0' || chars[1] == '1' || *count.1 == n - 1) {
            // println!("{} {}", count.0, count.1)
        }
        count_set.insert(*count.1);
    }
    for count in count_set {
        println!("{}", count);
    }
    return 0;
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
