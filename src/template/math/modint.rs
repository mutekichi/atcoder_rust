// --- SNAP START ---

use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Mint998 = ModInt<998_244_353>;
pub type Mint107 = ModInt<1_000_000_007>;

#[cfg(debug_assertions)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rational(pub i64, pub i64);

#[cfg(debug_assertions)]
impl fmt::Display for Rational {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "({}/{})", self.0, self.1)
    }
}

// Tracker active only when debug assertions are enabled.
#[cfg(debug_assertions)]
#[derive(Copy, Clone)]
pub struct ModTracker {
    float_val: f64,
    num: i64,
    den: i64,
}

// Zero-sized tracker for release.
#[cfg(not(debug_assertions))]
#[derive(Copy, Clone)]
pub struct ModTracker;

#[cfg(debug_assertions)]
impl ModTracker {
    fn new(val: i64) -> Self {
        Self {
            float_val: val as f64,
            num: val,
            den: 1,
        }
    }

    fn reduce(
        num: i64,
        den: i64,
    ) -> (i64, i64) {
        if den == 0 {
            return (num, den);
        }
        let g = Self::gcd(num.abs(), den.abs());
        let (n, d) = (num / g, den / g);
        if d < 0 { (-n, -d) } else { (n, d) }
    }

    fn gcd(
        mut a: i64,
        mut b: i64,
    ) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn add(
        self,
        other: Self,
    ) -> Self {
        let (num, den) = Self::reduce(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        );
        Self {
            float_val: self.float_val + other.float_val,
            num,
            den,
        }
    }

    fn sub(
        self,
        other: Self,
    ) -> Self {
        let (num, den) = Self::reduce(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        );
        Self {
            float_val: self.float_val - other.float_val,
            num,
            den,
        }
    }

    fn mul(
        self,
        other: Self,
    ) -> Self {
        let (num, den) = Self::reduce(self.num * other.num, self.den * other.den);
        Self {
            float_val: self.float_val * other.float_val,
            num,
            den,
        }
    }

    fn div(
        self,
        other: Self,
    ) -> Self {
        let (num, den) = Self::reduce(self.num * other.den, self.den * other.num);
        Self {
            float_val: self.float_val / other.float_val,
            num,
            den,
        }
    }

    fn inv(self) -> Self {
        let (num, den) = Self::reduce(self.den, self.num);
        Self {
            float_val: 1.0 / self.float_val,
            num,
            den,
        }
    }

    fn neg(self) -> Self {
        Self {
            float_val: -self.float_val,
            num: -self.num,
            den: self.den,
        }
    }

    fn pow(
        self,
        mut exp: u64,
    ) -> Self {
        let mut base = self;
        let mut res = Self::new(1);
        while exp > 0 {
            if exp % 2 == 1 {
                res = res.mul(base);
            }
            base = base.mul(base);
            exp /= 2;
        }
        res
    }
}

#[cfg(not(debug_assertions))]
impl ModTracker {
    #[inline(always)]
    fn new(_val: i64) -> Self {
        Self
    }
    #[inline(always)]
    fn add(
        self,
        _other: Self,
    ) -> Self {
        Self
    }
    #[inline(always)]
    fn sub(
        self,
        _other: Self,
    ) -> Self {
        Self
    }
    #[inline(always)]
    fn mul(
        self,
        _other: Self,
    ) -> Self {
        Self
    }
    #[inline(always)]
    fn div(
        self,
        _other: Self,
    ) -> Self {
        Self
    }
    #[inline(always)]
    fn inv(self) -> Self {
        Self
    }
    #[inline(always)]
    fn neg(self) -> Self {
        Self
    }
    #[inline(always)]
    fn pow(
        self,
        _exp: u64,
    ) -> Self {
        Self
    }
}

#[derive(Copy, Clone)]
pub struct ModInt<const M: u64> {
    val: u64,
    tracker: ModTracker,
}

impl<const M: u64> PartialEq for ModInt<M> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.val == other.val
    }
}

impl<const M: u64> Eq for ModInt<M> {}

impl<const M: u64> ModInt<M> {
    pub fn new(x: i64) -> Self {
        let mut rem = x % M as i64;
        if rem < 0 {
            rem += M as i64;
        }
        ModInt {
            val: rem as u64,
            tracker: ModTracker::new(x),
        }
    }

    pub fn val(&self) -> u64 {
        self.val
    }

    #[cfg(debug_assertions)]
    pub fn float_val(&self) -> f64 {
        self.tracker.float_val
    }

    #[cfg(debug_assertions)]
    pub fn rational_val(&self) -> Rational {
        Rational(self.tracker.num, self.tracker.den)
    }

    pub fn pow(
        &self,
        exp: u64,
    ) -> Self {
        let mut base = self.val;
        let mut res = 1;
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 {
                res = (res * base) % M;
            }
            base = (base * base) % M;
            e /= 2;
        }
        ModInt {
            val: res,
            tracker: self.tracker.pow(exp),
        }
    }

    pub fn inv(&self) -> Self {
        let mut base = self.val;
        let mut res = 1;
        let mut e = M - 2;
        while e > 0 {
            if e % 2 == 1 {
                res = (res * base) % M;
            }
            base = (base * base) % M;
            e /= 2;
        }
        ModInt {
            val: res,
            tracker: self.tracker.inv(),
        }
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
        #[cfg(debug_assertions)]
        {
            write!(
                f,
                "{} (approx: {}, {}/{})",
                self.val, self.tracker.float_val, self.tracker.num, self.tracker.den
            )
        }
        #[cfg(not(debug_assertions))]
        {
            write!(f, "{}", self.val)
        }
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
        let val = if self.val == 0 { 0 } else { M - self.val };
        ModInt {
            val,
            tracker: self.tracker.neg(),
        }
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
        ModInt {
            val: res,
            tracker: self.tracker.add(other.tracker),
        }
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
        ModInt {
            val: res,
            tracker: self.tracker.sub(other.tracker),
        }
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
            tracker: self.tracker.mul(other.tracker),
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
        let mut res = self * other.inv();
        res.tracker = self.tracker.div(other.tracker);
        res
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
