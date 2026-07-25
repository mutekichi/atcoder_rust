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
const DIR4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIR8: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
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
        n: usize, m: usize,
        A: [usize; n],
        UV: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFind::new(n);
    for &(u, v) in &UV {
        if A[u] == A[v] {
            uf.unite(u, v);
        }
    }
    let mut set = BTreeSet::new();

    for &(u, v) in &UV {
        let u = uf.find(u);
        let v = uf.find(v);
        if u == v {
            continue;
        } else {
            set.insert((min(u, v), max(u, v)));
        }
    }

    let mut v_set = BTreeSet::new();
    let mut graph = vec![vec![]; n];
    for &(u, v) in set.iter() {
        v_set.insert((A[u], u));
        v_set.insert((A[v], v));
        if A[u] < A[v] {
            graph[u].push(v);
        } else {
            graph[v].push(u);
        }
    }
    let mut depths = vec![INF_USIZE; n];
    depths[uf.find(0)] = 0;
    for (_, v) in v_set {
        let d = depths[v];
        if d == INF_USIZE {
            continue;
        }
        for &u in &graph[v] {
            depths[u] = if depths[u] == INF_USIZE {
                d + 1
            } else {
                max(depths[u], d + 1)
            };
        }
    }
    println!(
        "{}",
        if depths[uf.find(n - 1)] == INF_USIZE {
            0
        } else {
            depths[uf.find(n - 1)] + 1
        }
    );
}

// FOR TEMPLATE INJECTIONS

#[derive(Debug, Clone)]
/// Union-Find (Disjoint Set Union)
///
/// # Usage
/// ```
/// // Initialize
/// let mut uf = UnionFind::new(n);
///
/// // Unite
/// uf.unite(0, 1);
///
/// // Check connectivity
/// if uf.same(0, 1) {
///     println!("Connected!");
/// }
///
/// // Get component size
/// println!("Size of 0: {}", uf.size(0));
/// ```

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    /// Path compression helper
    fn compress(
        &mut self,
        x: usize,
        root: usize,
    ) {
        self.parent[x] = root;
    }

    pub fn find(
        &mut self,
        x: usize,
    ) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            let root = self.find(p);

            // Comment out the line below to disable path compression
            self.compress(x, root);

            root
        }
    }

    pub fn unite(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        // self.unite_by_size(x, y)
        self.unite_by_rank(x, y)
    }

    pub fn unite_by_rank(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            if self.rank[root_x] == self.rank[root_y] {
                self.rank[root_x] += 1;
            }
        }
        true
    }

    pub fn unite_by_size(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    pub fn size(
        &mut self,
        x: usize,
    ) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    pub fn same(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        self.find(x) == self.find(y)
    }
}

// END TEMPLATE INJECTIONS
