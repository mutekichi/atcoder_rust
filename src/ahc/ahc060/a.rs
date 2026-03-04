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

fn rand_range(
    low: usize,
    high: usize,
) -> usize {
    RNG.with(|rng| rng.borrow_mut().random_range(low..high))
}

fn rand_bool(p: f64) -> bool {
    RNG.with(|rng| rng.borrow_mut().random_bool(p))
}

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut out);

    out.flush().unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct IceCream {
    flavor: u64,
    len: usize,
}

impl IceCream {
    fn add(
        &mut self,
        is_white: bool,
    ) {
        if !is_white {
            self.flavor ^= 1u64 << self.len;
        }
        if self.len == 63 {
            eprintln!("Warning: IceCream length exceeds 64, flavor will be truncated");
        } else {
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

struct State {
    current_node: usize,
    from_node: usize,
    ice_cream: IceCream,
    delivered_ice_creams: Vec<BTreeSet<IceCream>>,
    steps_left: usize,
    graph: Vec<Vec<usize>>,
    node_status: Vec<NodeType>,
    n_total_turns: usize,
    n_total_trees: usize,
    n_red_trees: usize,
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

    let node_status = vec![NodeType::Shop; n_total_shops]
        .into_iter()
        .chain(vec![NodeType::White; n_total_nodes - n_total_shops])
        .collect::<Vec<_>>();

    let mut state = State {
        current_node: 0,
        from_node: !0,
        ice_cream: IceCream { flavor: 0, len: 0 },
        delivered_ice_creams: vec![BTreeSet::new(); n_total_shops],
        steps_left: n_move_limit,
        graph,
        node_status,
        n_total_turns: n_move_limit,
        n_total_trees: n_total_nodes - n_total_shops,
        n_red_trees: 0,
    };

    while state.steps_left > 0 {
        act_naive(&mut state, out);
    }
}

fn act_naive<W: Write>(
    state: &mut State,
    out: &mut W,
) {
    let neighbors = &state.graph[state.current_node];

    let mut valid_neighbors = Vec::new();
    for &next in neighbors {
        if next != state.from_node {
            valid_neighbors.push(next);
        }
    }

    if valid_neighbors.is_empty() {
        state.steps_left -= 1;
        return;
    }

    let next_node = valid_neighbors[rand_range(0, valid_neighbors.len())];

    writeln!(out, "{}", next_node).unwrap();
    state.steps_left -= 1;

    state.from_node = state.current_node;
    state.current_node = next_node;

    if state.steps_left == 0 {
        return;
    }

    if state.node_status[next_node] != NodeType::Shop {
        let is_white = state.node_status[next_node] == NodeType::White;
        state.ice_cream.add(is_white);

        if is_white {
            let progress =
                (state.n_total_turns - state.steps_left) as f64 / state.n_total_turns as f64;
            let red_ratio = state.n_red_trees as f64 / state.n_total_trees as f64;

            if red_ratio < progress {
                if rand_bool(0.8) {
                    writeln!(out, "-1").unwrap();
                    state.node_status[next_node] = NodeType::Red;
                    state.n_red_trees += 1;
                    state.steps_left -= 1;
                }
            }
        }
    } else {
        let shop_id = next_node;
        state.delivered_ice_creams[shop_id].insert(state.ice_cream.clone());
        state.ice_cream = IceCream { flavor: 0, len: 0 };
    }
}
