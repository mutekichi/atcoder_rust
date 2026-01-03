#![allow(dead_code)]

// --- SNAP START ---

/// Longest Increasing Subsequence (LIS)
///
/// Calculates the indices of the Longest Increasing Subsequence in O(N log N).
///
/// # Arguments
///
/// * `arr` - The input sequence.
/// * `is_strict` - If true, finds strictly increasing subsequence (a < b).
///                 If false, finds non-decreasing subsequence (a <= b).
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::math::lis::get_lis_indices;
///
/// let arr = vec![3, 1, 4, 1, 5, 9, 2];
/// let indices = get_lis_indices(&arr, true);
/// // One of the LIS is [1, 4, 5, 9], corresponding to indices [1, 2, 4, 5].
/// assert_eq!(indices.len(), 4);
/// assert_eq!(indices.iter().map(|&i| arr[i]).collect::<Vec<_>>(), vec![1, 4, 5, 9]);
///
/// let arr = vec![1, 2, 1, 3, 2];
/// let indices = get_lis_indices(&arr, false);
/// // Non-decreasing LIS could be [1, 1, 2] or [1, 2, 3] etc.
/// assert_eq!(indices.len(), 3);
/// ```
pub fn get_lis_indices<T: Ord>(
    arr: &[T],
    is_strict: bool,
) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }

    let n = arr.len();
    let mut dp = Vec::with_capacity(n);
    let mut pos = vec![0; n];

    for (i, x) in arr.iter().enumerate() {
        let idx = if is_strict {
            dp.partition_point(|&it| it < x)
        } else {
            dp.partition_point(|&it| it <= x)
        };

        if idx == dp.len() {
            dp.push(x);
        } else {
            dp[idx] = x;
        }
        pos[i] = idx;
    }

    // Restore indices from the back
    let length = dp.len();
    let mut res = vec![0; length];
    let mut current_target = length - 1;
    for i in (0..n).rev() {
        if pos[i] == current_target {
            res[current_target] = i;
            if current_target == 0 {
                break;
            }
            current_target -= 1;
        }
    }

    res
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lis_indices_strict() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2];
        // LIS: [3, 4, 5, 9] or [1, 4, 5, 9]
        let indices = get_lis_indices(&arr, true);
        assert_eq!(indices.len(), 4);

        let values: Vec<_> = indices.iter().map(|&i| arr[i]).collect();
        for i in 0..values.len() - 1 {
            assert!(values[i] < values[i + 1]);
        }
    }

    #[test]
    fn test_get_lis_indices_non_strict() {
        let arr = vec![2, 3, 2, 4, 3, 1, 5, 3, 4];
        // LIS is [2, 2, 3, 3, 4]
        let indices = get_lis_indices(&arr, false);
        assert_eq!(indices.len(), 5);

        let values: Vec<_> = indices.iter().map(|&i| arr[i]).collect();
        for i in 0..values.len() - 1 {
            assert!(values[i] <= values[i + 1]);
        }
    }

    #[test]
    fn test_empty_arr() {
        let arr: Vec<i32> = vec![];
        let indices = get_lis_indices(&arr, true);
        assert!(indices.is_empty());
    }
}
