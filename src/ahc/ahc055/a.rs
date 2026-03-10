#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{BufWriter, Write, stdout};
use std::mem::swap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::time::Instant;

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

struct Xorshift {
    seed: u64,
}

impl Xorshift {
    fn new(seed: u64) -> Self {
        Self {
            seed: if seed == 0 { 88172645463325252 } else { seed },
        }
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        (self.seed as f64) / (u64::MAX as f64)
    }
}

struct Input {
    n: usize,
    h: Vec<i32>,
    c: Vec<i32>,
    a: Vec<Vec<i32>>,
    c_scores: Vec<f64>,
    chest_weights: Vec<f64>,
}

impl Input {
    fn new() -> Self {
        input! {
            n: usize,
            h: [i32; n],
            c: [i32; n],
            a: [[i32; n]; n],
        }

        let mut c_scores = vec![0.0; n];
        for i in 0..n {
            let s = match c[i] {
                6 => 3.0,
                5 => 2.0,
                4 => 1.5,
                3 => 1.0,
                2 => 0.5,
                1 => 0.0,
                _ => 0.0,
            };
            c_scores[i] = if s > 0.0 { s } else { 0.001 };
        }

        let mut chest_weights = vec![0.0; n];
        for b in 0..n {
            let mut incoming_damages = Vec::with_capacity(n);
            for w in 0..n {
                incoming_damages.push(a[w][b]);
            }
            incoming_damages.sort_unstable_by(|x, y| y.cmp(x));

            let top6_sum: i32 = incoming_damages.iter().take(6).sum();
            let avg_top6 = top6_sum as f64 / 6.0;

            chest_weights[b] = 100.0 / avg_top6.max(1.0);
        }

        Self {
            n,
            h,
            c,
            a,
            c_scores,
            chest_weights,
        }
    }
}

struct State {
    h: Vec<i32>,
    durabilities: Vec<i32>,
    opened: Vec<bool>,
    unopened_chests: Vec<usize>,
    unopened_pos: Vec<usize>,
    active_weapons: Vec<usize>,
    open_count: usize,
    actions_count: usize,
    actions: Vec<(i32, usize)>,
    record_actions: bool,
    active_durability_sum: i32,
    temp_scores: Vec<f64>,
}

impl State {
    fn new(
        input: &Input,
        record_actions: bool,
    ) -> Self {
        Self {
            h: input.h.clone(),
            durabilities: input.c.clone(),
            opened: vec![false; input.n],
            unopened_chests: (0..input.n).collect(),
            unopened_pos: (0..input.n).collect(),
            active_weapons: Vec::with_capacity(input.n),
            open_count: 0,
            actions_count: 0,
            actions: if record_actions {
                Vec::with_capacity(10000)
            } else {
                Vec::new()
            },
            record_actions,
            active_durability_sum: 0,
            temp_scores: vec![0.0; input.n],
        }
    }

    #[inline]
    fn mark_opened(
        &mut self,
        b: usize,
        input: &Input,
    ) {
        self.opened[b] = true;
        self.open_count += 1;
        self.active_durability_sum += input.c[b];

        if input.c[b] > 0 {
            self.active_weapons.push(b);
        }

        let pos = self.unopened_pos[b];
        let last_b = *self.unopened_chests.last().unwrap();

        self.unopened_chests.swap_remove(pos);
        self.unopened_pos[last_b] = pos;
    }

    #[inline]
    fn calc_dynamic_weapon_score(
        &self,
        w: usize,
        input: &Input,
    ) -> f64 {
        let mut top1 = 0;
        let mut top2 = 0;
        let mut top3 = 0;

        for &b in &self.unopened_chests {
            if b == w {
                continue;
            }
            let dmg = input.a[w][b];
            if dmg > top1 {
                top3 = top2;
                top2 = top1;
                top1 = dmg;
            } else if dmg > top2 {
                top3 = top2;
                top2 = dmg;
            } else if dmg > top3 {
                top3 = dmg;
            }
        }

        (top1 + top2 + top3) as f64 * input.c_scores[w]
    }

    fn try_finish_with_bare_hands(
        &mut self,
        input: &Input,
        rng: &mut Xorshift,
    ) -> bool {
        let mut best_chest = usize::MAX;
        let mut max_priority = -1.0;

        let threshold_multiplier = match self.active_durability_sum {
            0 | 1 => 1.8,
            2 => 1.7,
            3 => 1.6,
            4 => 1.4,
            5 => 1.3,
            6 => 1.2,
            7 => 1.1,
            8 => 1.0,
            9 => 0.9,
            10 => 0.8,
            11 => 0.7,
            _ => 0.6,
        };

        for &i in &self.unopened_chests {
            let threshold_base = match input.c[i] {
                6 => 70.0,
                5 => 50.0,
                4 => 35.0,
                3 => 25.0,
                2 => 20.0,
                1 => 15.0,
                _ => 0.0,
            };

            let threshold = threshold_base * threshold_multiplier;

            if (self.h[i] as f64) <= threshold {
                let dynamic_score = self.calc_dynamic_weapon_score(i, input);
                let noise = 0.8 + 0.4 * rng.next_f64();
                let priority = (dynamic_score / (self.h[i] as f64).max(1.0)) * noise;
                if priority > max_priority {
                    max_priority = priority;
                    best_chest = i;
                }
            }
        }

        if max_priority >= 0.0 {
            let b = best_chest;
            if self.record_actions {
                while self.h[b] > 0 {
                    self.actions.push((-1, b));
                    self.h[b] -= 1;
                }
            } else {
                self.actions_count += self.h[b] as usize;
                self.h[b] = 0;
            }
            self.mark_opened(b, input);
            return true;
        }

        false
    }

    fn process_best_weapon_attack(
        &mut self,
        input: &Input,
        rng: &mut Xorshift,
    ) -> bool {
        let mut best_w_idx = usize::MAX;
        let mut best_w = usize::MAX;
        let mut best_b = usize::MAX;
        let mut max_score = -1.0;

        // Pre-calculate dynamic scores for this turn to avoid redundant calculations in the double loop
        for &b in &self.unopened_chests {
            self.temp_scores[b] = self.calc_dynamic_weapon_score(b, input);
        }

        for (idx, &w) in self.active_weapons.iter().enumerate() {
            for &b in &self.unopened_chests {
                let actual_damage = min(input.a[w][b], self.h[b]);
                let mut base_score = actual_damage as f64 * input.chest_weights[b];

                // Add bonus if this attack finishes the chest
                if input.a[w][b] >= self.h[b] {
                    base_score += self.temp_scores[b] * 0.1;
                }

                let noise = 0.8 + 0.4 * rng.next_f64();
                let score = base_score * noise;

                if score > max_score {
                    max_score = score;
                    best_w_idx = idx;
                    best_w = w;
                    best_b = b;
                }
            }
        }

        if max_score >= 0.0 {
            if self.record_actions {
                self.actions.push((best_w as i32, best_b));
            }
            self.actions_count += 1;
            self.h[best_b] -= input.a[best_w][best_b];
            self.durabilities[best_w] -= 1;
            self.active_durability_sum -= 1;

            if self.durabilities[best_w] == 0 {
                self.active_weapons.swap_remove(best_w_idx);
            }

            if self.h[best_b] <= 0 {
                self.mark_opened(best_b, input);
            }
            return true;
        }

        false
    }

    fn open_best_chest_from_scratch(
        &mut self,
        input: &Input,
        rng: &mut Xorshift,
    ) {
        let mut best_chest = self.unopened_chests[0];
        let mut best_score = -1.0;

        for &i in &self.unopened_chests {
            let h_penalty = self.h[i] as f64 + 140.0;
            let dynamic_score = self.calc_dynamic_weapon_score(i, input);
            let noise = 0.5 + 1.0 * rng.next_f64();
            let score = (dynamic_score / h_penalty) * noise;

            if score > best_score {
                best_score = score;
                best_chest = i;
            }
        }

        if self.record_actions {
            while self.h[best_chest] > 0 {
                self.actions.push((-1, best_chest));
                self.h[best_chest] -= 1;
            }
        } else {
            self.actions_count += self.h[best_chest] as usize;
            self.h[best_chest] = 0;
        }
        self.mark_opened(best_chest, input);
    }
}

fn main() {
    let start_time = Instant::now();
    let input = Input::new();

    let mut best_score = usize::MAX;
    let mut best_seed = 0;

    let mut main_rng = Xorshift::new(193);

    while start_time.elapsed().as_millis() < 1960 {
        let current_seed = main_rng.seed;
        let mut rng = Xorshift::new(current_seed);
        main_rng.next_f64();

        let mut state = State::new(&input, false);

        while state.open_count < input.n {
            if state.try_finish_with_bare_hands(&input, &mut rng) {
                continue;
            }
            if state.process_best_weapon_attack(&input, &mut rng) {
                continue;
            }
            state.open_best_chest_from_scratch(&input, &mut rng);
        }

        if state.actions_count < best_score {
            best_score = state.actions_count;
            best_seed = current_seed;
        }
    }

    let mut final_rng = Xorshift::new(best_seed);
    let mut final_state = State::new(&input, true);

    while final_state.open_count < input.n {
        if final_state.try_finish_with_bare_hands(&input, &mut final_rng) {
            continue;
        }
        if final_state.process_best_weapon_attack(&input, &mut final_rng) {
            continue;
        }
        final_state.open_best_chest_from_scratch(&input, &mut final_rng);
    }

    for (w, b) in final_state.actions {
        println!("{} {}", w, b);
    }
}
