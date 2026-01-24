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
    }
    let mut K = vec![];
    let mut data = vec![];
    let mut counts = vec![vec![0usize; 100010usize]; n];
    for i in 0..n {
        input! {
            k: usize,
            A: [usize; k],
        }
        K.push(k);
        data.push(BTreeMap::new());
        for a in A {
            counts[i][a] += 1;
            *(data[i]).entry(a).or_insert(0) += 1;
        }
    }
    let mut ans = 0f64;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let mut numerator = 0usize;
            for (&v, &count) in data[i].iter() {
                md!(i, v, count);
                numerator += count * counts[j][v];
            }
            md!(i, j, numerator, K[i] * K[j]);
            ans = ans.max(numerator as f64 / (K[i] * K[j]) as f64);
        }
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
