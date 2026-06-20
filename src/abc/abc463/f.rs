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

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        A: [usize; 2 * n],
    }
    if A.iter().all(|a| *a == A[0]) {
        let mut ans = vec![];
        for i in 0..2 * n {
            ans.push(Mint998::new(1) / Mint998::new(2 * n as i64));
        }
        println!("{}", ans.iter().join(" "));
        return;
    }
    let max_val = *A.iter().max().unwrap();
    let mut has_two_patterns = true;
    let mut vals = vec![];

    let mut c00 = 0;
    let mut c01 = 0;
    let mut c02 = 0;
    let mut c11 = 0;
    let mut c12 = 0;

    for i in 0..n {
        let a = A[2 * i];
        let b = A[2 * i + 1];
        let mut val = vec![];
        for x in vec![a, b] {
            val.push(if x == max_val {
                0
            } else if x == max_val - 1 {
                1
            } else {
                2
            });
        }
        val.sort();
        if val[0] == 0 && val[1] == 0 {
            has_two_patterns = false;
            c00 += 1;
        } else if val[0] == 0 && val[1] == 1 {
            c01 += 1;
        } else if val[0] == 0 && val[1] == 2 {
            c02 += 1;
        } else if val[0] == 1 && val[1] == 1 {
            c11 += 1;
        } else if val[0] == 1 && val[1] == 2 {
            c12 += 1;
        }

        vals.push(val);
    }
    md!(c00, c01, c02, c11, c12);
    let mut ans_vec = vec![Mint998::new(0); 2 * n];
    let comb = Combination::<C998244353>::new(2 * n);

    // pattern 1: score == max_val
    if has_two_patterns {
        md!("has");
        assert_eq!(c00, 0);
        // for 11
        {
            let nn = n;
            let mut ans = Mint998::new(1);
            // c01 のところは 1 が勝つ前提
            let n = c12 as usize;
            for i in 0..=n as i64 {
                ans += comb.ncr(n, i as usize) / Mint998::new(c11 + c01 + i);
            }
            // target c11
            ans /= Mint998::new(2);
            // c12
            for i in 0..n {
                ans /= Mint998::new(2);
            }
            // c01, c02 はすべて 0 でない側が勝つ
            for i in 0..(c01 + c02) {
                ans /= Mint998::new(2);
            }
            for i in 0..nn {
                if vals[i][0] == 1 && vals[i][1] == 1 {
                    md!(i);
                    ans_vec[i * 2] += ans;
                    ans_vec[i * 2 + 1] += ans;
                }
            }
        }
        // for 01
        {
            let nn = n;
            let mut ans = Mint998::new(1);
            let n = c12 as usize;
            for i in 0..=n {
                ans += comb.ncr(n, i as usize) / Mint998::new(c11 + c01 + i as i64);
            }
            for i in 0..n {
                ans /= Mint998::new(2);
            }
            // c01, c02 はすべて 0 でない側が勝つ
            for i in 0..(c01 + c02) {
                ans /= Mint998::new(2);
            }
            for i in 0..nn {
                if vals[i][0] == 0 && vals[i][1] == 1 {
                    let idx = vec![i * 2, i * 2 + 1];
                    for idx in idx {
                        if A[idx] == max_val - 1 {
                            ans_vec[idx] += ans;
                        }
                    }
                }
            }
        }
        // for 12
        {
            let nn = n;
            let mut ans = Mint998::new(1);
            let n = c12 as usize;
            for i in 0..=n - 1 {
                ans += comb.ncr(n - 1, i as usize) / Mint998::new(c11 + c01 + i as i64);
            }
            for i in 0..n {
                ans /= Mint998::new(2);
            }
            for i in 0..(c01 + c02) {
                ans /= Mint998::new(2);
            }
            for i in 0..nn {
                if vals[i][0] == 1 && vals[i][1] == 2 {
                    let idx = vec![i * 2, i * 2 + 1];
                    for idx in idx {
                        if A[idx] == max_val - 1 {
                            ans_vec[idx] += ans;
                        }
                    }
                }
            }
        }
    }

    // pattern 2: score == max_val + 1
    // for 00
    {
        let nn = n;
        let mut ans = Mint998::new(1);
        let n = c01 + c02;
        for i in 0..=n as i64 {
            ans += comb.ncr(n as usize, i as usize) / Mint998::new(c00 + i);
        }
        for i in 0..n + 1 {
            ans /= Mint998::new(2);
        }
        for i in 0..nn {
            if vals[i][0] == 0 && vals[i][1] == 0 {
                ans_vec[i * 2] += ans;
                ans_vec[i * 2 + 1] += ans;
            }
        }
    }

    // for 01 / 02
    {
        let nn = n;
        let mut ans = Mint998::new(1);
        let n = c01 + c02;
        for i in 0..=n as i64 - 1 {
            ans += comb.ncr(n as usize - 1, i as usize) / Mint998::new(c00 + 1 + i);
        }
        for i in 0..n {
            ans /= Mint998::new(2);
        }
        for i in 0..nn {
            if vals[i][0] == 0 && vals[i][1] != 0 {
                let idx = vec![i * 2, i * 2 + 1];
                for idx in idx {
                    if A[idx] == max_val {
                        ans_vec[idx] += ans;
                    }
                }
            }
        }
    }
    println!("{}", ans_vec.iter().join(" "));
}

// FOR TEMPLATE INJECTIONS