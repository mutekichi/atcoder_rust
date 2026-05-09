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
        // board: [[i64; n]; n],
    }
    let mut board = vec![vec![0i64; n]; n];
    for i in 1..n {
        board[n - 2][i] = 1;
    }
    for i in (0..n - 2).rev() {
        for j in 1..n {
            let mut prev_sum = 0;
            for jk in 0..j {
                prev_sum += board[i][jk];
            }
            for jk in j - 1..n {
                prev_sum += board[i + 1][jk];
            }
            for ik in i + 2..n {
                prev_sum += board[ik][n - 1];
            }
            for jk in 0..j {
                prev_sum -= board[i][jk];
            }
            for ik in i + 1..n {
                prev_sum -= board[ik][j];
            }
            board[i][j] = prev_sum + 1;
        }
    }

    // let mut set = BTreeSet::new();
    // dfs(n, &board, 0, 0, 0, &mut set);

    for i in 0..n {
        println!("{}", board[i].iter().join(" "));
    }

}

fn dfs(
    n: usize,
    board: &Vec<Vec<i64>>,
    i: usize,
    j: usize,
    sum: i64,
    set: &mut BTreeSet<i64>,
) {
    if i == n - 1 && j == n - 1 {
        assert!(!set.contains(&sum));
        assert!(sum < 6 * 1000000);
        set.insert(sum);
        return;
    }
    if i < n - 1 {
        dfs(n, board, i + 1, j, sum + board[i][j], set);
    }
    if j < n - 1 {
        dfs(n, board, i, j + 1, sum + board[i][j], set);
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
