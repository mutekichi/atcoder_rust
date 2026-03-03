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
            A: [Usize1; 2 * n],
        }
        let mut status = {
            let mut status = vec![false; 2 * n];
            let mut used = vec![false; n];
            for i in 0..(2 * n) {
                if !used[A[i]] {
                    used[A[i]] = true;
                } else {
                    status[i] = true;
                }
            }
            status
        };
        let pairs = {
            let mut pairs = vec![INF_USIZE; 2 * n];
            let mut positions = vec![INF_USIZE; n];
            for i in 0..2 * n {
                if positions[A[i]] == INF_USIZE {
                    positions[A[i]] = i;
                } else {
                    pairs[i] = positions[A[i]];
                    pairs[positions[A[i]]] = i;
                }
            }
            pairs
        };
        md!(status.iter().map(|e| if *e { 0 } else { 1 }).join(" "));
        md!(pairs.iter().join(" "));

        let mut total_flips = status.windows(2).filter(|w| w[1] != w[0]).count();
        while total_flips > n {
            md!(total_flips);
            for i in 0..2*n {
                let j = pairs[i];
                md!(j);
                let current_flips = if j == i + 1 {
                    let mut flips = 1;
                    if i > 0 && status[i - 1] != status[i] {
                        flips += 1;
                    }
                    if j < 2 * n - 1 && status[j] != status[j + 1] {
                        flips += 1;
                    }
                    flips
                } else {
                    let mut flips = 0;
                    for k in vec![i, j] {
                        if k > 0 && status[k - 1] != status[k] {
                            flips += 1;
                        }
                        if k < 2 * n - 1 && status[k] != status[k + 1] {
                            flips += 1;
                        }
                    }
                    flips
                };
                status[i] = !status[i];
                status[j] = !status[j];
                let next_flips = if j == i + 1 {
                    let mut flips = 1;
                    if i > 0 && status[i - 1] != status[i] {
                        flips += 1;
                    }
                    if j < 2 * n - 1 && status[j] != status[j + 1] {
                        flips += 1;
                    }
                    flips
                } else {
                    let mut flips = 0;
                    for k in vec![i, j] {
                        if k > 0 && status[k - 1] != status[k] {
                            flips += 1;
                        }
                        if k < 2 * n - 1 && status[k] != status[k + 1] {
                            flips += 1;
                        }
                    }
                    flips
                };
                md!(i);
                md!(current_flips, next_flips);
                if current_flips > next_flips {
                    total_flips -= current_flips - next_flips;
                }
                else {
                    status[i] = !status[i];
                    status[j] = !status[j];
                }
            }
        }
        let mut ans  =vec![];
        for i in 1..2*n {
            if status[i - 1] != status[i] {
                ans.push(i);
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
