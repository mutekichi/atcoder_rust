#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proconio::input;
use std::cmp::{max, min};
use std::io::{BufWriter, Write, stdout};
use std::time::{Duration, Instant};

const TIME_LIMIT_SEC: f64 = 2.95;

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
    fn gen_range(
        &mut self,
        min: usize,
        max: usize,
    ) -> usize {
        if max <= min {
            return min;
        }
        min + (self.next() as usize % (max - min))
    }
}

struct Input {
    n: usize,
    v: usize,
    a: Vec<i64>,
    neighbors: Vec<Vec<usize>>,
}

impl Input {
    fn new() -> Self {
        input! {
            n: usize,
            a_raw: [[i64; n]; n],
        }
        let v = n * n;
        let mut a = Vec::with_capacity(v);
        for row in a_raw {
            a.extend(row);
        }

        let mut neighbors = vec![vec![]; v];
        for r in 0..n {
            for c in 0..n {
                let u = r * n + c;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < n as isize && nc >= 0 && nc < n as isize {
                            neighbors[u].push((nr as usize) * n + (nc as usize));
                        }
                    }
                }
            }
        }

        Self { n, v, a, neighbors }
    }

    #[inline]
    fn is_adj(
        &self,
        u: usize,
        v: usize,
    ) -> bool {
        let r1 = u / self.n;
        let c1 = u % self.n;
        let r2 = v / self.n;
        let c2 = v % self.n;
        r1.abs_diff(r2) <= 1 && c1.abs_diff(c2) <= 1
    }
}

// Warnsdorff's heuristic
fn generate_random_path(
    input: &Input,
    rng: &mut XorShift,
) -> Vec<usize> {
    loop {
        let mut visited = vec![false; input.v];
        let mut path = Vec::with_capacity(input.v);

        let mut curr = rng.gen_range(0, input.v);
        visited[curr] = true;
        path.push(curr);

        let mut success = true;

        while path.len() < input.v {
            let mut min_deg = usize::MAX;
            let mut candidates = Vec::new();

            for &v in &input.neighbors[curr] {
                if !visited[v] {
                    let mut deg = 0;
                    for &w in &input.neighbors[v] {
                        if !visited[w] {
                            deg += 1;
                        }
                    }

                    if deg < min_deg {
                        min_deg = deg;
                        candidates.clear();
                        candidates.push(v);
                    } else if deg == min_deg {
                        candidates.push(v);
                    }
                }
            }

            if candidates.is_empty() {
                success = false;
                break; // Restart
            }

            let next_node = candidates[rng.gen_range(0, candidates.len())];
            visited[next_node] = true;
            path.push(next_node);
            curr = next_node;
        }

        if success {
            return path;
        }
    }
}

#[derive(Clone)]
struct State {
    order: Vec<usize>,
    pos: Vec<usize>,
    score: i64,
}

impl State {
    fn new(
        input: &Input,
        order: Vec<usize>,
    ) -> Self {
        let mut pos = vec![0; input.v];
        for (i, &u) in order.iter().enumerate() {
            pos[u] = i;
        }

        let mut score = 0;
        for (k, &u) in order.iter().enumerate() {
            score += (k as i64) * input.a[u];
        }

        Self { order, pos, score }
    }

    #[inline]
    fn try_2opt(
        &mut self,
        l: usize,
        r: usize,
        input: &Input,
    ) -> Option<i64> {
        if r <= l + 1 || r + 1 >= input.v {
            return None;
        }

        let u1 = self.order[l + 1];
        let v1 = self.order[r + 1];
        if !input.is_adj(u1, v1) {
            return None;
        }

        let mut diff: i64 = 0;
        let sum_idx = r + l + 1;
        for k in (l + 1)..=r {
            let new_k = sum_idx - k;
            diff += (new_k as i64 - k as i64) * input.a[self.order[k]];
        }

        Some(diff)
    }

    #[inline]
    fn apply_2opt(
        &mut self,
        l: usize,
        r: usize,
        diff: i64,
    ) {
        self.order[l + 1..=r].reverse();
        for k in l + 1..=r {
            self.pos[self.order[k]] = k;
        }
        self.score += diff;
    }
}

fn main() {
    let start_time = Instant::now();
    let input = Input::new();
    let mut rng = XorShift::new(42);

    let initial_order = generate_random_path(&input, &mut rng);
    let mut current_state = State::new(&input, initial_order);

    let mut rev_state = current_state.clone();
    rev_state.order.reverse();
    for (i, &u) in rev_state.order.iter().enumerate() {
        rev_state.pos[u] = i;
    }
    rev_state.score = 0;
    for (k, &u) in rev_state.order.iter().enumerate() {
        rev_state.score += (k as i64) * input.a[u];
    }
    if rev_state.score > current_state.score {
        current_state = rev_state;
    }

    let mut best_state = current_state.clone();

    let t0 = 2e7;
    let t1 = 1e3;
    let mut iter_count = 0;
    let mut accepted_count = 0;

    loop {
        if (iter_count & 1023) == 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed >= TIME_LIMIT_SEC {
                break;
            }
        }
        iter_count += 1;

        let elapsed = start_time.elapsed().as_secs_f64();
        let progress = elapsed / TIME_LIMIT_SEC;
        let temp = t0 as f64 * (t1 as f64 / t0).powf(progress);

        let l = rng.gen_range(0, input.v - 2);
        let u = current_state.order[l];

        let neighbors = &input.neighbors[u];
        let v_idx = rng.gen_range(0, neighbors.len());
        let v = neighbors[v_idx];

        let r = current_state.pos[v];

        if r <= l + 1 || r + 1 >= input.v {
            continue;
        }

        if let Some(diff) = current_state.try_2opt(l, r, &input) {
            if diff >= 0 || rng.next_f64() < (diff as f64 / temp).exp() {
                current_state.apply_2opt(l, r, diff);
                accepted_count += 1;

                if current_state.score > best_state.score {
                    best_state.score = current_state.score;
                    best_state.order.clone_from(&current_state.order);
                }
            }
        }
    }

    eprintln!("Total iterations: {}", iter_count);
    eprintln!("Accepted transitions: {}", accepted_count);
    eprintln!("Best Score: {}", best_state.score);

    let mut out = BufWriter::new(stdout());
    for &u in &best_state.order {
        writeln!(out, "{} {}", u / input.n, u % input.n).unwrap();
    }
    out.flush().unwrap();
}
