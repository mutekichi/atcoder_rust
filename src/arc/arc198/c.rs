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

fn error_swap(
    A: &mut Vec<i32>,
    i: usize,
    j: usize,
    operations: &mut Vec<(usize, usize)>,
) {
    let temp = A[i];
    A[i] = A[j] - 1;
    A[j] = temp + 1;
    operations.push((i, j));
}

fn main() {
    // let mut A = vec![0, 10, 20];
    // let B = vec![0, 9, 21];
    // for i in 0..3i32.pow(11) {
    //     A[0] = 0;
    //     A[1] = 10;
    //     A[2] = 20;

    //     let mut i = i;
    //     let mut operations = vec![];
    //     for _ in 0..11 {
    //         if i % 3 == 0 {
    //             error_swap(&mut A, 0, 1, &mut operations);
    //         } else if i % 3 == 1 {
    //             error_swap(&mut A, 0, 2, &mut operations);
    //         } else {
    //             error_swap(&mut A, 1, 2, &mut operations);
    //         }
    //         let mut ok = true;
    //         for j in 0..3 {
    //             if B[j] != A[j] {
    //                 ok = false;
    //                 break;
    //             }
    //         }
    //         if ok {
    //             println!("Ok");
    //             for &op in &operations {
    //                 println!("{}, {}", op.0, op.1);
    //             }
    //             return;
    //         }
    //         i /= 3;
    //     }
    // }
    // println!("Ng");
    // return;

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}
fn increment(
    A: &mut Vec<i32>,
    i: usize,
    operations: &mut Vec<(usize, usize)>,
) {
    if i == 0 {
        error_swap(A, i, i + 1, operations);
        error_swap(A, i + 1, i + 2, operations);
        error_swap(A, i, i + 1, operations);
        error_swap(A, i, i + 2, operations);
    } else if i == A.len() - 1 {
        unreachable!();
    } else {
        error_swap(A, i, i + 1, operations);
        error_swap(A, i - 1, i, operations);
        error_swap(A, i - 1, i + 1, operations);
        error_swap(A, i - 1, i, operations);
    }
}
fn decrement(
    A: &mut Vec<i32>,
    i: usize,
    operations: &mut Vec<(usize, usize)>,
) {
    if i == 0 {
        error_swap(A, i, i + 2, operations);
        error_swap(A, i + 1, i + 2, operations);
        error_swap(A, i, i + 2, operations);
        error_swap(A, i, i + 1, operations);
    } else if i == A.len() - 1 {
        unreachable!();
    } else {
        error_swap(A, i - 1, i, operations);
        error_swap(A, i - 1, i + 1, operations);
        error_swap(A, i - 1, i, operations);
        error_swap(A, i, i + 1, operations);
    }
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        mut A: [i32; n],
        B: [i32; n],
    }

    if A.iter().sum::<i32>() != B.iter().sum::<i32>() {
        wl!("No");
    } else {
        // let mut A_copy = A.clone();
        if n == 2 {
            if A[0] == B[0] {
                wl!("Yes");
                wl!(0);
            } else if A[0] == B[1] - 1 {
                wl!("Yes");
                wl!(1);
                wl!("1 2");
            } else {
                wl!("No");
            }
        } else {
            let mut operations = vec![];
            for i in 0..min(0, n.saturating_sub(4)) {
                if A[i] == B[i] {
                    continue;
                } else if A[i] < B[i] {
                    let mut idx = i;
                    if A[i] == B[i] - 1 {
                        increment(&mut A, i, &mut operations);
                        continue;
                    }
                    while A[idx] != B[i] + 1 {
                        if idx == n - 1 {
                            error_swap(&mut A, i, idx, &mut operations);
                            idx = i;
                        } else {
                            error_swap(&mut A, idx, idx + 1, &mut operations);
                            idx = idx + 1;
                        }
                    }
                    error_swap(&mut A, i, idx, &mut operations);
                    assert!(A[i] == B[i]);
                } else {
                    let mut idx = i;
                    if A[i] == B[i] + 1 {
                        decrement(&mut A, i, &mut operations);
                        continue;
                    }
                    while A[idx] != B[i] + 1 {
                        if idx == i {
                            error_swap(&mut A, idx, n - 1, &mut operations);
                            idx = n - 1;
                        } else {
                            error_swap(&mut A, idx, idx - 1, &mut operations);
                            idx = idx - 1;
                        }
                    }
                    error_swap(&mut A, i, idx, &mut operations);
                    assert!(A[i] == B[i]);
                }
            }
            for i in min(0, n.saturating_sub(4))..n {
                while A[i] != B[i] {
                    if A[i] < B[i] {
                        increment(&mut A, i, &mut operations);
                    } else {
                        decrement(&mut A, i, &mut operations);
                    }
                }
                assert!(A[i] == B[i]);
            }
            wl!("Yes");
            wl!(operations.len());
            for &op in &operations {
                wl!("{} {}", op.0 + 1, op.1 + 1);
            }
            wl!(operations.len());
            // let mut _ops = vec![];
            // for &op in &operations {
            //     error_swap(&mut A_copy, op.0, op.1, &mut _ops);
            // }
            //     eprintln!("{}", join_with_space(&A_copy));
            //     eprintln!("{}", join_with_space(&B));
        }
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

fn join_with_space<T: ToString>(arr: &[T]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
