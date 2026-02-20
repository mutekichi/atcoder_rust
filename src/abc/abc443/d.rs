#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut A: [Usize1; n],
        }
        let mut ans = 0;
        let mut seen = vec![false; n];
        let mut pq = BinaryHeap::new();
        for i in 0..n {
            pq.push(Reverse((A[i], i)));
        }
        while let Some(Reverse((a, i))) = pq.pop() {
            if seen[i] {
                continue;
            }
            seen[i] = false;
            let neighbors = {
                let mut neighbors = vec![];
                if i > 0 {
                    neighbors.push(i - 1);
                }
                if i < n - 1 {
                    neighbors.push(i + 1);
                }
                neighbors
            };
            for neighbor in neighbors {
                if A[neighbor] > a + 1 {
                    pq.push(Reverse((a + 1, neighbor)));
                    ans += A[neighbor] - (a + 1);
                    A[neighbor] = a + 1;
                }
            }
        }
        println!("{}", ans);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
