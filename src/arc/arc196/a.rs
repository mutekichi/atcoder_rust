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
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        mut A: [i64; n],
    }

    let sum: i64 = A.iter().take(n).sum();
    let mut ans = INF_I64;
    if n % 2 == 0 {
        A.sort_unstable();
        ans = A.into_iter().take(n / 2).sum::<i64>() * 2;
    } else {
        let m = n / 2;
        let mut lower_sums_forward = vec![];
        let mut lower_sums_reverse = vec![];
        lower_sums_forward.push(0);
        lower_sums_reverse.push(0);
        {
            let mut lower_pq: BinaryHeap<i64> = BinaryHeap::new();
            let mut upper_pq: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
            let mut lower_sum = 0;
            for i in 0..m {
                let small = min(A[i * 2], A[i * 2 + 1]);
                let large = max(A[i * 2], A[i * 2 + 1]);
                if i == 0 {
                    lower_pq.push(small);
                    upper_pq.push(Reverse(large));
                    lower_sum += small;
                    lower_sums_forward.push(lower_sum);
                } else {
                    let &lower_top = lower_pq.peek().unwrap();
                    let &Reverse(upper_bottom) = upper_pq.peek().unwrap();
                    if lower_top > large {
                        upper_pq.push(Reverse(lower_top));
                        lower_pq.pop();
                        lower_pq.push(small);
                        lower_pq.push(large);
                        lower_sum += small + large - lower_top;
                        lower_sums_forward.push(lower_sum);
                    } else if upper_bottom < small {
                        lower_pq.push(upper_bottom);
                        upper_pq.pop();
                        upper_pq.push(Reverse(large));
                        upper_pq.push(Reverse(small));
                        lower_sum += upper_bottom;
                        lower_sums_forward.push(lower_sum);
                    } else {
                        lower_pq.push(small);
                        upper_pq.push(Reverse(large));
                        lower_sum += small;
                        lower_sums_forward.push(lower_sum);
                    }
                }
            }
        }
        {
            let mut lower_pq: BinaryHeap<i64> = BinaryHeap::new();
            let mut upper_pq: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
            let mut lower_sum = 0;
            for i in 0..m {
                let small = min(A[n - i * 2 - 1], A[n - i * 2 - 2]);
                let large = max(A[n - i * 2 - 1], A[n - i * 2 - 2]);
                if i == 0 {
                    lower_pq.push(small);
                    upper_pq.push(Reverse(large));
                    lower_sum += small;
                    lower_sums_reverse.push(lower_sum);
                } else {
                    let &lower_top = lower_pq.peek().unwrap();
                    let &Reverse(upper_bottom) = upper_pq.peek().unwrap();
                    if lower_top > large {
                        upper_pq.push(Reverse(lower_top));
                        lower_pq.pop();
                        lower_pq.push(small);
                        lower_pq.push(large);
                        lower_sum += small + large - lower_top;
                        lower_sums_reverse.push(lower_sum);
                    } else if upper_bottom < small {
                        lower_pq.push(upper_bottom);
                        upper_pq.pop();
                        upper_pq.push(Reverse(large));
                        upper_pq.push(Reverse(small));
                        lower_sum += upper_bottom;
                        lower_sums_reverse.push(lower_sum);
                    } else {
                        lower_pq.push(small);
                        upper_pq.push(Reverse(large));
                        lower_sum += small;
                        lower_sums_reverse.push(lower_sum);
                    }
                }
            }
        }
        for i in 0..(m + 1) {
            md!(A[i * 2], lower_sums_forward[i], lower_sums_reverse[m - i]);
            ans = min(
                ans,
                A[i * 2] + (lower_sums_forward[i] + lower_sums_reverse[m - i]) * 2,
            );
        }
    }
    wl!(sum - ans);
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

fn join_with_space<T: ToString>(arr: &[T]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
