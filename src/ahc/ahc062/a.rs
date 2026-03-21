use proconio::input;
use std::io::{BufWriter, Write, stdout};
use std::time::Instant;

const TIME_LIMIT_SEC: f64 = 2.92;

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

fn generate_refined_path(
    input: &Input,
    rng: &mut XorShift,
) -> Vec<u32> {
    let start_time = Instant::now();
    let mut best_path = Vec::new();
    let mut max_score = i64::MIN;

    let mut visited = vec![false; input.v];
    let mut deg = vec![0; input.v];

    loop {
        // Build initial solutions until time threshold
        if start_time.elapsed().as_secs_f64() > 0.4 {
            if !best_path.is_empty() {
                return best_path;
            }
            break;
        }

        for i in 0..input.v {
            visited[i] = false;
            deg[i] = input.neighbors[i].len() as u32;
        }

        let mut path = Vec::with_capacity(input.v);
        let start_node = rng.gen_range(0, input.v) as u32;

        let mut curr = start_node;
        visited[curr as usize] = true;
        path.push(curr);

        for &nb in &input.neighbors[curr as usize] {
            deg[nb as usize] -= 1;
        }

        let mut dead_end = false;

        while path.len() < input.v {
            let mut next_node = u32::MAX;

            // Pencil puzzle logic 1: Forced moves
            let mut forced_candidates = 0;
            for &nb in &input.neighbors[curr as usize] {
                if !visited[nb as usize] {
                    if deg[nb as usize] == 1 {
                        next_node = nb;
                        forced_candidates += 1;
                    }
                }
            }

            if forced_candidates > 1 {
                // Contradiction: Multiple forced paths
                dead_end = true;
                break;
            }

            if forced_candidates == 0 {
                // Pencil puzzle logic 2: Free moves (Warnsdorff + Look-ahead + Score)
                let mut min_deg = 9;
                let mut best_eval = f64::INFINITY;

                for &v in &input.neighbors[curr as usize] {
                    if !visited[v as usize] {
                        // Look-ahead: Prevent isolating other neighbors
                        let mut will_isolate = false;
                        if path.len() < input.v - 2 {
                            for &v_nb in &input.neighbors[v as usize] {
                                if !visited[v_nb as usize] && v_nb != v {
                                    if deg[v_nb as usize] == 1 {
                                        will_isolate = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if will_isolate {
                            continue;
                        }

                        let d = deg[v as usize];
                        let eval = input.smooth_a[v as usize];

                        if d < min_deg || (d == min_deg && eval < best_eval) {
                            min_deg = d;
                            best_eval = eval;
                            next_node = v;
                        }
                    }
                }
            }

            // Fallback
            if next_node == u32::MAX {
                let mut min_deg = 9;
                let mut best_eval = f64::INFINITY;
                for &v in &input.neighbors[curr as usize] {
                    if !visited[v as usize] {
                        let d = deg[v as usize];
                        let eval = input.smooth_a[v as usize];
                        if d < min_deg || (d == min_deg && eval < best_eval) {
                            min_deg = d;
                            best_eval = eval;
                            next_node = v;
                        }
                    }
                }

                if next_node == u32::MAX {
                    dead_end = true;
                    break;
                }
            }

            curr = next_node;
            visited[curr as usize] = true;
            path.push(curr);

            // Update degrees and detect contradictions early
            let mut contradiction = false;
            for &nb in &input.neighbors[curr as usize] {
                if !visited[nb as usize] {
                    deg[nb as usize] -= 1;
                    if deg[nb as usize] == 0 && path.len() < input.v {
                        contradiction = true; // Complete isolation detected
                    }
                }
            }

            if contradiction {
                dead_end = true;
                break;
            }
        }

        if !dead_end && path.len() == input.v {
            let mut score = 0;
            for (k, &u) in path.iter().enumerate() {
                score += (k as i64) * input.a[u as usize];
            }
            if score > max_score {
                max_score = score;
                best_path = path;
            }
        }
    }

    if best_path.is_empty() {
        (0..input.v as u32).collect()
    } else {
        best_path
    }
}

#[derive(Clone)]
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

fn local_search(
    state: &mut State,
    input: &Input,
    rng: &mut XorShift,
    start_time: Instant,
    time_limit: f64,
) {
    let mut no_improve = 0;
    let max_no_improve = 15000;

    while no_improve < max_no_improve {
        if (no_improve & 0x3FF) == 0 && start_time.elapsed().as_secs_f64() >= time_limit {
            break;
        }

        let mut improved = false;
        let op = rng.gen_range(0, 3);

        if op == 0 {
            let mut l = rng.gen_range(0, input.v - 2);
            for _ in 0..10 {
                if l < input.v / 2 && input.a[state.order[l] as usize] as f64 > input.avg_a {
                    break;
                }
                l = rng.gen_range(0, input.v - 2);
            }

            let u = state.order[l];
            let nbrs = &input.neighbors[u as usize];
            let target_v = nbrs[rng.gen_range(0, nbrs.len())];
            let r = state.pos[target_v as usize] as usize;

            if let Some(diff) = state.get_diff_2opt(l, r, input) {
                if diff > 0 {
                    state.apply_2opt(l, r, diff, input);
                    improved = true;
                }
            }
        } else if op == 1 {
            let mut idx_b = rng.gen_range(1, input.v - 2);
            for _ in 0..10 {
                if idx_b < input.v / 2 && input.a[state.order[idx_b] as usize] as f64 > input.avg_a
                {
                    break;
                }
                idx_b = rng.gen_range(1, input.v - 2);
            }

            let b = state.order[idx_b];
            let nbrs = &input.neighbors[b as usize];
            let target_node = nbrs[rng.gen_range(0, nbrs.len())];
            let target_idx = state.pos[target_node as usize] as usize;

            if let Some(diff) = state.get_diff_or_opt(idx_b, target_idx, input) {
                if diff > 0 {
                    state.apply_or_opt(idx_b, target_idx, diff, input);
                    improved = true;
                }
            }
        } else {
            let mut idx_b = rng.gen_range(1, input.v - 2);
            for _ in 0..10 {
                if idx_b < input.v / 2 && input.a[state.order[idx_b] as usize] as f64 > input.avg_a
                {
                    break;
                }
                idx_b = rng.gen_range(1, input.v - 2);
            }

            let b1 = state.order[idx_b];
            let nbrs = &input.neighbors[b1 as usize];
            let target_node = nbrs[rng.gen_range(0, nbrs.len())];
            let target_idx = state.pos[target_node as usize] as usize;

            if let Some(diff) = state.get_diff_or_opt_2(idx_b, target_idx, input) {
                if diff > 0 {
                    state.apply_or_opt_2(idx_b, target_idx, diff, input);
                    improved = true;
                }
            }
        }

        if improved {
            no_improve = 0;
        } else {
            no_improve += 1;
        }
    }
}

fn kick(
    state: &mut State,
    input: &Input,
    rng: &mut XorShift,
) {
    let kick_steps = 10;
    let mut applied = 0;

    while applied < kick_steps {
        let op = rng.gen_range(0, 3);

        if op == 0 {
            let l = rng.gen_range(0, input.v - 2);
            let u = state.order[l];
            let nbrs = &input.neighbors[u as usize];
            let target_v = nbrs[rng.gen_range(0, nbrs.len())];
            let r = state.pos[target_v as usize] as usize;
            if let Some(diff) = state.get_diff_2opt(l, r, input) {
                state.apply_2opt(l, r, diff, input);
                applied += 1;
            }
        } else if op == 1 {
            let idx_b = rng.gen_range(1, input.v - 2);
            let b = state.order[idx_b];
            let nbrs = &input.neighbors[b as usize];
            let target_node = nbrs[rng.gen_range(0, nbrs.len())];
            let target_idx = state.pos[target_node as usize] as usize;
            if let Some(diff) = state.get_diff_or_opt(idx_b, target_idx, input) {
                state.apply_or_opt(idx_b, target_idx, diff, input);
                applied += 1;
            }
        } else {
            let idx_b = rng.gen_range(1, input.v - 2);
            let b1 = state.order[idx_b];
            let nbrs = &input.neighbors[b1 as usize];
            let target_node = nbrs[rng.gen_range(0, nbrs.len())];
            let target_idx = state.pos[target_node as usize] as usize;
            if let Some(diff) = state.get_diff_or_opt_2(idx_b, target_idx, input) {
                state.apply_or_opt_2(idx_b, target_idx, diff, input);
                applied += 1;
            }
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let input = Input::new();
    let mut rng = XorShift::new(42);

    let initial_order = generate_refined_path(&input, &mut rng);
    let mut current_state = State::new(&input, initial_order);

    local_search(
        &mut current_state,
        &input,
        &mut rng,
        start_time,
        TIME_LIMIT_SEC,
    );
    let mut best_state = current_state.clone();

    while start_time.elapsed().as_secs_f64() < TIME_LIMIT_SEC {
        let mut next_state = best_state.clone();

        kick(&mut next_state, &input, &mut rng);
        local_search(
            &mut next_state,
            &input,
            &mut rng,
            start_time,
            TIME_LIMIT_SEC,
        );

        if next_state.score > best_state.score {
            best_state = next_state;
        }
    }

    eprintln!("Best score: {}", best_state.score);

    let mut out = BufWriter::new(stdout());
    for &u in &best_state.order {
        writeln!(out, "{} {}", u / input.n as u32, u % input.n as u32).unwrap();
    }
}
