#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proconio::input;
use std::cmp::{max, min};
use std::io::{BufWriter, Write, stdout};
use std::time::{Duration, Instant};

const TIME_LIMIT_SEC: f64 = 2.85;

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

fn generate_initial_path(
    input: &Input,
    rng: &mut XorShift,
) -> Vec<usize> {
    loop {
        let mut visited = vec![false; input.v];
        let mut path = Vec::with_capacity(input.v);
        let mut curr = rng.gen_range(0, input.v);
        visited[curr] = true;
        path.push(curr);

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
                break;
            }

            // Heuristic: Prefer smaller population for early path indices
            let next_node = if candidates.len() == 1 {
                candidates[0]
            } else {
                candidates.sort_by_key(|&v| input.a[v]);
                let idx = (rng.next_f64() * rng.next_f64() * candidates.len() as f64) as usize;
                candidates[idx]
            };

            visited[next_node] = true;
            path.push(next_node);
            curr = next_node;
        }
        if path.len() == input.v {
            return path;
        }
    }
}

struct State {
    order: Vec<usize>,
    pos: Vec<usize>,
    score: i64,
    sum_a: Vec<i64>,  // Prefix sum of A
    sum_ka: Vec<i64>, // Prefix sum of k*A
}

impl State {
    fn new(
        input: &Input,
        order: Vec<usize>,
    ) -> Self {
        let mut pos = vec![0; input.v];
        let mut sum_a = vec![0; input.v + 1];
        let mut sum_ka = vec![0; input.v + 1];
        let mut score = 0;
        for (k, &u) in order.iter().enumerate() {
            pos[u] = k;
            score += (k as i64) * input.a[u];
            sum_a[k + 1] = sum_a[k] + input.a[u];
            sum_ka[k + 1] = sum_ka[k] + (k as i64) * input.a[u];
        }
        Self {
            order,
            pos,
            score,
            sum_a,
            sum_ka,
        }
    }

    #[inline]
    fn get_diff_2opt(
        &self,
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

        let s = (l + r + 1) as i64;
        let range_sum_a = self.sum_a[r + 1] - self.sum_a[l + 1];
        let range_sum_ka = self.sum_ka[r + 1] - self.sum_ka[l + 1];

        // New index k' = (l + 1) + (r - k) = l + r + 1 - k = s - k
        // Diff = sum_{k=l+1}^r (k' - k) * A[k] = sum (s - 2k) * A[k] = s * sum A - 2 * sum kA
        let diff = s * range_sum_a - 2 * range_sum_ka;
        Some(diff)
    }

    fn apply_2opt(
        &mut self,
        l: usize,
        r: usize,
        diff: i64,
        input: &Input,
    ) {
        self.order[l + 1..=r].reverse();
        for k in l + 1..=r {
            let u = self.order[k];
            self.pos[u] = k;
        }
        self.score += diff;
        // Update prefix sums for the changed range
        for k in l + 1..=input.v {
            let u = self.order[k - 1];
            self.sum_a[k] = self.sum_a[k - 1] + input.a[u];
            self.sum_ka[k] = self.sum_ka[k - 1] + (k as i64 - 1) * input.a[u];
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let input = Input::new();
    let mut rng = XorShift::new(42);

    let mut best_overall_score = -1;
    let mut best_overall_order = Vec::new();

    // Multistart loop
    while start_time.elapsed().as_secs_f64() < TIME_LIMIT_SEC {
        let initial_order = generate_initial_path(&input, &mut rng);
        let mut current_state = State::new(&input, initial_order);

        // Initial check for reversal
        let mut rev_order = current_state.order.clone();
        rev_order.reverse();
        let rev_state = State::new(&input, rev_order);
        if rev_state.score > current_state.score {
            current_state = rev_state;
        }

        let t0 = 1e6;
        let t1 = 1e2;
        let mut iter_count = 0;

        while iter_count < 200000 {
            if (iter_count & 255) == 0 && start_time.elapsed().as_secs_f64() >= TIME_LIMIT_SEC {
                break;
            }
            iter_count += 1;

            let progress = start_time.elapsed().as_secs_f64() / TIME_LIMIT_SEC;
            let temp = t0 * (t1 / t0).powf(progress);

            let l = rng.gen_range(0, input.v - 2);
            let u = current_state.order[l];
            let target_v = input.neighbors[u][rng.gen_range(0, input.neighbors[u].len())];
            let r = current_state.pos[target_v];

            if let Some(diff) = current_state.get_diff_2opt(l, r, &input) {
                if diff >= 0 || rng.next_f64() < (diff as f64 / temp).exp() {
                    current_state.apply_2opt(l, r, diff, &input);
                }
            }
        }

        if current_state.score > best_overall_score {
            best_overall_score = current_state.score;
            best_overall_order = current_state.order;
        }
    }

    let mut out = BufWriter::new(stdout());
    for &u in &best_overall_order {
        writeln!(out, "{} {}", u / input.n, u % input.n).unwrap();
    }
}
