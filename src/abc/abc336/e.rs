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

#[allow(unused_variables)]
fn main() {
    input! {
        n: Chars,
    }
    let digits = n
        .iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let n = n.len();

    let mut ans = 0u64;
    for total_sum in 1..=14 * 9 {
        // dp[i][j][k][l] = i 桁目まで、和がj、総和 mod total_sum が k、l: is_less
        let mut dp = vec![vec![vec![vec![0; 2]; total_sum]; total_sum + 1]; n + 1];
        dp[0][0][0][0] = 1;
        // i-th digit
        for i in 0..n {
            for digit in 0..10 {
                for sum in 0..=total_sum {
                    let next_sum = sum + digit;
                    if next_sum > total_sum {
                        continue;
                    }
                    for mod_sum in 0..total_sum {
                        let next_mod = (mod_sum * 10 + digit) % total_sum;
                        md!(next_sum, next_mod, sum, mod_sum, digit);
                        // less -> less
                        dp[i + 1][next_sum][next_mod][1] += dp[i][sum][mod_sum][1];
                        // not less -> less
                        if digit < digits[i] {
                            dp[i + 1][next_sum][next_mod][1] += dp[i][sum][mod_sum][0];
                        }
                        // not less -> not less
                        if digit == digits[i] {
                            dp[i + 1][next_sum][next_mod][0] += dp[i][sum][mod_sum][0];
                        }
                    }
                }
            }
        }
        ans += dp[n][total_sum][0][0];
        ans += dp[n][total_sum][0][1];
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
