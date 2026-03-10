#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! md {
    ($($arg:expr),* $(,)?) => {{
        eprint!("[{}:{}] ", file!(), line!());
        let mut _first = true;
        $(
            if !_first { eprint!(", "); }
            eprint!("{}: {}", stringify!($arg), $arg);
            _first = false;
        )*
        eprintln!();
    }};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! md {
    ($($arg:expr),* $(,)?) => {{}};
}

fn main() {
    input! {
        n: usize,
        h: [i32; n],
        c: [i32; n],
        a: [[i32; n]; n],
    }
    for i in 0..n {
        let mut b = vec![];
        for j in 0..n {
            b.push(a[i][j]);
        }
        b.sort_unstable();
        md!(b.iter().rev().take(5).join(" "));
    }
}
