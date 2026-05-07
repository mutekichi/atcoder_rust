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
    }
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    writeln!(out, "2").unwrap();
    let mut perm1 = vec![];
    perm1.push(n);
    for i in 1..=n - 1 {
        perm1.push(i);
    }
    let mut perm2 = vec![];
    perm2.push(2);
    perm2.push(1);
    for i in 3..=n {
        perm2.push(i);
    }
    writeln!(out, "{}", perm1.iter().join(" ")).unwrap();
    writeln!(out, "{}", perm2.iter().join(" ")).unwrap();

    out.flush().unwrap();

    input! {
        res: i32,
    }
    if res == -1 {
        return;
    }
    let mut A = vec![res as usize];
    input! {
        B: [usize; n - 1],
    }
    md!(B.iter().join(" "));
    for b in B {
        A.push(b);
    }

    md!(A.iter().join(" "));
    let mut A_copy = A.clone();
    let mut cursor = 0;
    let mut operations = vec![];
    loop {
        let next_cursor = if cursor + 1 == n { 0 } else { cursor + 1 };
        if A[cursor] > A[next_cursor] {
            if A[cursor] == n && A[next_cursor] == 1 {
            } else {
                operations.push(2);
                A.swap(cursor, next_cursor);
            }
        }
        operations.push(1);
        cursor += 1;
        if cursor == n {
            cursor = 0;
        }
        let mut ok = true;
        for i in 0..n {
            let j = if i == n - 1 { 0 } else {i +1};
            if A[i] == n { 
                if A[j] != 1 {
                    ok = false;
                    break;
                }
            }
            else {
                if A[j] != A[i] + 1 {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            break;
        }

        md!(cursor);
        md!(A.iter().join(" "));
    }
    while A[cursor] != 1 {
        operations.push(1);
        cursor += 1;
        if cursor == n {
            cursor = 0;
        }
    }
    for &op in &operations {
        if op == 1 {
            A_copy.push(A_copy[0]);
            A_copy.remove(0);
        } else {
            A_copy.swap(0, 1);
        }
        md!(A_copy.iter().join(""));
    }
    assert!(A_copy.iter().join("") == (1..=n).join(""));

    println!("{} {}", operations.len(), operations.iter().rev().join(" "));
}

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS
