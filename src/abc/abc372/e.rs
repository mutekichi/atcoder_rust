#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};
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
        q: usize,
    }
    let mut data = vec![Vec::new(); n];
    for i in 0..n {
        data[i].push(i);
    }
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {
            qtype: usize,
        }
        if qtype == 1 {
            input! {
                u: Usize1, v: Usize1,
            }
            if uf.same(u, v) {
                continue;
            } else {
                let uroot = uf.find(u);
                let vroot = uf.find(v);
                uf.unite(uroot, vroot);
                let root = uf.find(u);
                let (from, to) = if root == uroot {
                    (vroot, uroot)
                } else if root == vroot {
                    (uroot, vroot)
                } else {
                    unreachable!();
                };
                md!(from, to);
                // assert!(data[from].len() <= data[to].len());
                let mut new_data = vec![];
                while let Some(vfrom) = data[from].pop() {
                    loop {
                        if data[to].last().is_some()
                            && *data[to].last().unwrap() > vfrom
                        {
                            let val = data[to].pop().unwrap();
                            new_data.push(val);
                            if new_data.len() == 10 {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    new_data.push(vfrom);
                    if new_data.len() == 10 {
                        break;
                    }
                }
                data[from].clear();
                md!(data[to].len());
                while let Some(vto) = data[to].pop() {
                    if new_data.len() < 10 {
                        new_data.push(vto);
                    }
                }
                while let Some(val) = new_data.pop() {
                    data[to].push(val);
                }
                // assert!(data[from].is_empty());
                // assert!(data[to].is_sorted());
                // assert!(data[to].len() <= 10);
                md!(data[to].len());
                md!(data[to].iter().join(" "));
                md!(data[from].iter().join(" "));
            }
        } else {
            input! {
                v: Usize1, k: usize,
            }
            md!(2, v, k);
            let v = uf.find(v);
            if data[v].len() < k {
                println!("{}", -1);
            } else {
                println!("{}", data[v][data[v].len() - k] + 1);
            }
        }
    }
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
            // self.compress(x, root);

            root
        }
    }

    pub fn unite(
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        self.unite_by_size(x, y)
        // self.unite_by_rank(x, y)
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
