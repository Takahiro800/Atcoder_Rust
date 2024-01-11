#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: usize,
        M: usize,
        L: usize,
    };
    let mut ans = N * S;

    for i in 0..=N {
        for j in 0..=N {
            for k in 0..=N {
                if 6 * i + 8 * j + 12 * k >= N {
                    let sum = S * i + M * j + L * k;
                    if sum < ans {
                        ans = sum;
                    }
                }
            }
        }
    }
    println!("{}", ans)
}
