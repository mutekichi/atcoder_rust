#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

// FOR TEMPLATE INJECTIONS

/// Returns valid neighbor coordinates within the grid (h x w).
/// Usage:
/// ```
/// for (nh, nw) in get_next_positions(h, w, hh, ww, &DIR) {
///     // process (nh, nw)
/// }
/// ```
fn get_next_positions(
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<(usize, usize)> {
    let mut next_positions = Vec::with_capacity(directions.len());

    for &(di, dj) in directions {
        let next_i = i.wrapping_add_signed(di);
        let next_j = j.wrapping_add_signed(dj);
        if next_i < h && next_j < w {
            next_positions.push((next_i, next_j));
        }
    }
    next_positions
}

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        h: usize,
        w: usize,
        S: [Chars; h],
    }
    for i in 0..h {
        for j in 0..w {
            if S[i][j] == '#' {
                let mut count = 0;
                for (ni, nj) in get_next_positions(h, w, i, j, &DIR) {
                    if S[ni][nj] == '#' {
                        count += 1;
                    }
                }
                if count != 2 && count != 4 {
                    wl!("No");
                    return;
                }
            }
        }
    }
    wl!("Yes");
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

trait JoinExtended {
    fn join_with(
        self,
        sep: &str,
    ) -> String;
}

impl<I> JoinExtended for I
where
    I: Iterator,
    I::Item: Joinable,
{
    fn join_with(
        self,
        sep: &str,
    ) -> String {
        let mut peekable = self.peekable();
        let is_2d = if let Some(first) = peekable.peek() {
            first.is_container()
        } else {
            false
        };

        let res = peekable.map(|item| item.join_item(sep)).collect::<Vec<_>>();

        // Use newline for 2D rows, provided sep for 1D elements
        res.join(if is_2d { "\n" } else { sep })
    }
}

trait Joinable {
    fn join_item(
        &self,
        sep: &str,
    ) -> String;
    fn is_container(&self) -> bool;
}

macro_rules! impl_joinable_scalar {
    ($($t:ty),*) => {
        $(
            impl Joinable for &$t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
            impl Joinable for $t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
        )*
    };
}

impl_joinable_scalar!(
    i32, i64, i128, u32, u64, u128, usize, isize, f32, f64, char, String, &str
);

impl<T: std::fmt::Display> Joinable for &Vec<T> {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}

impl<T: std::fmt::Display> Joinable for &[T] {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}
