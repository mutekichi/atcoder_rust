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
use std::ops::Bound::{self, Excluded, Included, Unbounded};
use std::time::{Instant, Duration};

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
#[cfg(debug_assertions)]
macro_rules! md {
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
#[cfg(not(debug_assertions))]
macro_rules! md {
    ($($arg:expr),* $(,)?) => {{
    }};
}

// Fast PRNG
struct XorShift {
    seed: u32,
}

impl XorShift {
    fn new(seed: u32) -> Self {
        Self {
            seed: if seed == 0 { 123456789 } else { seed },
        }
    }

    #[inline]
    fn next(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u32::MAX as f64
    }

    #[inline]
    fn gen_range(&mut self, min: usize, max: usize) -> usize {
        if max <= min {
            return min;
        }
        min + (self.next() as usize % (max - min))
    }
}

// Hamiltonian path builder
struct PathGenerator {
    n: usize,
    v: usize,
    edge_count: usize,
    degree: Vec<usize>,
    other_end: Vec<usize>,
    avail: Vec<Vec<usize>>,
    adj: Vec<Vec<usize>>,
    
    bucket: Vec<Vec<usize>>,
    b_idx: Vec<isize>,
    b_pos: Vec<usize>,
    queue: Vec<usize>,

    initial_avail: Vec<Vec<usize>>,
    initial_bucket: Vec<Vec<usize>>,
    initial_b_idx: Vec<isize>,
    initial_b_pos: Vec<usize>,
}

impl PathGenerator {
    fn new(n: usize) -> Self {
        let v = n * n;
        let mut generator = Self {
            n, v, edge_count: 0,
            degree: vec![0; v],
            other_end: vec![0; v],
            avail: vec![vec![]; v],
            adj: vec![vec![]; v],
            bucket: vec![vec![]; 9],
            b_idx: vec![-1; v],
            b_pos: vec![0; v],
            queue: Vec::with_capacity(v),
            initial_avail: vec![vec![]; v],
            initial_bucket: vec![vec![]; 9],
            initial_b_idx: vec![-1; v],
            initial_b_pos: vec![0; v],
        };

        for r in 0..n {
            for c in 0..n {
                let u = r * n + c;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 { continue; }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < n as isize && nc >= 0 && nc < n as isize {
                            generator.initial_avail[u].push((nr as usize) * n + (nc as usize));
                        }
                    }
                }
                let count = generator.initial_avail[u].len();
                generator.initial_b_idx[u] = count as isize;
                generator.initial_b_pos[u] = generator.initial_bucket[count].len();
                generator.initial_bucket[count].push(u);
            }
        }
        generator
    }

    fn init(&mut self) {
        self.degree.fill(0);
        for a in &mut self.adj { a.clear(); }
        self.avail.clone_from(&self.initial_avail);
        for i in 0..self.v { self.other_end[i] = i; }
        self.edge_count = 0;

        self.bucket.clone_from(&self.initial_bucket);
        self.b_idx.clone_from(&self.initial_b_idx);
        self.b_pos.clone_from(&self.initial_b_pos);
        self.queue.clear();
    }

    fn update_bucket(&mut self, u: usize, new_count: isize) {
        if self.degree[u] >= 2 { return; }
        let old_count = self.b_idx[u];
        if old_count == new_count { return; }

        if old_count != -1 {
            let oc = old_count as usize;
            let pos = self.b_pos[u];
            let last_u = *self.bucket[oc].last().unwrap();
            self.bucket[oc][pos] = last_u;
            self.b_pos[last_u] = pos;
            self.bucket[oc].pop();
        }

        if new_count != -1 {
            let nc = new_count as usize;
            self.b_idx[u] = new_count;
            self.b_pos[u] = self.bucket[nc].len();
            self.bucket[nc].push(u);
        } else {
            self.b_idx[u] = -1;
        }
    }

    fn remove_avail(&mut self, u: usize, v: usize) -> bool {
        if let Some(pos) = self.avail[u].iter().position(|&x| x == v) {
            self.avail[u].swap_remove(pos);
            if self.degree[u] < 2 {
                self.update_bucket(u, self.avail[u].len() as isize);
            }
            let req = 2 - self.degree[u] as isize;
            if req > 0 {
                if (self.avail[u].len() as isize) == req {
                    self.queue.push(u);
                } else if (self.avail[u].len() as isize) < req {
                    return false;
                }
            }
        }
        true
    }

    fn add_edge(&mut self, u: usize, v: usize) -> bool {
        self.adj[u].push(v);
        self.adj[v].push(u);
        self.degree[u] += 1;
        self.degree[v] += 1;
        self.edge_count += 1;

        let end_u = self.other_end[u];
        let end_v = self.other_end[v];
        self.other_end[end_u] = end_v;
        self.other_end[end_v] = end_u;

        if self.edge_count < self.v - 1 {
            if !self.remove_avail(end_u, end_v) { return false; }
            if !self.remove_avail(end_v, end_u) { return false; }
        }

        if self.degree[u] == 2 {
            self.update_bucket(u, -1);
            let avail_u = self.avail[u].clone();
            for &w in &avail_u {
                if !self.remove_avail(w, u) { return false; }
            }
            self.avail[u].clear();
        } else {
            if !self.remove_avail(u, v) { return false; }
        }

        if self.degree[v] == 2 {
            self.update_bucket(v, -1);
            let avail_v = self.avail[v].clone();
            for &w in &avail_v {
                if !self.remove_avail(w, v) { return false; }
            }
            self.avail[v].clear();
        } else {
            if !self.remove_avail(v, u) { return false; }
        }

        true
    }

    fn process_queue(&mut self) -> bool {
        let mut head = 0;
        while head < self.queue.len() {
            let u = self.queue[head];
            head += 1;
            let mut req = 2 - self.degree[u] as isize;
            if req <= 0 { continue; }

            let targets = self.avail[u].clone();
            for &v in &targets {
                if req == 0 { break; }
                if self.avail[u].contains(&v) {
                    if !self.add_edge(u, v) { return false; }
                    req -= 1;
                }
            }
            if req > 0 { return false; }
        }
        self.queue.clear();
        true
    }

    fn random_connect(&mut self, rng: &mut XorShift) -> bool {
        let mut min_count = -1;
        for c in 1..=8 {
            if !self.bucket[c].is_empty() {
                min_count = c as isize;
                break;
            }
        }
        if min_count == -1 { return false; }

        let b_idx = min_count as usize;
        let u_idx = rng.gen_range(0, self.bucket[b_idx].len());
        let u = self.bucket[b_idx][u_idx];

        let v_idx = rng.gen_range(0, self.avail[u].len());
        let v = self.avail[u][v_idx];

        self.add_edge(u, v)
    }

    fn generate(&mut self, rng: &mut XorShift) -> Option<Vec<usize>> {
        self.init();
        let mut ok = true;

        while self.edge_count < self.v - 1 {
            if !self.queue.is_empty() {
                if !self.process_queue() {
                    ok = false;
                    break;
                }
            } else {
                if !self.random_connect(rng) {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            Some(self.extract_path())
        } else {
            None
        }
    }

    fn extract_path(&self) -> Vec<usize> {
        let mut path = Vec::with_capacity(self.v);
        let mut start_node = 0;
        for i in 0..self.v {
            if self.degree[i] == 1 {
                start_node = i;
                break;
            }
        }

        let mut curr = start_node;
        let mut prev = !0;
        path.push(curr);

        while path.len() < self.v {
            for &nxt in &self.adj[curr] {
                if nxt != prev {
                    prev = curr;
                    curr = nxt;
                    path.push(curr);
                    break;
                }
            }
        }
        path
    }
}

#[allow(unused_variables)]
fn main() {
    input! {
        N: usize,
        A: [[i64; N]; N],
    }

    let start_time = Instant::now();
    let time_limit = Duration::from_millis(2950);

    let mut best_score = -1;
    let mut best_path = Vec::new();
    let mut iterations = 0;
    let mut valid_count = 0;

    let mut generator = PathGenerator::new(N);
    let mut rng = XorShift::new(42);

    while start_time.elapsed() < time_limit {
        iterations += 1;
        
        if let Some(path) = generator.generate(&mut rng) {
            valid_count += 1;
            
            let mut score1 = 0;
            for (k, &u) in path.iter().enumerate() {
                let r = u / N;
                let c = u % N;
                score1 += (k as i64) * A[r][c];
            }
            if score1 > best_score {
                best_score = score1;
                best_path = path.clone();
            }

            let mut score2 = 0;
            for (k, &u) in path.iter().rev().enumerate() {
                let r = u / N;
                let c = u % N;
                score2 += (k as i64) * A[r][c];
            }
            if score2 > best_score {
                best_score = score2;
                let mut rev_path = path.clone();
                rev_path.reverse();
                best_path = rev_path;
            }
        }
    }

    eprintln!("Total iterations: {}", iterations);
    eprintln!("Valid paths generated: {}", valid_count);
    eprintln!("Best Score: {}", best_score);

    let mut out = BufWriter::new(stdout());
    if !best_path.is_empty() {
        for &u in &best_path {
            writeln!(out, "{} {}", u / N, u % N).unwrap();
        }
    } else {
        for r in 0..N {
            for c in 0..N {
                let col = if r % 2 == 0 { c } else { N - 1 - c };
                writeln!(out, "{} {}", r, col).unwrap();
            }
        }
    }
    out.flush().unwrap();
}