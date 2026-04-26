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
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut A: Chars,
            mut B: Chars,
        }
        A.insert(0, ')');
        B.insert(0, ')');

        let A = compress(&mut A);
        let B = compress(&mut B);
        md!(A, B);

        if A == B {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn compress(S: &mut Vec<char>) -> String {
    let mut state = 0; // 0: invalid, 1: (, 2: (x, 3: (xx
    let mut prev = 0;
    for i in 0..S.len() {
        md!(i, state, prev);
        md!(S.iter().join(""));

        if S[i] == '(' {
            state = 1;
            prev = i;
        } else if S[i] == ')' {
            if state == 3 {
                S[prev] = '*';
                S[i] = '*';
                if S[prev - 1] == '(' {
                    prev = prev - 1;
                    state = 3;
                } else {
                    state = 0;
                }
            } else {
                state = 0;
            }
        } else if S[i] == 'x' {
            if state == 0 {
                continue;
            } else if state == 1 {
                state = 2;
            } else if state == 2 {
                state = 3;
            } else {
                state = 0;
            }
        }
    }

    S.iter().filter(|&c| *c != '*').collect::<String>()
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
