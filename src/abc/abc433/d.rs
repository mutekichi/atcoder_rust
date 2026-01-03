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
        n: usize,
        m: i128,
        a_list: [i128; n],
    }

    let mut data = BTreeMap::new();
    let mut nagasa = vec![];

    for i in 0..n {
        let a = a_list[i];

        nagasa.push(a.to_string().chars().count() as i128);

        for keta in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
            let to_insert = (a * mod_pow(10, keta, m) % m, keta);
            data.entry(to_insert).and_modify(|e| {*e += 1}).or_insert(1);
        }
    }

    let mut ans: i128 = 0; // エグイ
    
    for i in 0..n {
        let a = a_list[i];
        let keta = nagasa[i];
        let mut to_search = (m - a) % m;
        if to_search < 0 {to_search += m};
        let tuplekey = (to_search, keta);


        if data.contains_key(&tuplekey) {
            let value = data[&tuplekey];
            ans += value;
        }
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
