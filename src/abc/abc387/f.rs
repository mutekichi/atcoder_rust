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

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
    input! {
        n: usize,
        m: usize,
        A: [Usize1; n],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        let a = i;
        let b = A[i];
        if a == b {
            continue;
        }
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut seen = vec![false; n];
    let mut ans = Mint998::new(1);

    let mut pow_table = vec![vec![Mint998::new(1); 2027]; 2027];
    for i in 0..2026 {
        for j in 0..2026 {
            pow_table[i][j + 1] = pow_table[i][j] * Mint998::new(i as i64);
        }
    }

    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut vec = vec![];
        let mut queue = VecDeque::new();
        queue.push_front(i);
        seen[i] = true;
        while let Some(v) = queue.pop_back() {
            vec.push(v);
            for &nv in &graph[v] {
                if !seen[nv] {
                    seen[nv] = true;
                    queue.push_back(nv);
                }
            }
        }
        let mut map = BTreeMap::new();
        for i in 0..vec.len() {
            map.insert(vec[i], i);
        }
        let mut scc = SccGraph::new(vec.len());
        for i in 0..vec.len() {
            scc.add_edge(i, map[&A[vec[i]]]);
        }
        let res = scc.scc();
        let n = res.groups.len();

        let mut dp = vec![vec![Mint998::new(0); m]; n];
        let mut vec = vec![];
        dfs(n - 1, &res, &mut vec);
        for v in vec {
            for i in 0..m {
                let mut val = Mint998::new(1);
                for &nv in &res.condensed_adj[v] {
                    val *= dp[nv][i];
                }
                dp[v][i] += val;
                if i > 0 {
                    let bef = dp[v][i - 1];
                    dp[v][i] += bef;
                }
            }
        }
        ans *= dp[n - 1][m - 1];
    }
    println!("{}", ans);
}

fn dfs(
    v: usize,
    res: &SccResult,
    vec: &mut Vec<usize>,
) {
    for &nv in &res.condensed_adj[v] {
        dfs(nv, res, vec);
    }
    vec.push(v);
}

// FOR TEMPLATE INJECTIONS

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Type alias for ModInt with modulus 998244353
pub type Mint998 = ModInt<998_244_353>;

/// Type alias for ModInt with modulus 1000000007
pub type Mint107 = ModInt<1_000_000_007>;

/// A struct for modular arithmetic.
///
/// Automatically handles modulo operations for addition, subtraction, multiplication, and division.
///
/// # Generics
/// - `M`: The modulus (e.g., 998244353). Must be a prime number for division to work correctly via Fermat's Little Theorem.
///
/// # Examples
///
/// ## 1. Basic Arithmetic
/// ```
/// use atcoder_rust::template::modint::Mint998;
///
/// let a = Mint998::new(10);
/// let b = Mint998::new(20);
///
/// assert_eq!((a + b).val(), 30);
/// assert_eq!((a - b).val(), 998244343); // 10 - 20 + MOD
/// assert_eq!((a * b).val(), 200);
/// assert_eq!(a.pow(3).val(), 1000);
/// ```
///
/// ## 2. Combination (nCr) Calculation
/// ```
/// use atcoder_rust::template::modint::Mint998;
///
/// fn combinations(n: usize, k: usize) -> Mint998 {
///     if k > n { return Mint998::new(0); }
///     
///     let mut num = Mint998::new(1);
///     let mut den = Mint998::new(1);
///     
///     for i in 0..k {
///         num *= (n - i) as i64;
///         den *= (i + 1) as i64;
///     }
///     
///     num / den
/// }
///
/// // 5C2 = 10
/// assert_eq!(combinations(5, 2).val(), 10);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<const M: u64> {
    val: u64,
}

impl<const M: u64> ModInt<M> {
    /// Creates a new `ModInt`.
    ///
    /// # Arguments
    /// - `x`: The initial value. Can be negative.
    pub fn new(x: i64) -> Self {
        let mut x = x % M as i64;
        if x < 0 {
            x += M as i64;
        }
        ModInt { val: x as u64 }
    }

    /// Returns the inner value (guaranteed to be in [0, M)).
    pub fn val(&self) -> u64 {
        self.val
    }

    /// Calculates base^exp % M.
    ///
    /// # Complexity
    /// - O(log exp)
    pub fn pow(
        &self,
        mut exp: u64,
    ) -> Self {
        let mut base = self.val;
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % M;
            }
            base = (base * base) % M;
            exp /= 2;
        }
        ModInt { val: res }
    }

    /// Calculates the modular inverse using Fermat's Little Theorem.
    ///
    /// # Note
    /// - Requires `M` to be prime.
    pub fn inv(&self) -> Self {
        self.pow(M - 2)
    }
}

// --- Trait Implementations ---

impl<const M: u64> fmt::Display for ModInt<M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u64> fmt::Debug for ModInt<M> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const M: u64> From<i64> for ModInt<M> {
    fn from(item: i64) -> Self {
        ModInt::new(item)
    }
}

impl<const M: u64> From<u64> for ModInt<M> {
    fn from(item: u64) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> From<usize> for ModInt<M> {
    fn from(item: usize) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> From<i32> for ModInt<M> {
    fn from(item: i32) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> Add for ModInt<M> {
    type Output = Self;
    fn add(
        self,
        other: Self,
    ) -> Self {
        let mut res = self.val + other.val;
        if res >= M {
            res -= M;
        }
        ModInt { val: res }
    }
}

impl<const M: u64> Add<i64> for ModInt<M> {
    type Output = Self;
    fn add(
        self,
        other: i64,
    ) -> Self {
        self + ModInt::new(other)
    }
}

impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self + other;
    }
}

impl<const M: u64> AddAssign<i64> for ModInt<M> {
    fn add_assign(
        &mut self,
        other: i64,
    ) {
        *self = *self + ModInt::new(other);
    }
}

impl<const M: u64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(
        self,
        other: Self,
    ) -> Self {
        let mut res = self.val;
        if res < other.val {
            res += M;
        }
        res -= other.val;
        ModInt { val: res }
    }
}

impl<const M: u64> Sub<i64> for ModInt<M> {
    type Output = Self;
    fn sub(
        self,
        other: i64,
    ) -> Self {
        self - ModInt::new(other)
    }
}

impl<const M: u64> SubAssign for ModInt<M> {
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self - other;
    }
}

impl<const M: u64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(
        self,
        other: Self,
    ) -> Self {
        ModInt {
            val: (self.val * other.val) % M,
        }
    }
}

impl<const M: u64> Mul<i64> for ModInt<M> {
    type Output = Self;
    fn mul(
        self,
        other: i64,
    ) -> Self {
        self * ModInt::new(other)
    }
}

impl<const M: u64> MulAssign for ModInt<M> {
    fn mul_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self * other;
    }
}

impl<const M: u64> Div for ModInt<M> {
    type Output = Self;
    fn div(
        self,
        other: Self,
    ) -> Self {
        self * other.inv()
    }
}

impl<const M: u64> DivAssign for ModInt<M> {
    fn div_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self / other;
    }
}

// Enable parsing from input (using proconio)
impl<const M: u64> proconio::source::Readable for ModInt<M> {
    type Output = Self;
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
        let val = i64::read(source);
        ModInt::new(val)
    }
}

impl<const M: u64> Default for ModInt<M> {
    fn default() -> Self {
        ModInt::new(0)
    }
}

pub struct SccGraph {
    n: usize,
    pub edges: Vec<(usize, usize)>,
}

pub struct SccResult {
    /// component ID for each vertex (topological order)
    pub ids: Vec<usize>,
    /// vertices belonging to each component
    pub groups: Vec<Vec<usize>>,
    /// adjacency list of the condensed DAG
    pub condensed_adj: Vec<Vec<usize>>,
}

impl SccGraph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
    ) {
        self.edges.push((from, to));
    }

    pub fn scc(&self) -> SccResult {
        let mut start = vec![0; self.n + 1];
        for &(u, _) in &self.edges {
            start[u + 1] += 1;
        }
        for i in 0..self.n {
            start[i + 1] += start[i];
        }

        let mut counter = start.clone();
        let mut elist = vec![0; self.edges.len()];
        for &(u, v) in &self.edges {
            elist[counter[u]] = v;
            counter[u] += 1;
        }

        let mut visited = Vec::with_capacity(self.n);
        let mut ord = vec![!0; self.n];
        let mut low = vec![!0; self.n];
        let mut now_ord = 0;
        let mut ids = vec![0; self.n];
        let mut group_num = 0;
        let mut stack = Vec::new();

        for i in 0..self.n {
            if ord[i] == !0 {
                self.dfs(
                    i,
                    &mut now_ord,
                    &start,
                    &elist,
                    &mut visited,
                    &mut ord,
                    &mut low,
                    &mut ids,
                    &mut group_num,
                    &mut stack,
                );
            }
        }

        for x in &mut ids {
            *x = group_num - 1 - *x;
        }

        let mut groups = vec![Vec::new(); group_num];
        for i in 0..self.n {
            groups[ids[i]].push(i);
        }

        // build condensed graph
        let mut condensed_adj = vec![Vec::new(); group_num];
        for &(u, v) in &self.edges {
            let id_u = ids[u];
            let id_v = ids[v];
            if id_u != id_v {
                condensed_adj[id_v].push(id_u);
            }
        }
        for v in &mut condensed_adj {
            v.sort_unstable();
            v.dedup();
        }

        SccResult {
            ids,
            groups,
            condensed_adj,
        }
    }

    fn dfs(
        &self,
        v: usize,
        now_ord: &mut usize,
        start: &[usize],
        elist: &[usize],
        visited: &mut Vec<usize>,
        ord: &mut [usize],
        low: &mut [usize],
        ids: &mut [usize],
        group_num: &mut usize,
        stack: &mut Vec<(usize, usize)>,
    ) {
        stack.push((v, start[v]));
        ord[v] = *now_ord;
        low[v] = *now_ord;
        *now_ord += 1;
        visited.push(v);

        while let Some((curr, next_idx)) = stack.pop() {
            if next_idx < start[curr + 1] {
                let to = elist[next_idx];
                stack.push((curr, next_idx + 1));
                if ord[to] == !0 {
                    ord[to] = *now_ord;
                    low[to] = *now_ord;
                    *now_ord += 1;
                    visited.push(to);
                    stack.push((to, start[to]));
                } else {
                    low[curr] = low[curr].min(ord[to]);
                }
            } else {
                if low[curr] == ord[curr] {
                    loop {
                        let u = visited.pop().unwrap();
                        ord[u] = self.n;
                        ids[u] = *group_num;
                        if u == curr {
                            break;
                        }
                    }
                    *group_num += 1;
                }
                if let Some((prev, _)) = stack.last() {
                    low[*prev] = low[*prev].min(low[curr]);
                }
            }
        }
    }
}
