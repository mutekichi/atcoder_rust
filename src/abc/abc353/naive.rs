#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        A: [i64; n],
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            ans += (A[i] + A[j]) % 100_000_000;
        }
    }
    println!("{}", ans);
}
