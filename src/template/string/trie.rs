#![allow(dead_code)]

// --- SNAP START ---

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
}

impl TrieNode {
    fn new(char_range: usize) -> Self {
        TrieNode {
            children: vec![None; char_range],
            common_count: 0,
            accept_count: 0,
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
            nodes: vec![TrieNode::new(char_range)],
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
    ) {
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

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new('a', 26);

        trie.insert("algo");
        trie.insert("algorithm");
        trie.insert("ant");

        assert_eq!(trie.count("algo"), 1);
        assert_eq!(trie.count("algorithm"), 1);
        assert_eq!(trie.count("alg"), 0);

        assert_eq!(trie.count_prefix("a"), 3);
        assert_eq!(trie.count_prefix("al"), 2);
        assert_eq!(trie.count_prefix("algo"), 2); // "algo", "algorithm"
        assert_eq!(trie.count_prefix("z"), 0);
    }
}
