#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Usize1;
use std::io::{BufWriter, Write, stdin, stdout};
use std::mem::swap;

#[allow(unused_variables)]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut a: Usize1,
            mut b: Usize1,
        }
        if n % 2 == 1 {
            println!("No");
            continue;
        }
        if (a + b) % 2 == 0 {
            println!("No");
            continue;
        }
        println!("Yes");
        let mut ans = Vec::with_capacity(n * n - 2);
        let mut before = true;
        for i in 0..n / 2 {
            if a / 2 == i {
                {
                    let mut before_2 = true;
                    for j in 0..n / 2 {
                        if b / 2 == j {
                            if a % 2 == 0 {
                                ans.push('D');
                                ans.push('R');
                            } else {
                                ans.push('R');
                                ans.push('D');
                            }
                            before_2 = false;
                        } else {
                            if before_2 {
                                ans.push('D');
                                ans.push('R');
                                ans.push('U');
                            } else {
                                ans.push('U');
                                ans.push('R');
                                ans.push('D');
                            }
                        }
                        if j != n / 2 - 1 {
                            ans.push('R');
                        }
                    }
                }

                before = false;
            } else {
                if before {
                    for _ in 0..n - 1 {
                        ans.push('R');
                    }
                    ans.push('D');
                    for _ in 0..n - 1 {
                        ans.push('L');
                    }
                } else {
                    for _ in 0..n - 1 {
                        ans.push('L');
                    }
                    ans.push('D');
                    for _ in 0..n - 1 {
                        ans.push('R');
                    }
                }
            }
            if i != n / 2 - 1 {
                ans.push('D');
            }
        }

        println!("{}", ans.iter().collect::<String>());
        {
            let mut i = 0;
            let mut j = 0;
            for &c in &ans {
                if c == 'D' {
                    i += 1;
                } else if c == 'U' {
                    i -= 1;
                } else if c == 'R' {
                    j += 1;
                } else if c == 'L' {
                    j -= 1;
                }
                assert!(i < n, "i");
                assert!(j < n, "j");
                assert!((i, j) != (a, b), "{} {} {}", n, a, b);
            }
            assert!((i, j) == (n - 1, n - 1));
        }
    }
}
