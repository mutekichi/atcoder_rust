#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use memoise::memoise;
use num_integer::gcd;
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::{Add, Mul};
use std::ops::Bound::{self, Excluded, Included, Unbounded};

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

const INF_I64: i64 = 1 << 60;
const INF_USIZE: usize = 1 << 60;
const INF_F64: f64 = 1e18;
const INF_I128: i128 = 1 << 120;
const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const C998244353: u64 = 998244353;
const C1000000007: u64 = 1000000007;

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

#[allow(unused_variables)]
fn main() {
    input! {
        k: usize,
        m: i64,
        CL: [(i64, i64); k],
    }
    let mut data_tens = vec![Data::new(10, m)];
    for i in 0..100000 {
        data_tens.push(data_tens[i] * data_tens[i]);
    }
    let mut data_ones = vec![Data::new(1, m)];
    for i in 0..100000 {
        data_ones.push(data_ones[i] * data_tens[i] + data_ones[i]);
    }

    let mut ans = Data::new(0, m);
    let mut l_sum = 0;
    for &(c, l) in CL.iter().rev() {
        let part_tens = {
            let mut base = Data::new(1, m);
            for i in 0..64 {
                if (l_sum >> i) & 1 == 1 {
                    base = base * data_tens[i];
                }
            }
            base
        };
        let part_ones = {
            let mut base = Data::new(1, m);
            for i in 0..32 {
                if ((l - 1) >> i) & 1 == 1 {
                    base = base * data_tens[i];
                    base = base + data_ones[i];
                }
            }
            base
        };
        md!(part_ones.to(), part_tens.to());
        let to_add = Data::new(c, m) * part_ones * part_tens;
        md!(to_add.to());
        ans = ans + to_add;
        l_sum += l;
    }
    println!("{}", ans.a);
}

#[derive(Copy, Clone)]
struct Data {
    a: i64,
    b: i64,
    m: i64,
}

impl Data  {
    fn new(n: i64, m: i64) -> Self {
        Data {
            a: n / m % 10007,
            b: n % m,
            m: m
        }
    }
    fn to(
        self
    ) -> i64 {
        self.a * self.m + self.b
    }
}

impl Add for Data {
    type Output = Self;
    fn add(
        self,
        other: Self,
    ) -> Self {
        let sa = self.a;
        let sb = self.b;
        let oa = other.a;
        let ob = other.b;
        Self {
            a: (sa + oa + if sb + ob >= self.m { 1 } else { 0 }) % 10007,
            b: (sb + ob) % self.m,
            m: self.m
        }
    }
}
impl Mul for Data {
    type Output = Self;
    fn mul(
        self,
        other: Self,
    ) -> Self {
        let a = self.a;
        let b = self.b;
        let c = other.a;
        let d = other.b;
        let m = self.m;
        let base = a * c * m + a * d + c * b;
        Self {
            a: (base + b * d / m) % 10007,
            b: b * d % m,
            m: m
        }
    }
}
