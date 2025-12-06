#![allow(dead_code)]

// --- SNAP START ---

/// Manacher's Algorithm
///
/// Enumerates the radius of the longest palindrome centered at each position in O(N).
/// To handle even-length palindromes, dummy characters are inserted between every character.
///
/// # Complexity
/// - O(N)
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::string::manacher::Manacher;
///
/// let s: Vec<char> = "abaaab".chars().collect();
/// // String with dummy: # a # b # a # a # a # b #
/// let rad = Manacher::build(&s, '#');
///
/// // rad[i] is the radius of the palindrome centered at i in the transformed string.
/// // The length of the palindrome in the original string corresponds to `rad[i] - 1`.
///
/// // Center 'b' (index 1 in original, index 3 in transformed: "#a#b#")
/// // rad[3] = 2 ("#b#") -> length 1 ("b")
///
/// // Center 'a' (index 2 in original, index 5 in transformed: "#a#b#a#a#a#")
/// // rad[5] = 4 ("#a#b#a#") -> length 3 ("aba")
///
/// // Center between 'a' and 'a' (index 7 in transformed: "#...#a#a#...#")
/// // rad[7] = 2 ("#a#a#") -> length 1? No, logic is (rad[i]-1).
/// // Actually, for "aa", the center is the dummy between them.
/// // Transformed: ... a # a ...
/// // The palindrome is "a#a". Radius is 2. Length is 2 - 1 = 1. Wait, length of "aa" is 2.
/// // Let's check the property:
/// // The value `rad[i]` means the palindrome in T extends from `i - rad[i] + 1` to `i + rad[i] - 1`.
/// // The length of the palindrome in the ORIGINAL string is simply `rad[i] - 1`.
///
/// assert_eq!(rad.iter().map(|&x| x - 1).max(), Some(3)); // Max palindrome length is 3 ("aba" or "aaa")
/// ```
pub struct Manacher;

impl Manacher {
    /// Constructs the palindrome radius array.
    ///
    /// The input slice `s` is transformed by inserting `dummy` between characters and at both ends.
    /// E.g., `['a', 'b']` with dummy `#` becomes `['#', 'a', '#', 'b', '#']`.
    ///
    /// # Returns
    /// A `Vec<usize>` of length `2 * s.len() + 1`.
    /// `ret[i]` is the radius of the palindrome centered at `i` in the transformed string.
    /// The length of the palindrome in the original string corresponding to `ret[i]` is `ret[i] - 1`.
    pub fn build<T: PartialEq + Clone>(
        s: &[T],
        dummy: T,
    ) -> Vec<usize> {
        let n = s.len();
        let mut t = Vec::with_capacity(2 * n + 1);
        for item in s {
            t.push(dummy.clone());
            t.push(item.clone());
        }
        t.push(dummy);

        let m = t.len();
        let mut rad = vec![0; m];
        let mut i = 0;
        let mut j = 0;
        while i < m {
            while i >= j && i + j < m && t[i - j] == t[i + j] {
                j += 1;
            }
            rad[i] = j;
            let mut k = 1;
            while i >= k && k + rad[i - k] < j {
                rad[i + k] = rad[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
        rad
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manacher_odd() {
        let s: Vec<char> = "aba".chars().collect();
        let rad = Manacher::build(&s, '#');
        // # a # b # a #
        // 1 2 1 4 1 2 1
        // len: 0 1 0 3 0 1 0
        assert_eq!(rad, vec![1, 2, 1, 4, 1, 2, 1]);
        assert_eq!(rad[3] - 1, 3); // "aba"
    }

    #[test]
    fn test_manacher_even() {
        let s: Vec<char> = "abba".chars().collect();
        let rad = Manacher::build(&s, '#');
        // # a # b # b # a #
        // 1 2 1 2 5 2 1 2 1
        // Center of "abba" is the middle '#' (index 4). rad[4] = 5 ("#a#b#b#a#"). len = 4.
        assert_eq!(rad[4] - 1, 4);
    }
}
