#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
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
        k: usize,
    }

    if k > 2 * n - 1 {
        println!("{}", 0);
        return;
    }

    let data = (1..n).map(Mint998::from).collect::<Vec<_>>();
    let res = prod(&data, 0, n - 1);

    let mut ans = res[k - n];
    for i in 1..=n {
        ans /= i;
    }
    println!("{}", ans);
}

fn prod(
    data: &Vec<ModInt<C998244353>>,
    l: usize,
    r: usize,
) -> Vec<ModInt<C998244353>> {
    if l == r {
        return vec![Mint998::from(1)];
    } else if l == r - 1 {
        return vec![Mint998::from(1), data[l]];
    } else {
        let mid = (l + r) / 2;
        return convolution(&prod(data, l, mid), &prod(data, mid, r));
    }
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

use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

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
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        source: &mut S
    ) -> Self {
        let val = i64::read(source);
        ModInt::new(val)
    }
}

impl<const M: u64> Default for ModInt<M> {
    fn default() -> Self {
        ModInt::new(0)
    }
}

/// Trait for types suitable for Number Theoretic Transform (NTT).
///
/// This trait abstracts the operations required for NTT so that the convolution algorithm
/// can be agnostic to the concrete ModInt implementation.
pub trait NttField:
    Copy
    + From<i64>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + PartialEq
{
    fn new(x: i64) -> Self;
    fn pow(
        &self,
        exp: u64,
    ) -> Self;
    fn inv(&self) -> Self;
    fn val(&self) -> u64;
    fn modulus() -> u64;
    fn primitive_root() -> u64;
}

// Implement NttField for the existing ModInt<M>
impl<const M: u64> NttField for ModInt<M> {
    fn new(x: i64) -> Self {
        ModInt::new(x)
    }
    fn pow(
        &self,
        exp: u64,
    ) -> Self {
        self.pow(exp)
    }
    fn inv(&self) -> Self {
        self.inv()
    }
    fn val(&self) -> u64 {
        self.val()
    }
    fn modulus() -> u64 {
        M
    }
    fn primitive_root() -> u64 {
        match M {
            998_244_353 => 3,
            167_772_161 => 3,
            469_762_049 => 3,
            754_974_721 => 11,
            _ => unimplemented!(
                "Primitive root not defined for modulus {}",
                M
            ),
        }
    }
}

/// Trait to support convolution on various types (ModInt, integers).
pub trait Convolution {
    type Output;
    /// Performs convolution.
    fn convolution(
        a: &[Self],
        b: &[Self],
    ) -> Vec<Self::Output>
    where
        Self: Sized;
}

// Implement Convolution for ModInt<M>
impl<const M: u64> Convolution for ModInt<M>
where
    ModInt<M>: NttField,
{
    type Output = Self;
    fn convolution(
        a: &[Self],
        b: &[Self],
    ) -> Vec<Self> {
        convolution_impl(a, b)
    }
}

// Macro to implement Convolution for integer types using Mint998
macro_rules! impl_convolution_int {
    ($($t:ty),*) => {
        $(
            impl Convolution for $t {
                type Output = $t;
                fn convolution(a: &[Self], b: &[Self]) -> Vec<Self::Output> {
                    let a_mint: Vec<Mint998> = a.iter().map(|&x| Mint998::from(x as i64)).collect();
                    let b_mint: Vec<Mint998> = b.iter().map(|&x| Mint998::from(x as i64)).collect();
                    let res_mint = convolution_impl(&a_mint, &b_mint);
                    res_mint.into_iter().map(|x| x.val() as $t).collect()
                }
            }
        )*
    };
}

// Apply to common integer types
impl_convolution_int!(usize, u64, i64, u32, i32, u128, i128);

/// Performs convolution (polynomial multiplication) of two sequences.
///
/// If `T` is `ModInt<M>`, it returns `Vec<ModInt<M>>` calculated modulo `M`.
/// If `T` is an integer type (e.g., `usize`, `i64`), it uses `ModInt<998244353>` internally
/// and returns `Vec<T>` where values are modulo 998244353.
///
/// # Arguments
/// - `a`: The first sequence.
/// - `b`: The second sequence.
///
/// # Returns
/// A vector containing the convolution of `a` and `b`.
/// The length of the result is `a.len() + b.len() - 1` (or 0 if inputs are empty).
///
/// # Examples
///
/// ## 1. Using with Mint998 (Standard)
/// ```
/// use atcoder_rust::template::math::modint::Mint998;
/// use atcoder_rust::template::math::convolution::convolution;
///
/// let a = vec![Mint998::new(1), Mint998::new(2), Mint998::new(3)];
/// let b = vec![Mint998::new(4), Mint998::new(5)];
/// let c = convolution(&a, &b);
/// // Result: [4, 13, 22, 15] (all Mint998)
/// ```
///
/// ## 2. Using with usize (Convenience)
/// ```
/// use atcoder_rust::template::math::convolution::convolution;
///
/// let a: Vec<usize> = vec![1, 2, 3];
/// let b: Vec<usize> = vec![4, 5];
/// let c = convolution(&a, &b);
///
/// assert_eq!(c, vec![4, 13, 22, 15]);
/// // The calculation is performed modulo 998244353.
/// ```
pub fn convolution<T: Convolution>(
    a: &[T],
    b: &[T],
) -> Vec<T::Output> {
    T::convolution(a, b)
}

/// Internal implementation of convolution using NTT.
fn convolution_impl<T: NttField>(
    a: &[T],
    b: &[T],
) -> Vec<T> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }

    let z = (n + m - 1).next_power_of_two();
    let mut a_copy = vec![T::new(0); z];
    let mut b_copy = vec![T::new(0); z];

    for i in 0..n {
        a_copy[i] = a[i];
    }
    for i in 0..m {
        b_copy[i] = b[i];
    }

    butterfly(&mut a_copy);
    butterfly(&mut b_copy);

    for i in 0..z {
        a_copy[i] = a_copy[i] * b_copy[i];
    }

    butterfly_inv(&mut a_copy);

    a_copy.resize(n + m - 1, T::new(0));
    let inv_z = T::new(z as i64).inv();
    for x in a_copy.iter_mut() {
        *x = *x * inv_z;
    }

    a_copy
}

// In-place NTT (DIF: Decimation In Frequency)
fn butterfly<T: NttField>(a: &mut [T]) {
    let n = a.len();
    let h = n.trailing_zeros() as usize;
    let g = T::new(T::primitive_root() as i64);

    for i in 0..h {
        let p = 1 << (h - 1 - i); // p = n/2, ..., 1
        let w = g.pow((T::modulus() - 1) / (2 * p as u64));

        for s in 0..(1 << i) {
            // Blocks
            let offset = s << (h - i); // s * 2p
            let mut rot = T::new(1);
            for j in 0..p {
                let l = a[offset + j];
                let r = a[offset + j + p];
                a[offset + j] = l + r;
                a[offset + j + p] = (l - r) * rot; // Output mult
                rot = rot * w;
            }
        }
    }
}

// In-place Inverse NTT (DIT: Decimation In Time)
fn butterfly_inv<T: NttField>(a: &mut [T]) {
    let n = a.len();
    let h = n.trailing_zeros() as usize;
    let g = T::new(T::primitive_root() as i64);

    for i in 0..h {
        let p = 1 << i; // p = 1, ..., n/2
        let w = g.pow((T::modulus() - 1) / (2 * p as u64)).inv();

        for s in 0..(1 << (h - 1 - i)) {
            // Blocks
            let offset = s << (i + 1); // s * 2p
            let mut rot = T::new(1);
            for j in 0..p {
                let l = a[offset + j];
                let r = a[offset + j + p] * rot; // Input mult
                a[offset + j] = l + r;
                a[offset + j + p] = l - r;
                rot = rot * w;
            }
        }
    }
}

// END TEMPLATE INJECTIONS
