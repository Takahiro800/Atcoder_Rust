#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        X: [isize;N],
    };

    let mut counter_bonus = vec![0; N + 1];
    for _ in 0..M {
        input! { c: usize, y: isize };
        counter_bonus[c] += y;
    }

    let mut dp = vec![vec![-std::isize::MAX; N + 1]; N + 2];
    dp[0][0] = 0;

    for i in 0..N {
        for j in 0..N {
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][j]);
            dp[i + 1][j + 1] = dp[i][j] + X[i] + counter_bonus[j + 1];
        }
    }
    let ans = dp[N].iter().max().unwrap();
    println!("{}", ans);
}
