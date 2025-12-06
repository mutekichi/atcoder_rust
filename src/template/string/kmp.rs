#![allow(dead_code)]

// --- SNAP START ---

/// Knuth-Morris-Pratt (KMP) Algorithm
///
/// Efficient string matching algorithm using a failure function (pi table).
///
/// # Complexity
/// - Precomputation (build): O(M), where M is the pattern length.
/// - Search: O(N), where N is the text length.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::string::kmp::KMP;
///
/// let text: Vec<char> = "abracadabra".chars().collect();
/// let pattern: Vec<char> = "abra".chars().collect();
///
/// // 1. Build failure table
/// let pi = KMP::build(&pattern);
///
/// // 2. Search pattern in text
/// let matches = KMP::search(&text, &pattern);
/// assert_eq!(matches, vec![0, 7]); // Indices where pattern starts
///
/// // 3. Analyze Periodicity using pi table
/// // For string S of length L, if L % (L - pi[L-1]) == 0 and pi[L-1] > 0,
/// // then the minimum period length is (L - pi[L-1]).
/// let s: Vec<char> = "abcabcabc".chars().collect();
/// let pi_s = KMP::build(&s);
/// let len = s.len();
/// let period = len - pi_s[len - 1];
/// assert_eq!(period, 3); // "abc" is the period
/// ```
pub struct KMP;

impl KMP {
    /// Constructs the failure function (pi table).
    ///
    /// `pi[i]` is the length of the longest proper prefix of `p[0..=i]`
    /// that is also a suffix of `p[0..=i]`.
    pub fn build<T: PartialEq>(p: &[T]) -> Vec<usize> {
        let m = p.len();
        if m == 0 {
            return vec![];
        }
        let mut pi = vec![0; m];
        let mut j = 0;
        for i in 1..m {
            while j > 0 && p[i] != p[j] {
                j = pi[j - 1];
            }
            if p[i] == p[j] {
                j += 1;
            }
            pi[i] = j;
        }
        pi
    }

    /// Searches for all occurrences of `pattern` in `text`.
    /// Returns a vector of starting indices (0-based).
    pub fn search<T: PartialEq>(
        text: &[T],
        pattern: &[T],
    ) -> Vec<usize> {
        let mut matches = vec![];
        if pattern.is_empty() {
            return matches;
        }

        let pi = KMP::build(pattern);
        let mut j = 0; // index for pattern

        for (i, c) in text.iter().enumerate() {
            while j > 0 && *c != pattern[j] {
                j = pi[j - 1];
            }
            if *c == pattern[j] {
                j += 1;
            }
            if j == pattern.len() {
                matches.push(i + 1 - j);
                j = pi[j - 1]; // Prepare for next match
            }
        }
        matches
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_build() {
        let p: Vec<char> = "aabaa".chars().collect();
        let pi = KMP::build(&p);
        // a: 0
        // aa: 1 ("a")
        // aab: 0
        // aaba: 1 ("a")
        // aabaa: 2 ("aa")
        assert_eq!(pi, vec![0, 1, 0, 1, 2]);
    }

    #[test]
    fn test_kmp_search() {
        let t: Vec<char> = "abababa".chars().collect();
        let p: Vec<char> = "aba".chars().collect();
        let matches = KMP::search(&t, &p);
        assert_eq!(matches, vec![0, 2, 4]);
    }
}
