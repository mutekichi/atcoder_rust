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
    pub terminate: bool,
}

impl TrieNode {
    fn new(char_range: usize) -> Self {
        TrieNode {
            children: vec![None; char_range],
            common_count: 0,
            accept_count: 0,
            terminate: false,
        }
    }
}

pub struct Trie {
    pub nodes: Vec<TrieNode>,
    base_char: u8,
    char_range: usize,
    valid_count: usize,
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
            valid_count: 0,
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
    ) {
        let mut node_idx = 0;
        let mut terminate = false;
        for c in s.chars() {
            self.nodes[node_idx].common_count += 1;
            let c_idx = self.char_to_index(c);
            if self.nodes[node_idx].children[c_idx].is_none() {
                self.nodes.push(TrieNode::new(self.char_range));
                let new_idx = self.nodes.len() - 1;
                self.nodes[node_idx].children[c_idx] = Some(new_idx);
            }
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
            if self.nodes[node_idx].terminate {
                md!("push terminated!", &s, c);
                terminate = true;
                break;
            }
        }
        if terminate {
            node_idx = 0;
            md!("back tracing..");
            for c in s.chars() {
                md!(c);
                md!(self.nodes[node_idx].terminate);
                self.nodes[node_idx].common_count -= 1;
                md!(self.nodes[node_idx].common_count);
                let c_idx = self.char_to_index(c);
                if self.nodes[node_idx].children[c_idx].is_none() {
                    self.nodes.push(TrieNode::new(self.char_range));
                    let new_idx = self.nodes.len() - 1;
                    self.nodes[node_idx].children[c_idx] = Some(new_idx);
                }
                node_idx = self.nodes[node_idx].children[c_idx].unwrap();
                if self.nodes[node_idx].terminate {
                    return;
                }
            }
        }
        else {
            self.nodes[node_idx].common_count += 1;
            self.nodes[node_idx].accept_count += 1;
            self.valid_count += 1;
        }
    }

    pub fn term(
        &mut self,
        s: &str,
    ) {
        let mut node_idx = 0;
        for c in s.chars() {
            let c_idx = self.char_to_index(c);
            if self.nodes[node_idx].terminate {
                return;
            }
            if self.nodes[node_idx].children[c_idx].is_none() {
                self.nodes.push(TrieNode::new(self.char_range));
                let new_idx = self.nodes.len() - 1;
                self.nodes[node_idx].children[c_idx] = Some(new_idx);
            }
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
        }
        self.nodes[node_idx].terminate = true;
        md!(node_idx, "terminate");
        let common_count = self.nodes[node_idx].common_count;
        md!(common_count);
        self.valid_count -= common_count;
        node_idx = 0;
        for c in s.chars() {
            let c_idx = self.char_to_index(c);
            self.nodes[node_idx].common_count -= common_count;
            md!(self.nodes[node_idx].common_count);
            node_idx = self.nodes[node_idx].children[c_idx].unwrap();
        }
        self.nodes[node_idx].common_count -= common_count;
        
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
        q: usize,
        TS: [(usize, String); q],
    }
    let mut trie = Trie::new('a', 26);
    for (t, s) in TS {
        if t == 1  {
            trie.term(&s);
        }
        else {
            trie.insert(&s);
        }
        wl!(trie.valid_count);
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
