// --- SNAP START ---

/// Generic matrix structure for competitive programming.
/// 
/// # Examples
/// 
/// ## Standard Matrix Exponentiation (Fibonacci)
/// ```
/// let mut m = Matrix::new(2, 2, 0i64);
/// m.data[0][0] = 1; m.data[0][1] = 1;
/// m.data[1][0] = 1; m.data[1][1] = 0;
/// 
/// let res = m.pow(10, |a, b| a + b, |a, b| a * b, 0, 1);
/// ```
/// 
/// ## Min-Plus Algebra (Shortest Path / DP)
/// ```
/// let inf = 1e18 as i64;
/// let mut m = Matrix::new(2, 2, inf);
/// m.data[0][0] = 0; m.data[0][1] = 5;
/// m.data[1][0] = inf; m.data[1][1] = 0;
/// 
/// let op_add = |a, b| std::cmp::min(a, b);
/// let op_mul = |a, b| a + b;
/// let res = m.pow(10, op_add, op_mul, inf, 0);
/// ```
#[derive(Clone, Debug)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Clone + Copy,
{
    pub fn new(rows: usize, cols: usize, val: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![val; cols]; rows],
        }
    }

    pub fn identity(size: usize, add_identity: T, mul_identity: T) -> Self {
        let mut res = Self::new(size, size, add_identity);
        for i in 0..size {
            res.data[i][i] = mul_identity;
        }
        res
    }

    pub fn multiply<F1, F2>(&self, other: &Self, op_add: F1, op_mul: F2, add_identity: T) -> Self
    where
        F1: Fn(T, T) -> T,
        F2: Fn(T, T) -> T,
    {
        assert_eq!(self.cols, other.rows);
        let mut res = Self::new(self.rows, other.cols, add_identity);
        for i in 0..self.rows {
            for k in 0..self.cols {
                for j in 0..other.cols {
                    res.data[i][j] = op_add(res.data[i][j], op_mul(self.data[i][k], other.data[k][j]));
                }
            }
        }
        res
    }

    pub fn pow<F1, F2>(&self, mut n: u64, op_add: F1, op_mul: F2, add_identity: T, mul_identity: T) -> Self
    where
        F1: Fn(T, T) -> T + Copy,
        F2: Fn(T, T) -> T + Copy,
    {
        assert_eq!(self.rows, self.cols);
        let mut res = Self::identity(self.rows, add_identity, mul_identity);
        let mut base = self.clone();
        while n > 0 {
            if n & 1 == 1 {
                res = res.multiply(&base, op_add, op_mul, add_identity);
            }
            base = base.multiply(&base, op_add, op_mul, add_identity);
            n >>= 1;
        }
        res
    }
}