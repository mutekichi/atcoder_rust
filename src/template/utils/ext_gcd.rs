#![allow(dead_code)]

// --- SNAP START ---

/// Extended Euclidean Algorithm
///
/// Returns `(g, x, y)` such that `a * x + b * y = g = gcd(a, b)`.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::math::ext_gcd::ext_gcd;
///
/// let (g, x, y) = ext_gcd(111, 30);
/// // 111 * 3 + 30 * (-11) = 3 = gcd(111, 30)
/// assert_eq!(g, 3);
/// assert_eq!(x, 3);
/// assert_eq!(y, -11);
/// ```
pub fn ext_gcd<T>(a: T, b: T) -> (T, T, T)
where
    T: ExtGcdImpl,
{
    T::ext_gcd(a, b)
}

pub trait ExtGcdImpl: Sized {
    fn ext_gcd(a: Self, b: Self) -> (Self, Self, Self);
}

macro_rules! impl_ext_gcd {
    ($($t:ty),*) => {
        $(
            impl ExtGcdImpl for $t {
                fn ext_gcd(a: Self, b: Self) -> (Self, Self, Self) {
                    if b == 0 {
                        (a, 1, 0)
                    } else {
                        let (g, y, x) = Self::ext_gcd(b, a % b);
                        (g, x, y - (a / b) * x)
                    }
                }
            }
        )*
    };
}

impl_ext_gcd!(i32, i64, i128, isize);

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_gcd_i64() {
        let a = 111i64;
        let b = 30i64;
        let (g, x, y) = ext_gcd(a, b);
        assert_eq!(g, 3);
        assert_eq!(a * x + b * y, g);
    }

    #[test]
    fn test_ext_gcd_i128() {
        let a = 240i128;
        let b = 46i128;
        let (g, x, y) = ext_gcd(a, b);
        assert_eq!(g, 2);
        assert_eq!(a * x + b * y, g);
    }
}