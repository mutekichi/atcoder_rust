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
        s: Chars,
        t: Chars,
    }
    let mut s_runs = vec![];
    let mut t_runs = vec![];
    let mut s_others = vec![];
    let mut t_others = vec![];
    let mut run_length = 0;
    for c in s {
        if c == 'A' {
            run_length += 1usize;
        } else {
            s_runs.push(run_length);
            run_length = 0;
            s_others.push(c);
        }
    }
    s_runs.push(run_length);
    run_length = 0;
    for c in t {
        if c == 'A' {
            run_length += 1;
        } else {
            t_runs.push(run_length);
            run_length = 0;
            t_others.push(c);
        }
    }
    t_runs.push(run_length);

    if s_others.len() != t_others.len() || !s_others.iter().zip(t_others).all(|(s, t)| *s == t) {
        println!("-1");
        return;
    }

    assert!(s_runs.len() == t_runs.len());
    let mut ans = 0usize;
    for i in 0..s_runs.len() {
        ans += s_runs[i].abs_diff(t_runs[i]);
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
