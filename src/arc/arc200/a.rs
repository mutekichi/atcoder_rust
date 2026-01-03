#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

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
pub fn ext_gcd<T>(
    a: T,
    b: T,
) -> (T, T, T)
where
    T: ExtGcdImpl,
{
    T::ext_gcd(a, b)
}

pub trait ExtGcdImpl: Sized {
    fn ext_gcd(
        a: Self,
        b: Self,
    ) -> (Self, Self, Self);
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

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        t: usize,
    }
    
    for _ in 0..t {
        input! {
            n: usize,
            A: [i128; n],
            B: [i128; n],
        }
        
        let g = gcd(A[0], B[0]);
        let ga = A[0] / g;
        let gb = B[0] / g;
        
        let mut idx = 0;
        for i in 1..n {
            let gg = gcd(A[i], B[i]);
            let gga = A[i] / gg;
            let ggb = B[i] / gg;
            if ga == gga && gb == ggb {
                continue;
            }
            idx = i;
            md!(idx);
            break;
        }

        if idx == 0 {
            wl!("No");
            continue;
        }
        wl!("Yes");
        let mut ans = vec![0; n];

        let a = A[0];
        let b = B[0];
        let c = A[idx];
        let d = B[idx];

        let (val_first, val_second) = if a * d > c * b {
            (c + d, -(a + b))
        } else {
            (-(c + d), a + b)
        };
        ans[0] = val_first;
        ans[idx] = val_second;
        wl!(ans.into_iter().map(|x| x.to_string()).join(" "));
        
    }
}

// --- Macros ---

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

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint without newline)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mep {
    ($x:expr) => { eprint!("{}", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mep {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint with space)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mepw { // stands for my_eprint_whitespace
    ($x:expr) => { eprint!("{} ", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mepw {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}
