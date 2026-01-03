#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// Constants
const INF: i64 = 1 << 60;
const MOD: i64 = 998244353;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

/// Coordinate Compression (Zaatsu)
///
/// Compresses a set of values into indices [0, N-1] preserving order.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::utils::coordinate_compression::CoordinateCompression;
///
/// let data = vec![100, 2, 100, 50, 2];
/// let cc = CoordinateCompression::new(data);
///
/// assert_eq!(cc.size(), 3); // {2, 50, 100}
/// assert_eq!(cc.compress(&2), 0);
/// assert_eq!(cc.compress(&50), 1);
/// assert_eq!(cc.compress(&100), 2);
/// assert_eq!(cc.decompress(1), 50);
/// ```
#[derive(Debug, Clone)]
pub struct CoordinateCompression<T> {
    pub xs: Vec<T>,
}

impl<T: Ord + Clone + Copy> CoordinateCompression<T> {
    /// Constructs a new `CoordinateCompression` from a vector of values.
    ///
    /// Duplicates are removed and the values are sorted.
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort();
        data.dedup();
        CoordinateCompression { xs: data }
    }

    /// Returns the compressed index for the given value.
    ///
    /// # Panics
    /// Panics if the value is not found (use `binary_search` directly if handling missing values).
    pub fn compress(&self, val: &T) -> usize {
        self.xs
            .binary_search(val)
            .expect("Value not found in compressed coordinates")
    }

    /// Returns the original value for the given compressed index.
    pub fn decompress(&self, i: usize) -> T {
        self.xs[i]
    }

    /// Returns the number of unique values.
    pub fn size(&self) -> usize {
        self.xs.len()
    }
}

// END TEMPLATE INJECTIONS

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

// Logic goes here
#[allow(unused_macros)]
#[allow(unused_variables)]
#[rustfmt::skip]
fn solve<W: Write>(out: &mut W) {
    macro_rules! wl {
        ($x:expr) => { writeln!(out, "{}", $x).unwrap(); };
        ($($arg:tt)*) => { writeln!(out, $($arg)*).unwrap(); };
    }

    input! {
        n: usize,
        xr: [(i128, i128); n],
    }

    let mut _coords: Vec<i128> = Vec::new();

    for &(x, r) in xr.iter() {
        _coords.push(x - r);
        _coords.push(x + r);
    }

    let hashset: HashSet<i128> = _coords.into_iter().collect();
    let coords: Vec<i128> = hashset.into_iter().collect();

    let size = coords.len();
    md!(size);
    for &co in &coords {
        md!(co);
    }

    let compression = CoordinateCompression::new(coords);

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); size];

    for &(x, r) in xr.iter() {
        let u = compression.compress(&(x - r));
        let v = compression.compress(&(x + r));
        md!(u, v);
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited: Vec<bool> = vec![false; size];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    let mut ans = 0;

    for i in 0..size {
        if visited[i] {
            continue;
        } else {
            q.push_back((i, 1000000));
            visited[i] = true;
            let mut is_tree = true;
            let mut counts = 0;

            while !q.is_empty() {
                let (v, from) = q.pop_front().unwrap();
                md!(v, from);
                counts += 1;
                for &nv in &graph[v] {
                    if nv != from {
                        if visited[nv] {
                            is_tree = false;
                        } else {
                            q.push_back((nv, v));
                            visited[nv] = true;
                        }
                    }
                }
            }
            ans += counts;
            if is_tree {
                ans -= 1;
            }
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
