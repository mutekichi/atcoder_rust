#![allow(dead_code)]

// --- SNAP START ---

/// Z-Algorithm
///
/// Calculates the length of the Longest Common Prefix (LCP) between the string `S`
/// and each of its suffixes `S[i..]`.
///
/// Time Complexity: O(N)
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::string::z_algorithm::ZAlgorithm;
///
/// let s: Vec<char> = "aaabaaaab".chars().collect();
/// let z = ZAlgorithm::build(&s);
///
/// // z[0] = 9 (LCP of "aaabaaaab" and "aaabaaaab")
/// // z[1] = 2 (LCP of "aaabaaaab" and "aabaaaab")
/// // z[2] = 1 (LCP of "aaabaaaab" and "abaaaab")
/// // z[3] = 0
/// // z[4] = 4 ("aaab")
///
/// assert_eq!(z, vec![9, 2, 1, 0, 4, 2, 1, 0, 1]);
/// ```
pub struct ZAlgorithm;

impl ZAlgorithm {
    /// Constructs the Z-array.
    ///
    /// # Arguments
    /// - `s`: Input slice (e.g., `&[char]`, `&[u8]`, `&[i32]`).
    ///
    /// # Returns
    /// A `Vec<usize>` of length `s.len()`, where the `i`-th element is the length of
    /// the LCP between `s` and `s[i..]`.
    pub fn build<T: PartialEq>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }
        let mut z = vec![0; n];
        z[0] = n;
        let mut i = 1;
        let mut j = 0;
        while i < n {
            while i + j < n && s[j] == s[i + j] {
                j += 1;
            }
            z[i] = j;
            if j == 0 {
                i += 1;
                continue;
            }
            let mut k = 1;
            while i + k < n && k + z[k] < j {
                z[i + k] = z[k];
                k += 1;
            }
            i += k;
            j -= k;
        }
        z
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_algorithm() {
        // "abcabc"
        // abcabc (6)
        // bcabc  (0)
        // cabc   (0)
        // abc    (3)
        // bc     (0)
        // c      (0)
        let s: Vec<char> = "abcabc".chars().collect();
        let z = ZAlgorithm::build(&s);
        assert_eq!(z, vec![6, 0, 0, 3, 0, 0]);

        // "aaaaa"
        let s2: Vec<char> = "aaaaa".chars().collect();
        let z2 = ZAlgorithm::build(&s2);
        assert_eq!(z2, vec![5, 4, 3, 2, 1]);
    }
}
