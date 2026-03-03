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
use std::mem::swap;
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
        h: usize, w: usize, n: usize,
        XY: [(usize, usize); n],
    }
    let mut set_x = BTreeSet::new();
    let mut set_y = BTreeSet::new();
    let mut ans = vec![(0, 0); n];
    for i in 0..n {
        let (x, y) = XY[i];
        set_x.insert((x, y, i));
        set_y.insert((y, x, i));
    }
    let mut current_x = 0;
    let mut current_y = 0;
    for _ in 0..n {
        let x_max = set_x.last().unwrap().0;
        let y_max = set_y.last().unwrap().0;
        md!(x_max, y_max);
        if x_max + current_x == h {
            md!("x");
            let i = set_x.last().unwrap().2;
            ans[i] = (current_x, current_y);
            let (x, y) = XY[i];
            current_y += y;
            set_x.remove(&(x, y, i));
            set_y.remove(&(y, x, i));
        }
        else if y_max + current_y == w {
            md!("y");
            let i = set_y.last().unwrap().2;
            ans[i] = (current_x, current_y);
            let (x, y) = XY[i];
            current_x += x;
            set_x.remove(&(x, y, i));
            set_y.remove(&(y, x, i));
        }
    }
    for i in 0..n {
        println!("{} {}", ans[i].0 + 1, ans[i].1 + 1);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
