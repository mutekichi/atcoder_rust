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

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            h: usize, w: usize,
        }
        if h % 2 == 0 || w % 2 == 0 {
            println!("{}", (h / 2) * (w / 2));
            for i in 0..h / 2 {
                for j in 0..w / 2 {
                    println!("{} {} {}", i * 2 + 1, j * 2 + 1, 1);
                }
            }
        } else {
            let mut ans = vec![];
            let l = min(h, w);

            ans.push((1, 1, l - 1));
            if (l - 1) % 4 == 0 {
                let k = (l - 1) / 2;
                let starts = vec![(2, 1), (1, k + 1), (k + 1, k + 2), (k + 2, 2)];
                for (s, t) in starts {
                    for i in 0..k / 2 {
                        for j in 0..k / 2 {
                            ans.push((s + i * 2, t + j * 2, 1));
                        }
                    }
                }
            } else {
                let k = (l - 3) / 4;
                md!(k);
                let starts = vec![(1, 2), (1 + l / 2, 2 + l / 2)];
                for (s, t) in starts {
                    for i in 0..(k + 1) {
                        for j in 0..k {
                            ans.push((s + i * 2, t + j * 2, 1));
                        }
                    }
                }
                let starts = vec![(2, l / 2 + 1), (l / 2 + 2, 1)];
                for (s, t) in starts {
                    for i in 0..k {
                        for j in 0..(k + 1) {
                            ans.push((s + i * 2, t + j * 2, 1));
                        }
                    }
                }
            }

            if h > w {
                for i in 0..(h - w) / 2 {
                    for j in 0..w / 2 {
                        ans.push((h + 1 + i * 2, 1 + j * 2, 1));
                    }
                }
            }
            if w > h {
                for i in 0..h / 2 {
                    for j in 0..(w - h) / 2 {
                        ans.push((1 + i * 2, w + 1 + j * 2, 1));
                    }
                }
            }
            println!("{}", ans.len());
            for &(a, b, c) in &ans {
                println!("{} {} {}", a, b, c);
            }
            let mut grid = vec![vec![0; w]; h];
            for i in 0..ans.len() {
                let (a, b, c) = ans[i];
                let a = a - 1;
                let b = b - 1;
                grid[a][b] = i;
                grid[a][b + c] = i;
                grid[a + c][b] = i;
                grid[a + c][b + c] = i;
            }
            for i in 0..h {
                println!("{}", grid[i].iter().join(" "));
            }
        }
    }
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIR8: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
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
