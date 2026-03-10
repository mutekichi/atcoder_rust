#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use rand::Rng;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! md {
    ($($arg:expr),* $(,)?) => {{
        eprint!("[{}:{}] ", file!(), line!());
        let mut _first = true;
        $(
            if !_first { eprint!(", "); }
            eprint!("{}: {}", stringify!($arg), $arg);
            _first = false;
        )*
        eprintln!();
    }};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! md {
    ($($arg:expr),* $(,)?) => {{}};
}

struct Solver {
    n: usize,
    h: Vec<i32>,
    c: Vec<i32>,
    a: Vec<Vec<i32>>,
    opened: Vec<bool>,
    durabilities: Vec<i32>,
    weapon_queue: VecDeque<usize>,
    open_count: usize,
}

impl Solver {
    fn new() -> Self {
        input! {
            n: usize,
            h: [i32; n],
            c: [i32; n],
            a: [[i32; n]; n],
        }

        Self {
            n,
            h,
            c: c.clone(),
            a,
            opened: vec![false; n],
            durabilities: c,
            weapon_queue: VecDeque::new(),
            open_count: 0,
        }
    }

    fn solve(&mut self) {
        while self.open_count < self.n {
            if self.weapon_queue.is_empty() {
                self.open_with_bare_hands();
            }
            self.process_weapons();
        }
    }

    fn open_with_bare_hands(&mut self) {
        let mut best_b = None;
        let mut min_h = i32::MAX;

        for b in 0..self.n {
            if !self.opened[b] && self.h[b] < min_h {
                min_h = self.h[b];
                best_b = Some(b);
            }
        }

        if let Some(b) = best_b {
            while self.h[b] > 0 {
                println!("-1 {}", b);
                self.h[b] -= 1;
            }
            self.mark_as_opened(b);
        }
    }

    fn process_weapons(&mut self) {
        while let Some(w) = self.weapon_queue.pop_front() {
            while self.durabilities[w] > 0 {
                if let Some(target_b) = self.find_best_target(w) {
                    println!("{} {}", w, target_b);
                    self.h[target_b] -= self.a[w][target_b];
                    self.durabilities[w] -= 1;

                    if self.h[target_b] <= 0 {
                        self.mark_as_opened(target_b);
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn find_best_target(
        &self,
        w: usize,
    ) -> Option<usize> {
        let mut best_b = None;
        let mut max_damage = -1;

        for b in 0..self.n {
            if !self.opened[b] && self.a[w][b] > max_damage {
                max_damage = self.a[w][b];
                best_b = Some(b);
            }
        }
        best_b
    }

    fn mark_as_opened(
        &mut self,
        b: usize,
    ) {
        if !self.opened[b] {
            self.opened[b] = true;
            self.open_count += 1;
            self.weapon_queue.push_back(b);
            md!(self.open_count, b);
        }
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
