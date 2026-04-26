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
        H: [i32; n],
    }
    let mut data = (0..n).collect::<Vec<_>>();
    data.sort_by_key(|e| H[*e]);

    let mut set = BTreeSet::new();
    let mut ans = vec![0isize; n + 1];
    for &i in data.iter().rev() {
        let prev = set.range(..i).next_back();
        let plus = if prev.is_none() { 0 } else { prev.unwrap() + 1 };
        md!(plus, i);
        ans[plus] += 1;
        ans[i + 1] -= 1;
        set.insert(i);
    }
    for i in 0..n {
        ans[i + 1] += ans[i];
    }
    println!("{}", ans.iter().skip(1).join(" "));
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
