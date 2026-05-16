use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <in_file> <out_file>", args[0]);
        std::process::exit(1);
    }
    let in_file = &args[1];
    let out_file = &args[2];

    let in_content = fs::read_to_string(in_file).unwrap_or_default();
    let out_content = fs::read_to_string(out_file).unwrap_or_default();

    let score = parse_and_calc(&in_content, &out_content).unwrap_or(0);
    println!("{}", score);
}

fn parse_and_calc(in_content: &str, out_content: &str) -> Option<i64> {
    let mut in_tokens = in_content.split_ascii_whitespace();
    let n: usize = in_tokens.next()?.parse().ok()?;
    
    let mut grid = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = in_tokens.next()?.parse().ok()?;
        }
    }

    let mut out_tokens = out_content.split_ascii_whitespace();
    let m_str = out_tokens.next()?;
    let m: usize = m_str.parse().ok()?;
    if m > n * n {
        eprintln!("Error: M exceeds N^2");
        return Some(0);
    }

    let mut belts = Vec::new();
    let mut cell_usage = vec![vec![0; n]; n];

    for _ in 0..m {
        let l: usize = out_tokens.next()?.parse().ok()?;
        if l < 2 {
            eprintln!("Error: Belt length < 2");
            return Some(0);
        }
        
        let mut belt = Vec::new();
        let mut unique_check = HashSet::new();
        for _ in 0..l {
            let r: usize = out_tokens.next()?.parse().ok()?;
            let c: usize = out_tokens.next()?.parse().ok()?;
            if r >= n || c >= n {
                eprintln!("Error: Coordinate out of bounds");
                return Some(0);
            }
            belt.push((r, c));
            cell_usage[r][c] += 1;
            if cell_usage[r][c] > 2 {
                eprintln!("Error: Cell used more than 2 times");
                return Some(0);
            }
            if !unique_check.insert((r, c)) {
                eprintln!("Error: Cell duplicated in a belt");
                return Some(0);
            }
        }
        
        for i in 0..l {
            let next = belt[(i + 1) % l];
            let curr = belt[i];
            let dr = (curr.0 as i32 - next.0 as i32).abs();
            let dc = (curr.1 as i32 - next.1 as i32).abs();
            if dr + dc != 1 {
                eprintln!("Error: Cells not adjacent");
                return Some(0);
            }
        }
        belts.push(belt);
    }

    let t: usize = out_tokens.next()?.parse().ok()?;
    if t > 100_000 {
        eprintln!("Error: T exceeds 100,000");
        return Some(0);
    }

    let mut ops = Vec::new();
    for _ in 0..t {
        let mt: usize = out_tokens.next()?.parse().ok()?;
        let dt: i32 = out_tokens.next()?.parse().ok()?;
        if mt >= m {
            eprintln!("Error: Invalid belt index");
            return Some(0);
        }
        if dt != 1 && dt != -1 {
            eprintln!("Error: Invalid direction");
            return Some(0);
        }
        ops.push((mt, dt));
    }

    let exit_r = 0;
    let exit_c = n / 2;
    let mut next_box = 0;

    if grid[exit_r][exit_c] == next_box {
        grid[exit_r][exit_c] = -1;
        next_box += 1;
    }

    for &(mt, dt) in &ops {
        if next_box == (n * n) as i32 {
            break;
        }
        
        let belt = &belts[mt];
        let len = belt.len();
        let mut new_vals = vec![0; len];
        
        for i in 0..len {
            let from_idx = (i as i32 - dt + len as i32) as usize % len;
            new_vals[i] = grid[belt[from_idx].0][belt[from_idx].1];
        }
        
        for i in 0..len {
            grid[belt[i].0][belt[i].1] = new_vals[i];
        }

        if grid[exit_r][exit_c] == next_box {
            grid[exit_r][exit_c] = -1;
            next_box += 1;
        }
    }

    let b = next_box as i64;
    let n_sq = (n * n) as i64;
    
    let score = if b == n_sq {
        let t_f = t.max(1) as f64;
        let s = 1_000_000.0 + 1_000_000.0 * (100_000.0 / t_f).log2();
        s.round() as i64
    } else {
        let s = 1_000_000.0 * (b as f64) / (n_sq as f64);
        s.round() as i64
    };

    Some(score)
}