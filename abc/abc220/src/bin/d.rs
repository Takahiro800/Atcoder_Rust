#![allow(non_snake_case)]
use std::vec;

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N]
    };

    const MOD: usize = 998_244_353;
    let mut dp = vec![vec![0; N]; 10];

    for (i, &a) in A.iter().enumerate() {
        if i == 0 {
            dp[a][i] = 1;
        } else {
            for j in 0..10 {
                let k = (j + a) % 10;
                dp[k][i] += dp[j][i - 1];

                let k = (j * a) % 10;
                dp[k][i] += dp[j][i - 1];
            }
            for j in 0..10 {
                dp[j][i] %= MOD;
            }
        }
    }

    for line in dp {
        println!("{}", line[N - 1]);
    }
}
