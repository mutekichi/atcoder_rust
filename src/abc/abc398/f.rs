#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use num_integer::gcd;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem;
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// FOR TEMPLATE INJECTIONS

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
        S: Chars,
    }
    let rad = Manacher::build(&S, '#');
    let n = S.len();
    let mut ans = INF_USIZE;
    for i in 0..rad.len() {
        if rad[rad.len() - 1 - i] == i + 1 {
            ans = i;
        }
    }
    let mut ans_chars = vec![];
    for i in 0..n {
        ans_chars.push(S[i]);
    }
    let to_push = n - ans;
    for i in 0..to_push {
        ans_chars.push(S[to_push - 1 - i]);
    }
    let ans_str: String = ans_chars.iter().collect();
    wl!(ans_str);
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

trait JoinExtended {
    fn join_with(
        self,
        sep: &str,
    ) -> String;
}

impl<I> JoinExtended for I
where
    I: Iterator,
    I::Item: Joinable,
{
    fn join_with(
        self,
        sep: &str,
    ) -> String {
        let mut peekable = self.peekable();
        let is_2d = if let Some(first) = peekable.peek() {
            first.is_container()
        } else {
            false
        };

        let res = peekable.map(|item| item.join_item(sep)).collect::<Vec<_>>();

        // Use newline for 2D rows, provided sep for 1D elements
        res.join(if is_2d { "\n" } else { sep })
    }
}

trait Joinable {
    fn join_item(
        &self,
        sep: &str,
    ) -> String;
    fn is_container(&self) -> bool;
}

macro_rules! impl_joinable_scalar {
    ($($t:ty),*) => {
        $(
            impl Joinable for &$t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
            impl Joinable for $t {
                fn join_item(&self, _sep: &str) -> String { self.to_string() }
                fn is_container(&self) -> bool { false }
            }
        )*
    };
}

impl_joinable_scalar!(
    i32, i64, i128, u32, u64, u128, usize, isize, f32, f64, char, String, &str
);

impl<T: std::fmt::Display> Joinable for &Vec<T> {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}

impl<T: std::fmt::Display> Joinable for &[T] {
    fn join_item(
        &self,
        sep: &str,
    ) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
    fn is_container(&self) -> bool {
        true
    }
}
