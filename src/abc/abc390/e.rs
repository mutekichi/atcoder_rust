#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use rand::Rng;
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
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

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
        x: usize,
        VAC: [(Usize1, i64, usize); n],
    }
    let table = (0..3)
        .map(|i| {
            let n = VAC.iter().filter(|(v, _, _)| *v == i).count();
            let AC = VAC
                .iter()
                .filter(|(v, _, _)| *v == i)
                .map(|(_, a, c)| (*a, *c))
                .collect::<Vec<_>>();
            let mut dp_table = vec![vec![0; x + 1]; n + 1];
            for i in 0..n {
                let (a, c) = AC[i];
                for j in 0..=x {
                    if j > 0 {
                        dp_table[i + 1][j] = max(dp_table[i][j], dp_table[i][j - 1]);
                    }
                    if j >= c {
                        dp_table[i + 1][j] = max(dp_table[i][j], dp_table[i][j - c] + a);
                    }
                }
            }
            dp_table[n].clone()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..=x {
        for j in 0..(x - i) {
            let k = x - i - j;
            ans = max(min(table[0][i], min(table[1][j], table[2][k])), ans);
        }
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
