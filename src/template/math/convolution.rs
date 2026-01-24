#![allow(dead_code)]

/// --- SNAP START ---
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

/// Trait for types suitable for Number Theoretic Transform (NTT).
///
/// This trait abstracts the operations required for NTT so that the convolution algorithm
/// can be agnostic to the concrete ModInt implementation.
pub trait NttField:
    Copy
    + From<i64>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
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
            _ => unimplemented!("Primitive root not defined for modulus {}", M),
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

// --- SNAP END ---

#[test]
fn test_convolution() {
    use crate::template::math::modint::Mint998;

    let a = vec![Mint998::new(1), Mint998::new(2), Mint998::new(3)];
    let b = vec![Mint998::new(4), Mint998::new(5)];

    let c = convolution(&a, &b);
    // print
    for x in &c {
        println!("{} ", x.val());
    }

    assert_eq!(c.len(), 4);
    assert_eq!(c[0].val(), 4);
    assert_eq!(c[1].val(), 13);
    assert_eq!(c[2].val(), 22);
    assert_eq!(c[3].val(), 15);
}
