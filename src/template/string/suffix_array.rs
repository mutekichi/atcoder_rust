#![allow(dead_code)]

// --- SNAP START ---

/// Suffix Array (Manber-Myers / Doubling Algorithm)
///
/// Constructs a Suffix Array for a given sequence.
/// The Suffix Array is a sorted array of all suffixes of a string.
///
/// # Complexity
/// - Construction: O(N (log N)^2)
/// - LCP Construction: O(N)
/// - Pattern Search: O(M log N) where M is pattern length
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::string::suffix_array::SuffixArray;
///
/// let s = "abracadabra";
/// let sa = SuffixArray::new(s);
///
/// // The suffix array (indices of sorted suffixes)
/// // [10("a"), 7("abra"), 0("abracadabra"), 3("acadabra"), 5("adabra"), 
/// //  8("bra"), 1("bracadabra"), 4("cadabra"), 6("dabra"), 9("ra"), 2("racadabra")]
/// let indices = sa.get_sa();
/// assert_eq!(indices[0], 10); // "a"
/// assert_eq!(indices[1], 7);  // "abra"
/// assert_eq!(indices[2], 0);  // "abracadabra"
/// 
/// // LCP Array (Longest Common Prefix between adjacent suffixes in SA)
/// let lcp = sa.get_lcp_array();
/// // lcp[0] is always 0. lcp[i] is LCP(suffix[sa[i-1]], suffix[sa[i]])
/// ```
pub struct SuffixArray<T> {
    n: usize,
    s: Vec<T>,
    sa: Vec<usize>,
    rank: Vec<usize>,
}

impl<T> SuffixArray<T>
where
    T: Ord + Clone + Copy,
{
    /// Constructs a Suffix Array from a slice (e.g., &str, &[u8], &[i32]).
    ///
    /// # Arguments
    /// - `s`: Input sequence. If it's a string, convert using `.as_bytes()` or `.chars().collect::<Vec<_>>()`.
    pub fn new<I>(s: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let s_vec: Vec<T> = s.into_iter().collect();
        let n = s_vec.len();
        let mut sa: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];

        // Initial rank based on the element itself
        // We map T to rank by sorting unique elements
        {
            let mut sorted_s = s_vec.clone();
            sorted_s.sort();
            sorted_s.dedup();
            for i in 0..n {
                rank[i] = sorted_s.binary_search(&s_vec[i]).unwrap();
            }
        }

        let mut k = 1;
        let mut tmp_rank = vec![0; n];

        // Doubling
        while k < n {
            // Sort based on (rank[i], rank[i+k])
            // Using simple sort_by_key is O(N (log N)^2)
            sa.sort_by(|&i, &j| {
                let rank_i = rank[i];
                let rank_j = rank[j];
                if rank_i != rank_j {
                    rank_i.cmp(&rank_j)
                } else {
                    let ri_k = if i + k < n { rank[i + k] as isize } else { -1 };
                    let rj_k = if j + k < n { rank[j + k] as isize } else { -1 };
                    ri_k.cmp(&rj_k)
                }
            });

            // Update ranks
            tmp_rank[sa[0]] = 0;
            for i in 1..n {
                let prev = sa[i - 1];
                let curr = sa[i];
                
                let rank_prev = rank[prev];
                let rank_curr = rank[curr];
                
                let second_rank_prev = if prev + k < n { rank[prev + k] as isize } else { -1 };
                let second_rank_curr = if curr + k < n { rank[curr + k] as isize } else { -1 };

                if rank_prev == rank_curr && second_rank_prev == second_rank_curr {
                    tmp_rank[curr] = tmp_rank[prev];
                } else {
                    tmp_rank[curr] = tmp_rank[prev] + 1;
                }
            }
            rank.copy_from_slice(&tmp_rank);
            
            k *= 2;
        }

        SuffixArray { n, s: s_vec, sa, rank }
    }

    /// Returns the Suffix Array (indices of sorted suffixes).
    pub fn get_sa(&self) -> &[usize] {
        &self.sa
    }

    /// Constructs the LCP (Longest Common Prefix) Array using Kasai's Algorithm.
    /// 
    /// `lcp[i]` stores the length of the LCP between `suffix[sa[i-1]]` and `suffix[sa[i]]`.
    /// `lcp[0]` is undefined (set to 0).
    ///
    /// # Complexity
    /// - O(N)
    pub fn get_lcp_array(&self) -> Vec<usize> {
        let n = self.n;
        let mut lcp = vec![0; n];
        let mut h = 0;

        // rank[i] is the rank of suffix starting at i in the sorted suffixes
        // self.rank is already computed in construction

        for i in 0..n {
            if self.rank[i] == 0 {
                continue;
            }
            let j = self.sa[self.rank[i] - 1];

            if h > 0 {
                h -= 1;
            }
            while i + h < n && j + h < n && self.s[i + h] == self.s[j + h] {
                h += 1;
            }
            lcp[self.rank[i]] = h;
        }
        lcp
    }

    /// Checks if the pattern `t` is contained in the string.
    /// Returns true if found.
    /// 
    /// # Complexity
    /// - O(M log N) where M is length of `t`
    pub fn contains(&self, t: &[T]) -> bool {
        let m = t.len();
        let mut l = 0;
        let mut r = self.n;
        
        while r - l > 0 {
            let mid = (l + r) / 2;
            let suffix_idx = self.sa[mid];
            
            // Compare suffix starting at `suffix_idx` with `t`
            let suffix_len = self.n - suffix_idx;
            let cmp_len = std::cmp::min(suffix_len, m);
            let suffix_slice = &self.s[suffix_idx..suffix_idx + cmp_len];
            
            if suffix_slice < t {
                l = mid + 1;
            } else if suffix_slice > t {
                r = mid;
            } else {
                if suffix_len >= m {
                    return true;
                } else {
                    // Suffix is a prefix of t, but shorter. t is "larger".
                    l = mid + 1;
                }
            }
        }
        false
    }
    
    /// Returns the number of occurrences of pattern `t`.
    /// 
    /// # Complexity
    /// - O(M log N)
    pub fn count(&self, t: &[T]) -> usize {
        // Find lower bound
        let mut l = 0;
        let mut r = self.n;
        while r - l > 0 {
            let mid = (l + r) / 2;
            let suffix_idx = self.sa[mid];
            let suffix_len = self.n - suffix_idx;
            let cmp_len = std::cmp::min(suffix_len, t.len());
            let suffix_slice = &self.s[suffix_idx..suffix_idx + cmp_len];
            
            if suffix_slice < t {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        let start = l;

        // Find upper bound
        let mut l = 0;
        let mut r = self.n;
        while r - l > 0 {
            let mid = (l + r) / 2;
            let suffix_idx = self.sa[mid];
            let suffix_len = self.n - suffix_idx;
            let cmp_len = std::cmp::min(suffix_len, t.len());
            let suffix_slice = &self.s[suffix_idx..suffix_idx + cmp_len];
            
            if suffix_slice <= t {
                // Treat suffix as <= t even if it matches exactly, but specific check for prefix match
                if suffix_slice == t && suffix_len >= t.len() {
                     l = mid + 1;
                } else if suffix_slice < t {
                     l = mid + 1;
                } else {
                     r = mid;
                }
            } else {
                r = mid;
            }
        }
        let end = l;
        
        if end >= start { end - start } else { 0 }
    }
}

// String specialized extension
impl SuffixArray<u8> {
    /// Helper to create from &str directly
    pub fn new_str(s: &str) -> Self {
        Self::new(s.bytes())
    }
}