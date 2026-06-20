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
use std::vec;

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const C998244353: usize = 998244353;
const C1000000007: usize = 1000000007;

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
fn naive(
    h: usize,
    w: usize,
    S: &Vec<Vec<usize>>,
    K: usize,
) -> usize {
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            for i2 in i..h {
                for j2 in j..w {
                    let mut sum = 0;
                    for i3 in i..=i2 {
                        for j3 in j..=j2 {
                            sum += S[i3][j3] as usize;
                        }
                    }
                    if sum <= K {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans
}

#[allow(unused_variables)]
fn main() {
    input! {
        h: usize, w: usize, K: usize,
        SS: [Chars; h],
    }

    let mut S = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if SS[i][j] == '1' {
                S[i][j] = 1
            }
        }
    }

    let mut accum_sums_i = vec![vec![0]; h];
    for i in 0..h {
        for j in 0..w {
            let to_add = accum_sums_i[i][j] + S[i][j];
            accum_sums_i[i].push(to_add);
        }
    }
    let mut accum_sums_j = vec![vec![0]; w];
    for i in 0..w {
        for j in 0..h {
            let to_add = accum_sums_j[i][j] + S[j][i];
            accum_sums_j[i].push(to_add);
        }
    }
    let mut ans = 0usize;

    for i in 0..h {
        for j in 0..w {
            let mut sum = 0;
            let mut right = w;
            for k in i..h {
                md!(i, j, k);
                sum += accum_sums_i[k][right] - accum_sums_i[k][j];
                md!(sum);
                while sum > K && right > j {
                    right -= 1;
                    sum -= accum_sums_j[right][k + 1] - accum_sums_j[right][i];
                    md!(right, sum);
                }
                ans += (right - j) as usize;
                md!("'''", i, j, ans);
            }
        }
    }
    md!(ans);
    if K > 0 {
        for i in 0..h {
            for j in 0..w {
                let mut sum = 0;
                let mut right = w;
                for k in i..h {
                    md!(i, j, k);
                    sum += accum_sums_i[k][right] - accum_sums_i[k][j];
                    md!(sum);
                    while sum > K - 1 && right > j {
                        right -= 1;
                        sum -= accum_sums_j[right][k + 1] - accum_sums_j[right][i];
                        md!(right, sum);
                    }
                    ans -= (right - j) as usize;
                    md!(ans);
                }
            }
        }
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
