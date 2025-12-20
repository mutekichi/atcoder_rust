#![allow(dead_code)]

/// --- SNAP START ---
use std::fmt;
use std::ops::{Add, AddAssign, Mul};

/// Matrix operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    pub mat: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix<T>
where
    T: Clone
        + Copy
        + Default
        + From<i32>
        + Add<Output = T>
        + Mul<Output = T>
        + AddAssign
        + PartialEq,
{
    /// Constructs a new `rows x cols` matrix filled with zeros (T::default()).
    pub fn new(
        rows: usize,
        cols: usize,
    ) -> Self {
        Matrix {
            mat: vec![vec![T::default(); cols]; rows],
            rows,
            cols,
        }
    }

    /// Constructs an identity matrix of size `n`.
    pub fn identity(n: usize) -> Self {
        let mut res = Matrix::new(n, n);
        for i in 0..n {
            res.mat[i][i] = T::from(1);
        }
        res
    }

    /// Performs matrix multiplication.
    /// Renamed from `mul` to `matmul` to avoid conflict with `std::ops::Mul`.
    ///
    /// # Complexity
    /// O(rows * cols * other.cols)
    pub fn matmul(
        &self,
        other: &Matrix<T>,
    ) -> Matrix<T> {
        assert_eq!(
            self.cols, other.rows,
            "Dimension mismatch for matrix multiplication"
        );
        let mut res = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                if self.mat[i][k] == T::default() {
                    continue;
                }
                for j in 0..other.cols {
                    let val = self.mat[i][k] * other.mat[k][j];
                    res.mat[i][j] += val;
                }
            }
        }
        res
    }

    /// Performs matrix exponentiation (A^exp).
    ///
    /// # Complexity
    /// O(n^3 log exp)
    pub fn pow(
        &self,
        mut exp: u64,
    ) -> Matrix<T> {
        assert_eq!(
            self.rows, self.cols,
            "Matrix must be square for exponentiation"
        );
        let mut res = Matrix::identity(self.rows);
        let mut base = self.clone();
        while exp > 0 {
            if exp % 2 == 1 {
                // Modified to use matmul
                res = res.matmul(&base);
            }
            // Modified to use matmul
            base = base.matmul(&base);
            exp /= 2;
        }
        res
    }

    /// Returns the element at (row, col).
    pub fn get(
        &self,
        row: usize,
        col: usize,
    ) -> T {
        self.mat[row][col]
    }

    /// Sets the element at (row, col) to `val`.
    pub fn set(
        &mut self,
        row: usize,
        col: usize,
        val: T,
    ) {
        self.mat[row][col] = val;
    }
}

// Enable `A * B` syntax
impl<T> Mul for Matrix<T>
where
    T: Clone
        + Copy
        + Default
        + From<i32>
        + Add<Output = T>
        + Mul<Output = T>
        + AddAssign
        + PartialEq,
{
    type Output = Self;
    fn mul(
        self,
        rhs: Self,
    ) -> Self {
        // Use matmul to avoid infinite recursion and borrow errors
        self.matmul(&rhs)
    }
}

// Display implementation for debugging
impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for row in &self.mat {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// --- SNAP END ---
#[allow(unused_imports)]
use super::modint::ModInt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        // Fibonacci sequence: F_0 = 0, F_1 = 1, F_n = F_{n-1} + F_{n-2}

        let mut a = Matrix::<i64>::new(2, 2);
        a.set(0, 0, 1);
        a.set(0, 1, 1);
        a.set(1, 0, 1);
        a.set(1, 1, 0);

        // A^10
        let a_pow_10 = a.pow(10);

        // Initial vector v = [1, 0]^T (represents [F_1, F_0])
        let mut v = Matrix::<i64>::new(2, 1);
        v.set(0, 0, 1);
        v.set(1, 0, 0);

        // Result = A^10 * v
        // Here we can use `*` operator which calls `mul` (trait) -> `matmul` (inherent)
        let res = a_pow_10 * v;

        // F_10 should be 55
        assert_eq!(res.get(1, 0), 55);
        // F_11 should be 89
        assert_eq!(res.get(0, 0), 89);
    }

    #[test]
    fn test_fibonacci_modint() {
        // Fibonacci sequence using ModInt<998244353>

        let mut a = Matrix::<ModInt<998244353>>::new(2, 2);
        a.set(0, 0, ModInt::from(1));
        a.set(0, 1, ModInt::from(1));
        a.set(1, 0, ModInt::from(1));
        a.set(1, 1, ModInt::from(0));

        // A^100
        let a_pow_100 = a.pow(100);

        // Initial vector v = [1, 0]^T (represents [F_1, F_0])
        let mut v = Matrix::<ModInt<998244353>>::new(2, 1);
        v.set(0, 0, ModInt::from(1));
        v.set(1, 0, ModInt::from(0));

        // Result = A^100 * v
        let res = a_pow_100.matmul(&v);

        // F_100 = 354224848179261915075 = 494958974 (mod 998244353)
        assert_eq!(res.get(1, 0).val(), 494958974);
    }
}
