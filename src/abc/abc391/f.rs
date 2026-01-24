#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use rand::Rng;
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

// FOR TEMPLATE INJECTIONS

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
        n: usize,
        k: usize,
        mut A: [i64; n],
        mut B: [i64; n],
        mut C: [i64; n],
    }
    let mut set = BTreeSet::new();
    A.sort_unstable();
    A.reverse();
    B.sort_unstable();
    B.reverse();
    C.sort_unstable();
    C.reverse();
    set.insert((Reverse(f(A[0], B[0], C[0])), 0, 0, 0));
    for i in 0..k {
        let (Reverse(v), a, b, c) = set.pop_first().unwrap();
        md!(v, a, b, c);
        if i == k - 1 {
            wl!(v);
            return;
        }
        if a != n - 1 {
            set.insert((Reverse(f(A[a + 1], B[b], C[c])), a + 1, b, c));
        }
        if b != n - 1 {
            set.insert((Reverse(f(A[a], B[b + 1], C[c])), a, b + 1, c));
        }
        if c != n - 1 {
            set.insert((Reverse(f(A[a], B[b], C[c + 1])), a, b, c + 1));
        }
    }
}

fn f(
    a: i64,
    b: i64,
    c: i64,
) -> i64 {
    a * b + b * c + c * a
}
