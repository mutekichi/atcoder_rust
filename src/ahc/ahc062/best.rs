#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proconio::input;
use std::io::{BufWriter, Write, stdout};
use std::time::Instant;

// --- CONSTANTS & HYPERPARAMETERS ---
const TIME_LIMIT_SEC: f64 = 2.92;

// SA Temperatures for Phase 1 (2-opt)
const T0_P1: f64 = 5e5;
const T1_P1: f64 = 1e2;

// SA Temperatures for Phase 2 (Or-opt)
const T0_P2: f64 = 1e5;
const T1_P2: f64 = 1e1;

const OR_OPT_2_PROB: f64 = 0.1; // Probability of selecting 2-node Or-opt
const SAMPLE_ATTEMPTS: usize = 15; // Attempts to find biased high-value nodes

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
    avg_a: f64,
    smooth_a: Vec<f64>,
    neighbors: Vec<Vec<u32>>,
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

        let mut sum_a_val = 0;
        for &val in &a {
            sum_a_val += val;
        }
        let avg_a = sum_a_val as f64 / v as f64;

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
                            neighbors[u].push((nr as u32) * (n as u32) + (nc as u32));
                        }
                    }
                }
            }
        }

        let mut smooth_a = vec![0.0; v];
        for u in 0..v {
            let mut sum_nbr = 0;
            for &nb in &neighbors[u] {
                sum_nbr += a[nb as usize];
            }
            smooth_a[u] = 3.0 * a[u] as f64 + (sum_nbr as f64 / neighbors[u].len() as f64);
        }

        Self {
            n,
            v,
            a,
            avg_a,
            smooth_a,
            neighbors,
        }
    }

    #[inline]
    fn is_adj(
        &self,
        u: u32,
        v: u32,
    ) -> bool {
        let r1 = u as usize / self.n;
        let c1 = u as usize % self.n;
        let r2 = v as usize / self.n;
        let c2 = v as usize % self.n;
        r1.abs_diff(r2) <= 1 && c1.abs_diff(c2) <= 1
    }
}

fn generate_initial_path(
    input: &Input,
    rng: &mut XorShift,
) -> Vec<u32> {
    let mut high_score_nodes: Vec<u32> = (0..input.v as u32).collect();
    high_score_nodes.sort_by(|&a, &b| {
        input.smooth_a[b as usize]
            .partial_cmp(&input.smooth_a[a as usize])
            .unwrap()
    });
    let candidates_count = 100.min(input.v);

    let start_time = Instant::now();
    let mut visited = vec![false; input.v];
    loop {
        if start_time.elapsed().as_secs_f64() > 0.4 {
            break;
        }
        for v in &mut visited {
            *v = false;
        }
        let mut path = Vec::with_capacity(input.v);
        let start_node = high_score_nodes[rng.gen_range(0, candidates_count)];
        let mut curr = start_node;
        visited[curr as usize] = true;
        path.push(curr);

        while path.len() < input.v {
            let mut min_deg_val = 9;
            let mut best_v = u32::MAX;
            let mut tied = 0;
            for &v in &input.neighbors[curr as usize] {
                if !visited[v as usize] {
                    let mut deg = 0;
                    for &w in &input.neighbors[v as usize] {
                        if !visited[w as usize] {
                            deg += 1;
                        }
                    }
                    if deg < min_deg_val {
                        min_deg_val = deg;
                        best_v = v;
                        tied = 1;
                    } else if deg == min_deg_val {
                        tied += 1;
                        if rng.gen_range(0, tied) == 0 {
                            best_v = v;
                        }
                    }
                }
            }
            if best_v == u32::MAX {
                break;
            }
            visited[best_v as usize] = true;
            path.push(best_v);
            curr = best_v;
        }
        if path.len() == input.v {
            path.reverse();
            return path;
        }
    }
    (0..input.v as u32).collect()
}

struct State {
    order: Vec<u32>,
    pos: Vec<u32>,
    score: i64,
    sum_a: Vec<i64>,
    sum_ka: Vec<i64>,
}

impl State {
    fn new(
        input: &Input,
        order: Vec<u32>,
    ) -> Self {
        let v = input.v;
        let mut pos = vec![0; v];
        let mut sum_a = vec![0; v + 1];
        let mut sum_ka = vec![0; v + 1];
        let mut score = 0;
        for (k, &u) in order.iter().enumerate() {
            pos[u as usize] = k as u32;
            score += (k as i64) * input.a[u as usize];
            sum_a[k + 1] = sum_a[k] + input.a[u as usize];
            sum_ka[k + 1] = sum_ka[k] + (k as i64) * input.a[u as usize];
        }
        Self {
            order,
            pos,
            score,
            sum_a,
            sum_ka,
        }
    }

    fn update_prefix_sums(
        &mut self,
        from: usize,
        input: &Input,
    ) {
        for k in from + 1..=input.v {
            let u = self.order[k - 1];
            self.sum_a[k] = self.sum_a[k - 1] + input.a[u as usize];
            self.sum_ka[k] = self.sum_ka[k - 1] + (k as i64 - 1) * input.a[u as usize];
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
        if !input.is_adj(self.order[l + 1], self.order[r + 1]) {
            return None;
        }
        let s = (l + r + 1) as i64;
        let range_sum_a = self.sum_a[r + 1] - self.sum_a[l + 1];
        let range_sum_ka = self.sum_ka[r + 1] - self.sum_ka[l + 1];
        Some(s * range_sum_a - 2 * range_sum_ka)
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
            self.pos[self.order[k] as usize] = k as u32;
        }
        self.score += diff;
        self.update_prefix_sums(l, input);
    }

    #[inline]
    fn get_diff_or_opt(
        &self,
        idx_b: usize,
        target_idx: usize,
        input: &Input,
    ) -> Option<i64> {
        if idx_b == 0 || idx_b == input.v - 1 || idx_b == target_idx || idx_b == target_idx + 1 {
            return None;
        }
        if !input.is_adj(self.order[idx_b - 1], self.order[idx_b + 1]) {
            return None;
        }
        if !input.is_adj(self.order[target_idx], self.order[idx_b]) {
            return None;
        }
        if target_idx + 1 < input.v && !input.is_adj(self.order[idx_b], self.order[target_idx + 1])
        {
            return None;
        }
        let b_val = input.a[self.order[idx_b] as usize];
        let k_b = idx_b as i64;
        let k_target = if idx_b < target_idx {
            target_idx as i64
        } else {
            (target_idx + 1) as i64
        };
        let diff = if idx_b < target_idx {
            let range_sum_a = self.sum_a[target_idx + 1] - self.sum_a[idx_b + 1];
            (k_target - k_b) * b_val - range_sum_a
        } else {
            let range_sum_a = self.sum_a[idx_b] - self.sum_a[target_idx + 1];
            (k_target - k_b) * b_val + range_sum_a
        };
        Some(diff)
    }

    fn apply_or_opt(
        &mut self,
        idx_b: usize,
        target_idx: usize,
        diff: i64,
        input: &Input,
    ) {
        let b = self.order.remove(idx_b);
        let new_idx = if idx_b < target_idx {
            target_idx
        } else {
            target_idx + 1
        };
        self.order.insert(new_idx, b);
        let start_upd = if idx_b < new_idx { idx_b } else { new_idx };
        for k in start_upd..self.order.len() {
            self.pos[self.order[k] as usize] = k as u32;
        }
        self.score += diff;
        self.update_prefix_sums(start_upd, input);
    }

    #[inline]
    fn get_diff_or_opt_2(
        &self,
        idx_b: usize,
        target_idx: usize,
        input: &Input,
    ) -> Option<i64> {
        if idx_b == 0
            || idx_b >= input.v - 2
            || target_idx == idx_b
            || target_idx == idx_b + 1
            || target_idx == idx_b + 2
        {
            return None;
        }
        let b1 = self.order[idx_b];
        let b2 = self.order[idx_b + 1];
        if !input.is_adj(self.order[idx_b - 1], self.order[idx_b + 2]) {
            return None;
        }
        if !input.is_adj(self.order[target_idx], b1) {
            return None;
        }
        if target_idx + 1 < input.v && !input.is_adj(b2, self.order[target_idx + 1]) {
            return None;
        }
        let sum_b = input.a[b1 as usize] + input.a[b2 as usize];
        let k_b = idx_b as i64;
        let k_target = if idx_b < target_idx {
            (target_idx - 1) as i64
        } else {
            (target_idx + 1) as i64
        };
        let diff = if idx_b < target_idx {
            let range_sum_a = self.sum_a[target_idx + 1] - self.sum_a[idx_b + 2];
            (k_target - k_b) * sum_b - 2 * range_sum_a
        } else {
            let range_sum_a = self.sum_a[idx_b] - self.sum_a[target_idx + 1];
            (k_target - k_b) * sum_b + 2 * range_sum_a
        };
        Some(diff)
    }

    fn apply_or_opt_2(
        &mut self,
        idx_b: usize,
        target_idx: usize,
        diff: i64,
        input: &Input,
    ) {
        let b2 = self.order.remove(idx_b + 1);
        let b1 = self.order.remove(idx_b);
        let new_idx = if idx_b < target_idx {
            target_idx - 1
        } else {
            target_idx + 1
        };
        self.order.insert(new_idx, b1);
        self.order.insert(new_idx + 1, b2);
        let start_upd = if idx_b < new_idx { idx_b } else { new_idx };
        for k in start_upd..self.order.len() {
            self.pos[self.order[k] as usize] = k as u32;
        }
        self.score += diff;
        self.update_prefix_sums(start_upd, input);
    }
}

fn main() {
    let start_time = Instant::now();
    let input = Input::new();
    let mut rng = XorShift::new(42);

    let initial_order = generate_initial_path(&input, &mut rng);
    let mut current_state = State::new(&input, initial_order);
    let mut best_state_order = current_state.order.clone();
    let mut best_score = current_state.score;

    let mut iter_count: u64 = 0;
    let mut accepted_count: u64 = 0;

    while start_time.elapsed().as_secs_f64() < TIME_LIMIT_SEC {
        iter_count += 1;
        if (iter_count & 0x3FF) == 0 && start_time.elapsed().as_secs_f64() >= TIME_LIMIT_SEC {
            break;
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let progress = elapsed / TIME_LIMIT_SEC;

        if progress < 0. {
            // Phase 1: 2-opt SA
            let temp = T0_P1 * (T1_P1 / T0_P1).powf(progress * 2.0);
            let mut l = rng.gen_range(0, input.v - 2);
            for _ in 0..SAMPLE_ATTEMPTS {
                if l < input.v / 2 && input.a[current_state.order[l] as usize] as f64 > input.avg_a
                {
                    break;
                }
                l = rng.gen_range(0, input.v - 2);
            }
            let u = current_state.order[l];
            let nbrs = &input.neighbors[u as usize];
            let target_v = nbrs[rng.gen_range(0, nbrs.len())];
            let r = current_state.pos[target_v as usize] as usize;

            if let Some(diff) = current_state.get_diff_2opt(l, r, &input) {
                if diff >= 0 || rng.next_f64() < (diff as f64 / temp).exp() {
                    current_state.apply_2opt(l, r, diff, &input);
                    accepted_count += 1;
                    if current_state.score > best_score {
                        best_score = current_state.score;
                        best_state_order = current_state.order.clone();
                    }
                }
            }
        } else {
            // Phase 2: Or-opt SA
            let temp = T0_P2 * (T1_P2 / T0_P2).powf((progress - 0.5) * 2.0);
            let mut idx_b = rng.gen_range(1, input.v - 2);
            for _ in 0..SAMPLE_ATTEMPTS {
                if idx_b < input.v / 2
                    && input.a[current_state.order[idx_b] as usize] as f64 > input.avg_a
                {
                    break;
                }
                idx_b = rng.gen_range(1, input.v - 2);
            }

            if rng.next_f64() > OR_OPT_2_PROB {
                let b = current_state.order[idx_b];
                let nbrs = &input.neighbors[b as usize];
                let target_node = nbrs[rng.gen_range(0, nbrs.len())];
                let target_idx = current_state.pos[target_node as usize] as usize;
                if let Some(diff) = current_state.get_diff_or_opt(idx_b, target_idx, &input) {
                    if diff >= 0 || rng.next_f64() < (diff as f64 / temp).exp() {
                        current_state.apply_or_opt(idx_b, target_idx, diff, &input);
                        accepted_count += 1;
                        if current_state.score > best_score {
                            best_score = current_state.score;
                            best_state_order = current_state.order.clone();
                        }
                    }
                }
            } else {
                let b1 = current_state.order[idx_b];
                let nbrs = &input.neighbors[b1 as usize];
                let target_node = nbrs[rng.gen_range(0, nbrs.len())];
                let target_idx = current_state.pos[target_node as usize] as usize;
                if let Some(diff) = current_state.get_diff_or_opt_2(idx_b, target_idx, &input) {
                    if diff >= 0 || rng.next_f64() < (diff as f64 / temp).exp() {
                        current_state.apply_or_opt_2(idx_b, target_idx, diff, &input);
                        accepted_count += 1;
                        if current_state.score > best_score {
                            best_score = current_state.score;
                            best_state_order = current_state.order.clone();
                        }
                    }
                }
            }
        }
    }

    eprintln!("Total iterations: {}", iter_count);
    eprintln!("Accepted transitions: {}", accepted_count);
    eprintln!("Best score: {}", best_score);

    let mut out = BufWriter::new(stdout());
    for &u in &best_state_order {
        writeln!(out, "{} {}", u / input.n as u32, u % input.n as u32).unwrap();
    }
}
