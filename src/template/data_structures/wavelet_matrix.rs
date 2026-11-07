#![allow(dead_code)]

use super::segment_tree::SegmentTree;

// INJECT: src/template/data_structures/segment_tree.rs
// --- SNAP START ---

/// Immutable bit vector with O(1) access and rank queries.
/// Uses 64-bit blocks and one prefix count per block (O(N / 64 + 1) words).
#[derive(Debug, Clone)]
pub struct BitVector {
    len: usize,
    blocks: Vec<u64>,
    prefix: Vec<usize>,
}

impl BitVector {
    pub fn new(bits: &[bool]) -> Self {
        let mut blocks = vec![0u64; bits.len().div_ceil(64)];
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                blocks[i / 64] |= 1u64 << (i % 64);
            }
        }
        let mut prefix = Vec::with_capacity(blocks.len() + 1);
        prefix.push(0);
        for &block in &blocks {
            prefix.push(
                prefix.last().unwrap() + block.count_ones() as usize,
            );
        }
        Self {
            len: bits.len(),
            blocks,
            prefix,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Panics if i >= len.
    pub fn get(
        &self,
        i: usize,
    ) -> bool {
        assert!(i < self.len);
        self.blocks[i / 64] >> (i % 64) & 1 != 0
    }

    /// Number of ones in [0, end). Panics if end > len.
    pub fn rank1(
        &self,
        end: usize,
    ) -> usize {
        assert!(end <= self.len);
        let (block, offset) = (end / 64, end % 64);
        let partial = if offset == 0 {
            0
        } else {
            (self.blocks[block] & ((1u64 << offset) - 1)).count_ones()
                as usize
        };
        self.prefix[block] + partial
    }

    /// Number of zeros in [0, end). Panics if end > len.
    pub fn rank0(
        &self,
        end: usize,
    ) -> usize {
        end - self.rank1(end)
    }
}

/// Static wavelet matrix for u64 keys.
///
/// Index intervals are [l, r); k is zero-based. Invalid index intervals panic.
/// Value intervals accept RangeBounds<u64>, including .. and ..=u64::MAX.
/// Empty/reversed value intervals contain no elements.
/// Construction: O(NH) time, O(H * (N / 64 + 1)) stored words,
/// where H = max(1, bit length of the maximum key), at most 64.
/// Queries: O(H) time. Keys cannot be updated.
///
/// ```
/// use atcoder_rust::template::data_structures::wavelet_matrix::WaveletMatrix;
/// let wm = WaveletMatrix::new(&[3, 1, 4, 1, 5, 9, 2]);
/// assert_eq!(wm.access(2), Some(4));
/// assert_eq!(wm.kth_smallest(1, 5, 2), Some(4));
/// assert_eq!(wm.range_freq(0, 7, 2..5), 3);
/// assert_eq!(wm.count(0, 7, 1), 2);
/// assert_eq!(wm.prev_value(0, 7, 4), Some(3));
/// assert_eq!(wm.next_value(0, 7, 4), Some(4));
/// ```
///
/// Coordinate compression: sort/deduplicate the original values, map keys to
/// their indices, and translate query bounds with partition_point. Decode a
/// kth result by indexing the sorted values. Signed keys can be handled this way.
///
/// References:
/// - https://info.atcoder.jp/entry/algorithm_lectures/wavelet_matrix_basic
/// - https://info.atcoder.jp/entry/algorithm_lectures/wavelet_matrix_advanced
#[derive(Debug, Clone)]
pub struct WaveletMatrix {
    len: usize,
    height: usize,
    bits: Vec<BitVector>,
    zeros: Vec<usize>,
}

impl WaveletMatrix {
    pub fn new(data: &[u64]) -> Self {
        Self::build(data, |_| {})
    }

    // Each layer is a stable permutation of original indices. Aggregation
    // backends consume it here so partitioning is implemented only once.
    fn build(
        data: &[u64],
        mut on_layer: impl FnMut(&[usize]),
    ) -> Self {
        let max = data.iter().copied().max().unwrap_or(0);
        let height = (64 - max.leading_zeros() as usize).max(1);
        let mut order: Vec<_> = (0..data.len()).collect();
        let mut next = Vec::with_capacity(data.len());
        let mut bits = Vec::with_capacity(height);
        let mut zeros = Vec::with_capacity(height);
        on_layer(&order);
        for shift in (0..height).rev() {
            let directions: Vec<_> =
                order.iter().map(|&i| data[i] >> shift & 1 != 0).collect();
            let bv = BitVector::new(&directions);
            zeros.push(bv.rank0(data.len()));
            bits.push(bv);
            next.clear();
            for direction in [false, true] {
                next.extend(order.iter().zip(&directions).filter_map(
                    |(&i, &bit)| (bit == direction).then_some(i),
                ));
            }
            std::mem::swap(&mut order, &mut next);
            on_layer(&order);
        }
        Self {
            len: data.len(),
            height,
            bits,
            zeros,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn check_range(
        &self,
        l: usize,
        r: usize,
    ) {
        assert!(l <= r && r <= self.len, "Invalid index range");
    }

    fn children(
        &self,
        depth: usize,
        l: usize,
        r: usize,
    ) -> ((usize, usize), (usize, usize)) {
        let (l1, r1) =
            (self.bits[depth].rank1(l), self.bits[depth].rank1(r));
        (
            (l - l1, r - r1),
            (self.zeros[depth] + l1, self.zeros[depth] + r1),
        )
    }

    /// Original key at i, or None if i is out of bounds.
    pub fn access(
        &self,
        mut i: usize,
    ) -> Option<u64> {
        if i >= self.len {
            return None;
        }
        let mut value = 0;
        for depth in 0..self.height {
            let bit = self.bits[depth].get(i);
            value = (value << 1) | u64::from(bit);
            i = if bit {
                self.zeros[depth] + self.bits[depth].rank1(i)
            } else {
                self.bits[depth].rank0(i)
            };
        }
        Some(value)
    }

    /// Returns None if k >= r - l, including an empty interval.
    pub fn kth_smallest(
        &self,
        mut l: usize,
        mut r: usize,
        mut k: usize,
    ) -> Option<u64> {
        self.check_range(l, r);
        if k >= r - l {
            return None;
        }
        let mut value = 0;
        for depth in 0..self.height {
            let (left, right) = self.children(depth, l, r);
            let left_size = left.1 - left.0;
            let bit = k >= left_size;
            value = (value << 1) | u64::from(bit);
            if bit {
                k -= left_size;
                (l, r) = right;
            } else {
                (l, r) = left;
            }
        }
        Some(value)
    }

    pub fn kth_largest(
        &self,
        l: usize,
        r: usize,
        k: usize,
    ) -> Option<u64> {
        self.check_range(l, r);
        if k >= r - l {
            None
        } else {
            self.kth_smallest(l, r, r - l - 1 - k)
        }
    }

    /// Number of occurrences of value in [l, r).
    pub fn count(
        &self,
        l: usize,
        r: usize,
        value: u64,
    ) -> usize {
        self.range_freq(l, r, value..=value)
    }

    /// Number of keys strictly below upper in [l, r).
    pub fn count_less_than(
        &self,
        l: usize,
        r: usize,
        upper: u64,
    ) -> usize {
        self.check_range(l, r);
        self.count_less_than_u128(l, r, upper as u128)
    }

    fn count_less_than_u128(
        &self,
        mut l: usize,
        mut r: usize,
        upper: u128,
    ) -> usize {
        if upper >= 1u128 << self.height {
            return r - l;
        }
        let mut count = 0;
        for depth in 0..self.height {
            let (left, right) = self.children(depth, l, r);
            if upper >> (self.height - 1 - depth) & 1 != 0 {
                count += left.1 - left.0;
                (l, r) = right;
            } else {
                (l, r) = left;
            }
        }
        count
    }

    pub fn range_freq<R: std::ops::RangeBounds<u64>>(
        &self,
        l: usize,
        r: usize,
        values: R,
    ) -> usize {
        self.check_range(l, r);
        let (lower, upper) = Self::value_bounds(values);
        if lower >= upper {
            return 0;
        }
        self.count_less_than_u128(l, r, upper)
            - self.count_less_than_u128(l, r, lower)
    }

    /// Largest key strictly below upper, or None if it does not exist.
    pub fn prev_value(
        &self,
        l: usize,
        r: usize,
        upper: u64,
    ) -> Option<u64> {
        let count = self.count_less_than(l, r, upper);
        count
            .checked_sub(1)
            .and_then(|k| self.kth_smallest(l, r, k))
    }

    /// Smallest key greater than or equal to lower, or None.
    pub fn next_value(
        &self,
        l: usize,
        r: usize,
        lower: u64,
    ) -> Option<u64> {
        self.kth_smallest(l, r, self.count_less_than(l, r, lower))
    }

    fn value_bounds<R: std::ops::RangeBounds<u64>>(
        values: R
    ) -> (u128, u128) {
        use std::ops::Bound;
        let lower = match values.start_bound() {
            Bound::Included(&x) => x as u128,
            Bound::Excluded(&x) => x as u128 + 1,
            Bound::Unbounded => 0,
        };
        let upper = match values.end_bound() {
            Bound::Included(&x) => x as u128 + 1,
            Bound::Excluded(&x) => x as u128,
            Bound::Unbounded => 1u128 << 64,
        };
        (lower, upper)
    }

    // Decompose a value interval into O(H) fully covered nodes. Each node
    // refers to [l, r) in the stable permutation at its depth (root = 0).
    fn visit_nodes(
        &self,
        l: usize,
        r: usize,
        lower: u128,
        upper: u128,
        mut visit: impl FnMut(usize, usize, usize),
    ) {
        if lower >= upper || l == r {
            return;
        }
        let mut stack = vec![(0, l, r, 0u128, 1u128 << self.height)];
        while let Some((depth, l, r, start, end)) = stack.pop() {
            if l == r || upper <= start || end <= lower {
                continue;
            }
            if lower <= start && end <= upper {
                visit(depth, l, r);
                continue;
            }
            // Integral query bounds guarantee a leaf is covered or disjoint.
            let middle = (start + end) / 2;
            let (left, right) = self.children(depth, l, r);
            stack.push((depth + 1, right.0, right.1, middle, end));
            stack.push((depth + 1, left.0, left.1, start, middle));
        }
    }
}

/// Associative, commutative operation with an identity.
/// Commutativity is required because wavelet layers reorder the elements.
pub trait CommutativeMonoid {
    type Value: Clone + std::fmt::Debug;
    fn identity() -> Self::Value;
    fn combine(
        a: &Self::Value,
        b: &Self::Value,
    ) -> Self::Value;
}

/// A commutative monoid whose elements all have inverses.
pub trait AbelianGroup: CommutativeMonoid {
    fn inverse(value: &Self::Value) -> Self::Value;
}

/// Static range aggregation backend. query(l, r) aggregates [l, r).
/// Empty intervals must return the monoid identity.
pub trait RangeAggregate {
    type Monoid: CommutativeMonoid;
    fn new(data: &[<Self::Monoid as CommutativeMonoid>::Value]) -> Self;
    fn query(
        &self,
        l: usize,
        r: usize,
    ) -> <Self::Monoid as CommutativeMonoid>::Value;
}

/// O(N) construction/space, O(1) queries using prefix aggregates.
/// Arithmetic must fit in the chosen Value type, including inverses.
pub struct PrefixAggregate<G: AbelianGroup> {
    prefix: Vec<G::Value>,
}

impl<G: AbelianGroup> RangeAggregate for PrefixAggregate<G> {
    type Monoid = G;

    fn new(data: &[G::Value]) -> Self {
        let mut prefix = Vec::with_capacity(data.len() + 1);
        prefix.push(G::identity());
        for value in data {
            prefix.push(G::combine(prefix.last().unwrap(), value));
        }
        Self { prefix }
    }

    fn query(
        &self,
        l: usize,
        r: usize,
    ) -> G::Value {
        assert!(l <= r && r < self.prefix.len());
        G::combine(&G::inverse(&self.prefix[l]), &self.prefix[r])
    }
}

/// O(N) construction/space, O(log N) queries. Reuses SegmentTree.
/// Supports any CommutativeMonoid; no inverse is required.
pub struct SegmentAggregate<M: CommutativeMonoid> {
    tree: SegmentTree<M::Value, fn(M::Value, M::Value) -> M::Value>,
}

impl<M: CommutativeMonoid> RangeAggregate for SegmentAggregate<M> {
    type Monoid = M;

    fn new(data: &[M::Value]) -> Self {
        Self {
            tree: SegmentTree::new(
                data,
                |a, b| M::combine(&a, &b),
                M::identity(),
            ),
        }
    }

    fn query(
        &self,
        l: usize,
        r: usize,
    ) -> M::Value {
        self.tree.query(l, r)
    }
}

/// Signed sums. The caller must ensure i128 arithmetic does not overflow.
pub enum SumI128 {}

impl CommutativeMonoid for SumI128 {
    type Value = i128;
    fn identity() -> i128 {
        0
    }
    fn combine(
        a: &i128,
        b: &i128,
    ) -> i128 {
        a + b
    }
}

impl AbelianGroup for SumI128 {
    fn inverse(value: &i128) -> i128 {
        -value
    }
}

/// Maximum of i64 weights. Empty results are i64::MIN.
pub enum MaxI64 {}

impl CommutativeMonoid for MaxI64 {
    type Value = i64;
    fn identity() -> i64 {
        i64::MIN
    }
    fn combine(
        a: &i64,
        b: &i64,
    ) -> i64 {
        (*a).max(*b)
    }
}

/// Wavelet matrix with a replaceable range aggregation backend per layer.
/// Keys and weights are static. Does not require an inverse: value intervals
/// are decomposed into O(H) covered nodes rather than subtracting two queries.
///
/// If the backend has construction time C(N), space S(N), query time Q(N),
/// construction takes O(NH + H*C(N)), space O(H*(N/64+1) + H*S(N)),
/// and query takes O(H * (1 + Q(N))).
/// PrefixAggregate gives O(NH) construction/space and O(H) queries;
/// SegmentAggregate gives O(NH) construction/space and O(H log(N+1)) queries.
/// Backend operations and cloning a weight are assumed O(1).
///
/// ```
/// use atcoder_rust::template::data_structures::wavelet_matrix::{
///     WaveletMatrixAggregate, PrefixAggregate, SegmentAggregate,
///     SumI128, MaxI64,
/// };
/// let keys = [3, 1, 4, 1, 5];
/// let sums = WaveletMatrixAggregate::<PrefixAggregate<SumI128>>::new(
///     &keys, &[30, 10, 40, -2, 50],
/// );
/// assert_eq!(sums.query(0, 5, 1..4), 38);
/// assert_eq!(sums.matrix().kth_smallest(0, 5, 1), Some(1));
/// let maxima = WaveletMatrixAggregate::<SegmentAggregate<MaxI64>>::new(
///     &keys, &[30, 10, 40, -2, 50],
/// );
/// assert_eq!(maxima.query(0, 5, 1..4), 30);
/// assert_eq!(maxima.query(0, 5, 6..), i64::MIN);
///
/// // To sum keys themselves, pass the keys converted to i128 as weights.
/// let weights: Vec<_> = keys.iter().map(|&x| x as i128).collect();
/// let wm = WaveletMatrixAggregate::<PrefixAggregate<SumI128>>::new(
///     &keys, &weights,
/// );
/// assert_eq!(wm.query(0, 5, 1..4), 5);
/// ```
///
/// Rectangle queries: sort points (x, y, weight) by x, build on their y keys,
/// and use partition_point on the sorted x coordinates to obtain [l, r).
/// Then query(l, r, bottom..top) answers [left, right) x [bottom, top).
/// With compressed coordinates, translate both y bounds using lower_bound.
///
/// ```
/// use atcoder_rust::template::data_structures::wavelet_matrix::{
///     WaveletMatrixAggregate, PrefixAggregate, SumI128,
/// };
/// let mut points = vec![(3i64, -2i64, 7i128), (-1, 4, 10), (3, 4, -2)];
/// points.sort_unstable_by_key(|p| p.0);
/// let mut ys: Vec<_> = points.iter().map(|p| p.1).collect();
/// ys.sort_unstable();
/// ys.dedup();
/// let keys: Vec<_> = points.iter()
///     .map(|p| ys.binary_search(&p.1).unwrap() as u64).collect();
/// let weights: Vec<_> = points.iter().map(|p| p.2).collect();
/// let wm = WaveletMatrixAggregate::<PrefixAggregate<SumI128>>::new(
///     &keys, &weights,
/// );
/// // Rectangle: 0 <= x < 4, -3 <= y < 4 (bounds need not be stored coordinates).
/// let l = points.partition_point(|p| p.0 < 0);
/// let r = points.partition_point(|p| p.0 < 4);
/// let bottom = ys.partition_point(|&y| y < -3) as u64;
/// let top = ys.partition_point(|&y| y < 4) as u64;
/// assert_eq!(wm.query(l, r, bottom..top), 7);
/// ```
pub struct WaveletMatrixAggregate<B: RangeAggregate> {
    matrix: WaveletMatrix,
    layers: Vec<B>,
}

impl<B: RangeAggregate> WaveletMatrixAggregate<B> {
    /// Panics if keys and weights have different lengths.
    pub fn new(
        keys: &[u64],
        weights: &[<B::Monoid as CommutativeMonoid>::Value],
    ) -> Self {
        assert_eq!(keys.len(), weights.len());
        let mut layers = Vec::new();
        let matrix = WaveletMatrix::build(keys, |order| {
            let data: Vec<_> =
                order.iter().map(|&i| weights[i].clone()).collect();
            layers.push(B::new(&data));
        });
        Self { matrix, layers }
    }

    /// Access the unweighted queries without storing a second matrix.
    pub fn matrix(&self) -> &WaveletMatrix {
        &self.matrix
    }

    /// Aggregate weights whose indices are in [l, r) and keys are in values.
    /// Empty results return the monoid identity. Invalid indices panic.
    pub fn query<R: std::ops::RangeBounds<u64>>(
        &self,
        l: usize,
        r: usize,
        values: R,
    ) -> <B::Monoid as CommutativeMonoid>::Value {
        self.matrix.check_range(l, r);
        let (lower, upper) = WaveletMatrix::value_bounds(values);
        let mut result = B::Monoid::identity();
        self.matrix.visit_nodes(l, r, lower, upper, |depth, l, r| {
            result = B::Monoid::combine(
                &result,
                &self.layers[depth].query(l, r),
            );
        });
        result
    }
}

// --- SNAP END ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound::{Excluded, Included, Unbounded};

    fn random(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn bit_vector_block_boundaries() {
        for n in [0, 1, 63, 64, 65, 127, 128, 129, 256] {
            for pattern in 0..3 {
                let bits: Vec<_> = (0..n)
                    .map(|i| match pattern {
                        0 => false,
                        1 => true,
                        _ => i % 3 == 0,
                    })
                    .collect();
                let bv = BitVector::new(&bits);
                assert_eq!(bv.len(), n);
                assert_eq!(bv.is_empty(), n == 0);
                for end in 0..=n {
                    let ones = bits[..end].iter().filter(|&&x| x).count();
                    assert_eq!(bv.rank1(end), ones);
                    assert_eq!(bv.rank0(end), end - ones);
                }
                for (i, &bit) in bits.iter().enumerate() {
                    assert_eq!(bv.get(i), bit);
                }
            }
        }
    }

    fn check_matrix(data: &[u64]) {
        let wm = WaveletMatrix::new(data);
        let bounds = [0, 1, 2, 5, 16, 1 << 63, u64::MAX - 1, u64::MAX];
        assert_eq!(wm.len(), data.len());
        assert_eq!(wm.is_empty(), data.is_empty());
        for (i, &value) in data.iter().enumerate() {
            assert_eq!(wm.access(i), Some(value));
        }
        assert_eq!(wm.access(data.len()), None);
        assert_eq!(wm.access(usize::MAX), None);
        for l in 0..=data.len() {
            for r in l..=data.len() {
                let mut sorted = data[l..r].to_vec();
                sorted.sort_unstable();
                for k in 0..=sorted.len() {
                    assert_eq!(
                        wm.kth_smallest(l, r, k),
                        sorted.get(k).copied()
                    );
                    assert_eq!(
                        wm.kth_largest(l, r, k),
                        sorted.iter().rev().nth(k).copied()
                    );
                }
                assert_eq!(wm.kth_smallest(l, r, usize::MAX), None);
                assert_eq!(wm.kth_largest(l, r, usize::MAX), None);
                assert_eq!(wm.range_freq(l, r, ..), r - l);
                assert_eq!(wm.range_freq(l, r, ..=u64::MAX), r - l);
                for &x in &bounds {
                    assert_eq!(
                        wm.count(l, r, x),
                        sorted.iter().filter(|&&v| v == x).count()
                    );
                    assert_eq!(
                        wm.count_less_than(l, r, x),
                        sorted.iter().filter(|&&v| v < x).count()
                    );
                    assert_eq!(
                        wm.prev_value(l, r, x),
                        sorted.iter().copied().filter(|&v| v < x).max()
                    );
                    assert_eq!(
                        wm.next_value(l, r, x),
                        sorted.iter().copied().find(|&v| v >= x)
                    );
                    for &y in &bounds {
                        let expected = sorted
                            .iter()
                            .filter(|&&v| x <= v && v < y)
                            .count();
                        assert_eq!(wm.range_freq(l, r, x..y), expected);
                        let expected = sorted
                            .iter()
                            .filter(|&&v| x <= v && v <= y)
                            .count();
                        assert_eq!(wm.range_freq(l, r, x..=y), expected);
                        let expected = sorted
                            .iter()
                            .filter(|&&v| x < v && v <= y)
                            .count();
                        assert_eq!(
                            wm.range_freq(
                                l,
                                r,
                                (Excluded(x), Included(y))
                            ),
                            expected
                        );
                    }
                }
                assert_eq!(
                    wm.range_freq(l, r, (Excluded(u64::MAX), Unbounded)),
                    0
                );
            }
        }
    }

    #[test]
    fn matrix_against_sorting_and_filtering() {
        for data in [
            vec![],
            vec![0],
            vec![0; 9],
            vec![7; 9],
            vec![u64::MAX, 0, 1 << 63, u64::MAX, 1, u64::MAX - 1],
        ] {
            check_matrix(&data);
        }
        let mut seed = 875329;
        for case in 0..35 {
            let n = (random(&mut seed) % 12) as usize;
            let data: Vec<_> = (0..n)
                .map(|_| {
                    let x = random(&mut seed);
                    if case % 2 == 0 { x % 16 } else { x }
                })
                .collect();
            check_matrix(&data);
        }
    }

    enum XorGroup {}
    impl CommutativeMonoid for XorGroup {
        type Value = u64;
        fn identity() -> u64 {
            0
        }
        fn combine(
            a: &u64,
            b: &u64,
        ) -> u64 {
            a ^ b
        }
    }
    impl AbelianGroup for XorGroup {
        fn inverse(value: &u64) -> u64 {
            *value
        }
    }

    #[test]
    fn aggregation_backends_against_filtering() {
        let mut seed = 39875;
        for n in [0, 1, 6, 15, 63, 64, 65, 128, 129] {
            let keys: Vec<_> = (0..n)
                .map(|i| match i % 4 {
                    0 => 0,
                    1 => u64::MAX,
                    2 => random(&mut seed),
                    _ => random(&mut seed) % 16,
                })
                .collect();
            let weights: Vec<_> = (0..n)
                .map(|_| (random(&mut seed) % 101) as i128 - 50)
                .collect();
            let max_weights: Vec<_> =
                weights.iter().map(|&x| x as i64).collect();
            let xor_weights: Vec<_> =
                (0..n).map(|_| random(&mut seed)).collect();
            let prefix =
                WaveletMatrixAggregate::<PrefixAggregate<SumI128>>::new(
                    &keys, &weights,
                );
            let segment = WaveletMatrixAggregate::<
                SegmentAggregate<SumI128>,
            >::new(&keys, &weights);
            let maxima =
                WaveletMatrixAggregate::<SegmentAggregate<MaxI64>>::new(
                    &keys,
                    &max_weights,
                );
            let xors =
                WaveletMatrixAggregate::<PrefixAggregate<XorGroup>>::new(
                    &keys,
                    &xor_weights,
                );
            assert_eq!(prefix.matrix().len(), n);
            let endpoints = [
                Unbounded,
                Included(0),
                Excluded(0),
                Included(7),
                Excluded(7),
                Included(1 << 63),
                Excluded(1 << 63),
                Included(u64::MAX),
                Excluded(u64::MAX),
            ];
            for _ in 0..40 {
                let a = (random(&mut seed) % (n as u64 + 1)) as usize;
                let b = (random(&mut seed) % (n as u64 + 1)) as usize;
                let (l, r) = (a.min(b), a.max(b));
                for &start in &endpoints {
                    for &end in &endpoints {
                        let range = (start, end);
                        let selected: Vec<_> = (l..r)
                            .filter(|&i| {
                                use std::ops::RangeBounds;
                                range.contains(&keys[i])
                            })
                            .collect();
                        let sum: i128 =
                            selected.iter().map(|&i| weights[i]).sum();
                        let max = selected
                            .iter()
                            .map(|&i| max_weights[i])
                            .max()
                            .unwrap_or(i64::MIN);
                        let xor = selected
                            .iter()
                            .fold(0, |acc, &i| acc ^ xor_weights[i]);
                        assert_eq!(prefix.query(l, r, range), sum);
                        assert_eq!(segment.query(l, r, range), sum);
                        assert_eq!(maxima.query(l, r, range), max);
                        assert_eq!(xors.query(l, r, range), xor);
                    }
                }
            }
            assert_eq!(prefix.query(0, n, ..), weights.iter().sum());
        }
    }

    #[test]
    fn zero_keys_still_aggregate_leaf_weights() {
        let wm = WaveletMatrixAggregate::<PrefixAggregate<SumI128>>::new(
            &[0, 0, 0],
            &[10, -20, 30],
        );
        assert_eq!(wm.query(1, 3, 0..=0), 10);
        assert_eq!(wm.query(0, 3, 1..), 0);
    }

    #[test]
    #[should_panic(expected = "Invalid index range")]
    fn invalid_index_range_panics() {
        WaveletMatrix::new(&[1]).range_freq(0, 2, ..);
    }

    #[test]
    #[should_panic(expected = "Invalid index range")]
    fn reversed_index_range_panics() {
        WaveletMatrix::new(&[1]).kth_smallest(1, 0, 0);
    }

    #[test]
    #[should_panic]
    fn mismatched_weights_panics() {
        WaveletMatrixAggregate::<PrefixAggregate<SumI128>>::new(&[1], &[]);
    }
}
