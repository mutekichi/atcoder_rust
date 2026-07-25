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
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut P: (i64, i64),
            mut Q: (i64, i64),
            mut R: (i64, i64),
            mut S: (i64, i64),
        }
        P.0 *= 2;
        P.1 *= 2;
        Q.0 *= 2;
        Q.1 *= 2;
        R.0 *= 2;
        R.1 *= 2;
        S.0 *= 2;
        S.1 *= 2;

        let vec1x = P.0 - Q.0;
        let vec1y = P.1 - Q.1;
        let vec2x = R.0 - S.0;
        let vec2y = R.1 - S.1;
        let is_parallel = vec1x * vec2y - vec1y * vec2x == 0;
        let ok;
        if is_parallel {
            md!("is_parallel");
            let center1x = (P.0 + Q.0) / 2;
            let center1y = (P.1 + Q.1) / 2;
            let center2x = (R.0 + S.0) / 2;
            let center2y = (R.1 + S.1) / 2;
            if center1x == center2x && center1y == center2y {
                md!("parallel and center");
                ok = true;
            } else {
                let vecx = center1x - center2x;
                let vecy = center1y - center2y;
                if vec1x * vecx + vec1y * vecy == 0 {
                    md!("parallel and ortho");
                    ok = true;
                } else {
                    md!("parallel and not ortho");
                    ok = false;
                }
            }
        } else {
            md!("not is_parallel");
            ok = true;
        }
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
