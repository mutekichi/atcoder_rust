use itertools::Itertools;
use proconio::input;

fn main() {
    unsafe {
        solve();
    }
}

#[target_feature(enable = "avx2")]
fn solve() {
    input! {
        n: usize,
        m: usize,
        a: [[u16; m]; n],
    }
    let mut ans = 0usize;
    for (i, j) in (0..n).tuple_combinations() {
        ans += a[i].iter().zip(&a[j]).filter(|&e| *e.0 == *e.1).count() % 2;
    }
    println!("{}", ans);
}
