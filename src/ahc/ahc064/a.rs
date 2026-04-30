#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proconio::input;
use std::io::{BufWriter, Write, stdout};
use std::time::Instant;

// Fast RNG
struct XorShift {
    state: u64,
}

impl XorShift {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x123456789abcdef } else { seed },
        }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn shuffle<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 { return; }
        for i in (1..len).rev() {
            let j = (self.next() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

// Stack allocated array to prevent heap allocations during simulation
#[derive(Clone, Copy)]
struct FixedStack {
    data: [u8; 32],
    len: usize,
}

impl FixedStack {
    fn new() -> Self {
        Self { data: [0; 32], len: 0 }
    }

    fn from_slice(slice: &[usize]) -> Self {
        let mut s = Self::new();
        for &v in slice {
            s.data[s.len] = v as u8;
            s.len += 1;
        }
        s
    }

    fn pop_back_k(&mut self, k: usize, out: &mut [u8]) {
        self.len -= k;
        out[..k].copy_from_slice(&self.data[self.len..self.len + k]);
    }

    fn push_front_k(&mut self, k: usize, vals: &[u8]) {
        self.data.copy_within(0..self.len, k);
        self.data[0..k].copy_from_slice(&vals[..k]);
        self.len += k;
    }

    fn pop_front_k(&mut self, k: usize, out: &mut [u8]) {
        out[..k].copy_from_slice(&self.data[0..k]);
        self.data.copy_within(k..self.len, 0);
        self.len -= k;
    }

    fn push_back_k(&mut self, k: usize, vals: &[u8]) {
        self.data[self.len..self.len + k].copy_from_slice(&vals[..k]);
        self.len += k;
    }
}

fn main() {
    let start_time = Instant::now();
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out, start_time);

    out.flush().unwrap();
}

fn solve<W: Write>(out: &mut W, start_time: Instant) {
    input! {
        r: usize,
        y: [[usize; 10]; r],
    }

    let mut best_ans = Vec::new();
    let mut best_turn_count = usize::MAX;
    let mut seed = 1;

    // Buffers to be reused
    let mut starts_buf = [FixedStack::new(); 10];
    let mut avoids_buf = [FixedStack::new(); 10];
    let mut order_i = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut order_j = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    while start_time.elapsed().as_millis() < 1980 {
        if let Some(ans_turns) = run_simulation(
            r, &y, seed, &mut starts_buf, &mut avoids_buf, &mut order_i, &mut order_j
        ) {
            if ans_turns.len() < best_turn_count {
                best_turn_count = ans_turns.len();
                best_ans = ans_turns;
            }
        }
        seed += 1;
    }

    if best_ans.is_empty() {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{}", best_ans.len()).unwrap();
    for turn in best_ans {
        writeln!(out, "{}", turn.len()).unwrap();
        for &(type_op, i, j, k) in &turn {
            let from_idx = if type_op == 0 { i } else { j };
            let to_idx = if type_op == 0 { j } else { i };
            writeln!(out, "{} {} {} {}", type_op, from_idx, to_idx, k).unwrap();
        }
    }
}

fn run_simulation(
    r: usize,
    initial_y: &[Vec<usize>],
    seed: u64,
    starts: &mut [FixedStack; 10],
    avoids: &mut [FixedStack; 10],
    order_i: &mut [usize; 10],
    order_j: &mut [usize; 10],
) -> Option<Vec<Vec<(u8, usize, usize, usize)>>> {
    let mut rng = XorShift::new(seed);
    
    for i in 0..r {
        starts[i] = FixedStack::from_slice(&initial_y[i]);
        avoids[i] = FixedStack::new();
    }

    let mut ans_turns = Vec::with_capacity(200);
    let mut done_count = [0usize; 10];
    let mut turn_ops = Vec::with_capacity(10);
    let mut pairs = Vec::with_capacity(10);
    let mut temp_vals = [0u8; 32];

    loop {
        let mut all_done = true;
        for i in 0..r {
            done_count[i] = 0;
            let target_base = (i * 10) as u8;
            while done_count[i] < starts[i].len 
                && starts[i].data[done_count[i]] == target_base + (done_count[i] as u8) 
            {
                done_count[i] += 1;
            }
            if done_count[i] < 10 {
                all_done = false;
            }
        }

        if all_done { break; }
        if ans_turns.len() >= 4000 { return None; }

        turn_ops.clear();
        pairs.clear();
        let mut used_i = 0u16;
        let mut used_j = 0u16;

        let mut req_ids = [None; 10];
        for i in 0..r {
            if done_count[i] < 10 {
                req_ids[i] = Some((i * 10 + done_count[i]) as u8);
            }
        }

        let mut is_avoid_safe = 0u16;
        for j in 0..r {
            if avoids[j].len > 0 {
                let mut safe = true;
                for i in 0..r {
                    if req_ids[i] == Some(avoids[j].data[0]) {
                        safe = false;
                        break;
                    }
                }
                if safe {
                    is_avoid_safe |= 1 << j;
                }
            } else {
                is_avoid_safe |= 1 << j;
            }
        }

        rng.shuffle(order_i);
        rng.shuffle(order_j);

        // Priority 1
        for &target_i in order_i.iter() {
            if done_count[target_i] == 10 || (used_i & (1 << target_i)) != 0 { continue; }
            if starts[target_i].len > done_count[target_i] { continue; }

            let Some(req_id) = req_ids[target_i] else { continue; };

            let mut best_j = None;
            let mut best_k = 0;

            for &j in order_j.iter() {
                if (used_j & (1 << j)) != 0 || !can_add_pair_fast(target_i, j, &pairs) { continue; }

                let av = &avoids[j];
                if av.len == 0 { continue; }

                let mut k = 0;
                while k < av.len && av.data[k] == req_id + (k as u8) {
                    k += 1;
                }

                if k > 0 && starts[target_i].len + k <= 15 {
                    if k > best_k {
                        best_k = k;
                        best_j = Some(j);
                    }
                }
            }

            if let Some(j) = best_j {
                turn_ops.push((1, target_i, j, best_k));
                used_i |= 1 << target_i;
                used_j |= 1 << j;
                pairs.push((target_i, j));
            }
        }

        // Priority 2
        for &target_i in order_i.iter() {
            if done_count[target_i] == 10 || (used_i & (1 << target_i)) != 0 || starts[target_i].len <= done_count[target_i] {
                continue;
            }

            let Some(req_id) = req_ids[target_i] else { continue; };

            let mut req_at_avoid_top = false;
            for &j in order_j.iter() {
                if avoids[j].len > 0 && avoids[j].data[0] == req_id {
                    req_at_avoid_top = true;
                    break;
                }
            }

            if req_at_avoid_top {
                let available_k = starts[target_i].len - done_count[target_i];
                let mut best_j = None;
                let mut best_k = 0;
                let mut max_free = -1;

                for &j in order_j.iter() {
                    if (used_j & (1 << j)) != 0 || !can_add_pair_fast(target_i, j, &pairs) || (is_avoid_safe & (1 << j)) == 0 { continue; }

                    let free = 20 - avoids[j].len;
                    let k = available_k.min(free);
                    if k > 0 && (free as i32) > max_free {
                        max_free = free as i32;
                        best_k = k;
                        best_j = Some(j);
                    }
                }
                if let Some(j) = best_j {
                    turn_ops.push((0, target_i, j, best_k));
                    used_i |= 1 << target_i;
                    used_j |= 1 << j;
                    pairs.push((target_i, j));
                }
            }
        }

        // Priority 2.5
        for &j in order_j.iter() {
            if (used_j & (1 << j)) != 0 || avoids[j].len == 0 { continue; }

            let av_top = avoids[j].data[0];
            if av_top % 10 == 0 { continue; }
            let req_id = av_top - 1;

            let mut best_i = None;
            let mut best_k = 0;

            for &i in order_i.iter() {
                if (used_i & (1 << i)) != 0 || !can_add_pair_fast(i, j, &pairs) { continue; }
                if starts[i].len <= done_count[i] { continue; }

                let unconfirmed_len = starts[i].len - done_count[i];
                let free = 20 - avoids[j].len;
                let max_possible_k = unconfirmed_len.min(free).min((req_id % 10) as usize + 1);

                let mut max_k = 0;
                for k in 1..=max_possible_k {
                    let mut ok = true;
                    let start_idx = starts[i].len - k;
                    for l in 0..k {
                        if starts[i].data[start_idx + l] != req_id - (k as u8 - 1) + (l as u8) {
                            ok = false;
                            break;
                        }
                    }
                    if ok { max_k = k; }
                }

                if max_k > 0 && max_k > best_k {
                    best_k = max_k;
                    best_i = Some(i);
                }
            }

            if let Some(i) = best_i {
                turn_ops.push((0, i, j, best_k));
                used_i |= 1 << i;
                used_j |= 1 << j;
                pairs.push((i, j));
            }
        }

        // Priority 3
        for &target_i in order_i.iter() {
            if done_count[target_i] == 10 { continue; }
            let Some(req_id) = req_ids[target_i] else { continue; };

            for &i in order_i.iter() {
                if (used_i & (1 << i)) != 0 || i == target_i { continue; }
                if let Some(pos) = starts[i].data[0..starts[i].len].iter().position(|&x| x == req_id) {
                    let available_k = starts[i].len - pos;
                    if available_k > 0 {
                        let mut best_j = None;
                        let mut best_k = 0;
                        let mut max_free = -1;

                        for &j in order_j.iter() {
                            if (used_j & (1 << j)) != 0 || !can_add_pair_fast(i, j, &pairs) || (is_avoid_safe & (1 << j)) == 0 { continue; }

                            let free = 20 - avoids[j].len;
                            let k = available_k.min(free);
                            if k > 0 && (free as i32) > max_free {
                                max_free = free as i32;
                                best_k = k;
                                best_j = Some(j);
                            }
                        }
                        if let Some(j) = best_j {
                            turn_ops.push((0, i, j, best_k));
                            used_i |= 1 << i;
                            used_j |= 1 << j;
                            pairs.push((i, j));
                            break;
                        }
                    }
                }
            }
        }

        // Priority 4
        for &target_i in order_i.iter() {
            if done_count[target_i] == 10 { continue; }
            let Some(req_id) = req_ids[target_i] else { continue; };

            for &j in order_j.iter() {
                if (used_j & (1 << j)) != 0 { continue; }
                if let Some(pos) = avoids[j].data[0..avoids[j].len].iter().position(|&x| x == req_id) {
                    let available_k = pos;
                    if available_k > 0 {
                        let mut best_i = None;
                        let mut best_k = 0;
                        let mut max_free = -1;

                        for &i in order_i.iter() {
                            if (used_i & (1 << i)) != 0 || i == target_i || !can_add_pair_fast(i, j, &pairs) { continue; }
                            
                            let free = 15 - starts[i].len;
                            let k = available_k.min(free);
                            if k > 0 && (free as i32) > max_free {
                                max_free = free as i32;
                                best_k = k;
                                best_i = Some(i);
                            }
                        }
                        if let Some(i) = best_i {
                            turn_ops.push((1, i, j, best_k));
                            used_i |= 1 << i;
                            used_j |= 1 << j;
                            pairs.push((i, j));
                            break;
                        }
                    }
                }
            }
        }

        // Fallback
        if turn_ops.is_empty() {
            let mut moved = false;

            for is_safe_pass in [true, false] {
                for &i in order_i.iter() {
                    if (used_i & (1 << i)) != 0 || starts[i].len <= done_count[i] { continue; }
                    let available_k = starts[i].len - done_count[i];

                    for &j in order_j.iter() {
                        if (used_j & (1 << j)) != 0 || !can_add_pair_fast(i, j, &pairs) { continue; }
                        if is_safe_pass && (is_avoid_safe & (1 << j)) == 0 { continue; }

                        let free = 20 - avoids[j].len;
                        let k = available_k.min(free);
                        if k > 0 {
                            turn_ops.push((0, i, j, k));
                            used_i |= 1 << i;
                            used_j |= 1 << j;
                            pairs.push((i, j));
                            moved = true;
                            break;
                        }
                    }
                    if moved { break; }
                }

                if !moved {
                    for &j in order_j.iter() {
                        if (used_j & (1 << j)) != 0 || avoids[j].len == 0 { continue; }
                        let available_k = avoids[j].len;
                        
                        for &i in order_i.iter() {
                            if (used_i & (1 << i)) != 0 || !can_add_pair_fast(i, j, &pairs) { continue; }
                            
                            let free = 15 - starts[i].len;
                            let k = available_k.min(free);
                            if k > 0 {
                                turn_ops.push((1, i, j, k));
                                used_i |= 1 << i;
                                used_j |= 1 << j;
                                pairs.push((i, j));
                                moved = true;
                                break;
                            }
                        }
                        if moved { break; }
                    }
                }

                if moved { break; }
            }

            if !moved { break; }
        }

        // Apply operations
        for &(type_op, i, j, k) in &turn_ops {
            if type_op == 0 {
                starts[i].pop_back_k(k, &mut temp_vals);
                avoids[j].push_front_k(k, &temp_vals);
            } else {
                avoids[j].pop_front_k(k, &mut temp_vals);
                starts[i].push_back_k(k, &temp_vals);
            }
        }

        ans_turns.push(turn_ops.clone());
    }

    Some(ans_turns)
}

#[inline(always)]
fn can_add_pair_fast(i: usize, j: usize, pairs: &[(usize, usize)]) -> bool {
    for &(pi, pj) in pairs {
        if i == pi || j == pj {
            return false;
        }
        if (i < pi && j > pj) || (i > pi && j < pj) {
            return false;
        }
    }
    true
}