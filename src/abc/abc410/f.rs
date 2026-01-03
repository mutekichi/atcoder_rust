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

    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            _h: usize,
            _w: usize,
            S: [Chars; _h],
        }
        // ensures  h <= w
        let h = min(_h, _w);
        let w = max(_h ,_w);
        let mut grid: Vec<Vec<i64>> = vec![vec![0; w]; h];
        if _h < _w {
            for i in 0..h {
                for j in 0..w {
                    if S[i][j] == '#' {
                        grid[i][j] = 1;
                    } else {
                        grid[i][j] = -1;
                    }
                }
            }
        } else {
            for i in 0..h {
                for j in 0..w {
                    if S[j][i] == '#' {
                        grid[i][j] = 1;
                    }
                    else {
                        grid[i][j] = -1;
                    }
                }
            }
        }

        for i in 1..h {
            for j in 0..w {
                grid[i][j] += grid[i - 1][j];
            }
        }
        for i in 0..h {
            for j in 1..w {
                grid[i][j] += grid[i][j - 1];
            }
        }

        let mut ans: i64 = 0;
        let offset = h * w;
        let mut counts = vec![0; h * w * 2 + 1];
        for i in 0..h {
            for j in i..h {
                counts[offset] = 1;
                let mut used_list = vec![];
                for k in 0..w {
                    let count = (grid[j][k] - if i > 0 { grid[i - 1][k] } else {0} + offset as i64) as usize;
                    ans += counts[count];
                    counts[count] += 1;
                    used_list.push(count);
                }
                for used in used_list {
                    counts[used] = 0;
                }
            }
        }
        md!(ans);
        wl!(ans);
    }
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
