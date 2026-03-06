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
        A: [[i64; 3]; 3],
    }
    let mut state = vec![vec![0; 3]; 3];
    if check_wins_tern_player(&A, &mut state, true, 0) {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }
}

fn check_wins_tern_player(
    A: &Vec<Vec<i64>>,
    state: &mut Vec<Vec<usize>>,
    is_first_player_tern: bool,
    tern: usize,
) -> bool {
    for i in 0..3 {
        if state[0][i] == state[1][i] && state[1][i] == state[2][i] && state[0][i] != 0 {
            return false;
        }
        if state[i][0] == state[i][1] && state[i][1] == state[i][2] && state[i][0] != 0 {
            return false;
        }
    }
    if state[0][0] == state[1][1] && state[1][1] == state[2][2] && state[0][0] != 0 {
        return false;
    }
    if state[0][2] == state[1][1] && state[2][0] == state[1][1] && state[0][2] != 0 {
        return false;
    }
    if tern == 9 {
        let mut sum_first = 0;
        let mut sum_second = 0;
        for i in 0..3 {
            for j in 0..3 {
                if state[i][j] == 1 {
                    sum_first += A[i][j];
                }
                else {
                    sum_second += A[i][j];
                }
            }
        }
        if sum_first > sum_second {
            return false;
        }
        else {
            return true;
        }
    }
    let mut ans = false;
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] == 0 {
                state[i][j] = if is_first_player_tern { 1 } else { 2 };
                if !check_wins_tern_player(A, state, !is_first_player_tern, tern + 1) {
                    ans = true;
                }
                state[i][j] = 0;
            }
        }
    }
    ans
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
