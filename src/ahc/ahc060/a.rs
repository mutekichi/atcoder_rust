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

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(42));
}

fn rand_range(low: usize, high: usize) -> usize {
    RNG.with(|rng| rng.borrow_mut().random_range(low..high))
}

fn rand_bool(p: f64) -> bool {
    RNG.with(|rng| rng.borrow_mut().random_bool(p))
}

fn rand_f64() -> f64 {
    RNG.with(|rng| rng.borrow_mut().random::<f64>())
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct IceCream {
    flavors: Vec<bool>,
}

impl IceCream {
    fn new() -> Self {
        Self { flavors: Vec::new() }
    }

    fn add(&mut self, is_white: bool) {
        self.flavors.push(is_white);
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
}

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[allow(unused_variables)]
fn solve<W: Write>(out: &mut W) {
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

    let edge_weights = graph.iter().map(|neighbors| vec![1.0; neighbors.len()]).collect();

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
    };

    while state.steps_left > 0 {
        if state.current_plan.is_empty() {
            generate_plan(&mut state);
        }
        execute_action(&mut state, out);
    }
}

fn generate_plan(state: &mut State) {
    let sim_count = 50;
    let max_depth = 30;
    
    let mut best_plan = Vec::new();
    let mut best_score = -1.0;

    for _ in 0..sim_count {
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
            
            let progress = (state.n_total_turns - state.steps_left) as f64 / state.n_total_turns as f64;
            let target_red = progress * state.n_total_trees as f64;
            let needed_r = target_red - state.n_red_trees as f64;

            if !is_duplicate {
                for &(u, edge_idx) in &path_indices {
                    state.edge_weights[u][edge_idx] *= 1.1; 
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
                        let prob = ((needed_r / n_w as f64) * 0.5).clamp(0.0, 1.0);
                        let mut changed_nodes = BTreeSet::new();
                        
                        for &node in &path {
                            plan.push(Action::Move(node));
                            if state.node_status[node] == NodeType::White && !changed_nodes.contains(&node) {
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
                best_plan = plan;
                break; 
            } else {
                for &(u, edge_idx) in &path_indices {
                    state.edge_weights[u][edge_idx] *= 0.9;
                }

                if needed_r > 0.0 {
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
                        
                        if rand_bool(0.3) {
                            let score = 0.5; 
                            if score > best_score {
                                best_score = score;
                                best_plan = plan;
                            }
                        }
                    }
                }
            }
        }
    }

    if best_plan.is_empty() {
        let neighbors = &state.graph[state.current_node];
        let mut valid_neighbors = Vec::new();
        for &next in neighbors {
            if next != state.from_node {
                valid_neighbors.push(next);
            }
        }
        if !valid_neighbors.is_empty() {
            let next_node = valid_neighbors[rand_range(0, valid_neighbors.len())];
            best_plan.push(Action::Move(next_node));
        }
    }

    // Reverse plan to use pop()
    best_plan.reverse(); 
    state.current_plan = best_plan;
}

fn execute_action<W: Write>(state: &mut State, out: &mut W) {
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