#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

// Common imports
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

// External crates (Available in AtCoder)
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

// Constants
const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const MOD: i64 = 998244353;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

/// Lazy Segment Tree (Range Update, Range Query)
///
/// Processes range updates and range queries in O(log N) for a Monoid with an Operator.
///
/// # Generics
/// - `T`: Type of data (Element of Monoid)
/// - `U`: Type of operator (Element of Operator Monoid)
/// - `F`: Binary operation for data `f(T, T) -> T`
/// - `M`: Function to apply operator to data `mapping(f(T), U) -> T`
/// - `C`: Function to compose operators `composition(f(U), g(U)) -> U` (f ∘ g)
///
/// # Examples
///
/// ## 1. Range Update Query (RUQ) + Range Minimum Query (RMQ)
/// Range update, Range minimum.
/// - Data: `i64` (Inf)
/// - Operator: `i64` (None if specialized value, here we use i64::MAX as 'None')
///
/// ```
/// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
///
/// let inf = std::i64::MAX;
/// let id = std::i64::MAX; // Identity element (represents no update)
///
/// let data = vec![1, 2, 3, 4, 5];
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| std::cmp::min(a, b),                     // op: min
///     inf,                                            // e: inf
///     |acc, f| if f == id { acc } else { f },         // mapping: update if valid
///     |f, g| if f == id { g } else { f },             // composition: overwrite with new op f (f ∘ g)
///     id                                              // id: no update
/// );
///
/// assert_eq!(st.prod(0, 5), 1);
///
/// // Range Update: update [1, 4) to 0 -> [1, 0, 0, 0, 5]
/// st.apply_range(1, 4, 0);
///
/// assert_eq!(st.prod(0, 5), 0);
/// assert_eq!(st.prod(2, 4), 0);
/// assert_eq!(st.get(1), 0);
/// assert_eq!(st.get(4), 5);
/// ```
///
/// ## 2. Range Add Query (RAQ) + Range Sum Query (RSQ)
/// Range add, Range sum.
/// **Note**: When adding uniformly to a range sum, the "length of the range" is required when applying the operator.
/// Therefore, it is standard to include size information in the data type `T`, e.g., `(value, size)`.
///
/// ```
/// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
///
/// // (value, size)
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
///     |acc, f| S { val: acc.val + f * acc.size, size: acc.size }, // mapping: add (f * width)
///     |f, g| f + g,                                               // composition: sum of additions
///     0                                                           // id: 0
/// );
///
/// assert_eq!(st.prod(0, 5).val, 15);
///
/// // Range Add: add +1 to [0, 5) -> [2, 3, 4, 5, 6]
/// st.apply_range(0, 5, 1);
/// assert_eq!(st.prod(0, 5).val, 20); // 15 + 1*5
/// ```
///
/// ## 3. Range Affine Update + Range Sum Query
/// Range affine transformation (x <- bx + c), Range sum.
/// This also requires range width, so we include size in data type `T`.
///
/// ```
/// use atcoder_rust::template::lazy_segment_tree::LazySegmentTree;
///
/// #[derive(Clone, Copy, Debug)]
/// struct S { val: i64, size: i64 }
/// // Operator: (b, c) -> x * b + c
/// type F = (i64, i64);
///
/// let data_raw = vec![1, 2, 3, 4, 5];
/// let data: Vec<S> = data_raw.iter().map(|&x| S { val: x, size: 1 }).collect();
///
/// let mut st = LazySegmentTree::new(
///     &data,
///     |a, b| S { val: a.val + b.val, size: a.size + b.size },
///     S { val: 0, size: 0 },
///     |acc, f| S { val: acc.val * f.0 + f.1 * acc.size, size: acc.size }, // mapping
///     |f, g| (f.0 * g.0, f.0 * g.1 + f.1), // composition: f(g(x)) -> f.b(g.b*x + g.c) + f.c
///     (1, 0) // id: x * 1 + 0
/// );
///
/// // Apply x * 2 + 1 to [0, 5) -> [3, 5, 7, 9, 11]
/// st.apply_range(0, 5, (2, 1));
/// assert_eq!(st.prod(0, 5).val, 35);
/// ```
#[derive(Debug, Clone)]
pub struct LazySegmentTree<T, U, F, M, C> {
    n: usize,
    size: usize,
    log: usize,
    tree: Vec<T>,
    lazy: Vec<U>,
    op: F,
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
    /// Creates a new LazySegmentTree.
    ///
    /// C: (old, new) => composed
    pub fn new(
        data: &[T],
        op: F,
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
            tree[i] = op(tree[2 * i], tree[2 * i + 1]);
        }

        LazySegmentTree {
            n,
            size,
            log,
            tree,
            lazy,
            op,
            e,
            mapping,
            composition,
            id,
        }
    }

    /// Updates the element at index `p` to `x`.
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

    /// Gets the element at index `p`.
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

    /// Queries the aggregate value in range `[l, r)`.
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
                sml = (self.op)(sml, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(self.tree[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(sml, smr)
    }

    /// Gets the aggregate value of the entire range.
    pub fn all_prod(&self) -> T {
        self.tree[1]
    }

    /// Applies the operation `f` to the element at index `p`.
    pub fn apply(
        &mut self,
        mut p: usize,
        f: U,
    ) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.tree[p] = (self.mapping)(self.tree[p], f);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// Applies the operation `f` to all elements in range `[l, r)`.
    pub fn apply_range(
        &mut self,
        mut l: usize,
        mut r: usize,
        f: U,
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

        let l2 = l;
        let r2 = r;

        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, f);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }

        l = l2;
        r = r2;

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn update(
        &mut self,
        k: usize,
    ) {
        self.tree[k] = (self.op)(self.tree[2 * k], self.tree[2 * k + 1]);
    }

    fn all_apply(
        &mut self,
        k: usize,
        f: U,
    ) {
        self.tree[k] = (self.mapping)(self.tree[k], f);
        if k < self.size {
            self.lazy[k] = (self.composition)(f, self.lazy[k]);
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

fn add(
    left: usize,
    right: usize,
    c: char,
    lst: &mut LazySegmentTree<
        usize,
        usize,
        impl Fn(usize, usize) -> usize,
        impl Fn(usize, usize) -> usize,
        impl Fn(usize, usize) -> usize,
    >,
    m: &mut BTreeMap<usize, (usize, char)>,
) {
    lst.apply_range(left, right, right - left);
    m.insert(left, (right, c));
}

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
        q: usize,
        mut s: Chars,
    }

    s.push('*');

    let mut map: BTreeMap<usize, (usize, char)> = BTreeMap::new();
    let mut data = vec![0; n];

    let mut char_before ='*';
    let mut runlength = 0;
    let mut run_starts_at = 0;

    for i in 0..(n+1) {
        let c = s[i];
        if c == char_before {
            runlength += 1;
        }
        else {
            map.insert(run_starts_at, (run_starts_at + runlength, char_before));
            for i in run_starts_at..(run_starts_at + runlength) {
                data[i] = runlength;
            }
            runlength = 1;
            char_before = c;
            run_starts_at = i;
        }
    }

    let mut lst = LazySegmentTree::new(
        &data,
        |a, b| { max(a, b) },
        0 as usize,
        |acc, f| { if f == 0 { acc } else { f} },
        |f, g| { if f == 0 { g } else { f } },
        0 as usize,
    );

    for _ in 0..q {
        input! { qtype: usize, }
        if qtype == 1 {
            input! {
                idx: Usize1,
                x: char,
            }
            if s[idx] == x {
                continue;
            }
            s[idx] = x;
            let (&left, &(right, c)) = map.range(..=idx).next_back().unwrap();
            if c == x {
                continue;
            }
            // completely separable
            if left != idx && right - 1 != idx {
                map.remove(&left);
                add(left, idx, c, &mut lst, &mut map);
                add(idx, idx + 1, x, &mut lst, &mut map);
                add(idx + 1, right , c, &mut lst, &mut map);
            }
            else if left == idx && right - 1 == idx {
                let mut next_range_left = idx;
                let mut next_range_right = idx + 1;
                map.remove(&idx);
                if let Some((&prev_left, &(prev_right, prev_c))) = map.range(..idx).next_back() {
                    if prev_c == x {
                        next_range_left = prev_left;
                        map.remove(&prev_left);
                    }
                }
                if let Some((&next_left, &(next_right, next_c))) = map.range(idx..).next() {
                    if next_c == x {
                        next_range_right = next_right;
                        map.remove(&next_left);
                    }
                }
                add(next_range_left, next_range_right, x, &mut lst ,&mut map);
            }
            else if left == idx {
                map.remove(&left);
                add(idx + 1, right, c, &mut lst , &mut map);
                if let Some((&prev_left, &(prev_right, prev_c))) = map.range(..idx).next_back() {
                    if prev_c == x {
                        map.remove(&prev_left);
                        add(prev_left, idx + 1, x, &mut lst, &mut map);
                    }
                    else {
                        add(idx, idx + 1, x, &mut lst, &mut map);
                    }
                } else {
                    md!();
                    add(idx, idx + 1, x, &mut lst, &mut map);
                }
            }
            else if idx == right - 1 {
                map.remove(&left);
                add(left, idx, c, &mut lst, &mut map);
                if let Some((&next_left, &(next_right, next_c))) = map.range(idx..).next() {
                    if next_c == x {
                        map.remove(&next_left);
                        add(idx, next_right, x, &mut lst, &mut map);
                    }
                    else {
                        add(idx, idx + 1, x, &mut lst, &mut map);
                    }
                }
                else {
                    add(idx, idx + 1, x, &mut lst, &mut map);
                }
            } else {
                panic!();
            }

        } else {
            input! {
                l: Usize1,
                r: usize,
            }

            let mut alts = 0;
            let mut left_search = 0;
            let mut right_search = n;

            if let Some((left_max, (right_max, _))) = &map.range(..l).next_back() {
                alts = max(alts, *right_max - l);
                left_search = *right_max;
            }
            if let Some((left_max, (right_max, _))) = &map.range(..r).next_back() {
                alts = max(alts, r - **left_max);
                right_search = **left_max;
            }
            if left_search > right_search {
                wl!(r - l);
            } else {
                wl!(max(alts, lst.prod(left_search, right_search)));
            }
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

// Utility functions

// Utility functions
/// Returns valid neighbor coordinates within the grid (h x w).
/// Usage:
/// ```
/// for (nh, nw) in get_next_positions(h, w, hh, ww, &DIR) {
///     // process (nh, nw)
/// }
/// ```
fn get_next_positions(
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<(usize, usize)> {
    let mut next_positions = Vec::with_capacity(directions.len());

    for &(di, dj) in directions {
        let next_i = i.wrapping_add_signed(di);
        let next_j = j.wrapping_add_signed(dj);
        if next_i < h && next_j < w {
            next_positions.push((next_i, next_j));
        }
    }
    next_positions
}
