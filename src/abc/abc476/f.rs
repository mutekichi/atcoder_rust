#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use memoise::memoise;
use num_integer::gcd;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::btree_map::Entry;
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize, m: i64, A: [i64; n], B: [i64; n],
    }

    let mut diff_right = 0;
    let mut diff_down = 0;
    for i in 0..n {
        for j in 0..n {
            let val = A[i] * B[j] % m;
            if i == 0 && j == 0 {
                diff_right += val;
                diff_down += val;
                continue;
            } else {
                if i < j {
                    diff_right -= val;
                }
                if j < i {
                    diff_down -= val;
                }
            }
        }
    }
    md!(diff_right);
    let mut j_sum = vec![];
    for j in 0..n {
        let mut sum = 0;
        for i in 0..n {
            let val = A[i] * B[j] % m;
            sum += val;
        }
        j_sum.push(sum);
    }
    let mut migishita_sum = vec![0; 2 * n - 1];
    let mut migiue_sum = vec![0; 2 * n - 1];

    for i in 0..n {
        for j in 0..n {
            let val = A[i] * B[j] % m;
            migishita_sum[n - 1 + i - j] += val;
            migiue_sum[i + j] += val;
        }
    }
    md!(migishita_sum.iter().join(" "));
    md!(migiue_sum.iter().join(" "));
    md!(j_sum.iter().join(" "));

    let mut val = 0;
    for i in 0..n {
        for j in 0..n {
            let mval = A[i] * B[j] % m;
            val += mval * max(i, j) as i64;
        }
    }
    let mut ans = 0;
    md!(diff_right);
    md!(diff_down);

    let mut prev_val = val;
    let mut prev_diff_right = diff_right;
    for i in 0..n {
        val = prev_val;
        diff_right = prev_diff_right;
        for j in 0..n {
            md!(i, j, val);
            ans ^= val + (i * n + j) as i64;
            val += diff_right;
            if j != n - 1 {
                let j = j + 1;
                diff_right += migishita_sum[n - 1 + i - j];
                diff_right += migiue_sum[i + j];
            }
        }
        if i != n - 1 {
            md!(diff_down);
            md!(prev_diff_right);
            prev_val += diff_down;
            let i = i + 1;
            let j = 0;
            diff_down += migishita_sum[n - 1 + i - j];
            diff_down += migiue_sum[i + j];
            let i = i - 1;
            let j = 1;
            prev_diff_right += migishita_sum[n - 1 + i - j];
            prev_diff_right += migiue_sum[i + j];
        }
    }
    println!("{}", ans);
}

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[rustfmt::skip]
const DIR8: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
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

trait AsciiExt {
    fn to_idx(self) -> usize;
}

impl AsciiExt for char {
    fn to_idx(self) -> usize {
        (self as u8 - b'a') as usize
    }
}

impl AsciiExt for u8 {
    fn to_idx(self) -> usize {
        (self - b'a') as usize
    }
}

trait UsizeExt {
    fn to_char(self) -> char;
}

impl UsizeExt for usize {
    fn to_char(self) -> char {
        (self as u8 + b'a') as char
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
