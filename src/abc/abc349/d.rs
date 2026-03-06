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
use std::mem::swap;
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
        mut l: u64,
        mut r: u64,
    }
    let mut ans = vec![];
    for i in 0usize..64 {
        if ((l >> i) & 1 == 1) && (l + (1 << i)) <= r {
            ans.push((l, l + (1 << i)));
            md!(l, l + (1 << i));
            l += 1 << i;
        }
    }
    for i in (0usize..64).rev() {
        if (l >> i) & 1 != (r >> i) & 1 {
            ans.push((l, l + (1 << i)));
            md!(l, l + (1 << i));
            l += 1 << i;
        }
    }
    assert_eq!(l, r);
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i, j);
    }
}

fn msb(n: u64) -> usize {
    for i in (0..64).rev() {
        if (n >> i) & 1 == 1 {
            return i;
        }
    }
    return INF_USIZE;
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
