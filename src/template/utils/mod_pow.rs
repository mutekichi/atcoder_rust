#![allow(dead_code)]

// --- SNAP START ---

/// Modular Exponentiation
///
/// Calculates `base ^ exp % modulo` efficiently.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::math::mod_pow::mod_pow;
///
/// assert_eq!(mod_pow(2, 10, 1000), 24); // 1024 % 1000 = 24
/// assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
/// ```
pub fn mod_pow<T>(base: T, exp: T, modulo: T) -> T
where
    T: ModPowImpl,
{
    T::mod_pow(base, exp, modulo)
}

pub trait ModPowImpl: Sized + Copy {
    fn mod_pow(base: Self, exp: Self, modulo: Self) -> Self;
}

macro_rules! impl_mod_pow {
    ($($t:ty),*) => {
        $(
            impl ModPowImpl for $t {
                fn mod_pow(mut base: Self, mut exp: Self, modulo: Self) -> Self {
                    let mut res = 1;
                    base %= modulo;
                    while exp > 0 {
                        if exp % 2 == 1 {
                            res = (res * base) % modulo;
                        }
                        base = (base * base) % modulo;
                        exp /= 2;
                    }
                    res
                }
            }
        )*
    };
}

// Implement for standard unsigned/signed integers
impl_mod_pow!(u32, u64, u128, usize, i32, i64, i128, isize);

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pow() {
        let m = 1_000_000_007i64;
        assert_eq!(mod_pow(2i64, 0i64, m), 1);
        assert_eq!(mod_pow(2i64, 3i64, m), 8);
        assert_eq!(mod_pow(2i64, 10i64, m), 1024);

        // 3^4 = 81, 81 % 10 = 1
        assert_eq!(mod_pow(3u64, 4u64, 10u64), 1);
    }
}
