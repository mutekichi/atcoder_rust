use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let ans: i32 = a.iter().sum();
    println!("{}", ans);
}
