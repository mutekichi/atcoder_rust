#![allow(dead_code)]

use super::modint::{Mint998, ModInt};
use std::ops::{Add, Div, Mul, Sub};

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
