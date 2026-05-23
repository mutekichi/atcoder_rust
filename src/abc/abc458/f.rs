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

#[allow(unused_variables)]
fn main() {
    input! {
        n: i64,
        k: usize,
        S: [String; k]
    }

    let mut trie = Trie::new('a', 26);

    for i in 0..k {
        trie.insert(&S[i], i);
    }
    trie.build_aho_corasick();

    let dim = trie.nodes.len();
    md!(dim);
    let mut mat: Matrix<Mint998> = Matrix::new(dim, dim);
    for i in 0..dim {
        for c in 'a'..='z' {
            let next_node = trie.next_state(i, c);
            if trie.nodes[next_node].pattern_ids.len() == 0 {
                // mat.set(i, next_node, mat.get(i, next_node) + 1);
                mat.set(next_node, i, mat.get(next_node, i) + 1);
            }
        }
    }
    let mut vec: Matrix<Mint998> = Matrix::new(dim, 1);
    vec.set(0, 0, Mint998::new(1));
    for i in 0..60 {
        if (n >> i) & 1 == 1 {
            vec = mat.matmul(&vec);
        }
        mat = mat.matmul(&mat);
    }
    let mut ans = Mint998::new(0);
    for i in 0..dim {
        ans += vec.get(i, 0);
    }
    println!("{}", ans);
}

// FOR TEMPLATE INJECTIONS

use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Mint998 = ModInt<998_244_353>;
pub type Mint107 = ModInt<1_000_000_007>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<const M: u64> {
    val: u64,
}

impl<const M: u64> ModInt<M> {
    pub fn new(x: i64) -> Self {
        let mut x = x % M as i64;
        if x < 0 {
            x += M as i64;
        }
        ModInt { val: x as u64 }
    }

    pub fn val(&self) -> u64 {
        self.val
    }

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

    pub fn inv(&self) -> Self {
        self.pow(M - 2)
    }
}

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

impl<const M: u64> From<u32> for ModInt<M> {
    fn from(item: u32) -> Self {
        ModInt::new(item as i64)
    }
}

impl<const M: u64> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        ModInt::new(-(self.val as i64))
    }
}

impl<const M: u64> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(0), |a, b| a + b)
    }
}

impl<const M: u64> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(1), |a, b| a * b)
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

impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        *self = *self + other;
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

macro_rules! impl_modint_ops {
    ($($t:ty),*) => {
        $(
            impl<const M: u64> Add<$t> for ModInt<M> {
                type Output = Self;
                fn add(self, other: $t) -> Self { self + ModInt::from(other) }
            }
            impl<const M: u64> Sub<$t> for ModInt<M> {
                type Output = Self;
                fn sub(self, other: $t) -> Self { self - ModInt::from(other) }
            }
            impl<const M: u64> Mul<$t> for ModInt<M> {
                type Output = Self;
                fn mul(self, other: $t) -> Self { self * ModInt::from(other) }
            }
            impl<const M: u64> Div<$t> for ModInt<M> {
                type Output = Self;
                fn div(self, other: $t) -> Self { self / ModInt::from(other) }
            }
            impl<const M: u64> AddAssign<$t> for ModInt<M> {
                fn add_assign(&mut self, other: $t) { *self = *self + other; }
            }
            impl<const M: u64> SubAssign<$t> for ModInt<M> {
                fn sub_assign(&mut self, other: $t) { *self = *self - other; }
            }
            impl<const M: u64> MulAssign<$t> for ModInt<M> {
                fn mul_assign(&mut self, other: $t) { *self = *self * other; }
            }
            impl<const M: u64> DivAssign<$t> for ModInt<M> {
                fn div_assign(&mut self, other: $t) { *self = *self / other; }
            }
            impl<const M: u64> Add<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn add(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) + other }
            }
            impl<const M: u64> Sub<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn sub(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) - other }
            }
            impl<const M: u64> Mul<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn mul(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) * other }
            }
            impl<const M: u64> Div<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn div(self, other: ModInt<M>) -> ModInt<M> { ModInt::from(self) / other }
            }
        )*
    };
}

impl_modint_ops!(i32, i64, u32, u64, usize);

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

/// Matrix operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    pub mat: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix<T>
where
    T: Clone
        + Copy
        + Default
        + From<i32>
        + Add<Output = T>
        + Mul<Output = T>
        + AddAssign
        + PartialEq,
{
    /// Constructs a new `rows x cols` matrix filled with zeros (T::default()).
    pub fn new(
        rows: usize,
        cols: usize,
    ) -> Self {
        Matrix {
            mat: vec![vec![T::default(); cols]; rows],
            rows,
            cols,
        }
    }

    /// Constructs an identity matrix of size `n`.
    pub fn identity(n: usize) -> Self {
        let mut res = Matrix::new(n, n);
        for i in 0..n {
            res.mat[i][i] = T::from(1);
        }
        res
    }

    /// Performs matrix multiplication.
    /// Renamed from `mul` to `matmul` to avoid conflict with `std::ops::Mul`.
    ///
    /// # Complexity
    /// O(rows * cols * other.cols)
    pub fn matmul(
        &self,
        other: &Matrix<T>,
    ) -> Matrix<T> {
        assert_eq!(
            self.cols, other.rows,
            "Dimension mismatch for matrix multiplication"
        );
        let mut res = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                if self.mat[i][k] == T::default() {
                    continue;
                }
                for j in 0..other.cols {
                    let val = self.mat[i][k] * other.mat[k][j];
                    res.mat[i][j] += val;
                }
            }
        }
        res
    }

    /// Performs matrix exponentiation (A^exp).
    ///
    /// # Complexity
    /// O(n^3 log exp)
    pub fn pow(
        &self,
        mut exp: u64,
    ) -> Matrix<T> {
        assert_eq!(
            self.rows, self.cols,
            "Matrix must be square for exponentiation"
        );
        let mut res = Matrix::identity(self.rows);
        let mut base = self.clone();
        while exp > 0 {
            if exp % 2 == 1 {
                // Modified to use matmul
                res = res.matmul(&base);
            }
            // Modified to use matmul
            base = base.matmul(&base);
            exp /= 2;
        }
        res
    }

    /// Returns the element at (row, col).
    pub fn get(
        &self,
        row: usize,
        col: usize,
    ) -> T {
        self.mat[row][col]
    }

    /// Sets the element at (row, col) to `val`.
    pub fn set(
        &mut self,
        row: usize,
        col: usize,
        val: T,
    ) {
        self.mat[row][col] = val;
    }
}

// Enable `A * B` syntax
impl<T> Mul for Matrix<T>
where
    T: Clone
        + Copy
        + Default
        + From<i32>
        + Add<Output = T>
        + Mul<Output = T>
        + AddAssign
        + PartialEq,
{
    type Output = Self;
    fn mul(
        self,
        rhs: Self,
    ) -> Self {
        // Use matmul to avoid infinite recursion and borrow errors
        self.matmul(&rhs)
    }
}

// Display implementation for debugging
impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for row in &self.mat {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Trie (Prefix Tree) Extended with Aho-Corasick Automaton capabilities
///
/// A tree/DFA data structure used for efficiently storing and retrieving keys,
/// and performing multi-pattern string matching.
#[derive(Debug, Clone)]
pub struct TrieNode {
    /// Indices of children nodes. `None` if the edge does not exist (before DFA conversion).
    /// After calling `build_aho_corasick()`, this will act as a complete DFA transition table,
    /// where `None` is replaced by the corresponding failure transition.
    pub children: Vec<Option<usize>>,
    /// Number of words passing through this node (prefix count).
    pub common_count: usize,
    /// Number of words ending at this node (exact match count).
    /// After `build_aho_corasick()`, this accumulates matches from failure links.
    pub accept_count: usize,
    /// The failure link destination node index.
    pub fail: usize,
    /// (Optional) Can store a list of specific pattern IDs that end here.
    pub pattern_ids: Vec<usize>,
}

impl TrieNode {
    fn new(char_range: usize) -> Self {
        TrieNode {
            children: vec![None; char_range],
            common_count: 0,
            accept_count: 0,
            fail: 0,
            pattern_ids: Vec::new(),
        }
    }
}

pub struct Trie {
    pub nodes: Vec<TrieNode>,
    base_char: u8,
    char_range: usize,
    /// Indicates if `build_aho_corasick` has been called and the Trie is now a DFA.
    pub is_dfa_built: bool,
}

impl Trie {
    /// Creates a new Trie.
    ///
    /// # Arguments
    /// - `base_char`: The starting character for indexing (e.g., 'a').
    /// - `char_range`: The number of possible characters (e.g., 26 for 'a'-'z').
    pub fn new(
        base_char: char,
        char_range: usize,
    ) -> Self {
        Trie {
            nodes: vec![TrieNode::new(char_range)],
            base_char: base_char as u8,
            char_range,
            is_dfa_built: false,
        }
    }

    pub fn char_to_index(
        &self,
        c: char,
    ) -> usize {
        let idx = (c as u8).wrapping_sub(self.base_char) as usize;
        // assert!(idx < self.char_range, "Character out of range");
        idx
    }

    /// Inserts a string into the Trie and assigns an optional pattern ID.
    pub fn insert(
        &mut self,
        s: &str,
        pattern_id: usize,
    ) {
        // Modifying the Trie after building DFA invalidates the automaton.
        // In a strict environment, you might want to panic or reset here.
        self.is_dfa_built = false;

        let mut node_idx = 0;
        for c in s.chars() {
            self.nodes[node_idx].common_count += 1;
            let c_idx = self.char_to_index(c);
            if self.nodes[node_idx].children[c_idx].is_none() {
                self.nodes.push(TrieNode::new(self.char_range));
                let new_idx = self.nodes.len() - 1;
                self.nodes[node_idx].children[c_idx] = Some(new_idx);
            }
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
        }
        self.nodes[node_idx].common_count += 1;
        self.nodes[node_idx].accept_count += 1;
        self.nodes[node_idx].pattern_ids.push(pattern_id);
    }

    /// Builds the failure links and converts the Trie into a Deterministic Finite Automaton (DFA).
    /// This allows O(1) state transitions using the `next_state` method.
    pub fn build_aho_corasick(&mut self) {
        if self.is_dfa_built {
            return;
        }

        let mut queue = VecDeque::new();

        // Initialize depth 1
        for c_idx in 0..self.char_range {
            if let Some(child_idx) = self.nodes[0].children[c_idx] {
                self.nodes[child_idx].fail = 0;
                queue.push_back(child_idx);
            } else {
                // Point missing root transitions to root (DFA optimization)
                self.nodes[0].children[c_idx] = Some(0);
            }
        }

        // BFS to propagate failure links and accept states
        while let Some(u) = queue.pop_front() {
            for c_idx in 0..self.char_range {
                if let Some(v) = self.nodes[u].children[c_idx] {
                    // Find the fail target for v
                    let fail_u = self.nodes[u].fail;
                    let fail_v = self.nodes[fail_u].children[c_idx].unwrap();

                    self.nodes[v].fail = fail_v;

                    // Accumulate accept count
                    let fail_v_accept = self.nodes[fail_v].accept_count;
                    self.nodes[v].accept_count += fail_v_accept;

                    // Accumulate pattern IDs
                    let mut extra_patterns = self.nodes[fail_v].pattern_ids.clone();
                    self.nodes[v].pattern_ids.append(&mut extra_patterns);

                    queue.push_back(v);
                } else {
                    // Fill missing transitions with fail transitions (DFA optimization)
                    let fail_u = self.nodes[u].fail;
                    self.nodes[u].children[c_idx] = self.nodes[fail_u].children[c_idx];
                }
            }
        }

        self.is_dfa_built = true;
    }

    /// Returns the next state (node index) given a current state and a character.
    /// Requires `build_aho_corasick()` to have been called.
    ///
    /// Useful for Dynamic Programming on Automata in competitive programming.
    #[inline]
    pub fn next_state(
        &self,
        current_node: usize,
        c: char,
    ) -> usize {
        debug_assert!(
            self.is_dfa_built,
            "build_aho_corasick() must be called before next_state()"
        );
        let c_idx = self.char_to_index(c);
        self.nodes[current_node].children[c_idx].unwrap_or(0)
    }

    /// Searches the text and returns a list of matched (end_index, pattern_id).
    pub fn search(
        &self,
        text: &str,
    ) -> Vec<(usize, usize)> {
        debug_assert!(
            self.is_dfa_built,
            "build_aho_corasick() must be called before searching"
        );
        let mut results = Vec::new();
        let mut node_idx = 0;

        for (i, c) in text.chars().enumerate() {
            node_idx = self.next_state(node_idx, c);

            for &pattern_id in &self.nodes[node_idx].pattern_ids {
                results.push((i, pattern_id));
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick_dfa() {
        let mut trie = Trie::new('a', 26);
        trie.insert("he", 0);
        trie.insert("she", 1);
        trie.insert("his", 2);
        trie.insert("hers", 3);

        trie.build_aho_corasick();

        // 1. Test DP state transition
        let mut state = 0; // Root
        state = trie.next_state(state, 's');
        state = trie.next_state(state, 'h');
        state = trie.next_state(state, 'e'); // "she" state

        // At "she", accept count should be 2 ("she" and "he")
        assert_eq!(trie.nodes[state].accept_count, 2);
        assert!(trie.nodes[state].pattern_ids.contains(&0)); // "he"
        assert!(trie.nodes[state].pattern_ids.contains(&1)); // "she"

        // Transition from "she" with 'r' should fail back to "he" and then go to "her" state
        state = trie.next_state(state, 'r');
        state = trie.next_state(state, 's'); // "hers" state
        assert_eq!(trie.nodes[state].accept_count, 1);
        assert!(trie.nodes[state].pattern_ids.contains(&3)); // "hers"

        // 2. Test full search
        let text = "ushers";
        let results = trie.search(text);

        // Expected matches in "ushers":
        // index 3: "she" (id 1), "he" (id 0) -> note 'e' is at index 3
        // index 5: "hers" (id 3) -> note 's' is at index 5
        let matched_ids: Vec<usize> = results.iter().map(|&(_, id)| id).collect();
        assert!(matched_ids.contains(&0));
        assert!(matched_ids.contains(&1));
        assert!(matched_ids.contains(&3));
    }
}

// END TEMPLATE INJECTIONS
