#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};
use std::thread::current;

use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }
    #[derive(Clone)]
    enum Query {
        Query1(usize),
        Query2(usize, Vec<char>),
        Query3(usize),
    }

    input! {
        n: usize,
        q: usize,
    }
    let mut queries = vec![];
    
    for _ in 0..q {
        input! {
            qtype: usize,
        }
        if qtype == 1 {
            input! {
                p: Usize1,
            }
            queries.push(Query::Query1(p));
        }
        else if qtype == 2 {
            input! {
                p: Usize1,
                s: Chars,
            }
            queries.push(Query::Query2(p, s));
        }
        else {
            input! {
                p: Usize1,
            }
            queries.push(Query::Query3(p));
        }
    }
    let mut current_string = VecDeque::new();
    let mut to_trace = n;
    for i in (0..q).rev() {
        let query = &queries[i];
        match query {
            Query::Query1(p) => {
                if to_trace == *p {
                    to_trace = n;
                }
            }
            Query::Query2(p, s) => {
                if to_trace == *p {
                    for i in (0..s.len()).rev() {
                        current_string.push_front(s[i]);
                    }
                }
            }
            Query::Query3(p) => {
                if to_trace == n {
                    to_trace = *p;
                }
            }
        }
    }
    let mut ans = vec![];
    while let Some(f) = current_string.pop_front() {
        ans.push(f);
    }
    wl!(ans.into_iter().collect::<String>());
}

// --- Macros ---

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

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint without newline)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mep {
    ($x:expr) => { eprint!("{}", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mep {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
// Usage: mep!(val) (-> eprint with space)
// mep!("{:<1$}", val, width) (-> left align with width)
// mep!("{:>1$}", val, width)
macro_rules! mepw { // stands for my_eprint_whitespace
    ($x:expr) => { eprint!("{} ", $x); };
    ($($arg:tt)+) => { eprint!($($arg)+); };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! mepw {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}
