use std::collections::VecDeque;

// --- SNAP START ---

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
    pub fn new(base_char: char, char_range: usize) -> Self {
        Trie {
            nodes: vec![TrieNode::new(char_range)],
            base_char: base_char as u8,
            char_range,
            is_dfa_built: false,
        }
    }

    pub fn char_to_index(&self, c: char) -> usize {
        let idx = (c as u8).wrapping_sub(self.base_char) as usize;
        // assert!(idx < self.char_range, "Character out of range");
        idx
    }

    /// Inserts a string into the Trie and assigns an optional pattern ID.
    pub fn insert(&mut self, s: &str, pattern_id: usize) {
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
    pub fn next_state(&self, current_node: usize, c: char) -> usize {
        debug_assert!(self.is_dfa_built, "build_aho_corasick() must be called before next_state()");
        let c_idx = self.char_to_index(c);
        self.nodes[current_node].children[c_idx].unwrap_or(0)
    }

    /// Searches the text and returns a list of matched (end_index, pattern_id).
    pub fn search(&self, text: &str) -> Vec<(usize, usize)> {
        debug_assert!(self.is_dfa_built, "build_aho_corasick() must be called before searching");
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