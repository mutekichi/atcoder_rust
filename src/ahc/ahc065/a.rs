use std::io::{self, BufRead};
use std::time::Instant;

const N: usize = 20;
const N_SQ: usize = 400;
const EXIT_R: u8 = 0;
const EXIT_C: u8 = 10;
const MAX_MOVES: usize = 100000;

const BEAM_WIDTH: usize = 14;
const BEAM_DEPTH: usize = 6;
const FOCUS_K: usize = 5;

const SCORE_BASE_WEIGHT: f64 = 10000.0;
const SCORE_DIST_WEIGHT_INIT: f64 = 100.0;
const SCORE_DIST_DECAY: f64 = 0.95;
const SCORE_DIST_DECAY_FIRST: f64 = 0.7;

const ENABLE_DEBUG_LOG: bool = false;

#[derive(Clone)]
struct State {
    grid: [[i16; N]; N],
    pos: [(u8, u8); N_SQ],
    target: i16,
    last_move: Option<(usize, i8)>,
}

struct Conveyor {
    cells: Vec<(u8, u8)>,
}

impl State {
    fn new(initial_grid: &[[i16; N]; N]) -> Self {
        let mut grid = [[0; N]; N];
        let mut pos = [(0, 0); N_SQ];
        for r in 0..N {
            for c in 0..N {
                grid[r][c] = initial_grid[r][c];
                if grid[r][c] >= 0 {
                    pos[grid[r][c] as usize] = (r as u8, c as u8);
                }
            }
        }
        let mut state = State {
            grid,
            pos,
            target: 0,
            last_move: None,
        };
        state.check_exit();
        state
    }

    fn check_exit(&mut self) {
        while self.target < N_SQ as i16 {
            if self.grid[EXIT_R as usize][EXIT_C as usize] == self.target {
                self.grid[EXIT_R as usize][EXIT_C as usize] = -1;
                self.target += 1;
            } else {
                break;
            }
        }
    }

    fn apply_op(
        &mut self,
        conveyor: &Conveyor,
        conv_id: usize,
        dir: i8,
    ) {
        let len = conveyor.cells.len();
        if dir == 1 {
            let (last_r, last_c) = conveyor.cells[len - 1];
            let last_val = self.grid[last_r as usize][last_c as usize];

            for i in (1..len).rev() {
                let (r_to, c_to) = conveyor.cells[i];
                let (r_from, c_from) = conveyor.cells[i - 1];
                let val = self.grid[r_from as usize][c_from as usize];

                self.grid[r_to as usize][c_to as usize] = val;
                if val >= 0 {
                    self.pos[val as usize] = (r_to, c_to);
                }
            }
            let (first_r, first_c) = conveyor.cells[0];
            self.grid[first_r as usize][first_c as usize] = last_val;
            if last_val >= 0 {
                self.pos[last_val as usize] = (first_r, first_c);
            }
        } else {
            let (first_r, first_c) = conveyor.cells[0];
            let first_val = self.grid[first_r as usize][first_c as usize];

            for i in 0..len - 1 {
                let (r_to, c_to) = conveyor.cells[i];
                let (r_from, c_from) = conveyor.cells[i + 1];
                let val = self.grid[r_from as usize][c_from as usize];

                self.grid[r_to as usize][c_to as usize] = val;
                if val >= 0 {
                    self.pos[val as usize] = (r_to, c_to);
                }
            }
            let (last_r, last_c) = conveyor.cells[len - 1];
            self.grid[last_r as usize][last_c as usize] = first_val;
            if first_val >= 0 {
                self.pos[first_val as usize] = (last_r, last_c);
            }
        }
        self.last_move = Some((conv_id, dir));
        self.check_exit();
    }

    fn calc_score(&self) -> f64 {
        let mut score = -(self.target as f64) * SCORE_BASE_WEIGHT;
        let mut weight = SCORE_DIST_WEIGHT_INIT;

        for id in (self.target as usize)..N_SQ {
            let (r, c) = self.pos[id];
            let d = r.abs_diff(EXIT_R) + c.abs_diff(EXIT_C);
            score += d as f64 * weight;

            if id == self.target as usize {
                weight *= SCORE_DIST_DECAY_FIRST;
            } else {
                weight *= SCORE_DIST_DECAY;
            }
        }
        score
    }
}

fn get_best_move(
    initial_state: &State,
    conveyors: &[Conveyor],
) -> (usize, i8) {
    let mut current_beam = vec![(initial_state.calc_score(), initial_state.clone(), None)];

    for _ in 0..BEAM_DEPTH {
        let mut next_beam = Vec::with_capacity(current_beam.len() * 16);

        for (score, st, first_move) in &current_beam {
            if st.target == N_SQ as i16 {
                next_beam.push((*score, st.clone(), *first_move));
                continue;
            }

            let mut active_conveyors = Vec::with_capacity(FOCUS_K * 2);
            for i in 0..FOCUS_K {
                let id = st.target as usize + i;
                if id < N_SQ {
                    let (r, c) = st.pos[id];
                    active_conveyors.push(r as usize / 2);
                    active_conveyors.push(10 + c as usize / 2);
                }
            }
            active_conveyors.sort_unstable();
            active_conveyors.dedup();

            for &m in &active_conveyors {
                for &d in &[-1, 1] {
                    if let Some((prev_m, prev_d)) = st.last_move {
                        if prev_m == m && prev_d == -d {
                            continue;
                        }
                    }

                    let mut next_st = st.clone();
                    next_st.apply_op(&conveyors[m], m, d);
                    let next_score = next_st.calc_score();
                    let fm = first_move.unwrap_or((m, d));
                    next_beam.push((next_score, next_st, Some(fm)));
                }
            }
        }

        if next_beam.is_empty() {
            break;
        }

        next_beam.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        next_beam.truncate(BEAM_WIDTH);
        current_beam = next_beam;
    }

    current_beam[0].2.expect("No valid move found")
}

fn main() {
    let _start_time = Instant::now();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n_in_line = lines.next().unwrap().unwrap();
    let n_in: usize = n_in_line.trim().parse().unwrap();
    let mut initial_grid = [[0i16; N]; N];
    for r in 0..n_in {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i16> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for c in 0..N {
            initial_grid[r][c] = row[c];
        }
    }

    let mut conveyors = Vec::new();

    for k in 0..N / 2 {
        let mut cells = Vec::new();
        for c in 0..N {
            cells.push((2 * k as u8, c as u8));
        }
        for c in (0..N).rev() {
            cells.push((2 * k as u8 + 1, c as u8));
        }
        conveyors.push(Conveyor { cells });
    }

    for k in 0..N / 2 {
        let mut cells = Vec::new();
        for r in 0..N {
            cells.push((r as u8, 2 * k as u8));
        }
        for r in (0..N).rev() {
            cells.push((r as u8, 2 * k as u8 + 1));
        }
        conveyors.push(Conveyor { cells });
    }

    println!("{}", conveyors.len());
    for conv in &conveyors {
        print!("{}", conv.cells.len());
        for &(r, c) in &conv.cells {
            print!(" {} {}", r, c);
        }
        println!();
    }

    let mut current_state = State::new(&initial_grid);
    let mut final_moves = Vec::new();

    while current_state.target < N_SQ as i16 && final_moves.len() < MAX_MOVES {
        let (m, d) = get_best_move(&current_state, &conveyors);
        current_state.apply_op(&conveyors[m], m, d);
        final_moves.push((m, d));
    }

    println!("{}", final_moves.len());
    for (m, d) in final_moves {
        println!("{} {}", m, d);
    }
}
