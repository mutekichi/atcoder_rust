#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
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
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        _S: Chars,
        _T: Chars,
    }

    let S = _S
        .iter()
        .map(|c| (*c as u8 - b'a') as usize)
        .collect::<Vec<_>>();
    let T = _T
        .iter()
        .map(|c| (*c as u8 - b'a') as usize)
        .collect::<Vec<_>>();

    let mut to_list = vec![INF_USIZE; 26];
    let mut from_list = vec![vec![]; 26];
    let mut graph = vec![vec![]; 26];
    let mut used_for_loop = vec![false; 26];
    let mut self_loop = vec![false; 26];

    for i in 0..n {
        let s = S[i];
        let t = T[i];

        if to_list[s] != INF_USIZE {
            if to_list[s] == t {
                continue;
            } else {
                wl!(-1);
                return;
            }
        }
        to_list[s] = t;
        if s != t {
            from_list[t].push(s);
            graph[s].push(t);
            graph[t].push(s);
        } else {
            used_for_loop[s] = true;
            self_loop[s] = true;
        }
    }

    for i in 0..26 {
        if to_list[i] == i {
            to_list[i] = INF_USIZE;
        }
    }

    let mut ans = 0usize;
    let mut seen = vec![false; 26];

    for i in 0..26 {
        if seen[i] {
            continue;
        }
        let mut nodes = vec![];
        let mut q = VecDeque::new();
        seen[i] = true;
        q.push_back(i);
        while let Some(v) = q.pop_front() {
            nodes.push(v);
            for &nv in &graph[v] {
                if !seen[nv] {
                    q.push_back(nv);
                    seen[nv] = true;
                }
            }
        }

        // only self loop
        if nodes.len() == 1 {
            continue;
        }
        // loop
        let mut all_one_in_deg = true;
        // has sink
        let mut all_one_to = true;

        for &v in &nodes {
            if from_list[v].len() != 1 {
                all_one_in_deg = false;
            }
            if to_list[v] == INF_USIZE {
                all_one_to = false;
            }
        }
        if all_one_in_deg {
            ans += nodes.len() + 1;
        } else if all_one_to {
            ans += nodes.len();
        } else {
            ans += nodes.len() - 1;
        }
        if all_one_in_deg {
            for v in nodes {
                used_for_loop[v] = true;
            }
        }
    }
    if used_for_loop.iter().all(|v| *v) {
        if self_loop.iter().all(|v| *v) {
            wl!(0);
            return;
        }
        wl!(-1);
        return;
    }
    wl!(ans);
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

trait JoinExtended {
    fn join_with(
        self,
        sep: &str,
    ) -> String;
}

impl<I> JoinExtended for I
where
    I: Iterator,
    I::Item: Joinable,
{
    fn join_with(
        self,
        sep: &str,
    ) -> String {
        let mut peekable = self.peekable();
        let is_2d = if let Some(first) = peekable.peek() {
            first.is_container()
        } else {
            false
        };

        let res = peekable.map(|item| item.join_item(sep)).collect::<Vec<_>>();

        // Use newline for 2D rows, provided sep for 1D elements
        res.join(if is_2d { "\n" } else { sep })
    }
}

trait Joinable {
    fn join_item(
        &self,
        sep: &str,
    ) -> String;
    fn is_container(&self) -> bool;
}

macro_rules! impl_joinable_scalar {
    ($($t:ty),*) => {
        $(
            impl Joinable for &$t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
            impl Joinable for $t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
        )*
    };
}

impl_joinable_scalar!(
    i32, i64, i128, u32, u64, u128, usize, isize, f32, f64, char, String, &str
);

impl<T: std::fmt::Display> Joinable for &Vec<T> {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}

impl<T: std::fmt::Display> Joinable for &[T] {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}
