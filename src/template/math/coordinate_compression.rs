#![allow(dead_code)]

// --- SNAP START ---

/// Coordinate Compression (Zaatsu)
///
/// Compresses a set of values into indices [0, N-1] preserving order.
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::utils::coordinate_compression::CoordinateCompression;
///
/// let data = vec![100, 2, 100, 50, 2];
/// let cc = CoordinateCompression::new(data);
///
/// assert_eq!(cc.size(), 3); // {2, 50, 100}
/// assert_eq!(cc.compress(&2), 0);
/// assert_eq!(cc.compress(&50), 1);
/// assert_eq!(cc.compress(&100), 2);
/// assert_eq!(cc.decompress(1), 50);
/// ```
#[derive(Debug, Clone)]
pub struct CoordinateCompression<T> {
    pub xs: Vec<T>,
}

impl<T: Ord + Clone + Copy> CoordinateCompression<T> {
    /// Constructs a new `CoordinateCompression` from a vector of values.
    ///
    /// Duplicates are removed and the values are sorted.
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort();
        data.dedup();
        CoordinateCompression { xs: data }
    }

    /// Returns the compressed index for the given value.
    ///
    /// # Panics
    /// Panics if the value is not found (use `binary_search` directly if handling missing values).
    pub fn compress(&self, val: &T) -> usize {
        self.xs.binary_search(val).expect("Value not found in compressed coordinates")
    }

    /// Returns the original value for the given compressed index.
    pub fn decompress(&self, i: usize) -> T {
        self.xs[i]
    }

    /// Returns the number of unique values.
    pub fn size(&self) -> usize {
        self.xs.len()
    }
}
