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
        m: usize,
        a: usize,
        b: usize,
    }
    let mut status = vec![vec![3; m]; m];
    for i in 0..m {
        for j in 0..m {
            let mut s = i;
            let mut t = j;
            let mut history = vec![];
            let mut ok = true;
            loop {
                history.push((s, t));
                if s * t == 0 {
                    ok = false;
                    break;
                }
                if status[s][t] != 3 {
                    if status[s][t] == 0 {
                        ok = false;
                    }
                    break;
                }
                status[s][t] = 2;
                (s, t) = (t, (s * b + t * a) % m);
            }
            for (s, t) in history {
                if ok {
                    status[s][t] = 1;
                }
                else {
                    status[s][t] = 0;
                }
            }
        }
    }
    let mut ans = 0usize;
    for i in 0..m {
        for j in 0..m {
            if status[i][j] == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
