use std::io::{self, BufRead};
use std::time::Instant;

const N: usize = 20;
const N_SQ: usize = 400;
const EXIT_R: u8 = 0;
const EXIT_C: u8 = 10;
const MAX_MOVES: usize = 100000;

const BEAM_WIDTH: usize = 10;
const BEAM_DEPTH: usize = 3;
const LOOKAHEAD_K: usize = 8;

const ENABLE_DEBUG_LOG: bool = false;

#[derive(Clone)]
struct State {
    grid: [[i16; N]; N],
    pos: [(u8, u8); N_SQ],
    target: i16,
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
        self.check_exit();
    }

    fn calc_score(&self) -> f64 {
        let mut score = -(self.target as f64) * 10000.0;
        let mut weight = 100.0;

        for i in 0..LOOKAHEAD_K {
            let id = self.target as usize + i;
            if id < N_SQ {
                let (r, c) = self.pos[id];
                let d = r.abs_diff(EXIT_R) + c.abs_diff(EXIT_C);
                score += d as f64 * weight;
            }
            weight *= 0.5;
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
        let mut next_beam = Vec::with_capacity(current_beam.len() * 40);

        for (score, st, first_move) in &current_beam {
            if st.target == N_SQ as i16 {
                next_beam.push((*score, st.clone(), *first_move));
                continue;
            }

            for m in 0..conveyors.len() {
                for &d in &[-1, 1] {
                    let mut next_st = st.clone();
                    next_st.apply_op(&conveyors[m], d);
                    let next_score = next_st.calc_score();
                    let fm = first_move.unwrap_or((m, d));
                    next_beam.push((next_score, next_st, Some(fm)));
                }
            }
        }

        next_beam.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        next_beam.truncate(BEAM_WIDTH);
        current_beam = next_beam;
    }

    current_beam[0].2.unwrap()
}

fn main() {
    let start_time = Instant::now();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n_in: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
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
    let mut last_target = -1;

    while current_state.target < N_SQ as i16 && final_moves.len() < MAX_MOVES {
        if current_state.target != last_target || final_moves.len() % 1000 == 0 {
            if ENABLE_DEBUG_LOG {
                eprintln!(
                    "Target: {:3} / {}, Moves: {:5}, Elapsed: {:?}",
                    current_state.target,
                    N_SQ,
                    final_moves.len(),
                    start_time.elapsed()
                );
            }
            last_target = current_state.target;
        }

        let (m, d) = get_best_move(&current_state, &conveyors);
        current_state.apply_op(&conveyors[m], d);
        final_moves.push((m, d));
    }

    if ENABLE_DEBUG_LOG {
        eprintln!(
            "Finished. Target: {:3} / {}, Moves: {:5}, Elapsed: {:?}",
            current_state.target,
            N_SQ,
            final_moves.len(),
            start_time.elapsed()
        );
    }

    println!("{}", final_moves.len());
    for (m, d) in final_moves {
        println!("{} {}", m, d);
    }
}
