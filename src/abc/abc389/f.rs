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

/// Lazy Segment Tree (Range Update, Range Query)
///
/// Processes range updates and range queries in O(log N).
///
/// # Generics
/// - `T`: Type of data (Element of Monoid)
/// - `U`: Type of operator (Element of Operator Monoid)
/// - `F`: Binary operation for data `f(T, T) -> T`
/// - `M`: Function to apply operator to data `mapping(x, op) -> T`
/// - `C`: Function to compose operators `composition(new_op, old_op) -> U` (new_op ∘ old_op)
///
/// # Examples
///
/// ## 1. Range Update Query (RUQ) + Range Minimum Query (RMQ)
/// ```
/// let inf = std::i64::MAX;
/// let id = std::i64::MAX;
///
/// let data = vec![1, 2, 3, 4, 5];
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| std::cmp::min(a, b),
///     inf,
///     |x, op| if op == id { x } else { op },
///     |new_op, old_op| if new_op == id { old_op } else { new_op },
///     id
/// );
/// ```
///
/// ## 2. Range Add Query (RAQ) + Range Sum Query (RSQ)
/// ```
/// #[derive(Clone, Copy, Debug)]
/// struct S { val: i64, size: i64 }
///
/// let data_raw = vec![1, 2, 3, 4, 5];
/// let data: Vec<S> = data_raw.iter().map(|&x| S { val: x, size: 1 }).collect();
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| S { val: a.val + b.val, size: a.size + b.size },     // op: sum
///     S { val: 0, size: 0 },                                      // e
///     |x, op| S { val: x.val + op * x.size, size: x.size },       // mapping: add (op * width)
///     |new_op, old_op| new_op + old_op,                           // composition: sum of additions
///     0                                                           // id: 0
/// );
/// ```
///
/// ## 3. Range Affine Update + Range Sum Query
/// ```
/// #[derive(Clone, Copy, Debug)]
/// struct S { val: i64, size: i64 }
/// type Op = (i64, i64); // (b, c) -> x * b + c
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| S { val: a.val + b.val, size: a.size + b.size },
///     S { val: 0, size: 0 },
///     |x, op| S { val: x.val * op.0 + op.1 * x.size, size: x.size },
///     |new_op, old_op| (new_op.0 * old_op.0, new_op.0 * old_op.1 + new_op.1),
///     (1, 0)
/// );
/// ```
#[derive(Debug, Clone)]
pub struct LazySegmentTree<T, U, F, M, C> {
    n: usize,
    size: usize,
    log: usize,
    tree: Vec<T>,
    lazy: Vec<U>,
    op_tree: F,
    e: T,
    mapping: M,
    composition: C,
    id: U,
}

impl<T, U, F, M, C> LazySegmentTree<T, U, F, M, C>
where
    T: Copy + Clone + std::fmt::Debug,
    U: Copy + Clone + std::fmt::Debug + PartialEq,
    F: Fn(T, T) -> T,
    M: Fn(T, U) -> T,
    C: Fn(U, U) -> U,
{
    pub fn new(
        data: &[T],
        op_tree: F,
        e: T,
        mapping: M,
        composition: C,
        id: U,
    ) -> Self {
        let n = data.len();
        let mut log = 0;
        while (1 << log) < n {
            log += 1;
        }
        let size = 1 << log;

        let mut tree = vec![e; 2 * size];
        let lazy = vec![id; size];

        for (i, &item) in data.iter().enumerate() {
            tree[size + i] = item;
        }
        for i in (1..size).rev() {
            tree[i] = op_tree(tree[2 * i], tree[2 * i + 1]);
        }

        LazySegmentTree {
            n,
            size,
            log,
            tree,
            lazy,
            op_tree,
            e,
            mapping,
            composition,
            id,
        }
    }

    pub fn set(
        &mut self,
        mut p: usize,
        x: T,
    ) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(
        &mut self,
        mut p: usize,
    ) -> T {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p]
    }

    pub fn prod(
        &mut self,
        mut l: usize,
        mut r: usize,
    ) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e;
        }

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let mut sml = self.e;
        let mut smr = self.e;

        while l < r {
            if l & 1 == 1 {
                sml = (self.op_tree)(sml, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op_tree)(self.tree[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op_tree)(sml, smr)
    }

    pub fn all_prod(&self) -> T {
        self.tree[1]
    }

    pub fn apply(
        &mut self,
        mut p: usize,
        op: U,
    ) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p] = (self.mapping)(self.tree[p], op);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn apply_range(
        &mut self,
        mut l: usize,
        mut r: usize,
        op: U,
    ) {
        assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let l_copy = l;
        let r_copy = r;

        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, op);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, op);
            }
            l >>= 1;
            r >>= 1;
        }

        l = l_copy;
        r = r_copy;

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    /// Finds the largest `r` such that `g(prod(l, r))` is true.
    pub fn max_right<G>(
        &mut self,
        mut l: usize,
        g: G,
    ) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(l <= self.n);
        assert!(g(self.e));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut sm = self.e;
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g((self.op_tree)(sm, self.tree[l])) {
                while l < self.size {
                    self.push(l);
                    l = 2 * l;
                    if g((self.op_tree)(sm, self.tree[l])) {
                        sm = (self.op_tree)(sm, self.tree[l]);
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = (self.op_tree)(sm, self.tree[l]);
            l += 1;
            if (l & !l) == l {
                break;
            }
        }
        self.n
    }

    /// Finds the smallest `l` such that `g(prod(l, r))` is true.
    pub fn min_left<G>(
        &mut self,
        mut r: usize,
        g: G,
    ) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(r <= self.n);
        assert!(g(self.e));
        if r == 0 {
            return 0;
        }
        r += self.size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut sm = self.e;
        loop {
            r -= 1;
            while r > 1 && (r % 2) == 1 {
                r >>= 1;
            }
            if !g((self.op_tree)(self.tree[r], sm)) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    if g((self.op_tree)(self.tree[r], sm)) {
                        sm = (self.op_tree)(self.tree[r], sm);
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = (self.op_tree)(self.tree[r], sm);
            if (r & !r) == r {
                break;
            }
        }
        0
    }

    /// Returns the current state of the underlying array.
    /// This method pushes all lazy updates to the leaves.
    pub fn to_vec(&mut self) -> Vec<T> {
        for i in 1..self.size {
            self.push(i);
        }
        self.tree[self.size..self.size + self.n].to_vec()
    }

    fn update(
        &mut self,
        k: usize,
    ) {
        self.tree[k] = (self.op_tree)(self.tree[2 * k], self.tree[2 * k + 1]);
    }

    fn all_apply(
        &mut self,
        k: usize,
        op: U,
    ) {
        self.tree[k] = (self.mapping)(self.tree[k], op);
        if k < self.size {
            self.lazy[k] = (self.composition)(op, self.lazy[k]);
        }
    }

    fn push(
        &mut self,
        k: usize,
    ) {
        self.all_apply(2 * k, self.lazy[k]);
        self.all_apply(2 * k + 1, self.lazy[k]);
        self.lazy[k] = self.id;
    }
}

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
        LR: [(usize, usize); n],
    }
    let size = 500100usize;
    let data = (0..size).collect::<Vec<usize>>();
    let mut lst_max = LazySegmentTree::new(
        &data,
        |a, b| max(a, b),
        0,
        |x, op| x + op,
        |new_op, old_op| new_op + old_op,
        0,
    );
    for (l, r) in LR {
        let left = lst_max.max_right(0, |u| u < l);
        let right = lst_max.max_right(0, |u| u <= r);
        md!(left, right);
        lst_max.apply_range(left, right, 1);
    }
    input! {
        q: usize,
        X: [usize; q],
    }
    for x in X {
        wl!(lst_max.get(x));
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
