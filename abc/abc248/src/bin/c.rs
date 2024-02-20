#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize
    };

    const MOD: usize = 998_244_353;
    let mut dp = vec![1];

    for _ in 0..N {
        let mut next = vec![0; dp.len() + M];
        for (i, dp) in dp.iter().enumerate() {
            for j in 1..=M {
                next[i + j] += dp;
            }
        }
        dp = next;
        dp.iter_mut().for_each(|dp| *dp %= MOD)
    }
    dp.truncate(K + 1);
    let ans = dp.iter().sum::<usize>() % MOD;
    println!("{}", ans);
}
