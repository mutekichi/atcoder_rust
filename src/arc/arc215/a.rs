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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut k: i64,
            l: i64,
            mut A: [i64; n],
        }
        A.sort_unstable();
        A.dedup();
        let mut intervals = A.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        intervals.sort_unstable();
        let mut ans = 0;
        let mut mergin_sum = A[0] + l - A[A.len() - 1];
        let mut mergin_large = max(A[0], l - A[A.len() - 1]);
        let mut sum = 0;

        for i in 0..=min(k as usize, intervals.len()) {
            let rem = k - i as i64;
            if rem > 0 {
                ans = max(ans, sum + mergin_sum * (rem - 1) + mergin_large);
            }
            else {
                ans = max(ans, sum);
            }

            if i < min(k as usize, intervals.len()) {
                let interval = intervals[intervals.len() - 1 - i];
                sum += interval / 2;
                mergin_sum += interval;
                mergin_large += interval / 2;
            }
        }
        println!("{}", ans);
    }
}

fn calc_kougo(
    k: i64,
    left: i64,
    right: i64,
) -> i64 {
    let mut kougo = (left + right) * (k / 2);
    if k % 2 == 1 {
        kougo += max(left, right);
    };
    kougo
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
