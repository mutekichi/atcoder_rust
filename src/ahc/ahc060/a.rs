#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proconio::input;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::{BufWriter, Write, stdout};
use std::time::Instant;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(42));
}

fn rand_range(
    low: usize,
    high: usize,
) -> usize {
    RNG.with(|rng| rng.borrow_mut().random_range(low..high))
}

fn rand_bool(p: f64) -> bool {
    RNG.with(|rng| rng.borrow_mut().random_bool(p))
}

fn rand_f64() -> f64 {
    RNG.with(|rng| rng.borrow_mut().random::<f64>())
}

fn get_env_f64(
    name: &str,
    default: f64,
) -> f64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct IceCream {
    data: [u64; 4],
    len: usize,
}

impl IceCream {
    fn new() -> Self {
        Self {
            data: [0; 4],
            len: 0,
        }
    }

    fn add(
        &mut self,
        is_white: bool,
    ) {
        if self.len < 256 {
            let idx = self.len / 64;
            let bit = self.len % 64;
            if !is_white {
                self.data[idx] |= 1u64 << bit;
            }
            self.len += 1;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NodeType {
    Shop,
    Red,
    White,
}

#[derive(Clone, Debug)]
enum Action {
    Move(usize),
    ChangeColor,
}

struct State {
    current_node: usize,
    from_node: usize,
    ice_cream: IceCream,
    delivered_ice_creams: Vec<BTreeSet<IceCream>>,
    steps_left: usize,
    graph: Vec<Vec<usize>>,
    edge_weights: Vec<Vec<f64>>,
    node_status: Vec<NodeType>,
    n_total_turns: usize,
    n_total_trees: usize,
    n_red_trees: usize,
    current_plan: Vec<Action>,
    start_time: Instant,

    p_penalty: f64,
    p_reward_base: f64,
    p_reward_coef: f64,
    p_prob_factor: f64,
    p_fallback_prob: f64,
}

fn main() {
    let start_time = Instant::now();
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out, start_time);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(
    out: &mut W,
    start_time: Instant,
) {
    input! {
        n_total_nodes: usize,
        n_edges: usize,
        n_total_shops: usize,
        n_move_limit: usize,
        ab: [(usize, usize); n_edges],
        _xy: [(usize, usize); n_total_nodes],
    }

    let graph = {
        let mut graph = vec![vec![]; n_total_nodes];
        for (a, b) in ab {
            graph[a].push(b);
            graph[b].push(a);
        }
        for i in 0..n_total_nodes {
            graph[i].sort_unstable();
        }
        graph
    };

    let edge_weights = graph
        .iter()
        .map(|neighbors| vec![1.0; neighbors.len()])
        .collect();

    let node_status = vec![NodeType::Shop; n_total_shops]
        .into_iter()
        .chain(vec![NodeType::White; n_total_nodes - n_total_shops])
        .collect::<Vec<_>>();

    let mut state = State {
        current_node: 0,
        from_node: !0,
        ice_cream: IceCream::new(),
        delivered_ice_creams: vec![BTreeSet::new(); n_total_shops],
        steps_left: n_move_limit,
        graph,
        edge_weights,
        node_status,
        n_total_turns: n_move_limit,
        n_total_trees: n_total_nodes - n_total_shops,
        n_red_trees: 0,
        current_plan: Vec::new(),
        start_time,

        p_penalty: get_env_f64("PARAM_PENALTY", 0.99),
        p_reward_base: get_env_f64("PARAM_REWARD_BASE", 1.0),
        p_reward_coef: get_env_f64("PARAM_REWARD_COEF", 0.1),
        p_prob_factor: get_env_f64("PARAM_PROB_FACTOR", 0.5),
        p_fallback_prob: get_env_f64("PARAM_FALLBACK_PROB", 0.3),
    };

    while state.steps_left > 0 {
        if state.current_plan.is_empty() {
            generate_plan(&mut state);
        }
        execute_action(&mut state, out);
    }

    let mut total_score = 0;
    for shop in &state.delivered_ice_creams {
        total_score += shop.len();
    }
    eprintln!("Score: {}", total_score);
}

fn generate_plan(state: &mut State) {
    let global_time_limit_ms = 1900.0;
    let global_elapsed_ms = state.start_time.elapsed().as_millis() as f64;
    let remaining_ms = global_time_limit_ms - global_elapsed_ms;

    if remaining_ms <= 0.0 {
        fallback_move(state);
        return;
    }

    let estimated_plans_needed = (state.steps_left as f64 / 15.0).max(1.0);
    let time_for_this_plan = remaining_ms / estimated_plans_needed;

    let plan_start = Instant::now();
    let max_depth = 64;

    let mut best_plan = Vec::new();
    let mut shortest_valid_len = usize::MAX;
    let mut iter = 0;

    loop {
        if iter & 15 == 0 {
            let current_elapsed = state.start_time.elapsed().as_millis() as f64;
            if current_elapsed >= global_time_limit_ms {
                break;
            }
            let plan_elapsed = plan_start.elapsed().as_secs_f64() * 1000.0;
            if plan_elapsed >= time_for_this_plan && !best_plan.is_empty() {
                break;
            }
            if plan_elapsed >= time_for_this_plan * 3.0 {
                break;
            }
        }
        iter += 1;

        let mut curr = state.current_node;
        let mut prev = state.from_node;
        let mut path = Vec::new();
        let mut path_indices = Vec::new();
        let mut sim_ice_cream = state.ice_cream.clone();

        let mut reached_shop = false;
        let mut target_shop_id = 0;

        for _ in 0..max_depth {
            let neighbors = &state.graph[curr];
            let weights = &state.edge_weights[curr];

            let mut weight_sum = 0.0;
            let mut valid_moves = Vec::new();

            for (i, &next) in neighbors.iter().enumerate() {
                if next != prev {
                    valid_moves.push((i, next, weights[i]));
                    weight_sum += weights[i];
                }
            }

            if valid_moves.is_empty() {
                break;
            }

            let mut rand_val = rand_f64() * weight_sum;
            let mut chosen_idx = 0;
            let mut chosen_next = valid_moves[0].1;

            for &(i, next, w) in &valid_moves {
                rand_val -= w;
                if rand_val <= 0.0 {
                    chosen_idx = i;
                    chosen_next = next;
                    break;
                }
            }

            path.push(chosen_next);
            path_indices.push((curr, chosen_idx));

            if state.node_status[chosen_next] == NodeType::Shop {
                reached_shop = true;
                target_shop_id = chosen_next;
                break;
            } else {
                sim_ice_cream.add(state.node_status[chosen_next] == NodeType::White);
            }

            prev = curr;
            curr = chosen_next;
        }

        if reached_shop {
            let is_duplicate = state.delivered_ice_creams[target_shop_id].contains(&sim_ice_cream);

            let progress =
                (state.n_total_turns - state.steps_left) as f64 / state.n_total_turns as f64;
            let target_red = progress * state.n_total_trees as f64;
            let needed_r = target_red - state.n_red_trees as f64;

            if !is_duplicate {
                let len_factor = 5.0 / (path.len() as f64).max(1.0);
                let multiplier = state.p_reward_base + (state.p_reward_coef * len_factor);

                for &(u, edge_idx) in &path_indices {
                    state.edge_weights[u][edge_idx] =
                        (state.edge_weights[u][edge_idx] * multiplier).min(100.0);
                }

                let mut plan = Vec::new();
                if needed_r > 0.0 {
                    let mut w_nodes = BTreeSet::new();
                    for &node in &path {
                        if state.node_status[node] == NodeType::White {
                            w_nodes.insert(node);
                        }
                    }

                    let n_w = w_nodes.len();
                    if n_w > 0 {
                        let prob = ((needed_r / n_w as f64) * state.p_prob_factor).clamp(0.0, 1.0);
                        let mut changed_nodes = BTreeSet::new();

                        for &node in &path {
                            plan.push(Action::Move(node));
                            if state.node_status[node] == NodeType::White
                                && !changed_nodes.contains(&node)
                            {
                                if rand_bool(prob) {
                                    plan.push(Action::ChangeColor);
                                    changed_nodes.insert(node);
                                }
                            }
                        }
                    } else {
                        for &node in &path {
                            plan.push(Action::Move(node));
                        }
                    }
                } else {
                    for &node in &path {
                        plan.push(Action::Move(node));
                    }
                }

                if path.len() < shortest_valid_len {
                    shortest_valid_len = path.len();
                    best_plan = plan;
                }
            } else {
                for &(u, edge_idx) in &path_indices {
                    state.edge_weights[u][edge_idx] =
                        (state.edge_weights[u][edge_idx] * state.p_penalty).max(0.01);
                }

                if needed_r > 0.0 && best_plan.is_empty() {
                    let mut w_nodes = BTreeSet::new();
                    for &node in &path {
                        if state.node_status[node] == NodeType::White {
                            w_nodes.insert(node);
                        }
                    }

                    if !w_nodes.is_empty() {
                        let w_nodes_vec: Vec<_> = w_nodes.into_iter().collect();
                        let target_node = w_nodes_vec[rand_range(0, w_nodes_vec.len())];

                        let mut plan = Vec::new();
                        let mut changed = false;
                        for &node in &path {
                            plan.push(Action::Move(node));
                            if node == target_node && !changed {
                                plan.push(Action::ChangeColor);
                                changed = true;
                            }
                        }

                        if rand_bool(state.p_fallback_prob) {
                            best_plan = plan;
                        }
                    }
                }
            }
        }
    }

    if best_plan.is_empty() {
        fallback_move(state);
    } else {
        best_plan.reverse();
        state.current_plan = best_plan;
    }
}

fn fallback_move(state: &mut State) {
    let neighbors = &state.graph[state.current_node];
    let mut valid_neighbors = Vec::new();
    for &next in neighbors {
        if next != state.from_node {
            valid_neighbors.push(next);
        }
    }
    if !valid_neighbors.is_empty() {
        let next_node = valid_neighbors[rand_range(0, valid_neighbors.len())];
        state.current_plan.push(Action::Move(next_node));
    }
}

fn execute_action<W: Write>(
    state: &mut State,
    out: &mut W,
) {
    if let Some(action) = state.current_plan.pop() {
        match action {
            Action::Move(next_node) => {
                writeln!(out, "{}", next_node).unwrap();
                state.from_node = state.current_node;
                state.current_node = next_node;

                if state.node_status[next_node] != NodeType::Shop {
                    let is_white = state.node_status[next_node] == NodeType::White;
                    state.ice_cream.add(is_white);
                } else {
                    let shop_id = next_node;
                    state.delivered_ice_creams[shop_id].insert(state.ice_cream.clone());
                    state.ice_cream = IceCream::new();
                }
            }
            Action::ChangeColor => {
                writeln!(out, "-1").unwrap();
                state.node_status[state.current_node] = NodeType::Red;
                state.n_red_trees += 1;
            }
        }
        state.steps_left -= 1;
    } else {
        state.steps_left -= 1;
    }
}
