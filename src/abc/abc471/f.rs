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
        n: usize,
        k: usize,
        S: [Chars; n],
    }
    let S = S
        .iter()
        .map(|s| {
            s.iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut data = vec![];
    let mut non_lzero_max = None;

    let mut counts_keta = vec![0; 10];

    for S in S {
        let mut leading_zeros = 0;
        let mut non_zero_val = 0;
        let mut is_leading_zero = true;
        for i in 0..S.len() {
            if is_leading_zero {
                if S[i] == 0 {
                    leading_zeros += 1i64;
                } else {
                    is_leading_zero = false;
                }
            }
            non_zero_val *= 10i64;
            non_zero_val += S[i] as i64;
        }
        if leading_zeros == 0 {
            if non_lzero_max.is_none() {
                non_lzero_max = Some((non_zero_val, S.len()));
            } else {
                non_lzero_max = Some(max(non_lzero_max.unwrap(), (non_zero_val, S.len())));
            }
        }
        data.push((S.len() as i64, non_zero_val, leading_zeros));
        counts_keta[S.len() - 1] += 1;
    }
    data.sort_unstable();
    data.reverse();

    let mut to_use_cands = vec![vec![]; 2];
    for i in 0..k {
        to_use_cands[0].push(data[i]);
        to_use_cands[1].push(data[i]);
    }
    let mut ans = vec![vec![]; 2];
    let mut ng = false;
    if k != n {
        to_use_cands[1].pop();
        let mut max_val = 0;
        for i in k..n {
            max_val = max(max_val, data[i].1);
        }
        if max_val == 0 {
            ng = true;
        }
        let mut val = max_val;
        let mut to_push = vec![];
        while val > 0 {
            to_push.push(char::from_digit(val as u32 % 10, 10).unwrap());
            val /= 10;
        }
        while let Some(val) = to_push.pop() {
            ans[1].push(val);
        }
    }

    let mut pows = vec![1i64];
    for i in 0..12 {
        pows.push(pows[i] * 10);
    }
    for i in 0..2 {
        to_use_cands[i].sort_by(|&a, &b| concat(b, a, &pows).cmp(&concat(a, b, &pows)));
    }

    for j in 0..2 {
        if j == 1 && ng {
            break;
        }
        let mut ok = false;
        for i in 0..k {
            if j == 1 && i == k - 1 {
                continue;
            }
            let (len, val, leading_zeros) = to_use_cands[j][i];
            md!(j, len, val, leading_zeros);
            if j == 1 || ok {
                for _ in 0..leading_zeros {
                    ans[j].push('0');
                }
            }
            if val > 0 {
                ok = true;
            }
            let mut to_push = vec![];
            let mut val = val;
            while val > 0 {
                to_push.push(char::from_digit(val as u32 % 10, 10).unwrap());
                val /= 10;
            }
            while let Some(val) = to_push.pop() {
                ans[j].push(val);
            }
        }
    }
    md!(ans[0].len(), ans[1].len());
    if ans[0].len() > ans[1].len() {
        println!("{}", ans[0].iter().join(""));
    } else if ans[0].len() < ans[1].len() {
        println!("{}", ans[1].iter().join(""));
    } else {
        let mut idx = 0;
        let mut same = true;
        for i in 0..ans[idx].len() {
            print!("{}", ans[idx][i]);
            if same {
                if ans[0][i] > ans[1][i] {
                    same = false;
                } else if ans[0][i] < ans[1][i] {
                    same = false;
                    idx = 1;
                }
            }
        }
        println!();
    }
}

fn concat(
    a: (i64, i64, i64),
    b: (i64, i64, i64),
    pows: &Vec<i64>,
) -> i64 {
    a.1 * pows[b.0 as usize] + b.1
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
