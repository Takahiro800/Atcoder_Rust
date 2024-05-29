#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
    };
    const MOD: usize = 10_usize.pow(9) + 7;

    let mut dp = vec![0; N + 1];
    dp[0] = 1;

    for i in 1..=N {
        let mut v = dp[i - 1];

        if i >= L {
            v += dp[i - L];
        }
        dp[i] = v % MOD;
    }
    let ans = dp[N];
    println!("{}", ans);
}
