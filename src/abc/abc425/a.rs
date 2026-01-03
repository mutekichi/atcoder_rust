#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// Constants
const ILONGINF: i64 = 1 << 60;
const ISHORTINF: i32 = 1 << 30;
const ULONGINF: u64 = 1 << 60;
const USHORTINF: u32 = 1 << 30;
const ISIZEINF: isize = 1 << 30;
const USIZEINF: usize = 1 << 30;
const MOD: i64 = 998244353;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

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

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

// Logic goes here
#[allow(unused_macros)]
#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        // INPUT
        n: isize,
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += mod_pow(-1, i, 100000) * mod_pow(i, 3,  100000)
    }
    
    wl!(ans);
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
