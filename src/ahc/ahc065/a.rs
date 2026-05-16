use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::time::Instant;

const N: usize = 20;
const N_SQ: usize = 400;
const EXIT_R: u8 = 0;
const EXIT_C: u8 = 10;
const MAX_T: usize = 60000;
const TIME_LIMIT_SECS: f64 = 1.95;

#[derive(Clone, Copy, PartialEq, Eq)]
struct StateNode {
    f: u16,
    g: u16,
    t: u16,
    r: u8,
    c: u8,
}

impl Ord for StateNode {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        other
            .f
            .cmp(&self.f)
            .then_with(|| other.g.cmp(&self.g))
            .then_with(|| self.t.cmp(&other.t))
            .then_with(|| self.r.cmp(&other.r))
            .then_with(|| self.c.cmp(&other.c))
    }
}

impl PartialOrd for StateNode {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
struct ParentInfo {
    pt: u16,
    pr: u8,
    pc: u8,
    action: u8,
}

struct Conveyor {
    cells: Vec<(u8, u8)>,
}

type CellConveyors = [[[(usize, usize); 2]; N]; N];

struct Solver {
    history_flat: Vec<i16>,
    exit_times: Vec<usize>,
    locked: Vec<bool>,
    dist: Vec<u16>,
    parent: Vec<ParentInfo>,
    visited_nodes: Vec<(u16, u8, u8)>,
}

impl Solver {
    fn new() -> Self {
        Self {
            history_flat: vec![0; MAX_T * N_SQ],
            exit_times: vec![usize::MAX; N_SQ],
            locked: vec![false; MAX_T * 20],
            dist: vec![u16::MAX; MAX_T * N_SQ],
            parent: vec![
                ParentInfo {
                    pt: 0,
                    pr: 0,
                    pc: 0,
                    action: 0
                };
                MAX_T * N_SQ
            ],
            visited_nodes: Vec::with_capacity(100000),
        }
    }
}

#[inline]
fn get_idx(
    t: u16,
    r: u8,
    c: u8,
) -> usize {
    t as usize * N_SQ + r as usize * N + c as usize
}

fn apply_op(
    grid: &mut [[i16; N]; N],
    conveyor: &Conveyor,
    dir: i8,
) {
    let len = conveyor.cells.len();
    if dir == 1 {
        let (last_r, last_c) = conveyor.cells[len - 1];
        let last_val = grid[last_r as usize][last_c as usize];
        for i in (1..len).rev() {
            let (r_to, c_to) = conveyor.cells[i];
            let (r_from, c_from) = conveyor.cells[i - 1];
            grid[r_to as usize][c_to as usize] = grid[r_from as usize][c_from as usize];
        }
        let (first_r, first_c) = conveyor.cells[0];
        grid[first_r as usize][first_c as usize] = last_val;
    } else {
        let (first_r, first_c) = conveyor.cells[0];
        let first_val = grid[first_r as usize][first_c as usize];
        for i in 0..len - 1 {
            let (r_to, c_to) = conveyor.cells[i];
            let (r_from, c_from) = conveyor.cells[i + 1];
            grid[r_to as usize][c_to as usize] = grid[r_from as usize][c_from as usize];
        }
        let (last_r, last_c) = conveyor.cells[len - 1];
        grid[last_r as usize][last_c as usize] = first_val;
    }
}

fn get_next_pos(
    r: u8,
    c: u8,
    m: usize,
    d: i8,
    conveyors: &[Conveyor],
    cell_conveyors: &CellConveyors,
) -> (u8, u8) {
    for &(conv_id, idx) in &cell_conveyors[r as usize][c as usize] {
        if conv_id == m {
            let conv = &conveyors[m];
            let len = conv.cells.len();
            let next_idx = if d == 1 {
                (idx + 1) % len
            } else {
                (idx + len - 1) % len
            };
            return conv.cells[next_idx];
        }
    }
    (r, c)
}

fn simulate(
    initial_grid: &[[i16; N]; N],
    moves: &[(usize, i8)],
    conveyors: &[Conveyor],
    solver: &mut Solver,
) {
    solver.exit_times.fill(usize::MAX);
    let mut current_grid = *initial_grid;
    let mut target = 0;

    let mut check_exit = |grid: &mut [[i16; N]; N], tgt: &mut usize, t: usize| {
        while *tgt < N_SQ && grid[EXIT_R as usize][EXIT_C as usize] == *tgt as i16 {
            grid[EXIT_R as usize][EXIT_C as usize] = -1;
            solver.exit_times[*tgt] = t;
            *tgt += 1;
        }
    };

    check_exit(&mut current_grid, &mut target, 0);

    for r in 0..N {
        for c in 0..N {
            solver.history_flat[r * N + c] = current_grid[r][c];
        }
    }

    for (i, &(m, d)) in moves.iter().enumerate() {
        apply_op(&mut current_grid, &conveyors[m], d);
        check_exit(&mut current_grid, &mut target, i + 1);

        let offset = (i + 1) * N_SQ;
        for r in 0..N {
            for c in 0..N {
                solver.history_flat[offset + r * N + c] = current_grid[r][c];
            }
        }
    }
}

fn update_locked(
    moves_len: usize,
    current_box: usize,
    conveyors: &[Conveyor],
    solver: &mut Solver,
) {
    for t in 0..=moves_len {
        for m in 0..20 {
            let mut is_locked = false;
            for &(r, c) in &conveyors[m].cells {
                let val = solver.history_flat[t * N_SQ + r as usize * N + c as usize];
                if val >= 0 && (val as usize) < current_box {
                    is_locked = true;
                    break;
                }
            }
            solver.locked[t * 20 + m] = is_locked;
        }
    }
}

fn main() {
    let start_time = Instant::now();
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

    // Horizontal loops
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

    // Vertical loops
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

    let mut cell_conveyors: CellConveyors = [[[(usize::MAX, usize::MAX); 2]; N]; N];
    for (id, conv) in conveyors.iter().enumerate() {
        for (idx, &(r, c)) in conv.cells.iter().enumerate() {
            let cell = &mut cell_conveyors[r as usize][c as usize];
            if cell[0].0 == usize::MAX {
                cell[0] = (id, idx);
            } else {
                cell[1] = (id, idx);
            }
        }
    }

    let mut moves: Vec<(usize, i8)> = Vec::new();
    let mut solver = Solver::new();
    let mut pq = BinaryHeap::with_capacity(100000);

    for i in 0..N_SQ {
        simulate(&initial_grid, &moves, &conveyors, &mut solver);

        if solver.exit_times[i] < usize::MAX {
            // Already exited
            continue;
        }

        let t_prev = if i == 0 { 0 } else { solver.exit_times[i - 1] };
        update_locked(moves.len(), i, &conveyors, &mut solver);

        let mut start_r = 0;
        let mut start_c = 0;
        for r in 0..N {
            for c in 0..N {
                if initial_grid[r][c] == i as i16 {
                    start_r = r as u8;
                    start_c = c as u8;
                }
            }
        }

        pq.clear();
        for &(t, r, c) in &solver.visited_nodes {
            solver.dist[get_idx(t, r, c)] = u16::MAX;
        }
        solver.visited_nodes.clear();

        let start_idx = get_idx(0, start_r, start_c);
        solver.dist[start_idx] = 0;
        solver.visited_nodes.push((0, start_r, start_c));

        let h_start = start_r.abs_diff(EXIT_R) + start_c.abs_diff(EXIT_C);
        pq.push(std::cmp::Reverse(StateNode {
            f: h_start as u16,
            g: 0,
            t: 0,
            r: start_r,
            c: start_c,
        }));

        let mut goal_node = None;

        let elapsed = start_time.elapsed().as_secs_f64();
        // Increase heuristic weight to speed up A* if time is running out
        let h_weight: u16 = if elapsed > 1.8 {
            5
        } else if elapsed > 1.5 {
            3
        } else {
            2
        };

        while let Some(std::cmp::Reverse(node)) = pq.pop() {
            let StateNode { f: _, g, t, r, c } = node;

            if solver.dist[get_idx(t, r, c)] < g {
                continue;
            }

            if r == EXIT_R && c == EXIT_C && t >= t_prev as u16 {
                goal_node = Some((t, r, c));
                break;
            }

            let mut add_transition = |nt: u16, nr: u8, nc: u8, cost_inc: u16, action: u8| {
                if nt as usize >= MAX_T {
                    return;
                }
                let ng = g + cost_inc;
                let nidx = get_idx(nt, nr, nc);
                if ng < solver.dist[nidx] {
                    solver.dist[nidx] = ng;
                    solver.parent[nidx] = ParentInfo {
                        pt: t,
                        pr: r,
                        pc: c,
                        action,
                    };
                    solver.visited_nodes.push((nt, nr, nc));

                    let h = nr.abs_diff(EXIT_R) + nc.abs_diff(EXIT_C);
                    let f = ng + h as u16 * h_weight;
                    pq.push(std::cmp::Reverse(StateNode {
                        f,
                        g: ng,
                        t: nt,
                        r: nr,
                        c: nc,
                    }));
                }
            };

            // Transition 1: Consume existing move or wait
            if (t as usize) < moves.len() {
                let (m, d) = moves[t as usize];
                let mut nr = r;
                let mut nc = c;

                let in_conv = cell_conveyors[r as usize][c as usize]
                    .iter()
                    .any(|&(conv_id, _)| conv_id == m);
                if in_conv {
                    let next_pos = get_next_pos(r, c, m, d, &conveyors, &cell_conveyors);
                    nr = next_pos.0;
                    nc = next_pos.1;
                }
                add_transition(t + 1, nr, nc, 0, 0);
            } else {
                // If out of moves but need to wait for t_prev
                add_transition(t + 1, r, c, 0, 0);
            }

            // Transition 2: Insert new move
            for &(m, _idx) in &cell_conveyors[r as usize][c as usize] {
                if m == usize::MAX {
                    continue;
                }

                let locked_val = if (t as usize) <= moves.len() {
                    solver.locked[(t as usize) * 20 + m]
                } else {
                    false
                };

                if !locked_val {
                    for &d in &[1, -1] {
                        let (nr, nc) = get_next_pos(r, c, m, d, &conveyors, &cell_conveyors);
                        let action = if d == 1 {
                            1 + m as u8 * 2
                        } else {
                            2 + m as u8 * 2
                        };
                        add_transition(t, nr, nc, 1, action);
                    }
                }
            }
        }

        if let Some((gt, gr, gc)) = goal_node {
            let mut path = Vec::new();
            let mut curr_t = gt;
            let mut curr_r = gr;
            let mut curr_c = gc;

            while curr_t != 0 || curr_r != start_r || curr_c != start_c {
                let p = solver.parent[get_idx(curr_t, curr_r, curr_c)];
                if p.action != 0 {
                    let m = ((p.action - 1) / 2) as usize;
                    let d = if (p.action - 1) % 2 == 0 { 1 } else { -1 };
                    path.push((p.pt, Some((m, d))));
                } else {
                    path.push((p.pt, None));
                }
                curr_t = p.pt;
                curr_r = p.pr;
                curr_c = p.pc;
            }

            path.reverse();

            let mut new_moves = Vec::new();
            let mut old_t = 0;

            for (pt, action) in path {
                while old_t < pt as usize {
                    if old_t < moves.len() {
                        new_moves.push(moves[old_t]);
                    }
                    old_t += 1;
                }

                if let Some(op) = action {
                    new_moves.push(op);
                } else {
                    if old_t < moves.len() {
                        new_moves.push(moves[old_t]);
                    }
                    old_t += 1;
                }
            }
            while old_t < moves.len() {
                new_moves.push(moves[old_t]);
                old_t += 1;
            }

            moves = new_moves;
        } else {
            eprintln!("Blocked at box {}. Stopping.", i);
            break;
        }
    }

    println!("{}", moves.len());
    for (m, d) in moves {
        println!("{} {}", m, d);
    }
}
