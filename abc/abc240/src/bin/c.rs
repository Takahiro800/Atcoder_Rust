#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        N: usize,
        X: usize,
        AB: [(usize,usize); N]
    };

    let mut dp = vec![vec![false; X + 1]; N];

    for (i, &(a, b)) in AB.iter().enumerate() {
        if i == 0 {
            if a <= X {
                dp[i][a] = true
            };
            if b <= X {
                dp[i][b] = true
            };
        } else {
            let prev = &dp[i - 1].clone();
            let line = &mut dp[i];

            for j in 0..X {
                if prev[j] {
                    if j + a <= X {
                        line[j + a] = true;
                    }
                    if j + b <= X {
                        line[j + b] = true;
                    }
                }
            }
        }
    }
    println!("{}", if dp[N - 1][X] { "Yes" } else { "No" });
}
