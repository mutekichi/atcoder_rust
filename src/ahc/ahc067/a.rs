use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::time::Instant;

const N: usize = 20;
const M: usize = 50;

struct XorShift {
    seed: u64,
}

impl XorShift {
    fn new(seed: u64) -> Self {
        XorShift { seed }
    }
    fn next(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    fn next_usize(&mut self) -> usize {
        self.next() as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    r: usize,
    c: usize,
}

struct Door {
    d: usize,
    i: usize,
    j: usize,
    g: usize,
}

struct Switch {
    p: usize,
    q: usize,
    s: usize,
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_edge(
    u: Pos,
    v: Pos,
) -> (Pos, Pos) {
    if u.r < v.r || (u.r == v.r && u.c < v.c) {
        (u, v)
    } else {
        (v, u)
    }
}

// Fast edge management using 2D arrays instead of HashSet
#[derive(Clone)]
struct EdgeMap {
    h: [[bool; N]; N],
    v: [[bool; N]; N],
}

impl EdgeMap {
    fn new() -> Self {
        Self {
            h: [[false; N]; N],
            v: [[false; N]; N],
        }
    }

    fn insert(
        &mut self,
        u: Pos,
        v: Pos,
    ) {
        if u.r == v.r {
            self.h[u.r][u.c.min(v.c)] = true;
        } else {
            self.v[u.r.min(v.r)][u.c] = true;
        }
    }

    fn contains(
        &self,
        u: Pos,
        v: Pos,
    ) -> bool {
        if u.r == v.r {
            self.h[u.r][u.c.min(v.c)]
        } else {
            self.v[u.r.min(v.r)][u.c]
        }
    }

    fn count(&self) -> usize {
        let mut c = 0;
        for r in 0..N {
            for i in 0..N {
                if self.h[r][i] {
                    c += 1;
                }
                if self.v[r][i] {
                    c += 1;
                }
            }
        }
        c
    }

    fn iter(&self) -> Vec<(Pos, Pos)> {
        let mut res = Vec::new();
        for r in 0..N {
            for c in 0..N - 1 {
                if self.h[r][c] {
                    res.push((Pos { r, c }, Pos { r, c: c + 1 }));
                }
            }
        }
        for r in 0..N - 1 {
            for c in 0..N {
                if self.v[r][c] {
                    res.push((Pos { r, c }, Pos { r: r + 1, c }));
                }
            }
        }
        res
    }
}

struct DoorManager {
    doors: Vec<Door>,
    used_edges: EdgeMap,
}

impl DoorManager {
    fn new() -> Self {
        Self {
            doors: Vec::new(),
            used_edges: EdgeMap::new(),
        }
    }

    fn add(
        &mut self,
        u: Pos,
        v: Pos,
        g: usize,
    ) -> bool {
        if self.used_edges.contains(u, v) {
            return false;
        }
        if self.doors.len() >= M {
            return false;
        }
        self.used_edges.insert(u, v);

        let (d, i, j) = if u.r == v.r {
            (1, u.r, u.c.min(v.c))
        } else {
            (0, u.r.min(v.r), u.c)
        };
        self.doors.push(Door { d, i, j, g });
        true
    }
}

fn generate_random_path_A(
    grid: &[Vec<char>],
    start: Pos,
    goal: Pos,
    rng: &mut XorShift,
) -> Vec<Pos> {
    let mut visited = [[false; N]; N];
    let mut current_path = Vec::with_capacity(400);
    let mut found = false;
    let mut step_count = 0;

    fn dfs(
        u: Pos,
        goal: Pos,
        grid: &[Vec<char>],
        visited: &mut [[bool; N]; N],
        current_path: &mut Vec<Pos>,
        found: &mut bool,
        rng: &mut XorShift,
        step_count: &mut usize,
    ) {
        if *found || *step_count > 1000 {
            return;
        }
        *step_count += 1;

        visited[u.r][u.c] = true;
        current_path.push(u);

        if u == goal {
            *found = true;
            return;
        }

        let mut dirs = DIRS;
        for i in (1..4).rev() {
            dirs.swap(i, rng.next_usize() % (i + 1));
        }

        for &d in &dirs {
            let nr = u.r as i32 + d.0;
            let nc = u.c as i32 + d.1;
            if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == '.' && !visited[nr][nc] {
                    let mut adj = 0;
                    for &d2 in &DIRS {
                        let nnr = nr as i32 + d2.0;
                        let nnc = nc as i32 + d2.1;
                        if nnr >= 0 && nnr < N as i32 && nnc >= 0 && nnc < N as i32 {
                            if visited[nnr as usize][nnc as usize] {
                                adj += 1;
                            }
                        }
                    }
                    if adj <= 1 {
                        dfs(
                            Pos { r: nr, c: nc },
                            goal,
                            grid,
                            visited,
                            current_path,
                            found,
                            rng,
                            step_count,
                        );
                        if *found {
                            return;
                        }
                    }
                }
            }
        }
        if !*found {
            current_path.pop();
            visited[u.r][u.c] = false;
        }
    }

    dfs(
        start,
        goal,
        grid,
        &mut visited,
        &mut current_path,
        &mut found,
        rng,
        &mut step_count,
    );
    current_path
}

fn get_longest_path_B(
    start: Pos,
    grid: &[Vec<char>],
    u_initial: &[[bool; N]; N],
    rng: &mut XorShift,
    max_steps: usize,
) -> Vec<Pos> {
    let mut best_path = Vec::new();
    let mut current_path = Vec::with_capacity(400);
    let mut visited = *u_initial;
    let mut step_count = 0;

    fn dfs(
        u: Pos,
        grid: &[Vec<char>],
        visited: &mut [[bool; N]; N],
        current_path: &mut Vec<Pos>,
        best_path: &mut Vec<Pos>,
        step_count: &mut usize,
        max_steps: usize,
        rng: &mut XorShift,
    ) {
        if *step_count > max_steps {
            return;
        }
        *step_count += 1;

        current_path.push(u);
        visited[u.r][u.c] = true;

        if current_path.len() > best_path.len() {
            best_path.clear();
            best_path.extend_from_slice(current_path);
        }

        let mut valid_nexts = [((0, Pos { r: 0, c: 0 })); 4];
        let mut count = 0;

        for &d in &DIRS {
            let nr = u.r as i32 + d.0;
            let nc = u.c as i32 + d.1;
            if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == '.' && !visited[nr][nc] {
                    let mut adj_count = 0;
                    for &d2 in &DIRS {
                        let nnr = nr as i32 + d2.0;
                        let nnc = nc as i32 + d2.1;
                        if nnr >= 0 && nnr < N as i32 && nnc >= 0 && nnc < N as i32 {
                            if visited[nnr as usize][nnc as usize] {
                                adj_count += 1;
                            }
                        }
                    }
                    if adj_count == 1 {
                        let mut open_neighbors = 0;
                        for &d2 in &DIRS {
                            let nnr = nr as i32 + d2.0;
                            let nnc = nc as i32 + d2.1;
                            if nnr >= 0 && nnr < N as i32 && nnc >= 0 && nnc < N as i32 {
                                if grid[nnr as usize][nnc as usize] == '.'
                                    && !visited[nnr as usize][nnc as usize]
                                {
                                    open_neighbors += 1;
                                }
                            }
                        }
                        valid_nexts[count] = (open_neighbors, Pos { r: nr, c: nc });
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            for i in (1..count).rev() {
                valid_nexts.swap(i, rng.next_usize() % (i + 1));
            }
            valid_nexts[..count].sort_by_key(|k| k.0);

            for i in 0..count {
                dfs(
                    valid_nexts[i].1,
                    grid,
                    visited,
                    current_path,
                    best_path,
                    step_count,
                    max_steps,
                    rng,
                );
            }
        }

        visited[u.r][u.c] = false;
        current_path.pop();
    }

    dfs(
        start,
        grid,
        &mut visited,
        &mut current_path,
        &mut best_path,
        &mut step_count,
        max_steps,
        rng,
    );
    if best_path.is_empty() {
        best_path.push(start);
    }
    best_path
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _first_line = lines.next().unwrap().unwrap();
    let mut grid = vec![];
    for _ in 0..N {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let start = Pos { r: 0, c: 0 };
    let goal = Pos { r: N - 1, c: N - 1 };
    let mut rng = XorShift::new(42);

    let mut best_solution: Option<(Vec<Pos>, Vec<(usize, Pos)>, Vec<Vec<Pos>>, Vec<(Pos, Pos)>)> =
        None;
    let mut best_score = -999999999;
    let start_time = Instant::now();

    let mut iter_count = 0;
    let mut q = VecDeque::with_capacity(400);

    loop {
        iter_count += 1;
        // Check time periodically to reduce overhead
        if (iter_count & 255) == 0 && start_time.elapsed().as_secs_f64() > 1.85 {
            break;
        }

        let path_a = generate_random_path_A(&grid, start, goal, &mut rng);
        if path_a.len() < 12 {
            continue;
        }

        let mut path_a_set = [[false; N]; N];
        for p in &path_a {
            path_a_set[p.r][p.c] = true;
        }

        let mut possible_branches = vec![];
        for i in 0..path_a.len() - 2 {
            let u = path_a[i];
            let mut dirs = DIRS;
            for j in (1..4).rev() {
                dirs.swap(j, rng.next_usize() % (j + 1));
            }

            for &d in &dirs {
                let nr = u.r as i32 + d.0;
                let nc = u.c as i32 + d.1;
                if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] == '.' && !path_a_set[nr][nc] {
                        let mut adj_main = 0;
                        for &d2 in &DIRS {
                            let nnr = nr as i32 + d2.0;
                            let nnc = nc as i32 + d2.1;
                            if nnr >= 0 && nnr < N as i32 && nnc >= 0 && nnc < N as i32 {
                                if path_a_set[nnr as usize][nnc as usize] {
                                    adj_main += 1;
                                }
                            }
                        }
                        if adj_main == 1 {
                            possible_branches.push((i, Pos { r: nr, c: nc }));
                        }
                    }
                }
            }
        }

        let mut best_indices = vec![];
        let mut best_hub_score = -1;

        // Try multiple hub combinations to maximize distance between earlier switches
        for _ in 0..50 {
            let mut indices: Vec<usize> = (0..possible_branches.len()).collect();
            for i in (1..indices.len()).rev() {
                indices.swap(i, rng.next_usize() % (i + 1));
            }
            indices.truncate(10);
            indices.sort_unstable();

            if indices.len() == 10 {
                let mut ok_all = true;
                let mut used_v_temp = [[false; N]; N];
                for &idx in &indices {
                    let v = possible_branches[idx].1;
                    let mut ok = true;
                    for &d in &DIRS {
                        let nr = v.r as i32 + d.0;
                        let nc = v.c as i32 + d.1;
                        if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                            if used_v_temp[nr as usize][nc as usize] {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if !ok {
                        ok_all = false;
                        break;
                    }
                    used_v_temp[v.r][v.c] = true;
                }

                if ok_all {
                    let mut hub_score = 0;
                    for k in 0..9 {
                        let dist =
                            possible_branches[indices[k + 1]].0 - possible_branches[indices[k]].0;
                        hub_score += dist as i64 * (1 << (10 - k));
                    }
                    if hub_score > best_hub_score {
                        best_hub_score = hub_score;
                        best_indices = indices;
                    }
                }
            }
        }

        if best_indices.is_empty() {
            continue;
        }

        let mut branch_nodes = vec![];
        for &idx in &best_indices {
            branch_nodes.push(possible_branches[idx]);
        }

        let mut branch_paths = vec![];
        let mut u_initial = [[false; N]; N];
        for p in &path_a {
            u_initial[p.r][p.c] = true;
        }
        for &(_, v) in &branch_nodes {
            u_initial[v.r][v.c] = true;
        }

        for k in 0..10 {
            let v = branch_nodes[k].1;
            u_initial[v.r][v.c] = false;
            let path = get_longest_path_B(v, &grid, &u_initial, &mut rng, 2000);
            for p in &path {
                u_initial[p.r][p.c] = true;
            }
            branch_paths.push(path);
        }

        let mut e_intended = EdgeMap::new();
        for i in 0..path_a.len() - 1 {
            e_intended.insert(path_a[i], path_a[i + 1]);
        }

        for k in 0..10 {
            let idx = branch_nodes[k].0;
            let hub = path_a[idx];
            let path_b = &branch_paths[k];

            e_intended.insert(hub, path_b[0]);
            for i in 0..path_b.len() - 1 {
                e_intended.insert(path_b[i], path_b[i + 1]);
            }
        }

        let u_used = u_initial;
        let mut leak_edges = EdgeMap::new();

        for r in 0..N {
            for c in 0..N {
                if u_used[r][c] {
                    let u = Pos { r, c };
                    for &d in &[(1, 0), (0, 1)] {
                        let nr = r as i32 + d.0;
                        let nc = c as i32 + d.1;
                        if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if u_used[nr][nc] {
                                let v = Pos { r: nr, c: nc };
                                if !e_intended.contains(u, v) {
                                    leak_edges.insert(u, v);
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut comp_id = [[-1_i32; N]; N];
        let mut id = 0;

        q.clear();
        for r in 0..N {
            for c in 0..N {
                if grid[r][c] == '.' && !u_used[r][c] && comp_id[r][c] == -1 {
                    q.push_back(Pos { r, c });
                    comp_id[r][c] = id;
                    while let Some(u) = q.pop_front() {
                        for d in &DIRS {
                            let nr = u.r as i32 + d.0;
                            let nc = u.c as i32 + d.1;
                            if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                                let nr = nr as usize;
                                let nc = nc as usize;
                                if grid[nr][nc] == '.' && !u_used[nr][nc] && comp_id[nr][nc] == -1 {
                                    comp_id[nr][nc] = id;
                                    q.push_back(Pos { r: nr, c: nc });
                                }
                            }
                        }
                    }
                    id += 1;
                }
            }
        }

        let mut current_adj_nodes = [[false; N]; N];

        for cid in 0..id {
            let mut connecting_edges = vec![];
            let mut adj_count = 0;

            for r in 0..N {
                current_adj_nodes[r].fill(false);
            }

            for r in 0..N {
                for c in 0..N {
                    if comp_id[r][c] == cid {
                        let u = Pos { r, c };
                        for &d in &DIRS {
                            let nr = r as i32 + d.0;
                            let nc = c as i32 + d.1;
                            if nr >= 0 && nr < N as i32 && nc >= 0 && nc < N as i32 {
                                let nr = nr as usize;
                                let nc = nc as usize;
                                if u_used[nr][nc] {
                                    connecting_edges.push(get_edge(u, Pos { r: nr, c: nc }));
                                    if !current_adj_nodes[nr][nc] {
                                        current_adj_nodes[nr][nc] = true;
                                        adj_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if adj_count >= 2 {
                connecting_edges.sort_by(|a, b| {
                    if a.0.r != b.0.r {
                        a.0.r.cmp(&b.0.r)
                    } else if a.0.c != b.0.c {
                        a.0.c.cmp(&b.0.c)
                    } else if a.1.r != b.1.r {
                        a.1.r.cmp(&b.1.r)
                    } else {
                        a.1.c.cmp(&b.1.c)
                    }
                });
                connecting_edges.dedup();

                if !connecting_edges.is_empty() {
                    for i in 1..connecting_edges.len() {
                        leak_edges.insert(connecting_edges[i].0, connecting_edges[i].1);
                    }
                }
            }
        }

        let leak_count = leak_edges.count();
        let is_valid = leak_count <= 31;

        let score = if is_valid {
            let mut s = 0i64;
            for k in 0..10 {
                s += branch_paths[k].len() as i64 * (1 << (10 - k));
            }
            for k in 0..9 {
                let dist = branch_nodes[k + 1].0 - branch_nodes[k].0;
                s += dist as i64 * (1 << (10 - k));
            }
            s * 100 - leak_count as i64
        } else {
            -(leak_count as i64)
        };

        if score > best_score {
            best_score = score;
            best_solution = Some((path_a, branch_nodes, branch_paths, leak_edges.iter()));
        }
    }

    if let Some((path_a, branch_nodes, branch_paths, leak_edges)) = best_solution {
        let mut door_manager = DoorManager::new();
        let mut switches = vec![];

        for k in 0..10 {
            let idx = branch_nodes[k].0;
            let hub = path_a[idx];
            let v = branch_paths[k][0];
            let sw_pos = *branch_paths[k].last().unwrap();

            if k == 0 {
                switches.push(Switch {
                    p: sw_pos.r,
                    q: sw_pos.c,
                    s: 0,
                });
            } else {
                door_manager.add(hub, v, 2 * k - 1);
                door_manager.add(hub, path_a[idx + 1], 2 * k - 2);
                switches.push(Switch {
                    p: sw_pos.r,
                    q: sw_pos.c,
                    s: k,
                });
            }
        }

        let prev_goal = path_a[path_a.len() - 2];
        let goal_pos = path_a[path_a.len() - 1];
        door_manager.add(prev_goal, goal_pos, 19);

        for edge in leak_edges {
            door_manager.add(edge.0, edge.1, 19);
        }

        println!("{}", door_manager.doors.len());
        for door in door_manager.doors {
            println!("{} {} {} {}", door.d, door.i, door.j, door.g);
        }

        println!("{}", switches.len());
        for sw in switches {
            println!("{} {} {}", sw.p, sw.q, sw.s);
        }
    } else {
        println!("0\n0");
    }
}
