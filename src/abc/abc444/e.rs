#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
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

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    input! {
        n: usize,
        d: i64,
        A: [i64; n],
    }
    let mut counter = BTreeSet::new();
    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;
    loop {
        md!(l, r);
        loop {
            md!("2", l, r);
            if r == n || counter
                .range((A[r] - d + 1, 0)..(A[r] + d - 1, INF_USIZE))
                .count()
                > 0
            {
                break;
            } else {
                counter.insert((A[r], r));
                r += 1;
                ans += r - l;
            }
        }
        if r == n {
            break;
        }
        loop {
            md!("3", l, r);
            if l == r || counter
                .range((A[r] - d + 1, 0)..(A[r] + d - 1, INF_USIZE))
                .count()
                == 0
            {
                break;
            } else {
                counter.remove(&(A[l], l));
                l += 1;
            }
        }
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
