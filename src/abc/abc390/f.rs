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
        A: [Usize1; n],
    }
    let mut last_positions = vec![-INF_I64; n];
    let mut ans = 0i64;
    let mut to_add = 0i64;
    for i in 0..n {
        to_add += (i + 1) as i64;
        let a = A[i];
        let pos_minus = if a != 0 {
            last_positions[a - 1]
        } else {
            -INF_I64
        };
        let pos_eq = last_positions[a];
        let pos_plus = if a != n - 1 {
            last_positions[a + 1]
        } else {
            -INF_I64
        };
        let pos_max = max(pos_minus, max(pos_eq, pos_plus));
        if pos_max != -INF_I64 {
            if pos_max == pos_minus {
                to_add -= pos_minus + 1;
                if pos_plus > pos_eq {
                    to_add -= pos_plus + 1 - max(pos_eq + 1, 0);
                }
            } else if pos_max == pos_eq {
                to_add -= pos_eq + 1;
            } else {
                to_add -= pos_plus + 1;
                if pos_minus > pos_eq {
                    to_add -= pos_minus + 1 - max(pos_eq + 1, 0);
                }
            }
        }
        last_positions[a] = i as i64;
        md!(ans);
        ans += to_add;
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
