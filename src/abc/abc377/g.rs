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
    let mut trie = Trie::new('a', 26);
    input! {
        n: usize,
        S_vec: [String; n],
    }
    trie.insert(&S_vec[0]);
    println!("{}", S_vec[0].len());
    for S in S_vec.iter().skip(1) {
        let res = trie.insert(&S);
        md!(S, res.0, res.1);
        println!("{}", min(S.len() - res.0 + res.1, S.len()));
    }
}

// FOR TEMPLATE INJECTIONS

/// Trie (Prefix Tree)
///
/// A tree data structure used for efficiently storing and retrieving keys in a dataset of strings.
///
/// # Features
/// - Insert string O(L)
/// - Search string O(L)
/// - Prefix counting
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::string::trie::Trie;
///
/// // Create a Trie for lowercase English letters ('a' to 'z')
/// let mut trie = Trie::new('a', 26);
///
/// trie.insert("apple");
/// trie.insert("app");
/// trie.insert("apricot");
///
/// assert_eq!(trie.count("app"), 1);       // Exact match "app"
/// assert_eq!(trie.count_prefix("ap"), 3); // "apple", "app", "apricot"
/// assert_eq!(trie.count("banana"), 0);
/// ```
#[derive(Debug, Clone)]
pub struct TrieNode {
    /// Indices of children nodes. `None` if the edge does not exist.
    pub children: Vec<Option<usize>>,
    /// Number of words passing through this node (prefix count).
    pub common_count: usize,
    /// Number of words ending at this node (exact match count).
    pub accept_count: usize,
    pub depth: usize,
}

impl TrieNode {
    fn new(
        char_range: usize,
        depth: usize,
    ) -> Self {
        TrieNode {
            children: vec![None; char_range],
            common_count: 0,
            accept_count: 0,
            depth: depth,
        }
    }
}

pub struct Trie {
    pub nodes: Vec<TrieNode>,
    base_char: u8,
    char_range: usize,
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
            nodes: vec![TrieNode::new(char_range, INF_USIZE)],
            base_char: base_char as u8,
            char_range,
        }
    }

    fn char_to_index(
        &self,
        c: char,
    ) -> usize {
        let idx = (c as u8).wrapping_sub(self.base_char) as usize;
        // If validation is needed, uncomment below:
        // assert!(idx < self.char_range, "Character out of range");
        idx
    }

    /// Inserts a string into the Trie.
    ///
    /// # Complexity
    /// - O(L) where L is the length of the string.
    pub fn insert(
        &mut self,
        s: &str,
    ) -> (usize, usize) {
        let mut node_idx = 0;
        let mut len = 0;
        let mut ans = (0, 0);
        let mut ok = true;
        self.nodes[node_idx].depth = min(self.nodes[node_idx].depth, s.len());
        let mut depth = s.len() - 1;
        for c in s.chars() {
            self.nodes[node_idx].common_count += 1;
            let c_idx = self.char_to_index(c);
            if self.nodes[node_idx].children[c_idx].is_none() {
                if ok {
                    ans = (len, self.nodes[node_idx].depth);
                    ok = false;
                }
                self.nodes.push(TrieNode::new(self.char_range, depth));
                let new_idx = self.nodes.len() - 1;
                self.nodes[node_idx].children[c_idx] = Some(new_idx);
            } else {
                len += 1;
            }
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
            if depth > 0 {
                depth -= 1;
            }
        }
        if ok {
            ans = (len, self.nodes[node_idx].depth + 1);
        }
        self.nodes[node_idx].common_count += 1;
        self.nodes[node_idx].accept_count += 1;

        node_idx = 0;
        depth = s.len() - 1;
        for c in s.chars() {
            let c_idx = self.char_to_index(c);
            self.nodes[node_idx].depth = min(self.nodes[node_idx].depth, depth);
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
            if depth > 0 {
                depth -= 1;
            }
        }

        ans
    }

    /// Returns the number of times the exact string `s` has been inserted.
    ///
    /// # Complexity
    /// - O(L)
    pub fn count(
        &self,
        s: &str,
    ) -> usize {
        let mut node_idx = 0;
        for c in s.chars() {
            let c_idx = self.char_to_index(c);
            match self.nodes[node_idx].children[c_idx] {
                Some(next_idx) => node_idx = next_idx,
                None => return 0,
            }
        }
        self.nodes[node_idx].accept_count
    }

    /// Returns the number of inserted strings that start with `prefix`.
    ///
    /// # Complexity
    /// - O(L)
    pub fn count_prefix(
        &self,
        prefix: &str,
    ) -> usize {
        let mut node_idx = 0;
        for c in prefix.chars() {
            let c_idx = self.char_to_index(c);
            match self.nodes[node_idx].children[c_idx] {
                Some(next_idx) => node_idx = next_idx,
                None => return 0,
            }
        }
        self.nodes[node_idx].common_count
    }
}

// END TEMPLATE INJECTIONS
