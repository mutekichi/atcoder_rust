#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

use proconio::input;
use std::time::Instant;

const TIME_LIMIT_SEC: f64 = 1.95;

struct Xorshift {
    seed: u32,
}

impl Xorshift {
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
        min + (self.next() as usize % (max - min))
    }
}

struct Input {
    n: usize,
    h: Vec<i32>,
    c: Vec<i32>,
    a: Vec<i32>,
}

impl Input {
    fn new() -> Self {
        input! {
            n: usize,
            h: [i32; n],
            c: [i32; n],
            a_raw: [[i32; n]; n],
        }

        let mut a: Vec<i32> = Vec::with_capacity(n * n);
        for row in a_raw.iter() {
            a.extend(row);
        }

        Self { n, h, c, a }
    }

    #[inline]
    fn get_a(
        &self,
        w: usize,
        b: usize,
    ) -> i32 {
        self.a[w * self.n + b]
    }
}

// Simulates the game based on the given order of chests to open.
fn evaluate_order(
    order: &[usize],
    input: &Input,
    get_actions: bool,
) -> (i32, Vec<(i32, usize)>) {
    let mut h = input.h.clone();
    let mut dur = input.c.clone();
    let mut active_weapons: Vec<usize> = Vec::with_capacity(input.n);
    let mut actions = Vec::new();
    let mut bare_hands_count = 0;

    for &b in order {
        // Attack chest `b` until it opens
        while h[b] > 0 {
            let mut best_w_idx = usize::MAX;
            let mut best_dmg = 0;

            // Find the most effective weapon currently available
            for (idx, &w) in active_weapons.iter().enumerate() {
                let dmg = input.get_a(w, b);

                // Simple heuristic: use weapon if it deals reasonable damage
                // In a real competition, you would tune this threshold or add more logic
                if dmg > best_dmg && dmg >= 15 {
                    best_dmg = dmg;
                    best_w_idx = idx;
                }
            }

            if best_w_idx != usize::MAX {
                // Attack with the chosen weapon
                let w = active_weapons[best_w_idx];
                let actual_dmg = h[b].min(best_dmg);
                h[b] -= actual_dmg;
                dur[w] -= 1;

                if get_actions {
                    actions.push((w as i32, b));
                }

                if dur[w] == 0 {
                    active_weapons.swap_remove(best_w_idx);
                }
            } else {
                // No effective weapon found, finish with bare hands
                bare_hands_count += h[b];
                if get_actions {
                    for _ in 0..h[b] {
                        actions.push((-1, b));
                    }
                }
                h[b] = 0;
            }
        }

        // Add the new weapon from the opened chest if it has durability
        if input.c[b] > 0 {
            active_weapons.push(b);
        }
    }

    (bare_hands_count, actions)
}

fn main() {
    let start_time = Instant::now();
    let input = Input::new();
    let mut rng = Xorshift::new(42);

    // Initial state: 0, 1, ..., N-1
    let mut current_order: Vec<usize> = (0..input.n).collect();
    // Shuffle initial order to avoid getting stuck in a bad local minimum
    for i in (1..input.n).rev() {
        let j = rng.gen_range(0, i + 1);
        current_order.swap(i, j);
    }

    let (mut current_score, _) = evaluate_order(&current_order, &input, false);
    let mut best_order = current_order.clone();
    let mut best_score = current_score;

    let t0 = 100.0;
    let t1 = 0.1;
    let mut iter_count = 0;

    loop {
        if (iter_count & 255) == 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed >= TIME_LIMIT_SEC {
                break;
            }
        }
        iter_count += 1;

        let elapsed = start_time.elapsed().as_secs_f64();
        let progress = elapsed / TIME_LIMIT_SEC;
        let temp = t0 * (1.0 - progress) + t1 * progress; // Linear cooling

        // Neighborhood operations
        let op = rng.gen_range(0, 2);
        let mut next_order = current_order.clone();

        if op == 0 {
            // Swap two elements
            let idx1 = rng.gen_range(0, input.n);
            let idx2 = rng.gen_range(0, input.n);
            next_order.swap(idx1, idx2);
        } else {
            // Insert an element into a new position
            let idx_from = rng.gen_range(0, input.n);
            let idx_to = rng.gen_range(0, input.n);
            let val = next_order.remove(idx_from);
            next_order.insert(idx_to, val);
        }

        let (next_score, _) = evaluate_order(&next_order, &input, false);
        let diff = next_score - current_score;

        // Simulated Annealing acceptance criterion
        if diff <= 0 || rng.next_f64() < (-diff as f64 / temp).exp() {
            current_order = next_order;
            current_score = next_score;

            if current_score < best_score {
                best_score = current_score;
                best_order = current_order.clone();
            }
        }
    }

    // Generate final actions from the best order
    let (final_score, final_actions) = evaluate_order(&best_order, &input, true);

    eprintln!("Iterations: {}", iter_count);
    eprintln!("Best Score (Bare hands): {}", final_score);

    for (w, b) in final_actions {
        println!("{} {}", w, b);
    }
}
