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
            h: usize,
            w: usize,
            S: [Chars; h],
        }
        fn to_node_id(
            i: usize,
            j: usize,
            w: usize,
            dir: usize,
        ) -> usize {
            2 * (i * w + j) + dir
        }
        fn to_edge_pos(
            node_id: usize,
            w: usize,
        ) -> (usize, usize) {
            let i = node_id / 2 / w;
            let j = node_id / 2 % w;
            if node_id % 2 == 0 {
                (i * 2, j * 2 + 1)
            } else {
                (i * 2 + 1, j * 2)
            }
        }
        let mut graph = vec![vec![]; 2 * w * h];
        let mut states = vec![vec![0; 2 * w]; 2 * h];
        for i in 0..h {
            for j in 0..w {
                let top = {
                    let ii = (i + h - 1) % h;
                    let jj = j;
                    to_node_id(ii, j, w, 1)
                };
                let bottom = {
                    to_node_id(i, j, w, 1)
                };
                let right = {
                    to_node_id(i, j, w, 0)
                };
                let left = { 
                    let ii = i;
                    let jj = (j + w - 1) % w;
                    to_node_id(ii, jj, w, 0)
                };
                if S[i][j] == 'A' {
                    graph[top].push((bottom, false));
                    graph[bottom].push((top, false));
                    graph[left].push((right, false));
                    graph[right].push((left, false));
                } else {
                    graph[top].push((bottom, true));
                    graph[bottom].push((top, true));
                    graph[left].push((right, true));
                    graph[right].push((left, true));
                    graph[top].push((right, false));
                    graph[top].push((left, false));
                    graph[bottom].push((right, false));
                    graph[bottom].push((left, false));
                    graph[left].push((top, false));
                    graph[left].push((bottom, false));
                    graph[right].push((top, false));
                    graph[right].push((bottom, false));
                }
            }
        }
        let mut q = VecDeque::new();
        let mut remain_node_ids = BTreeSet::new();
        for i in 0..h {
            for j in 0..w {
                for dir in 0..2 {
                    let node_id = to_node_id(i, j, w, dir);
                    remain_node_ids.insert(node_id);
                }
            }
        }
        let mut count: i64 = 0;
        let mut ok = true;
        while let Some(node_id_to_add) = remain_node_ids.pop_first() {
            q.push_back(node_id_to_add);
            let (i, j) = to_edge_pos(node_id_to_add, w);
            states[i][j] = 1;
            count += 1;
            while let Some(node_id) = q.pop_front() {
                let (i, j) = to_edge_pos(node_id, w);
                let state = states[i][j];
                for &(next_node_id, sign) in &graph[node_id] {
                    let (next_i, next_j) = to_edge_pos(next_node_id, w);
                    let next_state = if sign { state } else { 3 - state };
                    if states[next_i][next_j] == 0 {
                        states[next_i][next_j] = next_state;
                        q.push_back(next_node_id);
                        remain_node_ids.remove(&next_node_id);
                    } else if states[next_i][next_j] != next_state {
                        md!(next_i, next_j, states[next_i][next_j], next_state);
                        wl!(0);
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            let mut ans = 1i64;
            for _ in 0..count {
                ans *= 2;
                ans %= 998244353i64;
            }
            wl!(ans);
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
