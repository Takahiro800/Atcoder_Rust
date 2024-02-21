#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        X: [usize;N],
    };

    let mut counter_bonus = vec![0; N + 1];
    for _ in 0..M {
        input! { c: usize, y: usize };
        counter_bonus[c] += y;
    }

    let mut dp = vec![vec![0; N + 1]; N + 2];
    dp[0][0] = 0;

    for i in 0..N {
        dp[i + 1][0] = *dp[i].iter().max().unwrap();
        for j in 0..=i {
            dp[i + 1][j + 1] = dp[i][j] + X[i] + counter_bonus[j + 1];
        }
    }
    let ans = dp[N].iter().max().unwrap();
    println!("{}", ans);
}
